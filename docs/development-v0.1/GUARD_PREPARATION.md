# P25 착수 준비·검증 기록

2026-10-12. **Draft / 미승인·미구현**. [제안 계약](GUARD_STAGE_B_PROPOSAL.md)·[수용 계획](guard-proposal-fixtures/README.md)을 준비한 기록이다. 언어 구현 완료 기록이 아니다.

- 보완 초안 연결: NOVA-014/043/046/047/048/074/076/078/081/083/087/091/094/095/136에 P25 Draft 경계를 추가했다. 확정된 의미를 적용하지 않는다.
- 61-production 제안 grammar: P24 match_arm만 확장·match_guard 하나 추가·기존 59개 보존.
- 두 파일 main/types·추가 정상 3·부정 16·Runtime 1, 원 source 22개 UTF-8/LF byte Span·제안 출력 16줄.
- 독립 finite-domain reference oracle 6개: unguarded union coverage/usefulness·mismatch skip·false continuation·guard trace.
- 문서 generator/validator PASS: 원본 148개 hash·P01~P24 승인 ledger·grammar/링크·source/Span·Draft 상태.
- 별도 hash 대조 PASS: 보호 파일 884개 bytes·accepted_proposals/accepted_decisions·D01~D30 결정 본문 보존.
  Compiler/Runtime/LLVM production·Cargo·기존 승인 grammar/계약/fixture·사용자 예제와 IDE 작업을 변경하지 않았다.
- Cargo fmt/check --workspace --all-features --offline/clippy --workspace --all-targets --all-features --offline -- -D warnings PASS.
- cargo test --workspace --offline: **392 PASS / 0 FAIL / 76 ignored**. Runtime stage_a.rs rustfmt PASS.
  76개 opt-in LLVM/Native는 이번 문서 작업에서 재실행하지 않았다. 2026-10-08 P24 기록의 LLVM 20·Native 56은 [P24 구현 기록](NESTED_PATTERN_IMPLEMENTATION.md)을 따른다.
- 검증 프로세스에만 CARGO_INCREMENTAL=0을 설정했다. Cargo/소스 설정은 변경하지 않았다.
- 현재 구현된 examples/nested_patterns.nova의 CLI check exit 0, P25 types.nova의 기존 helper 선언 CLI check exit 0을 확인했다.
- 현재 P25 main.nova의 CLI check는 **exit 1 / N1102**의 unsupported guard 진단이다.
  proposed_result의 향후 exit 0·16줄·부정 진단/Span·Runtime Abort는 아직 Compiler/Native에서 검증하지 않았다.

승인 후 AST/HIR/Source·guard scope/type/flow·unguarded matrix coverage·MIR/LLVM와 실제 Native 수용 검증을 진행한다.
