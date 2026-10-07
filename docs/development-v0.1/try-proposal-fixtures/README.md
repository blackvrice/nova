# P16 Copy try·Result 오류 전파 수용 fixture

상태 **Accepted / 사용자 승인 완료 / 구현 완료**.
[최소 계약](../TRY_STAGE_B_PROPOSAL.md)과 [51-production EBNF](../GRAMMAR_STAGE_B_TRY.ebnf)의 수용 source·검증 기대값이다.
[구현 기록](../TRY_IMPLEMENTATION.md)의 Windows Native debug/release에서 실제 실행했다.

## 정상 프로그램과 검증 출력

main/effects 두 파일은 type import alias, Source/destination 성공 타입 차이, single evaluation·left-to-right,
Error에서 미실행 sibling/statement, nested try, Unit, Copy snapshot, Bool short-circuit와 while continue를 포함한다.
stdout은 다음 19줄과 각 LF, stderr 없음, exit 0이다.
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

저장소 root의 PowerShell에서 아래 명령으로 직접 확인한다. check는 출력 없이 exit 0이다.

```powershell
cargo run -p nova-cli --offline -- check docs/development-v0.1/try-proposal-fixtures/main.nova --source-root docs/development-v0.1/try-proposal-fixtures
$env:NOVA_CLANG = (Resolve-Path target/toolchains/llvm-21.1.8/bin/clang.exe).Path
cargo run -p nova-cli --offline -- run docs/development-v0.1/try-proposal-fixtures/main.nova --source-root docs/development-v0.1/try-proposal-fixtures --profile debug
cargo run -p nova-cli --offline -- run docs/development-v0.1/try-proposal-fixtures/main.nova --source-root docs/development-v0.1/try-proposal-fixtures --profile release
```

## 부정 사례

[expected.json](expected.json)의 18사례는 source/code/UTF-8 primary byte Span의 검증 기대값이다.
const legality 우선순위와 ErrorType cascade 억제는 forbidden_diagnostics도 지정한다.
문서 validator의 성공은 Compiler/Native 실행 증거가 아니다. 실제 Compiler 회귀 테스트로 code/Span/cascade를 확인했다.

독립 P16 예제와 현재 기능 전체의 명령은 [TESTING.md](../../../TESTING.md)를 따른다.
