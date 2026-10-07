# Stage B 상수 표현식 함수 기본 인수 최소 계약 — P18

작성일/승인일/구현일: 2026-10-07. 상태: **Accepted / 사용자 승인 / 구현 완료**.
사용자 “P18 승인하고 함수 기본 인수 구현 진행” 답변으로 승인했다. [구현·검증 기록](DEFAULT_ARGUMENTS_IMPLEMENTATION.md)을 따른다.
기존 D01~D05/P01~P17·Canonical·원본 148개를 보존한다.
전체 D09/D11/D16/D25/D30 승인이 아니다. 아래 P18 subset만 Compiler에 적용했다.

## Specification Change Proposal

- 관련 문서: [Canonical](CANONICAL.md), [원본 함수](../04_Functions_Control/NOVA-035_함수_메서드_호출_규약_Receiver_사양서.md),
  [원본 인수](../04_Functions_Control/NOVA-036_위치_이름_기본_인수_사양서.md),
  [원본 const](../03_Types_Declarations/NOVA-032_상수_표현식_const_평가_사양서.md),
  [P05/P06](GLOBAL_CONST_STAGE_B_PROPOSAL.md), [P11 Module](MODULE_STAGE_B_PROPOSAL.md),
  [P15](OPTION_RESULT_STAGE_B_PROPOSAL.md), [P16 try](TRY_STAGE_B_PROPOSAL.md),
  [P17 이름 인수](NAMED_ARGUMENTS_STAGE_B_PROPOSAL.md), [결정](DECISIONS.md).
- 변경 전 사양: 기본 인수는 caller에서 평가한다. P17은 모든 parameter를 정확히 한 번 제공해야 하며 기본값을 포함하지 않는다.
  기본값 선언 문법·이름 scope·생략 순서·허용 표현식·실패/예산의 구체 계약은 아직 승인되지 않았다.
- 발견된 문제: optional parameter를 callee 내부에서 채우면 기존 caller-side 규칙과 ABI를 바꾼다.
  호출자의 local 이름으로 default를 다시 해석하면 import/private/shadow에 따라 값이 달라진다.
  arbitrary runtime default는 effect·parameter 참조·try enclosing function/cleanup의 추가 계약이 필요하다.
- 제안 변경: `name: Type = expression`을 사용자 함수 parameter에 허용한다.
  첫 subset은 기존 const evaluator가 지원하는 상수 표현식으로 제한한다. 선언 module scope에서 한 번 검사·계산하고,
  caller는 제공 인수의 평가·snapshot 후 빠진 default 값을 declaration order로 materialize하고 완전한 인수를 전달한다.
- 변경 이유: P17의 이름 대응을 확장하면서 기존 type context·checked const·Copy/String 표현·try·private ABI를 보존한다.
- 영향 범위: EBNF·AST/Parser·HIR/Source·Resolver declaration scope·signature/default value 검사·typed call plan·
  const 평가·MIR/검증·Compiler/Native tests. Lexer/END·LLVM/Runtime API·public ABI는 변경하지 않는다.
- Backward Compatibility: 기본값 없는 P01~P17 함수의 동작/진단을 보존한다.
  `=`는 현재 unsupported인 parameter 위치에서만 추가된다. 기존 const의 허용성/예산은 바꾸지 않는다.
  기본값 변경은 호출을 재컴파일할 때 반영된다. 별도 컴파일·ABI versioning·cache invalidation 구현은 이번 범위가 아니다.
- 대안: 일반 runtime default·앞 parameter 참조·메서드·overload까지 함께 동결한다.
  이번에는 상수 표현식 default만 제공하며 나머지는 후속 제안으로 남긴다.

## 선언·문법·대상

[전용 EBNF](GRAMMAR_STAGE_B_DEFAULT_ARGUMENTS.ebnf)는 P17 `parameter` 한 production만 확장한다.
나머지 **51개 production을 동일하게 유지하며 총 52개**다. 새 keyword/token/END 규칙은 없다.

