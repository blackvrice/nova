# NOVA-142 — Property·Differential Testing 사양서

| 항목 | 값 |
|---|---|
| 대상 | Nova 0.1 |
| 작성일 | 2026-10-03 |
| 문서 상태 | Draft — 기존 결정은 유지, 추가 상세는 승인 대기 |
| 적용 Stage | 전 Stage |
| 근거 | [원본 NOVA-142](../../13_Testing_Release/NOVA-142_Property_Differential_Testing_사양서.md) |

이 문서는 원본을 대체하는 확정 사양이 아니다. 기존 확정 기준은 [CANONICAL](../CANONICAL.md)을 따르며, 새 의미·문법·API·정책은 [DECISIONS](../DECISIONS.md)의 승인이 필요하다. D01~D05 Lexer 상세는 [승인 기준](../ACCEPTED_LEXER.md)을 따른다. [전체 색인](../INDEX.md).

## Property testing
source reconstruction, Span validity, parse/format semantic equivalence, formatter idempotence, serial/parallel compile equality, optimization observable equivalence를 속성으로 둔다.

## Differential oracle
같은 Nova program의 O0/O2/두 Target을 비교한다. C++/Rust 프로그램을 oracle로 쓸 때는 Nova overflow/order/Drop 의미와 일치하는 subset만 사용한다. target-specific ABI 값을 무조건 같다고 비교하지 않는다.

## 검증
numeric boundary generator, equivalent parentheses/semicolon/newline transformation, alias normalization, shared race model. 실패의 seed와 minimized source를 보존한다.
