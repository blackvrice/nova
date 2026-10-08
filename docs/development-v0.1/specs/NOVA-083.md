# NOVA-083 — MIR 검증기 사양서

| 항목 | 값 |
|---|---|
| 대상 | Nova 0.1 |
| 작성일 | 2026-10-03 |
| 문서 상태 | Draft — 기존 결정은 유지, 추가 상세는 승인 대기 |
| 적용 Stage | A~E |
| 근거 | [원본 NOVA-083](../../08_MIR_Middleend/NOVA-083_MIR_검증기_사양서.md) |

이 문서는 원본을 대체하는 확정 사양이 아니다. 기존 확정 기준은 [CANONICAL](../CANONICAL.md)을 따르며, 새 의미·문법·API·정책은 [DECISIONS](../DECISIONS.md)의 승인이 필요하다. D01~D05 Lexer 상세는 [승인 기준](../ACCEPTED_LEXER.md)을 따른다. [전체 색인](../INDEX.md).

Stage A MIR은 원본 CFG/Place 기준과 P02의 평가 순서에 따라 구현했다. [MIR 구현 기록](../MIR_IMPLEMENTATION.md)은 현재 API/검증 경계이며 runtime/미래 Stage 정책의 승인이 아니다.

Stage B 가변 지역 변수·반복문 최소 부분은 사용자 진행 요청으로 승인한 [P04](../CONTROL_STAGE_B_PROPOSAL.md)와 [전용 EBNF](../GRAMMAR_STAGE_B_CONTROL.ebnf)가 우선한다. [구현·검증 기록](../CONTROL_IMPLEMENTATION.md). 전체 Stage B와 ownership/Drop 정책은 Draft다.

nominal Copy struct·위치 생성·type import·가변 field 경로·const·private layout/ABI와 자원 제한은 사용자 승인 [P12](../STRUCT_STAGE_B_PROPOSAL.md)와 [37-production EBNF](../GRAMMAR_STAGE_B_STRUCT.ebnf)를 따른다. [구현·검증 기록](../STRUCT_IMPLEMENTATION.md). String field·init/Drop·일반 Move/borrow와 전체 D06/D10/D12/D16/D30은 후속이다.

structural Copy Tuple·numeric selector subspan·혼합 가변 경로·const·private aggregate ABI와 자원 제한은 사용자 승인 [P13](../TUPLE_STAGE_B_PROPOSAL.md)와 [40-production EBNF](../GRAMMAR_STAGE_B_TUPLE.ebnf)를 따른다. [구현·검증 기록](../TUPLE_IMPLEMENTATION.md). Array/일반 Move element와 전체 D09/D10/D12/D16은 후속이다.

사용자 승인한 Copy Enum·qualified variant·Enum/Bool statement match·coverage/binder·const·private tagged ABI는 [P14](../ENUM_STAGE_B_PROPOSAL.md), [48-production EBNF](../GRAMMAR_STAGE_B_ENUM.ebnf), [수용 fixture](../enum-proposal-fixtures/README.md), [구현 기록](../ENUM_IMPLEMENTATION.md)을 따른다. 전체 D06/D08/D09/D10/D12/D16/D25/D30은 계속 Draft다.

사용자 승인한 Copy Option<T>/Result<T,E>·T?·qualified 생성/none·문맥·match·const·private tagged ABI는 [P15 Accepted](../OPTION_RESULT_STAGE_B_PROPOSAL.md), [51-production EBNF](../GRAMMAR_STAGE_B_OPTION_RESULT.ebnf), [수용 fixture](../option-result-proposal-fixtures/README.md), [구현 기록](../OPTION_RESULT_IMPLEMENTATION.md)을 따른다. try·Move/Drop·사용자 Generic과 전체 D06/D08/D09/D10/D12/D15/D16/D23/D25/D30은 후속이다.

사용자 승인한 Copy prefix try·Result Error 조기 반환·operand 문맥 격리·정확한 E·const 금지·Source/CFG 검증은 [P16 Accepted](../TRY_STAGE_B_PROPOSAL.md), [51-production EBNF](../GRAMMAR_STAGE_B_TRY.ebnf), [수용 fixture](../try-proposal-fixtures/README.md), [구현 기록](../TRY_IMPLEMENTATION.md)을 따른다. Option try·error conversion·일반 Move/Drop와 전체 D08/D09/D10/D12/D16/D23/D25는 후속이다.

사용자 승인한 함수 이름 인수·parameter mapping·source-order snapshot/try·진단 계약은 [P17 Accepted](../NAMED_ARGUMENTS_STAGE_B_PROPOSAL.md), [52-production EBNF](../GRAMMAR_STAGE_B_NAMED_ARGUMENTS.ebnf), [수용 fixture](../named-arguments-proposal-fixtures/README.md), [구현 기록](../NAMED_ARGUMENTS_IMPLEMENTATION.md)을 따른다. 구현·검증 완료이며 기본 인수·overload·named constructor와 전체 D11/D16/D25/D30 승인이 아니다.

