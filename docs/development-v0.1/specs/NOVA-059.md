# NOVA-059 — Pinning·자기 참조 타입 사양서

| 항목 | 값 |
|---|---|
| 대상 | Nova 0.1 |
| 작성일 | 2026-10-03 |
| 문서 상태 | Draft — 기존 결정은 유지, 추가 상세는 승인 대기 |
| 적용 Stage | 제외/후속 |
| 근거 | [원본 NOVA-059](../../05_Ownership_Safety/NOVA-059_Pinning_자기_참조_타입_사양서.md) |

이 문서는 원본을 대체하는 확정 사양이 아니다. 기존 확정 기준은 [CANONICAL](../CANONICAL.md)을 따르며, 새 의미·문법·API·정책은 [DECISIONS](../DECISIONS.md)의 승인이 필요하다. D01~D05 Lexer 상세는 [승인 기준](../ACCEPTED_LEXER.md)을 따른다. [전체 색인](../INDEX.md).

## 0.1 제외 결정
Pinning와 self-reference는 Canonical 비지원이다. stable address를 이유로 owner 내부를 가리키는 안전 self view를 허용하지 않는다. Class heap allocation도 self-reference 안전성의 자동 증명이 아니다.

## 진단
노출된 pin syntax/API, 자신의 이동 가능한 storage를 가리키는 view field는 제외 기능 또는 lifetime 오류로 보고한다. unsafe raw pointer 패턴은 안전 self-reference 타입 지원이라는 뜻이 아니며 FFI 계약의 책임을 남긴다.

## 후속 연구 조건
Stage C의 Move/Drop/View를 먼저 검증한 뒤 address stability, projection, destructor 계약, ABI 영향을 별도 버전 제안한다.

## 검증
owner 내부 reference 저장/반환 fail, move 이후 dangling 포인터를 safe program이 만들 수 없는지 corpus로 검사한다.
