# NOVA-147 — 언어 호환성 Test Suite 사양서

| 항목 | 값 |
|---|---|
| 대상 | Nova 0.1 |
| 작성일 | 2026-10-03 |
| 문서 상태 | Draft — 기존 결정은 유지, 추가 상세는 승인 대기 |
| 적용 Stage | 전 Stage |
| 근거 | [원본 NOVA-147](../../13_Testing_Release/NOVA-147_언어_호환성_Test_Suite_사양서.md) |

이 문서는 원본을 대체하는 확정 사양이 아니다. 기존 확정 기준은 [CANONICAL](../CANONICAL.md)을 따르며, 새 의미·문법·API·정책은 [DECISIONS](../DECISIONS.md)의 승인이 필요하다. D01~D05 Lexer 상세는 [승인 기준](../ACCEPTED_LEXER.md)을 따른다. [전체 색인](../INDEX.md).

## Compatibility suite
language version별 positive/negative corpus와 expected stdout/exit/Drop를 보존한다. parser acceptance만으로 호환성을 판단하지 않는다. diagnostic code/schema, std signature, package schema, ABI는 별도 matrix다.

## 초안 — D29
patch release는 이전 accepted programs와 normative rejects를 유지하는 것을 기본으로 한다. bug fix exception은 명확한 사양 근거, impact, migration을 기록한다.

## 검증
old compiler/new compiler cross-run, old lock/new resolver, old metadata/new compiler deliberate rebuild, Unicode/Target corpus. nondeterministic process IDs/addresses를 expected output에 넣지 않는다.
