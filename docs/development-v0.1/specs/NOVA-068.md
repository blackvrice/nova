# NOVA-068 — Compiler 전체 아키텍처 사양서

| 항목 | 값 |
|---|---|
| 대상 | Nova 0.1 |
| 작성일 | 2026-10-03 |
| 문서 상태 | Draft — 기존 결정은 유지, 추가 상세는 승인 대기 |
| 적용 Stage | A~E |
| 근거 | [원본 NOVA-068](../../07_Compiler_Frontend/NOVA-068_Compiler_전체_아키텍처_사양서.md) |

이 문서는 원본을 대체하는 확정 사양이 아니다. 기존 확정 기준은 [CANONICAL](../CANONICAL.md)을 따르며, 새 의미·문법·API·정책은 [DECISIONS](../DECISIONS.md)의 승인이 필요하다. [전체 색인](../INDEX.md).

## Pipeline 계약
Source → Lexer → END → Parser → AST → HIR → Resolution/Types → MIR → Validation → Move/Borrow/Drop → Optimization → nova-codegen → LLVM Adapter → Object → Link.

## 각 경계
결과는 데이터+진단+SourceInfo이며 user error를 Result/diagnostic으로 전달한다. AST와 HIR/Typed tables를 분리하고 Backend는 verified concrete MIR만 받는다. LLVM type는 core에 노출하지 않는다.

## Query
source_file/lex/parse/lower_hir/resolve_names/type_check/build_mir/analyze_moves/analyze_borrows/elaborate_drops/codegen_unit 경계를 유지한다. Stage A에서는 query를 일반 함수로 구현하고 아직 disk cache/병렬 scheduler를 만들 필요는 없다.

## 검증
단계별 dump, invalid source의 codegen 차단, frontend test의 LLVM 설치 불필요, Target 독립 type 판단을 확인한다.
