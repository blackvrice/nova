# Stage B 함수 이름 인수 최소 계약 — P17

작성일: 2026-10-07. 상태: **Draft / 사용자 승인 대기 / 미구현**.
기존 D01~D05/P01~P16·Canonical·원본 148개 문서를 보존한다.
전체 D11/D16/D25/D30 승인이 아니다. 승인 전에 Compiler에 적용하지 않는다.

## Specification Change Proposal

- 관련 문서: [Canonical](CANONICAL.md), [원본 함수](../04_Functions_Control/NOVA-035_함수_메서드_호출_규약_Receiver_사양서.md),
  [원본 인수](../04_Functions_Control/NOVA-036_위치_이름_기본_인수_사양서.md),
  [P02](SEMANTICS_STAGE_A_PROPOSAL.md), [P11 Module](MODULE_STAGE_B_PROPOSAL.md),
  [P12 생성](STRUCT_STAGE_B_PROPOSAL.md), [P15](OPTION_RESULT_STAGE_B_PROPOSAL.md),
  [P16 try](TRY_STAGE_B_PROPOSAL.md), [결정](DECISIONS.md).
- 현재 확정 의미: 인수는 소스 순서로 평가한다. 현재 함수 호출은 위치 인수만 지원한다.
  원본은 이름 인수를 테스트 대상으로 정하지만 이름의 대응·중복·혼합·미지원 callee 진단을 구체화하지 않는다.
- 발견된 문제: 호출 인수를 선언 순서로 재배열해 평가하면 effect와 try 조기 반환 의미가 바뀐다.
  기존 TypeChecker의 위치별 expected type과 MIR의 위치별 전달을 그대로 사용할 수 없다.
- 제안 변경: `label: expression`을 사용자 함수 호출에서 허용하고, 매개변수 이름 대응과
  source-order 평가 후 parameter-order 전달을 아래 subset으로 동결한다.
- 변경 이유: Stage B 함수 호출을 확장하면서 기존 타입 문맥·Copy ABI·String arena·try의 실행 순서를 보존한다.
- 영향 범위: EBNF·AST/Parser·HIR/source 검증·함수 signature/typed call mapping·MIR/검증·compile/runtime tests.
  Lexer keyword/END와 Runtime API, LLVM ABI는 유지한다. Adapter는 검증된 기존 Call을 소비한다.
- Backward Compatibility: 위치 인수 프로그램과 진단·내부 ABI를 보존한다. 이름 인수에서만 새 대응 규칙을 적용한다.
  함수 매개변수 이름 변경은 해당 이름 호출의 source compatibility에 영향을 준다.
- 대안: 모든 이름 인수를 선언 순서로만 허용하거나 기본 인수·overload까지 함께 구현할 수 있다.
  전자는 이름 인수의 재배열을 제한하고 후자는 이번 최소 범위를 넘어선다.

## 문법·대상·이름 대응

[전용 EBNF](GRAMMAR_STAGE_B_NAMED_ARGUMENTS.ebnf)는 P16 `arguments`만 바꾸고 `argument`를 추가한다.
기존 50개 production을 그대로 보존하며 총 **52개**다. parameter 문법은 바꾸지 않는다.

```nova
func join(left:int8, right:int16)->int32 { return (left as int32)*100+(right as int32) }
let a=join(right:200, left:7)
let b=join(1, right:2)
```

1. label은 기존 Unicode XID IDENT token이며 `:` 다음에 일반 expression을 쓴다.
   정확한 식별자 철자를 사용한다. 대소문자·Unicode 정규화 정책은 기존 D02를 유지한다.
   label은 값 참조가 아니며 local/global value 이름 해석이나 shadow의 대상이 아니다.
   매개변수 이름을 매핑한 뒤 lexical DefId/parameter index로 변환한다.
2. resolved callee가 사용자 `func`인 경우만 이름 인수를 허용한다.
   forward/recursive 함수, import/as alias, 괄호로 묶은 direct callee와 사용자 함수 `print`에도 동일하다.
   label은 import alias의 철자가 아니라 원 함수 선언의 parameter 이름이다.
   기존 값 shadow·접근 제한·이름 해석은 그대로 적용하며 이름 인수로 overload를 새로 선택하지 않는다.
3. 위치 인수는 앞에서부터 선언의 0, 1, … parameter를 채운다.
   그 뒤 이름 인수는 미충족 parameter를 어떤 순서로든 채울 수 있다.
   첫 이름 인수 뒤 위치 인수는 N2201이다. `_`도 일반 parameter 이름일 때만 대응한다.
   external/internal label 분리, 이름 생략 shorthand, spread, keyword label, qualified label은 없다.
4. 모든 parameter는 정확히 한 번 채워야 한다. 알 수 없는 label, label 중복,
   위치 인수로 이미 채운 parameter의 label 중복, 누락·초과는 N2201이다.
   source order로 대응을 검사한다. 잘못된 대응이 하나라도 있으면 call은 ErrorType이며 Codegen을 막는다.
