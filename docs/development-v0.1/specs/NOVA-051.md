# NOVA-051 — 메모리·소유권 모델 사양서

| 항목 | 값 |
|---|---|
| 대상 | Nova 0.1 |
| 작성일 | 2026-10-03 |
| 문서 상태 | Draft — 기존 결정은 유지, 추가 상세는 승인 대기 |
| 적용 Stage | C |
| 근거 | [원본 NOVA-051](../../05_Ownership_Safety/NOVA-051_메모리_소유권_모델_사양서.md) |

이 문서는 원본을 대체하는 확정 사양이 아니다. 기존 확정 기준은 [CANONICAL](../CANONICAL.md)을 따르며, 새 의미·문법·API·정책은 [DECISIONS](../DECISIONS.md)의 승인이 필요하다. D01~D05 Lexer 상세는 [승인 기준](../ACCEPTED_LEXER.md)을 따른다. [전체 색인](../INDEX.md).

nominal Copy struct·위치 생성·type import·가변 field 경로·const·private layout/ABI와 자원 제한은 사용자 승인 [P12](../STRUCT_STAGE_B_PROPOSAL.md)와 [37-production EBNF](../GRAMMAR_STAGE_B_STRUCT.ebnf)를 따른다. [구현·검증 기록](../STRUCT_IMPLEMENTATION.md). String field·init/Drop·일반 Move/borrow와 전체 D06/D10/D12/D16/D30은 후속이다.

structural Copy Tuple·numeric selector subspan·혼합 가변 경로·const·private aggregate ABI와 자원 제한은 사용자 승인 [P13](../TUPLE_STAGE_B_PROPOSAL.md)와 [40-production EBNF](../GRAMMAR_STAGE_B_TUPLE.ebnf)를 따른다. [구현·검증 기록](../TUPLE_IMPLEMENTATION.md). Array/일반 Move element와 전체 D09/D10/D12/D16은 후속이다.

사용자 승인한 Copy Enum·qualified variant·Enum/Bool statement match·coverage/binder·const·private tagged ABI는 [P14](../ENUM_STAGE_B_PROPOSAL.md), [48-production EBNF](../GRAMMAR_STAGE_B_ENUM.ebnf), [수용 fixture](../enum-proposal-fixtures/README.md), [구현 기록](../ENUM_IMPLEMENTATION.md)을 따른다. 전체 D06/D08/D09/D10/D12/D16/D25/D30은 계속 Draft다.

## Ownership 기준
Move 타입은 하나의 owner를 가지며 Read는 owner를 소비하지 않는다. take 이후 원본은 재초기화까지 사용할 수 없다. Copy/Move 판정과 local initialization/assignment의 암묵 이동 여부는 D10 초안으로 구분한다.

## 초안 — D10
Copy는 primitive, Unit, 모든 component가 Copy인 tuple/struct/enum 및 사용자 drop 없는 타입에 한정한다. string/Array/Class/Shared는 기본 Move다. let y=x와 return x는 owned value 문맥에서 Move를 허용하고 호출의 take parameter는 take x를 명시한다.

## 검증
Copy x 재사용 pass, Move x 재사용 N4101, Read 호출 뒤 재사용 pass, take parameter에 modifier 누락 fail. Shared refcount increment는 암묵 복사가 아닌 clone API로 제안한다.
