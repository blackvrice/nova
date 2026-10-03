# NOVA-103 — Thread·Atomic·Lock Runtime 사양서

| 항목 | 값 |
|---|---|
| 대상 | Nova 0.1 |
| 작성일 | 2026-10-03 |
| 문서 상태 | Draft — 기존 결정은 유지, 추가 상세는 승인 대기 |
| 적용 Stage | A~E |
| 근거 | [원본 NOVA-103](../../09_Backend_Runtime/NOVA-103_Thread_Atomic_Lock_Runtime_사양서.md) |

이 문서는 원본을 대체하는 확정 사양이 아니다. 기존 확정 기준은 [CANONICAL](../CANONICAL.md)을 따르며, 새 의미·문법·API·정책은 [DECISIONS](../DECISIONS.md)의 승인이 필요하다. [전체 색인](../INDEX.md).

## Thread/Lock/Atomic runtime 초안 — D20
thread spawn/join, mutex lock guard, fixed-width atomic operations를 platform layer 뒤에 제공한다. join 결과와 worker panic Abort는 별개의 failure class다.

## 안전 계약
lock guard는 scope Drop로 unlock하며 lifetime은 mutex보다 짧다. Mutex<T> 공유는 T predicate를 검사한다. atomic order는 Relaxed/Acquire/Release/AcqRel/SeqCst를 명시하고 불가능한 load/store order는 거부한다.

## 검증
guard early return unlock, deadlock timeout test, worker ownership transfer, relaxed counter vs publication 예제, OS resource close. async runtime/condition-free spin을 기본 구현으로 추가하지 않는다.
