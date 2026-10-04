# NOVA-083 — MIR 검증기 사양서

| 항목 | 값 |
|---|---|
| 대상 | Nova 0.1 |
| 작성일 | 2026-10-03 |
| 문서 상태 | Draft — 기존 결정은 유지, 추가 상세는 승인 대기 |
| 적용 Stage | A~E |
| 근거 | [원본 NOVA-083](../../08_MIR_Middleend/NOVA-083_MIR_검증기_사양서.md) |

이 문서는 원본을 대체하는 확정 사양이 아니다. 기존 확정 기준은 [CANONICAL](../CANONICAL.md)을 따르며, 새 의미·문법·API·정책은 [DECISIONS](../DECISIONS.md)의 승인이 필요하다. D01~D05 Lexer 상세는 [승인 기준](../ACCEPTED_LEXER.md)을 따른다. [전체 색인](../INDEX.md).

Stage A MIR은 원본 CFG/Place 기준과 P02의 평가 순서에 따라 구현했다. [MIR 구현 기록](../MIR_IMPLEMENTATION.md)은 현재 API/검증 경계이며 runtime/미래 Stage 정책의 승인이 아니다.

Stage B 가변 지역 변수·반복문 최소 부분은 사용자 진행 요청으로 승인한 [P04](../CONTROL_STAGE_B_PROPOSAL.md)와 [전용 EBNF](../GRAMMAR_STAGE_B_CONTROL.ebnf)가 우선한다. [구현·검증 기록](../CONTROL_IMPLEMENTATION.md). 전체 Stage B와 ownership/Drop 정책은 Draft다.

## Validator
pre-analysis와 post-drop/optimization validator를 분리한다. block terminator 존재, successor/local/type IDs 유효, projection type 일치, operand/rvalue type, return/call signature, switch tag 범위를 확인한다.

## 안전 입력
codegen 전에는 ErrorType, unresolved generic, uninitialized read, illegal Move/Loan, unelaborated Drop를 허용하지 않는다. early MIR에서는 아직 분석 전 정보가 있다는 점을 validator phase로 표현한다.

## 검증
deliberately malformed MIR corpus를 만들고 mismatch마다 stable internal code와 source origin을 보고한다. optimizer 전후 validator 모두 통과해야 한다. invalid CFG의 cycle 자체를 오류로 보지는 않는다.