```nova
func join(left:int8=6,right:int16=20)->int32 { return (left as int32)*100+(right as int32) }
func holes(a:int8=4,b:int8,c:int8=6)->int32 { return (a as int32)*100+(b as int32)*10+(c as int32) }
let a=join()               // 620
let b=join(right:2)        // 602
let c=join(1)              // 120
let d=holes(b:5)           // 456
```

1. typed ordinary user `func` parameter에만 `= expression`을 추가한다. type annotation은 필수다.
   default 없는 parameter와 default 있는 parameter를 어느 선언 순서로도 혼합할 수 있다.
   `holes(5)`는 첫 a를 채우며 b는 여전히 누락이다. 위치 인수의 빈 슬롯·spread·외부 label 분리는 없다.
2. forward/recursive/import alias/grouped direct callee·사용자 print shadow에도 같은 규칙이다.
   default는 원 function signature의 parameter index에 귀속하며 alias/caller의 이름으로 다시 선언하지 않는다.
   builtin print·Struct/Enum/Option/Result constructor·Tuple literal에는 default나 named constructor를 추가하지 않는다.
3. 필드/variant/let의 초기화 문법은 기존 계약을 따른다. 명시적 init·method/receiver·overload·사용자 Generic은 후속이다.
   default 있는 parameter도 parameter 수에 포함하므로 Native entry main은 기존대로 parameterless/Unit이어야 한다.
4. multiline/trailing comma/보간/중첩 및 잘못된 입력의 복구는 기존 D01~D05/P01을 따른다.
   `=` 뒤 expression 누락은 기존 N1101이다. 기본값 없는 AST/HIR/debug dump의 기존 정보를 보존한다.

## 선언 scope·type context·상수 평가

1. default expression의 value/type 이름은 **해당 함수가 선언된 module의 top-level scope**에서 해석한다.
   같은 module 전역 const·type/function 이름·P11 direct import/alias와 declaration-site private 접근을 사용한다.
   자기 함수의 parameter·body local·호출자 local은 이 scope에 없다. module global과 parameter의 이름이 같으면 global을 참조한다.
   없는 이름은 N2001이다. caller가 같은 철자로 shadow해도 default의 DefId/value는 바뀌지 않는다.
2. 전역 const의 기존 forward reference·dependency/cycle·cross-file 규칙을 유지한다.
   default 자체는 global const declaration도 dependency vertex도 아니다. 함수 호출이 const에서 금지되므로
   default→function-call→default 재귀 계산을 만들지 않는다. global const 실패를 파생 N3201로 다시 보고하지 않는다.
3. default는 해당 parameter annotation을 expected type/secondary로 받는다. 기존 숫자 literal 범위·lossless 승격·
   explicit checked cast·Bool/Char/Unit/String·Copy Tuple/Struct/Enum·Option/Result/none 문맥을 따른다.
   완성된 aggregate component widening·String field/payload·새 cast·타입 추론 정책은 추가하지 않는다.
4. 허용 표현식은 **현재 승인된 P05~P15 const subset**과 같다: literal/name/group, 기존 scalar unary/binary/cast,
   Copy tuple·위치 aggregate/sum 생성·projection, 성공한 global const 참조다.
   String은 기존 decoded literal/상수 참조/그룹만 허용한다. String 보간/concat·사용자 함수/print 호출·try·
   parameter/local runtime 값·const function·named constructor는 허용하지 않는다.
   function/type 이름은 기존 callee/type 문맥에서만 사용하며 first-class function을 추가하지 않는다.
5. 모든 default를 사용 여부와 무관하게 declaration당 한 번 typecheck·legality 검사·상수 평가한다.
   함수가 호출되지 않거나 모든 호출이 해당 값을 override해도 잘못된 default는 오류다.
   literal/type/name 오류를 먼저 보존하고 ErrorType의 파생 상수 평가 오류를 억제한다.
