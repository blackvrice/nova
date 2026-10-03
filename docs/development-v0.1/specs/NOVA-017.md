# NOVA-017 — Parser 오류 복구 사양서

| 항목 | 값 |
|---|---|
| 대상 | Nova 0.1 |
| 작성일 | 2026-10-03 |
| 문서 상태 | Draft — 기존 결정은 유지, 추가 상세는 승인 대기 |
| 적용 Stage | A~B |
| 근거 | [원본 NOVA-017](../../01_Source_Syntax/NOVA-017_Parser_오류_복구_사양서.md) |

이 문서는 원본을 대체하는 확정 사양이 아니다. 기존 확정 기준은 [CANONICAL](../CANONICAL.md)을 따르며, 새 의미·문법·API·정책은 [DECISIONS](../DECISIONS.md)의 승인이 필요하다. D01~D05 Lexer 상세는 [승인 기준](../ACCEPTED_LEXER.md)을 따른다. [전체 색인](../INDEX.md).

## 복구 전략
예상 token 누락이면 zero-width synthetic token과 진단을 추가한다. 예상하지 않은 token은 소비하거나 동기화한다. END/}/top-level 선언/주요 statement keyword가 동기화 후보다.

## 진행 불변 조건
각 반복은 cursor 증가, enclosing parser로 반환, 또는 EOF 종료 중 하나를 수행한다. 내부 }를 outer block 닫힘으로 무조건 삼키지 않는다. delimiter stack에는 opening Span을 저장한다.

## 진단 제한 초안 — D30
한 원인에서 다수의 expected-token 오류가 나오면 최초 오류와 구조 복구 note로 제한한다. per-file budget 도달 시 추가 오류 요약을 출력하고 종료한다. IDE mode와 batch mode의 성공 판정은 같아야 한다.

## 검증
func f(, let x=, if {, 닫힘 없는 nested block, 임의 token stream을 timeout 아래 처리하고 후속 정상 선언이 보존되는지 확인한다.
