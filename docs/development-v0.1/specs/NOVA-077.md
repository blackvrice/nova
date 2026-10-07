# NOVA-077 — 타입 검사기·소유권 검사기 아키텍처 사양서

| 항목 | 값 |
|---|---|
| 대상 | Nova 0.1 |
| 작성일 | 2026-10-03 |
| 문서 상태 | Draft — 기존 결정은 유지, 추가 상세는 승인 대기 |
| 적용 Stage | A~E |
| 근거 | [원본 NOVA-077](../../07_Compiler_Frontend/NOVA-077_타입_검사기_소유권_검사기_아키텍처_사양서.md) |

이 문서는 원본을 대체하는 확정 사양이 아니다. 기존 확정 기준은 [CANONICAL](../CANONICAL.md)을 따르며, 새 의미·문법·API·정책은 [DECISIONS](../DECISIONS.md)의 승인이 필요하다. D01~D05 Lexer 상세는 [승인 기준](../ACCEPTED_LEXER.md)을 따른다. [전체 색인](../INDEX.md).

Stage A 단일 파일 의미 검사는 사용자 승인 [P02](../SEMANTICS_STAGE_A_PROPOSAL.md)가 우선한다. runtime/전체 타입/미래 Stage의 추가 상세는 Draft다.

Stage B 가변 지역 변수·반복문 최소 부분은 사용자 진행 요청으로 승인한 [P04](../CONTROL_STAGE_B_PROPOSAL.md)와 [전용 EBNF](../GRAMMAR_STAGE_B_CONTROL.ebnf)가 우선한다. [구현·검증 기록](../CONTROL_IMPLEMENTATION.md). 전체 Stage B와 ownership/Drop 정책은 Draft다.

함수 내부 const와 제한된 상수 평가는 사용자 승인 [P05](../CONST_STAGE_B_PROPOSAL.md)와 [전용 EBNF](../GRAMMAR_STAGE_B_CONST.ebnf)가 우선한다. [구현·검증 기록](../CONST_IMPLEMENTATION.md). 전역 상수는 P05 범위가 아니며 const function/전체 D09 상세는 Draft다.

단일 파일 전역 const·forward dependency/cycle 최소 부분은 사용자 승인 [P06](../GLOBAL_CONST_STAGE_B_PROPOSAL.md)와 [전용 EBNF](../GRAMMAR_STAGE_B_GLOBAL_CONST.ebnf)를 따른다. [구현·검증 기록](../GLOBAL_CONST_IMPLEMENTATION.md). 함수 print shadow는 기존 P02대로 허용하고 전역 const print만 N2002다. module/const function/전체 D06·D09는 Draft다.

8종 고정 폭 정수·기대/peer literal 문맥·lossless 승격·checked runtime/const·MIR 변환·보간은 사용자 승인 [P07](../INTEGER_STAGE_B_PROPOSAL.md)를 따른다. [구현·검증 기록](../INTEGER_IMPLEMENTATION.md). P06 grammar를 재사용한다. 숫자 cast는 P10, 전체 D07은 Draft다.

char의 Unicode scalar 값·동일 타입 비교·선언/대입/함수/const·UTF-8 보간·private scalar ABI는 사용자 승인 [P08](../CHAR_STAGE_B_PROPOSAL.md)과 [CHAR primary EBNF](../GRAMMAR_STAGE_B_CHAR.ebnf)를 따른다. [구현·검증 기록](../CHAR_IMPLEMENTATION.md). D04 Lexer/escape/END는 유지하며 cast/char 산술·전체 D07은 Draft다.

binary32/64 literal·손실 없는 숫자 승격·IEEE 산술/비교·canonical NaN·const·최단 fixed decimal 보간·private ABI는 사용자 승인 [P09](../FLOAT_STAGE_B_PROPOSAL.md)와 [FLOAT primary EBNF](../GRAMMAR_STAGE_B_FLOAT.ebnf)를 따른다. [구현·검증 기록](../FLOAT_IMPLEMENTATION.md). float IEEE 결과는 const 실패가 아니며 INT checked 정책은 유지한다. 숫자 cast는 P10, float remainder/math API·전체 D07은 Draft다.

