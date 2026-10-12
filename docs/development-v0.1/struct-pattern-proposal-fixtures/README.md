# P33 Copy struct 위치 패턴 수용 fixture

Accepted / 구현·검증 완료. [계약](../STRUCT_PATTERN_STAGE_B_PROPOSAL.md)·[64-production EBNF](../GRAMMAR_STAGE_B_STRUCT_PATTERN.ebnf)·[구현 기록](../STRUCT_PATTERN_IMPLEMENTATION.md).
26개 UTF-8/LF source SHA-256을 고정한다. main/values 두 파일·정상 3·부정 20·Runtime 1이다.

```text
scrutinee
first
second
nested=-7/🙂
let=3/true/가
var=5/true
const=9/true/-7
empty
complete=3
option=5
try-ok
result=1
error=4
loop=1
```

debug/release의 LF·빈 stderr·exit 0이다. imported type alias·single scrutinee·nested struct·let/var/local/global const·method 제외 arity·Empty·Bool 합집합·Option·guard 순서·try·loop를 관측했다.
private_local은 같은 module의 private field, alias_annotation은 transparent type alias annotation, guard_skipped는 패턴 실패 뒤 guard 생략을 검사한다.
부정 20개의 실제 code별 byte Span·check exit 1을 expected.json에 기록한다. private.nova는 다른 module의 private field를 wildcard로 분해해도 N2004다.
const_refutable/global_refutable은 N3103 뒤 const 계산/파생 오류를 억제한다. const_failure/cycle은 N3201/N3202다.
Runtime guard_abort는 기존 Int32 overflow·operation Span·before-only 출력·exit 1이다. 문서 validator는 구조/hash/Span 검사이며 Compiler/Native 실행과 별도다.

```powershell
cargo run -p nova-cli --offline -- check docs/development-v0.1/struct-pattern-proposal-fixtures/main.nova
$env:NOVA_CLANG=(Resolve-Path target/toolchains/llvm-21.1.8/bin/clang.exe).Path
cargo run -p nova-cli --offline -- run docs/development-v0.1/struct-pattern-proposal-fixtures/main.nova --profile debug
cargo run -p nova-cli --offline -- check docs/development-v0.1/struct-pattern-proposal-fixtures/private.nova
```
