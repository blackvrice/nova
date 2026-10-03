# NOVA-044 — break·continue·return 사양서

| 항목 | 값 |
|---|---|
| 대상 | Nova 0.1 |
| 작성일 | 2026-10-03 |
| 문서 상태 | Draft — 기존 결정은 유지, 추가 상세는 승인 대기 |
| 적용 Stage | A~B |
| 근거 | [원본 NOVA-044](../../04_Functions_Control/NOVA-044_break_continue_return_사양서.md) |

이 문서는 원본을 대체하는 확정 사양이 아니다. 기존 확정 기준은 [CANONICAL](../CANONICAL.md)을 따르며, 새 의미·문법·API·정책은 [DECISIONS](../DECISIONS.md)의 승인이 필요하다. D01~D05 Lexer 상세는 [승인 기준](../ACCEPTED_LEXER.md)을 따른다. [전체 색인](../INDEX.md).

## Jump 의미
return은 enclosing function, break/continue는 가장 가까운 loop에 대응한다. lambda 내부 jump가 바깥 function/loop를 벗어나지 않는다. labeled jump 문법은 이번 초안에서 제외한다.

## 초안 — D08
Unit 함수의 bare return 허용, 값 return은 declared type에 check한다. non-Unit 함수의 reachable fallthrough는 오류다. break value를 지원하지 않는다. return expression은 이동/평가 후 local cleanup을 실행한다.

## 검증
loop 밖 break/continue fail, function 밖 return fail, 한 branch return 누락 fail, never callee 뒤 unreachable, return된 owner가 local Drop로 파괴되지 않는지 확인한다. END의 return newline 의미는 D05와 일치해야 한다.
