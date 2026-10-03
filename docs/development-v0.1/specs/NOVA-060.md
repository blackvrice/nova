# NOVA-060 — Thread 이동·공유 안전성 사양서

| 항목 | 값 |
|---|---|
| 대상 | Nova 0.1 |
| 작성일 | 2026-10-03 |
| 문서 상태 | Draft — 기존 결정은 유지, 추가 상세는 승인 대기 |
| 적용 Stage | C~E |
| 근거 | [원본 NOVA-060](../../05_Ownership_Safety/NOVA-060_Thread_이동_공유_안전성_사양서.md) |

이 문서는 원본을 대체하는 확정 사양이 아니다. 기존 확정 기준은 [CANONICAL](../CANONICAL.md)을 따르며, 새 의미·문법·API·정책은 [DECISIONS](../DECISIONS.md)의 승인이 필요하다. D01~D05 Lexer 상세는 [승인 기준](../ACCEPTED_LEXER.md)을 따른다. [전체 색인](../INDEX.md).

## Thread safety 초안 — D20
타입의 thread 이동 가능과 shared read 가능을 별도 compiler predicate로 관리한다. 이름/소스 trait 문법은 추가하지 않고 internal Sendable/Shareable 판정을 제안한다.

## 전파
primitive/owned immutable data는 component 조건을 따른다. raw pointer/borrowed local view/비동기화 mutable state는 기본 reject다. Shared<T>가 thread-safe라고 T의 동시 변경까지 허용하지 않는다.

## 검증
소유 값 thread 이전 pass, stack view escape fail, non-shareable foreign handle spawn fail, atomic refcount와 payload access 별도 race 검증. Thread API를 추가해 async/await를 선행 구현하지 않는다.
