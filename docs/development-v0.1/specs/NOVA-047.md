# NOVA-047 — Match 완전성·도달 불가 Arm 분석서

| 항목 | 값 |
|---|---|
| 대상 | Nova 0.1 |
| 작성일 | 2026-10-03 |
| 문서 상태 | Draft — 기존 결정은 유지, 추가 상세는 승인 대기 |
| 적용 Stage | A~B |
| 근거 | [원본 NOVA-047](../../04_Functions_Control/NOVA-047_Match_완전성_도달_불가_Arm_분석서.md) |

이 문서는 원본을 대체하는 확정 사양이 아니다. 기존 확정 기준은 [CANONICAL](../CANONICAL.md)을 따르며, 새 의미·문법·API·정책은 [DECISIONS](../DECISIONS.md)의 승인이 필요하다. D01~D05 Lexer 상세는 [승인 기준](../ACCEPTED_LEXER.md)을 따른다. [전체 색인](../INDEX.md).

사용자 승인한 Copy Enum·qualified variant·Enum/Bool statement match·coverage/binder·const·private tagged ABI는 [P14](../ENUM_STAGE_B_PROPOSAL.md), [48-production EBNF](../GRAMMAR_STAGE_B_ENUM.ebnf), [수용 fixture](../enum-proposal-fixtures/README.md), [구현 기록](../ENUM_IMPLEMENTATION.md)을 따른다. 전체 D06/D08/D09/D10/D12/D16/D25/D30은 계속 Draft다.

사용자 승인한 Copy Option<T>/Result<T,E>·T?·qualified 생성/none·문맥·match·const·private tagged ABI는 [P15 Accepted](../OPTION_RESULT_STAGE_B_PROPOSAL.md), [51-production EBNF](../GRAMMAR_STAGE_B_OPTION_RESULT.ebnf), [수용 fixture](../option-result-proposal-fixtures/README.md), [구현 기록](../OPTION_RESULT_IMPLEMENTATION.md)을 따른다. try·Move/Drop·사용자 Generic과 전체 D06/D08/D09/D10/D12/D15/D16/D23/D25/D30은 후속이다.

사용자 승인한 Copy prefix try·Result Error 조기 반환·operand 문맥 격리·정확한 E·const 금지·Source/CFG 검증은 [P16 Accepted](../TRY_STAGE_B_PROPOSAL.md), [51-production EBNF](../GRAMMAR_STAGE_B_TRY.ebnf), [수용 fixture](../try-proposal-fixtures/README.md), [구현 기록](../TRY_IMPLEMENTATION.md)을 따른다. Option try·error conversion·일반 Move/Drop와 전체 D08/D09/D10/D12/D16/D23/D25는 후속이다.

중첩 Copy sum/tuple pattern·Unit/Copy Tuple statement match·recursive binder·matrix coverage·Source/MIR 검증은 [P24 Draft](../NESTED_PATTERN_STAGE_B_PROPOSAL.md)·[60-production 제안 EBNF](../GRAMMAR_STAGE_B_NESTED_PATTERN.ebnf)·[수용 계획](../nested-pattern-proposal-fixtures/README.md)에 제안했다. 미승인·미구현이며 P14/P15의 flat match와 P01~P23 승인 범위를 보존한다. guard/일반 literal/struct destructuring·Array·Move/loan/Drop·전체 D08/D10/D12/D16/D25/D30은 제외한다.

## Exhaustiveness
bool/Enum/Option/Result/tuple 조합은 constructor coverage로 검사한다. integer/string의 임의 값 공간에는 wildcard가 필요하다. guard가 있는 arm은 exhaustive coverage 증명으로 사용하지 않는 초안을 제안한다.

## 도달성
앞 arm이 완전히 덮는 뒤 arm은 unreachable 진단이며 severity는 D25에서 결정한다. declaration/source arm 순서를 보존한다. 누락 경우는 가능한 최소 pattern 예시로 보여준다.

## 검증
bool 한 arm 누락 fail, Option Some만 fail, wildcard 후 arm unreachable, guarded wildcard만 존재할 때 incomplete, nested enum/tuple coverage를 검사한다. algorithm recursion에는 결정적인 resource budget을 둔다.
