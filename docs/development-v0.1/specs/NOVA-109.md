# NOVA-109 — 외부 예외→Result 변환 사양서

| 항목 | 값 |
|---|---|
| 대상 | Nova 0.1 |
| 작성일 | 2026-10-03 |
| 문서 상태 | Draft — 기존 결정은 유지, 추가 상세는 승인 대기 |
| 적용 Stage | A~E |
| 근거 | [원본 NOVA-109](../../10_FFI/NOVA-109_외부_예외_Result_변환_사양서.md) |

이 문서는 원본을 대체하는 확정 사양이 아니다. 기존 확정 기준은 [CANONICAL](../CANONICAL.md)을 따르며, 새 의미·문법·API·정책은 [DECISIONS](../DECISIONS.md)의 승인이 필요하다. D01~D05 Lexer 상세는 [승인 기준](../ACCEPTED_LEXER.md)을 따른다. [전체 색인](../INDEX.md).

## 외부 예외
외부 exception은 Nova call stack/ABI로 직접 전파하지 않는다. C/C++ shim 또는 hosted adapter가 포착하여 status+error payload를 반환하고 safe wrapper가 Result<T,E>로 바꾼다.

## 초안 — D17
변환 불가능한 예외와 계약 위반은 Abort를 제안한다. error message의 소유권과 encoding, release 함수, original error category를 명시한다. foreign success payload가 생성되기 전에 실패하면 미초기화 값 Drop을 하지 않는다.

## 검증
success/error/shim catch/unknown exception, error string release, partially constructed owned result cleanup, try propagation. Nova panic은 foreign catch로 복구 가능한 예외가 아니다.
