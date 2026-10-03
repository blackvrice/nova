# NOVA-032 — 상수 표현식·const 평가 사양서

| 항목 | 값 |
|---|---|
| 대상 | Nova 0.1 |
| 작성일 | 2026-10-03 |
| 문서 상태 | Draft — 기존 결정은 유지, 추가 상세는 승인 대기 |
| 적용 Stage | B~E |
| 근거 | [원본 NOVA-032](../../03_Types_Declarations/NOVA-032_상수_표현식_const_평가_사양서.md) |

이 문서는 원본을 대체하는 확정 사양이 아니다. 기존 확정 기준은 [CANONICAL](../CANONICAL.md)을 따르며, 새 의미·문법·API·정책은 [DECISIONS](../DECISIONS.md)의 승인이 필요하다. [전체 색인](../INDEX.md).

## const 초안 — D09
const는 compile-time에 평가 가능한 불변 binding이다. literal, approved primitive operation, tuple/enum/struct construction을 허용하고 I/O, heap allocation, foreign call, mutable global access는 첫 승인안에서 금지한다.

## 평가 계약
Target width/rounding을 사용하고 host usize/float 동작에 의존하지 않는다. overflow/bounds/division 실패는 runtime panic 대신 해당 const expression의 compile error다. dependency cycle은 모든 참조 경로를 보여준다.

## resource limit
step/depth budget은 결정적인 옵션으로 기록하고 budget 초과를 user diagnostic으로 처리한다. arbitrary compile-time user function은 별도 const-function 설계가 승인되기 전 제외한다.

## 검증
const 1+2 정상, const 1/0 실패, cross-module cycle, 32-bit/64-bit Target에서 같은 fixed-width 값, budget 재현성을 검사한다.
