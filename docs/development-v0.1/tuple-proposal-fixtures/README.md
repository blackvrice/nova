# P13 Copy Tuple 수용 fixture (Draft)

[제안 계약](../TUPLE_STAGE_B_PROPOSAL.md)의 검토용 데이터이며 아직 승인/구현/Native 검증하지 않았다.
`expected.json`의 proposed_result와 부정 10사례는 기대값이다. 현 Compiler의 결과나 통과 증거가 아니다.

- main.nova/tuples.nova: structural tuple type·one-tuple·nested selector·struct 내 tuple·const import·Copy/return·가변 경로.
- 제안 stdout: `original=21, snapshot=20, shifted=21, one=7, tag=🙂` + LF, exit 0.
- 부정 사례: 불변 root/field, index 범위, arity/element/nominal mismatch, String element,
  exponent selector, aggregate 보간, struct/tuple recursive layout. Primary는 UTF-8 반열린 byte Span이다.

승인 후 구현을 마치면 다음 사용자 실행 명령을 검증해 제공한다. 현재 버전에서는 tuple 미지원 진단이 예상된다.

```powershell
cargo run -p nova-cli --offline -- check docs/development-v0.1/tuple-proposal-fixtures/main.nova
cargo run -p nova-cli --offline -- run docs/development-v0.1/tuple-proposal-fixtures/main.nova --profile debug
cargo run -p nova-cli --offline -- run docs/development-v0.1/tuple-proposal-fixtures/main.nova --profile release
```

문서 validator는 grammar·상태·기대값·진단 코드·Span의 유효성만 검사한다. Compiler/Native 실행은 별도다.
현재 실행 가능한 [P12 예제와 테스트 명령](../../../TESTING.md)을 함께 제공한다.
