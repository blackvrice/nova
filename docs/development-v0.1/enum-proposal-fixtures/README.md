# P14 Copy Enum·statement match 수용 fixture

상태 **Draft / 사용자 승인 대기 / 미구현**. [최소 계약](../ENUM_STAGE_B_PROPOSAL.md),
[48-production EBNF](../GRAMMAR_STAGE_B_ENUM.ebnf)를 검토하기 위한 source와 기대값이다.
문서 validator는 UTF-8 byte Span과 metadata를 확인하며 Compiler/Native를 실행하지 않는다.

## 정상 프로그램과 제안 출력

main.nova는 events.nova의 Event를 E로 import하고 factory가 반환한 Copy snapshot을 분기한다.
원 변수를 Empty로 교체해도 snapshot은 Data다. private type 이름을 main에서 쓰지 않고
추론된 Pair payload의 public field와 Tuple element를 사용한다. Bool match도 검사한다.

```text
make
sum=30, code=7, flag=true
original=empty
ok
```

stdout은 위 네 줄과 각 LF, stderr 없음, exit 0을 제안한다. 아직 실행 검증하지 않았다.
아래 명령은 **P14 승인·구현 후** 실행할 수 있다. 현재 P13 Compiler의 통과 명령으로 안내하지 않는다.

```powershell
cargo run -p nova-cli --offline -- check docs/development-v0.1/enum-proposal-fixtures/main.nova --source-root docs/development-v0.1/enum-proposal-fixtures
$env:NOVA_CLANG = (Resolve-Path target/toolchains/llvm-21.1.8/bin/clang.exe).Path
cargo run -p nova-cli --offline -- run docs/development-v0.1/enum-proposal-fixtures/main.nova --source-root docs/development-v0.1/enum-proposal-fixtures --profile debug
cargo run -p nova-cli --offline -- run docs/development-v0.1/enum-proposal-fixtures/main.nova --source-root docs/development-v0.1/enum-proposal-fixtures --profile release
```

## 부정 사례와 구현 수용 기준

[expected.json](expected.json)의 16개 독립 부정 파일은 code와 primary UTF-8 byte Span을 제안한다.
누락 variant/Bool·중복/가려진 arm·다른 Enum·undefined variant·arity·binder 불변성/중복/scope,
String payload·unused recursion·duplicate variant·nullary call·guard 거부를 포함한다.
다른 파생 진단도 존재할 수 있지만 구현 수용 시 지정 code/Span은 실제 Compiler 결과에 있어야 한다.

한도 경계·독립 layout oracle·스크루티니 effect 1회·while jump/모든 arm return·활성 tag proof
손상 MIR·모든 scalar ABI·mixed/ZST·const budget/cycle은 승인 후 별도 unit/integration/native tests로 검증한다.
현재 실행 가능한 P13 예제 명령은 [TESTING.md](../../../TESTING.md)를 따른다.
