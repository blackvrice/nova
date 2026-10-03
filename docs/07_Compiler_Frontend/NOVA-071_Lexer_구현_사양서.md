# Lexer 구현 사양서

## 입력·출력

UTF-8 Source를 Trivia 포함 Token Stream으로 변환한다.

```rust
struct Token {
    kind: TokenKind,
    span: Span,
}
```

## Token

- Identifier와 Keyword
- Integer·Float·String·Char
- Interpolation Start·Text·Expression Boundary
- `(){}[],:.;`
- Operator
- NewLine
- Whitespace·Comment Trivia
- Error
- EOF

## 규칙

- 최장 일치
- 잘못된 문자는 Error Token으로 보존
- 숫자 범위 검사는 타입 단계
- 중첩 Block Comment 허용
- String 보간: `"value: {value}"`
- `{{`, `}}`는 문자 중괄호

## END Normalizer

문장을 끝낼 수 있는 Token 다음의 NewLine을 `END`로 바꾼다. `()`, `[]` 내부에서는 억제하고 `{}` 내부에서는 허용한다. `else`, `.`, `,`, 연산자 앞뒤 연속 줄은 END를 억제한다.

## 완료

- 선형 시간
- Unicode·CRLF
- 보간 중첩
- Fuzz Crash 없음
- Token Snapshot 결정성
