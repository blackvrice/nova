# NOVA-123 — 시간·난수·환경 변수 사양서

| 항목 | 값 |
|---|---|
| 대상 | Nova 0.1 |
| 작성일 | 2026-10-03 |
| 문서 상태 | Draft — 기존 결정은 유지, 추가 상세는 승인 대기 |
| 적용 Stage | B~E (print: A) |
| 근거 | [원본 NOVA-123](../../11_Standard_Library/NOVA-123_시간_난수_환경_변수_사양서.md) |

이 문서는 원본을 대체하는 확정 사양이 아니다. 기존 확정 기준은 [CANONICAL](../CANONICAL.md)을 따르며, 새 의미·문법·API·정책은 [DECISIONS](../DECISIONS.md)의 승인이 필요하다. [전체 색인](../INDEX.md).

## 시간/난수/환경 초안 — D23
wallClock timestamp와 monotonic Duration를 구분한다. elapsed 계산은 monotonic만 사용한다. secureRandom(bytes)→Result와 seedable deterministic RNG를 다른 API로 제공한다.

## 환경 계약
getEnv(name)→Result<Option<string>,EnvError>, args()→Array<string>를 제안한다. 비UTF-8 OS 값은 명시 변환 오류 또는 별도 raw API이며 replacement를 숨기지 않는다. 환경의 부작용은 pure가 아니다.

## 검증
clock adjustment에도 monotonic duration, deterministic seed corpus, OS RNG failure, missing vs empty env, invalid encoding. secret env 값을 diagnostic/cache dump에 자동 넣지 않는다.
