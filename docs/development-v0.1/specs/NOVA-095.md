# NOVA-095 — Backend 타입 레이아웃·정렬·Niche 구현서

| 항목 | 값 |
|---|---|
| 대상 | Nova 0.1 |
| 작성일 | 2026-10-03 |
| 문서 상태 | Draft — 기존 결정은 유지, 추가 상세는 승인 대기 |
| 적용 Stage | A~E |
| 근거 | [원본 NOVA-095](../../09_Backend_Runtime/NOVA-095_Backend_타입_레이아웃_정렬_Niche_구현서.md) |

이 문서는 원본을 대체하는 확정 사양이 아니다. 기존 확정 기준은 [CANONICAL](../CANONICAL.md)을 따르며, 새 의미·문법·API·정책은 [DECISIONS](../DECISIONS.md)의 승인이 필요하다. D01~D05 Lexer 상세는 [승인 기준](../ACCEPTED_LEXER.md)을 따른다. [전체 색인](../INDEX.md).

Stage A Native 최소 arithmetic/print/entry/internal ABI/toolchain 계약은 사용자 승인 [P03](../NATIVE_STAGE_A_PROPOSAL.md)를 따른다. [구현·지원·검증 범위](../NATIVE_IMPLEMENTATION.md). 전체 D07~D28과 미래 Stage 정책은 Draft다.

8종 고정 폭 정수·기대/peer literal 문맥·lossless 승격·checked runtime/const·MIR 변환·보간은 사용자 승인 [P07](../INTEGER_STAGE_B_PROPOSAL.md)를 따른다. [구현·검증 기록](../INTEGER_IMPLEMENTATION.md). P06 grammar를 재사용한다. 숫자 cast는 P10, 전체 D07은 Draft다.

char의 Unicode scalar 값·동일 타입 비교·선언/대입/함수/const·UTF-8 보간·private scalar ABI는 사용자 승인 [P08](../CHAR_STAGE_B_PROPOSAL.md)과 [CHAR primary EBNF](../GRAMMAR_STAGE_B_CHAR.ebnf)를 따른다. [구현·검증 기록](../CHAR_IMPLEMENTATION.md). D04 Lexer/escape/END는 유지하며 cast/char 산술·전체 D07은 Draft다.

binary32/64 literal·손실 없는 숫자 승격·IEEE 산술/비교·canonical NaN·const·최단 fixed decimal 보간·private ABI는 사용자 승인 [P09](../FLOAT_STAGE_B_PROPOSAL.md)와 [FLOAT primary EBNF](../GRAMMAR_STAGE_B_FLOAT.ebnf)를 따른다. [구현·검증 기록](../FLOAT_IMPLEMENTATION.md). float IEEE 결과는 const 실패가 아니며 INT checked 정책은 유지한다. 숫자 cast는 P10, float remainder/math API·전체 D07은 Draft다.

숫자 10종의 postfix as·operand literal 문맥 격리·checked 범위/직접 RN 반올림·float truncation·const N3201/Runtime Abort는 사용자 승인 [P10](../CAST_STAGE_B_PROPOSAL.md)과 [전용 EBNF](../GRAMMAR_STAGE_B_CAST.ebnf)를 따른다. [구현·검증 기록](../CAST_IMPLEMENTATION.md). Bool/Char/unsafe cast와 전체 D07은 Draft다.

root-relative 함수/전역 const item import·alias·internal/private/public·reachable graph·cross-file const·entry/source identity는 사용자 승인 [P11](../MODULE_STAGE_B_PROPOSAL.md)과 [전용 EBNF](../GRAMMAR_STAGE_B_MODULE.ebnf)를 따른다. [구현·검증 기록](../MODULE_IMPLEMENTATION.md). module alias/qualified value/reexport/Package와 전체 D06/D30은 후속이다.

nominal Copy struct·위치 생성·type import·가변 field 경로·const·private layout/ABI와 자원 제한은 사용자 승인 [P12](../STRUCT_STAGE_B_PROPOSAL.md)와 [37-production EBNF](../GRAMMAR_STAGE_B_STRUCT.ebnf)를 따른다. [구현·검증 기록](../STRUCT_IMPLEMENTATION.md). String field·init/Drop·일반 Move/borrow와 전체 D06/D10/D12/D16/D30은 후속이다.

