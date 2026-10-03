# NOVA-140 — Runtime Test Harness 사양서

| 항목 | 값 |
|---|---|
| 대상 | Nova 0.1 |
| 작성일 | 2026-10-03 |
| 문서 상태 | Draft — 기존 결정은 유지, 추가 상세는 승인 대기 |
| 적용 Stage | 전 Stage |
| 근거 | [원본 NOVA-140](../../13_Testing_Release/NOVA-140_Runtime_Test_Harness_사양서.md) |

이 문서는 원본을 대체하는 확정 사양이 아니다. 기존 확정 기준은 [CANONICAL](../CANONICAL.md)을 따르며, 새 의미·문법·API·정책은 [DECISIONS](../DECISIONS.md)의 승인이 필요하다. [전체 색인](../INDEX.md).

## Runtime Harness
임시 출력 디렉터리에서 build 후 child process를 실행한다. expected stdout/stderr/exit class/timeout/Drop trace를 분리한다. child process는 crash/timeout 시 종료하고 남은 process/resource를 정리한다.

## 초안 — D25
UTF-8 stdout byte comparison, OS newline normalization의 제한된 허용, abort의 platform-specific exit class mapping을 제안한다. interactive/stdin/resource-heavy test는 명시 annotation을 둔다.

## 검증
Hello exact output, recursion/overflow/bounds abort, no cleanup-on-abort, output beyond pipe capacity, spaces/unicode executable path, concurrent fixtures isolation.
