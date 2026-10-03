# NOVA-016 — Parser·AST 사양서

| 항목 | 값 |
|---|---|
| 대상 | Nova 0.1 |
| 작성일 | 2026-10-03 |
| 문서 상태 | Draft — 기존 결정은 유지, 추가 상세는 승인 대기 |
| 적용 Stage | A~B |
| 근거 | [원본 NOVA-016](../../01_Source_Syntax/NOVA-016_Parser_AST_사양서.md) |

이 문서는 원본을 대체하는 확정 사양이 아니다. 기존 확정 기준은 [CANONICAL](../CANONICAL.md)을 따르며, 새 의미·문법·API·정책은 [DECISIONS](../DECISIONS.md)의 승인이 필요하다. [전체 색인](../INDEX.md).

## Parser/AST 계약
정규화 token에서 소스 구조 AST를 만든다. declaration/statement/type은 Recursive Descent, expression은 Pratt다. Arena AstNodeId, Node Span, delimiter token 위치, trivia anchor를 보존한다. 타입과 DefId는 AST 필드가 아니다.

## 결과
ParseResult는 arena, root, diagnostics, recovered flag를 반환한다. 유효 프로그램과 복구 프로그램 모두 dump 가능하지만 오류 AST를 의미상 성공으로 표시하지 않는다. EOF에서 cursor가 진행하지 않는 반복을 금지한다.

## 검증
func/call/let/if/return 최소 노드와 production별 Span, missing delimiter synthetic Span, ErrorNode 방문을 검사한다. grammar version과 parser snapshot version을 연결한다.
