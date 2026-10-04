# NOVA-096 — 함수 ABI·인수·반환 Lowering 사양서

| 항목 | 값 |
|---|---|
| 대상 | Nova 0.1 |
| 작성일 | 2026-10-03 |
| 문서 상태 | Draft — 기존 결정은 유지, 추가 상세는 승인 대기 |
| 적용 Stage | A~E |
| 근거 | [원본 NOVA-096](../../09_Backend_Runtime/NOVA-096_함수_ABI_인수_반환_Lowering_사양서.md) |

이 문서는 원본을 대체하는 확정 사양이 아니다. 기존 확정 기준은 [CANONICAL](../CANONICAL.md)을 따르며, 새 의미·문법·API·정책은 [DECISIONS](../DECISIONS.md)의 승인이 필요하다. D01~D05 Lexer 상세는 [승인 기준](../ACCEPTED_LEXER.md)을 따른다. [전체 색인](../INDEX.md).

Stage A Native 최소 arithmetic/print/entry/internal ABI/toolchain 계약은 사용자 승인 [P03](../NATIVE_STAGE_A_PROPOSAL.md)를 따른다. [구현·지원·검증 범위](../NATIVE_IMPLEMENTATION.md). 전체 D07~D28과 미래 Stage 정책은 Draft다.

## Call lowering
semantic Signature → AbiSignature(args PassMode,return,destination,calling convention). source evaluation temp와 physical register/stack position을 분리한다.

## modes
Read scalar direct 또는 aggregate readonly pointer, change exclusive pointer, take owned representation 전달을 제안한다. readonly/noalias/nocapture는 분석으로 증명된 범위만 부착한다. Shared pointer와 raw FFI에는 추측 noalias를 넣지 않는다.

## 검증
caller/callee ABI 일치, named source order, large return destination, foreign callback context lifetime, stack alignment/aggregate classification across first two Targets.
