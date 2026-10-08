# P20 Copy Option postfix exists 수용 fixture — Accepted

상태: **사용자 승인 / 구현·검증 완료**. [최소 계약](../EXISTS_STAGE_B_PROPOSAL.md)을 따른다.
main/helpers 두 파일·독립 정상 2개·부정 20개·Runtime 실패 1개를 제공한다.
expected.json의 validated_result는 Compiler와 Windows Native debug/release로 확인한 기대값이다.

## 두 파일 정상 사례

main.nova는 import/alias·private nominal payload·nested None·forward const/default·snapshot·
named argument effect 순서·short-circuit·prefix 결합·Result try 성공/오류를 검사한다.
아래 18줄에 각각 LF, 빈 stderr, exit 0이 검증된 기대값이다.

```text
basic=true/false
payload=true/true
nested=true/true
const=true/true
default=true
once
once=true
short=false/true
snapshot=true/false
tuple=true
negate=true
second
first
order=false
try
try-ok=true
try
try-error=-1
```

## PowerShell 실행 명령

저장소 root에서 실행한다. 현재 P20 compiler로 check/debug/release가 성공한다.

```powershell
cargo run -p nova-cli --offline -- check docs/development-v0.1/exists-proposal-fixtures/main.nova --source-root docs/development-v0.1/exists-proposal-fixtures
$env:NOVA_CLANG = (Resolve-Path target/toolchains/llvm-21.1.8/bin/clang.exe).Path
cargo run -p nova-cli --offline -- run docs/development-v0.1/exists-proposal-fixtures/main.nova --source-root docs/development-v0.1/exists-proposal-fixtures --profile debug
cargo run -p nova-cli --offline -- run docs/development-v0.1/exists-proposal-fixtures/main.nova --source-root docs/development-v0.1/exists-proposal-fixtures --profile release
cargo run -p nova-cli --offline -- check docs/development-v0.1/exists-proposal-fixtures/undefined_operand.nova
cargo run -p nova-cli --offline -- run docs/development-v0.1/exists-proposal-fixtures/operand_abort.nova --profile debug
```

undefined_operand는 N2001/exit 1과 exists 파생 N2101 억제를 기대한다.
operand_abort는 before LF만 출력하고 arithmetic 원 Span의 N5201/비정상 exit를 기대한다.
부정 사례 primary는 [expected.json](expected.json)의 UTF-8 byte Span이다.
독립 예제와 전체 테스트 명령은 [TESTING.md](../../../TESTING.md)에 있다.
문서 validator는 fixture bytes·code·Span·metadata를 검사하며 Compiler/Native를 실행하지 않는다.
