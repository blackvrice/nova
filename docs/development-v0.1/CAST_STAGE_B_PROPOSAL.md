# Stage B 명시적 숫자 변환 착수안 — P10

작성일: 2026-10-05. 상태: **Draft / 사용자 승인 대기**.
검토용 제안이며 Compiler에는 적용하지 않았다. D01~D05/P01~P09와 원본 Canonical을 보존한다.
이번 범위는 단일 파일의 숫자 `as`다. 전체 D07/Stage B 승인이 아니다.

## Specification Change Proposal

- 관련 문서: [Canonical NOVA-004](../00_Governance/NOVA-004_Nova_용어_키워드_Canonical_표.md),
  [타입 NOVA-025](../03_Types_Declarations/NOVA-025_타입_시스템_타입_추론_형변환_사양서.md),
  [Primitive NOVA-026](../03_Types_Declarations/NOVA-026_Primitive_Literal_기본_타입_결정_사양서.md),
  [숫자 초안](NUMERIC_RULES.md), [D07](DECISIONS.md), [P07 정수](INTEGER_STAGE_B_PROPOSAL.md),
  [P09 float](FLOAT_STAGE_B_PROPOSAL.md), [END](ACCEPTED_LEXER.md),
  [전체 Draft 문법](GRAMMAR.ebnf), [테스트 NOVA-136](../13_Testing_Release/NOVA-136_Compiler_테스트_전략서.md).
- 현재 사양: `as`는 Canonical keyword다. 제한적 손실 없는 암묵 승격은 확정이다.
  전체 Draft EBNF는 postfix `as type`를 제안하고 NUMERIC_RULES는 checked narrowing을 제안한다.
  현재 Parser는 source cast를 N1102로 거부한다. P09은 cast를 후속으로 명시했다.
- 발견된 문제: 정수 signed/unsigned 경계, float truncation 전후의 범위 검사, finite narrowing overflow,
  literal 기대 문맥과 cast 우선순위, const/Runtime 실패 위치가 아직 동결되지 않았다.
- 제안 변경: 아래 숫자 10종의 명시적 checked `as`·반올림·실패·구문·const·MIR/Native 계약을 적용한다.
- 변경 이유: P07/P09의 암묵 승격 경계를 유지하면서 narrowing을 소스에 명확히 표시하고,
  aggregate/module 전에 숫자 변환의 실행·진단 계약을 검증한다.
- 영향 범위: Parser/AST/HIR, Types/TypeChecker, const, MIR/validator, LLVM Adapter/private Runtime,
  CLI/수용 테스트, [전용 EBNF](GRAMMAR_STAGE_B_CAST.ebnf)와 승인 ledger.
- Backward Compatibility: 기존 implicit conversion·checked 정수·float IEEE 산술·NaN/±0·출력을 유지한다.
  `as`가 없는 기존 소스와 Hello snapshots는 바뀌지 않는다. Lexer/END의 새 규칙은 추가하지 않는다.
- 대안: 정수 cast만 먼저 지원하거나, Float64→Float32 overflow를 IEEE Infinity로 허용하거나,
  float→int의 소수 부분을 포함한 원 값 자체에 범위 검사를 적용하는 정책.
  이번 제안은 숫자 전 계열을 지원하며 finite narrowing overflow는 실패,
  float→int는 소수 부분 제거 뒤의 정수 범위를 검사한다. 이 선택은 사용자 승인 대상이다.

## 구문·우선순위·문맥

1. `expression as type`의 type에는 기존 단순 type 문법을 사용한다. 성공 가능한 target은
   int8/16/32/64, uint8/16/32/64, Float32/64와 byte/int/uint/float/double alias다.
   Bool/Char/String/Unit/함수·aggregate/Option/Result·never 변환은 이번 범위에 없다.
   이미 지원하는 nonnumeric type을 target으로 쓰면 N2101, undefined type은 N2001,
   never 등 기존 미지원 type은 기존 N1102 규칙을 따른다.
2. cast는 호출과 같은 postfix 층이고 source order로 왼쪽 결합한다. prefix보다 강하다.
   `f(x) as double`, `x as int8 as double`을 허용한다.
   `a+b as double`은 `a+(b as double)`, `-x as double`은 `-(x as double)`이다.
   전체 덧셈이나 음수 값을 변환하려면 `(a+b) as double`, `(-x) as double`을 쓴다.
   호출/cast postfix를 섞어도 Parser는 source order를 보존하며 noncallable 값 호출은 N2101이다.
