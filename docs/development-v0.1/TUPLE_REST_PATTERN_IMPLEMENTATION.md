# P36 Copy Tuple trailing rest 패턴 구현·검증

2026-10-20. Accepted / 구현·검증 완료.
사용자 “다음 작업 승인할테니 개발 진행해줘”에 따라 [제한 계약](TUPLE_REST_PATTERN_STAGE_B_PROPOSAL.md)을 구현 전에 기록했다.
전체 D06/D08/D09/D10/D12/D16/D25/D30 승인이 아니다.

## Implemented / Changed

Parser는 (first,..), (first,second,..), (..), (..,)와 전체 prefix 뒤 rest를 지원한다. rest는 한 번, 마지막이며 두 점이 인접해야 한다.
P35 struct와 공통 marker 소비 helper를 사용하며 기존 struct 진단 문구/Span을 유지한다. Unit ()·일반 한 요소 Tuple의 comma·rest 없는 arity는 기존 규칙이다.
AST/HIR는 Source 순서의 명시 prefix와 마지막 PatternRest를 보존한다. marker spelling/shape/order/parent·delimiter/trivia·단일 Tuple comma를 검증한다.
(..)도 Type::Tuple을 요구한다. Unit/scalar/struct에는 N2101, 긴 prefix에는 N2201이다. alias/import·중첩 product/sum·정수 문맥·mutable binder를 기존 규칙으로 검사한다.
Checked Tuple children은 명시 prefix만 저장하고 field_indices=0..prefix 길이를 일대일 저장한다. P35 struct의 sparse indices를 유지한다.
rest marker는 Unit/Wildcard typed bookkeeping으로 source node 한 개를 센다. implicit wildcard에 synthetic Source/HIR node를 만들지 않는다.
Matrix는 전체 Tuple field vector를 wildcard로 만들고 명시 prefix 위치만 채운다. N3101/N3102·guard 없는 합집합·위치 witness·자원 한도를 유지한다.
let/var/local/global const는 명시 prefix의 irrefutability·N3103·Source 순서의 첫 refutable secondary·파생 오류 억제를 유지한다.
MIR은 전체 initializer/scrutinee를 한 번 평가·snapshot하고 명시 prefix만 projection한다. 생략한 요소의 effect/Abort/try도 실행한다.
const는 전체 initializer 평가 후 prefix binder cache를 저장한다. 생략 요소 실패·전역 forward/순환·default·10,000-node 예산을 보존한다.
Checked indices/children/type/cache와 MIR 같은 타입 FieldId/Source/guard branch 변조는 원 HIR 재계산 및 private full-body proof로 거부한다.
기존 Parser pattern nesting 128 한도가 struct constructor head에서 누락된 것을 수정했다. Tuple/struct의 128 성공·129 N8901 경계를 검사한다.
Grammar는 tuple_pattern을 확장하고 tuple_pattern_rest 하나를 추가해 68 productions다. 다른 66개는 보존했다.
LLVM/Runtime production·ABI/layout·Lexer/END·Cargo dependencies·diagnostic registry는 변경하지 않았다.

## Tests Added / Validation

새 테스트 19개: 기본 16·LLVM opt-in 1·Native opt-in 2.
rest-only/한 요소/전체 prefix·Unit 구분·arity/type/range·scope/mutability·중첩 product/sum·guard/try를 검사한다.
네 prefix Bool pattern region의 16개 부분집합에 대한 독립 합집합 oracle로 completeness/reachability를 비교했다.
local/global const의 prefix cache·forward/default·cycle·생략 요소 const 실패·N3103 파생 오류 억제를 검사한다.
rest를 포함한 정확한 10,000/10,001 source-node 경계와 implicit wildcard Source node 부재를 검사한다. 모든 UTF-8 prefix의 결정적 Parser 복구도 확인했다.
AST/HIR marker spelling/shape/order와 한 요소 comma 변조, Checked indices/child/cache 및 MIR 같은 타입 FieldId/Source/branch 변조를 거부한다.
MIR make 호출은 initializer/match마다 한 번이며 prefix index 0만 projection하는 것을 확인했다.
P35의 과거 Tuple rest 거부 source·당시 N1102 metadata는 byte 보존하고 P36 historical override 및 현재 성공 assertion을 추가했다.

| 검사 | 실제 결과 |
|---|---|
| fmt / Runtime rustfmt / all-features check / all-targets all-features Clippy -D warnings | PASS |
| 전체 기본 tests | 554 PASS / 0 FAIL / 111 ignored |
| LLVM 전체 opt-in COFF/ELF O0/O2 | 32 PASS / 0 FAIL |
| Windows Native 전체 opt-in debug/release | 79 PASS / 0 FAIL |

총 665 PASS / 0 FAIL. Native는 P36 두 개를 먼저 실행하고 기존 77개를 --skip p36_로 실행해 중복 없는 전체 79개를 검증했다.
[고정 fixture](tuple-rest-pattern-proposal-fixtures/README.md)는 두 파일·정상 3·부정 21 code별 byte Span·Runtime 2 및 P35 historical override를 검증했다.
Native 두 profile에서 scrutinee/initializer 단일 평가·생략 요소의 함수 효과/Abort·const/var/default·nested Option·guard 순서/생략·try/loop를 관측했다.
LLVM은 결정적 prefix projection/equality IR과 COFF/ELF O0/O2 object를 확인했다.
[독립 예제](../../examples/tuple_rest_patterns.nova)는 first=3, mutable=5, match=7 세 줄/LF·빈 stderr·exit 0이다.

## Specification / Remaining

관련 NOVA-026/029/035/038/044/046/047/048/073/074/076/078/081/083/087/091/094/095/136 보완 문서에 제한된 P36 bridge를 추가했다.
P01~P35 ledger·D01~D30 본문·Canonical·원본 148개·기존 fixtures/grammar bytes를 보존하고 문서 validator/generation idempotency를 별도 검증했다.
Rust 1.99.0·LLVM 21.1.8·Windows x64 MSVC에서 검증했다. MSRV 1.80·Linux Native host 실행은 이번에 검증하지 않았다.
CARGO_INCREMENTAL=0은 검증 프로세스에만 설정했다. Windows hard-link/junction 기본 검사는 sandbox 밖에서 실행했다.
middle rest·rest binder·Enum/sum payload rest·Tuple 이름 label·shorthand/brace·Array·String/Move element·parameter/for/assignment pattern·const 함수·일반 Move/borrow/Drop은 후속 계약이다.
[TESTING.md](../../TESTING.md)에 직접 실행 명령을 제공한다.
