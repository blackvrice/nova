# Stage B float·반올림·숫자 승격·보간 착수안 — P09

작성일: 2026-10-04. 상태: **Accepted / 2026-10-04 사용자 승인**.
승인 기록: 사용자 “P09 승인하고 float 구현 진행”. 구현·검증은 [P09 기록](FLOAT_IMPLEMENTATION.md)을 따른다.
D01~D05/P01~P08 승인과 기존 구현을 보존한다.
float의 최소 실행 계약이며 source cast·float remainder·전체 D07/Stage B 승인은 아니다.

## Specification Change Proposal

- 관련 문서: [Canonical NOVA-004](../00_Governance/NOVA-004_Nova_용어_키워드_Canonical_표.md),
  [Literal NOVA-010](../01_Source_Syntax/NOVA-010_숫자_문자_문자열_Literal_사양서.md),
  [타입 NOVA-025](../03_Types_Declarations/NOVA-025_타입_시스템_타입_추론_형변환_사양서.md),
  [Primitive NOVA-026](../03_Types_Declarations/NOVA-026_Primitive_Literal_기본_타입_결정_사양서.md),
  [숫자 전체 초안](NUMERIC_RULES.md), [D07](DECISIONS.md), [승인 Lexer](ACCEPTED_LEXER.md),
  [P07 정수](INTEGER_STAGE_B_PROPOSAL.md), [P08 char](CHAR_STAGE_B_PROPOSAL.md),
  [테스트 NOVA-136](../13_Testing_Release/NOVA-136_Compiler_테스트_전략서.md).
- 현재 사양: float=float32, double=float64 alias와 손실 없는 제한적 숫자 승격은 Canonical이다.
  D04는 decimal/exponent Float token을 승인했다. 현재 Parser/TypeChecker는 float를 N1102로 거부한다.
  IEEE 결과·literal 문맥·NaN bits·출력·const/Native 연결은 구현 전에 별도 동결해야 한다.
- 발견된 문제: Rust/LLVM의 NaN payload나 fast-math를 그대로 노출하면 metadata 결정성 또는
  const/Native 결과가 달라질 수 있다. Float32 literal을 먼저 Float64로 읽으면 이중 반올림이 발생할 수 있다.
  정수 checked 실패를 float에 그대로 적용하면 Infinity/NaN 계약과 충돌한다.
- 제안 변경: 아래 binary32/64·literal·승격·연산/비교·const·MIR·Native/formatter 계약을 적용한다.
  [전용 EBNF](GRAMMAR_STAGE_B_FLOAT.ebnf)는 P08 primary에 FLOAT terminal만 추가한다.
- 변경 이유: Primitive 확장을 단일 파일의 전체 pipeline에서 검증한 뒤 aggregate/module로 진행한다.
- 영향 범위: Parser/AST/HIR, Types/TypeChecker, const engine, MIR/validator, LLVM Adapter,
  private Runtime, CLI/Native tests와 승인 ledger·관련 NOVA 보완 문서.
- Backward Compatibility: integer-only P07, char P08, print(String), END/escape/진단 code를 유지한다.
  float를 포함한 새 프로그램만 확장하며 암묵 narrowing, integer-only 연산의 float 우회는 없다.
- 대안: float를 literal/타입 검사만 지원하는 단계, 또는 모든 cast/remainder/math API까지 한꺼번에 동결.
  전자는 실행을 막고 후자는 안전한 변환과 함수별 정밀도 계약까지 넓어져 이번 최소 범위에서 제외한다.

## 값·literal·타입 문맥

1. `float32`는 IEEE binary32, `float64`는 binary64다. `float`/`double`은 기존 alias다.
   finite normal/subnormal, ±0, ±Infinity, NaN을 값으로 허용한다.
2. D04 철자를 유지한다: `1.25`, `1e3`, `1_000.25`, `1.0e-3` 등.
   `.5`, `1.`, hex float, suffix `1.0f32`, NaN/Infinity 전용 literal이나 keyword는 추가하지 않는다.
   부호는 기존 prefix token이다. 잘못된 철자는 Lexer N1002, recovery/EOF는 기존 규칙이다.
