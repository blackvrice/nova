# P11 Module 수용 fixture

상태: **Accepted / 2026-10-05 사용자 승인 · implementation_verified=true**.
[P11 계약](../MODULE_STAGE_B_PROPOSAL.md)과 [구현·검증 기록](../MODULE_IMPLEMENTATION.md)을 따른다.

- entry: [main.nova](main.nova), source root: 이 디렉터리(기본값).
- imported module: [math.nova](math.nova). main↔math 함수 import cycle를 허용한다.
- alias plus는 math의 add DefId, OFFSET은 cached global const다. SECRET은 private다.
- check exit 0; Native debug/release stdout `value=42` + LF, stderr 없음, exit 0.
- [기계 기대값](expected.json). 문서 validator는 데이터를 검사하며 실행은 별도 Cargo Native test가 수행한다.

```powershell
cargo run -p nova-cli -- check docs/development-v0.1/module-proposal-fixtures/main.nova
cargo run -p nova-cli -- run docs/development-v0.1/module-proposal-fixtures/main.nova --profile debug
cargo run -p nova-cli -- run docs/development-v0.1/module-proposal-fixtures/main.nova --profile release
```

Native는 LLVM 21.1.8/NOVA_CLANG과 Windows x64 Rust/MSVC가 필요하다.
