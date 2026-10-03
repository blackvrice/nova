# NOVA-039 — Closure Capture·호출 Receiver 사양서

| 항목 | 값 |
|---|---|
| 대상 | Nova 0.1 |
| 작성일 | 2026-10-03 |
| 문서 상태 | Draft — 기존 결정은 유지, 추가 상세는 승인 대기 |
| 적용 Stage | D |
| 근거 | [원본 NOVA-039](../../04_Functions_Control/NOVA-039_Closure_Capture_호출_Receiver_사양서.md) |

이 문서는 원본을 대체하는 확정 사양이 아니다. 기존 확정 기준은 [CANONICAL](../CANONICAL.md)을 따르며, 새 의미·문법·API·정책은 [DECISIONS](../DECISIONS.md)의 승인이 필요하다. D01~D05 Lexer 상세는 [승인 기준](../ACCEPTED_LEXER.md)을 따른다. [전체 색인](../INDEX.md).

## Capture 분석
closure body의 free Place 사용을 모은 뒤 Read/change/take 필요도를 결정한다. 필요 모드가 충돌하면 가장 강한 의미로 조용히 바꾸지 않고 closure 선언에서 소유권 요구를 보여준다.

## 초안 — D13
Read capture는 loan, change는 exclusive loan, take는 owned environment field다. capture list의 명시 철자와 기본 capture 정책을 D13에서 제안한다. take capture가 소비되는 closure는 take call receiver로 한 번만 호출 가능하다.

## 검증
같은 owner의 두 change closure 동시 live fail, owner보다 오래 사는 borrowed closure fail, move closure 반환 pass, field partial capture가 사용자 partial move를 우회하지 못하는지 확인한다.
