# P35 Copy struct trailing rest 패턴 구현·검증

2026-10-19. Accepted / 구현·검증 완료.
사용자 “다음 작업 승인할테니 개발 진행해줘”에 따라 [제한 계약](STRUCT_REST_PATTERN_STAGE_B_PROPOSAL.md)을 구현 전에 기록했다.
전체 D06/D08/D09/D10/D12/D16/D25/D30 승인이 아니다.

## Implemented / Changed

Parser는 struct argument의 마지막 ..만 marker로 저장한다. 기존 인접 Dot token 두 개를 사용하고 사이 trivia·중복/중간 rest·rest binder/value/tuple/variant/root는 거부한다.
AST/HIR PatternRest는 정확한 .. Span과 child 없는 marker다. spelling·parent placement·trailing order/shape·delimiter/trivia를 검증한다.
P34 field 대응 helper는 rest를 field ordinal/label로 세지 않고 missing-field 검사만 생략한다. rest 없는 기존 arity/label/누락 진단을 유지한다.
P33 nominal/import head·alias annotation·유효 Copy layout·전체 field 가시성을 보존한다. 다른 module의 private field는 생략하더라도 N2004다.
AST/HIR와 binder/타입 진단 방문은 Source 순서다. Checked children은 명시한 field만 선언 순서로 정렬하고 field_indices를 일대일 저장한다.
rest marker는 Unit/Wildcard typed bookkeeping으로 source node 한 개를 세며 값/binder/projection으로 쓰지 않는다. implicit wildcard에 synthetic Source/HIR node를 만들지 않는다.
Matrix는 struct 전체 field vector를 wildcard로 만들고 선택한 field_indices만 채운다. N3101/N3102·guard 없는 합집합·opaque-only shortcut·위치 witness·자원 한도를 보존한다.
let/var/const의 N3103는 명시한 component에 적용하고 Source 순서의 첫 refutable secondary를 유지한다.
MIR은 전체 initializer/scrutinee를 한 번 평가·snapshot하고 명시한 field만 실제 FieldId로 projection한다. 생략한 constructor operand의 effect/Abort/try는 그대로다.
const도 전체 initializer를 평가한 뒤 sparse field 번호로 binder cache를 저장한다. 생략한 field의 실패·전역 forward/순환·default·기존 10,000-node 예산을 유지한다.
Checked indices/children/type/cache와 MIR 같은 타입 field index/Source 변조를 원 HIR 재계산 및 private full-body proof로 거부한다.
Grammar는 struct_pattern_arguments를 확장하고 struct_pattern_rest 하나를 추가해 67 productions다. 나머지 65개를 보존한다.
LLVM/Runtime production·ABI/layout·Lexer/END·Cargo dependencies·diagnostic registry는 변경하지 않았다.

## Tests Added / Validation

새 테스트 17개: 기본 14·LLVM opt-in 1·Native opt-in 2.
rest-only/empty/전체 명시 뒤 rest·역순 label/위치 prefix·sparse nonzero field index/타입/range·누락/중복·scope·private 생략·mutable binder·guard/try를 검사한다.
네 sparse Bool pattern region의 16개 부분집합에 대한 독립 합집합 oracle로 completeness/reachability를 비교했다.
local/global const의 실제 field 번호/cached 값·forward/default·cycle·생략한 field의 const 실패·N3103 파생 오류 억제를 검사한다.
rest를 포함한 정확한 10,000/10,001-node 경계와 implicit wildcard Source node 부재를 검사한다. 모든 UTF-8 prefix의 결정적 Parser 복구도 확인한다.
AST marker spelling/shape/order/parent, Checked sparse indices/child/cache와 MIR 같은 타입 FieldId/Source 변조를 거부한다.
MIR make 호출은 initializer/match마다 한 번, struct field projection은 명시한 index만 있음을 확인했다.
P33/P34의 당시 P(..) 거부 source를 현재 Parser 성공 assertion으로 옮기고 고정 fixtures/grammar bytes를 보존했다.

| 검사 | 실제 결과 |
|---|---|
| fmt / Runtime rustfmt / all-features check / all-targets all-features Clippy -D warnings | PASS |
| 전체 기본 tests | 538 PASS / 0 FAIL / 108 ignored |
| LLVM 전체 opt-in COFF/ELF O0/O2 | 31 PASS / 0 FAIL |
| Windows Native 전체 opt-in debug/release | 77 PASS / 0 FAIL |

총 646 PASS / 0 FAIL. Native는 P35 두 개를 먼저 실행하고 기존 75개를 --skip p35_로 실행해 중복 없는 전체 77개를 검증했다.
[고정 fixture](struct-rest-pattern-proposal-fixtures/README.md)는 두 파일·정상 3·부정 21 code별 byte Span·Runtime 2를 검증했다.
Native 두 profile에서 single scrutinee/initializer·생략 field의 함수 효과/Abort·sparse const/var/default·nested Option·guard 순서/생략·try/loop를 관측했다.
LLVM은 결정적 sparse projection/equality IR과 COFF/ELF O0/O2 object를 확인했다.
[독립 예제](../../examples/struct_rest_patterns.nova)는 flag=true, mutable=5, match=7 세 줄/LF·빈 stderr·exit 0이다.

## Specification / Remaining

관련 NOVA-026/029/035/038/044/046/047/048/073/074/076/078/081/083/087/091/094/095/136 보완 문서에 제한된 P35 bridge를 추가했다.
P01~P34 ledger·D01~D30 본문·Canonical·원본 148개·기존 fixtures/grammar bytes를 보존하고 문서 validator/generation idempotency를 별도 검증했다.
Rust 1.99.0·LLVM 21.1.8·Windows x64 MSVC에서 검증했다. MSRV 1.80·Linux Native host 실행은 이번에 검증하지 않았다.
CARGO_INCREMENTAL=0은 검증 프로세스에만 설정했다. Windows hard-link/junction 기본 검사는 sandbox 밖에서 실행했다.
shorthand/brace·Tuple/Enum/sum payload rest·rest binder·private 접근 완화·field default·or/range 패턴·Array·String/Move field·parameter/for/assignment pattern·const 함수·일반 Move/borrow/Drop은 후속 계약이다.
[TESTING.md](../../TESTING.md)에 직접 실행 명령을 제공한다.
