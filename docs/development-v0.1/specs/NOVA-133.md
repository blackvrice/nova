# NOVA-133 — Language Server·LSP 사양서

| 항목 | 값 |
|---|---|
| 대상 | Nova 0.1 |
| 작성일 | 2026-10-03 |
| 문서 상태 | Draft — 기존 결정은 유지, 추가 상세는 승인 대기 |
| 적용 Stage | E |
| 근거 | [원본 NOVA-133](../../12_Tooling_Packaging/NOVA-133_Language_Server_LSP_사양서.md) |

이 문서는 원본을 대체하는 확정 사양이 아니다. 기존 확정 기준은 [CANONICAL](../CANONICAL.md)을 따르며, 새 의미·문법·API·정책은 [DECISIONS](../DECISIONS.md)의 승인이 필요하다. [전체 색인](../INDEX.md).

## LSP 구현 계약 초안 — D24
document URI/version/text snapshot, incremental edits, diagnostics, completion, hover, definition/references, rename, formatting을 단계적으로 제공한다. compiler byte Span과 protocol position을 변환하며 UTF-16를 기본 상호운용 제안으로 둔다.

## 동시성
analysis 결과의 document version이 최신과 다르면 publish하지 않는다. cancellation은 incomplete result를 cache 성공으로 저장하지 않는다. batch compiler와 같은 semantic core를 사용한다.

## 검증
emoji/CRLF 위치, stale diagnostics race, rename visibility/collision, incomplete syntax no crash, multi-file edits invalidation. 실제 protocol version/capability는 도입 때 공식 명세를 확인한다.
