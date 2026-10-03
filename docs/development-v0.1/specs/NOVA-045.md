# NOVA-045 — Range until·through 사양서

| 항목 | 값 |
|---|---|
| 대상 | Nova 0.1 |
| 작성일 | 2026-10-03 |
| 문서 상태 | Draft — 기존 결정은 유지, 추가 상세는 승인 대기 |
| 적용 Stage | A~B |
| 근거 | [원본 NOVA-045](../../04_Functions_Control/NOVA-045_Range_until_through_사양서.md) |

이 문서는 원본을 대체하는 확정 사양이 아니다. 기존 확정 기준은 [CANONICAL](../CANONICAL.md)을 따르며, 새 의미·문법·API·정책은 [DECISIONS](../DECISIONS.md)의 승인이 필요하다. D01~D05 Lexer 상세는 [승인 기준](../ACCEPTED_LEXER.md)을 따른다. [전체 색인](../INDEX.md).

## Range 계약
until는 upper exclusive, through는 inclusive다. 시작/끝은 한 번씩 source order로 평가한다. 첫 승인안 D08은 정수의 오름차순 step=1만 제공하고 custom step/descending은 제외한다.

## 경계 의미 초안
start≥end인 until는 빈 범위, start>end인 through는 빈 범위다. inclusive 최댓값은 마지막 값을 처리한 뒤 종료하며 end+1 계산으로 overflow하지 않는다. endpoints는 공통 lossless 정수 타입을 요구한다.

## 검증
0 until 3→0,1,2; 0 through 3→0,1,2,3; 3 until 3→empty; MAX through MAX→한 번. signed/unsigned 무손실 공통 타입 없는 조합은 fail이다.
