# 진단 시스템·Error Code 관리 사양서

## 구조

```rust
struct Diagnostic {
    code: DiagnosticCode,
    severity: Severity,
    message: String,
    primary: Label,
    secondary: Vec<Label>,
    notes: Vec<String>,
    suggestions: Vec<Suggestion>,
}
```

## Code 영역

```text
N1xxx Lexer·Parser
N2xxx 이름·타입·함수·Interface·Generic
N3xxx 제어 흐름·Match·Const
N4xxx Move·Borrow·Drop·수명
N5xxx ABI·FFI·Runtime
N8xxx Tooling·Package
N9xxx ICE·내부 검증
```

## Renderer

- ANSI Terminal
- Plain Text
- JSON
- Snapshot

Span은 바이트 기준, 표시 열은 Unicode Scalar 기준이다.

## Suggestion

- MachineApplicable
- MaybeIncorrect
- HasPlaceholders
- Unspecified

## ICE

Compiler 버전, Target, Query Stack, Source 위치, 재현 Dump 정보를 출력한다.
