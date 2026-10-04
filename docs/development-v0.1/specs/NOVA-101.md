# NOVA-101 — Panic Runtime·Stack Trace 사양서

| 항목 | 값 |
|---|---|
| 대상 | Nova 0.1 |
| 작성일 | 2026-10-03 |
| 문서 상태 | Draft — 기존 결정은 유지, 추가 상세는 승인 대기 |
| 적용 Stage | A~E |
| 근거 | [원본 NOVA-101](../../09_Backend_Runtime/NOVA-101_Panic_Runtime_Stack_Trace_사양서.md) |

이 문서는 원본을 대체하는 확정 사양이 아니다. 기존 확정 기준은 [CANONICAL](../CANONICAL.md)을 따르며, 새 의미·문법·API·정책은 [DECISIONS](../DECISIONS.md)의 승인이 필요하다. D01~D05 Lexer 상세는 [승인 기준](../ACCEPTED_LEXER.md)을 따른다. [전체 색인](../INDEX.md).

Stage A Native 최소 arithmetic/print/entry/internal ABI/toolchain 계약은 사용자 승인 [P03](../NATIVE_STAGE_A_PROPOSAL.md)를 따른다. [구현·지원·검증 범위](../NATIVE_IMPLEMENTATION.md). 전체 D07~D28과 미래 Stage 정책은 Draft다.

## Panic
0.1 panic은 Abort다. unwind/catch를 도입하지 않고 FFI 경계를 넘어 예외를 전파하지 않는다. message와 Source 위치를 기록한다.

## Runtime 초안 — D18
runtime panic entry는 code/message/file/line을 받고 stderr best-effort 출력 후 abort한다. recursive panic/allocator failure 상황에서도 무한 재귀를 피한다. stack trace는 optional hook이며 실패해도 abort가 진행된다.

## 검증
explicit panic, bounds, integer overflow, division error, no cleanup-on-abort, stderr 위치. OS별 abort exit 값은 numeric 하나로 고정하지 않고 abnormal termination class로 검사한다.
