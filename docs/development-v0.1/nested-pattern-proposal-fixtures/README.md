# P24 중첩 Copy 패턴·Tuple match 수용 fixture — Accepted

2026-10-08 사용자 승인. **Accepted / 구현·검증 완료**.
[최소 계약](../NESTED_PATTERN_STAGE_B_PROPOSAL.md)·[60-production EBNF](../GRAMMAR_STAGE_B_NESTED_PATTERN.ebnf)·[구현 기록](../NESTED_PATTERN_IMPLEMENTATION.md)을 따른다.
두 파일 main/types·추가 정상 3·부정 18·Runtime 1과 finite-domain coverage vector 6개다.
expected.json의 **validated_result**는 실제 Compiler/Native 검증 결과다. implementation_verified=true다.
24개 Nova source의 원 UTF-8/LF bytes와 Draft 주석은 승인 전 fixture의 기록으로 보존하며 source_sha256으로 검사한다.

## 검증된 출력

```text
scrutinee
nested=7
packet=9/false/한
snapshot=true/6
original=false/false
single=true
unit
classify=3
after try
ok=4
error=5
loop=2
```

선택 arm·단일 scrutinee 평가·import nominal identity·Unicode·독립 snapshot·singleton Tuple·Unit·전체 return·try early return·loop jump다.
partial_overlap는 앞 arm과 일부 겹쳐도 남은 값을 처리한다. union_coverage는 여러 arm의 합집합으로 완전하다.
bare_binder의 값은 const 조회가 아닌 immutable struct Copy binder다. nested_abort는 선택된 int8 payload에서 checked Abort한다.
부정 사례는 recursive type/arity/scope·중복 binder·누락·합집합 unreachable·제외 문법을 UTF-8 byte Span으로 고정한다.

## 문서·독립 oracle 검사

저장소 루트 PowerShell:

```powershell
node tools/docs/validate-pack.mjs
node tools/tests/nested-pattern-oracle.mjs
cargo fmt --check
cargo test --workspace --offline
```

oracle는 명시적인 작은 유한 domain을 열거해 고정 coverage vector를 검사한다.
문서 validator는 source/Span/grammar/ledger 검사다. 둘 다 Nova Compiler/Native 성공을 증명하지 않는다.

## 직접 실행하는 수용 명령

아래 명령은 check exit 0, debug/release의 위 12줄·빈 stderr·exit 0을 확인한다.

```powershell
cargo run -p nova-cli --offline -- check docs/development-v0.1/nested-pattern-proposal-fixtures/main.nova --source-root docs/development-v0.1/nested-pattern-proposal-fixtures
$env:NOVA_CLANG = (Resolve-Path target/toolchains/llvm-21.1.8/bin/clang.exe).Path
cargo run -p nova-cli --offline -- run docs/development-v0.1/nested-pattern-proposal-fixtures/main.nova --source-root docs/development-v0.1/nested-pattern-proposal-fixtures --profile debug
cargo run -p nova-cli --offline -- run docs/development-v0.1/nested-pattern-proposal-fixtures/main.nova --source-root docs/development-v0.1/nested-pattern-proposal-fixtures --profile release
```

집중 회귀는 `cargo test --workspace --offline p24_`다. 실제 backend 회귀는 NOVA_CLANG 설정 후
`cargo test -p nova-codegen-llvm --test emission --offline p24_ -- --ignored --test-threads=1`과
`cargo test -p nova-cli --test native --offline p24_ -- --ignored --test-threads=1`이다.
[TESTING.md](../../../TESTING.md)의 독립 examples/nested_patterns.nova도 실행할 수 있다.
