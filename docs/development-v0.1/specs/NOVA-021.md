# NOVA-021 — 접근 제한·가시성 사양서

| 항목 | 값 |
|---|---|
| 대상 | Nova 0.1 |
| 작성일 | 2026-10-03 |
| 문서 상태 | Draft — 기존 결정은 유지, 추가 상세는 승인 대기 |
| 적용 Stage | B~E |
| 근거 | [원본 NOVA-021](../../02_Names_Modules/NOVA-021_접근_제한_가시성_사양서.md) |

이 문서는 원본을 대체하는 확정 사양이 아니다. 기존 확정 기준은 [CANONICAL](../CANONICAL.md)을 따르며, 새 의미·문법·API·정책은 [DECISIONS](../DECISIONS.md)의 승인이 필요하다. [전체 색인](../INDEX.md).

## Visibility 초안 — D06
public는 외부 package까지, internal은 동일 package, private는 선언 module 내부로 제안한다. top-level 기본은 internal, member 기본은 private를 제안하며 결정 승인 전 사용하지 않는다.

## 유효 접근성
public item의 signature가 private type을 노출하면 오류다. reexport는 원 정의의 접근성을 확장하지 못한다. nested type, interface method, constructor도 동일한 접근 검사에 포함한다.

## 구현
Definition에는 declared/effective visibility, owner module/package를 저장한다. overload 후보의 접근 불가를 무조건 undefined-name으로 숨기지 않고 접근 위치와 원 선언을 보여준다.

## 검증
같은 module/package/다른 package의 3개 caller에서 public/internal/private를 교차 검사한다. private field 접근과 public function의 private return type을 fail로 둔다.
