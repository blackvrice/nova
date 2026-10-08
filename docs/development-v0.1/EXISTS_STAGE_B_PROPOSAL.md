# Stage B Copy Option postfix exists 최소 계약 — P20

작성일: 2026-10-08. 상태: **Draft / 사용자 승인 대기 / 미구현**.
기존 P01~P19 승인 사양은 보존한다. 이 문서·[전용 EBNF](GRAMMAR_STAGE_B_EXISTS.ebnf)·
[수용 fixture](exists-proposal-fixtures/README.md)는 검토 자료이며 구현 성공을 뜻하지 않는다.

## Specification Change Proposal

- 관련 문서: [Canonical](CANONICAL.md), [MVP Freeze](../00_Governance/NOVA-002_Nova_0.1_MVP_기능_동결표.md),
  [원본 NOVA-028](../03_Types_Declarations/NOVA-028_Nullable_Option_Result_타입_규칙.md),
  [NOVA-028 보완 초안](specs/NOVA-028.md), NOVA-014/016/024/073~078/081/083/087/091/136.
  의존 계약은 [P15](OPTION_RESULT_STAGE_B_PROPOSAL.md), [P16](TRY_STAGE_B_PROPOSAL.md),
  [P18](DEFAULT_ARGUMENTS_STAGE_B_PROPOSAL.md), [P19](RANGE_LOOP_STAGE_B_PROPOSAL.md)다.
- 현재 사양: exists는 D01의 keyword다. 전체 Draft grammar는 postfix exists,
  NOVA-028 보완 초안은 payload를 소비하지 않는 Bool 검사로 제안했다. P15는 exists를 제외했다.
- 발견된 문제: intrinsic Option만 허용할지, payload truthiness·nested None·문맥·결합성,
  const node budget·단일 평가·Source/MIR 검증의 상세가 미동결이다.
- 제안 변경: 기존 Copy intrinsic Option<T>의 postfix 존재 검사만 아래 최소 계약으로 동결한다.
- 변경 이유: 기존 Option/nullable·tagged representation·Bool·const·CFG를 연결한다.
  Array는 owning buffer·원소 Drop·Move/borrow 검사가 필요하므로 기존 Stage C 의존성을 보존한다.
- 영향 범위: Parser/AST/HIR/TypeChecker/const, MIR/validator·LLVM/Native tests 및 문서.
  Scanner/END·Types layout·Runtime API·private ABI는 변경하지 않는다.
- Backward Compatibility: 기존 유효 프로그램·D01~D05/P01~P19·원본 148개·Canonical을 보존한다.
  기존 미지원 exists 표현식만 수용하며 Option 이름 shadow 정책은 P15 그대로다.
- 대안: match만 사용하거나 Move payload/flow narrowing까지 함께 구현할 수 있다.
  이 안은 Bool predicate만 추가하고 payload 추출·narrowing·일반 ownership은 후속으로 남긴다.

## 문법·결합성·END

1. expression exists는 postfix expression이다. P19의 postfix_expr 한 production에
   | "exists"만 추가한다. 총 **54개 production**, 기존 **53개 보존**이다.
   call·field/tuple selector·as와 같은 postfix 층에서 소스 순서로 왼쪽 결합한다.
   postfix는 prefix !/try·산술·비교·논리보다 강하다.
2. !x exists는 !(x exists), x exists == true는 (x exists) == true다.
   try f() exists는 try (f() exists)다. Result<Option<T>,E> 결과를 검사하려면
   (try f()) exists를 쓴다. x exists exists는 파싱한 후 첫 결과 Bool에서 N2101이다.
   x exists()도 Bool을 callee로 해석해 기존 N2101 not-callable로 거부한다.
3. prefix exists x, 인수 목록/선언 이름의 exists, Result 존재 검사·사용자 operator를 추가하지 않는다.
   bare prefix exists는 기존 미지원 keyword N1102다. postfix에 추가 인수를 받는 의미도 없다.
