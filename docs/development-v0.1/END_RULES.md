# END 정규화 상세 초안

근거: NOVA-013/071. D05 승인 대기. 원문 NewLine/semicolon 위치는 synthetic END의 origin으로 남긴다.

## 규칙 우선순위

1. string text 안의 newline은 lexical 오류이며 statement event가 아니다.
2. ()/[] 내부의 newline은 END로 만들지 않는다. interpolation expression은 자체 delimiter stack이다.
3. semicolon은 END. 빈 END 연속은 trivia origin를 보존한 하나 또는 복수의 separator로 처리 가능하다.
4. header 종료 token → {, } → else, else → if/{ 연결에서는 newline END를 억제한다.
5. return/break/continue 바로 뒤 newline은 bare jump 종료를 우선한다.
6. dot/comma/binary operator 앞뒤는 expression continuation이다. prefix/binary 판정은 이전
   expression 종료 가능 여부를 사용한다. a newline - b는 continuation 제안이다.
7. 나머지는 앞 token이 statement end 가능하고 뒤 token이 continuation 아닌 경우 END다.
8. }/EOF 직전 semicolon/newline이 없어도 grammar boundary가 simple statement 종료를 허용한다.

Header state는 func signature/if/while/for/match/constructor/type declaration별로 관리한다.
generic declaration/type 문맥의 < > 안에서는 newline을 억제하는 D05 안을 제안한다.
comparison < >를 delimiter로 단순 계수해서는 안 된다. type annotation/return type/where
context와 expression context를 구분하는 상태 전이를 prototype와 fixture로 검증해야 한다.
단순 두 token 검사만으로 foo() 다음 {와 func foo() 다음 {를 같게 처리하지 않는다.
explicit ;가 header의 금지 위치에 있으면 Parser syntax 오류이며 제거하여 추정하지 않는다.

## 예제

| 원문 의미 | 정규화/판정 |
|---|---|
| let a=1 newline let b=2 | 1 뒤 END |
| let a=1 newline +2 | continuation → 1+2 |
| f(1,newline 2) | call 내부 END 없음 |
| if true newline { f() } newline else { g() } | header/else 연결 |
| return newline f() | bare return END, 후속 f() |
| f() newline .member() | continuation |
| let a=1 /* newline */ let b=2 | comment newline도 END 후보 |
| let a=1 } | boundary로 종료 |
| f(); g() | END separator |

Return 같은 jump와 operator continuation 충돌을 명시 우선순위로 해결한다. Formatter는
이 규칙을 바꾸는 줄 재배치를 하지 않는다. Stage A에서 END 규칙을 부분 구현하더라도
지원 subset이 무엇인지 진단해야 하며 유효한 전체 문법이라고 주장하지 않는다.

## 검사

semicolon/newline 동등 AST, header state nested delimiters, else 연결, return
newline, comment events, interpolation token stream. T006과 production snapshots에 연결한다.
