# P11 Draft Module fixture

상태: **Draft / implementation_verified=false**. 현재 Compiler에는 use/visibility를 적용하지 않았다.
[P11 검토 계약](../MODULE_STAGE_B_PROPOSAL.md)의 구현 뒤 수용 기준이며 현재 성공 예제가 아니다.

- entry: [main.nova](main.nova), source root: 이 디렉터리.
- imported module: [math.nova](math.nova). main↔math import cycle는 함수 참조이므로 허용한다.
- alias plus는 math의 add DefId, OFFSET은 cached global const다. SECRET은 private라 main에서 import할 수 없다.
- 구현 뒤 check exit 0, Native stdout `value=42` + LF, exit 0을 기대한다.
- [기계 판독 기대값](expected.json). 이 데이터는 현재 실행 결과가 아니다.
