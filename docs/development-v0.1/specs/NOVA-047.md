# NOVA-047 — Match 완전성·도달 불가 Arm 분석서

| 항목 | 값 |
|---|---|
| 대상 | Nova 0.1 |
| 작성일 | 2026-10-03 |
| 문서 상태 | Draft — 기존 결정은 유지, 추가 상세는 승인 대기 |
| 적용 Stage | A~B |
| 근거 | [원본 NOVA-047](../../04_Functions_Control/NOVA-047_Match_완전성_도달_불가_Arm_분석서.md) |

이 문서는 원본을 대체하는 확정 사양이 아니다. 기존 확정 기준은 [CANONICAL](../CANONICAL.md)을 따르며, 새 의미·문법·API·정책은 [DECISIONS](../DECISIONS.md)의 승인이 필요하다. D01~D05 Lexer 상세는 [승인 기준](../ACCEPTED_LEXER.md)을 따른다. [전체 색인](../INDEX.md).

## Exhaustiveness
bool/Enum/Option/Result/tuple 조합은 constructor coverage로 검사한다. integer/string의 임의 값 공간에는 wildcard가 필요하다. guard가 있는 arm은 exhaustive coverage 증명으로 사용하지 않는 초안을 제안한다.

## 도달성
앞 arm이 완전히 덮는 뒤 arm은 unreachable 진단이며 severity는 D25에서 결정한다. declaration/source arm 순서를 보존한다. 누락 경우는 가능한 최소 pattern 예시로 보여준다.

## 검증
bool 한 arm 누락 fail, Option Some만 fail, wildcard 후 arm unreachable, guarded wildcard만 존재할 때 incomplete, nested enum/tuple coverage를 검사한다. algorithm recursion에는 결정적인 resource budget을 둔다.
