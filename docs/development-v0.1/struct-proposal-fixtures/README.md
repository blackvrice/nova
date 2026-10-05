# P12 Copy struct 수용 fixture (Draft)

[제안 계약](../STRUCT_STAGE_B_PROPOSAL.md)의 검토용 데이터다. 승인/구현/Native 검증은 아직 하지 않았다.
`expected.json`의 proposed_result와 negative_cases는 구현 후 확인할 기대값이며 현재 compiler의 결과가 아니다.

- main.nova: type alias import, 전역 const 생성, nested Copy, 가변 field 경로, let snapshot, 함수 return과 scalar 보간.
- geometry.nova: 공개 struct/field와 typed function. 선언 순서와 원 nominal ID가 유지되어야 한다.
- 기대 stdout: `original=21, snapshot=20, shifted=21, tag=🙂` + LF, exit 0.
- 부정 사례 10개: String field, recursive layout, let root/field, 생성 인수 개수, missing field,
  nominal mismatch, value shadow, aggregate interpolation, 사용자 init. 각 primary는 UTF-8 반열린 byte Span이다.

문서 validator는 상태/기대 데이터/진단 코드/Span/UTF-8과 grammar 변경 범위를 검사한다.
Compiler pass/fail 또는 Native 실행 검증은 수행하지 않는다. private field/type와 다중 namespace import,
자원 상한·const budget·MIR 손상 사례는 제안서의 별도 구현 테스트 계획에 포함된다.