6. legality는 생략된 logical RHS까지 검사하고 실제 값 계산은 기존 Bool short-circuit를 따른다.
   `false && predicate()`는 N3201, `false && (1/0==0)`는 false로 성공한다.
   checked integer/cast 실패는 compile-time N3201이다. float의 IEEE/NaN/finite narrowing 의미는 기존 P09/P10 그대로다.
7. default initializer당 기존 **10,000-node** 예산을 새로 시작한다. root·expression·생략 RHS를 센다.
   constructor head/type syntax/projection field spelling은 기존 evaluator처럼 별도 expression으로 세지 않는다.
   cached const reference는 1 node다. 10,001번째 preorder node의 Span에 N3202를 보고한다.
   parameter/type annotation은 예산에서 제외한다. 허용성/예산의 첫 실패 후 계산을 중단한다.
   global cycle와 parser 128/module 1,024/aggregate·specialization 한도는 기존 계약을 유지한다.
8. default 안 try는 keyword N3201, secondary parameter 선언이며 enclosing return N3002를 파생하지 않는다.
   이는 default가 상수 문맥이라는 규칙이다. 호출자가 **명시적으로 제공한 인수**의 try는 기존 P16 runtime 의미를 유지한다.

## 생략 인수 대응·caller 실행

1. P17대로 leading positional을 0,1,… parameter에 대응하고 이후 named를 정확한 이름으로 대응한다.
   unknown/duplicate/positional collision/positional-after-named·excess 규칙을 유지한다.
   제공된 argument는 정확히 한 번만 채우며 default가 duplicate/잘못된 label을 숨기지 않는다.
2. 제공 인수 대응이 성공한 뒤 미충족 parameter를 declaration order로 확인한다.
   default가 있으면 그 declaration의 precomputed typed value로 채운다. default 없는 첫 미충족 parameter만
   N2201 call 전체/누락 parameter secondary로 보고한다. 이미 mapping 오류가 있으면 파생 missing 오류를 추가하지 않는다.
3. 모든 parameter가 default인 `f()`와 부분 positional/named 생략 호출을 허용한다.
   기본값 없는 함수의 count/mapping 진단은 P17 그대로다. default 있는 순수 positional 초과는 기존 count N2201 call 전체,
   이름 호출의 excess positional은 기존 P17 해당 expression Span을 유지한다.
4. caller는 제공 인수를 **소스 순서로 정확히 한 번** 평가·변환·snapshot한다.
   다음으로 생략된 default 값을 **parameter 선언 순서**로 caller MIR의 temporaries에 materialize한다.
   모든 값을 완성한 후 parameter order로 기존 Call을 한 번 실행한다. callee는 항상 완전한 기존 signature/ABI를 받는다.
   default는 compiler 상수이므로 runtime effect·산술·호출이 없다. caller-side constant folding/materialization이며
   callee prologue에서 default를 실행하거나 생략 표시 ABI를 추가하는 방식은 아니다.
5. override한 default는 runtime materialize하지 않는다. 제공 인수 안 try Error는 앞선 effect를 유지하고
   뒤 제공 인수·default materialization·callee·후속 statement를 실행하지 않는다.
   enclosing short-circuit가 Call을 건너뛰면 제공 인수/default/callee 모두 실행하지 않는다.
6. Copy 값은 독립 snapshot이고 String default는 기존 immutable literal/arena 표현과 수명을 따른다.
   함수별 cleanup·일반 Move/borrow/Drop·default-side effect/parameter 참조는 이번 범위 밖이다.
7. `const x=f()`는 f의 모든 parameter가 default여도 기존 **전체 Call N3201**이다.
   함수 자체를 const function으로 바꾸거나 const에서 caller default를 평가해 함수를 호출하지 않는다.

## 진단·Source·MIR 검증

