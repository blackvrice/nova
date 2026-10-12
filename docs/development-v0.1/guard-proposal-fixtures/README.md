# P25 Copy match guard 수용 fixture — Accepted

2026-10-12 사용자 승인. **Accepted / 구현·검증 완료**. [계약](../GUARD_STAGE_B_PROPOSAL.md)·[61-production EBNF](../GRAMMAR_STAGE_B_GUARD.ebnf)을 따른다.
두 파일 main/types·추가 정상 3·부정 16·Runtime 1이다. source 22개는 UTF-8/LF다.
expected.json의 validated_result는 실제 Compiler/Native 검증 결과이며 implementation_verified=true다.
22개 Nova source의 원 UTF-8/LF bytes와 Draft 주석은 승인 전 기록으로 보존하고 source_sha256으로 검사한다. [구현 기록](../GUARD_IMPLEMENTATION.md)을 따른다.

## 검증된 main 출력

```text
scrutinee
g1
g2
selected=7
g3
fallback
snapshot-guard
snapshot=true/6
original=false/6
probe
worker-fallback
ok=9
probe
error=5
loop=2
unit
```

guard 함수 trace는 mismatch skip·false continuation·source order를 관측한다.
snapshot 바인더는 original의 body mutation과 독립적이다. worker의 try Success(false)는 fallback을 실행하고 Error는 뒤 arm을 건너뛴다.
loop continue·Unit guard·literal true/false의 보수적 coverage와 Unicode binder scope를 포함한다.
Runtime guard_abort는 Bool guard 계산 중 int8 checked overflow에 Abort하며 fallback을 실행하지 않는다.

## 현재 실행할 수 있는 문서 검사

```powershell
node tools/docs/validate-pack.mjs
node tools/tests/guard-oracle.mjs
cargo fmt --check
cargo test --workspace --offline
```

oracle는 독립 유한 domain의 unguarded coverage/usefulness와 runtime 선택/guard trace를 검사한다.
문서 validator는 source·byte Span·grammar·Draft ledger 검사다. 둘 다 P25 Compiler/Native 성공을 증명하지 않는다.

## 직접 실행하는 수용 명령

아래 명령은 check exit 0·debug/release의 위 16줄·빈 stderr·exit 0을 확인한다.

```powershell
cargo run -p nova-cli --offline -- check docs/development-v0.1/guard-proposal-fixtures/main.nova --source-root docs/development-v0.1/guard-proposal-fixtures
$env:NOVA_CLANG = (Resolve-Path target/toolchains/llvm-21.1.8/bin/clang.exe).Path
cargo run -p nova-cli --offline -- run docs/development-v0.1/guard-proposal-fixtures/main.nova --source-root docs/development-v0.1/guard-proposal-fixtures --profile debug
cargo run -p nova-cli --offline -- run docs/development-v0.1/guard-proposal-fixtures/main.nova --source-root docs/development-v0.1/guard-proposal-fixtures --profile release
```

현재 P25 [독립 예제](../../../examples/match_guards.nova)·[실행 명령](../../../TESTING.md)을 제공한다.
집중 회귀는 `cargo test --workspace --offline p25_`다. NOVA_CLANG 설정 후 실제 LLVM/Native는
`cargo test -p nova-codegen-llvm --test emission --offline p25_ -- --ignored --test-threads=1`과
`cargo test -p nova-cli --test native --offline p25_ -- --ignored --test-threads=1`이다.
