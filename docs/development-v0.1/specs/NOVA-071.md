# NOVA-071 — Lexer 구현 사양서

| 항목 | 값 |
|---|---|
| 대상 | Nova 0.1 |
| 작성일 | 2026-10-03 |
| 문서 상태 | Draft — 기존 결정은 유지, 추가 상세는 승인 대기 |
| 적용 Stage | A~E |
| 근거 | [원본 NOVA-071](../../07_Compiler_Frontend/NOVA-071_Lexer_구현_사양서.md) |

이 문서는 원본을 대체하는 확정 사양이 아니다. 기존 확정 기준은 [CANONICAL](../CANONICAL.md)을 따르며, 새 의미·문법·API·정책은 [DECISIONS](../DECISIONS.md)의 승인이 필요하다. [전체 색인](../INDEX.md).

## Lexer 인터페이스
lex(SourceFile) → RawTokens,Trivia,Diagnostics. 원문 byte 범위가 token/trivia에서 겹치거나 누락되지 않으며 EOF는 length 위치의 빈 Span이다.

## 실행 구조
cursor는 UTF-8 char boundary를 지킨다. identifier/number/operator scanning은 최장 일치, string/comment/interpolation은 mode stack. 잘못된 char는 최소 한 scalar를 소비한 Error token이다. number range는 type 단계에서 검사한다.

## 복잡도
각 byte를 상수 횟수 방문하여 O(n)을 목표로 한다. 매 identifier마다 suffix 전체 복사, nested comment 재검색, 반복 문자열 concat을 피한다. depth limit은 D30 진단으로 처리한다.

## 검증
LEXICAL/END_RULES fixture, source reconstruction, Unicode boundary, fuzz timeout/crash, 동일 raw input의 stable dump.