4. D05 END를 보존한다. exists는 이미 can-finish token이며 statement 끝에서 END를 만든다.
   x와 exists 사이 newline이 END를 생성하면 두 문장이다. 다음 줄 exists는 N1102다.
   괄호 안 줄바꿈은 기존 delimiter 억제로 연결한다. ; 뒤 exists도 연결하지 않는다.
   Scanner/raw token/최장 일치/normalized dump·기존 type argument adapter는 변경하지 않는다.

## 타입·값·단일 평가

1. operand는 **구체 intrinsic Option<T>**, T는 P15의 기존 Copy payload만 허용한다.
   T?도 같은 intrinsic 타입이다. Bool 결과는 Some이면 true, None이면 false다.
   Some(0)·Some(false)·Some(())·Some(None)은 모두 true다. inner payload 값은 검사하지 않는다.
   Result<T,E>·Bool/숫자/String/Unit/Tuple·user Enum은 N2101이다.
   사용자 Enum Option에 Some/None이 있어도 intrinsic Option으로 간주하지 않는다.
   type import shadow·alias·private nominal payload identity는 P15를 유지한다.
2. operand를 expected type 없는 독립 expression으로 검사한다. 외부 Bool/인수/return 문맥은
   payload type으로 전달하지 않는다. Some(1)은 기존 Int32 payload를 추론한다.
   none exists, Option::None exists, Option::Some(none) exists는 N2103이다.
   먼저 typed Option을 선언한 후 검사해야 한다. 추론 실패/undefined operand의 ErrorType은
   파생 exists N2101을 억제한다. 새 진단 code는 등록하지 않는다.
3. operand는 소스 위치에서 한 번만 평가하고 Copy snapshot의 discriminant를 Bool로 변환한다.
   본 연산은 payload read·소비·변경·추출을 하지 않는다. 기존 variable은 이후 match로 다시 사용할 수 있다.
   field replacement 뒤에도 이미 얻은 Bool은 그대로다. opaque private payload 검사도
   field 접근 권한이나 public layout 정보를 추가로 공개하지 않는다.
4. &&/||는 기존 Bool short-circuit와 effect 순서를 보존한다. skip된 operand의 call/Abort/try는 실행하지 않는다.
   실행된 operand의 Abort/try Error는 Bool 생성보다 먼저 함수 exit로 간다.
   named argument·P18 default materialization·while/for/loop/if/match에 기존 Bool로 사용할 수 있다.
5. unwrap·Some binder 생성·flow narrowing·Option equality/cast/보간·Result exists·Option try·method API·
   String/Move payload·일반 Read/change/take/Drop·사용자 Generic은 포함하지 않는다.

## Const·진단·자원

1. local/global const와 P18 parameter default에서도 순수 exists를 허용한다.
   operand가 기존 const subset일 때만 discriminant를 Bool ConstValue로 평가한다.
   runtime 함수 호출·try는 기존 N3201이다. 논리 short-circuit로 skip된 RHS도
   전체 permission/typing/static dependency 검사를 유지한다.
   const/default의 exists operand 내부 runtime call은 해당 call Span N3201이다.
2. exists HIR expression은 **1 node + operand subtree**다. Group·cached const reference·constructor head의
   기존 counting 규칙, initializer/default마다 **10,000-node** 한도와 N3202를 유지한다.
   exists가 payload를 읽지 않아도 operand 생성에 필요한 모든 expression을 평가한다.
   Some(1 / 0) exists는 division Span N3201이다. referenced global const는 기존 cache를 사용한다.
3. 정적 dependency 수집은 exists operand 내부의 모든 이름과 skipped logical RHS를 포함한다.
   const A:bool=Option::Some(B) exists; const B:bool=A는 P06/P11의 static cycle N3202다.
   Some이라는 사실만 보고 operand를 생략해 cycle/effect/예산을 숨기지 않는다.
4. non-Option N2101 primary는 원 operand 전체 byte Span이다. repeated exists의 두 번째 연산은
   첫 exists expression을 operand Span으로 쓴다. Bool numeric cast·not-callable·try·추론 진단은
   기존 operation/callee/operand/none Span을 사용한다. [정확한 사례](exists-proposal-fixtures/expected.json)를 따른다.