사용자 승인한 상수 표현식 함수 기본 인수·declaration scope·caller materialization·생략 인수 대응·상수 실패/예산 계약은 [P18 Accepted](../DEFAULT_ARGUMENTS_STAGE_B_PROPOSAL.md), [52-production EBNF](../GRAMMAR_STAGE_B_DEFAULT_ARGUMENTS.ebnf), [수용 fixture](../default-arguments-proposal-fixtures/README.md), [구현 기록](../DEFAULT_ARGUMENTS_IMPLEMENTATION.md)을 따른다. 구현·검증 완료이며 runtime/parameter 의존 default·overload·named constructor와 전체 D09/D11/D16/D25/D30 승인이 아니다.

사용자 승인한 loop·정수 범위 for는 [P19 Accepted](../RANGE_LOOP_STAGE_B_PROPOSAL.md), [54-production EBNF](../GRAMMAR_STAGE_B_RANGE_LOOP.ebnf), [수용 fixture](../range-loop-proposal-fixtures/README.md)를 따른다. 구현·검증 완료이며 [구현 기록](../RANGE_LOOP_IMPLEMENTATION.md)을 제공한다. 기존 P01~P18과 일반 iterable/Array/Move/Drop 경계는 보존한다.

Copy intrinsic Option postfix exists·Bool·const/default·단일 평가·Source/MIR 검증은 [P20 Accepted](../EXISTS_STAGE_B_PROPOSAL.md), [54-production EBNF](../GRAMMAR_STAGE_B_EXISTS.ebnf), [수용 fixture](../exists-proposal-fixtures/README.md)를 따른다. [구현 기록](../EXISTS_IMPLEMENTATION.md)에 검증을 기록했다. Result exists·flow narrowing·Move/Drop·Array와 전체 D08/D09/D10/D16/D23/D25/D30 승인이 아니다.

비제네릭 transparent type alias·type 위치·forward/import·cycle/자원·Source/MIR 검증은 [P21 Accepted](../ALIAS_STAGE_B_PROPOSAL.md), [55-production EBNF](../GRAMMAR_STAGE_B_ALIAS.ebnf), [수용 fixture](../alias-proposal-fixtures/README.md)를 따른다. [구현 기록](../ALIAS_IMPLEMENTATION.md)에 검증을 기록했다. type은 D01에서 이미 keyword이며 전체 D06/D10/D11/D12/D16/D30·generic alias/newtype·alias constructor/variant·API leak/export 정책은 승인하지 않았다.

Copy struct Read instance method·contextual self·member scope/visibility·receiver-first snapshot·named/default·Source/MIR 검증은 [P22 Accepted](../METHOD_STAGE_B_PROPOSAL.md), [58-production EBNF](../GRAMMAR_STAGE_B_METHOD.ebnf), [수용 fixture](../method-proposal-fixtures/README.md)와 [구현 기록](../METHOD_IMPLEMENTATION.md)을 따른다. 구현·검증 완료다. change/take·Move/borrow/Drop·init·overload·bound method·Enum method·Array와 전체 D06/D09/D10/D11/D12/D16/D25/D30은 승인하지 않았다.

Copy struct generated constructor 이름 인수·field mapping·source-order snapshot·const/default·Source/MIR 계약은 [P23 Accepted](../STRUCT_NAMED_ARGUMENTS_STAGE_B_PROPOSAL.md)·[수용 fixture](../struct-named-arguments-proposal-fixtures/README.md)·[구현 기록](../STRUCT_NAMED_ARGUMENTS_IMPLEMENTATION.md)을 따른다. 구현·검증 완료이며 승인 P22 58-production EBNF를 변경 없이 재사용한다. explicit init·field default·Enum/sum named constructor·Array·일반 Move/Drop은 범위 밖이다.

중첩 Copy sum/tuple pattern·Unit/Copy Tuple statement match·recursive binder·matrix coverage·Source/MIR 검증은 [P24 Accepted](../NESTED_PATTERN_STAGE_B_PROPOSAL.md)·[60-production EBNF](../GRAMMAR_STAGE_B_NESTED_PATTERN.ebnf)·[수용 fixture](../nested-pattern-proposal-fixtures/README.md)와 [구현 기록](../NESTED_PATTERN_IMPLEMENTATION.md)을 따른다. 구현·검증 완료이며 P14/P15의 flat match와 P01~P23 승인 범위를 보존한다. guard/일반 literal/struct destructuring·Array·Move/loan/Drop·전체 D08/D10/D12/D16/D25/D30은 제외한다.

## Validator
pre-analysis와 post-drop/optimization validator를 분리한다. block terminator 존재, successor/local/type IDs 유효, projection type 일치, operand/rvalue type, return/call signature, switch tag 범위를 확인한다.

## 안전 입력
codegen 전에는 ErrorType, unresolved generic, uninitialized read, illegal Move/Loan, unelaborated Drop를 허용하지 않는다. early MIR에서는 아직 분석 전 정보가 있다는 점을 validator phase로 표현한다.

## 검증
deliberately malformed MIR corpus를 만들고 mismatch마다 stable internal code와 source origin을 보고한다. optimizer 전후 validator 모두 통과해야 한다. invalid CFG의 cycle 자체를 오류로 보지는 않는다.
