# NOVA-082 — HIR→MIR Lowering 사양서

| 항목 | 값 |
|---|---|
| 대상 | Nova 0.1 |
| 작성일 | 2026-10-03 |
| 문서 상태 | Draft — 기존 결정은 유지, 추가 상세는 승인 대기 |
| 적용 Stage | A~E |
| 근거 | [원본 NOVA-082](../../08_MIR_Middleend/NOVA-082_HIR_MIR_Lowering_사양서.md) |

이 문서는 원본을 대체하는 확정 사양이 아니다. 기존 확정 기준은 [CANONICAL](../CANONICAL.md)을 따르며, 새 의미·문법·API·정책은 [DECISIONS](../DECISIONS.md)의 승인이 필요하다. D01~D05 Lexer 상세는 [승인 기준](../ACCEPTED_LEXER.md)을 따른다. [전체 색인](../INDEX.md).

Stage A MIR은 원본 CFG/Place 기준과 P02의 평가 순서에 따라 구현했다. [MIR 구현 기록](../MIR_IMPLEMENTATION.md)은 현재 API/검증 경계이며 runtime/미래 Stage 정책의 승인이 아니다.

Stage B 가변 지역 변수·반복문 최소 부분은 사용자 진행 요청으로 승인한 [P04](../CONTROL_STAGE_B_PROPOSAL.md)와 [전용 EBNF](../GRAMMAR_STAGE_B_CONTROL.ebnf)가 우선한다. [구현·검증 기록](../CONTROL_IMPLEMENTATION.md). 전체 Stage B와 ownership/Drop 정책은 Draft다.

## HIR→MIR
typed HIR/call resolution/layout-independent type info를 입력으로 받는다. Error body는 lowering 성공이 아니며 backend unit에서 제외한다. temporary/local IDs는 source traversal에 따라 결정적으로 할당한다.

## lowering 계약
값을 한 번 평가해 temporary에 저장하고 source-order operation을 이어간다. if/loop/short-circuit은 CFG, match/try는 tag Switch, named/default arguments는 평가 temp와 전달 order를 분리한다.

## 검증
side-effect function trace, assignment RHS-before-old-drop, early return cleanup, loop continue target, view lifetime source origin. MIR dump가 source 의미를 역추적할 수 있어야 한다.
