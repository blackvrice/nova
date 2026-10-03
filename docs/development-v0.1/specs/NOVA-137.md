# NOVA-137 — Lexer·Parser Snapshot 테스트 사양서

| 항목 | 값 |
|---|---|
| 대상 | Nova 0.1 |
| 작성일 | 2026-10-03 |
| 문서 상태 | Draft — 기존 결정은 유지, 추가 상세는 승인 대기 |
| 적용 Stage | 전 Stage |
| 근거 | [원본 NOVA-137](../../13_Testing_Release/NOVA-137_Lexer_Parser_Snapshot_테스트_사양서.md) |

이 문서는 원본을 대체하는 확정 사양이 아니다. 기존 확정 기준은 [CANONICAL](../CANONICAL.md)을 따르며, 새 의미·문법·API·정책은 [DECISIONS](../DECISIONS.md)의 승인이 필요하다. [전체 색인](../INDEX.md).

## Token/AST snapshot schema
token kind/start/end/raw spelling/trivia와 synthetic marker를 출력한다. AST는 node kind/child order/Span을 출력하고 memory pointer/random ID를 제거한다. source reconstruction 검증은 별도 assertion으로 수행한다.

## update 절차
snapshot 변경은 grammar/decision 이유와 diff 검토를 요구한다. 자동 accept를 CI 기본으로 사용하지 않는다. comment/END/call argument order를 normalize하여 숨기지 않는다.

## 검증
Unicode multiline strings, nested interpolation/comment, missing delimiter, keyword adjacency, deterministic serial/parallel output. GRAMMAR production마다 정상/오류 사례를 연결한다.
