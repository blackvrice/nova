# NOVA-057 — Shared·Weak·참조 횟수 모델 사양서

| 항목 | 값 |
|---|---|
| 대상 | Nova 0.1 |
| 작성일 | 2026-10-03 |
| 문서 상태 | Draft — 기존 결정은 유지, 추가 상세는 승인 대기 |
| 적용 Stage | C~E |
| 근거 | [원본 NOVA-057](../../05_Ownership_Safety/NOVA-057_Shared_Weak_참조_횟수_모델_사양서.md) |

이 문서는 원본을 대체하는 확정 사양이 아니다. 기존 확정 기준은 [CANONICAL](../CANONICAL.md)을 따르며, 새 의미·문법·API·정책은 [DECISIONS](../DECISIONS.md)의 승인이 필요하다. D01~D05 Lexer 상세는 [승인 기준](../ACCEPTED_LEXER.md)을 따른다. [전체 색인](../INDEX.md).

## Shared/Weak
마지막 Strong 해제에서 payload를 Drop하고 Weak는 control block만 연장한다. strong cycle은 자동 수집하지 않으며 Weak로 끊는다. Weak upgrade는 살아 있는 Strong이 있을 때만 성공한다.

## 소유권 초안 — D23
Shared는 명시 clone으로 refcount 증가하는 Move handle이다. shared와 weak는 ownership wrapper type로 제안한다. Shared의 T mutation은 자동 change loan을 허용하지 않고 별도 동기화/interior-mutation API가 필요하다.

## 검증
strong 1→0와 weak 마지막 free, upgrade/last-drop race, count overflow Abort, 순환 누수 예제, mutable Shared alias 거부. control block이 해제된 뒤 upgrade가 읽지 않는지 sanitizer로 검사한다.