| 상황 | code·primary / secondary |
|---|---|
| initializer syntax 누락 | 기존 N1101, 누락 expression 위치 |
| default 안 undefined name | 기존 N2001, name / module scope note |
| default type/literal 범위 오류 | 기존 N2101/N2102, expression/literal / 해당 parameter annotation |
| 금지된 default call/보간 | N3201, 해당 expression 전체 / parameter 선언 |
| default try | N3201, try keyword / parameter 선언; 파생 N3002 억제 |
| default checked 평가 실패 | N3201, 실패 operation / parameter 선언 |
| default node budget | N3202, 10,001번째 node / parameter 선언과 limit note |
| global const cycle/failed reference | 기존 P06 cycle/name/type 진단; 파생 default 평가 오류 억제 |
| default 없는 parameter 누락 | N2201, Call 전체 / 첫 누락 parameter와 name note |
| unknown/duplicate/order/excess·미지원 constructor label | 기존 P17 code·Span·secondary |
| const 함수 호출 | 기존 N3201, Call 전체 / const 선언 |

새 code는 추가하지 않는다. N3201/N3202 상수 진단의 declaration secondary 대상에 parameter를 추가하는 subset 제안이다.
선언 default 오류는 한 번 보고하며 각 caller에서 동일한 default 실패를 중복 보고하지 않는다.
독립 인수 오류는 계속 검사하고 실패 값/불완전 call plan은 MIR/Adapter에 들어가지 않는다.

- AST/HIR parameter는 기존 name/type/원 AstId에 optional `=` Span·initializer HirId/Span을 연결한다.
  type child를 먼저 유지하고 default expression을 추가한다. 정확한 identifier/`=` spelling·trivia gap·
  child 순서/shape·동일 file·parent 포함/UTF-8 경계를 외부 AST lowering에서 검증한다.
  Call AST에는 제공 인수만 source order로 보존한다. 존재하지 않는 default caller AST를 생성하지 않는다.
- typed default metadata는 원 function/parameter identity·index·initializer SourceInfo·선언/annotation Span·ConstValue와
  성공/실패 상태를 보존한다. call plan은 제공 argument→parameter mapping과 빠진 default의 declaration identity/index를 구분한다.
  반드시 모든 slot을 한 번 채우고 callee signature/type와 일치해야 한다. 기본값 없는 P17 bijection은 보존한다.
- MIR의 제공 인수 snapshot SourceInfo는 caller의 원 expression이다. default materialization SourceInfo는 선언 module의
  원 initializer다. Call/continuation의 SourceInfo는 caller Call이다. 서로 다른 FileId를 caller Span으로 위조하지 않는다.
  두 파일 source origin·parameter declaration·caller use 관계를 private provenance로 보존한다.
- validator는 typed/default value 위조·다른 parameter/default origin·같은 타입 값 swap·누락/중복/default 순서 변경·
  explicit override 후 default write·snapshot overwrite·effect 재배열/복제·try early return 우회·잘못된 callee/CFG를 거부한다.
  단순 arity/type 검사만으로 mapping/default 정합성을 증명하지 않는다. Adapter는 검증된 기존 완전한 Call을 소비한다.
- 구현은 iterative 작업과 signature lookup을 사용한다. default declaration은 caller 수만큼 재평가하지 않는다.
  새 parameter/argument/call 한도는 추가하지 않는다. 대량 평평한 default/생략 호출은 작은 host stack에서 검증하며
  파일 전체 CPU/memory 상한을 약속하지 않는다. 새로운 cache/query/optimizer pass를 미리 구현하지 않는다.

## 수용 기준·검토 자료

[두 파일 fixture](default-arguments-proposal-fixtures/README.md)·[기대값](default-arguments-proposal-fixtures/expected.json)은
**검증된 수용 데이터**이며 Accepted / `implementation_verified:true`다. Compiler와 Windows Native O0/O2로 확인했다.

