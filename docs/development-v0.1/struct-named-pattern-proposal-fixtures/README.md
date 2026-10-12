# P34 Copy struct 이름 필드 패턴 수용 fixture

Accepted / 구현·검증 완료. [계약](../STRUCT_NAMED_PATTERN_STAGE_B_PROPOSAL.md)·[66-production EBNF](../GRAMMAR_STAGE_B_STRUCT_NAMED_PATTERN.ebnf)·[구현 기록](../STRUCT_NAMED_PATTERN_IMPLEMENTATION.md).
31개 UTF-8/LF source SHA-256을 고정한다. main/values 두 파일·정상 3·부정 25·Runtime 1이다.

```text
scrutinee
first
second
nested=-7/🙂
initializer
let=3/true/가
var=5/true
const=9/true/-7
empty
complete=3
option=5
try-ok
result=1
error=4
loop=1
```

debug/release의 LF·빈 stderr·exit 0이다. type import alias·역순/위치 혼합 label·single scrutinee/initializer·nested struct·let/var/local/global const·default·Empty·Bool 합집합·Option·guard 순서·try·loop를 관측했다.
private_local은 같은 module의 private field, alias_annotation은 type alias annotation, guard_skipped는 실패한 패턴 뒤 guard 생략을 검사한다.
부정 25개의 실제 code별 byte Span·check exit 1은 expected.json에 기록한다. unknown/duplicate/overlap/after-named/missing/method label은 N2201이다.
다른 module의 private field는 label/wildcard여도 N2004이며 binder 이름 중복은 별도 N2002다.
refutable const는 N3103 뒤 평가/파생 오류를 억제한다. const_failure/cycle은 N3201/N3202다. Enum 이름 payload는 이번 범위 밖이다. enum_named의 P01 N1101은 닫는 괄호 삽입 point Span 57..57이며 primary_kind=insertion으로 명시한다. 추가 N1102도 원 CLI 로그에서 확인했다.
Runtime guard_abort는 Int32 overflow·원 operation Span·before-only 출력·exit 1이다. 문서 validator는 구조/hash/Span 검사이며 Compiler/Native 실행과 별도다.

```powershell
cargo run -p nova-cli --offline -- check docs/development-v0.1/struct-named-pattern-proposal-fixtures/main.nova
$env:NOVA_CLANG=(Resolve-Path target/toolchains/llvm-21.1.8/bin/clang.exe).Path
cargo run -p nova-cli --offline -- run docs/development-v0.1/struct-named-pattern-proposal-fixtures/main.nova --profile debug
cargo run -p nova-cli --offline -- check docs/development-v0.1/struct-named-pattern-proposal-fixtures/duplicate_label.nova
```
