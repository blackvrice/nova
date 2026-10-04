# Stage A Parser·AST 착수안 — P01

작성일: 2026-10-04. 상태: **Accepted / 2026-10-04 사용자 승인**.
승인 근거: 사용자 답변 “P01 승인하고 Stage A Parser 구현 진행”.
기존 [D01~D05 승인](ACCEPTED_LEXER.md)은 유지한다. 이 안의 승인은 D06~D30 전체
승인이 아니며, 아래에 한정된 문법·복구 계약만 동결한다.

## Specification Change Proposal

- 관련 문서: NOVA-014/015/016/035/043/044/072/073/136.
- 현재 사양: func/let/return/if와 함수 호출, 필수 중괄호, optional semicolon 및 END,
  Recursive Descent+Pratt, Arena/AstNodeId, Error Node/Synthetic Token은 정해져 있다.
- 발견된 문제: 원본에는 함수 parameter/return type, let initializer, if/else의 실제
  production이 없다. 전체 보완 EBNF는 Draft이며 미래 Stage 구문이 함께 들어 있다.
- 제안 변경: [Stage A 전용 EBNF](GRAMMAR_STAGE_A.ebnf) 및 아래 지원·복구 계약을 채택한다.
- 변경 이유: 구현이 문법을 임의로 결정하지 않고 작은 승인 범위로 Hello Nova frontend를 완성한다.
- 영향 범위: Parser, AST, parser fixtures, 추후 HIR/type checker. Lexer의 확정 토큰 철자는 유지한다.
- Backward Compatibility: 현재 Parser가 없어 실행 프로그램의 변경은 없다. 기존 Canonical과
  Lexer 기준을 유지하며 이 subset의 구문을 신규 승인한다. 새로운 semantic/type/runtime 의미는 동결하지 않는다.
- 대안: parameterless func와 print(String)만 우선 구현하고 let/if/return은 별도 단계로 분리한다.

## 지원 문법

1. 단일 파일 top-level은 함수 선언만 허용한다. main 존재·중복 이름 검사는 후속 의미 단계다.
2. `func name(x: int, y: string) -> int { ... }`. 반환 표기는 선택, parameter type은 필수,
   parameter/call list의 마지막 comma는 허용한다. 타입 구문은 단순 Identifier 또는 `()`다.
   `void`와 반환 생략의 Unit 의미는 기존 Canonical을 따른다. 타입 이름의 유효성은 Parser가 판단하지 않는다.
3. `let name [: type] = expression`은 initializer 필수다. `return [expression]`을 지원한다.
   일반 expression statement를 허용한다. assignment, var/const와 loop는 이번 subset에 없다.
4. `if expression { ... } [else { ... } | else if ...]`는 statement다. block 마지막 expression을
   값으로 취급하지 않는다. bool-only 조건/평가 순서/short circuit의 타입·실행 계약(D08)은 별도 승인 대상이다.
5. Integer/String/true/false/Unit, name, grouping, positional function call, `+ - !` prefix,
   `* / % + -`, 비교, `&& ||`를 파싱한다. 비교는 nonassoc이며 `a < b < c`를 거부한다.
   우선순위·결합 방향은 승인 D03와 원본 NOVA-072를 따른다. 숫자 크기·기본 타입·overflow(D07)는 판단하지 않는다.
6. D04의 문자열 보간은 expression AST를 보존한다. String decode/formatting/runtime 지원은 후속 lowering이다.
   Float/Char/none, member/index/cast/range, named/default arguments, generic/type wrapper,
   receiver, module/visibility, aggregate/lambda/ownership는 이번 Parser에서 unsupported 진단으로 거부한다.
7. simple statement는 END 또는 닫는 중괄호/EOF 경계에서 끝난다. 누락된 closing delimiter는
   문장을 승인하는 뜻이 아니며 별도 syntax 오류다. raw tokens/trivia와 END origin은 보존한다.

## AST 및 복구

- nova-ast는 Arena/AstNodeId, byte Span, syntax node 및 source-order Visitor/dump를 제공한다.
  semantic TypeId/DefId, LLVM 타입은 저장하지 않는다. node 간 연결은 ID로 유지한다.
- nova-parser는 normalized tokens → AST+diagnostics. Lexer → Parser 의존은 만들지 않는다.
- missing token은 현재 위치의 빈 Span을 가진 Synthetic Token으로 기록한다. 실제 token과 구분한다.
- END/닫는 중괄호/func/let/if/return에서 동기화한다. 모든 반복은 소비하거나 종료한다.
  다음 함수의 func를 만났을 때 앞 함수의 누락 `}`를 복구해 다음 선언을 보존한다.
- N1101 expected token, N1102 unsupported syntax, N1103 chained comparison을 필요한 부분만
  채택한다. lexer Error에서 파생된 중복 진단은 억제하고 Error Node를 남긴다. 전체 D25 승인은 아니다.
- 깊은 입력에서 host stack overflow를 피하도록 ParserOptions.max_nesting=128을 기본으로
  제안한다. 호출자는 더 작은 한도를 지정할 수 있다. 초과는 N1102와 limit 설명으로 처리한다.
  더 높은 한도나 무제한 구현은 별도 검증 대상이다. 이는 D30 전체 compiler budget 승인과 별개다.

## 수용·검증 계획

- Hello Nova, typed function/call/return, let/if/else, 연산자 AST, Unit/grouping, 보간.
- semicolon/newline 동등 syntax tree, EOF/닫는 중괄호 boundary, Unicode/CRLF Span.
- initializer 누락, parameter type 누락, chained compare, unsupported stage feature의 code+Span.
- 누락 delimiter 뒤 다음 함수 보존, Error/synthetic node 방문, deterministic dump.
- empty input, truncated IDE input, deep nesting limit, deterministic adversarial corpus, 소비 진행.
- Parser-pass/fail은 compile-pass/fail이나 runtime 성공을 뜻하지 않는다.
- cargo fmt/clippy/test/check, 문서 기계 검증 후 결과와 GitHub commit을 보고한다.

## 승인 질문

P01의 Stage A 문법·AST·복구·nesting 한도는 승인되어 이 단계에 적용한다.
D06~D30 전체 정책과 미래 Stage 문법은 여전히 Draft다.
