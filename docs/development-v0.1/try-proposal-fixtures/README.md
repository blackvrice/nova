# P16 Copy try·Result 오류 전파 수용 fixture

상태 **Draft / 사용자 승인 대기 / 미구현**.
[최소 계약](../TRY_STAGE_B_PROPOSAL.md)과 [51-production EBNF](../GRAMMAR_STAGE_B_TRY.ebnf)의 검토용 source·제안 기대값이다.
현재 Compiler는 try를 거부한다. 아래 출력은 아직 실행한 결과가 아니다.

## 정상 프로그램과 제안 출력

main/effects 두 파일은 type import alias, Source/destination 성공 타입 차이, single evaluation·left-to-right,
Error에서 미실행 sibling/statement, nested try, Unit, Copy snapshot, Bool short-circuit와 while continue를 포함한다.
stdout은 다음 19줄과 각 LF, stderr 없음, exit 0을 제안한다.
실패 pipeline에는 second/after가 없고 shortCircuit에는 truth가 없어야 한다.

```text
start
leaf
second
after
ok=8
start
leaf
error=-1
leaf
nested=7
leaf
nested-error=-1
ping
unit=success
snapshot=9
short=false
leaf
leaf
loop=7
```

아래 명령은 **P16 승인·구현 후** 사용할 명령이다. 현재 성공 결과가 아니다.

```powershell
cargo run -p nova-cli --offline -- check docs/development-v0.1/try-proposal-fixtures/main.nova --source-root docs/development-v0.1/try-proposal-fixtures
$env:NOVA_CLANG = (Resolve-Path target/toolchains/llvm-21.1.8/bin/clang.exe).Path
cargo run -p nova-cli --offline -- run docs/development-v0.1/try-proposal-fixtures/main.nova --source-root docs/development-v0.1/try-proposal-fixtures --profile debug
cargo run -p nova-cli --offline -- run docs/development-v0.1/try-proposal-fixtures/main.nova --source-root docs/development-v0.1/try-proposal-fixtures --profile release
```

## 부정 사례

[expected.json](expected.json)의 18사례는 source/code/UTF-8 primary byte Span의 제안 기대값이다.
const legality 우선순위와 ErrorType cascade 억제는 forbidden_diagnostics도 지정한다.
문서 validator의 성공은 Compiler/Native 실행 증거가 아니다. 승인 후 실제 회귀 테스트로 연결한다.

현재 실행 가능한 P15 예제와 명령은 [TESTING.md](../../../TESTING.md)를 따른다.