structural Copy Tuple·numeric selector subspan·혼합 가변 경로·const·private aggregate ABI와 자원 제한은 사용자 승인 [P13](../TUPLE_STAGE_B_PROPOSAL.md)와 [40-production EBNF](../GRAMMAR_STAGE_B_TUPLE.ebnf)를 따른다. [구현·검증 기록](../TUPLE_IMPLEMENTATION.md). Array/일반 Move element와 전체 D09/D10/D12/D16은 후속이다.

사용자 승인한 Copy Enum·qualified variant·Enum/Bool statement match·coverage/binder·const·private tagged ABI는 [P14](../ENUM_STAGE_B_PROPOSAL.md), [48-production EBNF](../GRAMMAR_STAGE_B_ENUM.ebnf), [수용 fixture](../enum-proposal-fixtures/README.md), [구현 기록](../ENUM_IMPLEMENTATION.md)을 따른다. 전체 D06/D08/D09/D10/D12/D16/D25/D30은 계속 Draft다.

사용자 승인한 Copy Option<T>/Result<T,E>·T?·qualified 생성/none·문맥·match·const·private tagged ABI는 [P15 Accepted](../OPTION_RESULT_STAGE_B_PROPOSAL.md), [51-production EBNF](../GRAMMAR_STAGE_B_OPTION_RESULT.ebnf), [수용 fixture](../option-result-proposal-fixtures/README.md), [구현 기록](../OPTION_RESULT_IMPLEMENTATION.md)을 따른다. try·Move/Drop·사용자 Generic과 전체 D06/D08/D09/D10/D12/D15/D16/D23/D25/D30은 후속이다.

사용자 승인한 Copy prefix try·Result Error 조기 반환·operand 문맥 격리·정확한 E·const 금지·Source/CFG 검증은 [P16 Accepted](../TRY_STAGE_B_PROPOSAL.md), [51-production EBNF](../GRAMMAR_STAGE_B_TRY.ebnf), [수용 fixture](../try-proposal-fixtures/README.md), [구현 기록](../TRY_IMPLEMENTATION.md)을 따른다. Option try·error conversion·일반 Move/Drop와 전체 D08/D09/D10/D12/D16/D23/D25는 후속이다.

Copy struct Read instance method·contextual self·member scope/visibility·receiver-first snapshot·named/default·Source/MIR 검증은 [P22 Accepted](../METHOD_STAGE_B_PROPOSAL.md), [58-production EBNF](../GRAMMAR_STAGE_B_METHOD.ebnf), [수용 fixture](../method-proposal-fixtures/README.md)와 [구현 기록](../METHOD_IMPLEMENTATION.md)을 따른다. 구현·검증 완료다. change/take·Move/borrow/Drop·init·overload·bound method·Enum method·Array와 전체 D06/D09/D10/D11/D12/D16/D25/D30은 승인하지 않았다.

Copy struct generated constructor 이름 인수·field mapping·source-order snapshot·const/default·Source/MIR 계약은 [P23 Accepted](../STRUCT_NAMED_ARGUMENTS_STAGE_B_PROPOSAL.md)·[수용 fixture](../struct-named-arguments-proposal-fixtures/README.md)·[구현 기록](../STRUCT_NAMED_ARGUMENTS_IMPLEMENTATION.md)을 따른다. 구현·검증 완료이며 승인 P22 58-production EBNF를 변경 없이 재사용한다. explicit init·field default·Enum/sum named constructor·Array·일반 Move/Drop은 범위 밖이다.

중첩 Copy sum/tuple pattern·Unit/Copy Tuple statement match·recursive binder·matrix coverage·Source/MIR 검증은 [P24 Accepted](../NESTED_PATTERN_STAGE_B_PROPOSAL.md)·[60-production EBNF](../GRAMMAR_STAGE_B_NESTED_PATTERN.ebnf)·[수용 fixture](../nested-pattern-proposal-fixtures/README.md)와 [구현 기록](../NESTED_PATTERN_IMPLEMENTATION.md)을 따른다. 구현·검증 완료이며 P14/P15의 flat match와 P01~P23 승인 범위를 보존한다. guard/일반 literal/struct destructuring·Array·Move/loan/Drop·전체 D08/D10/D12/D16/D25/D30은 제외한다.

