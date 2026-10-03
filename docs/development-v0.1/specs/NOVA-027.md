# NOVA-027 — Struct·Class·Enum 선언 사양서

| 항목 | 값 |
|---|---|
| 대상 | Nova 0.1 |
| 작성일 | 2026-10-03 |
| 문서 상태 | Draft — 기존 결정은 유지, 추가 상세는 승인 대기 |
| 적용 Stage | B~E |
| 근거 | [원본 NOVA-027](../../03_Types_Declarations/NOVA-027_Struct_Class_Enum_선언_사양서.md) |

이 문서는 원본을 대체하는 확정 사양이 아니다. 기존 확정 기준은 [CANONICAL](../CANONICAL.md)을 따르며, 새 의미·문법·API·정책은 [DECISIONS](../DECISIONS.md)의 승인이 필요하다. D01~D05 Lexer 상세는 [승인 기준](../ACCEPTED_LEXER.md)을 따른다. [전체 색인](../INDEX.md).

## 값/Handle 구분
Struct는 값 의미, Class는 정체성을 가진 owned handle, Enum은 하나의 활성 variant와 payload다. 모든 필드는 let/var를 명시하고 직접 재귀 값 포함은 금지한다. Option/Result는 특별 null sentinel가 아닌 일반 Enum 의미다.

## 선언 초안 — D12
struct/class/enum 이름 뒤 generic parameter와 implements 목록, {} member body를 제안한다. Enum variant는 이름과 위치 payload tuple로 시작하며 custom discriminant/field syntax는 보류한다. 직접 순환 layout graph는 size query에서 오류다.

## Copy/Drop
Struct Copy 여부는 모든 field의 Copy와 사용자 drop 부재에 따라 결정하는 D10 제안이다. Class handle 복사는 금지하고 take로 이전한다. inactive Enum payload를 읽거나 Drop하지 않는다.

## 검증
recursive-by-value fail, handle indirection pass, immutable field reassignment fail, enum payload Drop count 및 zero-sized aggregate를 다룬다.
