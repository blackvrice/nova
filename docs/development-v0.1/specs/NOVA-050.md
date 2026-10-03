# NOVA-050 — using Lowering 사양서

| 항목 | 값 |
|---|---|
| 대상 | Nova 0.1 |
| 작성일 | 2026-10-03 |
| 문서 상태 | Draft — 기존 결정은 유지, 추가 상세는 승인 대기 |
| 적용 Stage | A~B |
| 근거 | [원본 NOVA-050](../../04_Functions_Control/NOVA-050_using_Lowering_사양서.md) |

이 문서는 원본을 대체하는 확정 사양이 아니다. 기존 확정 기준은 [CANONICAL](../CANONICAL.md)을 따르며, 새 의미·문법·API·정책은 [DECISIONS](../DECISIONS.md)의 승인이 필요하다. D01~D05 Lexer 상세는 [승인 기준](../ACCEPTED_LEXER.md)을 따른다. [전체 색인](../INDEX.md).

## using 초안 — D08/D10
using let resource = expression을 block scope의 owned binding sugar로 제안한다. initializer는 한 번 평가하며 정상/return/break/continue/try exit에서 resource Drop을 보장한다.

## 경계
abort panic은 unwind하지 않으므로 using cleanup을 보장하지 않는다. Dispose 같은 새 interface를 자동 호출하지 않고 일반 drop glue에 연결한다. scope 밖으로 take하여 resource lifetime을 연장하는 정책은 첫 승인안에서 거부 제안이다.

## 검증
파일 open 성공/오류, nested using 역순 cleanup, early return, initializer 실패 때 미생성 resource Drop 없음, panic 시 종료를 각각 확인한다.
