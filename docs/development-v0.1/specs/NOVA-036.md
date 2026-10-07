# NOVA-036 — 위치·이름·기본 인수 사양서

| 항목 | 값 |
|---|---|
| 대상 | Nova 0.1 |
| 작성일 | 2026-10-03 |
| 문서 상태 | Draft — 기존 결정은 유지, 추가 상세는 승인 대기 |
| 적용 Stage | A~B |
| 근거 | [원본 NOVA-036](../../04_Functions_Control/NOVA-036_위치_이름_기본_인수_사양서.md) |

이 문서는 원본을 대체하는 확정 사양이 아니다. 기존 확정 기준은 [CANONICAL](../CANONICAL.md)을 따르며, 새 의미·문법·API·정책은 [DECISIONS](../DECISIONS.md)의 승인이 필요하다. D01~D05 Lexer 상세는 [승인 기준](../ACCEPTED_LEXER.md)을 따른다. [전체 색인](../INDEX.md).

사용자 승인한 함수 이름 인수·parameter mapping·source-order snapshot/try·진단 계약은 [P17 Accepted](../NAMED_ARGUMENTS_STAGE_B_PROPOSAL.md), [52-production EBNF](../GRAMMAR_STAGE_B_NAMED_ARGUMENTS.ebnf), [수용 fixture](../named-arguments-proposal-fixtures/README.md), [구현 기록](../NAMED_ARGUMENTS_IMPLEMENTATION.md)을 따른다. 구현·검증 완료이며 기본 인수·overload·named constructor와 전체 D11/D16/D25/D30 승인이 아니다.

## Argument mapping 초안 — D11
위치 인수 뒤에 이름 인수(label: expression)를 허용하고 이름 인수 뒤 위치 인수는 거부한다. 같은 parameter를 두 번 채우거나 알 수 없는 label은 오류다. label은 overload 선택 signature의 일부다.

## 평가
제공된 인수는 source order로 임시값에 저장한 후 parameter order로 전달한다. 기본 인수는 원본 규칙대로 caller에서 평가한다. 빠진 default 인수는 제공 인수 뒤 declaration order로 평가하며 이전 parameter 참조는 첫 승인안에서 금지 제안이다.

## 검증
named 순서 반전에도 출력은 source order, 중복/누락/unknown label fail, default side effect 한 번, default 타입 오류는 선언과 호출을 함께 표시한다.
