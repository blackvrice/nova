# NOVA-066 — Associated Type·Dynamic Interface Object 사양서

| 항목 | 값 |
|---|---|
| 대상 | Nova 0.1 |
| 작성일 | 2026-10-03 |
| 문서 상태 | Draft — 기존 결정은 유지, 추가 상세는 승인 대기 |
| 적용 Stage | 제외/후속 |
| 근거 | [원본 NOVA-066](../../06_Interfaces_Generics/NOVA-066_Associated_Type_Dynamic_Interface_Object_사양서.md) |

이 문서는 원본을 대체하는 확정 사양이 아니다. 기존 확정 기준은 [CANONICAL](../CANONICAL.md)을 따르며, 새 의미·문법·API·정책은 [DECISIONS](../DECISIONS.md)의 승인이 필요하다. D01~D05 Lexer 상세는 [승인 기준](../ACCEPTED_LEXER.md)을 따른다. [전체 색인](../INDEX.md).

## 비지원 경계
Associated Type과 dynamic interface object는 0.1 제외다. interface 자체를 runtime field/parameter/return 값 타입으로 쓰거나 type-erased vtable handle을 암묵 생성하지 않는다.

## 허용 대안
Generic T where T implements I, concrete type에 implements, explicit generic interface parameters는 static dispatch 범위에서 검토 가능하다. 이것이 Associated Type syntax 지원을 뜻하지 않는다.

## 검증
interface value storage fail, associated type declaration fail, constrained concrete generic pass. error message는 concrete generic 사용 대안을 설명하되 자동 source rewrite는 하지 않는다.
