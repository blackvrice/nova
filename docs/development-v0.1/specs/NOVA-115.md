# NOVA-115 — Primitive 메서드 API 사양서

| 항목 | 값 |
|---|---|
| 대상 | Nova 0.1 |
| 작성일 | 2026-10-03 |
| 문서 상태 | Draft — 기존 결정은 유지, 추가 상세는 승인 대기 |
| 적용 Stage | B~E (print: A) |
| 근거 | [원본 NOVA-115](../../11_Standard_Library/NOVA-115_Primitive_메서드_API_사양서.md) |

이 문서는 원본을 대체하는 확정 사양이 아니다. 기존 확정 기준은 [CANONICAL](../CANONICAL.md)을 따르며, 새 의미·문법·API·정책은 [DECISIONS](../DECISIONS.md)의 승인이 필요하다. D01~D05 Lexer 상세는 [승인 기준](../ACCEPTED_LEXER.md)을 따른다. [전체 색인](../INDEX.md).

## Primitive API 초안 — D23
정수 checkedAdd/checkedSub/checkedMul → Option<Self>, wrappingAdd/wrappingSub/wrappingMul → Self를 제안한다. normal operator는 D07의 checked/Abort 정책과 구분한다. parse(text) → Result<T,ParseError>, toString() → string.

## float/char
float isNaN/isInfinite/abs, char isAscii와 scalar 검사 API를 제안한다. locale-dependent formatting은 core 기본에서 제외한다. narrowing은 explicit checked cast 계약을 따른다.

## 검증
MIN/MAX wrapping, parse whitespace/sign/base/overflow, NaN/-0 출력 정책, Unicode scalar char. API signature의 allocation/panic/cost/mode를 STDLIB_API에서 표시한다.
