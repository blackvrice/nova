# NOVA-053 — 부분 이동 정책 사양서

| 항목 | 값 |
|---|---|
| 대상 | Nova 0.1 |
| 작성일 | 2026-10-03 |
| 문서 상태 | Draft — 기존 결정은 유지, 추가 상세는 승인 대기 |
| 적용 Stage | C |
| 근거 | [원본 NOVA-053](../../05_Ownership_Safety/NOVA-053_부분_이동_정책_사양서.md) |

이 문서는 원본을 대체하는 확정 사양이 아니다. 기존 확정 기준은 [CANONICAL](../CANONICAL.md)을 따르며, 새 의미·문법·API·정책은 [DECISIONS](../DECISIONS.md)의 승인이 필요하다. D01~D05 Lexer 상세는 [승인 기준](../ACCEPTED_LEXER.md)을 따른다. [전체 색인](../INDEX.md).

## Canonical 금지
사용자 partial move는 지원하지 않는다. aggregate의 field만 take하여 나머지를 다시 사용하는 표현은 거부한다. whole-value take와 Copy field read는 허용 범주다.

## 구체 경계 — D10
take value.moveField, destructuring으로 일부 Move payload만 소유하는 pattern, closure의 일부 field take capture를 같은 규칙으로 검사한다. take 전체 Enum을 consume한 후 활성 payload 처리하는 내부 lowering은 사용자 partial move와 구별한다.

## 내부 구현
compiler의 constructor/Array 부분 초기화와 drop glue field 상태는 허용된다. 사용자 제한을 이유로 double-drop 방지용 field state를 제거하지 않는다.

## 검증
struct Move field extraction fail, Copy integer projection pass, whole struct take pass, partially initialized cleanup은 initialized field만 역순으로 Drop한다.
