# NOVA-120 — Result<T,E>·try API 사양서

| 항목 | 값 |
|---|---|
| 대상 | Nova 0.1 |
| 작성일 | 2026-10-03 |
| 문서 상태 | Draft — 기존 결정은 유지, 추가 상세는 승인 대기 |
| 적용 Stage | B~E (print: A) |
| 근거 | [원본 NOVA-120](../../11_Standard_Library/NOVA-120_Result_TE_try_API_사양서.md) |

이 문서는 원본을 대체하는 확정 사양이 아니다. 기존 확정 기준은 [CANONICAL](../CANONICAL.md)을 따르며, 새 의미·문법·API·정책은 [DECISIONS](../DECISIONS.md)의 승인이 필요하다. D01~D05 Lexer 상세는 [승인 기준](../ACCEPTED_LEXER.md)을 따른다. [전체 색인](../INDEX.md).

## Result<T,E>
Success(T)/Error(E), Unit 성공 Success(()), try Error 조기 반환을 사용한다. payload mode와 enclosing E mismatch를 검사한다.

## API 초안 — D23
isSuccess/isError는 Read; take unwrap→T는 Error panic; take unwrapError→E는 Success panic. map/mapError/andThen는 Closure 지원 이후 signatures를 확정하며 call mode와 Error cleanup을 명시한다.

## 검증
Move T/E 정확히 한 번, try chain order, success/Error branch inactive Drop 없음, unwrap panic stderr, no automatic E conversion.
