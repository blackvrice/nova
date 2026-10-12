# P27 가변 Copy var 구조 분해 구현·검증 기록

2026-10-12 사용자 “다음 개발 진행해줘”와 “또한 다음개발 작업 승인할께”에 따라 다음 단계 P27을 진행했다.
계약을 먼저 문서화하고 제한된 local var Copy 범위를 구현했다. 상태: **Accepted / 구현·검증 완료**.
[최소 계약](VAR_DESTRUCTURING_STAGE_B_PROPOSAL.md)·[62-production EBNF](GRAMMAR_STAGE_B_VAR_DESTRUCTURING.ebnf)는 P26 binding 하나만 확장하며 나머지 61개 production을 보존한다.

## 구현과 API 경계

- AST/HIR DestructuringBinding과 Checked plan에 mutable flag를 추가했다. 정확한 let/var keyword와 일치해야 Source gate를 통과한다.
  Parser는 기존 compound pattern·annotation·END·>= split·복구를 유지한다. const compound는 여전히 N1102다.
- Resolver는 initializer 후 선언하는 P26 순서를 유지하며 compound var의 모든 실제 binder DefId를 mutable로 설정한다.
  duplicate/같은 scope 충돌·shadow·escape·self-reference는 기존 의미를 유지한다. wildcard는 DefId/local을 만들지 않는다.
- Type Checker의 원 HIR 기반 mutable binder owner 검증을 추가했다. 공개 resolver의 flag 변조는 InvalidResolution이다.
  Checked plan의 flag·initializer·Source·path는 기존 전체 재계산 gate로 검증한다.
  P26 타입/annotation/irrefutability/cascade·10,000-node/100,000-task/1,000,000-cell·Copy/layout·진단 registry를 변경하지 않았다.
- binder 직접 대입·mixed tuple/struct path는 기존 P04/P12/P13 경로를 사용한다. immutable let field N3004·private field N2004·let/match binder N3004를 유지한다.
  mutable binder는 initializer의 Copy snapshot이며 원 aggregate의 변경이나 binder 변경이 상대 값을 바꾸지 않는다.
- MIR/LLVM/Runtime production·ABI/layout·Lexer/keyword/END·Cargo를 변경하지 않았다.
  P26 단일 평가/snapshot·tag-dominated projection·source-order binding·try/Abort·private full-body proof를 재사용한다.

## 실제 검사 결과

| 검사 | 결과 |
|---|---|
| cargo fmt --check | PASS |
| cargo check --workspace --all-features --offline | PASS |
| cargo clippy --workspace --all-targets --all-features --offline -- -D warnings | PASS |
| cargo test --workspace --offline | 433 PASS / 0 FAIL / 85 ignored |
| Runtime stage_a.rs rustfmt | PASS |
| 실제 LLVM 전체 opt-in | 23 PASS / 0 FAIL, COFF/ELF O0/O2 |
| Windows Native 전체 opt-in | 62 PASS / 0 FAIL, debug/release |
| 문서 validator | PASS |

총 518 PASS / 0 FAIL. 새 테스트 16개는 기본 13·LLVM 1·Native 2다.
[수용 fixture](var-destructuring-proposal-fixtures/README.md)의 main 13줄·정상 2·부정 18의 코드/UTF-8 primary Span/cascade·Runtime 2를 검증했다.
[독립 예제](../../examples/var_destructuring.nova)는 두 profile에서 3줄/LF·빈 stderr·exit 0이다.

Parser의 mutable flag/keyword·UTF-8 prefix·제외 문법, public AST flag/spelling/child order·Resolved flag·Checked flag 변조를 거부한다.
MIR same-type assignment target·projection path·snapshot 삭제·body 삭제를 거부한다. discard/Unit initializer calls와 zero scalar locals를 확인한다.
10,000/10,001-node와 matrix live-cell 예산·원 var keyword N8901을 검사하며 P24의 task/cache/cell 회귀도 유지한다.
Native는 nested mutable paths·private/type import·int/Bool/char/float·Unit/nullary Enum·snapshot·scope/loop·try·initializer/mutation Abort를 확인한다.
전체 회귀 후 추가 whole-Enum/Unit binder 재대입도 P27 집중 Native debug/release에서 재검증했다.

P26 var_pattern.nova와 원 expected.json의 N1102는 승인 당시 역사로 보존했다.
현재 check/native 성공·빈 출력을 새 historical_overrides/Driver/Native에 명시했으며 P26 Parser의 former unsupported var case도 P27 성공 검증으로 보완했다.
나머지 P26 진단·출력과 let/match immutable binder는 유지한다.

검증 프로세스에만 CARGO_INCREMENTAL=0을 설정했다. Windows hard-link/junction 검사는 sandbox 밖에서 실행했다.
Rust 1.99.0·LLVM 21.1.8·Windows x64 MSVC에서 검증했다. MSRV 1.80·Linux Native host 실행은 이번에 검증하지 않았다.
문서 구조/hash/grammar 검사는 Compiler/Native 실행과 별도다.

## 후속 범위

[TESTING.md](../../TESTING.md)에 사용자가 직접 실행할 명령과 예상 출력을 제공한다.
const/global/parameter/for/assignment 구조 분해·binder별 modifier·refutable else/guard·struct/or/range/general literal pattern·Array·일반 Move/borrow/Drop·전체 D08/D10/D12/D16/D25/D30은 별도 계약이다.
P01~P26 승인 ledger·D01~D30 본문·Canonical·원본 148개·기존 fixture bytes를 보존한다.
934개 보호 대상 파일의 원 byte hash를 확인했고, 19개 topic 문서는 P27 연결 문단만 추가했다. 문서 생성은 byte-idempotent다.
