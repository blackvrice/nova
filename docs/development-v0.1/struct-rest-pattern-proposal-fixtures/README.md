# P35 Copy struct trailing rest 패턴 수용 fixture

Accepted / 구현·검증 완료. [계약](../STRUCT_REST_PATTERN_STAGE_B_PROPOSAL.md)·[67-production EBNF](../GRAMMAR_STAGE_B_STRUCT_REST_PATTERN.ebnf)·[구현 기록](../STRUCT_REST_PATTERN_IMPLEMENTATION.md).
28개 UTF-8/LF source SHA-256을 고정한다. main/values 두 파일·정상 3·부정 21·Runtime 2다.

```text
scrutinee
first
second
nested=-7
initializer
omitted
let=3
var=5/true
const=9/-7
empty
rest-only
complete=3
option=5
try-ok
result=1
error=4
loop=1
```

debug/release의 LF·빈 stderr·exit 0이다. rest-only/Empty·부분 field/역순 label/위치 혼합·중첩·local/global const/default·sparse var binder·omitted-field 효과·guard/try/loop를 관측했다.
initializer/omitted 두 줄은 생략 field의 함수 호출도 전체 initializer에서 한 번 평가됨을 보여 준다.
private_local은 같은 module의 private 생략, full_and_empty는 전체 명시 뒤 rest/쉼표·binder 없는 const를, guard_skipped는 패턴 실패 뒤 guard 생략을 검사한다.
부정 21개는 실제 code별 byte Span·check exit 1을 expected.json에 기록한다. 다른 module의 private field는 rest로 생략해도 N2004다.
const_omitted_failure는 생략한 field의 const 계산도 실패하며 N3201/파생 억제를 유지한다.
Runtime guard_abort/omitted_abort는 기존 Int32 overflow·operation Span·before-only 출력·exit 1을 확인한다.
문서 validator는 구조/hash/Span 검사이며 Compiler/Native 실행과 별도다.

```powershell
cargo run -p nova-cli --offline -- check docs/development-v0.1/struct-rest-pattern-proposal-fixtures/main.nova
$env:NOVA_CLANG=(Resolve-Path target/toolchains/llvm-21.1.8/bin/clang.exe).Path
cargo run -p nova-cli --offline -- run docs/development-v0.1/struct-rest-pattern-proposal-fixtures/main.nova --profile debug
cargo run -p nova-cli --offline -- check docs/development-v0.1/struct-rest-pattern-proposal-fixtures/private.nova
```
