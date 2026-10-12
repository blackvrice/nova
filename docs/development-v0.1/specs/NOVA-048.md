# NOVA-048 — Match Lowering·Decision Tree 사양서

| 항목 | 값 |
|---|---|
| 대상 | Nova 0.1 |
| 작성일 | 2026-10-03 |
| 문서 상태 | Draft — 기존 결정은 유지, 추가 상세는 승인 대기 |
| 적용 Stage | A~B |
| 근거 | [원본 NOVA-048](../../04_Functions_Control/NOVA-048_Match_Lowering_Decision_Tree_사양서.md) |

이 문서는 원본을 대체하는 확정 사양이 아니다. 기존 확정 기준은 [CANONICAL](../CANONICAL.md)을 따르며, 새 의미·문법·API·정책은 [DECISIONS](../DECISIONS.md)의 승인이 필요하다. D01~D05 Lexer 상세는 [승인 기준](../ACCEPTED_LEXER.md)을 따른다. [전체 색인](../INDEX.md).

사용자 승인한 Copy Enum·qualified variant·Enum/Bool statement match·coverage/binder·const·private tagged ABI는 [P14](../ENUM_STAGE_B_PROPOSAL.md), [48-production EBNF](../GRAMMAR_STAGE_B_ENUM.ebnf), [수용 fixture](../enum-proposal-fixtures/README.md), [구현 기록](../ENUM_IMPLEMENTATION.md)을 따른다. 전체 D06/D08/D09/D10/D12/D16/D25/D30은 계속 Draft다.

사용자 승인한 Copy Option<T>/Result<T,E>·T?·qualified 생성/none·문맥·match·const·private tagged ABI는 [P15 Accepted](../OPTION_RESULT_STAGE_B_PROPOSAL.md), [51-production EBNF](../GRAMMAR_STAGE_B_OPTION_RESULT.ebnf), [수용 fixture](../option-result-proposal-fixtures/README.md), [구현 기록](../OPTION_RESULT_IMPLEMENTATION.md)을 따른다. try·Move/Drop·사용자 Generic과 전체 D06/D08/D09/D10/D12/D15/D16/D23/D25/D30은 후속이다.

사용자 승인한 Copy prefix try·Result Error 조기 반환·operand 문맥 격리·정확한 E·const 금지·Source/CFG 검증은 [P16 Accepted](../TRY_STAGE_B_PROPOSAL.md), [51-production EBNF](../GRAMMAR_STAGE_B_TRY.ebnf), [수용 fixture](../try-proposal-fixtures/README.md), [구현 기록](../TRY_IMPLEMENTATION.md)을 따른다. Option try·error conversion·일반 Move/Drop와 전체 D08/D09/D10/D12/D16/D23/D25는 후속이다.

중첩 Copy sum/tuple pattern·Unit/Copy Tuple statement match·recursive binder·matrix coverage·Source/MIR 검증은 [P24 Accepted](../NESTED_PATTERN_STAGE_B_PROPOSAL.md)·[60-production EBNF](../GRAMMAR_STAGE_B_NESTED_PATTERN.ebnf)·[수용 fixture](../nested-pattern-proposal-fixtures/README.md)와 [구현 기록](../NESTED_PATTERN_IMPLEMENTATION.md)을 따른다. 구현·검증 완료이며 P14/P15의 flat match와 P01~P23 승인 범위를 보존한다. guard/일반 literal/struct destructuring·Array·Move/loan/Drop·전체 D08/D10/D12/D16/D25/D30은 제외한다.

Copy match Bool guard·pattern-success 단일 평가·false continuation·unguarded 합집합 coverage·try/Source/MIR 계약은 [P25 Accepted](../GUARD_STAGE_B_PROPOSAL.md)·[61-production EBNF](../GRAMMAR_STAGE_B_GUARD.ebnf)·[수용 fixture](../guard-proposal-fixtures/README.md)와 [구현 기록](../GUARD_IMPLEMENTATION.md)을 따른다. 구현·검증 완료이며 P01~P24와 guard 없는 경로를 보존한다. guard truth/refinement 증명·Array·Move/loan/Drop·전체 D08/D10/D12/D16/D25/D30은 제외한다.

Irrefutable Copy local let 구조 분해·Tuple/Unit/single-variant Enum·initializer 단일 평가/snapshot·선언 시점·Source/MIR 계약은 [P26 Accepted](../DESTRUCTURING_STAGE_B_PROPOSAL.md)·[62-production EBNF](../GRAMMAR_STAGE_B_DESTRUCTURING.ebnf)·[수용 fixture](../destructuring-proposal-fixtures/README.md)와 [구현 기록](../DESTRUCTURING_IMPLEMENTATION.md)을 따른다. 구현·검증 완료이며 N3103을 등록했다. P01~P25·일반 IDENT binding을 보존하고 var/const/global/parameter/for/assignment destructuring·Array·Move/loan/Drop·전체 D08/D10/D12/D16/D25/D30은 제외한다.

