# P17 함수 이름 인수 수용 fixture

상태 **Draft / 사용자 승인 대기 / 미구현**.
[최소 계약](../NAMED_ARGUMENTS_STAGE_B_PROPOSAL.md)·[52-production EBNF](../GRAMMAR_STAGE_B_NAMED_ARGUMENTS.ebnf)의 제안 source와 기대값이다.
현재 P16 compiler로 이름 인수 프로그램이 성공한다고 주장하지 않는다.

## 정상 프로그램과 제안 출력

main/helpers 두 파일에서 원 함수 parameter 이름과 import alias, 뒤집은 인수의 source-order effect,
위치/이름 혼합, mapped int8 literal boundary, Unicode label, String·Unit·Copy aggregate·Option 문맥,
try의 성공/실패와 이전 effect 보존·뒤 인수/callee 미실행·Bool short-circuit를 검사한다.
기대 stdout은 다음 24줄과 각 LF, stderr 없음, exit 0이다. `unreachable`/`skipped`는 출력되지 않는다.

```text
right
left
reverse=702
positional
named
mixed=102
second
first
text=first/second
unit=3
unicode=304
minimum=-12500
copy=4/5
option=7
leaf
later
after
success=102
leaf
error=-1
before
leaf
prior-error=-1
short=false
```

## 승인·구현 후 실행할 명령

아래 명령은 **P17 구현 후** 저장소 root의 PowerShell에서 실행한다.
현재 사용할 P16 예제와 명령은 [TESTING.md](../../../TESTING.md)에 있다.

```powershell
cargo run -p nova-cli --offline -- check docs/development-v0.1/named-arguments-proposal-fixtures/main.nova --source-root docs/development-v0.1/named-arguments-proposal-fixtures
$env:NOVA_CLANG = (Resolve-Path target/toolchains/llvm-21.1.8/bin/clang.exe).Path
cargo run -p nova-cli --offline -- run docs/development-v0.1/named-arguments-proposal-fixtures/main.nova --source-root docs/development-v0.1/named-arguments-proposal-fixtures --profile debug
cargo run -p nova-cli --offline -- run docs/development-v0.1/named-arguments-proposal-fixtures/main.nova --source-root docs/development-v0.1/named-arguments-proposal-fixtures --profile release
```

check 기대값은 출력 없이 exit 0이며 debug/release 기대값은 위 stdout이다.

## 부정 사례와 검증 경계

[function_print_shadow.nova](function_print_shadow.nova)는 builtin을 shadow하는 사용자 함수 print와
label/value namespace 분리를, [forward_recursive_grouped.nova](forward_recursive_grouped.nova)는
선언 전 참조·괄호 direct callee·재귀 이름 호출을 검사할 별도 정상 fixture다.
각 제안 기대값은 check/run exit 0, stdout/stderr 없음이다. 아직 compiler로 검증한 이름 인수 source가 아니다.

[expected.json](expected.json)의 16사례는 **제안** code·정확한 UTF-8 byte primary Span·cascade 금지 데이터다.
unknown/duplicate/positional collision/order/missing/excess, mapped type/range,
builtin/Struct/Enum/sum 생성 label 거부, undefined callee·Unicode label·const call·private import를 포함한다.
문서 validator는 Draft ledger·grammar 참조/변경·byte Span·기대값 상태만 확인한다.
Compiler/Native 수용 검사는 승인 후 구현 단계에서 수행한다. helpers 자체는 기존 P16 문법이다.
