# NOVA-023 — Package 간 이름 해석 사양서

| 항목 | 값 |
|---|---|
| 대상 | Nova 0.1 |
| 작성일 | 2026-10-03 |
| 문서 상태 | Draft — 기존 결정은 유지, 추가 상세는 승인 대기 |
| 적용 Stage | B~E |
| 근거 | [원본 NOVA-023](../../02_Names_Modules/NOVA-023_Package_간_이름_해석_사양서.md) |

이 문서는 원본을 대체하는 확정 사양이 아니다. 기존 확정 기준은 [CANONICAL](../CANONICAL.md)을 따르며, 새 의미·문법·API·정책은 [DECISIONS](../DECISIONS.md)의 승인이 필요하다. D01~D05 Lexer 상세는 [승인 기준](../ACCEPTED_LEXER.md)을 따른다. [전체 색인](../INDEX.md).

root-relative 함수/전역 const item import·alias·internal/private/public·reachable graph·cross-file const·entry/source identity는 사용자 승인 [P11](../MODULE_STAGE_B_PROPOSAL.md)과 [전용 EBNF](../GRAMMAR_STAGE_B_MODULE.ebnf)를 따른다. [구현·검증 기록](../MODULE_IMPLEMENTATION.md). module alias/qualified value/reexport/Package와 전체 D06/D30은 후속이다.

nominal Copy struct·위치 생성·type import·가변 field 경로·const·private layout/ABI와 자원 제한은 사용자 승인 [P12](../STRUCT_STAGE_B_PROPOSAL.md)와 [37-production EBNF](../GRAMMAR_STAGE_B_STRUCT.ebnf)를 따른다. [구현·검증 기록](../STRUCT_IMPLEMENTATION.md). String field·init/Drop·일반 Move/borrow와 전체 D06/D10/D12/D16/D30은 후속이다.

비제네릭 transparent type alias·type 위치·forward/import·cycle/자원·Source/MIR 검증은 [P21 Accepted](../ALIAS_STAGE_B_PROPOSAL.md), [55-production EBNF](../GRAMMAR_STAGE_B_ALIAS.ebnf), [수용 fixture](../alias-proposal-fixtures/README.md)를 따른다. [구현 기록](../ALIAS_IMPLEMENTATION.md)에 검증을 기록했다. type은 D01에서 이미 keyword이며 전체 D06/D10/D11/D12/D16/D30·generic alias/newtype·alias constructor/variant·API leak/export 정책은 승인하지 않았다.

Copy struct Read instance method·contextual self·member scope/visibility·receiver-first snapshot·named/default·Source/MIR 검증은 [P22 Accepted](../METHOD_STAGE_B_PROPOSAL.md), [58-production EBNF](../GRAMMAR_STAGE_B_METHOD.ebnf), [수용 fixture](../method-proposal-fixtures/README.md)와 [구현 기록](../METHOD_IMPLEMENTATION.md)을 따른다. 구현·검증 완료다. change/take·Move/borrow/Drop·init·overload·bound method·Enum method·Array와 전체 D06/D09/D10/D11/D12/D16/D25/D30은 승인하지 않았다.

Copy struct generated constructor 이름 인수·field mapping·source-order snapshot·const/default·Source/MIR 계약은 [P23 Accepted](../STRUCT_NAMED_ARGUMENTS_STAGE_B_PROPOSAL.md)·[수용 fixture](../struct-named-arguments-proposal-fixtures/README.md)·[구현 기록](../STRUCT_NAMED_ARGUMENTS_IMPLEMENTATION.md)을 따른다. 구현·검증 완료이며 승인 P22 58-production EBNF를 변경 없이 재사용한다. explicit init·field default·Enum/sum named constructor·Array·일반 Move/Drop은 범위 밖이다.

## Package 경계
package identity에는 이름만이 아니라 source와 정확한 version/revision을 포함한다. 동일 이름 다른 source package를 하나로 합치지 않는다. 외부 package 참조는 NOVA-126/127의 resolved graph를 입력으로 받는다.

## Import 계약 초안 — D06/D21
dependency alias를 root segment로 사용한다. export metadata만 접근 가능하고 private/internal을 외부에서 해석하지 않는다. 동일 alias의 복수 dependency는 manifest 단계에서 오류다.

## 구현
compiler resolver는 network를 직접 조회하지 않는다. package 계층이 검증한 source/export artifact를 넘긴다. metadata compiler/ABI/schema mismatch는 source rebuild 또는 명시 오류다.

## 검증
version 다중 공존, alias collision, private symbol 거부, lock된 revision 교체 탐지, import 순서 독립성을 확인한다.
