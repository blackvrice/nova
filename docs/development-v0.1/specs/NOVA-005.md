# NOVA-005 — Nova 문서 인덱스·의존 관계도

| 항목 | 값 |
|---|---|
| 대상 | Nova 0.1 |
| 작성일 | 2026-10-03 |
| 문서 상태 | Draft — 기존 결정은 유지, 추가 상세는 승인 대기 |
| 적용 Stage | 전 Stage |
| 근거 | [원본 NOVA-005](../../00_Governance/NOVA-005_Nova_문서_인덱스_의존_관계도.md) |

이 문서는 원본을 대체하는 확정 사양이 아니다. 기존 확정 기준은 [CANONICAL](../CANONICAL.md)을 따르며, 새 의미·문법·API·정책은 [DECISIONS](../DECISIONS.md)의 승인이 필요하다. [전체 색인](../INDEX.md).

## 문서 의존 방향
Governance → Source/Syntax → Names/Types → Functions/Control → Ownership/Abstraction → Compiler IR → Backend/Runtime → FFI/Library → Tooling → Release 순서로 읽는다. 구현 순서는 별도 ROADMAP을 따른다.

## 문서 유형
언어 문서는 관찰 가능한 행동과 거부 조건을 정의한다. 구현 문서는 자료구조와 단계 경계를 정의한다. API 문서는 모드/오류/비용을 정의한다. 테스트 문서는 각 요구사항의 관측 방법을 정의한다.

## 추적 계약
INDEX의 각 NOVA ID는 원본, 보완 문서, Stage와 연결된다. DECISIONS의 D번호는 여러 문서가 공유하는 미확정 의미의 단일 출처다. CONFORMANCE의 T번호는 통과 기준이며 아직 실행된 테스트라는 뜻이 아니다.

## 검증
중복 ID, 누락 문서, 끊어진 링크, EBNF 미정의 nonterminal, 존재하지 않는 D/T 참조는 문서 검증 실패다.
