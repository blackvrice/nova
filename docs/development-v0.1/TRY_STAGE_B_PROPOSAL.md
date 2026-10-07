# Stage B Copy try·Result 오류 전파 최소 계약 — P16

작성일: 2026-10-07. 상태: **Accepted / 사용자 승인 완료 / 구현 완료**.
승인일: 2026-10-07. 사용자 답변: “P16 승인하고 Copy try 구현 진행”.
현재 구현·검증 범위는 [구현 기록](TRY_IMPLEMENTATION.md)을 따른다.
기존 D01~D05/P01~P15·Canonical·원본 148개 문서를 보존한다.
전체 D08/D09/D10/D12/D16/D23/D25 승인이 아니다.

## Specification Change Proposal

- 관련 문서: [Canonical](CANONICAL.md), [원본 Nullable/Result](../03_Types_Declarations/NOVA-028_Nullable_Option_Result_타입_규칙.md),
  [원본 try lowering](../04_Functions_Control/NOVA-049_try_Result_전파_Lowering_사양서.md),
  [원본 Result API](../11_Standard_Library/NOVA-120_Result_TE_try_API_사양서.md),
  [P15](OPTION_RESULT_STAGE_B_PROPOSAL.md), [P14](ENUM_STAGE_B_PROPOSAL.md),
  [P03 Runtime](NATIVE_IMPLEMENTATION.md), [const](CONST_STAGE_B_PROPOSAL.md), [결정](DECISIONS.md).
- 현재 확정 의미: try는 Result Error를 조기 반환하고 Success(())는 Unit 성공이다.
  P15는 intrinsic Copy Result 생성/match까지 구현했으며 try를 명시적으로 제외했다.
- 발견된 문제: prefix 결합 순서, enclosing function의 반환 타입, error 타입 동등성,
  operand 생성 문맥, const 금지 진단, 단일 평가/CFG/Source 증거의 상세가 동결되지 않았다.
- 제안 변경: 아래 prefix try·정확히 동일한 E·Copy 분기/조기 반환 subset을 동결한다.
- 변경 이유: P15의 verified tagged values와 P14 active-payload proof를 Error 전파에 연결한다.
- 영향 범위: AST/Parser/HIR/TypeChecker/const legality·MIR/검증·LLVM Adapter·compile/runtime tests.
  Lexer raw token/END·P15 생성 문맥·기존 숫자/Runtime 의미는 유지한다.
- 대안: expected T/E로 constructor 문맥을 자동 합성하거나 error 변환 protocol을 추가할 수 있다.
  이 안은 operand 문맥을 격리하고 E exact identity만 허용해 P15의 추론 경계를 유지한다.
- Backward Compatibility: 기존 승인 source와 ABI를 유지한다. 기존 try의 N1102 거부만 새 subset 진단으로 대체한다.

## 문법·결합·문맥

[전용 EBNF](GRAMMAR_STAGE_B_TRY.ebnf)는 P15 prefix_expr 한 production에 "try"만 추가한다.
추가 production 없이 **51개**다. 기존 Keyword::Try·END continues-after 규칙을 재사용한다.

1. `try expr`은 prefix expression이며 + / - / !와 같은 prefix 층에서 오른쪽으로 결합한다.
   postfix call·field/tuple selector·as가 먼저 결합하고 arithmetic/comparison/logical은 나중이다.
   `try leaf() + 1`은 `(try leaf()) + 1`, `try try nested()`는 `try (try nested())`다.
   `try leaf() as int8`은 `try (leaf() as int8)`이며 Result에 numeric cast를 시도해 N2101이다.
   Success 값에 cast/projection하려면 `(try leaf()) as int8` / `(try leaf()).field`처럼 괄호를 쓴다.
2. try operand는 완성된 intrinsic `Result<T,E>` 값이어야 한다.
   사용자 Enum의 Result/Success/Error lookalike와 Option·primitive operand는 N2101이다.
   nominal type 이름이나 local value shadow 대신 P15의 intrinsic family certificate로 구분한다.
3. 가장 안쪽 enclosing function은 완성된 intrinsic `Result<U,E>`를 반환해야 한다.
   main의 기존 Unit entry, Unit/Option/사용자 Enum 반환 함수는 N3002다.
   Source T와 function U는 서로 달라도 된다. Error E는 두 concrete 타입에서 정규화 후 정확히 같아야 한다.
   alias/nullable/동일 specialization은 같은 타입, 다른 nominal ID는 다르다.
   scalar widening·sum payload 변환·자동 error wrapping·사용자 error conversion은 없다.
