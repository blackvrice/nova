# NOVA-011 — 문자열 보간 Lexer Mode 사양서

| 항목 | 값 |
|---|---|
| 대상 | Nova 0.1 |
| 작성일 | 2026-10-03 |
| 문서 상태 | Draft — 기존 결정은 유지, 추가 상세는 승인 대기 |
| 적용 Stage | A~B |
| 근거 | [원본 NOVA-011](../../01_Source_Syntax/NOVA-011_문자열_보간_Lexer_Mode_사양서.md) |

이 문서는 원본을 대체하는 확정 사양이 아니다. 기존 확정 기준은 [CANONICAL](../CANONICAL.md)을 따르며, 새 의미·문법·API·정책은 [DECISIONS](../DECISIONS.md)의 승인이 필요하다. D01~D05 Lexer 상세는 [승인 기준](../ACCEPTED_LEXER.md)을 따른다. [전체 색인](../INDEX.md).

## Mode stack
Normal → StringText → InterpolationExpr → 중첩 StringText 전이를 stack으로 보존한다. 표현식의 { } 깊이는 문자열의 닫힘과 구분한다. 정상 string의 {{와 }}는 문자 중괄호이며 단일 {는 보간 시작이다.

## 상세 초안 — D04
보간 내부는 일반 표현식 문법을 사용한다. 닫는 }는 깊이가 0일 때만 문자열로 복귀한다. StringText에 단독 }가 나오면 오류다. format specifier와 사용자 정의 포맷 protocol은 0.1 첫 승인안에서 제외한다.

## 오류 복구
escape 실패는 해당 escape Span, 닫히지 않은 string은 opening quote와 EOF를 표시한다. 복구 결과는 Error token으로 남기고 codegen을 차단한다. mode depth resource limit은 내부 stack overflow 대신 진단으로 처리한다.

## 검증
중첩 호출/string/brace, 주석 속 brace, {{x}}, 빈 보간, EOF에서 각 mode의 상태와 결정적 token dump를 확인한다.
