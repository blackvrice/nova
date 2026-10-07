# P18 상수 표현식 함수 기본 인수 제안 fixture

상태 **Draft / 사용자 승인 대기 / 미구현**.
[최소 계약](../DEFAULT_ARGUMENTS_STAGE_B_PROPOSAL.md)·[52-production EBNF](../GRAMMAR_STAGE_B_DEFAULT_ARGUMENTS.ebnf)의 제안 source/기대값이다.
현재 P17 compiler는 parameter default를 거부한다. 아래 출력은 구현 후 수용 목표이며 실제 P18 실행 결과가 아니다.

## 정상 프로그램과 제안 출력

main/helpers 두 파일에서 all-default/부분 positional/named·import alias·private global의 declaration-site scope·caller shadow,
필수/default parameter 혼합·String/Unit/Copy tuple/Char·nullable/Result 생성 문맥·provided effect/try·Bool short-circuit를 검사한다.
기대 stdout은 다음 20줄과 각 LF, stderr 없음·exit 0이다.

```text
defaults=620
right
named=602
left
positional=120
second
first
reverse=702
holes=456
text=default/provided
unit=3
copy=4/🙂
option=none
result=7
leaf
after
success=602
leaf
error=-1
short=false
```

이전의 `SECRET=6`은 helpers의 private global이다. caller의 같은 이름 `SECRET=99`가 default를 바꾸지 않아야 한다.
명시적으로 제공한 right가 먼저 실행되고 left default는 caller에서 이후 materialize한다.
default는 상수로 제한되므로 default-side effect 출력은 없다. MIR provenance/위조 검사는 별도 수용 기준이다.

## 승인·구현 후 실행할 명령

아래는 **P18 구현 후** 저장소 root PowerShell에서 실행한다. 현재 실행 가능한 P17 명령은 [TESTING.md](../../../TESTING.md)에 있다.

```powershell
cargo run -p nova-cli --offline -- check docs/development-v0.1/default-arguments-proposal-fixtures/main.nova --source-root docs/development-v0.1/default-arguments-proposal-fixtures
$env:NOVA_CLANG = (Resolve-Path target/toolchains/llvm-21.1.8/bin/clang.exe).Path
cargo run -p nova-cli --offline -- run docs/development-v0.1/default-arguments-proposal-fixtures/main.nova --source-root docs/development-v0.1/default-arguments-proposal-fixtures --profile debug
cargo run -p nova-cli --offline -- run docs/development-v0.1/default-arguments-proposal-fixtures/main.nova --source-root docs/development-v0.1/default-arguments-proposal-fixtures --profile release
```

check의 제안 기대값은 출력 없이 exit 0, 두 Native 실행은 위 20줄이다.

## 별도 정상·부정 사례와 검증 경계

[Unicode print shadow](unicode_print_shadow.nova)는 global과 parameter의 같은 철자/default lookup·caller local shadow·사용자 print를,
[forward/recursive/grouped](forward_recursive_grouped.nova)는 원 signature의 default identity와 재귀/괄호 호출을 검사할 별도 정상 source다.
두 사례의 제안 check/run 기대값은 stdout/stderr 없음·exit 0이다. 아직 P18 Compiler로 검증한 결과가 아니다.

[expected.json](expected.json)의 부정 20개는 제안 code·정확한 UTF-8 byte primary Span·cascade 금지 데이터다.
잘못된 default type/range/checked 연산·사용하지 않는 default 실패·parameter/self/caller 이름·함수 호출/보간/try·
생략된 RHS의 금지 call·default 없는 required 누락·unknown/duplicate/order/excess·const 함수 호출·builtin label·private import를 포함한다.
P11 private import N2004는 기존대로 END를 포함한 전체 import를 primary로 한다.

문서 validator는 Draft ledger/grammar/Span/제안 metadata만 검사한다. Compiler·LLVM·Native 수용 증거는 승인 후 구현한다.
기존 P17 문법의 명시적 값만 사용하는 ignored target 대조 프로그램의 실행 결과는 proposal의 초안 준비 검증에 따로 기록한다.
