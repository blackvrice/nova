# Stage B 고정 폭 정수 타입·손실 없는 승격 착수안 — P07

작성일: 2026-10-04. 상태: **Draft / 사용자 승인 대기**.
이 문서는 검토용 제안이며 Compiler에 적용하지 않았다.
[P02](SEMANTICS_STAGE_A_PROPOSAL.md)/[P03](NATIVE_STAGE_A_PROPOSAL.md)의 Int32와
[P04](CONTROL_STAGE_B_PROPOSAL.md)~[P06](GLOBAL_CONST_STAGE_B_PROPOSAL.md)의 기존 동작을 보존한다.
D07의 정수 부분만 구체화하며 전체 Primitive/숫자 모델·Stage B의 승인안은 아니다.

## Specification Change Proposal

- 관련 문서: [Canonical 타입/alias NOVA-004](../00_Governance/NOVA-004_Nova_용어_키워드_Canonical_표.md),
  [타입/변환 NOVA-025](../03_Types_Declarations/NOVA-025_타입_시스템_타입_추론_형변환_사양서.md),
  [Primitive/Literal NOVA-026](../03_Types_Declarations/NOVA-026_Primitive_Literal_기본_타입_결정_사양서.md),
  [D07](DECISIONS.md), [숫자 초안](NUMERIC_RULES.md), [현재 전역 const](GLOBAL_CONST_IMPLEMENTATION.md).
- 현재 사양: 고정 폭 signed/unsigned 타입과 byte/int/uint alias, 양방향 검사와 제한적인
  lossless numeric widening은 Canonical/원본 기준이다. 현재 Compiler는 Int32만 검사·평가·실행한다.
  폭별 literal 문맥과 혼합 연산·checked 산술·MIR 변환·Runtime 출력의 상세는 미동결이다.
- 발견된 문제: 타입 이름만 수용하면 모든 계산을 Int32로 수행하거나 host integer 폭에 의존하게 된다.
  타입 승격을 LLVM에 맡기거나 작은 현재 값만 보고 narrowing을 허용하면 const/runtime 의미가 달라진다.
- 제안 변경: 아래 8종 정수, 기대 타입/리터럴 전용 부분식 문맥, 전체 범위 기반 widening과
  공통 정수 타입을 구현한다. 폭·부호에 맞는 checked 산술, const, 명시적인 내부 MIR 변환과 보간을 연결한다.
- 변경 이유: aggregate/module 전에 기본 scalar의 타입·값·변환·실행 의미를 일치시키고
  기존 const/제어 흐름/Native 기반을 재사용한다.
- 영향 범위: nova-types, TypeChecker의 literal/type/coercion metadata, const engine,
  MIR Constant/변환/validator, Codegen Interface의 지원 타입, LLVM Adapter, private Runtime과 tests·문서.
  기존 integer token·숫자 철자·END·AST/Parser production은 그대로 사용한다.
- Backward Compatibility: 기존 Int32 프로그램의 default·범위·signed MIN·checked 실패·출력은 유지한다.
  지금까지 unsupported인 다른 정수 타입과 lossless 변환을 추가한다. 타입 없는 큰 literal을 자동 승격하지 않는다.
  내부 Rust type/value/typed table API는 정수 종류·폭·변환을 표현하도록 확장할 수 있으며
  이 단계에서 전체 Compiler API/FFI/serialized artifact의 버전 안정성을 새로 약속하지 않는다.
- 대안: Int64만 추가하면 폭별 정책을 여러 단계에서 반복 동결해야 한다.
  float/char/explicit cast를 함께 추가하면 rounding·Unicode·range-cast 정책도 필요하므로 후속으로 분리한다.

## 지원 타입과 범위

| Canonical 타입 | 최소 | 최대 |
|---|---:|---:|
| int8 | -128 | 127 |
| int16 | -32768 | 32767 |
| int32 | -2147483648 | 2147483647 |
| int64 | -9223372036854775808 | 9223372036854775807 |
| uint8 | 0 | 255 |
| uint16 | 0 | 65535 |
| uint32 | 0 | 4294967295 |
| uint64 | 0 | 18446744073709551615 |

`int=int32`, `uint=uint32`, `byte=uint8`는 동일한 Canonical 타입이다.
Bool/String/Unit과 기존 function/error 타입은 유지한다. float32/64·char·never의 실행 지원은 추가하지 않는다.
정수는 모든 Target에서 이 fixed width이며 usize/host pointer 폭을 사용하지 않는다.
annotation·parameter·return·local/global const·let/var·대입에 같은 정수 계약을 적용한다.

## Literal와 기대 타입 계약