Copy match Bool guard·pattern-success 단일 평가·false continuation·unguarded 합집합 coverage·try/Source/MIR 계약은 [P25 Accepted](../GUARD_STAGE_B_PROPOSAL.md)·[61-production EBNF](../GRAMMAR_STAGE_B_GUARD.ebnf)·[수용 fixture](../guard-proposal-fixtures/README.md)와 [구현 기록](../GUARD_IMPLEMENTATION.md)을 따른다. 구현·검증 완료이며 P01~P24와 guard 없는 경로를 보존한다. guard truth/refinement 증명·Array·Move/loan/Drop·전체 D08/D10/D12/D16/D25/D30은 제외한다.

Irrefutable Copy local let 구조 분해·Tuple/Unit/single-variant Enum·initializer 단일 평가/snapshot·선언 시점·Source/MIR 계약은 [P26 Accepted](../DESTRUCTURING_STAGE_B_PROPOSAL.md)·[62-production EBNF](../GRAMMAR_STAGE_B_DESTRUCTURING.ebnf)·[수용 fixture](../destructuring-proposal-fixtures/README.md)와 [구현 기록](../DESTRUCTURING_IMPLEMENTATION.md)을 따른다. 구현·검증 완료이며 N3103을 등록했다. P01~P25·일반 IDENT binding을 보존하고 var/const/global/parameter/for/assignment destructuring·Array·Move/loan/Drop·전체 D08/D10/D12/D16/D25/D30은 제외한다.

가변 Copy local var 구조 분해·mutable binder·원 let/var Source identity·기존 경로 대입과 snapshot 계약은 [P27 Accepted](../VAR_DESTRUCTURING_STAGE_B_PROPOSAL.md)·[62-production EBNF](../GRAMMAR_STAGE_B_VAR_DESTRUCTURING.ebnf)·[수용 fixture](../var-destructuring-proposal-fixtures/README.md)·[구현 기록](../VAR_DESTRUCTURING_IMPLEMENTATION.md)을 따른다. 구현·검증 완료이며 P26 let/match immutable binder와 N3103/자원 한도를 보존한다. const/global/parameter/for/assignment destructuring·Array·일반 Move/borrow/Drop·전체 D08/D10/D12/D16/D25/D30은 제외한다.

함수 내부 Copy const 구조 분해·initializer 한 번 평가·whole/component cached 값·독립 const/pattern 예산·Source/Checked/MIR 계약은 [P28 Accepted](../CONST_DESTRUCTURING_STAGE_B_PROPOSAL.md)·[62-production EBNF](../GRAMMAR_STAGE_B_CONST_DESTRUCTURING.ebnf)·[수용 fixture](../const-destructuring-proposal-fixtures/README.md)·[구현 기록](../CONST_DESTRUCTURING_IMPLEMENTATION.md)을 따른다. 구현·검증 완료이며 기존 let/var·단일 const·전역 const·N3103/N3201/N3202를 보존한다. 전역/parameter/for/assignment destructuring·const function·String/Move element·Array·일반 Move/borrow/Drop·전체 D09/D10/D12/D16/D25/D30은 제외한다.

전역 Copy const 구조 분해·공유 declaration owner·component import/visibility·정적 group SCC·단일 initializer cached 값·Source/Resolved/Checked/MIR 계약은 [P29 Accepted](../GLOBAL_CONST_DESTRUCTURING_STAGE_B_PROPOSAL.md)·[62-production EBNF](../GRAMMAR_STAGE_B_GLOBAL_CONST_DESTRUCTURING.ebnf)·[수용 fixture](../global-const-destructuring-proposal-fixtures/README.md)·[구현 기록](../GLOBAL_CONST_DESTRUCTURING_IMPLEMENTATION.md)을 따른다. 기존 local 의미·61개 production·P01~P28 ledger·const node 예산을 보존한다. const function·String/Move element·Array·일반 Move/borrow/Drop·전체 D09/D10/D12/D16/D25/D30은 제외한다.

숫자·char·Copy struct statement match의 whole binder/wildcard·Bool guard·snapshot·opaque domain coverage·Source/Checked/MIR 계약은 [P30 Accepted](../COPY_VALUE_MATCH_STAGE_B_PROPOSAL.md)·[수용 fixture](../copy-value-match-proposal-fixtures/README.md)·[구현 기록](../COPY_VALUE_MATCH_IMPLEMENTATION.md)을 따른다. P29 62-production EBNF와 기존 P01~P29 의미·평가 순서·예산을 보존한다. literal/field/or/range pattern·Array·String/Move·match expression·일반 Move/borrow/Drop·전체 D08/D10/D12/D16/D25/D30은 제외한다.

