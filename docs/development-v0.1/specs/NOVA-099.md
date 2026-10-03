# NOVA-099 — Runtime Startup·main 호출 사양서

| 항목 | 값 |
|---|---|
| 대상 | Nova 0.1 |
| 작성일 | 2026-10-03 |
| 문서 상태 | Draft — 기존 결정은 유지, 추가 상세는 승인 대기 |
| 적용 Stage | A~E |
| 근거 | [원본 NOVA-099](../../09_Backend_Runtime/NOVA-099_Runtime_Startup_main_호출_사양서.md) |

이 문서는 원본을 대체하는 확정 사양이 아니다. 기존 확정 기준은 [CANONICAL](../CANONICAL.md)을 따르며, 새 의미·문법·API·정책은 [DECISIONS](../DECISIONS.md)의 승인이 필요하다. [전체 색인](../INDEX.md).

## Startup
OS entry → platform runtime initialization → Nova main → 정상 shutdown/exit. user main return/argv 의미는 D19를 따른다. source-level main을 platform ABI로 직접 노출하지 않는다.

## cleanup 경계
정상 종료는 main local cleanup 후 runtime resource 종료다. Abort panic은 unwinding/global destructor 보장을 하지 않는다. module runtime 초기화 순환은 금지하고 deterministic startup order를 유지한다.

## 검증
빈 main exit0, print UTF-8, allocation/panic startup path, runtime symbol link, repeated process invocation independence, library target에 entry 미생성.