1. 정수 literal은 정수 기대 타입이 있으면 그 타입의 범위를 검사하고, 없으면 Int32다.
   자동으로 int64/uint64를 골라 큰 literal을 수용하지 않는다.
   decimal/base/underscore 철자는 기존 D04를 유지하며 새 suffix나 literal 종류는 추가하지 않는다.
2. 직접 unary minus와 바로 뒤의 Integer token은 signed magnitude로 함께 검사한다.
   `const MIN: int64 = -9223372036854775808`은 허용한다.
   Int32 문맥의 `-(2147483648)`은 기존처럼 N2102이며 괄호 속 양수 magnitude를 먼저 검사한다.
   다른 signed width도 동일하다. unsigned 기대 타입의 직접 음수 표기(`-0` 포함)는 N2102다.
3. 기대 정수 타입은 binding annotation, mutable target, call parameter, return signature에서 전달한다.
   group은 문맥을 전달한다. 이미 선언된 name/call 결과의 타입을 바꾸거나 literal처럼 재해석하지 않는다.
4. **리터럴 전용 정수 부분식**은 Integer, group, prefix `+ -`, arithmetic `+ - * / %`만으로
   구성된 subtree다. name/call/comparison/logical expression은 이 분류에 들어가지 않는다.
   이 분류는 구문 기반이며 값을 실행·folding해서 판정하지 않는다.
5. 산술 binary의 리터럴 전용 operand에 전달할 문맥은 정수 기대 타입이 있으면 그 타입이 우선이다.
   그렇지 않고 반대 operand가 리터럴 전용이 아닌 정수 식이면 그 operand를 독립적으로 추론한 타입을 사용한다.
   둘 다 리터럴 전용이고 정수 기대 타입도 없으면 양쪽은 Int32다.
   기대 타입/반대 operand가 ErrorType이면 파생 변환 오류를 억제한다.
6. 순서/정수 equality 비교의 결과 기대 타입 Bool을 정수 literal에 전달하지 않는다.
   한쪽이 정수 식이면 리터럴 전용 반대 operand는 그 정수 타입을 문맥으로 사용한다.
   둘 다 리터럴 전용이면 Int32다. 따라서 `uint64` 값과 literal `0`의 비교가 가능하다.
7. 위 문맥으로 operand 타입을 확정한 뒤 아래 공통 타입과 결과 변환을 계산한다.
   두 typed operand를 결과의 기대 타입으로 미리 확대해서 산술을 수행하지 않는다.
   실제 operand 실행 순서는 추론 방문 순서와 무관하게 기존 source order다.

예를 들어 `a: int8`에 대해 `a + 1`은 Int8이고 `let r: int64 = a + 1`의 literal 문맥은 Int64다.
반면 `a: int8`, `b: int8`의 `let r: int64 = a + b`는 Int8 checked 덧셈 후 Int64로 승격한다.
결과 annotation이 기존 name의 연산 폭을 자동으로 바꾸지는 않는다.
`let r: int64 = 2147483648`은 허용하고 `let r = 2147483648`은 N2102다.
`const r: int8 = 127 + 1`은 두 literal의 범위는 유효하지만 Int8 계산 overflow N3201이다.
일반 let/var의 같은 산술 실패는 Runtime Abort이며 compile-time에 임의로 거부하지 않는다.

## Lossless widening과 공통 타입

암묵 변환은 **source 타입의 전체 값 범위가 destination에 포함되는 경우**만 허용한다.
현재 값이나 const 값이 작다는 이유로 좁은 타입으로 변환하지 않는다.

| Source → Destination | 허용 조건 |
|---|---|
| intN → intM | M ≥ N |
| uintN → uintM | M ≥ N |
| uintN → intM | M > N |
| intN → uintM | 불허 |
| Bool/String/Unit ↔ integer | 불허 |

- let/var/const annotation, 대입, 인수, return에서 이 변환을 허용한다.
  literal을 기대 타입으로 검사하는 규칙과 typed name/expression을 변환하는 규칙은 구분한다.
  `const A:int32=1; const B:int8=A`는 N2101이며 A가 literal 초기값을 가져도 narrowing하지 않는다.
- 산술과 정수 비교는 두 operand를 모두 lossless 변환할 수 있는 **최소 폭**의 지원 정수 타입을 선택한다.
  동일 폭 후보가 여럿이면 unsigned가 더 작은 전체 범위이므로 이를 우선한다.
  실제 가능한 후보에서는 signed operand의 음수 범위 때문에 unsigned 후보가 배제된다.
- `int8+uint8 → int16`, `int16+uint16 → int32`, `int32+uint32 → int64`,
  `int64+uint32 → int64`, `uint8+uint64 → uint64`다.
  `int64+uint64`와 `int32+uint64`는 공통 정수 타입이 없으므로 N2101이다.
  const의 작은 현재 값, 양수임을 증명한 값이나 float를 사용해 이 실패를 우회하지 않는다.