5. builtin `print`, Struct 위치 생성, 사용자 Enum variant, intrinsic Option/Result constructor는
   기존 위치 인수 전용이다. label이 있으면 N2201로 첫 label을 표시한다.
   사용자 함수 `print(value:string)`는 기존 P02 shadow 규칙에 따라 `print(value:"x")`가 가능하다.
   tuple literal·match payload binder는 call argument가 아니므로 이름 인수 문법을 추가하지 않는다.
6. 개행·trailing comma·보간 안 호출·delimiter recovery는 기존 D01~D05/P01을 유지한다.
   새 token이나 contextual keyword, END 합성 변경은 없다.
   colon 뒤 expression 누락은 기존 N1101, keyword label 등 미지원/잘못된 구문은 기존 parser 진단이다.

## 타입 문맥·const·실행

1. 인수 expression은 대응한 parameter의 타입과 annotation Span을 기대 문맥으로 받는다.
   선언 순서의 ordinal이 아닌 mapping을 적용한다. 숫자 literal 범위/승격·tuple/struct/sum 생성 문맥은
   기존 P07~P15를 그대로 사용한다. 이미 완성된 aggregate/sum 값의 component widening은 추가하지 않는다.
   int8 parameter로 대응한 `128`은 N2102, 잘못된 타입은 expression 위치 N2101이다.
2. 잘못된 label의 expression도 독립 syntax/name/type 오류를 검사하되 임의 parameter 타입을 주지 않는다.
   mapping 오류 때문에 N2101/N2102를 만들어내지 않는다. 기존 callee 자체가 ErrorType이면
   파생 N2201/non-callable 진단을 억제하고 인수의 독립 오류는 남긴다.
   올바른 callee의 private 접근 N2004와 독립 인수 오류는 기존 검사를 따른다.
3. caller는 각 제공 expression을 소스 순서로 정확히 한 번 평가하고 결과를 저장한다.
   모든 인수가 성공한 뒤 저장한 값을 parameter order로 전달해 함수를 한 번 호출한다.
   Copy 값은 독립 snapshot이며 String은 기존 P03 실행 arena의 값/수명을 유지한다.
   새 borrow/change/take·Drop·함수별 String cleanup·callee-side default 평가는 없다.
4. 인수 안 `try` Error는 enclosing caller에서 즉시 반환한다. 앞선 인수의 effect는 유지하고
   뒤 인수·callee 본문·후속 statement는 실행하지 않는다. 이름 인수 재배열은 이 순서를 바꾸지 않는다.
   nested call·short-circuit·while/match/return은 P04/P14/P16의 CFG 의미를 따른다.
5. 함수 호출은 이름 인수 여부와 무관하게 기존 const initializer에서 N3201이다.
   primary는 call 전체, secondary는 const 선언이다. static skipped RHS도 기존 legality 검사를 따른다.
   Struct/Enum/sum의 기존 위치 const 생성은 유지한다. named constructor의 const 허용은 추가하지 않는다.
   매개변수 기본값 선언, 생략된 인수 평가, const function은 이번 범위 밖이다.

## 진단·Source·MIR 검증·자원

| 상황 | code·primary / secondary |
|---|---|
| 알 수 없는 이름 | N2201, 해당 label / callee 선언과 알려진 parameter note |
| 같은 parameter 두 번 충족 | N2201, 두 번째 label / 첫 인수와 parameter 선언 |
| 이름 뒤 위치 인수 | N2201, 뒤 위치 expression 전체 / 첫 label |
| mixed call의 초과 위치 인수 | N2201, 초과 expression 전체 / callee 선언 |
| 이름 call의 parameter 누락 | N2201, call 전체 / 누락 parameter 선언과 이름 note |
| builtin/constructor에 이름 인수 | N2201, 첫 label / 위치 인수 전용 note |
| 인수 타입/범위 오류 | 기존 N2101/N2102, expression/literal / 대응 parameter annotation |
| undefined/non-callable callee | 기존 N2001/N2101, callee / 파생 mapping 오류 억제 |
| const 함수 호출 | 기존 N3201, call 전체 / const 선언 |
| 순수 위치 인수 count 오류 | 기존 N2201 call 전체와 기존 진단 유지 |

- 잘못된 label/순서/초과에 source order로 진단한다. 이미 대응 오류를 보고한 call에
  파생 누락 N2201은 추가하지 않는다. 대응 오류 없는 불완전 call에만 누락 N2201 한 개를 보고한다.
  type mismatch는 유효하게 대응한 인수에만 발생하며 ErrorType 파생 오류는 억제한다.
- AST/HIR은 source-order argument expression ID, optional label identifier Span·colon Span,
  전체 call Span·원 AstId를 보존한다. label count/child count, 정확한 IDENT 철자와 `:` 원문,
  UTF-8 경계·동일 파일·argument 안 포함/순서를 검증한다. labels를 child value 이름으로 해석하지 않는다.
