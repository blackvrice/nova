# NOVA-091 — SSA 변환 여부 결정서

| 항목 | 값 |
|---|---|
| 대상 | Nova 0.1 |
| 작성일 | 2026-10-03 |
| 문서 상태 | Draft — 기존 결정은 유지, 추가 상세는 승인 대기 |
| 적용 Stage | B~E |
| 근거 | [원본 NOVA-091](../../08_MIR_Middleend/NOVA-091_SSA_변환_여부_결정서.md) |

이 문서는 원본을 대체하는 확정 사양이 아니다. 기존 확정 기준은 [CANONICAL](../CANONICAL.md)을 따르며, 새 의미·문법·API·정책은 [DECISIONS](../DECISIONS.md)의 승인이 필요하다. [전체 색인](../INDEX.md).

## 비SSA 결정
MVP MIR은 Place 기반 비SSA다. Move/Borrow/Drop 분석은 SSA 이전에 수행한다. phi node 요구 때문에 source ownership 모델을 바꾸지 않는다.

## Backend 경계
LLVM의 memory-to-register promotion을 사용할 수 있으나 Nova cleanup/alias 계약은 먼저 MIR에 확정한다. 별도 optimization SSA는 후속 work item이다.

## 검증
loop-carried locals, conditional Drop flags, branch merge return, address-taken local을 O0/O2로 비교한다. compiler core에 LLVM SSA Value type를 노출하지 않는다.
