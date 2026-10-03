# 저장소·Crate·모듈 구조 사양서

## Crate 책임

| Crate | 책임 |
|---|---|
| nova-core-ids | 안정적 ID |
| nova-source | File·Span·LineIndex |
| nova-diagnostics | 진단과 Renderer |
| nova-syntax | TokenKind·Trivia |
| nova-lexer | Source→Token |
| nova-parser | Token→AST |
| nova-ast | AST Node·Visitor |
| nova-hir | HIR Node·Lowering |
| nova-resolve | Scope·DefId·Import |
| nova-types | Type Interner |
| nova-typecheck | 타입·호출·소유권 |
| nova-mir | MIR 구조·Lowering |
| nova-analysis | Move·Borrow·Drop |
| nova-codegen | Backend Trait |
| nova-codegen-llvm | LLVM 구현 |
| nova-package | Manifest·Artifact |
| nova-cli | 명령행 |

## 금지 의존

- Lexer→Parser 참조 금지
- HIR→Typecheck 참조 금지
- Types→LLVM 참조 금지
- Analysis→CLI 참조 금지
- LLVM 타입의 Core 노출 금지

## 기본 CI

```bash
cargo fmt --check
cargo clippy --workspace --all-targets -- -D warnings
cargo test --workspace
cargo check --workspace --all-features
```
