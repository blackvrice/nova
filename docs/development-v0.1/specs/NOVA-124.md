# NOVA-124 — Thread·Lock·Atomic API 사양서

| 항목 | 값 |
|---|---|
| 대상 | Nova 0.1 |
| 작성일 | 2026-10-03 |
| 문서 상태 | Draft — 기존 결정은 유지, 추가 상세는 승인 대기 |
| 적용 Stage | B~E (print: A) |
| 근거 | [원본 NOVA-124](../../11_Standard_Library/NOVA-124_Thread_Lock_Atomic_API_사양서.md) |

이 문서는 원본을 대체하는 확정 사양이 아니다. 기존 확정 기준은 [CANONICAL](../CANONICAL.md)을 따르며, 새 의미·문법·API·정책은 [DECISIONS](../DECISIONS.md)의 승인이 필요하다. [전체 색인](../INDEX.md).

## Thread API 초안 — D20/D23
spawn(take closure)→Result<ThreadHandle,ThreadError>, join(take handle)→Result<Unit,ThreadError>, Mutex<T>.lock()→Guard<T>, Atomic<intN> 명시 operations를 제안한다. Thread/atomic payload-return generic 확장은 signature 설계와 함께 검토한다.

## 수명/모드
closure가 borrowed local view를 캡처하여 owner보다 오래 살아서는 안 된다. Guard는 change 접근을 제공하며 Drop unlock, send/share predicate를 만족해야 한다. Mutex unlock이 panic cleanup을 보장한다는 뜻은 아니다; Abort다.

## 검증
take closure 이후 재사용 fail, thread view escape fail, guard return cleanup, atomic invalid order fail, runtime concurrent race corpus.