Unicode char literal 패턴·decoded scalar alias·observed/complement matrix coverage·guard·refutable binding·Source/Checked/MIR 계약은 [P31 Accepted](../CHAR_PATTERN_STAGE_B_PROPOSAL.md)·[62-production EBNF](../GRAMMAR_STAGE_B_CHAR_PATTERN.ebnf)·[수용 fixture](../char-pattern-proposal-fixtures/README.md)·[구현 기록](../CHAR_PATTERN_IMPLEMENTATION.md)을 따른다. 기존 61개 production·P01~P30 ledger·opaque-only char witness·Copy snapshot/guard 순서/예산을 보존한다. integer/float/string literal·field/or/range pattern·Array·String/Move·일반 Move/borrow/Drop·전체 D08/D10/D12/D16/D25/D30은 제외한다.

고정 폭 정수 literal 패턴·component 기대 타입/checked range·radix/zero alias·finite/observed-complement matrix·guard·refutable binding·Source/Checked/MIR 계약은 [P32 Accepted](../INTEGER_PATTERN_STAGE_B_PROPOSAL.md)·[63-production EBNF](../GRAMMAR_STAGE_B_INTEGER_PATTERN.ebnf)·[수용 fixture](../integer-pattern-proposal-fixtures/README.md)·[구현 기록](../INTEGER_PATTERN_IMPLEMENTATION.md)을 따른다. 기존 61개 production·P01~P31 ledger·opaque-only witness·snapshot/guard 순서/예산을 보존한다. float/string literal·field/or/range pattern·Array·String/Move·일반 Move/borrow/Drop·전체 D07/D08/D10/D12/D16/D25/D30은 제외한다.

Copy struct 위치 패턴·nominal/import head·전체 필드 순서/가시성·let/var/const 분해·bounded coverage·snapshot/guard·Source/Checked/MIR 계약은 [P33 Accepted](../STRUCT_PATTERN_STAGE_B_PROPOSAL.md)·[64-production EBNF](../GRAMMAR_STAGE_B_STRUCT_PATTERN.ebnf)·[수용 fixture](../struct-pattern-proposal-fixtures/README.md)·[구현 기록](../STRUCT_PATTERN_IMPLEMENTATION.md)을 따른다. 기존 61개 production·P01~P32 ledger·opaque-only witness·평가 순서/예산을 보존한다. 이름 field/rest/or/range 패턴·Array·String/Move field·일반 Move/borrow/Drop·전체 D06/D08/D09/D10/D12/D16/D25/D30은 제외한다.

Copy struct 이름 필드 패턴·역순/위치 prefix label 대응·Source 순서/선언 field 순서 plan·전체 필드 가시성·transparent wrapper·const/coverage·Source/Checked/MIR 계약은 [P34 Accepted](../STRUCT_NAMED_PATTERN_STAGE_B_PROPOSAL.md)·[66-production EBNF](../GRAMMAR_STAGE_B_STRUCT_NAMED_PATTERN.ebnf)·[수용 fixture](../struct-named-pattern-proposal-fixtures/README.md)·[구현 기록](../STRUCT_NAMED_PATTERN_IMPLEMENTATION.md)을 따른다. 기존 63개 production·P01~P33 ledger·평가 순서/예산을 보존한다. rest/shorthand/부분 필드·Enum/sum/Tuple 이름 payload·Array·String/Move field·일반 Move/borrow/Drop·전체 D06/D08/D09/D10/D12/D16/D25/D30은 제외한다.

Copy struct trailing rest 패턴·부분 field/명시 field_indices·전체 initializer 평가·기존 private 가시성·implicit wildcard coverage·sparse const/MIR·Source 검증은 [P35 Accepted](../STRUCT_REST_PATTERN_STAGE_B_PROPOSAL.md)·[67-production EBNF](../GRAMMAR_STAGE_B_STRUCT_REST_PATTERN.ebnf)·[수용 fixture](../struct-rest-pattern-proposal-fixtures/README.md)·[구현 기록](../STRUCT_REST_PATTERN_IMPLEMENTATION.md)을 따른다. 기존 65개 production·P01~P34 ledger·평가 순서/예산을 보존한다. shorthand·Tuple/Enum payload rest·private 접근 완화·Array·일반 Move/borrow/Drop·전체 D06/D08/D09/D10/D12/D16/D25/D30은 제외한다.

