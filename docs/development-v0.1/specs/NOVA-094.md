# NOVA-094 — Nova 타입→Backend 타입 Mapping 사양서

| 항목 | 값 |
|---|---|
| 대상 | Nova 0.1 |
| 작성일 | 2026-10-03 |
| 문서 상태 | Draft — 기존 결정은 유지, 추가 상세는 승인 대기 |
| 적용 Stage | A~E |
| 근거 | [원본 NOVA-094](../../09_Backend_Runtime/NOVA-094_Nova_타입_Backend_타입_Mapping_사양서.md) |

이 문서는 원본을 대체하는 확정 사양이 아니다. 기존 확정 기준은 [CANONICAL](../CANONICAL.md)을 따르며, 새 의미·문법·API·정책은 [DECISIONS](../DECISIONS.md)의 승인이 필요하다. [전체 색인](../INDEX.md).

## Mapping 계약
bool register i1, memory bool representation은 Layout; intN/uintN→iN; float/double→float/double; char→32-bit scalar storage를 D16에서 제안한다. string/Array/Enum/Class/view는 Layout/ABI 결과에 따라 lowered aggregate다.

## 금지 가정
Unit가 모든 ABI에서 1-byte인지, Never가 return register를 갖는지, pointer가 64-bit인지 가정하지 않는다. checked arithmetic는 overflow intrinsic/guard, bounds는 conditional abort로 표현한다.

## 검증
Target endian/align, unsigned comparison, sign extension vs zero extension, Unit return, Never call 뒤 unreachable, option niche와 일반 enum 같은 source 의미.