4. operand의 기대 타입은 try 바깥의 binding/argument/return 문맥에서 유도하지 않는다.
   `let n:int8=try Result::Success(1)`도 operand가 P15의 완성된 기대 Result 문맥을 갖지 못해 N2103이다.
   먼저 `let r:Result<int8,Failure>=Result::Success(1)`을 쓰거나 typed 함수 반환을 사용한다.
   같은 함수의 반환 E만으로 Source T를 추측하지 않는다. 소스에 없는 generic constructor 인수도 추가하지 않는다.
5. Success branch의 try expression 자체 타입은 Source T다. 그 값이 바깥 문맥에 쓰일 때 기존 scalar coercion은 가능하다.
   `let n:int16=try sourceInt8()`은 int8 값을 얻은 뒤 int16으로 승격한다.
   이미 존재하는 aggregate/sum 값의 component widening이나 implicit Result 성공 wrapping은 없다.
   `(try sourceInt32()) as int8`은 기존 checked numeric cast다.

## 실행·flow·const

1. operand는 소스 위치에서 한 번 평가해 Copy snapshot으로 보관한다.
   Success는 active payload T를 독립 Copy 값으로 추출한다.
   Error는 active payload E를 복사해 enclosing `Result<U,E>::Error`를 생성하고 그 함수에서 즉시 반환한다.
   같은 E라도 Source Result와 destination Result specialization ID는 다를 수 있다.
   Error는 Panic/Abort가 아니며 실패를 builtin print로 출력하거나 process exit로 바꾸지 않는다.
2. enclosing function의 이후 statement와 미평가 sibling expression/argument는 Error 경로에서 실행하지 않는다.
   기존 left-to-right call/aggregate/interpolation 평가 순서와 Bool short-circuit를 유지한다.
   이전 sibling의 effect는 유지한다. Error는 loop의 break/continue가 아니라 함수 return이다.
   성공 branch 이후의 while/match/break/continue는 기존 의미를 따른다.
3. TypeChecker는 성공 branch가 계속 진행할 수 있는 것으로 flow를 검사한다.
   operand가 known Error const여도 try만으로 함수의 mandatory return을 충족한다고 추론하지 않는다.
   함수 normal path에 기존 explicit return/완전한 all-arm return이 필요하다. 누락은 N3003이다.
   성공 T가 Unit이면 `try ping()` expression statement가 가능하다.
   T 자체가 Result일 수 있으므로 중첩 try와 `return try sourceNested()`도 기존 정확한 반환 타입 검사에 따른다.
4. const initializer의 try는 runtime control flow이므로 실행하지 않고 N3201로 거부한다.
   primary는 try keyword, secondary는 const 선언이다. 함수 내부/전역·생략된 논리 RHS에서도 동일하다.
   const legality가 ordinary try의 enclosing-return 검사보다 우선하며 const에 N3002를 추가하지 않는다.
   operand의 독립 syntax/name/type 오류는 별도로 남을 수 있고 ErrorType 파생 오류는 억제한다.
   기존 정적 dependency/cycle N3202와 10,000-node 평가 예산은 유지한다. const try 성공값/node 평가를 추가하지 않는다.
5. 이번 Result payload는 모두 P15 Copy다. payload Move·Drop/borrow/cleanup protocol은 구현하지 않는다.
   기존 String parameter/temporary는 P03 실행 단위 arena로 유지하며 정상 main 종료 시 해제를 보존한다.
   조기 return에서 새 함수별 String Drop이나 arena reset을 삽입하지 않는다.
   이전 String temporary와 호출 effect가 살아 있는 Error 경로를 Native로 검사한다.

## MIR·Source·Native·자원 한도

- HIR try는 전체 expression Span·정확한 3-byte keyword Span·한 operand child와 원 AstId를 유지한다.
  외부 AST/HIR의 shape/keyword/source metadata를 독립 검증하며 UTF-8 truncation/복구에서 compiler panic이 없어야 한다.
- MIR은 try의 source/destination intrinsic Result ID·T/E·enclosing function ID·SourceInfo/keyword·snapshot과
  두 successor의 immutable certificate를 유지한다. 명시적 try 분기는 Success/Error active tag proof를 설정한다.
  기존 variant payload read·constructor·Return/private ABI를 재사용한다.
- validator는 operand가 한 번 snapshot되고, Success active payload가 올바른 T로 이어지며,
  Error active E가 정확한 destination Error로 복사되어 enclosing return으로 이어지는 것을 독립 확인한다.
  함수 외부 또는 다른 specialization으로 전파, tag/case retarget, error 값을 다른 같은 타입 값으로 바꾸기,
  receiver overwrite, unproven read, Error 경로를 정상 continuation으로 보내기, early-return 우회 및 source 변조를 거부한다.
  Source operand가 오류이면 LLVM에 도달하지 않는다.
