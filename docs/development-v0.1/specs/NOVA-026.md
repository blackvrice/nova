# NOVA-026 — Primitive·Literal 기본 타입 결정 사양서

| 항목 | 값 |
|---|---|
| 대상 | Nova 0.1 |
| 작성일 | 2026-10-03 |
| 문서 상태 | Draft — 기존 결정은 유지, 추가 상세는 승인 대기 |
| 적용 Stage | A~B |
| 근거 | [원본 NOVA-026](../../03_Types_Declarations/NOVA-026_Primitive_Literal_기본_타입_결정_사양서.md) |

이 문서는 원본을 대체하는 확정 사양이 아니다. 기존 확정 기준은 [CANONICAL](../CANONICAL.md)을 따르며, 새 의미·문법·API·정책은 [DECISIONS](../DECISIONS.md)의 승인이 필요하다. D01~D05 Lexer 상세는 [승인 기준](../ACCEPTED_LEXER.md)을 따른다. [전체 색인](../INDEX.md).

Stage A 단일 파일 의미 검사는 사용자 승인 [P02](../SEMANTICS_STAGE_A_PROPOSAL.md)가 우선한다. runtime/전체 타입/미래 Stage의 추가 상세는 Draft다.

Stage A Native 최소 arithmetic/print/entry/internal ABI/toolchain 계약은 사용자 승인 [P03](../NATIVE_STAGE_A_PROPOSAL.md)를 따른다. [구현·지원·검증 범위](../NATIVE_IMPLEMENTATION.md). 전체 D07~D28과 미래 Stage 정책은 Draft다.

함수 내부 const와 제한된 상수 평가는 사용자 승인 [P05](../CONST_STAGE_B_PROPOSAL.md)와 [전용 EBNF](../GRAMMAR_STAGE_B_CONST.ebnf)가 우선한다. [구현·검증 기록](../CONST_IMPLEMENTATION.md). 전역 상수는 P05 범위가 아니며 const function/전체 D09 상세는 Draft다.

단일 파일 전역 const·forward dependency/cycle 최소 부분은 사용자 승인 [P06](../GLOBAL_CONST_STAGE_B_PROPOSAL.md)와 [전용 EBNF](../GRAMMAR_STAGE_B_GLOBAL_CONST.ebnf)를 따른다. [구현·검증 기록](../GLOBAL_CONST_IMPLEMENTATION.md). 함수 print shadow는 기존 P02대로 허용하고 전역 const print만 N2002다. module/const function/전체 D06·D09는 Draft다.

8종 고정 폭 정수·기대/peer literal 문맥·lossless 승격·checked runtime/const·MIR 변환·보간은 사용자 승인 [P07](../INTEGER_STAGE_B_PROPOSAL.md)를 따른다. [구현·검증 기록](../INTEGER_IMPLEMENTATION.md). P06 grammar를 재사용한다. 숫자 cast는 P10, 전체 D07은 Draft다.

char의 Unicode scalar 값·동일 타입 비교·선언/대입/함수/const·UTF-8 보간·private scalar ABI는 사용자 승인 [P08](../CHAR_STAGE_B_PROPOSAL.md)과 [CHAR primary EBNF](../GRAMMAR_STAGE_B_CHAR.ebnf)를 따른다. [구현·검증 기록](../CHAR_IMPLEMENTATION.md). D04 Lexer/escape/END는 유지하며 cast/char 산술·전체 D07은 Draft다.

binary32/64 literal·손실 없는 숫자 승격·IEEE 산술/비교·canonical NaN·const·최단 fixed decimal 보간·private ABI는 사용자 승인 [P09](../FLOAT_STAGE_B_PROPOSAL.md)와 [FLOAT primary EBNF](../GRAMMAR_STAGE_B_FLOAT.ebnf)를 따른다. [구현·검증 기록](../FLOAT_IMPLEMENTATION.md). float IEEE 결과는 const 실패가 아니며 INT checked 정책은 유지한다. 숫자 cast는 P10, float remainder/math API·전체 D07은 Draft다.

숫자 10종의 postfix as·operand literal 문맥 격리·checked 범위/직접 RN 반올림·float truncation·const N3201/Runtime Abort는 사용자 승인 [P10](../CAST_STAGE_B_PROPOSAL.md)과 [전용 EBNF](../GRAMMAR_STAGE_B_CAST.ebnf)를 따른다. [구현·검증 기록](../CAST_IMPLEMENTATION.md). Bool/Char/unsafe cast와 전체 D07은 Draft다.

