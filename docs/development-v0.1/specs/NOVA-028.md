# NOVA-028 — Nullable·Option·Result 타입 규칙

| 항목 | 값 |
|---|---|
| 대상 | Nova 0.1 |
| 작성일 | 2026-10-03 |
| 문서 상태 | Draft — 기존 결정은 유지, 추가 상세는 승인 대기 |
| 적용 Stage | B~E |
| 근거 | [원본 NOVA-028](../../03_Types_Declarations/NOVA-028_Nullable_Option_Result_타입_규칙.md) |

이 문서는 원본을 대체하는 확정 사양이 아니다. 기존 확정 기준은 [CANONICAL](../CANONICAL.md)을 따르며, 새 의미·문법·API·정책은 [DECISIONS](../DECISIONS.md)의 승인이 필요하다. D01~D05 Lexer 상세는 [승인 기준](../ACCEPTED_LEXER.md)을 따른다. [전체 색인](../INDEX.md).

사용자 승인한 Copy Option<T>/Result<T,E>·T?·qualified 생성/none·문맥·match·const·private tagged ABI는 [P15 Accepted](../OPTION_RESULT_STAGE_B_PROPOSAL.md), [51-production EBNF](../GRAMMAR_STAGE_B_OPTION_RESULT.ebnf), [수용 fixture](../option-result-proposal-fixtures/README.md), [구현 기록](../OPTION_RESULT_IMPLEMENTATION.md)을 따른다. try·Move/Drop·사용자 Generic과 전체 D06/D08/D09/D10/D12/D15/D16/D23/D25/D30은 후속이다.

## 확정 의미
T?는 Option<T>; variant는 Some/None다. Result<T,E>는 Success/Error이며 try는 Error를 조기 반환한다. Success(())는 Unit 성공이다. niche는 내부 최적화이므로 source 의미와 ABI 약속을 만들지 않는다.

## 상세 초안 — D08/D23
none은 기대 Option 타입이 필요하다. T→Option<T> 자동 wrapping은 제공하지 않고 Some(value)를 명시한다. x exists는 payload를 소비하지 않는 bool 테스트다. try는 enclosing Result의 Error 타입과 동일 E를 요구하며 자동 오류 변환은 첫 승인안에서 제외한다.

## 검증
Some/None match exhaustiveness, nested Option<Option<T>>, none inference fail, try Error cleanup, Move payload double use 거부. representation의 None가 항상 0이라는 가정은 금지한다.