3. `as`는 binary operator나 type 기대 문맥 생성자가 아니다.
   cast operand는 소비 위치의 기대 타입과 target 타입을 전달받지 않고 기존 P07/P09 규칙으로 검사한다.
   cast 자체의 결과 타입은 target이고, 소비 위치는 이 결과에 기존 implicit widening을 적용한다.
   cast subtree는 integer/float-literal-only subtree에 포함되지 않는다.
4. 따라서 `1 as float`는 Int32→Float32 명시 변환으로 통과한다.
   `1.0 as double`은 기본 Float32 값에서 Float64로 변환한다. decimal→Float64 직접 parsing이 아니다.
   `1e100 as double`은 먼저 Float32 literal이 범위를 초과하므로 N2102다.
   `let wide:double=1e100;let x=wide as double`은 통과한다.
   `2147483648 as int64`도 기본 Int32 literal 오류다. `let wide:int64=2147483648`로 먼저 타입을 정한다.
5. postfix가 prefix보다 강하므로 `-1 as uint8`은 unsigned unary minus 타입 오류다.
   `(-1) as uint8`은 음수 정수의 checked cast 실패다.
   `(-2147483648) as int64`는 기존 직접 음수 최소값 literal 규칙을 유지한다.
6. 기존 D05는 `as` 전후 줄바꿈을 operator continuation으로 취급한다.
   `value\n as\n int8`과 같은 연결은 기존 normalized token대로 해석한다.
   bare return/break/continue의 줄바꿈 종료 우선, 명시 semicolon END는 보존한다.
   generic/nullable/qualified cast target은 후속이며 새로운 generic delimiter 추측을 추가하지 않는다.
7. AST/HIR Cast는 operand와 target type syntax를 분리하고 전체 cast Span,
   `as`와 target의 source 위치 및 SourceOrigin을 보존한다.
   누락 target/잘못된 type/EOF에서 기존 Parser 오류 복구와 기본 nesting 한도 128을 유지한다.

## 변환 값과 실패 규칙

모든 숫자 source/target 조합은 type 단계에서 허용한다. 값 실패가 가능한 변환은 checked 연산이다.
같은 타입/alias identity도 명시적으로 허용한다. Bool/Char/String/Unit 혼용은 허용하지 않는다.

| Source → Target | 값 규칙 | 실패 |
|---|---|---|
| 정수 → 정수 | 원 수학적 정수 값 유지 | target min/max 밖 |
| 정수 → float | target 폭으로 직접 RN ties-even | 지원하는 64-bit 정수 범위에서는 overflow 없음 |
| float → 정수 | finite 값의 소수 부분을 0 방향으로 제거 | NaN/Infinity 또는 제거한 정수 값이 target 범위 밖 |
| Float32 → Float64 | 정확한 widening | 없음 |
| Float64 → Float32 | RN ties-even, gradual underflow, zero sign 유지 | finite source가 반올림 뒤 Infinity가 됨 |
| float → 같은 float | bits identity, canonical NaN 유지 | 없음 |

1. 정수 narrowing은 lower bits truncation/wrapping이 아니다.
   signed↔unsigned도 value가 target 범위에 있어야 한다. `255 as int8`은 실패하며 `127 as int8`은 성공한다.
   `(-1) as uint8`은 실패한다. uint64 값은 host signed i64로 먼저 재해석하지 않는다.
2. 정수→float는 손실을 명시적으로 허용한다. 작은 값만 허용하는 whole-type 조건을 적용하지 않는다.
   Int32 16777217→Float32는 16777216, UInt64 최대값→Float64는 정확한 RN 결과 18446744073709551616이다.
   정수→Float32를 Float64 경유로 구현하지 않는다. 중간 폭의 double rounding을 금지한다.
3. float→정수는 원 float의 정확한 binary 값에서 truncation한다. Float32/64를 먼저 다른 float 폭으로 바꾸지 않는다.
   127.9→int8은 127, -128.9→int8은 -128, -0.9→uint8은 0이다.
   128.0→int8, -129.0→int8, -1.0→uint8은 실패한다. ±0는 정수 0이 된다.
   `UInt64::MAX as Float64`의 RN 결과는 2^64이므로 다시 UInt64로 cast하면 실패한다.
   `i64::MAX`를 Float64로 반올림해 얻은 2^63도 Int64 upper bound로 재사용하지 않는다.
