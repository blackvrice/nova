# NOVA-117 — Array<T> API·메모리 모델 사양서

| 항목 | 값 |
|---|---|
| 대상 | Nova 0.1 |
| 작성일 | 2026-10-03 |
| 문서 상태 | Draft — 기존 결정은 유지, 추가 상세는 승인 대기 |
| 적용 Stage | B~E (print: A) |
| 근거 | [원본 NOVA-117](../../11_Standard_Library/NOVA-117_Array_T_API_메모리_모델_사양서.md) |

이 문서는 원본을 대체하는 확정 사양이 아니다. 기존 확정 기준은 [CANONICAL](../CANONICAL.md)을 따르며, 새 의미·문법·API·정책은 [DECISIONS](../DECISIONS.md)의 승인이 필요하다. D01~D05 Lexer 상세는 [승인 기준](../ACCEPTED_LEXER.md)을 따른다. [전체 색인](../INDEX.md).

## Array<T> 메모리
ptr/len/capacity와 initialized prefix를 관리한다. 0≤len≤cap, element size×cap overflow 없음, initialized 원소만 읽고 정확히 한 번 Drop한다.

## API 초안 — D23
length/capacity, push(take value), pop()→Option<T>, reserve(additional), readAt(index)→view T, changeAt(index)→change view T, asReadOnlySpan/asSpan을 제안한다. indexing bounds 실패는 panic, checked get은 Option view다.

## 검증
growth allocation, zero-sized T, Move T reallocation, pop ownership, reserve overflow, view live 중 push 거부. List는 Array와의 관계가 승인될 때까지 별도 중복 storage를 구현하지 않는다.
