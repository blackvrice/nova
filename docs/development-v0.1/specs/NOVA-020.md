# NOVA-020 — Module·Import 사양서

| 항목 | 값 |
|---|---|
| 대상 | Nova 0.1 |
| 작성일 | 2026-10-03 |
| 문서 상태 | Draft — 기존 결정은 유지, 추가 상세는 승인 대기 |
| 적용 Stage | B~E |
| 근거 | [원본 NOVA-020](../../02_Names_Modules/NOVA-020_Module_Import_사양서.md) |

이 문서는 원본을 대체하는 확정 사양이 아니다. 기존 확정 기준은 [CANONICAL](../CANONICAL.md)을 따르며, 새 의미·문법·API·정책은 [DECISIONS](../DECISIONS.md)의 승인이 필요하다. D01~D05 Lexer 상세는 [승인 기준](../ACCEPTED_LEXER.md)을 따른다. [전체 색인](../INDEX.md).

root-relative 함수/전역 const item import·alias·internal/private/public·reachable graph·cross-file const·entry/source identity는 사용자 승인 [P11](../MODULE_STAGE_B_PROPOSAL.md)과 [전용 EBNF](../GRAMMAR_STAGE_B_MODULE.ebnf)를 따른다. [구현·검증 기록](../MODULE_IMPLEMENTATION.md). module alias/qualified value/reexport/Package와 전체 D06/D30은 후속이다.

nominal Copy struct·위치 생성·type import·가변 field 경로·const·private layout/ABI와 자원 제한은 사용자 승인 [P12](../STRUCT_STAGE_B_PROPOSAL.md)와 [37-production EBNF](../GRAMMAR_STAGE_B_STRUCT.ebnf)를 따른다. [구현·검증 기록](../STRUCT_IMPLEMENTATION.md). String field·init/Drop·일반 Move/borrow와 전체 D06/D10/D12/D16/D30은 후속이다.

비제네릭 transparent type alias·type 위치·forward/import·cycle/자원·Source/MIR 검증은 [P21 Draft](../ALIAS_STAGE_B_PROPOSAL.md), [55-production EBNF](../GRAMMAR_STAGE_B_ALIAS.ebnf), [제안 fixture](../alias-proposal-fixtures/README.md)로 준비했다. 미승인·미구현이다. type은 D01에서 이미 keyword이며 전체 D06/D10/D11/D12/D16/D30·generic alias/newtype·alias constructor/variant·API leak/export 정책은 승인하지 않았다.

## Module 구조
원본 기준은 파일 경로에서 module 경로 결정, 순환 참조 허용, 순환 초기화 금지다. package source root, segment normalization, import grammar는 D01/D06 초안이다.

## 제안 계약
src/a/b.nova → package::a::b, src/main.nova는 binary root. use path [as alias]를 제안하고 wildcard는 첫 승인안에서 제외한다. 동일 경로 대소문자 충돌은 Windows/Linux 공통에서 오류로 제안한다. public use의 재export는 visibility보다 넓어질 수 없다.

## Graph 처리
모든 파일의 선언을 먼저 수집하고 SCC 단위로 이름을 확정한다. top-level runtime initializer는 첫 승인안에서 금지하며 const dependency cycle은 NOVA-032 오류다.

## 검증
mutual function call pass, 순환 const fail, 경로 충돌 fail, alias와 reexport access 검사. Stage A는 단일 파일이며 module feature를 흉내 내지 않는다.
