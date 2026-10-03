# Compiler 전체 아키텍처 사양서

## 파이프라인

```text
Source
→ Lexer
→ END Normalizer
→ Parser
→ AST
→ HIR
→ Name Resolution
→ Type·Ownership Check
→ MIR
→ Move·Borrow·Drop Analysis
→ MIR Optimization
→ LLVM IR
→ Object
→ Link
```

## 구현 언어와 Backend

- Compiler: Rust
- Native Backend: LLVM Adapter
- Runtime: Rust와 최소 C ABI
- 표준 라이브러리: 초기에는 Nova+Runtime Intrinsic 혼합

## Workspace

```text
nova/
├─ crates/
│  ├─ nova-core-ids
│  ├─ nova-source
│  ├─ nova-diagnostics
│  ├─ nova-syntax
│  ├─ nova-lexer
│  ├─ nova-parser
│  ├─ nova-ast
│  ├─ nova-hir
│  ├─ nova-resolve
│  ├─ nova-types
│  ├─ nova-typecheck
│  ├─ nova-mir
│  ├─ nova-analysis
│  ├─ nova-codegen
│  ├─ nova-codegen-llvm
│  ├─ nova-package
│  └─ nova-cli
├─ runtime/
├─ std/
├─ tests/
├─ examples/
└─ docs/
```

## Query

```text
source_file(FileId)
lex(FileId)
parse(FileId)
lower_hir(ModuleId)
resolve_names(ModuleId)
type_check(FunctionId)
build_mir(FunctionId)
analyze_moves(FunctionId)
analyze_borrows(FunctionId)
elaborate_drops(FunctionId)
codegen_unit(CodegenUnitId)
```

## 필수 Dump

```bash
nova check file.nova --emit=tokens
nova check file.nova --emit=ast
nova check file.nova --emit=hir
nova check file.nova --emit=typed-hir
nova check file.nova --emit=mir
nova build file.nova --emit=llvm
```

## 첫 구현 순서

1. ID·Source·Span
2. Diagnostic
3. Lexer
4. Parser·AST
5. HIR
6. 최소 Resolve·Type
7. 최소 MIR
8. LLVM `main`
9. Console Runtime
10. Hello Nova E2E
