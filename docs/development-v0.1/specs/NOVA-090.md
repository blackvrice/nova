# NOVA-090 — MIR 최적화 Pass 목록·순서 사양서

| 항목 | 값 |
|---|---|
| 대상 | Nova 0.1 |
| 작성일 | 2026-10-03 |
| 문서 상태 | Draft — 기존 결정은 유지, 추가 상세는 승인 대기 |
| 적용 Stage | B~E |
| 근거 | [원본 NOVA-090](../../08_MIR_Middleend/NOVA-090_MIR_최적화_Pass_목록_순서_사양서.md) |

이 문서는 원본을 대체하는 확정 사양이 아니다. 기존 확정 기준은 [CANONICAL](../CANONICAL.md)을 따르며, 새 의미·문법·API·정책은 [DECISIONS](../DECISIONS.md)의 승인이 필요하다. [전체 색인](../INDEX.md).

## Optimization 초안 — D26
Stage A는 검증 → 상수 folding(동일 숫자 의미) → unreachable block 제거 → 단순 CFG 정리 → 검증을 제안한다. C 이후에는 Drop elaboration 완료 뒤 dead temporary/copy propagation을 제한적으로 추가한다.

## 금지
side effect/Drop/abort 가능 operation을 제거하거나 reorder하지 않는다. integer checked arithmetic를 LLVM unchecked add로 바꾸지 않는다. fast-math/reassociation은 의미 변경이므로 기본 제외다.

## 검증
O0/O2 관찰 결과 비교: stdout/exit/Drop trace/error, overflow/bounds, short-circuit. 성능 개선은 correctness suite 통과 이후 측정한다.
