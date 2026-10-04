# NOVA-043 — if·while·for·loop 사양서

| 항목 | 값 |
|---|---|
| 대상 | Nova 0.1 |
| 작성일 | 2026-10-03 |
| 문서 상태 | Draft — 기존 결정은 유지, 추가 상세는 승인 대기 |
| 적용 Stage | A~B |
| 근거 | [원본 NOVA-043](../../04_Functions_Control/NOVA-043_if_while_for_loop_사양서.md) |

이 문서는 원본을 대체하는 확정 사양이 아니다. 기존 확정 기준은 [CANONICAL](../CANONICAL.md)을 따르며, 새 의미·문법·API·정책은 [DECISIONS](../DECISIONS.md)의 승인이 필요하다. D01~D05 Lexer 상세는 [승인 기준](../ACCEPTED_LEXER.md)을 따른다. [전체 색인](../INDEX.md).

Stage A Parser 구문/복구는 사용자 승인 [P01](../PARSER_STAGE_A_PROPOSAL.md)과 [전용 EBNF](../GRAMMAR_STAGE_A.ebnf)가 우선한다. 나머지 추가 상세는 Draft다.

Stage A 단일 파일 의미 검사는 사용자 승인 [P02](../SEMANTICS_STAGE_A_PROPOSAL.md)가 우선한다. runtime/전체 타입/미래 Stage의 추가 상세는 Draft다.

Stage B 가변 지역 변수·반복문 최소 부분은 사용자 진행 요청으로 승인한 [P04](../CONTROL_STAGE_B_PROPOSAL.md)와 [전용 EBNF](../GRAMMAR_STAGE_B_CONTROL.ebnf)가 우선한다. [구현·검증 기록](../CONTROL_IMPLEMENTATION.md). 전체 Stage B와 ownership/Drop 정책은 Draft다.

## 제어 흐름 초안 — D08
if/while 조건은 bool만 허용한다. if/while/for/loop는 statement이며 value expression으로 사용하지 않는다. for binding in expression { ... }는 iterable/range를 한 번 평가한다.

## 평가/Scope
if는 선택 arm만 실행한다. while는 매 반복 조건을 다시 평가한다. loop는 break까지 반복한다. loop body scope의 local은 각 iteration 끝에 Drop한다. continue도 그 iteration cleanup을 거친다.

## Range
until은 끝 제외, through는 포함이다. iterable protocol은 Associated Type 없이 D15의 명시 generic interface 계약으로 제안한다.

## 검증
integer condition fail, else-if chain, zero iteration, continue cleanup, inclusive 최댓값 종료에서 overflow 없이 끝나는 range를 검사한다.
