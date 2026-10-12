# P25 Copy match guard 수용 fixture — Draft

**사용자 승인 대기 / 미구현**. [계약](../GUARD_STAGE_B_PROPOSAL.md)·[61-production 제안 EBNF](../GRAMMAR_STAGE_B_GUARD.ebnf)을 따른다.
두 파일 main/types·추가 정상 3·부정 16·Runtime 1이다. source 22개는 UTF-8/LF다.
expected.json의 proposed_result는 향후 수용 기준이며 implementation_verified=false다. 현재 Compiler/Native 성공 기록이 아니다.

## 제안된 main 출력

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

## 승인 후 구현에 사용할 수용 명령

아래 명령은 **아직 성공하지 않는 향후 수용 명령**이다. 구현 후 debug/release의 위 16줄·빈 stderr·exit 0을 검증한다.

```powershell
cargo run -p nova-cli --offline -- check docs/development-v0.1/guard-proposal-fixtures/main.nova --source-root docs/development-v0.1/guard-proposal-fixtures
$env:NOVA_CLANG = (Resolve-Path target/toolchains/llvm-21.1.8/bin/clang.exe).Path
cargo run -p nova-cli --offline -- run docs/development-v0.1/guard-proposal-fixtures/main.nova --source-root docs/development-v0.1/guard-proposal-fixtures --profile debug
cargo run -p nova-cli --offline -- run docs/development-v0.1/guard-proposal-fixtures/main.nova --source-root docs/development-v0.1/guard-proposal-fixtures --profile release
```

현재 구현된 P24를 직접 실행하려면 [TESTING.md](../../../TESTING.md)의 examples/nested_patterns.nova 명령을 사용한다.
