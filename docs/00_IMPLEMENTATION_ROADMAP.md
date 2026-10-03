# Nova Compiler 구현 로드맵

## Phase 0 — Workspace

Rust Workspace, Source, Diagnostic, Snapshot Harness를 만든다.

## Phase 1 — Lexer·Parser

TokenKind, Lexer Mode, END Normalizer, Recursive Descent, Pratt Parser, AST Dump를 완성한다.

```bash
nova check examples/syntax.nova --emit=ast
```

## Phase 2 — HIR·이름·타입

AST→HIR, Module, DefId, Primitive, 함수, Struct 타입 검사를 구현한다.

## Phase 3 — 최소 MIR·LLVM

표현식, 분기, 함수 MIR과 LLVM Scalar Codegen, Runtime Startup을 구현한다.

```bash
nova run examples/hello.nova
```

## Phase 4 — Enum·Match·Result

Enum Layout, Pattern, Match, Option, Result, try, Cleanup을 구현한다.

## Phase 5 — 소유권

Move, take, change, Borrow, Drop, View를 구현한다.

## Phase 6 — 추상화

Class, Interface, Generic, Monomorphization, Closure를 구현한다.

## Phase 7 — 제품화

Package, Incremental Cache, Formatter, C FFI, Windows·Linux Release를 구현한다.

현재 Phase에 필요한 문서만 동결하고, 후속 문서는 코드 검증 결과에 따라 결정 기록으로 수정한다.