- 완전 초기화/CFG reachability/return type과 P14/P15 certificate 검사를 유지한다.
  LLVM Adapter만 tag branch·Copy snapshot·payload·out-pointer return을 LLVM에 표현한다.
  Runtime API·public ABI·FFI·niche·main Result 반환을 추가하지 않는다.
- 기존 Parser max nesting 128, mixed aggregate depth 128·size 1 MiB·occurrence 65,536,
  intrinsic specialization 4,096와 사용자 Enum/Tuple 예산을 그대로 적용한다. 새 try 언어 한도는 추가하지 않는다.
  평평한 다수 try와 nested try의 분석/CFG/drop은 반복형 또는 기존 검증한 중첩 한도 내에서 처리한다.
- Windows Native debug/release 및 COFF/ELF O0/O2는 별도 실행 증거다. Linux Native host 실행은 후속이다.

## 진단·수용 기준

| 상황 | code·primary |
|---|---|
| operand 누락·잘못된 문법 | 기존 N1101/N1103, parser 원 위치 |
| 중첩 한도 초과 | 기존 P01 N1102 / loop 내부 P04 N8901, limit note |
| primitive/Option/user Enum operand | N2101, operand expression |
| enclosing function이 intrinsic Result를 반환하지 않음 | N3002, try keyword / function return annotation note |
| Error E가 다름 | N2101, try keyword / operand·function E annotation note |
| operand Result constructor 문맥 부족 | P15 N2103, constructor 전체 |
| 추출 T가 binding/argument/return/condition 문맥과 다름 | 기존 N2101/N3001, try expression 또는 사용 위치 |
| 정상 경로 return 누락 | 기존 N3003, function body |
| const try | N3201, try keyword / const 선언 |
| Source ErrorType | 파생 try 오류 억제, 원 syntax/name/type 진단 유지 |

[두 파일 수용 fixture](try-proposal-fixtures/README.md)는 import/alias·nested try·Unit·snapshot·short-circuit·loop와
성공/실패 effect order의 검증 stdout을 포함한다. [expected.json](try-proposal-fixtures/expected.json)은
부정 18사례의 검증 code·정확한 UTF-8 byte Span과 cascade 금지 기대값이다.
문서 validator는 승인 상태/grammar/metadata만 검사하며 Compiler/Native 실행 증거와 구분한다.

수용 검사는 parsing/결합/cast·UTF-8 truncation·bare-jump END, exact E/alias/nominal/shadow/context,
const legality/cycle·flow·자원 경계·Source/CFG/error-payload identity gate를 검사한다.
Native는 조기 실패의 이후 effect 부재·이전 String temporary·Unit/ZST·숫자 10종/Char/Bool/mixed Copy,
private factory, nested Result/try·while jump·all-return과 기존 전체 회귀를 debug/release에서 확인한다.

미포함: Option try·implicit error conversion·try block/catch/exception, String/Move Result payload,
사용자 Generic·일반 borrow/Drop·Array·메서드 API·public ABI/FFI·Result main.
2026-10-07 승인된 P16 subset을 Compiler와 accepted ledger에 적용했다.
현재 직접 실행할 P16/P15 예제와 명령은 [TESTING.md](../../TESTING.md)를 따른다.

## 초안 준비 당시 검증 — 2026-10-07

- 문서 build/validator PASS: 원본 hash·로컬 링크·Draft ledger·51개 production과 기존 50개 보존·부정 18사례 UTF-8 byte Span·제안 출력/cascade metadata.
- HEAD 대비 accepted P01~P15·D01~D05와 원본 148개 SHA-256이 동일하다. Compiler/Runtime/Cargo source 변경은 없다.
- 기존 `cargo test --workspace --offline`: **287 PASS / 0 FAIL / 49 ignored**.
  LLVM/Native opt-in 49개는 이번 문서 작업에서 재실행하지 않았다. P15의 기존 실행 증거는 [구현 기록](OPTION_RESULT_IMPLEMENTATION.md)을 따른다.
- `cargo fmt --check`, Runtime `rustfmt --check --edition 2021 crates/nova-cli/runtime/stage_a.rs`,
  `cargo clippy --workspace --all-targets --offline -- -D warnings`, `cargo check --workspace --all-features --offline` PASS.
- try가 없는 helper `try-proposal-fixtures/effects.nova`와 현재 P15 `examples/option_result.nova`의 check는 exit 0이다.
  이는 초안 작성 당시의 기록이다. 이후 P16 main/부정 진단/Native 실행 증거는 [구현 기록](TRY_IMPLEMENTATION.md)을 따른다.
