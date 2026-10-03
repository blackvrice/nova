# NOVA-052 — 초기화·이동 상태 분석 사양서

| 항목 | 값 |
|---|---|
| 대상 | Nova 0.1 |
| 작성일 | 2026-10-03 |
| 문서 상태 | Draft — 기존 결정은 유지, 추가 상세는 승인 대기 |
| 적용 Stage | C |
| 근거 | [원본 NOVA-052](../../05_Ownership_Safety/NOVA-052_초기화_이동_상태_분석_사양서.md) |

이 문서는 원본을 대체하는 확정 사양이 아니다. 기존 확정 기준은 [CANONICAL](../CANONICAL.md)을 따르며, 새 의미·문법·API·정책은 [DECISIONS](../DECISIONS.md)의 승인이 필요하다. D01~D05 Lexer 상세는 [승인 기준](../ACCEPTED_LEXER.md)을 따른다. [전체 색인](../INDEX.md).

## 상태 lattice
각 local/field는 Uninitialized, Initialized, Moved, MaybeInitialized를 가진다. entry에서 parameter는 initialized, 미초기화 local은 uninitialized다. predecessor 상태가 다르면 merge에서 불확정 상태가 된다.

## transfer
읽기/빌림/take는 initialized를 요구한다. take는 moved로 전이, 유효 대입은 initialized로 전이한다. var 재대입은 기존 initialized 값을 먼저 Drop한 뒤 교체하지만 RHS는 기존 값 파괴 전에 평가하는 D10 제안이다.

## 검증
분기 한쪽 이동 뒤 읽기 fail, 모든 branch 재초기화 후 읽기 pass, loop backedge 이동, constructor field partial init, RHS 실패 시 기존 값 처리 의미를 검사한다. unknown/ErrorType 상태에서 연쇄 move 오류를 억제한다.
