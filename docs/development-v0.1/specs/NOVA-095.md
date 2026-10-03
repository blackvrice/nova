# NOVA-095 — Backend 타입 레이아웃·정렬·Niche 구현서

| 항목 | 값 |
|---|---|
| 대상 | Nova 0.1 |
| 작성일 | 2026-10-03 |
| 문서 상태 | Draft — 기존 결정은 유지, 추가 상세는 승인 대기 |
| 적용 Stage | A~E |
| 근거 | [원본 NOVA-095](../../09_Backend_Runtime/NOVA-095_Backend_타입_레이아웃_정렬_Niche_구현서.md) |

이 문서는 원본을 대체하는 확정 사양이 아니다. 기존 확정 기준은 [CANONICAL](../CANONICAL.md)을 따르며, 새 의미·문법·API·정책은 [DECISIONS](../DECISIONS.md)의 승인이 필요하다. D01~D05 Lexer 상세는 [승인 기준](../ACCEPTED_LEXER.md)을 따른다. [전체 색인](../INDEX.md).

## Layout lowering
core Layout를 LLVM DataLayout와 대조한다. field offsets/align/padding/tag/payload를 하나의 Target query 결과로 계산하고 backend의 임의 재배치를 금지한다.

## Niche
valid bit pattern 밖의 representation만 사용한다. optimization enable/disable이 discriminant test/Drop 의미를 바꾸지 않아야 한다. foreign representation은 explicit ABI-safe mapping으로 제한한다.

## 검증
sizeof/alignof/offset golden, LLVM structural type padding, over-aligned allocation, zero-sized element, enum maximum payload, C roundtrip harness.
