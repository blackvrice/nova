# P26 착수 준비·검증 기록

2026-10-12. **Draft / 미승인·미구현**. [제안 계약](DESTRUCTURING_STAGE_B_PROPOSAL.md)·[수용 계획](destructuring-proposal-fixtures/README.md)을 준비한 기록이다. 언어 구현 완료 기록이 아니다.

- 62-production 제안 grammar: P25 binding만 확장·destructuring_pattern 추가·기존 60개 보존.
- 두 파일 main/types·정상 3·부정 22·Runtime 1, source 28개 UTF-8/LF byte Span·제안 출력 17줄.
- 독립 finite type-domain irrefutability oracle 6개: Tuple/Unit/single Enum·known Bool/Enum/Option initializer가 타입 domain을 좁히지 않음.
- N3103은 P26에만 제안한 미등록 code다. 기존 DIAGNOSTICS.csv·공용 registry·Compiler에는 추가하지 않았다.
- 보완 초안 연결: NOVA-014/029/035/037/038/044/046/047/048/073/074/075/076/077/078/081/083/087/091/094/095/136에 P26 Draft 경계를 추가했다.
- 문서 generator/validator PASS: 원본 148개 hash·승인 ledger·grammar/링크·Source/Span·Draft/proposed 진단 상태.
- 별도 hash 대조 PASS: 보호 파일 908개 bytes·P01~P25 accepted_proposals/accepted_decisions·D01~D30 결정 본문 보존.
  Compiler/Runtime/LLVM production·Cargo·기존 승인 grammar/계약/fixture·사용자 예제/IDE 작업은 변경하지 않았다.
- Cargo fmt/check --workspace --all-features --offline/clippy --workspace --all-targets --all-features --offline -- -D warnings PASS.
- cargo test --workspace --offline: **404 PASS / 0 FAIL / 79 ignored**. Runtime stage_a.rs rustfmt PASS.
  79개 opt-in LLVM/Native는 이번 문서 작업에서 재실행하지 않았다. 직전 P25의 LLVM 21·Native 58은 [P25 구현 기록](GUARD_IMPLEMENTATION.md)을 따른다.
- 검증 프로세스에만 CARGO_INCREMENTAL=0을 설정했으며 소스/Cargo 설정은 변경하지 않았다.
- 현재 examples/match_guards.nova CLI check exit 0, P26 types.nova의 기존 helper 선언 CLI check exit 0을 확인했다.
- 현재 P26 main.nova CLI check는 **exit 1 / N1101 등 미지원 구문 진단**이다.
  proposed_result의 향후 exit 0·17줄·부정 진단/Span·Runtime Abort는 아직 Compiler/Native에서 검증하지 않았다.

승인 후 AST/HIR/Source·initializer-first scope/typing·irrefutability·진단 등록·MIR/LLVM와 실제 Native 수용 검증을 진행한다.
