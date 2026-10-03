# NOVA-055 — Drop 정교화·Drop Flag 사양서

| 항목 | 값 |
|---|---|
| 대상 | Nova 0.1 |
| 작성일 | 2026-10-03 |
| 문서 상태 | Draft — 기존 결정은 유지, 추가 상세는 승인 대기 |
| 적용 Stage | C |
| 근거 | [원본 NOVA-055](../../05_Ownership_Safety/NOVA-055_Drop_정교화_Drop_Flag_사양서.md) |

이 문서는 원본을 대체하는 확정 사양이 아니다. 기존 확정 기준은 [CANONICAL](../CANONICAL.md)을 따르며, 새 의미·문법·API·정책은 [DECISIONS](../DECISIONS.md)의 승인이 필요하다. D01~D05 Lexer 상세는 [승인 기준](../ACCEPTED_LEXER.md)을 따른다. [전체 색인](../INDEX.md).

## Drop 의미
local과 field는 선언 역순 Drop한다. moved 원본은 Drop하지 않으며 조건부 초기화에는 Drop Flag를 쓴다. 정상 scope exit, return, break, continue, try를 모두 cleanup으로 연결한다.

## 순서 초안 — D10
return value/RHS는 먼저 평가하여 목적지에 안전하게 보존하고, 이후 떠나는 scope를 안쪽부터 정리한다. 사용자 drop body와 field glue 순서는 NOVA-056의 제안을 따른다. Abort는 cleanup 경로가 없다.

## 검증
Drop trace의 exact order, branch move flag, var replacement, early return, constructor 실패, inactive enum payload를 확인한다. Drop flag 최적화는 관측되는 Drop 횟수를 바꾸지 않아야 한다.
