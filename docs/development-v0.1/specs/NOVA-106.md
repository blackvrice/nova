# NOVA-106 — C ABI Import·Export 사양서

| 항목 | 값 |
|---|---|
| 대상 | Nova 0.1 |
| 작성일 | 2026-10-03 |
| 문서 상태 | Draft — 기존 결정은 유지, 추가 상세는 승인 대기 |
| 적용 Stage | A~E |
| 근거 | [원본 NOVA-106](../../10_FFI/NOVA-106_C_ABI_Import_Export_사양서.md) |

이 문서는 원본을 대체하는 확정 사양이 아니다. 기존 확정 기준은 [CANONICAL](../CANONICAL.md)을 따르며, 새 의미·문법·API·정책은 [DECISIONS](../DECISIONS.md)의 승인이 필요하다. D01~D05 Lexer 상세는 [승인 기준](../ACCEPTED_LEXER.md)을 따른다. [전체 색인](../INDEX.md).

## C ABI 경계
Nova 내부 ABI와 C ABI를 구분한다. scalar fixed-width, opaque pointer, explicit C representation aggregate만 허용하는 초안을 제안한다. Nova bool/char/string/Option/Result를 C int/string에 암묵 대응시키지 않는다.

## 계약 — D16/D17
C integer signedness/width, calling convention, symbol, nullable pointer, length units, out parameter, callback lifetime을 명시한다. C errno/status는 wrapper에서 Result로 매핑하고 runtime ownership로 raw resource를 감싼다.

## 검증
C가 부른 Nova export/Nova가 부른 C import, large aggregate return, Windows/Linux ABI differences, null/output buffer/callback roundtrip. C shim이 예외를 포착하며 unwind가 ABI를 넘지 않는다.
