# NOVA-111 — .NET Hosted Adapter 사양서

| 항목 | 값 |
|---|---|
| 대상 | Nova 0.1 |
| 작성일 | 2026-10-03 |
| 문서 상태 | Draft — 기존 결정은 유지, 추가 상세는 승인 대기 |
| 적용 Stage | 제외/후속 |
| 근거 | [원본 NOVA-111](../../10_FFI/NOVA-111_.NET_Hosted_Adapter_사양서.md) |

이 문서는 원본을 대체하는 확정 사양이 아니다. 기존 확정 기준은 [CANONICAL](../CANONICAL.md)을 따르며, 새 의미·문법·API·정책은 [DECISIONS](../DECISIONS.md)의 승인이 필요하다. D01~D05 Lexer 상세는 [승인 기준](../ACCEPTED_LEXER.md)을 따른다. [전체 색인](../INDEX.md).

## .NET Hosted Adapter 범위
Nova 0.1 확정 범위는 C FFI다. .NET host adapter는 추가 런타임 의존성을 갖는 후속 문서이며 현재 구현 완료 조건이 아니다(D27).

## 후속 계약
runtime startup/version, managed reference rooting/pinning, GC↔Nova ownership, exception translation, thread attach, string encoding, callback lifetime, assembly resolution을 설계해야 한다. managed pointer를 무기한 안전 view로 노출하지 않는다.

## 검증 조건
GC stress 중 handle validity, exception boundary, unload/reload, cross-thread callback, exactly-once release. 이러한 증거 없이 foreign dotnet 구문을 compiler에 추가하지 않는다.
