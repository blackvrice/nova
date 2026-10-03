# NOVA-077 — 타입 검사기·소유권 검사기 아키텍처 사양서

| 항목 | 값 |
|---|---|
| 대상 | Nova 0.1 |
| 작성일 | 2026-10-03 |
| 문서 상태 | Draft — 기존 결정은 유지, 추가 상세는 승인 대기 |
| 적용 Stage | A~E |
| 근거 | [원본 NOVA-077](../../07_Compiler_Frontend/NOVA-077_타입_검사기_소유권_검사기_아키텍처_사양서.md) |

이 문서는 원본을 대체하는 확정 사양이 아니다. 기존 확정 기준은 [CANONICAL](../CANONICAL.md)을 따르며, 새 의미·문법·API·정책은 [DECISIONS](../DECISIONS.md)의 승인이 필요하다. D01~D05 Lexer 상세는 [승인 기준](../ACCEPTED_LEXER.md)을 따른다. [전체 색인](../INDEX.md).

## Type/Ownership 분리
type checker는 typed expressions/call resolution/coercion과 요구 ownership mode를 계산한다. MIR analysis가 concrete CFG에서 initialization/move/borrow/drop 검사를 수행한다. source 수준 검사와 CFG 수준 검사를 중복 구현하더라도 책임을 명확히 한다.

## 데이터 계약
TypedBody는 TypeTable/CallResolution/OwnershipAction/Effects/Diagnostics를 포함한다. generic body는 symbolic constraint 상태를 구체 body와 구분한다. LLVM pointer type를 타입 안전성 판단에 쓰지 않는다.

## 검증
type mismatch에서 파생 borrow 오류 억제, mode-aware call, branch merge, ErrorType 차단, stage가 지원하는 타입 subset 선언.
