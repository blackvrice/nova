# P37 Copy Enum·sum payload trailing rest 패턴 구현·검증

2026-10-20. Accepted / 구현·검증 완료.
사용자 “다음 작업 승인할태니 개발 해줘”에 따라 [제한 계약](PAYLOAD_REST_PATTERN_STAGE_B_PROPOSAL.md)을 구현 전에 기록했다.
전체 D06/D08/D09/D10/D12/D16/D25/D30 승인이 아니다.

## Implemented / Changed

Parser는 E::Data(first,..), E::Data(..), 전체 prefix 뒤 rest·trailing comma와 intrinsic Option/Result payload rest를 지원한다.
P35/P36 공통 marker helper와 정확한 .. Span을 재사용한다. rest는 한 번·마지막이며 두 점이 인접해야 한다. named payload는 지원하지 않는다.
AST/HIR는 Source 순서의 explicit prefix와 마지막 PatternRest를 보존하고 spelling/shape/order/parent·punctuation/trivia를 검증한다.
nullary E::End(..)/E::End()/Option::None(..)/Option::None()는 N2201이다. bare E::End·Option::None·none을 유지한다.
nonempty payload의 과도한 prefix는 N2201이다. family/nominal identity·import·alias head 금지·타입/정수 문맥·scope·visibility·mutable binder를 기존 규칙으로 검사한다.
Checked Variant children은 explicit prefix만 저장한다. payload ordinal은 기존 0..prefix 길이이며 non-product field_indices는 비어 있다.
rest marker는 Unit/Wildcard bookkeeping으로 source node 한 개를 센다. implicit wildcard에 synthetic Source/HIR node나 DefId를 만들지 않는다.
Matrix는 선택 Variant의 전체 payload vector를 wildcard로 만들고 명시 prefix 위치만 채운다. 태그·N3101/N3102·guard 없는 합집합·위치 witness·기존 예산을 보존한다.
let/var/local/global const는 단일 variant Enum과 irrefutable prefix에만 허용한다. 여러 variant/Option/Result는 기존 N3103·첫 refutable Source secondary·파생 오류 억제를 유지한다.
기존 MIR 경로는 전체 initializer/scrutinee 단일 평가·snapshot 뒤 tag 검사와 활성 payload의 explicit prefix read만 수행한다. rest-only에도 tag 검사가 남는다.
생략 payload operand의 effect/Abort/try는 그대로다. 기존 const 평가도 전체 initializer를 계산하고 prefix binder cache·whole value·forward/cycle/default·10,000-node 예산을 보존한다.
Checked children/cache/field_indices와 MIR 같은 타입 payload index/Source/tag dispatch 변조는 원 HIR 재계산 및 기존 private full-body proof로 거부한다.
Grammar는 pattern_arguments를 확장하고 payload_pattern_rest 하나를 추가해 69 productions다. 다른 67개는 보존했다.
Production 변경은 Parser·HIR shape gate·recursive pattern typing/coverage다. const/MIR/LLVM/Runtime production·ABI/layout·Lexer/END·Cargo dependencies·diagnostic registry는 변경하지 않았다.

## Tests Added / Validation

새 테스트 19개: 기본 16·LLVM opt-in 1·Native opt-in 2.
Enum/Option/Result rest-only·전체 prefix·nullary parentheses 거부/bare 허용·nested product/sum·arity/type/range·scope/mutability·alias/undefined head·guard/try를 검사한다.
다른 variant의 tag는 rest로 덮지 않음을 확인했다. 네 Bool payload region의 16개 부분집합에 대한 독립 oracle로 completeness/reachability를 비교했다.
local/global const의 prefix cache·forward/default·cycle·생략 payload const 실패·단일/여러 variant N3103·파생 오류 억제를 검사한다.
rest를 포함한 정확한 10,000/10,001 source-node 경계와 implicit wildcard Source node 부재, nesting 128/129·모든 UTF-8 prefix의 결정적 Parser 복구를 검사했다.
AST/HIR marker spelling/shape/order/argument parent, Checked children/cache/field_indices 및 MIR 같은 타입 payload index/Source/tag 변조를 거부했다.
MIR make 호출은 initializer/match마다 한 번이고 payload prefix index 0만 읽는다. rest-only에는 payload read가 없고 tag dispatch는 남는다.
P36의 과거 Enum rest 거부 source·당시 N1102 metadata는 byte 보존하고 P37 historical override 및 현재 성공 assertion을 추가했다.

| 검사 | 실제 결과 |
|---|---|
| fmt / Runtime rustfmt / all-features check / all-targets all-features Clippy -D warnings | PASS |
| 전체 기본 tests | 570 PASS / 0 FAIL / 114 ignored |
| LLVM 전체 opt-in COFF/ELF O0/O2 | 33 PASS / 0 FAIL |
| Windows Native 전체 opt-in debug/release | 81 PASS / 0 FAIL |

총 684 PASS / 0 FAIL. Native는 P37 두 개를 먼저 실행하고 기존 79개를 --skip p37_로 실행해 중복 없는 전체 81개를 검증했다.
[고정 fixture](payload-rest-pattern-proposal-fixtures/README.md)는 두 파일·정상 3·부정 21 code별 byte Span·Runtime 2 및 P36 historical override를 검증했다.
Native 두 profile에서 single scrutinee/initializer·omitted payload 효과/Abort·const/var/default·nested Option/Result·guard 순서/생략·try/loop를 관측했다.
LLVM은 결정적 tagged prefix/equality IR과 COFF/ELF O0/O2 object를 확인했다.
[독립 예제](../../examples/payload_rest_patterns.nova)는 first=3, mutable=5, match=7 세 줄/LF·빈 stderr·exit 0이다.

## Specification / Remaining

관련 NOVA-026/029/035/038/044/046/047/048/073/074/076/078/081/083/087/091/094/095/136 보완 문서에 제한된 P37 bridge를 추가했다.
P01~P36 ledger·D01~D30 본문·Canonical·원본 148개·기존 fixtures/grammar bytes를 보존하고 문서 validator/generation idempotency를 별도 검증했다.
Rust 1.99.0·LLVM 21.1.8·Windows x64 MSVC에서 검증했다. MSRV 1.80·Linux Native host 실행은 이번에 검증하지 않았다.
CARGO_INCREMENTAL=0은 검증 프로세스에만 설정했다. Windows hard-link/junction 기본 검사는 sandbox 밖에서 실행했다.
middle rest·rest binder·named Enum payload·nullary 괄호 완화·shorthand/brace·Array·String/Move payload·parameter/for/assignment pattern·const 함수·일반 Move/borrow/Drop은 후속 계약이다.
[TESTING.md](../../TESTING.md)에 직접 실행 명령을 제공한다.
