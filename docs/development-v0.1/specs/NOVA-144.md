# NOVA-144 — 메모리 안전성 검증 전략서

| 항목 | 값 |
|---|---|
| 대상 | Nova 0.1 |
| 작성일 | 2026-10-03 |
| 문서 상태 | Draft — 기존 결정은 유지, 추가 상세는 승인 대기 |
| 적용 Stage | 전 Stage |
| 근거 | [원본 NOVA-144](../../13_Testing_Release/NOVA-144_메모리_안전성_검증_전략서.md) |

이 문서는 원본을 대체하는 확정 사양이 아니다. 기존 확정 기준은 [CANONICAL](../CANONICAL.md)을 따르며, 새 의미·문법·API·정책은 [DECISIONS](../DECISIONS.md)의 승인이 필요하다. [전체 색인](../INDEX.md).

## 메모리 안전성 증거
safe source의 uninitialized read/use-after-move/alias violation/dangling view/partial move를 compiler-fail corpus로 검증한다. runtime Array/Shared/FFI unsafe 부분은 sanitizer/model/fault injection으로 검사한다.

## 한계
테스트 통과가 soundness proof는 아니다. Loan/Place/Drop 알고리즘의 불변 조건과 counterexample review를 함께 관리한다. unsafe 사용자 계약 위반과 compiler가 잘못 허용한 safe source를 구분한다.

## 검증
Miri 적용 가능한 Rust core, address/undefined/thread sanitizer 적용 runtime harness, double-free/unaligned access/OOM/weak race, report→minimal regression 연결.