사용자 승인한 loop·정수 범위 for는 [P19 Accepted](../RANGE_LOOP_STAGE_B_PROPOSAL.md), [54-production EBNF](../GRAMMAR_STAGE_B_RANGE_LOOP.ebnf), [수용 fixture](../range-loop-proposal-fixtures/README.md)를 따른다. 구현·검증 완료이며 [구현 기록](../RANGE_LOOP_IMPLEMENTATION.md)을 제공한다. 기존 P01~P18과 일반 iterable/Array/Move/Drop 경계는 보존한다.

숫자·char·Copy struct statement match의 whole binder/wildcard·Bool guard·snapshot·opaque domain coverage·Source/Checked/MIR 계약은 [P30 Accepted](../COPY_VALUE_MATCH_STAGE_B_PROPOSAL.md)·[수용 fixture](../copy-value-match-proposal-fixtures/README.md)·[구현 기록](../COPY_VALUE_MATCH_IMPLEMENTATION.md)을 따른다. P29 62-production EBNF와 기존 P01~P29 의미·평가 순서·예산을 보존한다. literal/field/or/range pattern·Array·String/Move·match expression·일반 Move/borrow/Drop·전체 D08/D10/D12/D16/D25/D30은 제외한다.

고정 폭 정수 literal 패턴·component 기대 타입/checked range·radix/zero alias·finite/observed-complement matrix·guard·refutable binding·Source/Checked/MIR 계약은 [P32 Accepted](../INTEGER_PATTERN_STAGE_B_PROPOSAL.md)·[63-production EBNF](../GRAMMAR_STAGE_B_INTEGER_PATTERN.ebnf)·[수용 fixture](../integer-pattern-proposal-fixtures/README.md)·[구현 기록](../INTEGER_PATTERN_IMPLEMENTATION.md)을 따른다. 기존 61개 production·P01~P31 ledger·opaque-only witness·snapshot/guard 순서/예산을 보존한다. float/string literal·field/or/range pattern·Array·String/Move·일반 Move/borrow/Drop·전체 D07/D08/D10/D12/D16/D25/D30은 제외한다.

Copy struct 위치 패턴·nominal/import head·전체 필드 순서/가시성·let/var/const 분해·bounded coverage·snapshot/guard·Source/Checked/MIR 계약은 [P33 Accepted](../STRUCT_PATTERN_STAGE_B_PROPOSAL.md)·[64-production EBNF](../GRAMMAR_STAGE_B_STRUCT_PATTERN.ebnf)·[수용 fixture](../struct-pattern-proposal-fixtures/README.md)·[구현 기록](../STRUCT_PATTERN_IMPLEMENTATION.md)을 따른다. 기존 61개 production·P01~P32 ledger·opaque-only witness·평가 순서/예산을 보존한다. 이름 field/rest/or/range 패턴·Array·String/Move field·일반 Move/borrow/Drop·전체 D06/D08/D09/D10/D12/D16/D25/D30은 제외한다.

Copy struct 이름 필드 패턴·역순/위치 prefix label 대응·Source 순서/선언 field 순서 plan·전체 필드 가시성·transparent wrapper·const/coverage·Source/Checked/MIR 계약은 [P34 Accepted](../STRUCT_NAMED_PATTERN_STAGE_B_PROPOSAL.md)·[66-production EBNF](../GRAMMAR_STAGE_B_STRUCT_NAMED_PATTERN.ebnf)·[수용 fixture](../struct-named-pattern-proposal-fixtures/README.md)·[구현 기록](../STRUCT_NAMED_PATTERN_IMPLEMENTATION.md)을 따른다. 기존 63개 production·P01~P33 ledger·평가 순서/예산을 보존한다. rest/shorthand/부분 필드·Enum/sum/Tuple 이름 payload·Array·String/Move field·일반 Move/borrow/Drop·전체 D06/D08/D09/D10/D12/D16/D25/D30은 제외한다.

