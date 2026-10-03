# NOVA-112 — JVM Hosted Adapter 사양서

| 항목 | 값 |
|---|---|
| 대상 | Nova 0.1 |
| 작성일 | 2026-10-03 |
| 문서 상태 | Draft — 기존 결정은 유지, 추가 상세는 승인 대기 |
| 적용 Stage | 제외/후속 |
| 근거 | [원본 NOVA-112](../../10_FFI/NOVA-112_JVM_Hosted_Adapter_사양서.md) |

이 문서는 원본을 대체하는 확정 사양이 아니다. 기존 확정 기준은 [CANONICAL](../CANONICAL.md)을 따르며, 새 의미·문법·API·정책은 [DECISIONS](../DECISIONS.md)의 승인이 필요하다. [전체 색인](../INDEX.md).

## JVM Hosted Adapter 범위
JVM adapter는 C FFI와 다른 후속 runtime feature이며 D27 승인 없이는 Nova 0.1 구현 범위에 넣지 않는다.

## 후속 계약
VM creation/attach/detach, local/global references, Java exception→Result, classpath/version, UTF conversion, callback thread, direct-buffer lifetime, host shutdown을 명시한다. local reference를 호출 종료 뒤 safe owner처럼 저장하지 않는다.

## 검증 조건
GC/moving reference stress, pending exception handling, attach leak, Unicode supplementary char, direct buffer invalidation. C ABI entry만 사용한다고 JVM 수명 문제가 해결되는 것은 아니다.
