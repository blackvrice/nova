# P22 Copy struct Read 메서드 수용 fixture — Accepted

**2026-10-08 사용자 승인 / 구현 완료**. [계약](../METHOD_STAGE_B_PROPOSAL.md)·[58-production 문법](../GRAMMAR_STAGE_B_METHOD.ebnf)을 검토한다.
main/types 두 파일·정상 1·부정 20·Runtime 1사례다. expected.json의 validated_result와 원 UTF-8 Span을 Compiler/Native에서 검증했다.

## 검증된 출력

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
부정 20사례의 primary는 expected.json의 UTF-8 byte Span이며 실제 진단과 모두 일치했다.
Runtime method_abort는 before LF만 출력하고 원 self.x+1 Span의 checked overflow로 Abort한다.

## 직접 실행

다음 check는 출력 없이 exit 0이다. Draft source 주석은 초안 작성 시점 기록이며 bytes/Span을 보존했다.

```powershell
cargo run -p nova-cli --offline -- check docs/development-v0.1/method-proposal-fixtures/main.nova --source-root docs/development-v0.1/method-proposal-fixtures
```

다음 debug/release는 위 15줄/LF·빈 stderr·exit 0이다.

```powershell
$env:NOVA_CLANG = (Resolve-Path target/toolchains/llvm-21.1.8/bin/clang.exe).Path
cargo run -p nova-cli --offline -- run docs/development-v0.1/method-proposal-fixtures/main.nova --source-root docs/development-v0.1/method-proposal-fixtures --profile debug
cargo run -p nova-cli --offline -- run docs/development-v0.1/method-proposal-fixtures/main.nova --source-root docs/development-v0.1/method-proposal-fixtures --profile release
```

현재 구현 P22의 직접 실행은 [TESTING.md](../../../TESTING.md)를 따른다.
문서 validator는 구조·링크·hash·grammar·fixture bytes/Span을 검사하며 Compiler/Native를 실행하지 않는다.
