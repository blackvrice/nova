# P28 함수 내부 Copy const 구조 분해 구현·검증 기록

2026-10-12 사용자 “다음 개발 승인할테니 개발 진행해줘”에 따라 다음 단계의 제한된 계약을 먼저 기록하고 구현했다.
상태: **Accepted / 구현·검증 완료**. [최소 계약](CONST_DESTRUCTURING_STAGE_B_PROPOSAL.md)·[62-production EBNF](GRAMMAR_STAGE_B_CONST_DESTRUCTURING.ebnf)는 P27 binding 하나만 확장하며 나머지 61개 production을 보존한다.

## 구현과 검증 경계

- AST/HIR/Checked DestructuringBinding에 constant flag를 추가했다. Source gate는 let/var/const spelling과 flags를 대조하고 constant+mutable를 거부한다.
  local compound const는 기존 pattern/annotation/END를 사용하며 global/visible global compound는 원 pattern N1102로 복구한다.
- Resolver는 initializer 전 scope·전체 binder의 source DFS 선언을 유지한다. compound const의 실제 binder는 immutable/constant이며 `_`는 DefId/local을 만들지 않는다.
  원 HIR owner로 flags를 재검증한다. shadow/duplicate/self/escape·const assignment와 let/var/match의 기존 의미를 유지한다.
- P26 typed pattern/irrefutability를 확인한 뒤 기존 bounded const engine을 initializer당 한 번 호출한다.
  Checked.const_destructuring_values는 whole Value/nodes·Invalid·Failed 상태를 저장하고 기존 const_values에 각 Copy component의 값과 공유 initializer nodes를 저장한다.
  새 sibling const_patterns 모듈은 iterative pattern traversal과 borrowed value projections를 사용한다. 뒤 const는 cached binder를 한 name node로 참조한다.
- pattern/type/name/resource 오류는 계산을 억제한다. 평가 실패는 원 operation/불허 expression/10,001번째 node와 compound declaration secondary·기존 limit note를 제공한다.
  전체 binder의 failure/ErrorType을 전파해 뒤 const 참조의 파생 실패를 억제한다. 모든 discard/Unit·return 뒤/while false const도 검사한다.
- pattern node/matrix와 initializer expression node 예산은 독립이다. 첫 source pattern 10,001번째 N8901·matrix const keyword N8901·expression 10,001번째 N3202를 유지한다.
  기존 short-circuit/checked numeric/cast·named constructor label budget·Copy registry/private·const function/method/call 제한을 유지한다.
- MIR은 cached Constant를 binder local에 직접 저장한다. initializer 산술/aggregate 생성·projection·tag branch를 runtime에서 실행하지 않는다.
  discard/Unit-only는 storage/statement가 없다. source-order binding과 public analysis 재계산/private full-body proof로 값/flag/node/Source/순서/body 변조를 거부한다.
- LLVM/Runtime production·ABI/layout·Lexer/END·Cargo·진단 registry는 변경하지 않았다. 새 Nxxxx code는 없다.

## 실제 결과

| 검사 | 결과 |
|---|---|
| cargo fmt --check | PASS |
| cargo check --workspace --all-features --offline | PASS |
| cargo clippy --workspace --all-targets --all-features --offline -- -D warnings | PASS |
| cargo test --workspace --offline | 450 PASS / 0 FAIL / 88 ignored |
| Runtime stage_a.rs rustfmt | PASS |
| 실제 LLVM 전체 opt-in | 24 PASS / 0 FAIL, COFF/ELF O0/O2 |
| Windows Native 전체 opt-in | 64 PASS / 0 FAIL, debug/release |
| 문서 validator | PASS |

총 538 PASS / 0 FAIL. 새 테스트 20개는 기본 17·LLVM 1·Native 2다.
[수용 fixture](const-destructuring-proposal-fixtures/README.md)의 main 10줄·정상 2·부정 20 코드/UTF-8 Span/cascade·Runtime 1을 검증했다.
[독립 예제](../../examples/const_destructuring.nova)는 debug/release에서 동일한 2줄/LF·빈 stderr·exit 0이다.

10,000/10,001 initializer nodes는 multiple binders/only-discard 모두 동일한 whole 예산으로 검사했다.
10,000 source pattern nodes는 cached global initializer name 한 node와 함께 성공하며, source/matrix 초과는 const 계산 이전에 N8901로 실패한다.
P24 task/cache/cell 경계·P05 permission/short-circuit/cast·P26/P27 let/var 회귀를 유지한다.
Source flags/keyword/child order/prefix·Resolved constant owner·Checked whole/binder Value/nodes/constant flag 변조를 거부한다.
MIR의 runtime initializer 잔존 여부·상수 operand·binder assignment 순서·body 삭제를 검사했다.
Native는 imported aggregate const·named struct/opaque projection·Option/Result/exists·int/Bool/char/float/Unit·shadow/loop·cached dependency와 Runtime checked overflow를 확인한다.

P26/P27 const_pattern.nova와 원 N1102 metadata는 승인 당시 역사로 보존했다.
현재 local compound const 성공을 새 historical_overrides/Driver/Native에 명시했으며 이전 Parser const 거부 사례는 P28 성공 검증으로 보완했다.
전역 const 선언/의존성/순환·let/var runtime initializer·전체 D09/D10/D12/D16/D25/D30은 확장하지 않았다.

Rust 1.99.0·LLVM 21.1.8·Windows x64 MSVC에서 검증했다. MSRV 1.80·Linux Native host 실행은 이번에 검증하지 않았다.
CARGO_INCREMENTAL=0은 검증 프로세스에만 설정했다. Windows hard-link/junction 검사는 sandbox 밖에서 실행했다.
문서 구조/hash/grammar 검사는 Compiler/Native 실행과 별도다.

## 후속 범위

[TESTING.md](../../TESTING.md)에 check·debug/release·집중/전체 회귀 명령과 출력을 제공한다.
전역/parameter/for/assignment 구조 분해·refutable else/guard·const 함수·String/Move aggregate element·Array·일반 Move/borrow/Drop은 후속 계약이다.
P01~P27 ledger·D01~D30 본문·Canonical·원본 148개·모든 이전 fixture bytes를 보존한다.
962개 보호 대상 파일의 원 byte hash와 20개 topic의 연결 문단 외 기존 내용을 확인했다. 문서 생성은 byte-idempotent다.
