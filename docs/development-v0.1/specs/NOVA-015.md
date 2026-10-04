# NOVA-015 — 연산자 우선순위·결합 방향 사양서

| 항목 | 값 |
|---|---|
| 대상 | Nova 0.1 |
| 작성일 | 2026-10-03 |
| 문서 상태 | Draft — 기존 결정은 유지, 추가 상세는 승인 대기 |
| 적용 Stage | A~B |
| 근거 | [원본 NOVA-015](../../01_Source_Syntax/NOVA-015_연산자_우선순위_결합_방향_사양서.md) |

이 문서는 원본을 대체하는 확정 사양이 아니다. 기존 확정 기준은 [CANONICAL](../CANONICAL.md)을 따르며, 새 의미·문법·API·정책은 [DECISIONS](../DECISIONS.md)의 승인이 필요하다. D01~D05 Lexer 상세는 [승인 기준](../ACCEPTED_LEXER.md)을 따른다. [전체 색인](../INDEX.md).

Stage A Parser 구문/복구는 사용자 승인 [P01](../PARSER_STAGE_A_PROPOSAL.md)과 [전용 EBNF](../GRAMMAR_STAGE_A.ebnf)가 우선한다. 나머지 추가 상세는 Draft다.

Stage B 가변 지역 변수·반복문 최소 부분은 사용자 진행 요청으로 승인한 [P04](../CONTROL_STAGE_B_PROPOSAL.md)와 [전용 EBNF](../GRAMMAR_STAGE_B_CONTROL.ebnf)가 우선한다. [구현·검증 기록](../CONTROL_IMPLEMENTATION.md). 전체 Stage B와 ownership/Drop 정책은 Draft다.

함수 내부 const와 제한된 상수 평가는 사용자 승인 [P05](../CONST_STAGE_B_PROPOSAL.md)와 [전용 EBNF](../GRAMMAR_STAGE_B_CONST.ebnf)가 우선한다. [구현·검증 기록](../CONST_IMPLEMENTATION.md). 전역 상수/const function/전체 D09는 Draft다.

## 우선순위
낮은 순서: until/through → || → && → == != < <= > >= → + - → * / % → prefix ! - + try → postfix call/member/index/exists. cast as의 정확한 위치는 D03에서 postfix 층을 제안한다.

## 결합성 초안 — D03
산술/논리는 왼쪽 결합, prefix는 오른쪽 결합, range와 comparison은 비결합이다. a < b < c와 a until b through c는 괄호 없이 거부한다. =는 Pratt 표에 넣지 않는다.

## Short-circuit
&&/||의 오른쪽은 조건에 따라 평가하지 않는다. precedence가 evaluation order를 변경하지 않는다. 나머지 operand는 소스 순서로 평가하는 D08 규칙을 따른다.

## 검증
1+2*3=7, (1+2)*3=9, !a&&b, f().x[i] exists의 AST를 검사한다. false&&panic()이 panic을 실행하지 않는 runtime fixture를 둔다.
