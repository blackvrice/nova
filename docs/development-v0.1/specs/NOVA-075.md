# NOVA-075 — AST→HIR Lowering 사양서

| 항목 | 값 |
|---|---|
| 대상 | Nova 0.1 |
| 작성일 | 2026-10-03 |
| 문서 상태 | Draft — 기존 결정은 유지, 추가 상세는 승인 대기 |
| 적용 Stage | A~E |
| 근거 | [원본 NOVA-075](../../07_Compiler_Frontend/NOVA-075_AST_HIR_Lowering_사양서.md) |

이 문서는 원본을 대체하는 확정 사양이 아니다. 기존 확정 기준은 [CANONICAL](../CANONICAL.md)을 따르며, 새 의미·문법·API·정책은 [DECISIONS](../DECISIONS.md)의 승인이 필요하다. D01~D05 Lexer 상세는 [승인 기준](../ACCEPTED_LEXER.md)을 따른다. [전체 색인](../INDEX.md).

Stage A 단일 파일 의미 검사는 사용자 승인 [P02](../SEMANTICS_STAGE_A_PROPOSAL.md)가 우선한다. runtime/전체 타입/미래 Stage의 추가 상세는 Draft다.

## Lowering 단계
AST arena → HIR arena와 lexical ScopeTree. 선언 수집과 body lowering을 분리하여 forward reference를 지원한다. identifier spelling은 SymbolId로 intern하되 아직 unresolved reference일 수 있다.

## Normalize 계약
Primitive alias/nullable/Unit sugar는 한 곳에서 처리한다. implicit default call argument 삽입은 resolution 이후 하며 SourceOrigin=DefaultArgument를 남긴다. ownership-sensitive for/try/using은 semantic 전용 node로 보존한다.

## 검증
같은 AST의 반복 lowering 결정성, Error AST→Error HIR, synthetic node note 위치, named argument 평가 순서, alias source spelling 진단을 검사한다.
