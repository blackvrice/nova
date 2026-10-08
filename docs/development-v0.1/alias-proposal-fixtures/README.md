# P21 Type Alias 수용 fixture — Accepted

2026-10-08 사용자 승인 계약을 구현했다. [계약](../ALIAS_STAGE_B_PROPOSAL.md)·[55-production 문법](../GRAMMAR_STAGE_B_ALIAS.ebnf)·[구현 기록](../ALIAS_IMPLEMENTATION.md)을 따른다.
main/types 두 파일·정상 1·부정 16사례다. multiline.nova는 RHS generic/Tuple 줄바꿈과 뒤 comparison을 검사한다.
expected.json의 validated_result는 실제 check·Windows Native debug/release 결과다.
원 fixture 소스의 Draft 주석은 초안 작성 시점의 역사 기록으로 보존했다.

## 검증된 출력

```text
small=7
pair=7/2
exists=true
flag=on
cast=7
text=한글
try=true
error=-1
```

각 LF·빈 stderr·exit 0이다. 선언 module scope·forward alias·nominal identity·Tuple/Option/Result·String/Unit·const/default·cast·try/exists를 포함한다.
부정 사례는 expected.json의 UTF-8 byte Span과 실제 진단을 비교했다. cyclic alias는 RHS 전체 N2103, nominal layout cycle은 기존 N2101이다.
generic alias의 primary는 첫 <다. alias constructor/variant head는 N1102이며 원 nominal head를 사용한다.

## 직접 실행

```powershell
cargo run -p nova-cli --offline -- check docs/development-v0.1/alias-proposal-fixtures/main.nova --source-root docs/development-v0.1/alias-proposal-fixtures
$env:NOVA_CLANG = (Resolve-Path target/toolchains/llvm-21.1.8/bin/clang.exe).Path
cargo run -p nova-cli --offline -- run docs/development-v0.1/alias-proposal-fixtures/main.nova --source-root docs/development-v0.1/alias-proposal-fixtures --profile debug
cargo run -p nova-cli --offline -- run docs/development-v0.1/alias-proposal-fixtures/main.nova --source-root docs/development-v0.1/alias-proposal-fixtures --profile release
cargo run -p nova-cli --offline -- check docs/development-v0.1/alias-proposal-fixtures/self_cycle.nova
```

정상 check는 출력 없이 exit 0, 마지막 명령은 N2103·exit 1이다.
[독립 예제](../../../examples/type_aliases.nova)·[기능별 테스트 명령](../../../TESTING.md)을 제공한다.
문서 validator는 구조·링크·hash·grammar·fixture bytes/Span을 검사하며 Compiler/Native를 실행하지 않는다.
