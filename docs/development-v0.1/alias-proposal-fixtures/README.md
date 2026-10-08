# P21 Type Alias 수용 fixture — Draft

**미승인 / 미구현**. [계약](../ALIAS_STAGE_B_PROPOSAL.md)과 [55-production 문법](../GRAMMAR_STAGE_B_ALIAS.ebnf)을 검토한다.
main/types 두 파일·정상 1·부정 16사례다. multiline.nova는 RHS generic/tuple의 줄바꿈과 뒤 comparison을 검사한다. expected.json의 proposed_result는 제안이며 실행 성공 기록이 아니다.

## 제안 출력

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

각각 LF·빈 stderr·exit 0을 기대한다. 선언 module scope·forward alias·nominal identity·
Tuple/Option/Result·String/Unit·const/default·cast·try/exists를 포함한다.
부정 사례의 primary는 expected.json의 UTF-8 byte Span이다. 전체 target cyclic alias는 N2103,
nominal layout cycle은 기존 N2101이다. generic alias의 primary는 첫 <다.

## 현재 확인과 승인 후 실행

현재 compiler는 type 선언에서 N1102를 보고하므로 아래 check가 실패하는 것이 정상이다.

```powershell
cargo run -p nova-cli --offline -- check docs/development-v0.1/alias-proposal-fixtures/main.nova --source-root docs/development-v0.1/alias-proposal-fixtures
```

승인·구현 후 같은 check는 exit 0이어야 한다. 이후 다음 명령의 8줄 출력을 확인한다.

```powershell
$env:NOVA_CLANG = (Resolve-Path target/toolchains/llvm-21.1.8/bin/clang.exe).Path
cargo run -p nova-cli --offline -- run docs/development-v0.1/alias-proposal-fixtures/main.nova --source-root docs/development-v0.1/alias-proposal-fixtures --profile debug
cargo run -p nova-cli --offline -- run docs/development-v0.1/alias-proposal-fixtures/main.nova --source-root docs/development-v0.1/alias-proposal-fixtures --profile release
```

현재 구현 P20의 직접 실행은 [TESTING.md](../../../TESTING.md)를 따른다.
문서 validator는 구조·링크·hash·grammar·fixture bytes/Span을 검사하며 Compiler/Native를 실행하지 않는다.
