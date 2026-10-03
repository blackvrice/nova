# NOVA-108 — C++ Bridge·Opaque Handle 사양서

| 항목 | 값 |
|---|---|
| 대상 | Nova 0.1 |
| 작성일 | 2026-10-03 |
| 문서 상태 | Draft — 기존 결정은 유지, 추가 상세는 승인 대기 |
| 적용 Stage | A~E |
| 근거 | [원본 NOVA-108](../../10_FFI/NOVA-108_C_Bridge_Opaque_Handle_사양서.md) |

이 문서는 원본을 대체하는 확정 사양이 아니다. 기존 확정 기준은 [CANONICAL](../CANONICAL.md)을 따르며, 새 의미·문법·API·정책은 [DECISIONS](../DECISIONS.md)의 승인이 필요하다. D01~D05 Lexer 상세는 [승인 기준](../ACCEPTED_LEXER.md)을 따른다. [전체 색인](../INDEX.md).

## Opaque handle wrapper
raw pointer와 safe owned handle를 구분한다. constructor 성공 시 non-null handle를 owner에 저장, read operation은 borrow, transfer operation은 take, drop은 matched release다.

## 초안 — D17
library lifetime은 모든 handle/callback보다 길어야 한다. null은 Option/Result wrapper로 처리한다. thread-affinity/thread-safety annotation은 compiler predicate에 연결하며 unknown은 conservative다.

## 검증
double release/use after release/null handle 거부, release allocator mismatch 방지, callback 재진입, library unload-before-handle fail. foreign pointer가 가리키는 memory를 Nova allocator로 해제하지 않는다.
