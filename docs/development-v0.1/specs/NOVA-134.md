# NOVA-134 — API 문서 생성기 사양서

| 항목 | 값 |
|---|---|
| 대상 | Nova 0.1 |
| 작성일 | 2026-10-03 |
| 문서 상태 | Draft — 기존 결정은 유지, 추가 상세는 승인 대기 |
| 적용 Stage | E |
| 근거 | [원본 NOVA-134](../../12_Tooling_Packaging/NOVA-134_API_문서_생성기_사양서.md) |

이 문서는 원본을 대체하는 확정 사양이 아니다. 기존 확정 기준은 [CANONICAL](../CANONICAL.md)을 따르며, 새 의미·문법·API·정책은 [DECISIONS](../DECISIONS.md)의 승인이 필요하다. D01~D05 Lexer 상세는 [승인 기준](../ACCEPTED_LEXER.md)을 따른다. [전체 색인](../INDEX.md).

## API docs
public definitions/signatures/modes/effects/visibility/doc comments를 export table에서 추출한다. private/internal 노출은 explicit option만 허용한다. generated example은 grammar/language version을 표시한다.

## 초안 — D24
HTML/index/search JSON output, cross-reference resolution, static example compile checks를 제안한다. doc comment HTML는 escape/sanitize하고 arbitrary embedded script를 실행하지 않는다. link target source Span을 유지한다.

## 검증
generic/receiver modes 표시, broken reference, source doc attachment, Unicode signature, invalid example report, repeated generation byte stability.
