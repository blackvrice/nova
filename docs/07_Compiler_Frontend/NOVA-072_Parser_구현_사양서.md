# Parser 구현 사양서

## 전략

- 선언·문장·타입: Recursive Descent
- 표현식: Pratt
- 오류 복구: Error Node·Synthetic Token
- 입력: END가 정규화된 Token Stream

## 상태

```rust
struct Parser<'a> {
    tokens: &'a [Token],
    cursor: usize,
    diagnostics: Vec<Diagnostic>,
    delimiters: Vec<TokenKind>,
}
```

## Top-level

- use
- const
- func
- struct·class·enum·interface
- foreign

Top-level 실행 문장은 금지한다.

## 표현식 우선순위

1. `until`, `through`
2. OR
3. AND
4. 비교
5. 덧셈
6. 곱셈
7. Prefix
8. Postfix Call·Member·Index·`exists`

할당은 Statement Parser가 처리한다.

## 복구

동기화:

```text
END }
func struct class enum interface const foreign
if while for loop match return break continue
```

## 완료

- 불완전 IDE 입력에서 Panic 없음
- 모든 문법 Production Snapshot
- 같은 Token은 같은 AST·진단
