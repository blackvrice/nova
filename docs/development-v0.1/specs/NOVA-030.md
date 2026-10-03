# NOVA-030 — Type Alias·타입 정규화 사양서

| 항목 | 값 |
|---|---|
| 대상 | Nova 0.1 |
| 작성일 | 2026-10-03 |
| 문서 상태 | Draft — 기존 결정은 유지, 추가 상세는 승인 대기 |
| 적용 Stage | B~E |
| 근거 | [원본 NOVA-030](../../03_Types_Declarations/NOVA-030_Type_Alias_타입_정규화_사양서.md) |

이 문서는 원본을 대체하는 확정 사양이 아니다. 기존 확정 기준은 [CANONICAL](../CANONICAL.md)을 따르며, 새 의미·문법·API·정책은 [DECISIONS](../DECISIONS.md)의 승인이 필요하다. [전체 색인](../INDEX.md).

## Type alias 초안 — D01/D12
type Name = Type 표기를 제안하지만 type은 원본 공식 keyword 표에 없으므로 승인 전 추가하지 않는다. alias는 nominal newtype가 아니라 동일 타입의 별칭이다.

## 정규화 계약
alias를 canonical type으로 확장하되 진단에 사용자 철자를 보존한다. alias dependency graph의 cycle은 chain과 각 선언 Span을 보여준다. Option Sugar와 Primitive alias를 같은 normalizer에서 처리한다.

## 검증
alias alias chain pass, A=B/B=A fail, alias를 통한 duplicate overload fail, private target type를 public alias로 노출하는 경우 visibility fail. alias 때문에 specialization을 중복 생성하지 않는다.
