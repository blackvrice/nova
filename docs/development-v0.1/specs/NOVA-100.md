# NOVA-100 — 메모리 Allocator·OOM 정책 사양서

| 항목 | 값 |
|---|---|
| 대상 | Nova 0.1 |
| 작성일 | 2026-10-03 |
| 문서 상태 | Draft — 기존 결정은 유지, 추가 상세는 승인 대기 |
| 적용 Stage | A~E |
| 근거 | [원본 NOVA-100](../../09_Backend_Runtime/NOVA-100_메모리_Allocator_OOM_정책_사양서.md) |

이 문서는 원본을 대체하는 확정 사양이 아니다. 기존 확정 기준은 [CANONICAL](../CANONICAL.md)을 따르며, 새 의미·문법·API·정책은 [DECISIONS](../DECISIONS.md)의 승인이 필요하다. [전체 색인](../INDEX.md).

## Allocation 초안 — D18
allocator API는 size/align/checked capacity multiplication을 받는다. allocation failure는 Abort로 제안하며 일부 API의 recoverable OOM 여부는 별도 승인 없이는 추가하지 않는다.

## 불변 조건
allocation provenance를 유지하고 alloc/free pair 및 align을 맞춘다. Array 성장 시 이전 initialized values가 새 storage로 안전하게 이전된 뒤 old storage를 해제한다. zero-sized allocation policy는 Layout/Runtime 계약에 명시한다.

## 검증
fault-injected OOM, capacity overflow, over-alignment, realloc 실패, double-free 없음, host/Target pointer width 차이. OOM 테스트는 실제 시스템 메모리 고갈 대신 allocator injection으로 수행한다.
