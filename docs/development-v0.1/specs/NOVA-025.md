# NOVA-025 — 타입 시스템·타입 추론·형변환 사양서

| 항목 | 값 |
|---|---|
| 대상 | Nova 0.1 |
| 작성일 | 2026-10-03 |
| 문서 상태 | Draft — 기존 결정은 유지, 추가 상세는 승인 대기 |
| 적용 Stage | A~B |
| 근거 | [원본 NOVA-025](../../03_Types_Declarations/NOVA-025_타입_시스템_타입_추론_형변환_사양서.md) |

이 문서는 원본을 대체하는 확정 사양이 아니다. 기존 확정 기준은 [CANONICAL](../CANONICAL.md)을 따르며, 새 의미·문법·API·정책은 [DECISIONS](../DECISIONS.md)의 승인이 필요하다. D01~D05 Lexer 상세는 [승인 기준](../ACCEPTED_LEXER.md)을 따른다. [전체 색인](../INDEX.md).

Stage A 단일 파일 의미 검사는 사용자 승인 [P02](../SEMANTICS_STAGE_A_PROPOSAL.md)가 우선한다. runtime/전체 타입/미래 Stage의 추가 상세는 Draft다.

Stage A Native 최소 arithmetic/print/entry/internal ABI/toolchain 계약은 사용자 승인 [P03](../NATIVE_STAGE_A_PROPOSAL.md)를 따른다. [구현·지원·검증 범위](../NATIVE_IMPLEMENTATION.md). 전체 D07~D28과 미래 Stage 정책은 Draft다.

함수 내부 const와 제한된 상수 평가는 사용자 승인 [P05](../CONST_STAGE_B_PROPOSAL.md)와 [전용 EBNF](../GRAMMAR_STAGE_B_CONST.ebnf)가 우선한다. [구현·검증 기록](../CONST_IMPLEMENTATION.md). 전역 상수는 P05 범위가 아니며 const function/전체 D09 상세는 Draft다.

단일 파일 전역 const·forward dependency/cycle 최소 부분은 사용자 승인 [P06](../GLOBAL_CONST_STAGE_B_PROPOSAL.md)와 [전용 EBNF](../GRAMMAR_STAGE_B_GLOBAL_CONST.ebnf)를 따른다. [구현·검증 기록](../GLOBAL_CONST_IMPLEMENTATION.md). 함수 print shadow는 기존 P02대로 허용하고 전역 const print만 N2002다. module/const function/전체 D06·D09는 Draft다.

8종 고정 폭 정수·기대/peer literal 문맥·lossless 승격·checked runtime/const·MIR 변환·보간은 사용자 승인 [P07](../INTEGER_STAGE_B_PROPOSAL.md)를 따른다. [구현·검증 기록](../INTEGER_IMPLEMENTATION.md). P06 grammar를 재사용하고 float/cast/전체 D07은 Draft다.

char의 Unicode scalar 값·동일 타입 비교·선언/대입/함수/const·UTF-8 보간·private scalar ABI는 사용자 승인 [P08](../CHAR_STAGE_B_PROPOSAL.md)과 [CHAR primary EBNF](../GRAMMAR_STAGE_B_CHAR.ebnf)를 따른다. [구현·검증 기록](../CHAR_IMPLEMENTATION.md). D04 Lexer/escape/END는 유지하며 float/cast/char 산술·전체 D07은 Draft다.

## 타입 검사
양방향 검사: 기대 타입이 있으면 check, 없으면 synthesize. TypeInterner는 Unit/Never/Primitive/Named/Tuple/Array/Function/GenericParam/Error를 구분한다. alias와 nullable 정규화는 unification 전에 수행한다.

## 변환 초안 — D07
같은 타입/alias identity, 제한적 lossless numeric widening만 암묵 허용한다. signed/unsigned는 전체 값 범위가 포함될 때만 허용한다. integer→float는 전 범위 정확 표현 가능할 때만 허용한다. int32→float32 및 int64→float64는 자동 허용하지 않는다.

## 오류
type mismatch는 기대 타입의 선언과 actual expression Span을 표시한다. ErrorType는 분석 지속 전용이며 codegen으로 통과하지 않는다. unconstrained literal은 기본 int/float를 제안하고 범위 초과는 추측 widening 대신 오류다.

## 검증
NUMERIC_RULES의 변환표와 overload 선택 결과가 일치해야 한다. bool↔int implicit 변환은 금지 제안이다.
