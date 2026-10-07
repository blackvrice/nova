# NOVA-049 — try·Result 전파 Lowering 사양서

| 항목 | 값 |
|---|---|
| 대상 | Nova 0.1 |
| 작성일 | 2026-10-03 |
| 문서 상태 | Draft — 기존 결정은 유지, 추가 상세는 승인 대기 |
| 적용 Stage | A~B |
| 근거 | [원본 NOVA-049](../../04_Functions_Control/NOVA-049_try_Result_전파_Lowering_사양서.md) |

이 문서는 원본을 대체하는 확정 사양이 아니다. 기존 확정 기준은 [CANONICAL](../CANONICAL.md)을 따르며, 새 의미·문법·API·정책은 [DECISIONS](../DECISIONS.md)의 승인이 필요하다. D01~D05 Lexer 상세는 [승인 기준](../ACCEPTED_LEXER.md)을 따른다. [전체 색인](../INDEX.md).

사용자 승인한 Copy prefix try·Result Error 조기 반환·operand 문맥 격리·정확한 E·const 금지·Source/CFG 검증은 [P16 Accepted](../TRY_STAGE_B_PROPOSAL.md), [51-production EBNF](../GRAMMAR_STAGE_B_TRY.ebnf), [수용 fixture](../try-proposal-fixtures/README.md), [구현 기록](../TRY_IMPLEMENTATION.md)을 따른다. Option try·error conversion·일반 Move/Drop와 전체 D08/D09/D10/D12/D16/D23/D25는 후속이다.

사용자 승인한 함수 이름 인수·parameter mapping·source-order snapshot/try·진단 계약은 [P17 Accepted](../NAMED_ARGUMENTS_STAGE_B_PROPOSAL.md), [52-production EBNF](../GRAMMAR_STAGE_B_NAMED_ARGUMENTS.ebnf), [수용 fixture](../named-arguments-proposal-fixtures/README.md), [구현 기록](../NAMED_ARGUMENTS_IMPLEMENTATION.md)을 따른다. 구현·검증 완료이며 기본 인수·overload·named constructor와 전체 D11/D16/D25/D30 승인이 아니다.

## 확정 Result 의미
try expression은 Result의 Success payload를 산출하고 Error payload를 enclosing function의 Error return으로 보낸다. Success(())를 Unit 성공으로 쓴다. throw/unwind를 도입하지 않는다.

## 초안 — D08
오류 타입 E가 enclosing return Result의 E와 정확히 같아야 한다. 변환이 필요하면 사용자가 explicit wrapper를 작성한다. try는 가장 가까운 lambda/function 경계를 사용하며 일반 Unit 함수에서는 거부한다.

## MIR
Result temporary → discriminant switch → Success extraction 또는 Error construction/return → local cleanup. 전체 result를 한 번만 consume하고 inactive payload를 Drop하지 않는다.

## 검증
nested try, success side effect, Error early return의 local 역순 Drop, E mismatch fail, try outside Result fail.
