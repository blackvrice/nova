# NOVA-033 — 타입 레이아웃·정렬·Niche 사양서

| 항목 | 값 |
|---|---|
| 대상 | Nova 0.1 |
| 작성일 | 2026-10-03 |
| 문서 상태 | Draft — 기존 결정은 유지, 추가 상세는 승인 대기 |
| 적용 Stage | B~E |
| 근거 | [원본 NOVA-033](../../03_Types_Declarations/NOVA-033_타입_레이아웃_정렬_Niche_사양서.md) |

이 문서는 원본을 대체하는 확정 사양이 아니다. 기존 확정 기준은 [CANONICAL](../CANONICAL.md)을 따르며, 새 의미·문법·API·정책은 [DECISIONS](../DECISIONS.md)의 승인이 필요하다. D01~D05 Lexer 상세는 [승인 기준](../ACCEPTED_LEXER.md)을 따른다. [전체 색인](../INDEX.md).

nominal Copy struct·위치 생성·type import·가변 field 경로·const·private layout/ABI와 자원 제한은 사용자 승인 [P12](../STRUCT_STAGE_B_PROPOSAL.md)와 [37-production EBNF](../GRAMMAR_STAGE_B_STRUCT.ebnf)를 따른다. [구현·검증 기록](../STRUCT_IMPLEMENTATION.md). String field·init/Drop·일반 Move/borrow와 전체 D06/D10/D12/D16/D30은 후속이다.

## Target layout 계약
Layout(TypeId,TargetSpec) → size, align, field offsets, variant layout, valid-bit-patterns/niche를 반환한다. pointer width/endian/aggregate ABI는 Target 입력이며 host에서 추정하지 않는다.

## 초안 — D16
Primitive size는 fixed-width, bool/char memory representation과 aggregate field packing은 ABI 문서에 명시한다. 기본 aggregate는 선언 field 순서, alignment padding을 제안한다. C compatible representation은 explicit foreign wrapper에서만 약속한다.

## Niche
Option의 invalid-bit-pattern 활용은 내부 layout 최적화다. public serialization/FFI는 niche를 그대로 노출하지 않는다. zero-sized 값의 storage 및 주소 identity는 D16 대상이다.

## 검증
alignment, padding, nested aggregate, recursive layout fail, Option handle와 payload enum, Target별 golden layout을 검사한다.
