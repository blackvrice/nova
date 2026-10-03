# NOVA-113 — Python Hosted Adapter 사양서

| 항목 | 값 |
|---|---|
| 대상 | Nova 0.1 |
| 작성일 | 2026-10-03 |
| 문서 상태 | Draft — 기존 결정은 유지, 추가 상세는 승인 대기 |
| 적용 Stage | 제외/후속 |
| 근거 | [원본 NOVA-113](../../10_FFI/NOVA-113_Python_Hosted_Adapter_사양서.md) |

이 문서는 원본을 대체하는 확정 사양이 아니다. 기존 확정 기준은 [CANONICAL](../CANONICAL.md)을 따르며, 새 의미·문법·API·정책은 [DECISIONS](../DECISIONS.md)의 승인이 필요하다. [전체 색인](../INDEX.md).

## Python Hosted Adapter 범위
Python adapter는 후속 feature(D27)다. Nova compiler 구현 언어 Rust와도 무관하다. 0.1 C FFI만으로 Python semantics를 자동 제공하지 않는다.

## 후속 계약
interpreter version/init/finalize, object retain/release, thread/GIL 정책, exception/result, bytes/UTF-8 string 구별, extension ownership, callback reentrancy, module loading을 명시한다.

## 검증 조건
object refcount, interpreter 종료 뒤 handle 사용 금지, exception cleanup, thread callback, Python 문자열/bytes roundtrip. version별 host API는 실제 도입 시 공식 문서로 검증한다.
