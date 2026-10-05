# NOVA-078 — 진단 시스템·Error Code 관리 사양서

| 항목 | 값 |
|---|---|
| 대상 | Nova 0.1 |
| 작성일 | 2026-10-03 |
| 문서 상태 | Draft — 기존 결정은 유지, 추가 상세는 승인 대기 |
| 적용 Stage | A~E |
| 근거 | [원본 NOVA-078](../../07_Compiler_Frontend/NOVA-078_진단_시스템_Error_Code_관리_사양서.md) |

이 문서는 원본을 대체하는 확정 사양이 아니다. 기존 확정 기준은 [CANONICAL](../CANONICAL.md)을 따르며, 새 의미·문법·API·정책은 [DECISIONS](../DECISIONS.md)의 승인이 필요하다. D01~D05 Lexer 상세는 [승인 기준](../ACCEPTED_LEXER.md)을 따른다. [전체 색인](../INDEX.md).

단일 파일 전역 const·forward dependency/cycle 최소 부분은 사용자 승인 [P06](../GLOBAL_CONST_STAGE_B_PROPOSAL.md)와 [전용 EBNF](../GRAMMAR_STAGE_B_GLOBAL_CONST.ebnf)를 따른다. [구현·검증 기록](../GLOBAL_CONST_IMPLEMENTATION.md). 함수 print shadow는 기존 P02대로 허용하고 전역 const print만 N2002다. module/const function/전체 D06·D09는 Draft다.

숫자 10종의 postfix as·operand literal 문맥 격리·checked 범위/직접 RN 반올림·float truncation·const N3201/Runtime Abort는 사용자 승인 [P10](../CAST_STAGE_B_PROPOSAL.md)과 [전용 EBNF](../GRAMMAR_STAGE_B_CAST.ebnf)를 따른다. [구현·검증 기록](../CAST_IMPLEMENTATION.md). Bool/Char/unsafe cast와 전체 D07은 Draft다.

root-relative 함수/전역 const item import·alias·internal/private/public·reachable graph·cross-file const·entry/source identity는 사용자 승인 [P11](../MODULE_STAGE_B_PROPOSAL.md)과 [전용 EBNF](../GRAMMAR_STAGE_B_MODULE.ebnf)를 따른다. [구현·검증 기록](../MODULE_IMPLEMENTATION.md). module alias/qualified value/reexport/Package와 전체 D06/D30은 후속이다.

## Diagnostic 구조
code/severity/message/primary/secondary/notes/suggestions를 보존한다. N1xxx syntax, N2xxx name/type, N3xxx control/const, N4xxx ownership, N5xxx ABI/runtime, N8xxx tooling/package, N9xxx ICE다.

## Renderer
Plain/ANSI/JSON/Snapshot을 제공한다. byte Span과 Unicode scalar 표시 열은 다르다. 다중 파일/다중 줄/EOF label, note/help, suggestion applicability를 잃지 않는다. JSON schema는 DIAGNOSTIC_SCHEMA에 제안한다.

## 코드 운영
DIAGNOSTICS.csv의 각 코드에는 category, trigger, primary/secondary 의미, Stage를 기록한다. proposal을 승인 전 안정 API라고 부르지 않는다. code는 폐기 후 재사용하지 않는 D25 제안이다.

## 검증
renderer metadata 보존, control character escaping, invalid internal Span은 ICE 또는 명시 error, 연쇄 오류 suppression, sorting 결정성.
