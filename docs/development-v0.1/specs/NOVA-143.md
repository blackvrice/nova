# NOVA-143 — 성능 Benchmark·회귀 기준서

| 항목 | 값 |
|---|---|
| 대상 | Nova 0.1 |
| 작성일 | 2026-10-03 |
| 문서 상태 | Draft — 기존 결정은 유지, 추가 상세는 승인 대기 |
| 적용 Stage | 전 Stage |
| 근거 | [원본 NOVA-143](../../13_Testing_Release/NOVA-143_성능_Benchmark_회귀_기준서.md) |

이 문서는 원본을 대체하는 확정 사양이 아니다. 기존 확정 기준은 [CANONICAL](../CANONICAL.md)을 따르며, 새 의미·문법·API·정책은 [DECISIONS](../DECISIONS.md)의 승인이 필요하다. D01~D05 Lexer 상세는 [승인 기준](../ACCEPTED_LEXER.md)을 따른다. [전체 색인](../INDEX.md).

## Benchmark 설계 초안 — D26
lex/parse/typecheck/MIR/codegen/link 시간, peak RSS, incremental hit/miss, executable runtime/size를 별도로 측정한다. toolchain/Target/hardware/power/flags/corpus size를 기록한다.

## 판정
warm/cold, repeated runs median 및 분산을 보고한다. 특정 비율 regression threshold는 baseline 측정 후 승인하며 사전에 근거 없는 수치를 성능 보장으로 쓰지 않는다. correctness 실패를 속도 이득으로 상쇄하지 않는다.

## 검증
대형 파일/많은 module/generic specialization/ownership CFG, optimized executable arithmetic/IO/containers. test machine 변화 시 baseline을 별도 유지한다.
