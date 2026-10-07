# P13 Copy Tuple 수용 fixture (Accepted)

[승인 계약](../TUPLE_STAGE_B_PROPOSAL.md)의 Compiler/Native 수용 데이터다.
`expected.json`의 validated_result와 부정 10사례의 정확한 code/UTF-8 byte Span을 검증했다.
[구현·검증 기록](../TUPLE_IMPLEMENTATION.md)을 따른다.

- main.nova/tuples.nova: structural tuple type·one-tuple·nested selector·struct 내 tuple·const import·Copy/return·가변 경로.
- 검증한 debug/release stdout: `original=21, snapshot=20, shifted=21, one=7, tag=🙂` + LF, exit 0.
- 부정 사례: 불변 root/field, index 범위, arity/element/nominal mismatch, String element,
  exponent selector, aggregate 보간, struct/tuple recursive layout. Primary는 UTF-8 반열린 byte Span이다.

저장소 root의 PowerShell에서 실행한다. LLVM 21.1.8/Rust/MSVC가 필요하다. 아래 명령을 실제 실행했다.

```powershell
$env:NOVA_CLANG = (Resolve-Path target/toolchains/llvm-21.1.8/bin/clang.exe).Path
cargo run -p nova-cli --offline -- check docs/development-v0.1/tuple-proposal-fixtures/main.nova
cargo run -p nova-cli --offline -- run docs/development-v0.1/tuple-proposal-fixtures/main.nova --profile debug
cargo run -p nova-cli --offline -- run docs/development-v0.1/tuple-proposal-fixtures/main.nova --profile release
```

문서 validator는 grammar·상태·기대값·진단 코드·Span의 유효성만 검사한다. Compiler/Native 실행은 별도다.
현재 실행 가능한 [예제와 테스트 명령](../../../TESTING.md)을 함께 제공한다.
