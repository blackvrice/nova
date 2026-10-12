# NOVA-035 — 함수·메서드·호출 규약·Receiver 사양서

| 항목 | 값 |
|---|---|
| 대상 | Nova 0.1 |
| 작성일 | 2026-10-03 |
| 문서 상태 | Draft — 기존 결정은 유지, 추가 상세는 승인 대기 |
| 적용 Stage | A~B |
| 근거 | [원본 NOVA-035](../../04_Functions_Control/NOVA-035_함수_메서드_호출_규약_Receiver_사양서.md) |

이 문서는 원본을 대체하는 확정 사양이 아니다. 기존 확정 기준은 [CANONICAL](../CANONICAL.md)을 따르며, 새 의미·문법·API·정책은 [DECISIONS](../DECISIONS.md)의 승인이 필요하다. D01~D05 Lexer 상세는 [승인 기준](../ACCEPTED_LEXER.md)을 따른다. [전체 색인](../INDEX.md).

Stage A Parser 구문/복구는 사용자 승인 [P01](../PARSER_STAGE_A_PROPOSAL.md)과 [전용 EBNF](../GRAMMAR_STAGE_A.ebnf)가 우선한다. 나머지 추가 상세는 Draft다.

Stage A 단일 파일 의미 검사는 사용자 승인 [P02](../SEMANTICS_STAGE_A_PROPOSAL.md)가 우선한다. runtime/전체 타입/미래 Stage의 추가 상세는 Draft다.

사용자 승인한 함수 이름 인수·parameter mapping·source-order snapshot/try·진단 계약은 [P17 Accepted](../NAMED_ARGUMENTS_STAGE_B_PROPOSAL.md), [52-production EBNF](../GRAMMAR_STAGE_B_NAMED_ARGUMENTS.ebnf), [수용 fixture](../named-arguments-proposal-fixtures/README.md), [구현 기록](../NAMED_ARGUMENTS_IMPLEMENTATION.md)을 따른다. 구현·검증 완료이며 기본 인수·overload·named constructor와 전체 D11/D16/D25/D30 승인이 아니다.

사용자 승인한 상수 표현식 함수 기본 인수·declaration scope·caller materialization·생략 인수 대응·상수 실패/예산 계약은 [P18 Accepted](../DEFAULT_ARGUMENTS_STAGE_B_PROPOSAL.md), [52-production EBNF](../GRAMMAR_STAGE_B_DEFAULT_ARGUMENTS.ebnf), [수용 fixture](../default-arguments-proposal-fixtures/README.md), [구현 기록](../DEFAULT_ARGUMENTS_IMPLEMENTATION.md)을 따른다. 구현·검증 완료이며 runtime/parameter 의존 default·overload·named constructor와 전체 D09/D11/D16/D25/D30 승인이 아니다.

Copy struct Read instance method·contextual self·member scope/visibility·receiver-first snapshot·named/default·Source/MIR 검증은 [P22 Accepted](../METHOD_STAGE_B_PROPOSAL.md), [58-production EBNF](../GRAMMAR_STAGE_B_METHOD.ebnf), [수용 fixture](../method-proposal-fixtures/README.md)와 [구현 기록](../METHOD_IMPLEMENTATION.md)을 따른다. 구현·검증 완료다. change/take·Move/borrow/Drop·init·overload·bound method·Enum method·Array와 전체 D06/D09/D10/D11/D12/D16/D25/D30은 승인하지 않았다.

Copy struct generated constructor 이름 인수·field mapping·source-order snapshot·const/default·Source/MIR 계약은 [P23 Accepted](../STRUCT_NAMED_ARGUMENTS_STAGE_B_PROPOSAL.md)·[수용 fixture](../struct-named-arguments-proposal-fixtures/README.md)·[구현 기록](../STRUCT_NAMED_ARGUMENTS_IMPLEMENTATION.md)을 따른다. 구현·검증 완료이며 승인 P22 58-production EBNF를 변경 없이 재사용한다. explicit init·field default·Enum/sum named constructor·Array·일반 Move/Drop은 범위 밖이다.

Irrefutable Copy local let 구조 분해·Tuple/Unit/single-variant Enum·initializer 단일 평가/snapshot·선언 시점·Source/MIR 계약은 [P26 Accepted](../DESTRUCTURING_STAGE_B_PROPOSAL.md)·[62-production EBNF](../GRAMMAR_STAGE_B_DESTRUCTURING.ebnf)·[수용 fixture](../destructuring-proposal-fixtures/README.md)와 [구현 기록](../DESTRUCTURING_IMPLEMENTATION.md)을 따른다. 구현·검증 완료이며 N3103을 등록했다. P01~P25·일반 IDENT binding을 보존하고 var/const/global/parameter/for/assignment destructuring·Array·Move/loan/Drop·전체 D08/D10/D12/D16/D25/D30은 제외한다.

가변 Copy local var 구조 분해·mutable binder·원 let/var Source identity·기존 경로 대입과 snapshot 계약은 [P27 Accepted](../VAR_DESTRUCTURING_STAGE_B_PROPOSAL.md)·[62-production EBNF](../GRAMMAR_STAGE_B_VAR_DESTRUCTURING.ebnf)·[수용 fixture](../var-destructuring-proposal-fixtures/README.md)·[구현 기록](../VAR_DESTRUCTURING_IMPLEMENTATION.md)을 따른다. 구현·검증 완료이며 P26 let/match immutable binder와 N3103/자원 한도를 보존한다. const/global/parameter/for/assignment destructuring·Array·일반 Move/borrow/Drop·전체 D08/D10/D12/D16/D25/D30은 제외한다.

함수 내부 Copy const 구조 분해·initializer 한 번 평가·whole/component cached 값·독립 const/pattern 예산·Source/Checked/MIR 계약은 [P28 Accepted](../CONST_DESTRUCTURING_STAGE_B_PROPOSAL.md)·[62-production EBNF](../GRAMMAR_STAGE_B_CONST_DESTRUCTURING.ebnf)·[수용 fixture](../const-destructuring-proposal-fixtures/README.md)·[구현 기록](../CONST_DESTRUCTURING_IMPLEMENTATION.md)을 따른다. 구현·검증 완료이며 기존 let/var·단일 const·전역 const·N3103/N3201/N3202를 보존한다. 전역/parameter/for/assignment destructuring·const function·String/Move element·Array·일반 Move/borrow/Drop·전체 D09/D10/D12/D16/D25/D30은 제외한다.

## Function signature
FunctionId, ReceiverMode, ParameterMode, ReturnType/ownership, Effects를 포함한다. modifier 부재는 Read, change는 exclusive loan, take는 이전이다. 인수는 소스 순서로 평가하고 return type만으로 overload하지 않는다.

## Receiver 초안 — D11
member function의 receiver를 func name(read/change/take self, ...)처럼 별도 read 키워드로 추가하지 않는다. 이 초안은 func name(self, ...), func name(change self, ...), func name(take self, ...)를 제안하며 self는 contextual binding이다.

## 반환
Unit 반환은 생략/void 정규화다. owned 값 반환은 owner를 caller로 이전한다. view 반환은 parameter-origin lifetime 계약이 필요하며 local owner를 가리키는 반환은 거부한다.

## 검증
mode mismatch, named argument 순서, early return cleanup, method mutation with read receiver fail, overload return-only conflict를 검사한다.
