# NOVA-048 — Match Lowering·Decision Tree 사양서

| 항목 | 값 |
|---|---|
| 대상 | Nova 0.1 |
| 작성일 | 2026-10-03 |
| 문서 상태 | Draft — 기존 결정은 유지, 추가 상세는 승인 대기 |
| 적용 Stage | A~B |
| 근거 | [원본 NOVA-048](../../04_Functions_Control/NOVA-048_Match_Lowering_Decision_Tree_사양서.md) |

이 문서는 원본을 대체하는 확정 사양이 아니다. 기존 확정 기준은 [CANONICAL](../CANONICAL.md)을 따르며, 새 의미·문법·API·정책은 [DECISIONS](../DECISIONS.md)의 승인이 필요하다. [전체 색인](../INDEX.md).

## Decision tree lowering
scrutinee는 한 번 평가하여 local/place에 저장한다. variant tag/literal test를 공유하되 arm 순서, guard side effect, loan lifetime은 보존한다. payload는 variant 검사가 성공한 뒤만 접근한다.

## CFG 계약
각 arm entry/binding/guard/body/join block을 구분한다. guard false는 다음 arm으로 이동하고 guard 임시값 cleanup을 수행한다. early return/try는 enclosing cleanup으로 연결한다.

## 검증
scrutinee call count=1, guard 순서 trace, inactive payload 접근 없음, source arm order가 최적화 뒤에도 동일한지 검사한다. exhaustive analysis 결과가 없으면 codegen으로 넘기지 않는다.
