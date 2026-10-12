# P32 고정 폭 정수 literal 패턴 수용 fixture

Accepted / 구현·검증 완료. [계약](../INTEGER_PATTERN_STAGE_B_PROPOSAL.md)·[63-production EBNF](../GRAMMAR_STAGE_B_INTEGER_PATTERN.ebnf)·[구현 기록](../INTEGER_PATTERN_IMPLEMENTATION.md).
24개 UTF-8/LF source SHA-256을 고정한다. main/values 두 파일·정상 3·부정 18·Runtime 1이다.

```text
scrutinee
first
second
nested
hex=255
zero
option=true
try-ok
result=1
error=4
loop=1
```

debug/release의 LF·빈 stderr·exit 0이다. single scrutinee·signed/radix/zero alias·nested Tuple/Enum/Option·guard 순서·try·loop를 관측했다.
boundaries는 signed MIN/unsigned MAX 8종을, full_uint8은 256개 전체 값의 fallback 없는 match를, guard_skipped는 실패한 literal 뒤 overflow guard 생략을 검사한다.
부정 18개의 실제 code별 byte Span·check exit 1은 expected.json에 기록한다. Runtime guard_abort는 기존 Int32 overflow·operation Span·before-only 출력·exit 1이다.
P24/P30 literal_pattern과 P31 integer_pattern의 source/원 N1102 metadata는 보존하고 현재 성공을 historical_overrides로 기록한다.
문서 validator는 구조/hash/Span 검사이며 Compiler/Native 실행과 별도다.

```powershell
cargo run -p nova-cli --offline -- check docs/development-v0.1/integer-pattern-proposal-fixtures/main.nova
$env:NOVA_CLANG=(Resolve-Path target/toolchains/llvm-21.1.8/bin/clang.exe).Path
cargo run -p nova-cli --offline -- run docs/development-v0.1/integer-pattern-proposal-fixtures/main.nova --profile debug
cargo run -p nova-cli --offline -- check docs/development-v0.1/integer-pattern-proposal-fixtures/duplicate_radix.nova
```
