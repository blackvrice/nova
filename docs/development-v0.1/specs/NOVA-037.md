# NOVA-037 — Overload 해석·변환 비용 사양서

| 항목 | 값 |
|---|---|
| 대상 | Nova 0.1 |
| 작성일 | 2026-10-03 |
| 문서 상태 | Draft — 기존 결정은 유지, 추가 상세는 승인 대기 |
| 적용 Stage | A~B |
| 근거 | [원본 NOVA-037](../../04_Functions_Control/NOVA-037_Overload_해석_변환_비용_사양서.md) |

이 문서는 원본을 대체하는 확정 사양이 아니다. 기존 확정 기준은 [CANONICAL](../CANONICAL.md)을 따르며, 새 의미·문법·API·정책은 [DECISIONS](../DECISIONS.md)의 승인이 필요하다. D01~D05 Lexer 상세는 [승인 기준](../ACCEPTED_LEXER.md)을 따른다. [전체 색인](../INDEX.md).

## Overload 초안 — D11
후보는 이름/visibility/arity/label/mode로 필터하고 generic inference 후 변환 비용을 계산한다. identity=0, lossless numeric widening=1, 나머지 implicit conversion은 불허를 제안한다.

## 선택 알고리즘
parameter별 비용 vector의 Pareto dominance로 후보를 비교한다. 하나가 모든 위치에서 같거나 작고 한 곳에서 작으면 우수하다. 유일한 우수 후보가 없으면 ambiguous다. declaration order, 기본 인수 개수, 반환 타입에 임의 tie-break를 넣지 않는다.

## 검증
exact vs widening, (0,1) vs (1,0) ambiguity, generic/non-generic 동률, inaccessible candidate, ErrorType로 오염된 후보의 중복 진단 억제를 다룬다.
