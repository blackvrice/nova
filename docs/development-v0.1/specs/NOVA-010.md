# NOVA-010 — 숫자·문자·문자열 Literal 사양서

| 항목 | 값 |
|---|---|
| 대상 | Nova 0.1 |
| 작성일 | 2026-10-03 |
| 문서 상태 | Draft — 기존 결정은 유지, 추가 상세는 승인 대기 |
| 적용 Stage | A~B |
| 근거 | [원본 NOVA-010](../../01_Source_Syntax/NOVA-010_숫자_문자_문자열_Literal_사양서.md) |

이 문서는 원본을 대체하는 확정 사양이 아니다. 기존 확정 기준은 [CANONICAL](../CANONICAL.md)을 따르며, 새 의미·문법·API·정책은 [DECISIONS](../DECISIONS.md)의 승인이 필요하다. D01~D05 Lexer 상세는 [승인 기준](../ACCEPTED_LEXER.md)을 따른다. [전체 색인](../INDEX.md).

char의 Unicode scalar 값·동일 타입 비교·선언/대입/함수/const·UTF-8 보간·private scalar ABI는 사용자 승인 [P08](../CHAR_STAGE_B_PROPOSAL.md)과 [CHAR primary EBNF](../GRAMMAR_STAGE_B_CHAR.ebnf)를 따른다. [구현·검증 기록](../CHAR_IMPLEMENTATION.md). D04 Lexer/escape/END는 유지하며 cast/char 산술·전체 D07은 Draft다.

binary32/64 literal·손실 없는 숫자 승격·IEEE 산술/비교·canonical NaN·const·최단 fixed decimal 보간·private ABI는 사용자 승인 [P09](../FLOAT_STAGE_B_PROPOSAL.md)와 [FLOAT primary EBNF](../GRAMMAR_STAGE_B_FLOAT.ebnf)를 따른다. [구현·검증 기록](../FLOAT_IMPLEMENTATION.md). float IEEE 결과는 const 실패가 아니며 INT checked 정책은 유지한다. 숫자 cast는 P10, float remainder/math API·전체 D07은 Draft다.

## Literal 초안 — D04
정수는 10진과 0x/0b/0o prefix, 자릿수 사이 _를 제안한다. 부호는 literal의 일부가 아니다. 실수는 decimal fractional 또는 exponent를 사용하고 suffix는 첫 승인안에서 제공하지 않는다. 문자 literal은 Unicode scalar 한 개, string은 UTF-8 sequence다.

## Escape 제안
\n, \r, \t, \0, \\, \" 및 \'와 \u{hex}를 허용한다. surrogate와 0x10FFFF 초과를 거부한다. raw/multiline string은 이번 초안에서 제외한다. 실수 overflow/underflow와 rounding은 D07에서 정의한다.

## 단계 분리
Lexer는 철자와 escape 유효성을 검사하고 큰 정수 원문을 보존한다. 타입 단계가 기대 타입, 범위, prefix 음수와 최소 signed 값을 판정한다. token을 host i32로 먼저 파싱해 잘라내지 않는다.

## 검증
0x, 1__2, 1e+, 두 scalar char는 fail; int32 최솟값과 큰 uint64는 정확히 처리한다. LEXICAL과 NUMBER 모델을 함께 따른다.
