# P12 Copy struct 수용 fixture (Accepted)

[승인 계약](../STRUCT_STAGE_B_PROPOSAL.md)을 구현한 Compiler/Native 수용 데이터다.
`expected.json`의 validated_result와 부정 10사례의 진단 코드·UTF-8 byte Span을 실제 검증했다.
[구현·검증 기록](../STRUCT_IMPLEMENTATION.md)에 테스트 범위와 환경을 정리했다.

- main.nova: type alias import, 전역 const 생성, nested Copy, 가변 field 경로, let snapshot, 함수 return과 scalar 보간.
- geometry.nova: 공개 struct/field와 typed function. alias는 원 nominal ID를 보존한다.
- Windows Native debug/release stdout: `original=21, snapshot=20, shifted=21, tag=🙂` + LF, stderr 없음, exit 0.
- 부정 10사례: String field, recursive layout, let root/field, 생성 인수 개수, missing field,
  nominal mismatch, value shadow, aggregate interpolation, 사용자 init. primary는 UTF-8 반열린 byte Span이다.

저장소 root에서 LLVM 21.1.8과 Rust/MSVC 환경으로 실행한다.

```powershell
cargo run -p nova-cli --offline -- check docs/development-v0.1/struct-proposal-fixtures/main.nova
$env:NOVA_CLANG = (Resolve-Path target/toolchains/llvm-21.1.8/bin/clang.exe).Path
cargo run -p nova-cli --offline -- run docs/development-v0.1/struct-proposal-fixtures/main.nova --profile debug
cargo run -p nova-cli --offline -- run docs/development-v0.1/struct-proposal-fixtures/main.nova --profile release
```

문서 validator는 승인 상태/결과 데이터/진단 코드/Span/UTF-8과 grammar 변경 범위를 검사한다.
Compiler와 Native는 별도 Cargo tests로 검증한다. private field/type·원자 namespace import·자원 상한·
const budget·손상 MIR은 추가 구현 테스트에 포함한다. standalone [예제](../../../examples/structs.nova)도 제공한다.
