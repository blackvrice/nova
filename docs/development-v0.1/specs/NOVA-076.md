# NOVA-076 — Symbol Table·Definition Registry 사양서

| 항목 | 값 |
|---|---|
| 대상 | Nova 0.1 |
| 작성일 | 2026-10-03 |
| 문서 상태 | Draft — 기존 결정은 유지, 추가 상세는 승인 대기 |
| 적용 Stage | A~E |
| 근거 | [원본 NOVA-076](../../07_Compiler_Frontend/NOVA-076_Symbol_Table_Definition_Registry_사양서.md) |

이 문서는 원본을 대체하는 확정 사양이 아니다. 기존 확정 기준은 [CANONICAL](../CANONICAL.md)을 따르며, 새 의미·문법·API·정책은 [DECISIONS](../DECISIONS.md)의 승인이 필요하다. D01~D05 Lexer 상세는 [승인 기준](../ACCEPTED_LEXER.md)을 따른다. [전체 색인](../INDEX.md).

## Registry
DefinitionRegistry는 DefId→name,kind,owner,visibility,Span,signature source를 보유한다. ScopeTree는 parent/namespace bindings/import edges를 보유한다. Symbol interner는 session 문자열만 관리한다.

## mutation 경계
declare 단계 종료 뒤 registry의 identity/owner를 고정하고 resolution/type side table은 별도로 생성한다. duplicate 선언은 임의 overwrite하지 않고 충돌을 기록한다. LocalId는 function body 소유다.

## 검증
중복 선언 보존/secondary label, cross-module forward call, import order permutation, root namespace 격리, unresolved reference→Error resolution 처리.
