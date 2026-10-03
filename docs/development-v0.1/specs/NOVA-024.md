# NOVA-024 — 이름 충돌·Shadowing 진단 기준서

| 항목 | 값 |
|---|---|
| 대상 | Nova 0.1 |
| 작성일 | 2026-10-03 |
| 문서 상태 | Draft — 기존 결정은 유지, 추가 상세는 승인 대기 |
| 적용 Stage | B~E |
| 근거 | [원본 NOVA-024](../../02_Names_Modules/NOVA-024_이름_충돌_Shadowing_진단_기준서.md) |

이 문서는 원본을 대체하는 확정 사양이 아니다. 기존 확정 기준은 [CANONICAL](../CANONICAL.md)을 따르며, 새 의미·문법·API·정책은 [DECISIONS](../DECISIONS.md)의 승인이 필요하다. D01~D05 Lexer 상세는 [승인 기준](../ACCEPTED_LEXER.md)을 따른다. [전체 색인](../INDEX.md).

## 충돌/Shadow 구분
duplicate는 같은 scope/namespace의 충돌, shadow는 다른 nested scope의 동일 이름이다. overload는 signature 규칙을 만족하는 함수만 묶으며 반환 타입만 다른 함수는 duplicate다.

## 진단 계약
duplicate의 primary는 뒤 선언, secondary는 앞 선언이다. ambiguity의 primary는 참조, secondary는 모든 경쟁 후보이며 정렬 순서는 module path/signature다. shadow lint는 기존 binding을 표시한다.

## 초안 코드
N2001 undefined name, N2002 duplicate definition, N2003 ambiguous name, N2004 inaccessible item을 DIAGNOSTICS.csv에서 제안한다. 아직 승인된 고정 코드라는 뜻은 아니다.

## 검증
순서를 바꿔도 후보 목록이 결정적이고, Error resolution에서 파생된 타입 오류를 중복 출력하지 않는지 확인한다.
