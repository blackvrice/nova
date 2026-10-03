# NOVA-038 — 함수 타입·Lambda·Closure 사양서

| 항목 | 값 |
|---|---|
| 대상 | Nova 0.1 |
| 작성일 | 2026-10-03 |
| 문서 상태 | Draft — 기존 결정은 유지, 추가 상세는 승인 대기 |
| 적용 Stage | D |
| 근거 | [원본 NOVA-038](../../04_Functions_Control/NOVA-038_함수_타입_Lambda_Closure_사양서.md) |

이 문서는 원본을 대체하는 확정 사양이 아니다. 기존 확정 기준은 [CANONICAL](../CANONICAL.md)을 따르며, 새 의미·문법·API·정책은 [DECISIONS](../DECISIONS.md)의 승인이 필요하다. D01~D05 Lexer 상세는 [승인 기준](../ACCEPTED_LEXER.md)을 따른다. [전체 색인](../INDEX.md).

## Lambda 초안 — D13
lambda (parameters) => expression 또는 lambda (parameters) => { statements } 철자는 제안이며 원본 keyword 목록에 없으므로 승인 전 contextual syntax로도 구현하지 않는다. expected function signature에서 parameter type을 추론한다.

## 의미
capture 없는 lambda는 function pointer로 변환 가능하다. capture 있는 closure는 environment+call function의 구체 타입이고 동적 interface object가 아니다. 캡처 모드는 Read/change/take다.

## 검증
capture-free conversion, expected type 부재 parameter 오류, returned borrowed closure의 owner 수명 오류, one-shot take capture 호출 2회 오류, environment Drop 정확성을 검사한다.
