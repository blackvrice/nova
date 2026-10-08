# NOVA-046 — Pattern 문법·Binding 사양서

| 항목 | 값 |
|---|---|
| 대상 | Nova 0.1 |
| 작성일 | 2026-10-03 |
| 문서 상태 | Draft — 기존 결정은 유지, 추가 상세는 승인 대기 |
| 적용 Stage | A~B |
| 근거 | [원본 NOVA-046](../../04_Functions_Control/NOVA-046_Pattern_문법_Binding_사양서.md) |

이 문서는 원본을 대체하는 확정 사양이 아니다. 기존 확정 기준은 [CANONICAL](../CANONICAL.md)을 따르며, 새 의미·문법·API·정책은 [DECISIONS](../DECISIONS.md)의 승인이 필요하다. D01~D05 Lexer 상세는 [승인 기준](../ACCEPTED_LEXER.md)을 따른다. [전체 색인](../INDEX.md).

중첩 Copy sum/tuple pattern·Unit/Copy Tuple statement match·recursive binder·matrix coverage·Source/MIR 검증은 [P24 Accepted](../NESTED_PATTERN_STAGE_B_PROPOSAL.md)·[60-production EBNF](../GRAMMAR_STAGE_B_NESTED_PATTERN.ebnf)·[수용 fixture](../nested-pattern-proposal-fixtures/README.md)와 [구현 기록](../NESTED_PATTERN_IMPLEMENTATION.md)을 따른다. 구현·검증 완료이며 P14/P15의 flat match와 P01~P23 승인 범위를 보존한다. guard/일반 literal/struct destructuring·Array·Move/loan/Drop·전체 D08/D10/D12/D16/D25/D30은 제외한다.

## Pattern 초안 — D08
wildcard _, binding, literal, tuple, qualified enum variant(payload), Some/None/Success/Error를 제안한다. repeated binding names는 오류이며 type checks는 scrutinee type을 따른다.

## Ownership
Read match는 payload loan을 만들고 take match는 전체 scrutinee owner를 소비한다. 사용자가 aggregate의 한 Move field만 빼오는 pattern은 partial move 금지로 거부한다. copy field projection은 이동이 아니다.

## 검증
tuple arity, variant payload type, duplicate binding, shadow scope, irrefutable let pattern, move field extraction 금지. guard의 binding 수명과 consumption은 D08/D10 승인 전에 명시적으로 제한한다.
