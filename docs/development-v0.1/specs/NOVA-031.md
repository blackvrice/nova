# NOVA-031 — 생성자·필드 초기화 사양서

| 항목 | 값 |
|---|---|
| 대상 | Nova 0.1 |
| 작성일 | 2026-10-03 |
| 문서 상태 | Draft — 기존 결정은 유지, 추가 상세는 승인 대기 |
| 적용 Stage | B~E |
| 근거 | [원본 NOVA-031](../../03_Types_Declarations/NOVA-031_생성자_필드_초기화_사양서.md) |

이 문서는 원본을 대체하는 확정 사양이 아니다. 기존 확정 기준은 [CANONICAL](../CANONICAL.md)을 따르며, 새 의미·문법·API·정책은 [DECISIONS](../DECISIONS.md)의 승인이 필요하다. D01~D05 Lexer 상세는 [승인 기준](../ACCEPTED_LEXER.md)을 따른다. [전체 색인](../INDEX.md).

## Constructor 초안 — D12
init(parameter list) { ... }를 제안하고 각 let field는 정확히 한 번 초기화해야 한다. method receiver 철자/self binding은 D11과 함께 결정한다. 생성 완료 전에 self를 외부로 노출하거나 일반 method를 호출하지 않는다.

## 초기화 계약
필드마다 Uninitialized/Initialized/MaybeInitialized 상태를 계산한다. 모든 정상 return path에서 모든 필드가 initialized여야 한다. 실패/조기 반환에서는 이미 초기화된 field만 선언 역순 Drop한다. zero initialization은 언어 기본 의미가 아니다.

## 검증
누락 field, 분기 한쪽에서만 초기화, let field 2회 대입, self escape, partial initialization cleanup. Array 내부 부분 초기화 허용과 사용자 partial move 금지를 구분한다.
