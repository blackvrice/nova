# NOVA-132 — Linter 규칙 사양서

| 항목 | 값 |
|---|---|
| 대상 | Nova 0.1 |
| 작성일 | 2026-10-03 |
| 문서 상태 | Draft — 기존 결정은 유지, 추가 상세는 승인 대기 |
| 적용 Stage | E |
| 근거 | [원본 NOVA-132](../../12_Tooling_Packaging/NOVA-132_Linter_규칙_사양서.md) |

이 문서는 원본을 대체하는 확정 사양이 아니다. 기존 확정 기준은 [CANONICAL](../CANONICAL.md)을 따르며, 새 의미·문법·API·정책은 [DECISIONS](../DECISIONS.md)의 승인이 필요하다. [전체 색인](../INDEX.md).

## Lint
allow/warn/deny 수준과 scope별 override를 지원한다. LintId와 diagnostic code를 분리한다. unused local/import, shadow, naming, unreachable candidate, needless mutable binding을 초안 D25로 제안한다.

## 자동 수정
MachineApplicable만 자동 적용 후보이며 다른 file/scope 의미를 바꾸면 MaybeIncorrect다. compiler correctness error를 allow lint로 끌 수 없다. suppression syntax는 D01/D25 승인 전 임의 keyword로 추가하지 않는다.

## 검증
override precedence, duplicate suppression, unused side effect initializer 유지, name collision fix 거부, formatter-after-fix semantic equality.
