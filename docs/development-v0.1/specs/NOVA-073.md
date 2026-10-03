# NOVA-073 — AST Node 구조서

| 항목 | 값 |
|---|---|
| 대상 | Nova 0.1 |
| 작성일 | 2026-10-03 |
| 문서 상태 | Draft — 기존 결정은 유지, 추가 상세는 승인 대기 |
| 적용 Stage | A~E |
| 근거 | [원본 NOVA-073](../../07_Compiler_Frontend/NOVA-073_AST_Node_구조서.md) |

이 문서는 원본을 대체하는 확정 사양이 아니다. 기존 확정 기준은 [CANONICAL](../CANONICAL.md)을 따르며, 새 의미·문법·API·정책은 [DECISIONS](../DECISIONS.md)의 승인이 필요하다. D01~D05 Lexer 상세는 [승인 기준](../ACCEPTED_LEXER.md)을 따른다. [전체 색인](../INDEX.md).

## AST 데이터
Arena node는 AstNodeId/Span/NodeKind/token anchor를 가진다. item은 Func/Struct/Class/Enum/Interface/Foreign/Use/Const/TypeAlias, statement는 Binding/Assign/Expr/Return/If/While/For/Loop/Match/Using/Unsafe/Error를 제안한다.

## Expression
Literal/Name/Prefix/Binary/Call/Member/Index/Cast/Exists/Tuple/Array/Lambda/Interpolation/Error. semantic TypeId와 DefId는 저장하지 않는다. grammar가 지원하지 않는 미래 node를 미리 구현하지 않는다.

## Visitor
source-order walking, immutable visit와 controlled transform을 분리한다. Error/synthetic node도 방문하고 Span/source origin을 잃지 않는다.

## 검증
node ID 유일성, 부모 Span child 포함, trivia anchor, ErrorNode visitor, arena dump의 deterministic order.
