# NOVA-009 — Token 종류·예약어·연산자 사양서

| 항목 | 값 |
|---|---|
| 대상 | Nova 0.1 |
| 작성일 | 2026-10-03 |
| 문서 상태 | Draft — 기존 결정은 유지, 추가 상세는 승인 대기 |
| 적용 Stage | A~B |
| 근거 | [원본 NOVA-009](../../01_Source_Syntax/NOVA-009_Token_종류_예약어_연산자_사양서.md) |

이 문서는 원본을 대체하는 확정 사양이 아니다. 기존 확정 기준은 [CANONICAL](../CANONICAL.md)을 따르며, 새 의미·문법·API·정책은 [DECISIONS](../DECISIONS.md)의 승인이 필요하다. [전체 색인](../INDEX.md).

## Token 모델
Token은 kind, Span, 원문 slice를 가진다. Identifier/Keyword, Integer/Float/String/Char, 보간 경계, 구분자, Operator, NewLine, Trivia, Error, EOF를 분리한다. Trivia까지 포함한 원문 연결이 원본과 같아야 한다.

## Operator 초안 — D03
기본 집합은 + - * / % ! && || == != < <= > >= = 및 -> => ? . , : ; ( ) [ ] { }다. until/through/as/exists는 단어 연산자다. Compound assignment, bitwise, shift, pointer 철자는 D03에서 승인 전 제외/보류한다.

## 규칙
최장 일치 후 정확한 키워드를 판정한다. 비교 연산자보다 긴 =>/->를 우선하고 숫자 부호는 prefix operator로 둔다. Generic 닫는 >와 비교 >는 같은 raw token이며 Parser가 문맥을 판정한다.

## 검증
identifier에 keyword prefix가 들어간 경우, =/==/=>, 잘못된 문자, EOF 1개, Span 겹침 없는 원문 재구성을 확인한다.
