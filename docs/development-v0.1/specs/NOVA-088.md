# NOVA-088 — Match Lowering 구현서

| 항목 | 값 |
|---|---|
| 대상 | Nova 0.1 |
| 작성일 | 2026-10-03 |
| 문서 상태 | Draft — 기존 결정은 유지, 추가 상세는 승인 대기 |
| 적용 Stage | B~E |
| 근거 | [원본 NOVA-088](../../08_MIR_Middleend/NOVA-088_Match_Lowering_구현서.md) |

이 문서는 원본을 대체하는 확정 사양이 아니다. 기존 확정 기준은 [CANONICAL](../CANONICAL.md)을 따르며, 새 의미·문법·API·정책은 [DECISIONS](../DECISIONS.md)의 승인이 필요하다. [전체 색인](../INDEX.md).

## Match pass
Pattern matrix에서 coverage와 decision tree를 만들되 NOVA-047 분석과 동일 constructor set을 사용한다. payload projection은 verified tag branch 아래에만 놓는다.

## 보존 조건
guard 순서/side effect, binding mode/loan region, scrutinee once, arm source order, cleanup을 유지한다. payload destructive extraction을 guard 전에 수행하여 실패 후 다음 arm에서 읽을 수 없게 만들지 않는다.

## 검증
nested enum tests 공유, wildcard unreachable, guarded fallback, moving whole scrutinee, partial move fail, same trace with/without tree optimization.