Copy struct trailing rest 패턴·부분 field/명시 field_indices·전체 initializer 평가·기존 private 가시성·implicit wildcard coverage·sparse const/MIR·Source 검증은 [P35 Accepted](../STRUCT_REST_PATTERN_STAGE_B_PROPOSAL.md)·[67-production EBNF](../GRAMMAR_STAGE_B_STRUCT_REST_PATTERN.ebnf)·[수용 fixture](../struct-rest-pattern-proposal-fixtures/README.md)·[구현 기록](../STRUCT_REST_PATTERN_IMPLEMENTATION.md)을 따른다. 기존 65개 production·P01~P34 ledger·평가 순서/예산을 보존한다. shorthand·Tuple/Enum payload rest·private 접근 완화·Array·일반 Move/borrow/Drop·전체 D06/D08/D09/D10/D12/D16/D25/D30은 제외한다.

Copy Tuple trailing rest 패턴·명시 prefix/field_indices·Unit 구분·전체 initializer 평가·implicit wildcard coverage·prefix const/MIR·Source 검증은 [P36 Accepted](../TUPLE_REST_PATTERN_STAGE_B_PROPOSAL.md)·[68-production EBNF](../GRAMMAR_STAGE_B_TUPLE_REST_PATTERN.ebnf)·[수용 fixture](../tuple-rest-pattern-proposal-fixtures/README.md)·[구현 기록](../TUPLE_REST_PATTERN_IMPLEMENTATION.md)을 따른다. 기존 66개 production·P01~P35 ledger·평가 순서/예산을 보존한다. middle rest·Enum payload rest·Array·일반 Move/borrow/Drop·전체 D06/D08/D09/D10/D12/D16/D25/D30은 제외한다.

Copy Enum·sum payload trailing rest 패턴·explicit prefix ordinal·nullary 괄호 금지·tag 보존·전체 initializer 평가·implicit wildcard coverage·const/MIR·Source 검증은 [P37 Accepted](../PAYLOAD_REST_PATTERN_STAGE_B_PROPOSAL.md)·[69-production EBNF](../GRAMMAR_STAGE_B_PAYLOAD_REST_PATTERN.ebnf)·[수용 fixture](../payload-rest-pattern-proposal-fixtures/README.md)·[구현 기록](../PAYLOAD_REST_PATTERN_IMPLEMENTATION.md)을 따른다. 기존 67개 production·P01~P36 ledger·평가 순서/예산을 보존한다. middle rest·named payload·Array·일반 Move/borrow/Drop·전체 D06/D08/D09/D10/D12/D16/D25/D30은 제외한다.

Copy Tuple 중간 rest 패턴·prefix/suffix 실제 field_indices·빈 생략·실제 annotation·전체 initializer 평가·const/MIR·Source 검증은 [P38 Accepted](../TUPLE_MIDDLE_REST_STAGE_B_PROPOSAL.md)·[70-production EBNF](../GRAMMAR_STAGE_B_TUPLE_MIDDLE_REST.ebnf)·[수용 fixture](../tuple-middle-rest-proposal-fixtures/README.md)·[구현 기록](../TUPLE_MIDDLE_REST_IMPLEMENTATION.md)을 따른다. 기존 68개 production·P01~P37 ledger·평가 순서/예산을 보존한다. struct/Enum payload middle rest·rest binder·Array·일반 Move/borrow/Drop·전체 D06/D08/D09/D10/D12/D16/D25/D30은 제외한다.

## Primitive 계약
int/uint는 32-bit, float/double은 binary32/binary64 alias다. fixed-width signed/unsigned, bool, Unicode char, owned UTF-8 string, Unit, Never를 구분한다.

## 기본값 초안 — D07
정수 literal은 문맥 우선 후 int32, 실수는 문맥 우선 후 float32. unsigned 문맥의 음수, char의 surrogate, 범위 초과 literal은 오류다. char를 byte/int로 자동 변환하지 않는다.

## 연산 초안
모든 profile의 integer overflow, divide-by-zero, signed MIN/-1, bounds 실패는 Abort panic을 제안한다. 실수는 IEEE binary32/64 semantics, NaN 비교를 그대로 따르고 fast-math 기본 off를 제안한다. explicit cast 범위 밖도 panic, wrapping은 별도 명시 API다.

## 검증
각 width 최소/최대 및 ±1, 음수 최소값 parsing, float32 precision 경계, NaN/-0/infinity, debug/release 결과 일치를 검사한다.
