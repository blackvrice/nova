# P24 착수 준비·검증 기록

2026-10-08. **Draft / 미승인·미구현**. [제안 계약](NESTED_PATTERN_STAGE_B_PROPOSAL.md)과
[수용 계획](nested-pattern-proposal-fixtures/README.md)을 준비한 기록이다. 언어 구현 완료 기록이 아니다.

- 60-production 제안 grammar: 기존 P22 pattern 두 production만 확장·두 production 추가·나머지 56개 보존.
- 두 파일 main/types·추가 정상 3·부정 18·Runtime 1과 고정 UTF-8/LF byte Span·제안 출력 12줄.
- 독립 finite-domain oracle 6개: product/sum·부분 overlap·합집합 unreachable·누락 coverage.
- 문서 generator/validator PASS: 원본 148개 hash·승인 grammar/ledger·링크·source/Span·Draft 상태.
- 별도 hash 대조 PASS: 기존 파일 602개, P01~P23 accepted_proposals, accepted_decisions와 D01~D30 결정 본문 보존.
- Cargo fmt / check --workspace --all-features --offline / clippy --workspace --all-targets --all-features --offline -- -D warnings PASS.
- cargo test --workspace --offline: **372 PASS / 0 FAIL / 73 ignored**. runtime rustfmt PASS.
  73개 opt-in LLVM/Native는 이번 문서 작업에서 재실행하지 않았다. 직전 P23의 실행 기록은 [P23 구현 기록](STRUCT_NAMED_ARGUMENTS_IMPLEMENTATION.md)을 따른다.
- OneDrive incremental cache에서 hard-link/cleanup 경고가 발생해 check/clippy는 CARGO_INCREMENTAL=0으로 다시 확인했다.
  이 설정은 해당 검증 프로세스에만 적용했으며 Cargo/소스 설정은 변경하지 않았다.
- 현재 구현된 examples/struct_named_arguments.nova의 CLI check exit 0을 확인했다.
- 현재 P24 main.nova의 CLI check는 exit 1 / N1102를 포함한 unsupported 진단이다.
  proposed_result의 향후 exit 0·출력·부정 진단은 아직 Compiler/Native에서 검증하지 않았다.

사용자 예제 변경·IDE 파일은 이번 작업에 포함하지 않는다. Compiler/Runtime/Cargo 구현은 변경하지 않았다.
승인 후 AST/HIR/Source·recursive typing/coverage·MIR/LLVM와 실제 Native 수용 검증을 진행한다.
