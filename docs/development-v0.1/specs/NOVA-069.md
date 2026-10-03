# NOVA-069 — 저장소·Crate·모듈 구조 사양서

| 항목 | 값 |
|---|---|
| 대상 | Nova 0.1 |
| 작성일 | 2026-10-03 |
| 문서 상태 | Draft — 기존 결정은 유지, 추가 상세는 승인 대기 |
| 적용 Stage | A~E |
| 근거 | [원본 NOVA-069](../../07_Compiler_Frontend/NOVA-069_저장소_Crate_모듈_구조_사양서.md) |

이 문서는 원본을 대체하는 확정 사양이 아니다. 기존 확정 기준은 [CANONICAL](../CANONICAL.md)을 따르며, 새 의미·문법·API·정책은 [DECISIONS](../DECISIONS.md)의 승인이 필요하다. D01~D05 Lexer 상세는 [승인 기준](../ACCEPTED_LEXER.md)을 따른다. [전체 색인](../INDEX.md).

## Workspace 책임
core-ids/source/diagnostics/syntax/lexer/parser/ast/hir/resolve/types/typecheck/mir/analysis/codegen/codegen-llvm/package/cli가 각각 ID, source, errors, tokens, scanning, parsing, syntax tree, lowering, lookup, canonical types, semantics, CFG, safety, backend API, LLVM, packages, driver를 담당한다.

## 의존 규칙
Lexer→Parser, HIR→Typecheck, Types→LLVM, Analysis→CLI 금지. public structs에 llvm_sys/inkwell type를 넣지 않는다. 현재 필요한 crate부터 만들고 비어 있는 미래 crate를 선행 추가하지 않는다.

## 품질
cargo fmt --check; cargo clippy --workspace --all-targets -- -D warnings; cargo test --workspace; cargo check --workspace --all-features를 CI에서 실행한다. Rust/LLVM 버전은 Stage backend 시작 때 toolchain policy D28로 고정한다.

## 검증
cargo metadata graph로 금지 edge와 cycle을 검사한다. unit test, fixtures, std/runtime source의 배치를 CONTRIBUTING에서 관리한다.
