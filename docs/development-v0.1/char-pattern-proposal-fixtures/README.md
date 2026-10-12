# P31 Unicode char literal 패턴 수용 fixture

Accepted / 구현·검증 완료. [계약](../CHAR_PATTERN_STAGE_B_PROPOSAL.md)·[62-production EBNF](../GRAMMAR_STAGE_B_CHAR_PATTERN.ebnf)·[구현 기록](../CHAR_PATTERN_IMPLEMENTATION.md).
18개 UTF-8/LF source SHA-256을 고정한다. main/values 두 파일·정상 2·부정 13·Runtime 1이다.

```text
scrutinee
first
second
nested=7
nul
maximum
before-surrogate
after-surrogate
option=true
try-ok
result=1
error=4
```

debug/release의 LF·빈 stderr·exit 0을 관측했다. single scrutinee·raw/escape alias·nested Event/Option/Tuple·NUL/MAX/surrogate 경계·guard 순서·try를 검증한다.
controls는 newline/backslash/quote/double quote/Hangul을, guard_skipped는 실패한 literal 뒤 overflow guard가 실행되지 않는 것을 검사한다.
13개 부정 check의 실제 code별 byte Span·exit 1을 expected.json에 기록한다. malformed scalar는 기존 Lexer N1002와 Parser 복구 N1102를 함께 낼 수 있다.
Runtime guard_abort는 literal 성공 뒤 기존 Int32 overflow·operation Span·before-only 출력·exit 1을 확인한다.
P30 char_pattern 원 source/N1102 metadata는 보존하고 현재 성공을 historical_overrides로 기록한다.
문서 validator는 구조/hash/Span 검사이며 Compiler/Native 실행과 별도다.

```powershell
cargo run -p nova-cli --offline -- check docs/development-v0.1/char-pattern-proposal-fixtures/main.nova
$env:NOVA_CLANG=(Resolve-Path target/toolchains/llvm-21.1.8/bin/clang.exe).Path
cargo run -p nova-cli --offline -- run docs/development-v0.1/char-pattern-proposal-fixtures/main.nova --profile debug
cargo run -p nova-cli --offline -- check docs/development-v0.1/char-pattern-proposal-fixtures/duplicate_escape.nova
```