Copy Tuple trailing rest 패턴·명시 prefix/field_indices·Unit 구분·전체 initializer 평가·implicit wildcard coverage·prefix const/MIR·Source 검증은 [P36 Accepted](../TUPLE_REST_PATTERN_STAGE_B_PROPOSAL.md)·[68-production EBNF](../GRAMMAR_STAGE_B_TUPLE_REST_PATTERN.ebnf)·[수용 fixture](../tuple-rest-pattern-proposal-fixtures/README.md)·[구현 기록](../TUPLE_REST_PATTERN_IMPLEMENTATION.md)을 따른다. 기존 66개 production·P01~P35 ledger·평가 순서/예산을 보존한다. middle rest·Enum payload rest·Array·일반 Move/borrow/Drop·전체 D06/D08/D09/D10/D12/D16/D25/D30은 제외한다.

Copy Enum·sum payload trailing rest 패턴·explicit prefix ordinal·nullary 괄호 금지·tag 보존·전체 initializer 평가·implicit wildcard coverage·const/MIR·Source 검증은 [P37 Accepted](../PAYLOAD_REST_PATTERN_STAGE_B_PROPOSAL.md)·[69-production EBNF](../GRAMMAR_STAGE_B_PAYLOAD_REST_PATTERN.ebnf)·[수용 fixture](../payload-rest-pattern-proposal-fixtures/README.md)·[구현 기록](../PAYLOAD_REST_PATTERN_IMPLEMENTATION.md)을 따른다. 기존 67개 production·P01~P36 ledger·평가 순서/예산을 보존한다. middle rest·named payload·Array·일반 Move/borrow/Drop·전체 D06/D08/D09/D10/D12/D16/D25/D30은 제외한다.

Copy Tuple 중간 rest 패턴·prefix/suffix 실제 field_indices·빈 생략·실제 annotation·전체 initializer 평가·const/MIR·Source 검증은 [P38 Accepted](../TUPLE_MIDDLE_REST_STAGE_B_PROPOSAL.md)·[70-production EBNF](../GRAMMAR_STAGE_B_TUPLE_MIDDLE_REST.ebnf)·[수용 fixture](../tuple-middle-rest-proposal-fixtures/README.md)·[구현 기록](../TUPLE_MIDDLE_REST_IMPLEMENTATION.md)을 따른다. 기존 68개 production·P01~P37 ledger·평가 순서/예산을 보존한다. struct/Enum payload middle rest·rest binder·Array·일반 Move/borrow/Drop·전체 D06/D08/D09/D10/D12/D16/D25/D30은 제외한다.

Copy Enum·sum payload 중간 rest 패턴·prefix/suffix 실제 field_indices·빈 생략·실제 annotation·전체 initializer 평가·const/MIR·Source 검증은 [P39 Accepted](../PAYLOAD_MIDDLE_REST_STAGE_B_PROPOSAL.md)·[70-production EBNF](../GRAMMAR_STAGE_B_PAYLOAD_MIDDLE_REST.ebnf)·[수용 fixture](../payload-middle-rest-proposal-fixtures/README.md)·[구현 기록](../PAYLOAD_MIDDLE_REST_IMPLEMENTATION.md)을 따른다. 기존 69개 production·P01~P38 ledger·평가 순서/예산을 보존한다. struct middle rest·rest binder·Array·일반 Move/borrow/Drop·전체 D06/D08/D09/D10/D12/D16/D25/D30은 제외한다.

## Layout lowering
core Layout를 LLVM DataLayout와 대조한다. field offsets/align/padding/tag/payload를 하나의 Target query 결과로 계산하고 backend의 임의 재배치를 금지한다.

## Niche
valid bit pattern 밖의 representation만 사용한다. optimization enable/disable이 discriminant test/Drop 의미를 바꾸지 않아야 한다. foreign representation은 explicit ABI-safe mapping으로 제한한다.

## 검증
sizeof/alignof/offset golden, LLVM structural type padding, over-aligned allocation, zero-sized element, enum maximum payload, C roundtrip harness.
