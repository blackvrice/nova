# NOVA-102 — Shared·Weak Runtime 사양서

| 항목 | 값 |
|---|---|
| 대상 | Nova 0.1 |
| 작성일 | 2026-10-03 |
| 문서 상태 | Draft — 기존 결정은 유지, 추가 상세는 승인 대기 |
| 적용 Stage | A~E |
| 근거 | [원본 NOVA-102](../../09_Backend_Runtime/NOVA-102_Shared_Weak_Runtime_사양서.md) |

이 문서는 원본을 대체하는 확정 사양이 아니다. 기존 확정 기준은 [CANONICAL](../CANONICAL.md)을 따르며, 새 의미·문법·API·정책은 [DECISIONS](../DECISIONS.md)의 승인이 필요하다. D01~D05 Lexer 상세는 [승인 기준](../ACCEPTED_LEXER.md)을 따른다. [전체 색인](../INDEX.md).

## Control block
atomic strong/weak counts, payload lifetime flag, drop/free function과 allocation metadata를 가진다. strong 0은 payload dead, weak 0은 control block free다. implicit weak bookkeeping 여부는 내부 구현이며 수명 의미를 바꾸지 않는다.

## Atomic protocol 초안 — D20
clone overflow checked increment, final decrement acquire/release synchronization, upgrade CAS nonzero strong을 제안한다. publication/payload access와 refcount ordering을 별도 증명한다.

## 검증
upgrade vs final drop interleavings, exactly one payload drop, last weak free, overflow, concurrency stress/model checking. refcount 자체의 atomicity가 T data race를 해결한다고 주장하지 않는다.