4. Float64→Float32는 소수부 손실을 허용한다. subnormal·underflow-to-zero는 성공이고 ±0 부호를 보존한다.
   source ±Infinity는 그대로 ±Infinity, source NaN은 target canonical NaN이다.
   finite source 1e100을 Float32로 변환하면 실패한다. P09 float 산술 overflow→Infinity 규칙은 그대로다.
5. 모든 cast float 결과는 P09의 canonical NaN bits를 따른다.
   RN ties-even·FTZ/DAZ off·masked exceptions와 const host 환경 제어/복원도 P09를 따른다.
   지원 host 범위를 늘리지 않는다.
6. 허용된 source/target의 값 실패는 Runtime에서 Abort다. private nova_panic reason 4를 사용하고
   첫 stderr 줄은 `Nova panic: numeric cast out of range at file#F:S..E`다.
   SourceInfo는 실패한 cast node의 전체 Span이며 뒤 statement/call effect는 실행되지 않는다.
   Integer arithmetic의 기존 panic reason 1~3, CLI child 종료 상태 보고와 exit 정책은 유지한다.
   일반 let/var/return/call의 cast를 상수 folding만으로 새로운 compile error로 만들지 않는다.

## Const·MIR·LLVM·Runtime

1. P05/P06 const 허용 표현식에 숫자 cast를 추가한다. global forward reference/cycle와
   skipped RHS의 정적 type/permission/dependency 검사는 유지한다.
   정적 허용성/타입 검사가 끝난 뒤 실제 실행하지 않는 logical RHS의 값 실패는 평가하지 않는다.
   `const C=false&&((128 as int8)==0)`은 false다. RHS에 금지된 함수 호출이 있으면 기존 N3201이다.
2. 실행한 const cast의 값 실패는 N3201이다. primary는 실패한 cast node, secondary는 기존 const 선언이다.
   target에 맞추기 위해 literal 오류를 cast 실패로 바꾸지 않는다. literal 실패는 먼저 N2102다.
3. 예산은 cast 연산 자체 1 node와 평가 대상 operand expression nodes를 센다.
   target type syntax는 값 평가 node가 아니며 implicit widening은 기존처럼 추가 비용이 없다.
   10,000-node 제한과 N3202를 유지하고 identity cast도 1 node로 센다.
4. TypeChecker는 raw operand/result 타입과 implicit coercion metadata를 분리한다.
   MIR은 새로운 abstract CheckedCast(operand,target)를 사용한다.
   기존 Widen(integer-only)와 NumericConvert(lossless-only)의 validator 계약을 넓히지 않는다.
   independent validator는 숫자 pair·result/destination 타입·초기화·SourceInfo를 검사한다.
   재계산 gate는 target/source 타입, coercion, const bits/값·count/table 변조를 거부한다.
5. LLVM lowering은 원 수학적 범위 검사를 먼저 수행하거나 같은 의미의 checked private helper를 사용한다.
   float→int에서 NaN/Infinity/범위 초과 값을 raw fptosi/fptoui의 실행 경로에 통과시키지 않는다.
   truncation 뒤의 target 정수 범위를 정확히 검사한다. 부정확하게 반올림한 MAX 비교는 금지한다.
   정수→float는 sitofp/uitofp의 source signedness와 직접 target 폭을 보존한다.
   float narrowing에는 fptrunc와 finite overflow guard·NaN canonicalization을 연결한다.
   fast-math/FMA/reassociation을 추가하지 않는다. 구현 방식은 LLVM이 Nova 의미를 바꾸지 않는 범위에서 선택한다.
6. cast는 operand를 한 번만 평가한다. chained cast는 안쪽부터 순서대로 검사하고 첫 실패에서 Abort다.
   함수 인수의 기존 좌→우 순서, short-circuit, loop·mutable binding·const source 추적을 보존한다.
   float 출력은 기존 P09 bit formatter와 String arena를 그대로 사용한다.
7. type 불일치·undefined/unsupported target·malformed source·const 실패는 CLI에서 도구 호출과
   output 생성 전에 기존 exit 1로 거부한다. 미지원 host exit 3도 유지한다. 새 Nxxxx code는 없다.

## Backend 근거와 정책의 구분

