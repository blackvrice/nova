# NOVA-141 — Lexer·Parser·MIR Fuzzing 사양서

| 항목 | 값 |
|---|---|
| 대상 | Nova 0.1 |
| 작성일 | 2026-10-03 |
| 문서 상태 | Draft — 기존 결정은 유지, 추가 상세는 승인 대기 |
| 적용 Stage | 전 Stage |
| 근거 | [원본 NOVA-141](../../13_Testing_Release/NOVA-141_Lexer_Parser_MIR_Fuzzing_사양서.md) |

이 문서는 원본을 대체하는 확정 사양이 아니다. 기존 확정 기준은 [CANONICAL](../CANONICAL.md)을 따르며, 새 의미·문법·API·정책은 [DECISIONS](../DECISIONS.md)의 승인이 필요하다. [전체 색인](../INDEX.md).

## Fuzz targets
bytes→source validation/lexer, normalized tokens→parser, structured MIR→validator/analysis. valid source mutation과 invalid source generation을 함께 사용한다.

## failure
crash/hang/ICE/non-determinism는 실패다. resource-limit diagnostic은 명시 limit을 충족하면 정상 거부다. wrong-code는 differential/observable fixture로 분리한다.

## 운용
seed/corpus/minimizer/compiler/Target/options를 저장하고 dedup한 최소 재현을 regression에 올린다. 깊은 mode/delimiter, UTF-8 boundaries, CFG cycles, move/loan permutations를 생성한다.

## 검증
시간/메모리 예산, sanitizer runtime, repeated seed 동일 outcome. 아직 C/D가 없으면 해당 fuzz target을 미리 구현하지 않는다.
