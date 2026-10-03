# NOVA-067 — VTable·Object Safety 사양서

| 항목 | 값 |
|---|---|
| 대상 | Nova 0.1 |
| 작성일 | 2026-10-03 |
| 문서 상태 | Draft — 기존 결정은 유지, 추가 상세는 승인 대기 |
| 적용 Stage | 제외/후속 |
| 근거 | [원본 NOVA-067](../../06_Interfaces_Generics/NOVA-067_VTable_Object_Safety_사양서.md) |

이 문서는 원본을 대체하는 확정 사양이 아니다. 기존 확정 기준은 [CANONICAL](../CANONICAL.md)을 따르며, 새 의미·문법·API·정책은 [DECISIONS](../DECISIONS.md)의 승인이 필요하다. D01~D05 Lexer 상세는 [승인 기준](../ACCEPTED_LEXER.md)을 따른다. [전체 색인](../INDEX.md).

## VTable/Object safety
0.1에는 dynamic interface dispatch가 없으므로 vtable layout/object-safety 알고리즘은 납품 범위가 아니다. runtime Class handle에도 interface vtable을 자동 넣지 않는다.

## 후속 검토 항목
object ownership, lifetime, receiver mode, generic method, associated type, ABI stability, drop dispatch를 한 묶음으로 새 버전에서 설계한다. 현재 Interface signature constraints는 NOVA-061의 static 검증만 수행한다.

## 검증
backend dump에 dynamic vtable이 불필요하게 생기지 않는지 검사한다. 미래용 TODO는 release 완료 조건이나 코드 구현 요청이 아니다.
