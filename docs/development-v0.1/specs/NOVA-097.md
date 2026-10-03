# NOVA-097 — Name Mangling·Symbol 사양서

| 항목 | 값 |
|---|---|
| 대상 | Nova 0.1 |
| 작성일 | 2026-10-03 |
| 문서 상태 | Draft — 기존 결정은 유지, 추가 상세는 승인 대기 |
| 적용 Stage | A~E |
| 근거 | [원본 NOVA-097](../../09_Backend_Runtime/NOVA-097_Name_Mangling_Symbol_사양서.md) |

이 문서는 원본을 대체하는 확정 사양이 아니다. 기존 확정 기준은 [CANONICAL](../CANONICAL.md)을 따르며, 새 의미·문법·API·정책은 [DECISIONS](../DECISIONS.md)의 승인이 필요하다. [전체 색인](../INDEX.md).

## Symbol 초안 — D16
Nova symbol은 language ABI version, package identity, module/item path, canonical signature/type args를 length-prefix encoding으로 포함한다. delimiter만 연결하여 이름 충돌을 만들지 않는다.

## 외부 이름
C export/import는 explicit symbol를 사용하며 Nova mangling을 붙이지 않는다. Entry wrapper C main과 user Nova main을 분리한다. Unicode identifiers는 UTF-8 byte length 또는 stable encoded spelling으로 처리한다.

## 검증
overload/generic/package revision 구별, alias canonical dedup, 동일 build의 symbol order, ambiguous concatenation 반례, external symbol duplicate linker diagnostic. ABI version 변경 시 rebuild한다.
