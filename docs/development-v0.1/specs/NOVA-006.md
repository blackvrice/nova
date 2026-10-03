# NOVA-006 — Nova 언어 변경 제안·결정 기록 절차

| 항목 | 값 |
|---|---|
| 대상 | Nova 0.1 |
| 작성일 | 2026-10-03 |
| 문서 상태 | Draft — 기존 결정은 유지, 추가 상세는 승인 대기 |
| 적용 Stage | 전 Stage |
| 근거 | [원본 NOVA-006](../../00_Governance/NOVA-006_Nova_언어_변경_제안_결정_기록_절차.md) |

이 문서는 원본을 대체하는 확정 사양이 아니다. 기존 확정 기준은 [CANONICAL](../CANONICAL.md)을 따르며, 새 의미·문법·API·정책은 [DECISIONS](../DECISIONS.md)의 승인이 필요하다. D01~D05 Lexer 상세는 [승인 기준](../ACCEPTED_LEXER.md)을 따른다. [전체 색인](../INDEX.md).

## 변경 절차
의미 변경은 제안 → 검토 → 승인 → 문서 동결 → 구현 → 회귀 테스트 순서다. 상태는 Draft, Accepted, Rejected, Superseded이며 작성 날짜와 승인 증거를 기록한다.

## 제안 필드
관련 NOVA ID, 현재 사양, 문제/반례, 제안 의미, 문법 예제, 영향 계층, 하위 호환성, 대안, 수용 테스트, 승인 기록을 필수로 한다. DECISIONS는 이 형식의 묶음이며 승인 기록이 없는 항목은 Draft다.

## 충돌 처리
Canonical > Governance/Freeze > Language > Architecture > Implementation > Code > Tests. 같은 우선순위의 충돌은 임의로 선택하지 않고 blocking issue로 기록한다. 구현 편의의 로컬 예외를 사양으로 승격하지 않는다.

## 완료
승인된 규칙의 문서/grammar/fixture가 함께 변경되어야 한다. 이번 보완팩은 원본 변경 없이 검토 가능한 초안을 제공한다.
