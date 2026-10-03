# NOVA-121 — List<T>·Map<K,V> 사양서

| 항목 | 값 |
|---|---|
| 대상 | Nova 0.1 |
| 작성일 | 2026-10-03 |
| 문서 상태 | Draft — 기존 결정은 유지, 추가 상세는 승인 대기 |
| 적용 Stage | B~E (print: A) |
| 근거 | [원본 NOVA-121](../../11_Standard_Library/NOVA-121_List_T_Map_KV_사양서.md) |

이 문서는 원본을 대체하는 확정 사양이 아니다. 기존 확정 기준은 [CANONICAL](../CANONICAL.md)을 따르며, 새 의미·문법·API·정책은 [DECISIONS](../DECISIONS.md)의 승인이 필요하다. [전체 색인](../INDEX.md).

## List/Map 초안 — D23
Array가 이미 growable owner이므로 List<T>는 Array의 별칭/편의 API로 제안하고 별도 같은 buffer type를 중복 구현하지 않는다. Map<K,V>는 hash-based owning container 후보이며 Hash/Equality generic constraint는 D15와 함께 검토한다.

## 계약
iteration order는 명시 보장 없으면 unspecified; compiler 결정적 output에 Map 순서를 그대로 쓰지 않는다. insert는 old value ownership를 반환하는 Option<V>, remove도 Option<V>를 제안한다. iterator live 중 structural mutation을 금지한다.

## 검증
collision equality, replacing Drop, key/value Move, rehash with views, adversarial hash input, deterministic compiler serialization sorting.