3. REAL literal은 기대 float 타입을 우선하고, 없으면 float typed peer 문맥을 사용한다.
   둘 다 없으면 Float32다. 기대 정수/Char/Bool/String 타입이 REAL의 값 타입을 바꾸지는 않는다.
   기대 Float64 literal은 decimal에서 Float64로 직접 반올림한다. Float32 경유는 금지한다.
4. literal decimal의 정확한 값에서 선택한 binary 타입으로 round-to-nearest, ties-to-even한다.
   round 결과가 ±Infinity면 N2102다. subnormal 및 underflow-to-zero는 수용한다.
   unary minus는 부호를 보존하므로 `-0.0`, 음수 underflow는 negative zero다.
   N2102 primary는 Float literal token Span, annotation/peer가 있으면 secondary로 표시한다.
5. float-literal-only subtree는 REAL, group, unary ±, `+ - * /`로만 구성된 subtree다.
   기대 float 문맥을 그 leaf까지 전달한다. 기대 문맥이 없고 한쪽만 이 subtree면 typed float peer의
   타입을 사용한다. 양쪽 모두 literal-only면 Float32가 기본이며 소스 실행 순서는 좌→우다.
   기대 문맥이 peer보다 우선한다. Name/call은 literal-only가 아니다.
6. INT literal과 integer-only subtree는 기존 P07 기대/peer 정수 규칙을 유지한다.
   float 기대 타입으로 INT 자체를 REAL로 재분류하지 않는다. float peer는 INT의 정수 기대 타입을 만들지 않는다.
   따라서 `1`은 float 문맥에서 기본 Int32이며 값 1만으로 Int32→Float32의 범위 규칙을 우회하지 않는다.
7. `let x:float=1.0`은 통과, `let x:float=1`은 N2101이다.
   `let x:double=1`은 Int32→Float64 승격으로 통과한다.
   `let a:float=1.0;let b=a+1`은 Float64, `let b:float=a+1`은 N2101,
   `let b:float=a+1.0`은 Float32다. 소수 계산에는 REAL 철자를 쓰는 것이 명시적이다.
8. Core 값은 타입과 원시 bits를 보존하는 invariant 모델을 사용한다.
   metadata의 Eq는 bits equality이며 Nova source `==`의 숫자 equality와 별개다.
   +0/-0를 metadata에서 구분하고 모든 NaN은 아래 canonical bits로 정규화한다.
   HIR은 원 literal spelling과 Span/SourceOrigin을 유지한다.

## 손실 없는 승격과 공통 숫자 타입

전체 source 타입 범위의 모든 값을 정확히 표현할 수 있을 때만 변환한다. 현재 값이 작거나
const가 정확히 표현된다는 이유로 금지된 타입 변환을 허용하지 않는다.

| Source | Float32 | Float64 |
|---|---|---|
| int8/uint8/int16/uint16 및 byte alias | 허용 | 허용 |
| int32/uint32 및 int/uint alias | 불허 | 허용 |
| int64/uint64 | 불허 | 불허 |
| Float32 | identity | 허용 |
| Float64 | 불허 | identity |
| Bool/Char/String/Unit | 불허 | 불허 |

1. 근거는 significand precision p=24/53이다. signed N비트는 N−1≤p,
   unsigned N비트는 N≤p일 때 전체 정수 범위가 정확하다.
2. binding/assignment/call/return/const에서 위 변환을 동일하게 적용한다.
   Float→integer, Float64→Float32와 Char/Bool 변환은 없다. source `as`는 후속이다.
3. binary에 float operand가 있으면 두 operand가 변환 가능한 Float32, 다음 Float64 순서로
   가장 작은 공통 타입을 선택한다. Float32+Int32는 Float64, Float32+Int16은 Float32,
   Float64+UInt64는 N2101이다. integer-only binary는 기존 P07 공통 정수 규칙만 사용한다.
