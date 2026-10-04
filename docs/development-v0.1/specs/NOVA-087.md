# NOVA-087 — Constant Evaluation Engine 사양서

| 항목 | 값 |
|---|---|
| 대상 | Nova 0.1 |
| 작성일 | 2026-10-03 |
| 문서 상태 | Draft — 기존 결정은 유지, 추가 상세는 승인 대기 |
| 적용 Stage | B~E |
| 근거 | [원본 NOVA-087](../../08_MIR_Middleend/NOVA-087_Constant_Evaluation_Engine_사양서.md) |

이 문서는 원본을 대체하는 확정 사양이 아니다. 기존 확정 기준은 [CANONICAL](../CANONICAL.md)을 따르며, 새 의미·문법·API·정책은 [DECISIONS](../DECISIONS.md)의 승인이 필요하다. D01~D05 Lexer 상세는 [승인 기준](../ACCEPTED_LEXER.md)을 따른다. [전체 색인](../INDEX.md).

함수 내부 const와 제한된 상수 평가는 사용자 승인 [P05](../CONST_STAGE_B_PROPOSAL.md)와 [전용 EBNF](../GRAMMAR_STAGE_B_CONST.ebnf)가 우선한다. [구현·검증 기록](../CONST_IMPLEMENTATION.md). 전역 상수/const function/전체 D09는 Draft다.

## Const engine
typed const IR/MIR의 허용 subset을 interpreter로 평가한다. Nova target primitive semantics를 구현하고 host arithmetic을 그대로 호출해 wrap하거나 UB를 만들지 않는다.

## 입력/출력
ConstKey=(DefId,substitution,Target,semantics version) → ConstValue 또는 Source diagnostic. recursive evaluation stack은 dependency cycle chain을 남긴다. allocation/I/O/FFI는 D09 subset에서 거부한다.

## 검증
타입검사 숫자 모델과 동등, bounds/div-zero compile fail, large integer parsing, float exact rounding corpus, deterministic step budget, cycle diagnostics.
