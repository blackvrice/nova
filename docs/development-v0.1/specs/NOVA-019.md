# NOVA-019 — 이름 해석·Scope 사양서

| 항목 | 값 |
|---|---|
| 대상 | Nova 0.1 |
| 작성일 | 2026-10-03 |
| 문서 상태 | Draft — 기존 결정은 유지, 추가 상세는 승인 대기 |
| 적용 Stage | B~E |
| 근거 | [원본 NOVA-019](../../02_Names_Modules/NOVA-019_이름_해석_Scope_사양서.md) |

이 문서는 원본을 대체하는 확정 사양이 아니다. 기존 확정 기준은 [CANONICAL](../CANONICAL.md)을 따르며, 새 의미·문법·API·정책은 [DECISIONS](../DECISIONS.md)의 승인이 필요하다. [전체 색인](../INDEX.md).

## 이름 해석
먼저 module/type/function 선언을 수집한 뒤 body 참조를 해석한다. 타입·값·모듈 namespace를 분리하고 nearest lexical scope를 검색한다. 결과는 문자열 대신 DefId/LocalId resolution side table이다.

## 상세 초안 — D06
동일 scope duplicate binding은 오류, nested scope shadowing은 허용하되 lint로 보고한다. let initializer는 새 binding 도입 전에 해석하므로 바깥 동일 이름을 참조할 수 있다. forward function/type 참조는 허용, forward local 참조는 금지다.

## 모호성
같은 이름의 후보를 import 순서로 고르지 않는다. overload set은 타입 검사로 넘기되 서로 다른 module에서 온 명확하지 않은 정의는 후보별 secondary label로 보고한다.

## 검증
호출 전 함수 선언, local 선언 전 참조, shadow initializer, namespace collision, import 순서 permutation을 다룬다.
