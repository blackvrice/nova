# P29 전역 Copy const 구조 분해 수용 fixture

Accepted / 구현·검증 완료. [계약](../GLOBAL_CONST_DESTRUCTURING_STAGE_B_PROPOSAL.md)·[EBNF](../GRAMMAR_STAGE_B_GLOBAL_CONST_DESTRUCTURING.ebnf)·[구현 기록](../GLOBAL_CONST_DESTRUCTURING_IMPLEMENTATION.md).
6개 UTF-8/LF source의 SHA-256을 고정한다. main/values 두 파일과 부정 4개다.
전역 component import/alias·forward reference·private 가시성·single-variant nested pattern·function default·local const 사용을 검사한다.

```text
global=8/true/🙂
local=9
```

check exit 0. debug/release 위 출력·빈 stderr·exit 0을 관측했다.
cycle N3202·discard division-by-zero N3201·private import N2004·duplicate N2002의 check exit 1과 UTF-8 byte Span을 expected.json에 기록한다.
P28 전역 거부 source/metadata는 보존하며 historical_overrides로 현재 성공을 기록한다.
문서 validator는 구조/hash/Span 검증이며 Compiler/Native 실행과 별도다.

```powershell
cargo run -p nova-cli --offline -- check docs/development-v0.1/global-const-destructuring-proposal-fixtures/main.nova
$env:NOVA_CLANG=(Resolve-Path target/toolchains/llvm-21.1.8/bin/clang.exe).Path
cargo run -p nova-cli --offline -- run docs/development-v0.1/global-const-destructuring-proposal-fixtures/main.nova --profile debug
cargo run -p nova-cli --offline -- check docs/development-v0.1/global-const-destructuring-proposal-fixtures/cycle.nova
```