4. 실제 arithmetic은 선택된 공통 타입 폭에서 수행한 후 소비 문맥으로 승격한다.
   `a:float32`, `b:float32`의 `let x:float64=a+b`는 Float32 덧셈 후 Float64 변환이다.
   literal이 기대 Float64로 해석된 `let x:float64=a+1.0`은 Float64 덧셈이다.
   원 operand 타입과 coercion destination을 별도 기록해 실제 폭을 검증한다.

## 연산·IEEE 결과·정규화

1. float의 unary `+ -`, binary `+ - * /`와 6종 비교 `== != < <= > >=`를 지원한다.
   `%`, `!`, `&& ||`의 float operand는 N2101이다. Bool-only 조건과 비교 chain 금지는 유지한다.
   float `%`의 fmod/IEEE remainder 선택은 별도 후속으로 남긴다.
2. 각 arithmetic operation마다 해당 binary 폭에서 round-to-nearest, ties-to-even한다.
   FMA contraction/reassociation/reciprocal approximation, fast-math 및 flush-to-zero는 사용하지 않는다.
   signed zero와 gradual underflow를 보존한다. 전용 FPU 환경/rounding mode API는 없다.
3. Runtime float overflow는 Infinity, underflow는 subnormal/zero, 0.0/0.0과 invalid arithmetic은 NaN이다.
   float 나눗셈의 ±0 divisor는 IEEE 결과이며 정수 div0/overflow Abort를 적용하지 않는다.
   const도 같은 IEEE 값을 허용하며 이 경우 N3201을 내지 않는다. INT 연산의 실패 계약은 그대로다.
4. 모든 NaN 생산 연산·변환·Core 생성은 sign/payload를 제거하고 다음 bits를 사용한다:
   Float32 `0x7FC00000`, Float64 `0x7FF8000000000000`.
   `-NaN`도 canonical NaN이다. 다른 finite/zero/infinity bits는 변경하지 않는다.
   host의 `NAN` 상수 payload를 계약으로 사용하지 않는다.
5. NaN operand의 `==`, `<`, `<=`, `>`, `>=`는 false, `!=`는 true다.
   ±0는 equality true다. Infinity는 일반 IEEE 숫자 순서로 비교한다.
   숫자 equality가 같은 +0/-0도 metadata bits는 다르다.
6. 지원하는 compiler host의 const 계산은 이 결과와 bits가 같아야 한다.
   host FP 모드의 검증 없이 Rust 연산을 의미 정의로 사용하지 않는다.
   동등한 deterministic 계산 또는 검증된 host FP 환경을 사용한다.
   Native entry는 ties-to-even·gradual underflow·non-trapping 모드를 보장한다.
   지원하지 못하는 host는 명시적으로 거부하며 잘못된 결과로 조용히 진행하지 않는다.

