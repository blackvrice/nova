# NOVA-003 — Nova 0.1 비지원 기능 목록

| 항목 | 값 |
|---|---|
| 대상 | Nova 0.1 |
| 작성일 | 2026-10-03 |
| 문서 상태 | Draft — 기존 결정은 유지, 추가 상세는 승인 대기 |
| 적용 Stage | 전 Stage |
| 근거 | [원본 NOVA-003](../../00_Governance/NOVA-003_Nova_0.1_비지원_기능_목록.md) |

이 문서는 원본을 대체하는 확정 사양이 아니다. 기존 확정 기준은 [CANONICAL](../CANONICAL.md)을 따르며, 새 의미·문법·API·정책은 [DECISIONS](../DECISIONS.md)의 승인이 필요하다. D01~D05 Lexer 상세는 [승인 기준](../ACCEPTED_LEXER.md)을 따른다. [전체 색인](../INDEX.md).

## 명시적 제외
Class 구현 상속, Dynamic Interface Object, Associated Type, async/await, Coroutine/Generator, Pinning/Self-reference, Panic Unwind, Reflection, Const Generic, 명시적 Generic 호출 인수, 사용자 Operator/Property, Registry 서버, Self-hosting은 0.1 제외다.

## 거부 계약
Lexer가 인식 가능한 표기는 소스 Span을 보존하고 Parser 또는 의미 단계에서 기능명을 포함한 진단을 준다. 제외 기능을 Rust/C++ 의미로 추정해 실행하지 않는다. 후속 예약 후보를 실제 예약어로 지정하는 것은 D01 결정 사항이다.

## 검증
상속 선언, interface 값 저장, f<int>(x), await 호출 각각을 fail fixture로 둔다. 특히 f<int>(x)를 비교식으로 조용히 해석하여 성공시키지 않는다. 미래 설계 문서는 구현 완료 목록에 포함하지 않는다.
