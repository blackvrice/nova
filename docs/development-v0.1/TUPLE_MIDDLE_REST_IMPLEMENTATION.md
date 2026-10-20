# P38 Copy Tuple 중간 rest 패턴 구현·검증

2026-10-20. Accepted / 구현·검증 완료.
사용자 “다음 작업 승인할태니 개발 해줘”에 따라 [제한 계약](TUPLE_MIDDLE_REST_STAGE_B_PROPOSAL.md)을 구현 전에 기록했다.
전체 D06/D08/D09/D10/D12/D16/D25/D30 승인이 아니다.

## Implemented / Changed

Parser는 (first,..,last), (..,last), 여러 suffix와 선택적 trailing comma를 지원한다. P36 trailing/rest-only도 유지한다.
marker 소비와 trailing-only helper를 분리해 struct/Enum payload의 정책과 진단을 유지했다. Tuple rest는 하나만 허용하며 두 점은 인접해야 한다.
AST/HIR는 marker를 포함한 Source 순서를 보존하고 duplicate marker·spelling/shape/order/parent·punctuation/trivia·단일 Tuple comma를 검증한다.
Type::Tuple만 허용한다. Unit/scalar/struct 적용은 N2101, explicit prefix+suffix가 arity보다 길면 N2201이다. Unit ()와 rest 없는 arity는 기존 규칙이다.
prefix 수 p·suffix 수 s·arity n에서 prefix index는 0..p, suffix는 n-s..n이다. 빈 생략도 허용한다.
Checked children은 marker를 제외한 Source 순서이며 field_indices는 실제 index와 일대일 대응한다. suffix 타입·secondary annotation도 해당 선언 요소를 가리킨다. annotation이 없으면 None을 유지한다.
rest marker는 Unit/Wildcard bookkeeping으로 source node 한 개를 센다. implicit wildcard에 synthetic Source/HIR node나 DefId를 만들지 않는다.
기존 Matrix는 전체 Tuple vector의 wildcard 중 field_indices만 채운다. N3101/N3102·guard 없는 합집합·witness·순서/예산을 유지한다.
let/var/local/global const는 explicit component의 irrefutability·N3103·Source 순서의 첫 refutable secondary·파생 오류 억제를 유지한다.
기존 MIR은 전체 initializer/scrutinee 단일 평가·snapshot 뒤 실제 prefix/suffix index만 projection한다. 생략한 중간 요소의 effect/Abort/try도 실행한다.
기존 const도 전체 initializer를 계산하고 실제 index의 prefix/suffix binder cache·whole value·forward/cycle/default·10,000-node 예산을 보존한다.
Checked field_indices/children/cache와 MIR 같은 타입 suffix index/Source/guard branch 변조는 원 HIR 재계산 및 private full-body proof로 거부한다.
Grammar는 tuple_pattern을 확장하고 tuple_pattern_tail을 추가해 70 productions다. 다른 68개를 보존했다.
Production 변경은 Parser·HIR shape gate·Tuple typing/annotation 대응이다. const/MIR/LLVM/Runtime production·ABI/layout·Lexer/END·Cargo dependencies·diagnostic registry는 변경하지 않았다.

## Tests Added / Validation

새 테스트 19개: 기본 16·LLVM opt-in 1·Native opt-in 2.
Tuple arity 1~5에서 가능한 prefix/suffix 조합 55개를 독립적으로 열거해 실제 field_indices를 검증했다.
leading/middle/trailing/rest-only·빈 생략·한 요소/Unit·suffix type/range/실제 int8 annotation·scope/mutability·nested product/sum·guard/try를 검사한다.
두 Bool endpoint와 생략하는 opaque Int middle의 네 pattern region에 대한 16개 부분집합 oracle로 completeness/reachability를 비교했다.
local/global const의 정수 prefix와 Unicode char suffix cache·forward/default·cycle·생략 중간 const 실패·N3103 파생 오류 억제를 검사한다.
rest를 포함한 10,000/10,001 source-node 경계·implicit Source node 부재·nesting 128/129와 모든 UTF-8 prefix의 결정적 Parser 복구를 검사했다.
AST/HIR marker spelling/shape/order와 중복 marker, Checked children/indices/suffix cache 및 MIR 같은 타입 suffix FieldId/Source/guard 변조를 거부했다.
MIR make 호출은 initializer/match마다 한 번이며 projection index [0,2,2]로 앞뒤/뒤쪽 선택만 수행하고 middle index 1은 읽지 않는 것을 확인했다.
P36 middle_rest.nova의 source·당시 N1102 metadata는 byte 보존하고 P38 historical override 및 현재 성공 assertion을 추가했다.

| 검사 | 실제 결과 |
|---|---|
| fmt / Runtime rustfmt / all-features check / all-targets all-features Clippy -D warnings | PASS |
| 전체 기본 tests | 586 PASS / 0 FAIL / 117 ignored |
| LLVM 전체 opt-in COFF/ELF O0/O2 | 34 PASS / 0 FAIL |
| Windows Native 전체 opt-in debug/release | 83 PASS / 0 FAIL |

총 703 PASS / 0 FAIL. Native는 P38 두 개를 먼저 실행하고 기존 81개를 --skip p38_로 실행해 중복 없는 전체 83개를 검증했다.
[고정 fixture](tuple-middle-rest-proposal-fixtures/README.md)는 두 파일·정상 3·부정 21 code별 byte Span·Runtime 2 및 P36 historical override를 검증했다.
Native 두 profile에서 single scrutinee/initializer·omitted middle 효과/Abort·const/var/default·nested Option/Result·guard 순서/생략·try/loop를 관측했다.
LLVM은 결정적 endpoint projection/equality IR과 COFF/ELF O0/O2 object를 확인했다.
[독립 예제](../../examples/tuple_middle_rest.nova)는 first=3, mutable=5, match=7 세 줄/LF·빈 stderr·exit 0이다.

## Specification / Remaining

관련 NOVA-026/029/035/038/044/046/047/048/073/074/076/078/081/083/087/091/094/095/136 보완 문서에 제한된 P38 bridge를 추가했다.
P01~P37 ledger·D01~D30 본문·Canonical·원본 148개·기존 fixtures/grammar bytes를 보존하고 문서 validator/generation idempotency를 별도 검증했다.
Rust 1.99.0·LLVM 21.1.8·Windows x64 MSVC에서 검증했다. MSRV 1.80·Linux Native host 실행은 이번에 검증하지 않았다.
CARGO_INCREMENTAL=0은 검증 프로세스에만 설정했다. Windows hard-link/junction 기본 검사는 sandbox 밖에서 실행했다.
struct/Enum payload middle rest·rest binder·named Enum payload·shorthand/brace·Array·String/Move element·parameter/for/assignment pattern·const 함수·일반 Move/borrow/Drop은 후속 계약이다.
[TESTING.md](../../TESTING.md)에 직접 실행 명령을 제공한다.