실행 근거와 참고 문서는
[Rust binary32](https://doc.rust-lang.org/std/primitive.f32.html),
[Rust binary64](https://doc.rust-lang.org/std/primitive.f64.html),
[LLVM 21.1 float semantics](https://releases.llvm.org/21.1.0/docs/LangRef.html#floating-point-semantics),
[LLVM fast-math](https://releases.llvm.org/21.1.0/docs/LangRef.html#fast-math-flags)다.
NaN canonical bits와 아래 출력 규칙은 Nova의 새 제안이며 Rust/LLVM이 보장하는 계약이라고 주장하지 않는다.

## const와 의미 검사

- `ConstValue`에 typed float bits를 연결한다. literal/group/const 이름, 허용 unary/arithmetic/compare,
  lossless numeric 변환을 const에 적용한다. 함수 call·일반 let/var 참조·보간/I/O는 여전히 불허다.
- P05의 10,000 HIR node budget은 유지하며 implicit conversion을 추가 node로 세지 않는다.
  Bool short-circuit은 평가를 생략하지만 skipped RHS 타입·permission·budget을 검사한다.
- P06 forward global const·정적 dependency/cycle·N3202, print shadow 경계를 유지한다.
  NaN const의 public table 비교도 bits equality로 결정적이며 +0/-0 변조는 차이로 판정한다.
- float는 Char/Bool/String/Unit과 혼용하지 않는다. `print(1.5)`는 N2101이며
  `print("{1.5}")`는 보간으로 수용한다. 함수 print shadow는 기존 P02대로다.

## MIR·LLVM·private Runtime

1. MIR typed float Constant와 value 타입을 추가한다. lossless 변환은 명시적 Rvalue로 보존한다.
   기존 integer Widen의 검증 계약은 유지한다. independent validator가 float→int/narrowing·mixed
   arithmetic·잘못된 operator/return/call과 uninitialized Place를 거부한다.
2. lower의 Resolve/Checked 정확한 재계산 gate는 float payload·source type·destination coercion,
   NaN/zero bits·const 값/count 변조를 codegen 전에 거부한다. SourceInfo를 보존한다.
3. LLVM private scalar ABI는 Float32=`float`, Float64=`double`이다.
   fadd/fsub/fmul/fdiv/fneg, fpext, signed/unsigned integer-to-float 변환을 올바르게 선택한다.
   비교는 ordered eq/lt/le/gt/ge, unordered-or-not-equal ne다. NaN은 canonical bits로 정규화한다.
   `fast`/`nnan`/`ninf`/`nsz`/`arcp`/`reassoc`/`contract` 등 의미를 바꾸는 flags는 붙이지 않는다.
   IEEE literal bits를 정확히 전달하며 decimal IR 재파싱으로 bits가 바뀌지 않게 검증한다.
4. private formatter는 원 타입의 bits를 받는다:
   `nova_format_f32(out:*mut NovaString,bits:u32,file:u32,start:u32,end:u32)`와
   `nova_format_f64(out:*mut NovaString,bits:u64,file:u32,start:u32,end:u32)`.
   LLVM에서는 각각 `(ptr,i32,i32,i32,i32)` / `(ptr,i64,i32,i32,i32)`다.
   Float32를 Float64로 확대한 뒤 Float64 decimal formatter로 출력하지 않는다.
5. formatter declaration 탐지는 callee/local뿐 아니라 global Const float만 사용하는 보간도 포함한다.
   float 없는 기존 IR snapshots와 integer/char Runtime symbol·panic reason을 보존한다.
   생성된 String은 기존 arena에 보존하며 print LF/flush와 OOM/I/O Abort는 유지한다.
6. 구현 대상은 기존 LLVM 21.1.8 Windows x64 Native다. Linux x64 IR/ELF object를 검증하되
   로컬 Linux link/run 환경이 없는 경우 Native 실행 성공으로 표시하지 않는다. public FFI/ABI는 후속이다.

## 보간 출력 계약

host locale·terminal·profile과 무관한 ASCII를 사용한다. 다음 문자열은 정확한 출력이다.

| 값 | 출력 |
|---|---|
| positive zero | `0` |
| negative zero | `-0` |
| positive/negative infinity | `inf` / `-inf` |
| 모든 NaN | `NaN` |
| Float32 0.1 | `0.1` |
| Float64 1.25 | `1.25` |
| finite 1.0 | `1` |

다른 finite 값은 원 binary 타입으로 round-trip하는 **가장 적은 유효 decimal digits**를 선택한다.
같은 digit 수 후보가 여러 개면 정확한 binary 값에 가장 가까운 decimal을 고르고,
거리가 같으면 끝 digit이 짝수인 후보를 고른다. 불필요한 유효 trailing zero를 제거한다.
선택한 coefficient×10^exponent는 exponent notation 없이 고정 소수점으로 전개한다.
필요한 leading/trailing zero는 유지하고 `+` sign·locale separator는 쓰지 않는다.
이 round-trip은 decimal→float numeric codec 기준이며 `1`을 Nova FLOAT token으로 재분류한다는 뜻이 아니다.
Rust typed Display를 구현 후보로 쓸 수 있지만 위 독립 oracle/경계 corpus와 일치해야 한다.
출력 예: Float32 `1e20` → `100000000000000000000`, `1e-7` → `0.0000001`.
precision/exponent format 옵션과 math API는 이번 범위에 추가하지 않는다.

## 진단과 수용 예제

새 diagnostic code는 추가하지 않는다. malformed numeric은 N1002, literal overflow는 N2102,
금지된 type/operator/conversion은 N2101, Bool 조건은 N3001, immutable assignment는 N3004다.
기존 const permission N3201, budget/cycle N3202와 Lexer/Parser cascade 억제는 유지한다.
float IEEE arithmetic 결과만으로 source error나 integer panic을 만들지 않는다.
source 오류는 CLI check/build/run에서 tool 호출·output 생성 전에 exit 1로 거부한다.

```nova
const HALF: float = 0.5
const TOTAL: double = 1.25 + 2.5
const ZERO: double = 0.0
const INF: double = 1.0 / ZERO
const NAN: double = ZERO / ZERO
const NZ: float = -0.0

func echo(value: double) -> double {
    return value
}

func main() {
    var value: float = HALF
    value = value + 0.25
    let tiny: int16 = 2
    let mixed = value + tiny
    let equal = NAN == NAN
    let different = NAN != NAN
    print("value={value}, mixed={mixed}, total={echo(TOTAL)}")
    print("inf={INF}, nan={NAN}, negzero={NZ}, eq={equal}, ne={different}")
}
```

승인 후 예상 stdout UTF-8 bytes이며 현재 실행 성공을 주장하지 않는다:
`value=0.75, mixed=2.75, total=3.75\ninf=inf, nan=NaN, negzero=-0, eq=false, ne=true\n`, exit 0.

## 검증 계획

1. Lexer Float/INT 구분·underscore/exponent·END·malformed/EOF와 Parser FLOAT leaf/Span/recovery,
   HIR spelling·alias·malformed public AST를 검사한다. 기존 accepted EBNF와 D04 철자는 보존한다.
2. binary32/64 literal의 ties-to-even, 직접 decimal→Float32/64, 이중 반올림 반례,
   max finite/overflow threshold·min normal/subnormal·half-min underflow·negative zero·긴 입력을 검사한다.
   기대/peer/기본 타입과 INT/REAL 경계의 pass/fail·정확한 Span을 대조한다.
3. 8×2 integer→float whole-range conversion과 float widening·공통 타입을 독립 oracle로 검사한다.
   const가 작아도 금지되는 narrowing, integer-only float 우회 부재와 raw operation 폭을 검증한다.
4. finite/±0/±Infinity/NaN의 arithmetic·6종 comparison을 const/Runtime에서 bits와 Bool로 대조한다.
   canonical NaN·zero sign, overflow/underflow, integer checked failures 회귀를 포함한다.
   FMA/contraction이 결과를 바꾸는 반례와 O0/O2/operand effect 순서·FP 환경도 검사한다.
5. local/global const·forward/cycle·skipped permission/type·10,000-node budget과
   NaN/±0 payload·source type/coercion/count 변조 gate, MIR illegal conversion/operator를 검증한다.
6. formatter를 원 타입 round-trip/독립 coefficient oracle·finite boundary/generated bits corpus와 대조한다.
   special text·locale 부재·global-constant-only formatter·함수 ABI·이전 String lifetime을 검사한다.
7. 실제 LLVM 21.1.8 O0/O2 Windows COFF/Linux ELF object와 Windows Native 예제·bytes·exit,
   CLI no-tool/no-output, 기존 Hello/정수/char snapshots·Native regression을 검사한다.
8. Cargo fmt/clippy/workspace test/all-features check, standalone Runtime rustfmt와 문서 validator를 실행한다.
   문서/grammar 검증과 Compiler/Native 수용 테스트 통과는 구분해 기록한다.

## 승인 경계

승인 대상은 binary32/64·alias, REAL literal 문맥·직접 ties-to-even과 overflow N2102,
전체 타입 범위 기반 lossless numeric promotion, IEEE `+ - * /`·unary·6종 비교,
canonical NaN/±0·const·MIR/LLVM/private formatter·출력과 FLOAT primary EBNF다.
`as`/narrowing·float remainder/math API·never·aggregate/module·const function·ownership/Drop,
public FFI·Linux Native와 전체 D07은 후속이다. P09 승인 범위만 Compiler/accepted ledger에 적용하며 기존 accepted EBNF를 보존한다.
