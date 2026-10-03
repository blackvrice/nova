# NOVA-072 — Parser 구현 사양서

| 항목 | 값 |
|---|---|
| 대상 | Nova 0.1 |
| 작성일 | 2026-10-03 |
| 문서 상태 | Draft — 기존 결정은 유지, 추가 상세는 승인 대기 |
| 적용 Stage | A~E |
| 근거 | [원본 NOVA-072](../../07_Compiler_Frontend/NOVA-072_Parser_구현_사양서.md) |

이 문서는 원본을 대체하는 확정 사양이 아니다. 기존 확정 기준은 [CANONICAL](../CANONICAL.md)을 따르며, 새 의미·문법·API·정책은 [DECISIONS](../DECISIONS.md)의 승인이 필요하다. D01~D05 Lexer 상세는 [승인 기준](../ACCEPTED_LEXER.md)을 따른다. [전체 색인](../INDEX.md).

## Parser 인터페이스
parse(NormalizedTokens) → AST arena/root/diagnostics. declaration/statement/type은 Recursive Descent, expression은 Pratt며 source trivia를 type checker로 보내지 않는다.

## 상태
cursor, delimiter stack(open Span 포함), diagnostics, recovery budget, grammar version을 가진다. 함수 boundary가 return 분석과 delimiter 복구의 경계다. generic >는 type context에서만 닫힘으로 처리한다.

## 검증
GRAMMAR.ebnf의 모든 production positive/negative fixture, chained compare/assignment-expression 거부, truncated IDE input, token consumption progress. Recovery AST는 debug dump는 되지만 codegen 성공으로 표시하지 않는다.
