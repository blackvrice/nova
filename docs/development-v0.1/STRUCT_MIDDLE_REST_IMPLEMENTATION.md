# P40 Copy struct 중간 rest 패턴 구현·검증

2026-10-20. Accepted / 구현·검증 완료.
사용자 “다음 작업 승인할태니 개발 해줘”의 다음 단계 승인으로 [제한 계약](STRUCT_MIDDLE_REST_STAGE_B_PROPOSAL.md)을 구현 전에 기록했다.
전체 D06/D08/D09/D10/D12/D16/D25/D30 승인이 아니다.

## Implemented / Changed

Parser는 Point(first,..,last), Point(..,last), 여러 suffix와 trailing comma를 지원한다. P35 trailing/rest-only·empty struct·기존 Tuple/Enum rest는 유지한다.
marker는 하나만 허용하고 두 점은 인접해야 한다. 중복/spaced/rest binder/field value/root rest는 N1102다. 더 이상 호출자가 없는 trailing-only helper를 제거했다.
AST/HIR는 marker 포함 Source 순서·spelling/shape/order/parent·중복 marker를 검증한다.
prefix p·suffix s·전체 fields n에서 prefix는 0..p, suffix는 n-s..n이다. suffix는 marker 뒤 직접 위치 인수만 세고 named field는 제외한다. 빈 생략도 허용한다.
이름 field는 marker 양쪽에서 선언 이름으로 대응한다. 이름 뒤 위치는 marker가 있어도 N2201이다. unknown/중복/위치-name overlap/초과/marker 없는 누락도 N2201이다. suffix가 이름 field를 피해서 이동하지 않는다.
private field를 포함한 struct는 다른 module에서 생략해도 N2004다. nominal/import·alias annotation은 유지하며 alias head는 N1102다.
AST/HIR와 binder/진단 방문은 Source 순서, Checked children은 marker 제외 선언 필드 순서다. field_indices는 실제 index와 일대일 대응한다.
Source child 타입/secondary는 실제 선언 field와 대응하고 marker는 Unit/None으로 가운데 slot에 남는다. 타입 불일치 N2101·정수 범위 N2102·중복 binder N2002·불변 N3004를 구분한다.
marker는 source pattern node 한 개를 센다. implicit wildcard에 synthetic Source/HIR node나 DefId를 만들지 않는다.
Matrix는 전체 struct vector wildcard 중 explicit indices만 채운다. N3101/N3102·guard 없는 합집합·witness·기존 예산을 유지한다.
let/var/local/global const는 explicit irrefutable component만 허용한다. refutable은 N3103·Source 순서 첫 refutable secondary·const 평가/파생 오류 억제를 유지한다.
기존 MIR은 전체 initializer/scrutinee 단일 평가·snapshot 뒤 explicit field만 선언 순서대로 projection한다. 생략 field operand의 effect/Abort/try를 건너뛰지 않으며 guard는 패턴 성공 후 한 번 평가한다.
기존 const는 전체 initializer 계산 후 실제 field index의 binder cache를 저장한다. whole value·forward/cycle/default·10,000-node 예산을 보존한다.
Source/Checked indices·children·타입·cache 및 MIR 같은 타입 FieldId/Source/guard 변조는 HIR 재계산과 private full-body proof로 거부한다.
Grammar는 struct_pattern_arguments 한 production만 확장한다. 총 70개이며 다른 69개를 보존했다. 새 production은 없다.
Production 변경은 Parser·HIR shape gate·struct 위치/이름 대응·Source slot 타입/secondary 정렬이다. const/MIR/LLVM/Runtime production·private ABI/layout·Lexer/END·Cargo dependencies·diagnostic registry는 변경하지 않았다.

## Tests Added / Validation

새 테스트 19개: 기본 16·LLVM opt-in 1·Native opt-in 2.
arity 0~5의 모든 prefix/suffix 조합 56개를 독립적인 range로 검사했다. 이름 field를 marker 앞/뒤/양쪽·역순으로 배치해 Source 순서와 checked declaration 순서를 비교했다.
named 뒤 위치·suffix-name overlap·explicit 초과·unknown을 N2201로 검사했다. 서로 다른 잘못된 타입의 역순 이름 패턴에서 N2101/N2102 순서와 실제 char/int8 선언 field의 secondary를 검증했다.
두 Bool endpoint와 opaque Int middle의 네 pattern region에 대한 16개 부분집합 oracle로 completeness/reachability를 비교했다.
local/global const의 int8 prefix·Unicode char suffix cache·forward/default·cycle·omitted const 실패·N3103 Source 첫 refutable/파생 억제를 검사했다.
rest를 포함한 10,000/10,001 source nodes·implicit Source 부재·128/129 nesting·모든 UTF-8 prefix의 결정적 복구를 검사했다.
AST/HIR marker spelling/shape/order/중복, Checked children/indices/suffix cache 및 MIR 같은 타입 suffix FieldId/Source/guard branch 변조를 거부했다.
MIR make 호출은 initializer/match마다 한 번이며 역순 이름 패턴도 declaration projection [0,2,2]로 선택한 field만 읽는다.
P35 middle_rest.nova와 P38/P39 struct_middle.nova의 source·당시 N1102 metadata는 byte 보존하고 P40 overrides/current acceptance를 추가했다.

| 검사 | 실제 결과 |
|---|---|
| fmt / Runtime rustfmt / all-features check / all-targets all-features Clippy -D warnings | PASS |
| 전체 기본 tests | 618 PASS / 0 FAIL / 123 ignored |
| LLVM 전체 opt-in COFF/ELF O0/O2 | 36 PASS / 0 FAIL |
| Windows Native 전체 opt-in debug/release | 87 PASS / 0 FAIL |

총 741 PASS / 0 FAIL. Native는 P40 두 개를 먼저 실행하고 기존 85개를 --skip p40_로 실행해 중복 없이 전체 87개를 검증했다.
[고정 fixture](struct-middle-rest-proposal-fixtures/README.md)는 두 파일·정상 3·부정 25 code별 byte Span·Runtime 2 및 P35/P38/P39 overrides를 검증했다.
Native 두 profile에서 단일 scrutinee/initializer·omitted middle 효과/Abort·const/var/default·nested Option/Result·guard 순서/생략·try/loop를 관측했다.
LLVM은 결정적 endpoint projection/equality IR과 COFF/ELF O0/O2 object를 확인했다.
[독립 예제](../../examples/struct_middle_rest.nova)는 first=3/true, mutable=5/true, match=7 세 줄/LF·빈 stderr·exit 0이다.

## Specification / Remaining

관련 NOVA-026/029/035/038/044/046/047/048/073/074/076/078/081/083/087/091/094/095/136 보완 문서에 제한된 P40 bridge를 추가했다.
P01~P39 ledger·D01~D30 본문·Canonical·원본 148개·기존 fixtures/grammar bytes·사용자 변경을 보존하고 문서 validator/generation idempotency를 별도 검증했다.
Rust 1.99.0·LLVM 21.1.8·Windows x64 MSVC에서 검증했다. MSRV 1.80·Linux Native host 실행은 이번에 검증하지 않았다.
CARGO_INCREMENTAL=0은 검증 프로세스에만 설정했다. Windows hard-link/junction 기본 검사는 sandbox 밖에서 실행했다.
shorthand/brace·rest binder·named Enum payload·Array·String/Move field·parameter/for/assignment pattern·const 함수·일반 Move/borrow/Drop은 후속 계약이다.
[TESTING.md](../../TESTING.md)에 직접 실행 명령을 제공한다.
