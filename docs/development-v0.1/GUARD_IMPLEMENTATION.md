# P25 Copy match guard 구현·검증 기록

2026-10-12 사용자 “P25 승인하고 Copy match guard 구현 진행” 답변으로 [최소 계약](GUARD_STAGE_B_PROPOSAL.md)을 승인했다.
상태: **Accepted / 구현·검증 완료**. [61-production EBNF](GRAMMAR_STAGE_B_GUARD.ebnf)는 P24 match_arm만 확장·match_guard 추가·기존 59개 보존이다.
NOVA-014/043/046/047/048/074/076/078/081/083/087/091/094/095/136 보완 문서에 이 승인 범위를 연결했다.

## 구현과 API 경계

- AST/HIR MatchGuard는 if keyword Span·한 condition child를 가진다. arm은 pattern/선택적 guard/body 순서다.
  Parser는 기존 if/END·expression을 사용한다. pattern-if 사이 END·중복 guard·누락 expression/arrow를 복구한다.
  Source gate는 UTF-8/XID keyword·condition containment·arm child order·=>·pattern-if END 금지를 검증한다.
- Resolver는 arm binder를 먼저 선언하고 guard/body가 같은 scope를 사용한다. immutable Copy·duplicate/shadow/private 접근은 기존 계약을 유지한다.
- Checked.guards는 arm-indexed MatchGuard plan에 원 pattern/guard/condition/body HirId와 if Span을 저장한다.
  guard는 Bool context이며 ErrorType을 전파한다. pattern 검증 후 모든 guard를 검사하고 coverage를 완료한 뒤 body를 검사해 guard 오류의 파생 N3101/N3102를 억제한다.
  독립 body 오류·원 try 진단과 all-return/jump 분석을 보존한다. Debug dump는 guard source mapping을 표시한다.
- guard가 있으면 P24 matrix를 사용하되 guard 없는 reachable rows만 coverage에 넣는다.
  guarded candidate도 앞 unguarded 합집합에 덮이면 N3102다. literal true/false를 coverage에서 접지 않는다.
  기존 guard 없는 flat/P24 경로·constructor/witness/secondary 순서·cache/arm/node/task/cell 예산을 유지한다.
- MIR은 단일 scrutinee snapshot·pattern-success Copy binders·guard expression·Bool branch·body/next-arm을 명시한다.
  try Error는 enclosing 함수의 기존 Error edge, checked Abort는 기존 Runtime failure edge로 연결한다.
  public Checked plan은 재계산 gate, private nested full-body/source/projection/tag proofs는 guard bypass·false/body edge 반전·effect 변경·snapshot overwrite·inactive payload·body 삭제를 거부한다.
- Lexer/keyword/END·Runtime/LLVM production·Cargo·aggregate ABI/layout은 변경하지 않았다. 기존 Branch/Call/Try/Copy lowering을 사용한다.

## 테스트와 실제 결과

| 검사 | 결과 |
|---|---|
| cargo fmt --check | PASS |
| cargo check --workspace --all-features --offline | PASS |
| cargo clippy --workspace --all-targets --all-features --offline -- -D warnings | PASS |
| cargo test --workspace --offline | 404 PASS / 0 FAIL / 79 ignored |
| Runtime stage_a.rs rustfmt | PASS |
| LLVM 전체 opt-in | 21 PASS / 0 FAIL, COFF/ELF O0/O2 |
| Native 전체 opt-in | 58 PASS / 0 FAIL, debug/release |
| 문서 validator / finite coverage·trace oracle 6개 | PASS |

총 483 PASS / 0 FAIL. 새 테스트 15개는 기본 12·LLVM 1·Native 2다.
22개 원 source bytes를 보존한 [수용 fixture](guard-proposal-fixtures/README.md)의 main/정상 3·부정 16·Runtime 1을 검증했다.
main 16줄·추가 정상 출력·guard overflow의 정확한 UTF-8 Span/before-only output을 debug/release에서 확인했다.
[독립 예제](../../examples/match_guards.nova)도 두 profile에서 7줄/LF·빈 stderr·exit 0이다.

Typecheck는 네 Bool-pair 값을 독립 열거해 2,916개 패턴/guard 조합의 usefulness/누락을 대조한다.
1,025/1,026 arms·10,000/10,001 pattern nodes·guard expression 제외·guarded cache-hit 반복을 검사한다.
P24의 100,000-task/1,000,000-cell 경계와 flat path 회귀도 유지한다.
Parser/UTF-8 prefix·Source keyword/span/child-order/condition forgery·public guard plan 삭제/owner/condition/body/type 변경·MIR effect/false branch/snapshot/inactive payload/body forgery를 거부한다.
Native는 guarded product overlap·nested inactive Empty/Int/Flag·Read method named/default·logical short-circuit·snapshot·try·loop·Abort를 관측한다.

P14 guard_unsupported.nova와 원 expected metadata는 승인 당시 기록으로 보존했다.
현재는 guard 문법이 허용되고 guard-only enum coverage가 N3101이므로 해당 기존 회귀의 override를 명시했다.
P14 Parser의 former unsupported guard 사례도 P25 성공 문법으로 명시했다. 다른 부정 경계는 유지한다.
OneDrive 파일 mapping 때문에 최초 fmt 쓰기에서 오류가 발생했으며 compiler 작업 종료 후 다시 fmt를 실행해 통과했다.
검증 프로세스에만 CARGO_INCREMENTAL=0을 설정했으며 소스/Cargo 설정은 변경하지 않았다.
문서 검사는 Compiler/Native 실행과 별도다. Rust 1.99.0·LLVM 21.1.8·Windows x64 MSVC에서 검증했다. MSRV 1.80·Linux Native host 실행은 이번에 검증하지 않았다.

## 직접 사용과 후속 범위

[TESTING.md](../../TESTING.md)에 check·debug/release·집중/전체 회귀 명령을 제공한다.
or/range/general literal·struct/let destructuring·match expression·guard truth/refinement 증명·Array·Read loan/take·일반 Move/borrow/Drop·전체 D08/D10/D12/D16/D25/D30은 후속 계약이다.
P01~P24·D01~D30 결정 본문·Canonical·원본 148개는 보존한다.