LLVM fptosi/fptoui는 결과가 정수 타입에 들어가지 않으면 poison을 만든다.
[LLVM 21.1 LangRef](https://releases.llvm.org/21.1.0/docs/LangRef.html#fptosi-to-instruction).
그래서 이번 제안은 변환 전 valid 경로를 확정하도록 요구한다.
fptrunc의 기본 FP 환경/NaN payload 규칙도 확인했다.
[LLVM fptrunc](https://releases.llvm.org/21.1.0/docs/LangRef.html#fptrunc-to-instruction).
canonical NaN과 finite narrowing Abort는 Nova 제안 정책이며 LLVM의 자동 보장이 아니다.

Rust의 numeric `as`는 integer truncation, float→int의 NaN→0·범위 saturation 등 다른 정책을 가진다.
[Rust Reference](https://doc.rust-lang.org/reference/expressions/operator-expr.html#numeric-cast).
검사 없는 Rust `as` 자체를 Nova checked 의미로 사용하지 않는다. backend 자료는 구현 근거이며 Nova 사양 승인을 대신하지 않는다.

## 수용 예제

승인 후 실행 대상으로 아래를 제안한다. 현재 compile/run 성공을 주장하지 않는다.

```nova
const LIMIT: int16 = 127
const SMALL = LIMIT as int8
const WHOLE = 127.9 as int8
const ZERO = (-0.9) as uint8
const SKIPPED = false && ((128 as int8) == 0)
func echo(value: float) -> float { return value }
func main() {
    let wide: int64 = 16777217
    let rounded = echo(wide as float)
    let precise: double = 1.25
    var value = precise as float
    value = value + (1 as float)
    print("small={SMALL}, whole={WHOLE}, zero={ZERO}, skipped={SKIPPED}")
    print("rounded={rounded}, value={value}, back={value as int}")
}
```

제안 stdout UTF-8 bytes/LF:
`small=127, whole=127, zero=0, skipped=false\nrounded=16777216, value=2.25, back=2\n`, exit 0.
값 실패 corpus: 일반 함수의 `128 as int8`, `(-1) as uint8`, NaN/Infinity→int,
finite double 1e100→float, Float64 2^63→Int64 및 2^64→UInt64.
동일 cast를 실제 평가하는 const는 N3201이고 뒤 Runtime effect는 실행되지 않아야 한다.

## 검증 계획

- Parser postfix/prefix/결합·call chain·as 전후 newline·bare return·target EOF/오류 복구,
  AST/HIR의 value/type 구분·Span/SourceOrigin과 malformed public AST를 검사한다.
- 숫자 10×10 pair의 identity·최소/최대·±1·signedness·consumer widening·alias와 nonnumeric 거부를 검사한다.
  target/source를 바꿔 default literal 또는 peer 규칙을 우회하지 못하는지 대조한다.
- 독립 정수/유리수 oracle로 정수→float direct rounding·midpoint·double rounding 반례,
  float→정수 truncation 및 ±0·subnormal·MIN/MAX 근접·2^63/2^64·NaN/Infinity를 검사한다.
  Float64→Float32 정상/underflow·finite overflow threshold·NaN/Infinity/zero sign도 bits로 대조한다.
- const 값·실패·skipped RHS permission/type/cycle와 cast count의 10,000/10,001 경계를 검사한다.
- MIR의 illegal CheckedCast·legacy Widen/NumericConvert widening 우회, raw type/target/coercion/
  signed zero·NaN·const count/table 변조 gate 및 initialization/source 검사를 수행한다.
- LLVM O0/O2 Windows COFF/Linux ELF에서 잘못된 float→int 경로가 poison 결과를 소비하지 않는지 검사한다.
  Windows Native 성공 값/Abort·SourceInfo·first failure·effect 한 번/순서·loop/String lifetime를 대조한다.
  Runtime/const 결과, 기존 Hello/정수/char/float 회귀와 CLI no-tool/no-output를 검사한다.
- Cargo fmt/clippy/workspace test/all-features, Runtime rustfmt와 문서/EBNF/ledger 검증을 수행한다.
  문서 검증 결과는 Compiler 구현·Native 실행 성공과 구분한다.

## 승인 경계

승인 대상은 numeric `as` postfix 구문·문맥 격리, 10종 numeric pair의 위 값/반올림·범위 정책,
Runtime Abort reason 4/SourceInfo, const N3201·node budget, CheckedCast와 Native 검증, 전용 EBNF다.
Bool/Char/String/Unit·unsafe/bitcast·wrapping/saturating/Option cast API·pointer/public FFI·float remainder/math,
aggregate/module·const function·Linux Native와 전체 D07은 후속이다.
사용자 승인 전 Compiler·accepted ledger와 기존 accepted EBNF를 변경하지 않는다.
