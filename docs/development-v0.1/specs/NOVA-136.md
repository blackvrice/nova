# NOVA-136 — Compiler 테스트 전략서

| 항목 | 값 |
|---|---|
| 대상 | Nova 0.1 |
| 작성일 | 2026-10-03 |
| 문서 상태 | Draft — 기존 결정은 유지, 추가 상세는 승인 대기 |
| 적용 Stage | 전 Stage |
| 근거 | [원본 NOVA-136](../../13_Testing_Release/NOVA-136_Compiler_테스트_전략서.md) |

이 문서는 원본을 대체하는 확정 사양이 아니다. 기존 확정 기준은 [CANONICAL](../CANONICAL.md)을 따르며, 새 의미·문법·API·정책은 [DECISIONS](../DECISIONS.md)의 승인이 필요하다. D01~D05 Lexer 상세는 [승인 기준](../ACCEPTED_LEXER.md)을 따른다. [전체 색인](../INDEX.md).

Stage A Parser 구문/복구는 사용자 승인 [P01](../PARSER_STAGE_A_PROPOSAL.md)과 [전용 EBNF](../GRAMMAR_STAGE_A.ebnf)가 우선한다. 나머지 추가 상세는 Draft다.

Stage A 단일 파일 의미 검사는 사용자 승인 [P02](../SEMANTICS_STAGE_A_PROPOSAL.md)가 우선한다. runtime/전체 타입/미래 Stage의 추가 상세는 Draft다.

Stage A MIR은 원본 CFG/Place 기준과 P02의 평가 순서에 따라 구현했다. [MIR 구현 기록](../MIR_IMPLEMENTATION.md)은 현재 API/검증 경계이며 runtime/미래 Stage 정책의 승인이 아니다.

Stage A Native 최소 arithmetic/print/entry/internal ABI/toolchain 계약은 사용자 승인 [P03](../NATIVE_STAGE_A_PROPOSAL.md)를 따른다. [구현·지원·검증 범위](../NATIVE_IMPLEMENTATION.md). 전체 D07~D28과 미래 Stage 정책은 Draft다.

## 테스트 계층
Unit → Lexer/Parser/HIR/MIR Snapshot → Compile-pass/fail → Runtime integration → E2E → Fuzz → Benchmark → Compatibility. 비용이 높은 계층은 해당 Stage에서 추가한다.

## traceability
CONFORMANCE의 T번호는 NOVA/D번호/Stage/fixture/기대 outcome를 연결한다. fail은 code+primary byte Span+필요 secondary를 확인하고 단순 nonzero exit만으로 통과시키지 않는다.

## 회귀
crash/잘못된 정상 허용/오진은 최소 재현을 추가한다. baseline을 바꾸기 전 implementation/spec/test/environment 원인을 분류한다.

## 검증
병렬 fixture 격리, Target annotation, timeout, stdout/stderr 구별, 실제 실행 command log. draft fixture는 current compiler pass 결과라는 뜻이 아니다.
