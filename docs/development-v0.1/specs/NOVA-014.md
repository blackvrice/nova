# NOVA-014 — 문법 명세 및 EBNF 사양서

| 항목 | 값 |
|---|---|
| 대상 | Nova 0.1 |
| 작성일 | 2026-10-03 |
| 문서 상태 | Draft — 기존 결정은 유지, 추가 상세는 승인 대기 |
| 적용 Stage | A~B |
| 근거 | [원본 NOVA-014](../../01_Source_Syntax/NOVA-014_문법_명세_및_EBNF_사양서.md) |

이 문서는 원본을 대체하는 확정 사양이 아니다. 기존 확정 기준은 [CANONICAL](../CANONICAL.md)을 따르며, 새 의미·문법·API·정책은 [DECISIONS](../DECISIONS.md)의 승인이 필요하다. D01~D05 Lexer 상세는 [승인 기준](../ACCEPTED_LEXER.md)을 따른다. [전체 색인](../INDEX.md).

Stage A Parser 구문/복구는 사용자 승인 [P01](../PARSER_STAGE_A_PROPOSAL.md)과 [전용 EBNF](../GRAMMAR_STAGE_A.ebnf)가 우선한다. 나머지 추가 상세는 Draft다.

Stage B 가변 지역 변수·반복문 최소 부분은 사용자 진행 요청으로 승인한 [P04](../CONTROL_STAGE_B_PROPOSAL.md)와 [전용 EBNF](../GRAMMAR_STAGE_B_CONTROL.ebnf)가 우선한다. [구현·검증 기록](../CONTROL_IMPLEMENTATION.md). 전체 Stage B와 ownership/Drop 정책은 Draft다.

함수 내부 const와 제한된 상수 평가는 사용자 승인 [P05](../CONST_STAGE_B_PROPOSAL.md)와 [전용 EBNF](../GRAMMAR_STAGE_B_CONST.ebnf)가 우선한다. [구현·검증 기록](../CONST_IMPLEMENTATION.md). 전역 상수/const function/전체 D09는 Draft다.

## 문법 산출물
GRAMMAR.ebnf가 구체 Production을 담고 GRAMMAR_NOTES가 Stage와 결정 번호를 설명한다. lexer 원문과 END 정규화 후 Parser 문법을 분리한다. 현재 원본 NOVA-014에는 Production이 없으므로 새 grammar 전체는 승인 대기 초안이다.

## 문법 원칙
block은 {} 필수, 세미콜론 선택, 할당은 statement, comparison chaining 금지, generic call의 명시 타입 인수 금지다. func main()과 foreign 예제의 확정 철자는 보존한다.

## 모호성 제어
generic parameter는 선언, generic argument는 type 문맥에서만 파싱한다. type name expression으로 generic construction이 필요한 경우 D12를 먼저 결정한다. if/match/loop가 값인지 statement인지는 D08 초안에 명시한다.

## 검증
모든 nonterminal 정의/사용을 기계 검사하고 production별 최소 pass/fail을 CONFORMANCE에 연결한다. parser 구현이 grammar를 대신하지 않는다.
