# NOVA-118 — Span<T>·ReadOnlySpan<T> 사양서

| 항목 | 값 |
|---|---|
| 대상 | Nova 0.1 |
| 작성일 | 2026-10-03 |
| 문서 상태 | Draft — 기존 결정은 유지, 추가 상세는 승인 대기 |
| 적용 Stage | B~E (print: A) |
| 근거 | [원본 NOVA-118](../../11_Standard_Library/NOVA-118_Span_T_ReadOnlySpan_T_사양서.md) |

이 문서는 원본을 대체하는 확정 사양이 아니다. 기존 확정 기준은 [CANONICAL](../CANONICAL.md)을 따르며, 새 의미·문법·API·정책은 [DECISIONS](../DECISIONS.md)의 승인이 필요하다. [전체 색인](../INDEX.md).

## View 의미
ReadOnlySpan<T>는 read view, Span<T>는 change view다. owner보다 오래 살 수 없으며 ptr+length ABI와 region origin을 갖는다. Compiler source Span과 별개의 언어 타입이다.

## API 초안 — D10/D23
length, checked slice(start,end), readAt/changeAt, splitAt(index)→두 비중첩 view를 제안한다. mutable split은 원 view를 소비/재빌림해 두 child view가 live 동안 parent access를 금지한다.

## 검증
0/len split, index>len error, child overlap 없음, owner Drop/Move 금지, parent 재사용 시점, dynamic-index 임의 disjoint 주장 거부. ABI pointer pair가 lifetime 검사를 대체하지 않는다.
