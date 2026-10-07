# NOVA-027 — Struct·Class·Enum 선언 사양서

| 항목 | 값 |
|---|---|
| 대상 | Nova 0.1 |
| 작성일 | 2026-10-03 |
| 문서 상태 | Draft — 기존 결정은 유지, 추가 상세는 승인 대기 |
| 적용 Stage | B~E |
| 근거 | [원본 NOVA-027](../../03_Types_Declarations/NOVA-027_Struct_Class_Enum_선언_사양서.md) |

이 문서는 원본을 대체하는 확정 사양이 아니다. 기존 확정 기준은 [CANONICAL](../CANONICAL.md)을 따르며, 새 의미·문법·API·정책은 [DECISIONS](../DECISIONS.md)의 승인이 필요하다. D01~D05 Lexer 상세는 [승인 기준](../ACCEPTED_LEXER.md)을 따른다. [전체 색인](../INDEX.md).

nominal Copy struct·위치 생성·type import·가변 field 경로·const·private layout/ABI와 자원 제한은 사용자 승인 [P12](../STRUCT_STAGE_B_PROPOSAL.md)와 [37-production EBNF](../GRAMMAR_STAGE_B_STRUCT.ebnf)를 따른다. [구현·검증 기록](../STRUCT_IMPLEMENTATION.md). String field·init/Drop·일반 Move/borrow와 전체 D06/D10/D12/D16/D30은 후속이다.

structural Copy Tuple·numeric selector subspan·혼합 가변 경로·const·private aggregate ABI와 자원 제한은 사용자 승인 [P13](../TUPLE_STAGE_B_PROPOSAL.md)와 [40-production EBNF](../GRAMMAR_STAGE_B_TUPLE.ebnf)를 따른다. [구현·검증 기록](../TUPLE_IMPLEMENTATION.md). Array/일반 Move element와 전체 D09/D10/D12/D16은 후속이다.

사용자 승인한 Copy Enum·qualified variant·Enum/Bool statement match·coverage/binder·const·private tagged ABI는 [P14](../ENUM_STAGE_B_PROPOSAL.md), [48-production EBNF](../GRAMMAR_STAGE_B_ENUM.ebnf), [수용 fixture](../enum-proposal-fixtures/README.md), [구현 기록](../ENUM_IMPLEMENTATION.md)을 따른다. 전체 D06/D08/D09/D10/D12/D16/D25/D30은 계속 Draft다.

## 값/Handle 구분
Struct는 값 의미, Class는 정체성을 가진 owned handle, Enum은 하나의 활성 variant와 payload다. 모든 필드는 let/var를 명시하고 직접 재귀 값 포함은 금지한다. Option/Result는 특별 null sentinel가 아닌 일반 Enum 의미다.

## 선언 초안 — D12
struct/class/enum 이름 뒤 generic parameter와 implements 목록, {} member body를 제안한다. Enum variant는 이름과 위치 payload tuple로 시작하며 custom discriminant/field syntax는 보류한다. 직접 순환 layout graph는 size query에서 오류다.

## Copy/Drop
Struct Copy 여부는 모든 field의 Copy와 사용자 drop 부재에 따라 결정하는 D10 제안이다. Class handle 복사는 금지하고 take로 이전한다. inactive Enum payload를 읽거나 Drop하지 않는다.

## 검증
recursive-by-value fail, handle indirection pass, immutable field reassignment fail, enum payload Drop count 및 zero-sized aggregate를 다룬다.