- typed mapping은 원 callee DefId·parameter index·argument SourceInfo를 가진다.
  MIR lowering 입구는 mapping completeness/uniqueness/bounds·signature identity를 검증한다.
  call마다 source-order snapshot local과 parameter-order Operand를 명시한다.
  기존 Caller/Callee 타입·Copy aggregate ABI·String descriptor·Call/Return은 재사용한다.
- private provenance는 callee identity·source argument identity/order·mapping·snapshot writes·Call의
  arguments/destination/continuation과 평가 CFG를 결합한다. validator는 동일 타입 인수를 바꿔 전달하기,
  effect call 재배열/복제/생략, snapshot overwrite, 잘못된 매핑·source/label 변조,
  try Error 경로에서 뒤 인수/callee 실행, 초기화 우회와 잘못된 callee를 거부해야 한다.
  단순 arity/type 검증만으로 전달 순서의 정확성을 증명했다고 보고하지 않는다.
- 원본 Span을 LLVM까지 유지하고 Core에 LLVM 타입을 도입하지 않는다.
  existing pure positional ABI/진단/snapshot과 validator gate는 보존한다.
- 새 언어 한도는 추가하지 않는다. label 대응은 symbol lookup으로 인수+parameter 수에 선형으로 구축하고,
  nested expression은 기존 parser max nesting 128 및 loop 내부 P04 한도를 따른다.
  module 1,024·aggregate/specialization/const 예산도 유지한다. 대량 평평한 이름 인수에서
  host stack recursion·quadratic label scanning을 피하고 UTF-8 truncation/recovery 결정성을 검사한다.

## 수용 기준·검토 자료

[두 파일 fixture](named-arguments-proposal-fixtures/README.md)·[기대값](named-arguments-proposal-fixtures/expected.json)은
import alias·source-order effect·mixed arguments·Unicode label·mapped literal/Option 문맥·Copy aggregate·String·Unit·
try 성공/실패·private 이름 대응·constructor 거부와 cascade 억제의 **제안** 데이터다.
Draft이고 `implementation_verified:false`이며 현재 compiler의 pass 결과가 아니다.

구현 후 검증할 항목:

- Parser AST/HIR dump와 label/colon byte Span, trailing comma·보간·중첩·UTF-8 truncation/복구.
- forward/import/recursive/grouped/direct call·shadow·private·label/value namespace·완전 대응.
- remapped numeric boundary·nullable/Result·Tuple/Struct/Enum·String/Unit 문맥과 const legality.
- forged typed mapping 및 MIR 동일 타입 swap·source/CFG·effect 중복·try continuation 변조 gate.
- 기존 전체 workspace 회귀와 fmt/clippy/all-features; 실제 LLVM COFF/ELF O0/O2 객체 검증.
- Windows Native debug/release의 정확한 stdout/LF/stderr/exit와 effect order·실패 시 미실행.
  Linux host Native 실행은 기존처럼 후속이며 object emission과 구분한다.

미포함: 기본 인수·외부 label 문법·overload·메서드/receiver·사용자 Generic·함수 값·Closure,
named Struct/Enum/Option/Result 생성·Array·일반 Move/borrow/Drop·public ABI/FFI와 전체 D11/D16/D25/D30.
승인 후 P17 subset만 Compiler·accepted ledger에 적용한다. 현재 실행 명령은 [TESTING.md](../../TESTING.md)를 따른다.

## 초안 준비 검증 — 2026-10-07

- 문서 build/validator PASS: 148개 원본 hash·로컬 링크·Draft ledger·52개 production과 기존 50개 보존·
  정상 2/부정 16사례·UTF-8 byte Span·cascade 금지·제안 24줄 stdout metadata를 검사했다.
- HEAD 대비 accepted D01~D05/P01~P16와 원본 148개 SHA-256이 동일하다. Compiler/Runtime/Cargo 변경은 없다.
- 기존 `cargo test --workspace --offline`: **297 PASS / 0 FAIL / 52 ignored**.
  기존 LLVM/Native opt-in 52개는 이번 문서 작업에서 재실행하지 않았다.
  기존 기능의 전체 실행 증거는 [P16 구현 기록](TRY_IMPLEMENTATION.md)을 따른다.
- `cargo fmt --check`, Runtime `rustfmt --check --edition 2021 crates/nova-cli/runtime/stage_a.rs`,
  `cargo clippy --workspace --all-targets --offline -- -D warnings`,
  `cargo check --workspace --all-features --offline` PASS.
- 이름 인수가 없는 helpers와 기존 P16 두 파일 main의 check는 exit 0이다.
  helper 초안의 `String` 표기를 기존 승인 source 타입 `string`으로 정정한 뒤 재검사했다.
- ignored `target/`에서 P16 위치 인수와 명시적 temporaries만 사용한 대조 프로그램의 check 및
  Windows Native debug/release를 실행했다. check 출력 없음·exit 0, Native는 제안과 같은 24줄 LF·stderr 없음·exit 0이다.
  이는 기존 연산·출력 계산의 대조 검증이며 **P17 이름 인수 source의 compile/Native 통과 증거가 아니다**.
  tracked 이름 인수 fixture는 Draft/`implementation_verified:false`로 유지한다.
