# NOVA-081 — MIR 구조·CFG·평가 순서 사양서

| 항목 | 값 |
|---|---|
| 대상 | Nova 0.1 |
| 작성일 | 2026-10-03 |
| 문서 상태 | Draft — 기존 결정은 유지, 추가 상세는 승인 대기 |
| 적용 Stage | A~E |
| 근거 | [원본 NOVA-081](../../08_MIR_Middleend/NOVA-081_MIR_구조_CFG_평가_순서_사양서.md) |

이 문서는 원본을 대체하는 확정 사양이 아니다. 기존 확정 기준은 [CANONICAL](../CANONICAL.md)을 따르며, 새 의미·문법·API·정책은 [DECISIONS](../DECISIONS.md)의 승인이 필요하다. D01~D05 Lexer 상세는 [승인 기준](../ACCEPTED_LEXER.md)을 따른다. [전체 색인](../INDEX.md).

Stage A MIR은 원본 CFG/Place 기준과 P02의 평가 순서에 따라 구현했다. [MIR 구현 기록](../MIR_IMPLEMENTATION.md)은 현재 API/검증 경계이며 runtime/미래 Stage 정책의 승인이 아니다.

## MIR 구조
Body는 typed locals/scopes/basic blocks를 가지며 block은 Statements와 하나의 Terminator다. Place=root+projection, Operand=Copy/Move/Constant, Rvalue=Use/Binary/Aggregate/Ref 등으로 제안한다.

## Terminator
Goto,Switch,Call,Drop,Return,Abort,Unreachable를 구분한다. Call/Drop은 원본 규칙대로 Terminator이며 successor를 명시한다. SourceInfo에 Span과 scope를 저장한다.

## 평가
left-to-right 임시값과 CFG로 source order를 고정한다. short circuit은 Switch, try는 variant branch, return은 return place+cleanup이다. unwind edge는 0.1 Abort 의미에 추가하지 않는다.

## 검증
typed operands, block target, place projections, cleanup path, no ErrorType/unresolved generic, exact Drop trace.
