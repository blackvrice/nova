# NOVA-030 — Type Alias·타입 정규화 사양서

| 항목 | 값 |
|---|---|
| 대상 | Nova 0.1 |
| 작성일 | 2026-10-03 |
| 문서 상태 | Draft — 기존 결정은 유지, 추가 상세는 승인 대기 |
| 적용 Stage | B~E |
| 근거 | [원본 NOVA-030](../../03_Types_Declarations/NOVA-030_Type_Alias_타입_정규화_사양서.md) |

이 문서는 원본을 대체하는 확정 사양이 아니다. 기존 확정 기준은 [CANONICAL](../CANONICAL.md)을 따르며, 새 의미·문법·API·정책은 [DECISIONS](../DECISIONS.md)의 승인이 필요하다. D01~D05 Lexer 상세는 [승인 기준](../ACCEPTED_LEXER.md)을 따른다. [전체 색인](../INDEX.md).

비제네릭 transparent type alias·type 위치·forward/import·cycle/자원·Source/MIR 검증은 [P21 Accepted](../ALIAS_STAGE_B_PROPOSAL.md), [55-production EBNF](../GRAMMAR_STAGE_B_ALIAS.ebnf), [수용 fixture](../alias-proposal-fixtures/README.md)를 따른다. [구현 기록](../ALIAS_IMPLEMENTATION.md)에 검증을 기록했다. type은 D01에서 이미 keyword이며 전체 D06/D10/D11/D12/D16/D30·generic alias/newtype·alias constructor/variant·API leak/export 정책은 승인하지 않았다.

Copy struct Read instance method·contextual self·member scope/visibility·receiver-first snapshot·named/default·Source/MIR 검증은 [P22 Accepted](../METHOD_STAGE_B_PROPOSAL.md), [58-production EBNF](../GRAMMAR_STAGE_B_METHOD.ebnf), [수용 fixture](../method-proposal-fixtures/README.md)와 [구현 기록](../METHOD_IMPLEMENTATION.md)을 따른다. 구현·검증 완료다. change/take·Move/borrow/Drop·init·overload·bound method·Enum method·Array와 전체 D06/D09/D10/D11/D12/D16/D25/D30은 승인하지 않았다.

Copy struct generated constructor 이름 인수·field mapping·source-order snapshot·const/default·Source/MIR 계약은 [P23 Draft](../STRUCT_NAMED_ARGUMENTS_STAGE_B_PROPOSAL.md)와 [수용 계획](../struct-named-arguments-proposal-fixtures/README.md)에 제안했다. 미승인·미구현이며 승인 P22 58-production EBNF를 변경 없이 재사용한다. explicit init·field default·Enum/sum named constructor·Array·일반 Move/Drop은 범위 밖이다.

## Type alias 초안 — D01/D12
type Name = Type 표기를 제안하지만 type은 원본 공식 keyword 표에 없으므로 승인 전 추가하지 않는다. alias는 nominal newtype가 아니라 동일 타입의 별칭이다.

## 정규화 계약
alias를 canonical type으로 확장하되 진단에 사용자 철자를 보존한다. alias dependency graph의 cycle은 chain과 각 선언 Span을 보여준다. Option Sugar와 Primitive alias를 같은 normalizer에서 처리한다.

## 검증
alias alias chain pass, A=B/B=A fail, alias를 통한 duplicate overload fail, private target type를 public alias로 노출하는 경우 visibility fail. alias 때문에 specialization을 중복 생성하지 않는다.
