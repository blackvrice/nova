# P34 Copy struct 이름 필드 패턴 구현·검증

2026-10-13. Accepted / 구현·검증 완료.
사용자 “다음 작업 승인할테니 개발 작업 진행해줘”에 따라 [제한 계약](STRUCT_NAMED_PATTERN_STAGE_B_PROPOSAL.md)을 구현 전에 기록했다.
전체 D06/D08/D09/D10/D12/D16/D25/D30 승인이 아니다.

## Implemented / Changed

Parser는 struct argument의 IDENT: lookahead로 PatternField wrapper를 만들고 이름 순서 변경·위치 prefix 혼합을 지원한다.
AST/HIR은 원 label/colon Span과 정확히 한 value pattern child를 보존한다. HIR은 Source label/colon/trivia/shape·parent placement·child 순서를 검증한다.
다중 파일 merge는 field label SymbolId를 remap한다. Resolver는 기존 Source 순서 DFS를 재사용하며 label은 값을 찾거나 선언하지 않는다.
별도 struct_patterns helper는 실제 field 이름에 대응한다. unknown·duplicate·위치/이름 중복·이름 뒤 위치·missing·method label은 P23과 같은 N2201이다.
실패한 mapping은 component 타입/coverage/const 파생 오류를 억제한다. binder 이름 중복 N2002는 label 대응과 구분한다.
P33 nominal/import head·alias annotation·전체 field 명시/가시성·Copy layout·field 기대 타입을 유지한다. private field는 wildcard/label 여부와 무관하게 N2004다.
AST/HIR children과 진단 방문은 Source 순서, Checked struct children은 선언 field 순서다. wrapper와 child는 mapped field의 타입/원 Source를 갖는다.
Matrix는 wrapper를 transparent하게 지우고 P33 nominal constructor를 사용한다. 이름 순서와 위치/이름 표기가 다른 동일 패턴은 같은 coverage다.
N3101/N3102·unguarded 합집합·guard 정책·기존 위치 witness/opaque-only shortcut을 보존한다.
let/var/const의 N3103 secondary는 Source 순서의 첫 refutable component다. const initializer는 한 번 평가하고 선언 순서 field와 wrapper를 통과하여 binder cache를 저장한다.
MIR은 single snapshot 후 선언 field 순서 Project와 wrapper의 같은 Place를 사용한다. tag proof·guard/try·return/jump·checked Abort를 보존한다.
Checked mapping/kind/type/cache와 MIR FieldId/Source/branch 변조를 원 HIR 재계산 및 기존 private full-body proof로 거부한다.
Grammar는 struct_pattern을 확장하고 두 argument production을 추가해 66 productions다. 나머지 63개를 보존한다.
label wrapper도 source pattern node 한 개로 센다. matrix task/domain과 const initializer 예산에는 새 의미를 추가하지 않는다.
LLVM/Runtime production·ABI/layout·Lexer/END·Cargo dependencies·diagnostic registry는 변경하지 않았다.

## Tests Added / Validation

새 테스트 18개: 기본 15·LLVM opt-in 1·Native opt-in 2.
역순 label과 위치 prefix·mapped component 타입/range·value/type namespace·scope·label 오류와 binder 중복·private·method·alias/nominal을 검사한다.
Bool 두 field의 네 값에 대한 16개 부분집합을 독립 oracle와 비교하고 위치/이름 동등성·duplicate coverage·guard union을 검사한다.
중첩 struct/Tuple/Option·local/global const projection·forward/default·Float Copy binder·refutable const 억제·순환을 검사한다.
wrapper를 포함한 정확한 10,000/10,001-node 경계와 모든 UTF-8 prefix의 결정적 Parser 복구를 검사한다.
AST name/colon/shape/order/parent/expression wrapper 변조, Checked 정규화 mapping/wrapper type/유효하지만 다른 상수 cache와 MIR 같은 타입 field index/Source/branch 변조를 거부한다.
const는 Runtime aggregate 생성 없이 materialize하고 initializer/match마다 make 호출이 한 번임을 MIR에서 확인했다.
P33 당시 이름 필드 거부 Parser source를 현재 성공 assertion으로 옮기고 기존 고정 fixtures/grammar bytes를 보존했다.
범위 밖 Enum 이름 payload는 기존 P01의 N1101 delimiter insertion point Span 57..57을 그대로 기록했다. 문서 validator는 N1101/primary_kind=insertion에 한해 0길이 Span을 허용하며 UTF-8 byte boundary를 검증한다. Compiler/진단 정책은 변경하지 않았다.

| 검사 | 실제 결과 |
|---|---|
| fmt / Runtime rustfmt / all-features check / all-targets all-features Clippy -D warnings | PASS |
| 전체 기본 tests | 524 PASS / 0 FAIL / 105 ignored |
| LLVM 전체 opt-in COFF/ELF O0/O2 | 30 PASS / 0 FAIL |
| Windows Native 전체 opt-in debug/release | 75 PASS / 0 FAIL |

총 629 PASS / 0 FAIL. Native는 P34 두 개를 먼저 실행하고 기존 73개를 --skip p34_로 실행하여 중복 없는 전체 75개를 검증했다.
[고정 fixture](struct-named-pattern-proposal-fixtures/README.md)는 두 파일·정상 3·부정 25 code별 byte Span·Runtime 1을 검증했다.
Native 두 profile에서 역순/위치 혼합 field·Unicode·single scrutinee/initializer·var/const/default·Empty·guard 순서/생략·try/loop·checked Abort를 관측했다.
LLVM은 결정적 struct projection/equality IR과 COFF/ELF O0/O2 object를 확인했다.
[독립 예제](../../examples/struct_named_patterns.nova)는 point=3/true, mutable=5/true, match=7 세 줄/LF·빈 stderr·exit 0이다.

## Specification / Remaining

관련 NOVA-026/029/035/038/044/046/047/048/073/074/076/078/081/083/087/091/094/095/136 보완 문서에 제한된 P34 bridge를 추가했다.
P01~P33 ledger·D01~D30 본문·Canonical·원본 148개·기존 fixtures/grammar bytes를 보존하고 문서 validator/generation idempotency를 별도 검증했다.
Rust 1.99.0·LLVM 21.1.8·Windows x64 MSVC에서 검증했다. MSRV 1.80·Linux Native host 실행은 이번에 검증하지 않았다.
CARGO_INCREMENTAL=0은 검증 프로세스에만 설정했다. Windows hard-link/junction 기본 검사는 sandbox 밖에서 실행했다.
rest/shorthand/부분 필드·Enum/sum/Tuple 이름 payload·field default·or/range 패턴·Array·String/Move field·parameter/for/assignment pattern·const 함수·일반 Move/borrow/Drop은 후속 계약이다.
[TESTING.md](../../TESTING.md)에 직접 실행 명령을 제공한다.