- 비교/equality 결과는 Bool이다. 산술 결과는 공통 정수 타입이다.
  결과를 소비하는 기대 타입에는 별도의 lossless 결과 변환만 적용한다.
  `< <= > >= == !=`의 정수 혼합은 위 규칙을 따르며 Bool equality/logical은 기존 P02다.
- unary `+`는 해당 정수 타입을 유지한다. unary `-`는 signed 타입만 허용하고 unsigned name/expression은 N2101이다.
  직접 음수 literal의 unsigned 기대 타입 오류는 위 N2102 규칙을 따른다. `!`는 Bool만 허용한다.
- implicit truthiness, bitwise/shift, compound assignment, source `as` cast와 wrapping/checked API는 추가하지 않는다.
  overload, 기본 인수·generic inference나 자동 stringify도 추가하지 않는다.

## Checked Runtime·const 의미

- 모든 폭의 `+ - * / %`와 signed negation은 해당 공통/operand 정수 타입에서 checked다.
  overflow는 모든 Native profile에서 Abort다. unsigned `0-1`도 wrap하지 않고 실패한다.
- division/remainder의 zero divisor는 signed/unsigned 모두 실패다.
  signed MIN/-1 division **및 remainder**는 P03과 같이 overflow 실패다.
  signed division은 0 방향 truncate, remainder는 dividend 부호다. unsigned는 비음수 몫·나머지다.
- MIR/runtime lowering은 나눗셈과 overflow가 발생할 수 있는 연산 전에 필요한 guard를 둔다.
  실패 가능한 signed 연산에 unchecked wrap/UB 또는 LLVM optimizer 가정을 적용하지 않는다.
- 동일 실패는 const에서 N3201이다. P05/P06의 permission·실제 Bool short-circuit·skipped subtree 검사,
  initializer당 10,000 HIR expression nodes·N3202, 전역 dependency cycle과 실패 전파는 유지한다.
- integer ConstValue는 종류/폭에 맞는 값과 타입을 가진다. uint64 최대값도 signed host 값으로 잘못 읽지 않는다.
  const의 implicit widening도 Runtime과 동일한 source/destination 규칙과 부호·범위를 사용한다.
  implicit 변환 metadata/MIR temporary를 HIR node budget에 추가로 세지 않는다.
- const initializer에서 사용자 call/print/보간 실행은 계속 불허한다.
  Runtime에서 보간하는 값에 정수 타입이 늘어나는 것과 const String 보간 허용은 다른 범위다.

## Typed table·MIR·Backend·private ABI

- Core 타입/값에 integer kind와 fixed width를 표현하고 literal/table의 범위를 검사한다.
  implicit conversion은 source와 destination 타입, 변환 지점의 SourceInfo를 분석 결과에 기록한다.
  타입 불일치를 숨기기 위해 원본 name 타입을 destination으로 덮어쓰지 않는다.
- MIR은 widening을 명시적인 내부 scalar 변환으로 나타낸다. source가 signed면 sign extension,
  unsigned면 zero extension이며 같은 타입은 identity다. frontend가 허용하지 않은 narrowing은 만들지 않는다.
  변환 뒤 arithmetic/call/store/return operand 타입은 명확히 일치해야 한다.
  public type/value/coercion table 변조와 illegal MIR conversion은 검증 단계에서 거부한다.
- LLVM Adapter는 i8/i16/i32/i64, 부호에 맞는 comparison/division/remainder와 checked 연산을 선택한다.
  signed/unsigned 타입이 같은 비트 폭을 가져도 Core 타입 의미는 구분한다.
  Native 내부 함수의 인수/return은 해당 고정 폭 값이며 conversion은 call 이전·return 이전에 명시한다.
- 기존 Runtime symbol `nova_format_int(out, i32, file, start, end)`와 Int32 출력 계약은 유지한다.
  정수 보간을 위해 private C ABI symbol `nova_format_i64(out, i64, file, start, end)`와
  `nova_format_u64(out, u64, file, start, end)`를 추가하는 안이다.
  signed narrow 값은 sign-extend, unsigned narrow 값은 zero-extend 후 전달한다.
  out은 기존 NovaString pointer, source 필드는 기존 u32이며 사용자 FFI API는 아니다.
- 보간은 locale-independent decimal ASCII다. signed MIN/uint64 MAX를 정확히 출력하고 suffix·grouping은 없다.
  `print(string) -> Unit`·LF·arena·OOM/I/O 실패 정책과 Bool/String 보간은 P03을 유지한다.
- 기존 Int32 overflow의 panic reason 1/문자열과 zero division reason 2는 보존한다.
  다른 정수 overflow에는 private reason 3/`integer overflow`를 추가한다. SourceInfo/Abort 방식은 동일하다.