- Parser/HIR: type/default child 순서·`=`/initializer Span·trailing comma·nested type `>=` split·보간·UTF-8 truncation/recovery·외부 AST 위조.
- scope/type/const: declaration-module private global/import alias·forward global·같은 철자의 parameter/caller shadow·필수/default 혼합·
  numeric 10종/Bool/Char/Unit/String/Copy aggregate·Option/Result/none·cast·skipped legality·global cycle·10,000/10,001 경계.
- call: all-default/partial/leading positional+named·완전 override·forward/recursive/grouped·사용자 print shadow·missing/duplicate/unknown/order/excess.
- MIR: 원 initializer/source/file·parameter-order 완전 전달·materialization declaration order·같은 타입 swap·typed/CFG/effect/default 값 위조 gate.
- CLI: 정상 두 파일·별도 정상 2개·부정 20개 exact UTF-8 primary Span/cascade·invalid check/build/run 도구 미실행·기존 output 보존.
- 회귀: 기존 전체 workspace/fmt/clippy/all-features. 실제 LLVM Windows COFF/Linux ELF O0/O2와
  Windows Native debug/release의 제안 20줄 stdout/LF/stderr/exit·provided effect/try·String/Copy 수명·overflow Span.
  const default는 effect가 없으므로 20줄 출력만으로 default write의 CFG 순서를 증명하지 않고 MIR 위조 검사를 함께 수행한다.

미포함: runtime/side-effect default·parameter 의존 default·const function·named constructor·overload·method/receiver·
사용자 Generic·Array·일반 Move/borrow/Drop·공용 ABI/FFI·별도 컴파일/cache 제품화와 전체 D09/D11/D16/D25/D30.
승인한 P18 subset만 Compiler·accepted ledger에 적용했다. 현재 사용 가능한 명령은 [TESTING.md](../../TESTING.md)를 따른다.

## 초안 준비 당시 검증 — 2026-10-07 (과거 기록)

- 문서 build/validator PASS: 148개 원본 hash·로컬 링크·Draft ledger·52-production EBNF와 기존 51개 보존·
  정상 2/부정 20사례의 UTF-8 byte Span·cascade 금지·제안 20줄 stdout metadata를 검사했다.
- HEAD 대비 accepted D01~D05/P01~P17 ledger·승인 문서/grammar·Canonical과 원본 148개 SHA-256을 보존했다.
  Compiler/Runtime/Cargo source 변경은 없다.
- 기존 `cargo test --workspace --offline`: **306 PASS / 0 FAIL / 55 ignored**.
  이번 문서 작업에서 실제 LLVM/Native opt-in 55개 전체를 재실행하지 않았다.
  기존 기능의 전체 실행 증거는 [P17 구현 기록](NAMED_ARGUMENTS_IMPLEMENTATION.md)을 따른다.
- `cargo fmt --check`, Runtime `rustfmt --check --edition 2021 crates/nova-cli/runtime/stage_a.rs`,
  `cargo clippy --workspace --all-targets --offline -- -D warnings`,
  `cargo check --workspace --all-features --offline` PASS.
- ignored target의 **기존 P17 문법/명시적 인수만 사용하는 대조 프로그램**을 check 및 Windows Native debug/release로 실행했다.
  check 출력 없음·exit 0, 두 Native 실행은 제안값과 같은 20줄 UTF-8/LF·빈 stderr·exit 0이었다.
  이는 기존 연산·출력 값의 검산이며 **P18 default source의 compile/Native 통과나 새 declaration-scope/생략 의미의 구현 증거가 아니다**.
  당시 tracked P18 fixture는 Draft/`implementation_verified:false`였다. 현재 검증은 [구현 기록](DEFAULT_ARGUMENTS_IMPLEMENTATION.md)을 따른다.
- 문서 fixture 준비 중 함수 선언 `g()`/`predicate()`와 default 안 호출의 철자가 같은 두 기대 Span은
  실제 제안 오류 대상인 default 안 호출 위치로 정리했다. Compiler 정책 변경은 아니다.