숫자 10종의 postfix as·operand literal 문맥 격리·checked 범위/직접 RN 반올림·float truncation·const N3201/Runtime Abort는 사용자 승인 [P10](../CAST_STAGE_B_PROPOSAL.md)과 [전용 EBNF](../GRAMMAR_STAGE_B_CAST.ebnf)를 따른다. [구현·검증 기록](../CAST_IMPLEMENTATION.md). Bool/Char/unsafe cast와 전체 D07은 Draft다.

root-relative 함수/전역 const item import·alias·internal/private/public·reachable graph·cross-file const·entry/source identity는 사용자 승인 [P11](../MODULE_STAGE_B_PROPOSAL.md)과 [전용 EBNF](../GRAMMAR_STAGE_B_MODULE.ebnf)를 따른다. [구현·검증 기록](../MODULE_IMPLEMENTATION.md). module alias/qualified value/reexport/Package와 전체 D06/D30은 후속이다.

nominal Copy struct·위치 생성·type import·가변 field 경로·const·private layout/ABI와 자원 제한은 사용자 승인 [P12](../STRUCT_STAGE_B_PROPOSAL.md)와 [37-production EBNF](../GRAMMAR_STAGE_B_STRUCT.ebnf)를 따른다. [구현·검증 기록](../STRUCT_IMPLEMENTATION.md). String field·init/Drop·일반 Move/borrow와 전체 D06/D10/D12/D16/D30은 후속이다.

structural Copy Tuple·numeric selector subspan·혼합 가변 경로·const·private aggregate ABI와 자원 제한은 사용자 승인 [P13](../TUPLE_STAGE_B_PROPOSAL.md)와 [40-production EBNF](../GRAMMAR_STAGE_B_TUPLE.ebnf)를 따른다. [구현·검증 기록](../TUPLE_IMPLEMENTATION.md). Array/일반 Move element와 전체 D09/D10/D12/D16은 후속이다.

사용자 승인한 Copy Enum·qualified variant·Enum/Bool statement match·coverage/binder·const·private tagged ABI는 [P14](../ENUM_STAGE_B_PROPOSAL.md), [48-production EBNF](../GRAMMAR_STAGE_B_ENUM.ebnf), [수용 fixture](../enum-proposal-fixtures/README.md), [구현 기록](../ENUM_IMPLEMENTATION.md)을 따른다. 전체 D06/D08/D09/D10/D12/D16/D25/D30은 계속 Draft다.

Copy Option<T>/Result<T,E>·T?·qualified 생성/none·문맥·match·const·private tagged ABI의 다음 최소 계약은 [P15 Draft](../OPTION_RESULT_STAGE_B_PROPOSAL.md), [51-production EBNF](../GRAMMAR_STAGE_B_OPTION_RESULT.ebnf), [제안 fixture](../option-result-proposal-fixtures/README.md)에 있다. 미승인/미구현이며 try·Move/Drop·사용자 Generic과 전체 D06/D08/D09/D10/D12/D15/D16/D23/D25/D30은 후속이다.

## Type/Ownership 분리
type checker는 typed expressions/call resolution/coercion과 요구 ownership mode를 계산한다. MIR analysis가 concrete CFG에서 initialization/move/borrow/drop 검사를 수행한다. source 수준 검사와 CFG 수준 검사를 중복 구현하더라도 책임을 명확히 한다.

## 데이터 계약
TypedBody는 TypeTable/CallResolution/OwnershipAction/Effects/Diagnostics를 포함한다. generic body는 symbolic constraint 상태를 구체 body와 구분한다. LLVM pointer type를 타입 안전성 판단에 쓰지 않는다.

## 검증
type mismatch에서 파생 borrow 오류 억제, mode-aware call, branch merge, ErrorType 차단, stage가 지원하는 타입 subset 선언.
