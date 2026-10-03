# Lexical 상세 초안

근거: NOVA-008~012/071. 추가 결정: D01~D04. 상태: Draft.

## 문자와 token

원문 UTF-8 bytes를 유지한다. newline LF/CRLF/CR, space/tab, BOM(파일 첫 위치만 허용)을
별도 trivia/event로 기록한다. identifier는 고정 Unicode XID+_ 제안이며 normalization
없이 byte 철자를 비교한다. 언어의 identifier Unicode 버전은 build toolchain 정책에 고정한다.
Primitive 이름은 type namespace의 builtin identifier이며 무조건 reserved keyword로 하지 않는다.

공식 keyword는 원본 NOVA-004. 추가 use/type/lambda는 D01 승인 후보, self/c/library는
contextual 후보, @symbol은 foreign attribute다. trait/external은 canonical alias로
재해석하지 않는다. async/await 등 후속 예약 후보는 D01에서 reserved 여부를 결정한다.

Operator 후보: + - * / % ! && || == != < <= > >= = -> => ? . , : :: ; @
( ) [ ] { }. < >는 generic type와 comparison에 공유한다. `&`, bitwise/shift/compound
assignment는 최초 grammar에 없다. unsupported spelling은 명확히 거부한다.

## 숫자 철자

```ebnf
decimal = digit, { digit | "_", digit } ;
integer = decimal | "0x", hexDigits | "0b", binaryDigits | "0o", octalDigits ;
float = decimal, ".", decimal, [ exponent ] | decimal, exponent ;
exponent = ("e"|"E"), ["+"|"-"], decimal ;
```

위 snippet의 digit/hexDigits 등은 lexical character classes이며 Parser GRAMMAR의
nonterminal 집합과 별개다. prefix는 lowercase를 제안한다. 1. 또는 .5는 최초 안에서
실수가 아니다. 1.foo는 INT/dot/IDENT로 처리한다. 1_2 유효, 1__2/1_/0x_1 실패.
부호는 prefix token이며 -2147483648은 type checker가 signed minimum으로 처리한다.

## 문자와 문자열

char는 '...' 한 scalar, string은 "..." UTF-8다. escape: \n \r \t \0 \\ \" \'
및 \u{1~6 hex digits}; invalid scalar 거부. 문자열 newline/raw literal은 최초 안에서
거부하며 후속 필요 시 별도 결정한다. 내부 NUL은 허용하되 C NUL 문자열과 혼용하지 않는다.
단일 {는 interpolation 시작, {{/}}는 literal brace, 단독 }는 오류다.

## comment와 오류

// line comment, nested /* block */; comment 내부 newline event는 END 후보로 유지한다.
문자열과 comment mode를 혼동하지 않는다. 닫히지 않은 comment/string은 opening Span
및 EOF label을 남긴다. invalid char는 한 scalar 이상 소비한다. invalid UTF-8은 lexer
진입 전 source 오류이며 아직 유효한 FileId/Span가 없으면 file input diagnostic을 사용한다.

## 검사

token/trivia concat=original bytes, EOF=(length,length), Span UTF-8 boundaries,
linear scan, 중첩 mode depth budget, [T002~T005](CONFORMANCE.md)를 검증한다.
