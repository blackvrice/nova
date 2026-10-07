# NOVA-119 — Option<T> API 사양서

| 항목 | 값 |
|---|---|
| 대상 | Nova 0.1 |
| 작성일 | 2026-10-03 |
| 문서 상태 | Draft — 기존 결정은 유지, 추가 상세는 승인 대기 |
| 적용 Stage | B~E (print: A) |
| 근거 | [원본 NOVA-119](../../11_Standard_Library/NOVA-119_Option_T_API_사양서.md) |

이 문서는 원본을 대체하는 확정 사양이 아니다. 기존 확정 기준은 [CANONICAL](../CANONICAL.md)을 따르며, 새 의미·문법·API·정책은 [DECISIONS](../DECISIONS.md)의 승인이 필요하다. D01~D05 Lexer 상세는 [승인 기준](../ACCEPTED_LEXER.md)을 따른다. [전체 색인](../INDEX.md).

Copy Option<T>/Result<T,E>·T?·qualified 생성/none·문맥·match·const·private tagged ABI의 다음 최소 계약은 [P15 Draft](../OPTION_RESULT_STAGE_B_PROPOSAL.md), [51-production EBNF](../GRAMMAR_STAGE_B_OPTION_RESULT.ebnf), [제안 fixture](../option-result-proposal-fixtures/README.md)에 있다. 미승인/미구현이며 try·Move/Drop·사용자 Generic과 전체 D06/D08/D09/D10/D12/D15/D16/D23/D25/D30은 후속이다.

## Option<T>
Some(T)/None variant와 match를 사용하며 T?는 같은 타입이다. move payload의 ownership가 보존되고 niche는 internal optimization이다.

## API 초안 — D23
isSome/isNone는 Read; take unwrap→T는 None일 때 panic; take unwrapOr(take fallback)→T는 fallback도 source order로 평가한다. lazy fallback API는 Closure가 준비된 D 이후에 정의한다. implicit null/Some wrapping은 제외 제안이다.

## 검증
Some owner unwrap 이후 Option moved, None fallback Drop, Copy T copy 규칙, nested nullable, map API callback mode가 기존 payload를 중복 소비하지 않는지 검사한다.
