# NOVA-104 — Platform Abstraction Layer 사양서

| 항목 | 값 |
|---|---|
| 대상 | Nova 0.1 |
| 작성일 | 2026-10-03 |
| 문서 상태 | Draft — 기존 결정은 유지, 추가 상세는 승인 대기 |
| 적용 Stage | A~E |
| 근거 | [원본 NOVA-104](../../09_Backend_Runtime/NOVA-104_Platform_Abstraction_Layer_사양서.md) |

이 문서는 원본을 대체하는 확정 사양이 아니다. 기존 확정 기준은 [CANONICAL](../CANONICAL.md)을 따르며, 새 의미·문법·API·정책은 [DECISIONS](../DECISIONS.md)의 승인이 필요하다. D01~D05 Lexer 상세는 [승인 기준](../ACCEPTED_LEXER.md)을 따른다. [전체 색인](../INDEX.md).

## Platform layer
file/console/env/clock/random/thread/allocator/dynamic library의 최소 OS 경계를 모듈화한다. 언어 의미와 API error category는 플랫폼에 독립, 구체 OS error code는 note/source error payload다.

## Target 계약
Windows wide path conversion과 Linux byte path 차이는 public Path API에서 정의한다. Console는 UTF-8 contract이며 Windows terminal conversion을 runtime에서 수행한다. newline text translation은 API가 명시한 경우만 한다.

## 검증
Unicode path/output, invalid OS encoding, permission error, monotonic time, secure random failure, resource leak counts across Windows/Linux. backend Target support와 std API support를 별도 표로 관리한다.
