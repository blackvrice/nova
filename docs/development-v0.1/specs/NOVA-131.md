# NOVA-131 — Formatter 사양·구현서

| 항목 | 값 |
|---|---|
| 대상 | Nova 0.1 |
| 작성일 | 2026-10-03 |
| 문서 상태 | Draft — 기존 결정은 유지, 추가 상세는 승인 대기 |
| 적용 Stage | E |
| 근거 | [원본 NOVA-131](../../12_Tooling_Packaging/NOVA-131_Formatter_사양_구현서.md) |

이 문서는 원본을 대체하는 확정 사양이 아니다. 기존 확정 기준은 [CANONICAL](../CANONICAL.md)을 따르며, 새 의미·문법·API·정책은 [DECISIONS](../DECISIONS.md)의 승인이 필요하다. D01~D05 Lexer 상세는 [승인 기준](../ACCEPTED_LEXER.md)을 따른다. [전체 색인](../INDEX.md).

## Formatter
AST+Trivia 기반 document model에서 단일 canonical output을 만든다. 원본 기본 줄 길이는 100이며 주석을 보존한다. roundtrip semantic equality와 idempotence를 요구한다.

## 초안 — D24
format check는 rewrite 없이 diff/exit 상태, format write는 atomic file replacement와 원 encoding 오류 보존을 제안한다. syntax error 파일은 기본 쓰지 않는다. string/comment contents는 그대로 유지한다.

## 검증
긴 call/generic/type/match, trailing comment, doc comment attachment, END-sensitive return/else/operator, CRLF→LF offset refresh, stdin/stdout mode.
