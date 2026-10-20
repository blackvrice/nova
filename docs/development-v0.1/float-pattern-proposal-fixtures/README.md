# P41 float literal 패턴 수용 fixture

Accepted / 구현·검증 완료. [계약](../FLOAT_PATTERN_STAGE_B_PROPOSAL.md)·[71-production EBNF](../GRAMMAR_STAGE_B_FLOAT_PATTERN.ebnf)·[구현 기록](../FLOAT_PATTERN_IMPLEMENTATION.md).
28개 UTF-8/LF source SHA-256을 고정한다. main/values 두 파일·정상 3·부정 21·Runtime 2다.

```text
scrutinee
first
second
double
zero
nan
infinity
subnormal
underflow
rounded
tuple
struct
option
try-ok
result=1
error=4
loop=1
const=1.5
```

debug/release에서 LF·빈 stderr·exit 0이다. float32/64 직접 반올림·decimal alias·signed zero·subnormal/underflow·NaN/Infinity fallback·nested product/sum/rest·guard/try/loop·기존 const/default를 관측했다.
zero_alias는 negative-zero scrutinee/positive-zero pattern 동등성, finite_alias는 Float64 직접 반올림 및 exponent/underscore alias를, guard_skipped는 literal 불일치 뒤 guard 생략을 검사한다.
부정 21개는 실제 code별 byte Span·check exit 1이다. zero/rounded/decimal alias·wildcard 뒤 arm은 N3102, finite-only/guard-only는 N3101이다.
타입 불일치 N2101·finite overflow N2102(FLOAT token primary)·nested binding N3103를 구분한다. const_refutable/global_refutable는 initializer의 1/0을 평가하지 않으며 파생 const 오류를 억제한다.
Runtime guard_abort/omitted_abort는 Int32 overflow·정확한 operation Span·before-only 출력·exit 1을 확인한다.
P32 float_pattern.nova의 고정 source·당시 N1102 metadata를 보존한다. historical_overrides는 같은 SHA-256 source의 현재 check/Native 성공을 기록한다.
문서 validator는 구조/hash/Span 검사이며 Compiler/Native 실행과 별도다.

```powershell
cargo run -p nova-cli --offline -- check docs/development-v0.1/float-pattern-proposal-fixtures/main.nova
$env:NOVA_CLANG=(Resolve-Path target/toolchains/llvm-21.1.8/bin/clang.exe).Path
cargo run -p nova-cli --offline -- run docs/development-v0.1/float-pattern-proposal-fixtures/main.nova --profile debug
cargo run -p nova-cli --offline -- check docs/development-v0.1/float-pattern-proposal-fixtures/duplicate_zero.nova
```
