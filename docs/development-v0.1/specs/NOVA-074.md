# NOVA-074 — HIR Node 구조서

| 항목 | 값 |
|---|---|
| 대상 | Nova 0.1 |
| 작성일 | 2026-10-03 |
| 문서 상태 | Draft — 기존 결정은 유지, 추가 상세는 승인 대기 |
| 적용 Stage | A~E |
| 근거 | [원본 NOVA-074](../../07_Compiler_Frontend/NOVA-074_HIR_Node_구조서.md) |

이 문서는 원본을 대체하는 확정 사양이 아니다. 기존 확정 기준은 [CANONICAL](../CANONICAL.md)을 따르며, 새 의미·문법·API·정책은 [DECISIONS](../DECISIONS.md)의 승인이 필요하다. D01~D05 Lexer 상세는 [승인 기준](../ACCEPTED_LEXER.md)을 따른다. [전체 색인](../INDEX.md).

Stage A 단일 파일 의미 검사는 사용자 승인 [P02](../SEMANTICS_STAGE_A_PROPOSAL.md)가 우선한다. runtime/전체 타입/미래 Stage의 추가 상세는 Draft다.

## HIR 데이터
HirItemId/HirExprId/HirStmtId/HirPatternId/ScopeId와 source origin을 사용한다. T?→Option<T>, void/생략→Unit, ()→UnitValue로 정규화한다.

## 유지하는 구문
try/exists/using/for는 전용 HIR로 보존한다. 이름 인수는 Label+SourceOrder, 할당은 HirStatement다. early lowering으로 argument reorder/cleanup 의미를 감추지 않는다.

## Side tables
ResolutionMap, TypeTable, CoercionTable, OwnershipTable, EffectTable을 독립 산출물로 관리한다. HIR은 trivia를 참조하지 않으며 Error HIR로 분석을 계속할 수 있다.

## 검증
AST sugar와 canonical HIR 동등성, synthetic source origin, source-order 유지, ErrorType/codegen 차단.
