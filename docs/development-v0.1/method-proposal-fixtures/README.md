# P22 Copy struct Read 메서드 수용 fixture — Draft

**미승인 / 미구현**. [계약](../METHOD_STAGE_B_PROPOSAL.md)·[58-production 문법](../GRAMMAR_STAGE_B_METHOD.ebnf)을 검토한다.
main/types 두 파일·정상 1·부정 20·Runtime 1사례다. expected.json의 proposed_result는 제안이며 실행 성공 기록이 아니다.

## 제안 출력

```text
sum=8
alias=7
copy=3/5
text=3/4
private=3
member-main=4
tuple=7
receiver
arg
order=9
fetch
after
ok=11
fetch
error=-1
```

15줄과 각 LF·빈 stderr·exit 0을 기대한다. Read self·forward/alias/nominal owner·private 호출·정의 module default·
mutable local Copy·Tuple receiver·member main 비entry·receiver-first 단일 평가·named 인수·try Error early return을 포함한다.
newline_and_self.nova는 trailing receiver comma·D05 dot continuation·일반 self 이름을 검토한다.
부정 20사례의 primary는 expected.json의 UTF-8 byte Span이며 구현 후 실제 진단과 비교할 계획이다.
Runtime method_abort는 before LF만 출력하고 원 self.x+1 Span의 checked overflow로 Abort할 계획이다.

## 현재 확인과 승인 후 실행

현재 P21 compiler는 struct method func 위치에 N1102를 보고한다. 다음 check는 현재 실패하는 것이 정상이다.

```powershell
cargo run -p nova-cli --offline -- check docs/development-v0.1/method-proposal-fixtures/main.nova --source-root docs/development-v0.1/method-proposal-fixtures
```

승인·구현 후 같은 check는 exit 0, 다음 debug/release는 위 15줄이어야 한다.

```powershell
$env:NOVA_CLANG = (Resolve-Path target/toolchains/llvm-21.1.8/bin/clang.exe).Path
cargo run -p nova-cli --offline -- run docs/development-v0.1/method-proposal-fixtures/main.nova --source-root docs/development-v0.1/method-proposal-fixtures --profile debug
cargo run -p nova-cli --offline -- run docs/development-v0.1/method-proposal-fixtures/main.nova --source-root docs/development-v0.1/method-proposal-fixtures --profile release
```

현재 구현 P21의 직접 실행은 [TESTING.md](../../../TESTING.md)를 따른다.
문서 validator는 구조·링크·hash·grammar·fixture bytes/Span을 검사하며 Compiler/Native를 실행하지 않는다.
