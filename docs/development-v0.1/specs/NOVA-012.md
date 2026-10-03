# NOVA-012 — 주석·중첩 주석 사양서

| 항목 | 값 |
|---|---|
| 대상 | Nova 0.1 |
| 작성일 | 2026-10-03 |
| 문서 상태 | Draft — 기존 결정은 유지, 추가 상세는 승인 대기 |
| 적용 Stage | A~B |
| 근거 | [원본 NOVA-012](../../01_Source_Syntax/NOVA-012_주석_중첩_주석_사양서.md) |

이 문서는 원본을 대체하는 확정 사양이 아니다. 기존 확정 기준은 [CANONICAL](../CANONICAL.md)을 따르며, 새 의미·문법·API·정책은 [DECISIONS](../DECISIONS.md)의 승인이 필요하다. D01~D05 Lexer 상세는 [승인 기준](../ACCEPTED_LEXER.md)을 따른다. [전체 색인](../INDEX.md).

## Comment 초안 — D04
//는 다음 논리 newline 전까지, /* ... */는 중첩 깊이 0까지 comment다. 문자열 안의 comment 표기는 text다. Block comment 안의 quote는 string mode를 열지 않는다.

## Source와 END
comment는 Span과 원문을 보존하고 내부 newline event를 END normalizer에 제공한다. a /* newline */ b를 a b와 같게 만들지 않는다. 최종 END 여부는 expression/delimiter 문맥에 따른다.

## Doc comment 제안 — D24
///와 /** ... */를 선언 문서 trivia로 분류한다. 일반 comment와 동일한 lexical 안전 규칙을 사용한다. 주석의 위치 이동이 doc 대상 선언을 바꾸면 formatter 오류로 본다.

## 검증
/* /* */ */ 정상, /* EOF 실패, // 마지막 EOF 정상. comment 제거 후 token Spans와 원문 복원이 동일한지, nesting limit 초과가 진단인지 확인한다.
