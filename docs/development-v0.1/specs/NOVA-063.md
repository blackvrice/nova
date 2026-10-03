# NOVA-063 — Generic 타입 추론 사양서

| 항목 | 값 |
|---|---|
| 대상 | Nova 0.1 |
| 작성일 | 2026-10-03 |
| 문서 상태 | Draft — 기존 결정은 유지, 추가 상세는 승인 대기 |
| 적용 Stage | D |
| 근거 | [원본 NOVA-063](../../06_Interfaces_Generics/NOVA-063_Generic_타입_추론_사양서.md) |

이 문서는 원본을 대체하는 확정 사양이 아니다. 기존 확정 기준은 [CANONICAL](../CANONICAL.md)을 따르며, 새 의미·문법·API·정책은 [DECISIONS](../DECISIONS.md)의 승인이 필요하다. [전체 색인](../INDEX.md).

## Inference
타입 인수는 argument/receiver/expected type에서 추론한다. 명시 generic call arguments는 금지다. constraint solving은 type inference를 돕되 임의 구현 하나를 골라 type를 결정하지 않는다.

## 알고리즘 초안 — D15
fresh inference variables → parameter/actual unify → expected return unify → occurs check → substitutions normalize → interface obligations 검증. 해결되지 않은 변수가 있으면 annotation help를 제안한다.

## 검증
identity(1), expected return에 따른 empty container, incompatible repeated T, recursive T=Array<T> occurs failure, f<int>(x) 제외 진단. 실패 inference를 기본 int로 덮지 않는다.
