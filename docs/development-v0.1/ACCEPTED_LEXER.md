# 승인된 Lexer 기준 — D01~D05

승인일: 2026-10-03. 승인 근거: 사용자의 “D01~D05 승인하고 Lexer 진행” 답변.
기존 Canonical 결정은 유지한다. 이 문서는 Lexer 상세에 대한 승인된 추가 기준이며
원본 파일을 재작성하지 않는다. D06~D30 및 Parser 전체 의미는 계속 Draft다.

## D01 — Keyword 분류

원본 NOVA-004 공식 목록에 use/type/lambda를 추가한다. self/c/library와 Primitive 이름은
일반 Identifier로 tokenize하여 이후 Parser/type context에서 판정한다. Read/trait/external을
새 키워드나 공식 키워드 alias로 만들지 않는다. noPanic과 후속 예약 후보의 reservation은
보류되어 현재 Identifier다. future feature의 실행 지원은 키워드 인식과 별개다.

## D02 — 문자 모델

UTF-8, Unicode XID_Start+밑줄/XID_Continue, NFC 변환 없음. Unicode 버전은 **18.0.0**으로
고정하며 unicode-ident 1.0.26 공식 배포본을 vendoring한다. update는 compatibility 검토가
필요하다. LF/CRLF/CR은 newline event, initial BOM은 trivia, middle BOM은 Error다.
각 token의 원문 반열린 byte Span을 보존한다. source의 invalid UTF-8는 기존 SourceError다.

## D03 — Operator

LEXICAL의 + - * / % ! && || == != < <= > >= = -> => ? :: @와 punctuation를 최장 일치한다.
단일 &/|, bitwise/shift/compound assignment는 현재 operator로 추가하지 않는다. until/through/as/
exists는 keyword다. 결합성·cast 층·comparison chain/assignment statement 의미는 Parser에서
검증할 사항이며 Lexer가 expression 의미를 판단하지 않는다.

## D04 — Literal/Comment/Interpolation

LEXICAL의 decimal/base prefix/underscore/exponent/escape를 따른다. 부호는 별도 prefix token이며
숫자 범위는 type checker로 남긴다. char는 정확히 Unicode scalar 하나다. raw/multiline string
제외, 문자열의 {{/}} literal brace와 단일 {expression} 보간, nested block comment를 지원한다.
Trivia와 Error를 포함해 raw token Span 연결로 모든 source bytes를 재구성한다. block comment
안의 newline은 comment chunks 사이 event로 제공한다. 문자열 value decode는 후속 lowering이다.

## D05 — END

raw tokens와 parser-facing normalized tokens를 분리한다. semicolon/newline END는 원 Span과
origin를 유지한다. ()/[]/보간 expression/type argument 내부 newline은 억제한다. 그 안의 nested
{} block에서는 statement END를 허용한다. source-order header state와 operator/else/dot/comma
continuation을 사용하며 bare return/break/continue newline은 종료 우선이다.

현재 normalizer는 Stage A headers와 declaration/type annotation의 generic delimiter 구분을
구현한다. **cast type와 comparison의 모호성, 모든 미래 generic/lambda/interface production의
정규화 완전성은 Parser 단계 검증 대상이다.** 이 구현은 전체 Parser 완료를 뜻하지 않는다.
semicolon을 허용하지 않는 header/type 내부 구문은 Parser에서 거부한다.

## 진단과 검증

N1001 invalid character, N1002 malformed literal/escape, N1003 unterminated delimiter를 사용한다.
기존 N1xxx 영역 안에서 Lexer 구현 코드로 채택하며 전체 D25 JSON/lint/test schema를 승인한
것은 아니다. primary byte Span과 opening/EOF secondary label을 검증한다.
lexical pass/fail은 프로그램 전체 compile-pass/fail과 별개다. 전체 type/runtime correctness는
후속 stage가 필요하다.
