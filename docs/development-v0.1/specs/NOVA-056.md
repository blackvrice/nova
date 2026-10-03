# NOVA-056 — 사용자 drop·Drop Glue 사양서

| 항목 | 값 |
|---|---|
| 대상 | Nova 0.1 |
| 작성일 | 2026-10-03 |
| 문서 상태 | Draft — 기존 결정은 유지, 추가 상세는 승인 대기 |
| 적용 Stage | C |
| 근거 | [원본 NOVA-056](../../05_Ownership_Safety/NOVA-056_사용자_drop_Drop_Glue_사양서.md) |

이 문서는 원본을 대체하는 확정 사양이 아니다. 기존 확정 기준은 [CANONICAL](../CANONICAL.md)을 따르며, 새 의미·문법·API·정책은 [DECISIONS](../DECISIONS.md)의 승인이 필요하다. [전체 색인](../INDEX.md).

## 사용자 drop 초안 — D10/D12
drop { ... } 또는 drop receiver 문법은 D12에서 최종 선정한다. 초안 grammar는 drop { ... }를 제안한다. body는 자기 객체의 제한된 change 접근을 갖지만 owner를 외부로 이동하거나 부활시키지 못한다.

## Glue 순서 제안
사용자 drop body → field 선언 역순 Drop → Class storage free. destructor panic은 Abort하며 다른 field cleanup을 약속하지 않는다. drop을 가진 타입은 자동 Copy가 아니다.

## 검증
user body와 field trace 순서, recursive Drop cycle/stack budget, destructor의 self escape 거부, Class free 정확히 한 번, zero-sized field의 논리 Drop 호출을 확인한다.