- 지원/검증 host는 기존 LLVM 21.1.8 Windows x64/MSVC다. Linux x64 IR/object 검증은 기존 범위이며
  Linux Native 실행·32-bit Target·새 toolchain 도입은 이번 승인 대상이 아니다.

## 문법·진단 경계

Source 문법은 [P06의 31-production EBNF](GRAMMAR_STAGE_B_GLOBAL_CONST.ebnf)를 재사용한다.
type는 기존 IDENT, expression/operator/literal 철자도 같으므로 새 EBNF production을 추가하지 않는다.
전체 [GRAMMAR](GRAMMAR.ebnf)의 `as`/float/char/미래 구문을 승인한 것으로 해석하지 않는다.

| 상황 | 코드 | Primary / secondary |
|---|---|---|
| 해당 기대/default 정수 범위 밖 literal | N2102 | literal/직접 minus expression / 기대 타입 위치 |
| typed narrowing/불허 signedness 변환 | N2101 | actual expression / 기대 타입 위치 |
| 공통 정수 타입 없음 | N2101 | binary expression / operand 타입 설명 |
| unsigned typed negation·Bool/numeric 혼용 | N2101 | operation/operand / 타입 설명 |
| 일반 call/condition/대입 오류 | 기존 N2201/N3001/N3004 등 | 기존 정책 |
| const overflow/div0/MIN/-1 | N3201 | failing expression / const 선언 |
| const budget/cycle | N3202 | 기존 P05/P06 위치·note |
| float/char 등 후속 타입/구문 | N1102 | 미지원 construct |

새 diagnostic code는 추가하지 않는다. ErrorType와 실패 const에서 파생되는 중복 오류를 억제하고,
오류 입력은 LLVM 호출보다 먼저 CLI source exit 1/no-codegen으로 거부한다.

## 목표 프로그램과 수용 계획

아래는 **승인 후 구현할 수용 예제**이며 현재 실행 성공을 주장하지 않는다.

```nova
const MAX: uint64 = 18446744073709551615
const MIN: int64 = -9223372036854775808

func widen(x: int8) -> int64 {
    return x
}

func main() {
    let signed: int8 = -1
    let unsigned: byte = 255
    let mixed = signed + unsigned
    var wide: int64 = widen(signed)
    wide = wide + mixed
    print("mixed={mixed}, wide={wide}, min={MIN}, max={MAX}")
}
```

예상 stdout UTF-8 bytes:
`mixed=254, wide=253, min=-9223372036854775808, max=18446744073709551615\n`, exit 0.
mixed는 Int16, wide는 Int64다.

1. 각 타입/alias의 min/max 및 ±1, decimal/base/underscore·직접 signed MIN/괄호 경계,
   unsigned 음수·매우 긴 숫자 입력, 기대 타입/peer literal/default·리터럴 전용 부분식을 검사한다.
2. 모든 8×8 typed integer 변환/공통 타입 조합을 독립 range containment oracle로 대조한다.
   small const도 narrowing 실패, Bool 혼용·공통 타입 부재, call/return/binding/대입의 정확한 code/Span을 검증한다.
3. 모든 폭/부호의 정상 연산·checked overflow·div0/MIN/-1·signed 몫/나머지를 Runtime과 const로 비교한다.
   다른 폭으로 결과를 소비해도 실제 arithmetic 폭이 보존되고 operand 실행 순서는 source order인지 검사한다.
4. local/global const, P06 forward dependency/cycle, Bool short-circuit·skipped permission/type,
   10,000-node budget과 값/type/coercion table 변조의 no-codegen을 회귀 검증한다.
5. MIR conversion·SourceInfo·validator, LLVM 폭/signedness/guard·deterministic IR,
   실제 Windows COFF/Linux ELF object와 기존 output 보호를 검사한다.
6. Windows Native O0/O2의 전체 폭 함수 인수/return·signed/unsigned 승격·MAX/MIN 보간과
   예제 stdout/exit, runtime 실패 Abort/Span, CLI 선행 오류/no-output을 확인한다.
7. 기존 Stage A/P04/P05/P06 pass/fail/Native 회귀, Cargo fmt/clippy/workspace test/all-features check,
   standalone Runtime rustfmt와 문서 validator를 통과시킨다.

## 승인 경계

승인 대상은 8종 정수/alias, 기대 타입·peer literal 문맥, 전체 범위 기반 lossless widening,
공통 정수 타입·checked 연산·const 확장, MIR 변환·정수 보간/private Runtime 확장이다.
float/char/never·cast/bitwise/shift·wrapping API·overload/aggregate/module·const function,
ownership/Drop·일반 FFI/ABI·Linux Native와 전체 D07/Stage B는 후속이다.
승인 전에는 Compiler·accepted ledger·기존 EBNF를 변경하지 않는다.
