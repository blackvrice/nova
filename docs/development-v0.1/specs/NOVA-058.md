# NOVA-058 — Raw Pointer·Unsafe Capability 사양서

| 항목 | 값 |
|---|---|
| 대상 | Nova 0.1 |
| 작성일 | 2026-10-03 |
| 문서 상태 | Draft — 기존 결정은 유지, 추가 상세는 승인 대기 |
| 적용 Stage | C~E |
| 근거 | [원본 NOVA-058](../../05_Ownership_Safety/NOVA-058_Raw_Pointer_Unsafe_Capability_사양서.md) |

이 문서는 원본을 대체하는 확정 사양이 아니다. 기존 확정 기준은 [CANONICAL](../CANONICAL.md)을 따르며, 새 의미·문법·API·정책은 [DECISIONS](../DECISIONS.md)의 승인이 필요하다. D01~D05 Lexer 상세는 [승인 기준](../ACCEPTED_LEXER.md)을 따른다. [전체 색인](../INDEX.md).

## unsafe 경계
unsafe block은 위험 연산 허용 capability이며 소유권·빌림 검사를 자동 해제하지 않는다. raw pointer는 안전 view보다 약한 보장을 가지며 safe code가 invalid pointer를 직접 생성/역참조할 수 없어야 한다.

## 초안 — D17
pointer type 철자, address-of/deref, nullable pointer, alignment/validity 계약은 FFI_TYPES에서 제안한다. 각 unsafe operation은 pointer 유효성, allocation origin, alignment, initialized range, alias, thread 조건을 기록한다.

## 검증
safe raw deref fail, pointer escape wrapper fail, foreign pointer null/length/UTF-8 검사, owner Drop 뒤 view 재사용 거부. unsafe가 ErrorType/codegen 검증을 우회하지 않아야 한다.