5. Parser nesting 한도 128/P04 N8901, aggregate/specialization 한도는 P12~P15 그대로다.
   새로운 specialization이나 payload field를 생성하지 않는다. flat postfix chain은 반복형으로 처리하고
   prefix truncation·작은 host stack·깊은 expression 검사/상수 평가/해제를 시험한다.

## Source·MIR·Native 계약과 수용 기준

- AST/HIR는 operand child 하나, exists keyword Span, whole expression Span과 FileId를 유지한다.
  외부 AST의 child count/order·keyword spelling·byte UTF-8/parent 포함·trivia gap을 검사한다.
  ExprId/HirId·TypeId·intrinsic specialization ID를 보존하고 public typed metadata는 재계산 검증한다.
- MIR은 단일 Copy snapshot/tag read·Bool 생성으로 명시한다. intrinsic kind/schema와 Some/None tag는
  P15 metadata를 사용한다. **None=0/nonnull pointer/payload truthiness**를 가정하지 않는다.
  LLVM은 검증된 tag CFG/비교를 lowering하고 family를 추측하지 않는다.
- private proof 또는 동등한 독립 validator로 operand/snapshot/source·result Bool·intrinsic family/type/schema·
  predicate direction·evaluation order·short-circuit·try bypass 위조를 CodegenUnit/LLVM 전에 거부한다.
  inactive payload access를 만들지 않는다. ABI/Runtime helper를 확장하지 않는다.
- [두 파일 fixture](exists-proposal-fixtures/README.md)는 nullable/import/alias·opaque private payload·nested None·
  const/default·snapshot·named effects·short-circuit·try의 **제안 18줄 출력**이다.
  정상 2·부정 20·Runtime 실패 1 source와 UTF-8 byte Span/cascade를 준비했다.
  구현 tests는 모든 Copy payload·shadow·Bool contexts/loops·reused Option match·zero/false/Unit·
  forward const·static cycle·10,000/10,001 nodes·cross-file failure Span을 포함한다.
- Windows Native debug/release, 실제 LLVM COFF/ELF O0/O2, fmt·별도 Runtime fmt·clippy·전체 workspace tests·
  all-features·문서 validator를 실행한다. Linux Native와 MSRV는 별도 증거가 필요하다.
  문서 validator PASS는 Compiler 구현/Native 성공을 뜻하지 않는다.

## 승인 경계

P20 Copy intrinsic Option postfix exists·Bool·const/default·Source/MIR 검증 subset만 별도 승인 대상이다.
Result exists·flow narrowing·unwrap·Move payload·일반 borrow/Drop·Array·methods·공용 ABI/FFI·
전체 D08/D09/D10/D16/D23/D25/D30을 승인하는 것이 아니다.
사용자의 승인을 받기 전에는 해당 사양 변경을 적용하지 않는다. 승인 후 구현하고 실제 검증 결과를 기록한다.

## 초안 준비 검증 — 2026-10-08

- 문서 build/validator PASS: 148개 원본 hash·링크·Draft ledger·54-production EBNF·기존 53개 production 보존,
  두 파일/정상 2/부정 20/Runtime 1 source·등록 code·UTF-8 byte Span·cascade·제안 18줄 metadata를 검사했다.
- 기존 전체 workspace baseline은 **326 PASS / 0 FAIL / 61 opt-in ignored**다.
  fmt·별도 Runtime rustfmt·clippy all-targets -D warnings·all-features도 PASS다.
- HEAD의 P19 구현 대비 D01~D05/P01~P19 accepted ledger·승인 문서/EBNF·Canonical/Accepted Lexer·
  전체 Draft grammar·원본 148개 SHA-256을 보존했다. Compiler/Runtime/Cargo source 변경은 없다.
  사용자 예제/IDE 변경은 이번 커밋에서 제외한다.
- P20 fixture는 기대값 준비만 완료했으며 Compiler/LLVM/Native에서 수용 성공을 검증하지 않았다.
  현재 compiler는 P19까지 구현했다. 이번에는 실제 LLVM/Native 회귀를 재실행하지 않았다.
