# P26 Copy let 구조 분해 구현·검증 기록

2026-10-12 사용자 “P26 승인하고 Copy let 구조 분해 구현 진행” 답변으로 [최소 계약](DESTRUCTURING_STAGE_B_PROPOSAL.md)을 승인했다.
상태: **Accepted / 구현·검증 완료**. [62-production EBNF](GRAMMAR_STAGE_B_DESTRUCTURING.ebnf)는 binding 하나 확장·destructuring_pattern 추가·기존 60개 보존이다.
N3103을 Compiler·공용 진단 registry에 등록했으며 NOVA-014/029/035/037/038/044/046/047/048/073/074/075/076/077/078/081/083/087/091/094/095/136에 이 승인 범위를 연결했다.

## 구현과 API 경계

- AST/HIR DestructuringBinding은 let keyword·colon/equals Span과 pattern/선택 type/initializer children을 가진다.
  기존 P24 pattern parser를 재사용하며 일반 IDENT/`_` binding·END·>= split·prefix recovery를 보존한다.
  Source gate는 정확한 let/colon/equals·containment·child order·pattern spelling·UTF-8 byte boundaries를 검사한다.
- Resolver는 initializer를 기존 scope에서 먼저 해석하고 source depth-first binder를 현재 block에 선언한다.
  binder는 immutable Copy local이다. duplicate/current-scope N2002·ancestor shadow·mutation N3004·scope escape/self-reference N2001을 유지한다.
- Checked.destructuring_bindings와 기존 recursive_patterns·definition_types·coercions는 statement/annotation/initializer·canonical component type·path·DefId를 저장한다.
  전체 initializer의 annotation context만 사용하며 pattern에서 타입을 추론하지 않는다.
  P24 typed pattern walker를 공유하고 기존 match/guard 경로를 보존한다. ErrorType/type/name/arity/resource 오류의 파생 N3103을 억제한다.
- bounded constructor matrix로 한 pattern이 타입 전체 domain을 덮는지 검사한다. 알려진 initializer 값으로 좁히지 않는다.
  N3103은 whole pattern primary·첫 refutable component secondary·최대 8개 missing witness+생략 표시다.
  statement당 10,000 pattern nodes·100,000 canonical tasks·1,000,000 live cells와 cache-hit 0·초과 전 allocation 제한을 유지한다.
- MIR은 initializer/coercion 후 Copy snapshot·source-order projection/binder assignment를 만든다. wildcard는 DefId/local을 만들지 않는다.
  Unit/모든 discard도 initializer effect를 보존한다. try Error는 snapshot/binder 전에 enclosing Result로 반환한다.
  single-variant payload도 명시적 tag edge 이후 읽는다. public analysis 재계산과 private full-body/source/tag/projection proofs가 same-type path·snapshot·effect·try/control·body 삭제를 거부한다.
- Lexer/keyword/END·Runtime/LLVM production·Cargo·aggregate ABI/layout은 변경하지 않았다. 기존 Copy/Project/EnumPayload/Call/Try를 사용한다.

## 테스트와 실제 결과

| 검사 | 결과 |
|---|---|
| cargo fmt --check | PASS |
| cargo check --workspace --all-features --offline | PASS |
| cargo clippy --workspace --all-targets --all-features --offline -- -D warnings | PASS |
| cargo test --workspace --offline | 420 PASS / 0 FAIL / 82 ignored |
| Runtime stage_a.rs rustfmt | PASS |
| LLVM 전체 opt-in | 22 PASS / 0 FAIL, COFF/ELF O0/O2 |
| Native 전체 opt-in | 60 PASS / 0 FAIL, debug/release |
| 문서 validator / finite irrefutability oracle 6개 | PASS |

총 502 PASS / 0 FAIL. 새 테스트 19개는 기본 16·LLVM 1·Native 2다.
28개 원 source bytes를 보존한 [수용 fixture](destructuring-proposal-fixtures/README.md)의 main/정상 3·부정 22·Runtime 1을 검증했다.
main 17줄·추가 정상 출력·initializer overflow의 before-only 출력/exit 1/정확한 Source Span을 debug/release에서 확인했다.
[독립 예제](../../examples/let_destructuring.nova)도 두 profile에서 3줄/LF·빈 stderr·exit 0이다.

Typecheck는 네 Bool-pair 값을 독립 열거해 9개 pattern × 3개 initializer 문맥의 타입 전체-domain irrefutability를 대조한다.
10,000/10,001 source nodes는 balanced product로, 별도 넓은 product는 1,000,000-live-cell 초과/let keyword/cascade를 검사한다.
P24 100,000-task/1,000,000-cell/cache-hit 경계·기존 match/guard 회귀도 유지한다.
Parser 제외 문법/depth·UTF-8 prefix·Source keyword/span/child-order forgery·Checked plan 삭제/initializer/annotation/pattern 변조·MIR same-type slot/effect/snapshot/inactive payload/body forgery를 거부한다.
Native는 annotation/import alias·opaque private struct·single/nullary Enum·float·zero-sized Unit·discard effects·snapshot mutation·shadow·loop·try·Abort를 관측한다.

Windows PID 재사용으로 이전 object 파일과 충돌했던 LLVM 회귀의 출력 경로를 PID+실행 timestamp로 구분했다. 기존 파일은 보존했다.
Native가 nova.exe를 실행하는 동안 전체 Cargo 재빌드가 Windows 파일 잠금에 막혀, Native 종료 후 전체 검증을 순서대로 실행했다.
검증 프로세스에만 CARGO_INCREMENTAL=0을 설정했으며 소스/Cargo 설정은 변경하지 않았다.
문서 검사는 Compiler/Native 실행과 별도다. Rust 1.99.0·LLVM 21.1.8·Windows x64 MSVC에서 검증했다. MSRV 1.80·Linux Native host 실행은 이번에 검증하지 않았다.

## 직접 사용과 후속 범위

[TESTING.md](../../TESTING.md)에 check·debug/release·집중/전체 회귀 명령을 제공한다.
var/const/global/parameter/for/assignment destructuring·let-else/guard·struct/or/range/general literal pattern·Array·일반 Move/borrow/Drop·전체 D08/D10/D12/D16/D25/D30은 후속 계약이다.
P01~P25·D01~D30 결정 본문·Canonical·원본 148개와 모든 이전 승인 fixture를 보존한다.
