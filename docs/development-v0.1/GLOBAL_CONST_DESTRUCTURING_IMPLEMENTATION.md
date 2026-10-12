# P29 전역 Copy const 구조 분해 구현·검증 기록

2026-10-12 사용자 “다음 개발 승읺 ㅏㄹ테니 개발 진행해줘”에 따라 다음 단계의 [제한 계약](GLOBAL_CONST_DESTRUCTURING_STAGE_B_PROPOSAL.md)을 구현 전에 기록했다.
Accepted / 구현·검증 완료. 전체 D09/D10/D12/D16/D25/D30 승인이 아니다.

## Implemented / Changed

Parser root/visibility에서 compound const를 허용하고 HIR Source shape gate는 constant=true/mutable=false로 제한한다.
global_const_decl 하나를 확장하고 기존 61개 production·Lexer/END·local let/var/const 의미를 보존한다.
Resolver는 모든 global binder를 먼저 수집하고 GlobalConst 정의별 declaration owner를 보존한다.
import/alias는 원 component DefId를 사용하고 visibility는 whole owner에서 확인한다. reserved print·duplicate·private import 기존 진단을 유지한다.
의존성 graph는 whole initializer별 vertex 하나를 가진다. 정적 skipped RHS·같은 group 자기 참조·cross-file SCC를 검사한다.
그룹별 단일 const 평가와 whole/component cache는 P28 engine을 재사용한다. 실패/Invalid는 모든 component와 의존 선언에 전파한다.
binder가 없는 Unit/discard도 평가한다. 전체 node 예산을 binder 수로 곱하거나 LLVM folding으로 대체하지 않는다.
MIR은 global 선언을 runtime body에서 제외하고 component 이름을 cached Constant로 바꾼다.
Resolved owner map·Checked whole/component cache는 원 HIR에서 재계산하고 MIR의 기존 private proof를 유지한다.
LLVM/Runtime production·ABI/layout·Cargo dependency·diagnostic registry는 변경하지 않았다.

## Tests Added / Validation

새 테스트 12개: 기본 10·LLVM opt-in 1·Native opt-in 1.
forward/default·예상 정수 타입·owner map 변조·whole/binder cache 변조·중첩 single-variant 패턴·import/internal/private/alias를 검사한다.
self/peer/skipped RHS·cross-file SCC·cascade 억제·unused discard 실패·duplicate/reserved print·known-value irrefutability를 검사한다. public global AST flags/keyword/children 변조와 모든 UTF-8 prefix를 거부/검사한다.
initializer 10,000/10,001 경계는 여러 binder와 discard에 공유되며 실패 시 뒤 상수에 파생 진단이 없다.
MIR은 initializer 산술을 runtime에서 실행하지 않고 전역 runtime body/storage를 추가하지 않는다.

| 검사 | 실제 결과 |
|---|---|
| fmt / all-features check / all-targets all-features Clippy -D warnings | PASS |
| cargo test --workspace --offline | 460 PASS / 0 FAIL / 90 ignored |
| LLVM 전체 opt-in, COFF/ELF O0/O2 | 25 PASS / 0 FAIL |
| Windows x64 MSVC Native 전체 opt-in, debug/release | 65 PASS / 0 FAIL |

총 550 PASS / 0 FAIL. [고정 fixture](global-const-destructuring-proposal-fixtures/README.md)는 두 파일·부정 4개의 실제 check/Native 결과·byte Span을 기록한다.
[독립 예제](../../examples/global_const_destructuring.nova)의 `global=8/true/🙂` 출력/LF·빈 stderr·exit 0을 두 profile에서 확인했다.
P28 global_pattern/visible_global_pattern source와 N1102 metadata는 그대로이며 현재 성공을 P29 historical override·Parser/Driver/Native에 명시했다.
기존 P26의 global const 거부 테스트는 P29 성공 검증 및 계속 제외되는 global var 거부 검증으로 보완했다.

## Specification / Remaining

Rust 1.99.0·LLVM 21.1.8·Windows x64 MSVC에서 검증했다. MSRV 1.80·Linux Native host 실행은 이번에 검증하지 않았다.
CARGO_INCREMENTAL=0은 검증 프로세스 환경에만 설정했다. Windows hard-link/junction 검사는 sandbox 밖에서 실행했다.
문서 validator·generation idempotency·원본 148개와 이전 fixture hash·P01~P28 ledger·D01~D30 본문을 별도 검증한다.
String/Move component·Array·const 함수·parameter/for/assignment 구조 분해·일반 Move/borrow/Drop은 후속 계약이다.
직접 실행 명령은 [TESTING.md](../../TESTING.md)에 제공한다.