가변 Copy local var 구조 분해·mutable binder·원 let/var Source identity·기존 경로 대입과 snapshot 계약은 [P27 Accepted](../VAR_DESTRUCTURING_STAGE_B_PROPOSAL.md)·[62-production EBNF](../GRAMMAR_STAGE_B_VAR_DESTRUCTURING.ebnf)·[수용 fixture](../var-destructuring-proposal-fixtures/README.md)·[구현 기록](../VAR_DESTRUCTURING_IMPLEMENTATION.md)을 따른다. 구현·검증 완료이며 P26 let/match immutable binder와 N3103/자원 한도를 보존한다. const/global/parameter/for/assignment destructuring·Array·일반 Move/borrow/Drop·전체 D08/D10/D12/D16/D25/D30은 제외한다.

함수 내부 Copy const 구조 분해·initializer 한 번 평가·whole/component cached 값·독립 const/pattern 예산·Source/Checked/MIR 계약은 [P28 Accepted](../CONST_DESTRUCTURING_STAGE_B_PROPOSAL.md)·[62-production EBNF](../GRAMMAR_STAGE_B_CONST_DESTRUCTURING.ebnf)·[수용 fixture](../const-destructuring-proposal-fixtures/README.md)·[구현 기록](../CONST_DESTRUCTURING_IMPLEMENTATION.md)을 따른다. 구현·검증 완료이며 기존 let/var·단일 const·전역 const·N3103/N3201/N3202를 보존한다. 전역/parameter/for/assignment destructuring·const function·String/Move element·Array·일반 Move/borrow/Drop·전체 D09/D10/D12/D16/D25/D30은 제외한다.

전역 Copy const 구조 분해·공유 declaration owner·component import/visibility·정적 group SCC·단일 initializer cached 값·Source/Resolved/Checked/MIR 계약은 [P29 Accepted](../GLOBAL_CONST_DESTRUCTURING_STAGE_B_PROPOSAL.md)·[62-production EBNF](../GRAMMAR_STAGE_B_GLOBAL_CONST_DESTRUCTURING.ebnf)·[수용 fixture](../global-const-destructuring-proposal-fixtures/README.md)·[구현 기록](../GLOBAL_CONST_DESTRUCTURING_IMPLEMENTATION.md)을 따른다. 기존 local 의미·61개 production·P01~P28 ledger·const node 예산을 보존한다. const function·String/Move element·Array·일반 Move/borrow/Drop·전체 D09/D10/D12/D16/D25/D30은 제외한다.

숫자·char·Copy struct statement match의 whole binder/wildcard·Bool guard·snapshot·opaque domain coverage·Source/Checked/MIR 계약은 [P30 Accepted](../COPY_VALUE_MATCH_STAGE_B_PROPOSAL.md)·[수용 fixture](../copy-value-match-proposal-fixtures/README.md)·[구현 기록](../COPY_VALUE_MATCH_IMPLEMENTATION.md)을 따른다. P29 62-production EBNF와 기존 P01~P29 의미·평가 순서·예산을 보존한다. literal/field/or/range pattern·Array·String/Move·match expression·일반 Move/borrow/Drop·전체 D08/D10/D12/D16/D25/D30은 제외한다.

Unicode char literal 패턴·decoded scalar alias·observed/complement matrix coverage·guard·refutable binding·Source/Checked/MIR 계약은 [P31 Accepted](../CHAR_PATTERN_STAGE_B_PROPOSAL.md)·[62-production EBNF](../GRAMMAR_STAGE_B_CHAR_PATTERN.ebnf)·[수용 fixture](../char-pattern-proposal-fixtures/README.md)·[구현 기록](../CHAR_PATTERN_IMPLEMENTATION.md)을 따른다. 기존 61개 production·P01~P30 ledger·opaque-only char witness·Copy snapshot/guard 순서/예산을 보존한다. integer/float/string literal·field/or/range pattern·Array·String/Move·일반 Move/borrow/Drop·전체 D08/D10/D12/D16/D25/D30은 제외한다.

## Decision tree lowering
scrutinee는 한 번 평가하여 local/place에 저장한다. variant tag/literal test를 공유하되 arm 순서, guard side effect, loan lifetime은 보존한다. payload는 variant 검사가 성공한 뒤만 접근한다.

## CFG 계약
각 arm entry/binding/guard/body/join block을 구분한다. guard false는 다음 arm으로 이동하고 guard 임시값 cleanup을 수행한다. early return/try는 enclosing cleanup으로 연결한다.

## 검증
scrutinee call count=1, guard 순서 trace, inactive payload 접근 없음, source arm order가 최적화 뒤에도 동일한지 검사한다. exhaustive analysis 결과가 없으면 codegen으로 넘기지 않는다.
