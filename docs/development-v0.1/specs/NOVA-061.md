# NOVA-061 — Interface·구현·정적 디스패치 사양서

| 항목 | 값 |
|---|---|
| 대상 | Nova 0.1 |
| 작성일 | 2026-10-03 |
| 문서 상태 | Draft — 기존 결정은 유지, 추가 상세는 승인 대기 |
| 적용 Stage | D |
| 근거 | [원본 NOVA-061](../../06_Interfaces_Generics/NOVA-061_Interface_구현_정적_디스패치_사양서.md) |

이 문서는 원본을 대체하는 확정 사양이 아니다. 기존 확정 기준은 [CANONICAL](../CANONICAL.md)을 따르며, 새 의미·문법·API·정책은 [DECISIONS](../DECISIONS.md)의 승인이 필요하다. [전체 색인](../INDEX.md).

## Interface
implements는 명시적이며 같은 type/interface 조합 구현은 하나다. 0.1 dispatch는 static이다. Interface는 compile-time constraint이며 dynamic owned object type가 아니다.

## 초안 — D15
interface body의 method signature와 receiver mode/return/effect를 정확히 구현해야 한다. default method와 interface 간 관계는 첫 승인안에서 보류한다. Generic interface는 Associated Type 대신 명시 type parameter를 사용한다.

## 검증
누락 method, change/read receiver mismatch, duplicate impl, ambiguous method, foreign package orphan 정책은 D15에 연결한다. Class 구현 상속을 interface 구현 관계로 우회하지 않는다.
