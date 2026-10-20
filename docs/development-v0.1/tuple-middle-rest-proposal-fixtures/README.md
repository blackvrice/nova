# P38 Copy Tuple 중간 rest 패턴 수용 fixture

Accepted / 구현·검증 완료. [계약](../TUPLE_MIDDLE_REST_STAGE_B_PROPOSAL.md)·[70-production EBNF](../GRAMMAR_STAGE_B_TUPLE_MIDDLE_REST.ebnf)·[구현 기록](../TUPLE_MIDDLE_REST_IMPLEMENTATION.md).
28개 UTF-8/LF source SHA-256을 고정한다. main/values 두 파일·정상 3·부정 21·Runtime 2다.

```text
scrutinee
first
second
nested=-7
initializer
omitted
let=3
var=5/true
const=9/-7
one
rest-only
complete=3
struct=5
option=6
try-ok
result=1
error=4
loop=1
```

debug/release의 LF·빈 stderr·exit 0이다. leading/middle/trailing/rest-only·nested product/sum·local/global const/default·suffix var·guard/try/loop를 관측했다.
initializer/omitted 두 줄은 생략한 중간 요소의 함수 호출도 전체 initializer에서 한 번 평가됨을 보여 준다.
full_and_single은 한 요소·빈 생략·전체 prefix/suffix·기존 Unit 패턴을, float_copy는 suffix float 상수를, guard_skipped는 suffix 불일치 뒤 guard 생략을 검사한다.
부정 21개는 실제 code별 byte Span·check exit 1을 expected.json에 기록한다. Unit/scalar 적용은 N2101, 과도한 explicit 요소는 N2201이다. struct/Enum payload의 middle rest는 계속 N1102다.
const_omitted_failure는 생략한 요소의 계산도 실패하며 N3201/파생 억제를 유지한다.
Runtime guard_abort/omitted_abort는 Int32 overflow·operation Span·before-only 출력·exit 1을 확인한다.
P36의 고정 middle_rest.nova와 당시 N1102 metadata는 보존한다. historical_overrides는 동일 source hash의 현재 check/Native 성공을 별도로 기록한다.
문서 validator는 구조/hash/Span 검사이며 Compiler/Native 실행과 별도다.

```powershell
cargo run -p nova-cli --offline -- check docs/development-v0.1/tuple-middle-rest-proposal-fixtures/main.nova
$env:NOVA_CLANG=(Resolve-Path target/toolchains/llvm-21.1.8/bin/clang.exe).Path
cargo run -p nova-cli --offline -- run docs/development-v0.1/tuple-middle-rest-proposal-fixtures/main.nova --profile debug
cargo run -p nova-cli --offline -- check docs/development-v0.1/tuple-middle-rest-proposal-fixtures/too_many.nova
```
