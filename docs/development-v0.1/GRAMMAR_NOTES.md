# Nova 문법 설명

상태: Draft. [문법 파일](GRAMMAR.ebnf)의 추가 구문은 [D01~D17](DECISIONS.md) 승인 대기다.

Stage A Parser 구현은 2026-10-04 승인된 [P01](PARSER_STAGE_A_PROPOSAL.md)과
[전용 EBNF](GRAMMAR_STAGE_A.ebnf)를 따른다. 아래 전체 문법의 미래 구문/의미는 Draft다.

## 표기와 Parser 입력

= 정의, , 연결, | 대안, [] 선택, {} 반복, () 묶음, (* *) 주석, ? ? 특수 조건을 쓴다.
따옴표 안의 END/EOF/IDENT/INT/FLOAT/STRING/CHAR/STRING_*와 INTERP_*는 token kind다.
나머지는 token 철자다. Whitespace/comment는 trivia이며 Parser가 별도로 보존한다.

boundary는 토큰을 소비하지 않고 }/EOF 직전에서만 성공한다. Parser 반복이 boundary로
무한히 성공하지 않도록 statement 본문은 반드시 최소 하나의 token을 소비해야 한다.
semicolon은 normalizer가 END로 바꾼다. compound statement 뒤 END는 block의 반복에서
소비할 수 있다. attribute 뒤 END도 허용한다.

## Stage subset

| Stage | grammar 사용 범위 |
|---|---|
| A | func/let/return/if, literals/name/call, Unit, int/string/bool 의미 subset |
| B | var/const, type/struct/enum, init/method, while/for/loop/match/try, module/import, aggregate expressions |
| C | change/take/using/view, drop, borrow/Move semantic checks |
| D | class/interface/generic/lambda/shared semantics의 의존 기능 |
| E | foreign/pointer/attributes, package 및 도구 |

grammar 수용과 stage별 semantic 지원은 다르다. bool 조건/비교/산술은 Stage A의 if
예제를 구체화하는 D03/D07/D08 제안이며 원본 Freeze의 추가 확정으로 취급하지 않는다.
Array/Option/Result의 source 선언은 B, 소유권·Generic 완성에는 C/D 의존성이 있다.
어느 Stage에서도 검사가 준비되지 않은 Move-safe 프로그램을 자동 정상으로 허용하지 않는다.

## 문맥/의미 검사가 필요한 부분

- assignment 왼쪽 expression은 의미 단계에서 mutable Place인지 검사한다.
- binding은 var만 initializer 생략을 허용하는 D10안; let/const 미초기화는 거부한다.
- receiver는 member에서만 첫 parameter로 허용하고 중복 self는 거부한다.
- pattern IDENT와 qualified variant는 resolution으로 구분한다. 1-segment variant name은
  선언 context에서 variant로 판정하며 모호하면 qualifier를 요구한다.
- comparison/range는 한 production에 한 operator만 허용한다.
- postfix cast의 종료 type와 뒤 binary operator는 type-parser/Pratt 경계에서 판정한다.
- func/class 이름 뒤 <...>는 선언/type에서만 허용한다. f<int>(x)는 explicit generic call
  제외 진단이 필요하며 grammar에서 우연히 compare가 되는 형태도 검사한다.
- ()는 Unit, (T,)는 single tuple, 일반 type grouping (T)는 이번 안에서 제외한다.
- enum payload constructor는 Name(args)로 호출하며 qualified name를 resolution한다.
- struct/class는 init 호출로 생성하며 implicit field literal는 이번 안에서 제외한다.
- take/change prefix는 모든 expression에서 무제한 유효하지 않다. call/owner context와
  Place/type/mode를 검사한다. user partial move는 여전히 금지다.
- raw pointer/foreign 계약과 static/interface/generic constraint는 별도 의미 검사가 필요하다.

P08 문자 리터럴은 사용자 승인 [char 계약](CHAR_STAGE_B_PROPOSAL.md)과 [전용 EBNF](GRAMMAR_STAGE_B_CHAR.ebnf)를 따른다.
P06의 primary에 기존 CHAR terminal만 추가한 31-production grammar이며 Parser에 적용했다.
타입·const·MIR·Native 검증은 [구현 기록](CHAR_IMPLEMENTATION.md)에 있다.

## 아직 보장하지 않는 것

P09은 사용자 승인 [float 계약](FLOAT_STAGE_B_PROPOSAL.md)과 [전용 EBNF](GRAMMAR_STAGE_B_FLOAT.ebnf)를 따른다.
P08 primary에 기존 FLOAT terminal만 추가한 31-production subset이다. Lexer/END를 보존한다.

nonterminal 검사 통과는 grammar 무모호성 증명이나 실제 parser 테스트 통과가 아니다.
문법 변경 시 Parser AST, formatter, fixtures, token 목록, 결정 기록을 함께 갱신한다.
reserved candidate async/await/yield 등은 D01에 따른 diagnostic classification이며
지원 production을 추가하지 않았다.
