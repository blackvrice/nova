# P18 상수 표현식 함수 기본 인수 제안 fixture

상태 **Accepted / 2026-10-07 사용자 승인 / 구현·검증 완료**.
[최소 계약](../DEFAULT_ARGUMENTS_STAGE_B_PROPOSAL.md)·[52-production EBNF](../GRAMMAR_STAGE_B_DEFAULT_ARGUMENTS.ebnf)의 수용 source/검증된 기대값이다.
[구현·검증 기록](../DEFAULT_ARGUMENTS_IMPLEMENTATION.md)을 따른다. 아래 출력은 실제 Windows Native debug/release 결과다.

## 정상 프로그램과 검증된 출력

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

## 직접 실행할 명령

아래는 저장소 root PowerShell에서 실행한다. [독립 예제·자동 테스트 명령](../../../TESTING.md)도 제공한다.

```powershell
cargo run -p nova-cli --offline -- check docs/development-v0.1/default-arguments-proposal-fixtures/main.nova --source-root docs/development-v0.1/default-arguments-proposal-fixtures
$env:NOVA_CLANG = (Resolve-Path target/toolchains/llvm-21.1.8/bin/clang.exe).Path
cargo run -p nova-cli --offline -- run docs/development-v0.1/default-arguments-proposal-fixtures/main.nova --source-root docs/development-v0.1/default-arguments-proposal-fixtures --profile debug
cargo run -p nova-cli --offline -- run docs/development-v0.1/default-arguments-proposal-fixtures/main.nova --source-root docs/development-v0.1/default-arguments-proposal-fixtures --profile release
```

check의 검증된 기대값은 출력 없이 exit 0, 두 Native 실행은 위 20줄이다.

## 별도 정상·부정 사례와 검증 경계

[Unicode print shadow](unicode_print_shadow.nova)는 global과 parameter의 같은 철자/default lookup·caller local shadow·사용자 print를,
[forward/recursive/grouped](forward_recursive_grouped.nova)는 원 signature의 default identity와 재귀/괄호 호출을 검사할 별도 정상 source다.
두 사례 모두 check 및 Native debug/release에서 stdout/stderr 없음·exit 0으로 검증했다.

[expected.json](expected.json)의 부정 20개는 검증된 code·정확한 UTF-8 byte primary Span·cascade 금지 데이터다.
잘못된 default type/range/checked 연산·사용하지 않는 default 실패·parameter/self/caller 이름·함수 호출/보간/try·
생략된 RHS의 금지 call·default 없는 required 누락·unknown/duplicate/order/excess·const 함수 호출·builtin label·private import를 포함한다.
P11 private import N2004는 기존대로 END를 포함한 전체 import를 primary로 한다.

문서 validator는 Accepted ledger/grammar/Span/검증 metadata를 검사한다. 실제 Compiler·LLVM·Native 증거는 별도 구현 기록에 있다.
초안의 기존 P17 대조 실행은 과거 검산 기록이며 현재 P18 source의 검증과 구분한다.
