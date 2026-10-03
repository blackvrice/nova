# NOVA-110 — Foreign Ownership·Drop 계약 사양서

| 항목 | 값 |
|---|---|
| 대상 | Nova 0.1 |
| 작성일 | 2026-10-03 |
| 문서 상태 | Draft — 기존 결정은 유지, 추가 상세는 승인 대기 |
| 적용 Stage | A~E |
| 근거 | [원본 NOVA-110](../../10_FFI/NOVA-110_Foreign_Ownership_Drop_계약_사양서.md) |

이 문서는 원본을 대체하는 확정 사양이 아니다. 기존 확정 기준은 [CANONICAL](../CANONICAL.md)을 따르며, 새 의미·문법·API·정책은 [DECISIONS](../DECISIONS.md)의 승인이 필요하다. D01~D05 Lexer 상세는 [승인 기준](../ACCEPTED_LEXER.md)을 따른다. [전체 색인](../INDEX.md).

## FFI Ownership 표
Borrowed: caller owner 유지/호출 또는 지정 기간 동안만 사용. Owned-in: take로 전달/성공·실패 시 소비 여부 명시. Owned-out: 성공 결과에 새 owner/release 부여. Shared: retain/release pair 명시.

## 계약 초안 — D17
각 raw binding에 validity, mode, duration, nullability, length/alignment, error-path ownership, allocator, thread affinity를 기록한다. status가 실패해도 callee가 resource를 소비하는지 별도 필드다.

## 검증
success/error 모두 release count, take 뒤 재사용 fail, foreign-retained borrowed pointer 거부, mismatched free, callback context Drop ordering. Drop glue는 foreign release 호출을 한 번만 생성한다.
