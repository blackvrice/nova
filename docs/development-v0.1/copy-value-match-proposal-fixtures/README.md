# P30 숫자·char·Copy struct statement match 수용 fixture

Accepted / 구현·검증 완료. [계약](../COPY_VALUE_MATCH_STAGE_B_PROPOSAL.md)·[구현 기록](../COPY_VALUE_MATCH_IMPLEMENTATION.md).
P29 [62-production EBNF](../GRAMMAR_STAGE_B_GLOBAL_CONST_DESTRUCTURING.ebnf)를 원 bytes 그대로 재사용한다.
16개 UTF-8/LF source SHA-256을 고정한다. main/values 두 파일·정상 scalar 1·부정 12·Runtime 1이다.

## 관측한 main 출력

```text
scrutinee
first
second
snapshot=7/current=9
char=🙂
nan
negative-zero
max=1
private-copy
success=3
result=1
error=4
loop=1
```

debug/release에서 LF·빈 stderr·exit 0이다. 한 번 scrutinee 평가·guard source order·whole Copy snapshot·alias/private struct·Read method·uint64 MAX·char·NaN/negative zero·try/loop를 관측한다.
scalar_boundaries는 8종 정수 min/MAX와 Float32 NaN·NUL char 비교를 두 profile에서 검사한다.
12개 부정 check의 실제 code·byte Span·exit 1은 expected.json에 기록한다. Runtime guard_abort는 Int32 overflow의 operation Span·before-only 출력·exit 1이다.
P14 invalid_scrutinee·P24/P25 root_scalar 원 source/N2101 metadata는 보존하고 현재 성공을 historical_overrides로 기록한다.
문서 validator는 구조/hash/UTF-8 Span 검사이며 Compiler/Native 실행과 별도다.

```powershell
cargo run -p nova-cli --offline -- check docs/development-v0.1/copy-value-match-proposal-fixtures/main.nova
$env:NOVA_CLANG=(Resolve-Path target/toolchains/llvm-21.1.8/bin/clang.exe).Path
cargo run -p nova-cli --offline -- run docs/development-v0.1/copy-value-match-proposal-fixtures/main.nova --profile debug
cargo run -p nova-cli --offline -- check docs/development-v0.1/copy-value-match-proposal-fixtures/guard_only.nova
```
