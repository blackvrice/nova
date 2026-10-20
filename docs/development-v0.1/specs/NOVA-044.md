# NOVA-044 — break·continue·return 사양서

| 항목 | 값 |
|---|---|
| 대상 | Nova 0.1 |
| 작성일 | 2026-10-03 |
| 문서 상태 | Draft — 기존 결정은 유지, 추가 상세는 승인 대기 |
| 적용 Stage | A~B |
| 근거 | [원본 NOVA-044](../../04_Functions_Control/NOVA-044_break_continue_return_사양서.md) |

이 문서는 원본을 대체하는 확정 사양이 아니다. 기존 확정 기준은 [CANONICAL](../CANONICAL.md)을 따르며, 새 의미·문법·API·정책은 [DECISIONS](../DECISIONS.md)의 승인이 필요하다. D01~D05 Lexer 상세는 [승인 기준](../ACCEPTED_LEXER.md)을 따른다. [전체 색인](../INDEX.md).

Stage A Parser 구문/복구는 사용자 승인 [P01](../PARSER_STAGE_A_PROPOSAL.md)과 [전용 EBNF](../GRAMMAR_STAGE_A.ebnf)가 우선한다. 나머지 추가 상세는 Draft다.

Stage A 단일 파일 의미 검사는 사용자 승인 [P02](../SEMANTICS_STAGE_A_PROPOSAL.md)가 우선한다. runtime/전체 타입/미래 Stage의 추가 상세는 Draft다.

Stage B 가변 지역 변수·반복문 최소 부분은 사용자 진행 요청으로 승인한 [P04](../CONTROL_STAGE_B_PROPOSAL.md)와 [전용 EBNF](../GRAMMAR_STAGE_B_CONTROL.ebnf)가 우선한다. [구현·검증 기록](../CONTROL_IMPLEMENTATION.md). 전체 Stage B와 ownership/Drop 정책은 Draft다.

사용자 승인한 Copy prefix try·Result Error 조기 반환·operand 문맥 격리·정확한 E·const 금지·Source/CFG 검증은 [P16 Accepted](../TRY_STAGE_B_PROPOSAL.md), [51-production EBNF](../GRAMMAR_STAGE_B_TRY.ebnf), [수용 fixture](../try-proposal-fixtures/README.md), [구현 기록](../TRY_IMPLEMENTATION.md)을 따른다. Option try·error conversion·일반 Move/Drop와 전체 D08/D09/D10/D12/D16/D23/D25는 후속이다.

사용자 승인한 loop·정수 범위 for는 [P19 Accepted](../RANGE_LOOP_STAGE_B_PROPOSAL.md), [54-production EBNF](../GRAMMAR_STAGE_B_RANGE_LOOP.ebnf), [수용 fixture](../range-loop-proposal-fixtures/README.md)를 따른다. 구현·검증 완료이며 [구현 기록](../RANGE_LOOP_IMPLEMENTATION.md)을 제공한다. 기존 P01~P18과 일반 iterable/Array/Move/Drop 경계는 보존한다.

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

## Jump 의미
return은 enclosing function, break/continue는 가장 가까운 loop에 대응한다. lambda 내부 jump가 바깥 function/loop를 벗어나지 않는다. labeled jump 문법은 이번 초안에서 제외한다.

## 초안 — D08
Unit 함수의 bare return 허용, 값 return은 declared type에 check한다. non-Unit 함수의 reachable fallthrough는 오류다. break value를 지원하지 않는다. return expression은 이동/평가 후 local cleanup을 실행한다.

## 검증
loop 밖 break/continue fail, function 밖 return fail, 한 branch return 누락 fail, never callee 뒤 unreachable, return된 owner가 local Drop로 파괴되지 않는지 확인한다. END의 return newline 의미는 D05와 일치해야 한다.
