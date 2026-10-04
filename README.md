# Nova 0.1 compiler

개발 전에 읽을 [전체 개발 문서 보완팩](docs/development-v0.1/README.md)을 작성했습니다.
148개 주제별 문서와 구체 EBNF, 30건 결정 초안, API/schema, 수용 테스트 계획을 포함합니다.
D01~D05 Lexer, P01 Parser, P02 Stage A 이름·타입 최소 계약은 사용자 승인으로 Accepted이며,
나머지 상세는 Draft입니다.

Nova 컴파일러의 첫 Stage A 기반 구현입니다. 언어 사양은 `docs/`의 원본
Documentation Pack과 사용자가 제공한 Canonical Decisions를 따릅니다.
원본 사양 파일은 보존하며, 사용자 승인된 상세 계약을 별도 문서로 추가했습니다.

## 현재 구현

- `nova-core-ids`: 데이터베이스 내에서 안정적인 정수 `FileId`.
- `nova-source`: append-only UTF-8 `SourceDatabase`, 반열린 바이트 `Span`,
  지연 계산되는 `LineIndex`, Unicode scalar 기준의 1-based 표시 위치.
- `nova-diagnostics`: `DiagnosticCode`, Severity, Primary/Secondary Label,
  Note, Suggestion 및 Plain/ANSI/JSON/Snapshot Renderer.
- `nova-syntax`: 공식/승인 키워드, token, END origin 등 단계 독립 자료형.
- `nova-lexer`: UTF-8 lossless scanning, Unicode XID, Literal/Escape, 중첩 주석/보간,
  오류 진단, 결정적 token dump, Stage A END 정규화.
- `nova-ast`: byte Span과 source-order 자식 ID를 보존하는 Arena, AstNodeId,
  Error Node, 반복형 Visitor와 결정적 dump. 의미 TypeId/DefId는 포함하지 않습니다.
- `nova-parser`: 승인된 Stage A 함수·let·return·if/else, typed parameter, positional call,
  기본 표현식·문자열 보간. Recursive Descent+Pratt, Synthetic Token 및 오류 복구.
- `nova-hir`: AST와 분리된 flat HIR, SymbolId/SourceOrigin, Primitive/Unit 정규화와 String decode.
- `nova-resolve`: ScopeTree/DefId/DefinitionRegistry/ResolutionMap, 함수 forward reference와 지역 Scope.
- `nova-types`: Stage A TypeInterner, Int32/Bool/String/Unit, internal Function 및 ErrorType.
- `nova-typecheck`: expected type/TypeTable, Literal 범위·인수·return·Bool 조건 검사, 오류 진단.
- `nova-mir`: 비SSA Place/Operand/Rvalue, BasicBlock CFG, source-order Call Terminator,
  short-circuit/if/return Lowering과 타입·초기화·CFG 검증.

의존 방향은 `nova-diagnostics → nova-source → nova-core-ids`입니다.
Lexer의 의존 방향은 `nova-lexer → nova-syntax/nova-source/nova-diagnostics`입니다.
Parser의 production 의존 방향은 `nova-parser → nova-ast/nova-syntax/nova-source/nova-diagnostics`입니다.
Lexer 연동은 Parser 테스트의 dev dependency로만 사용합니다.
HIR은 AST/Source/Syntax에, Resolver는 HIR에, TypeChecker는 HIR/Resolver/Types에 의존합니다.
HIR→TypeChecker, Types→LLVM 의존은 없습니다.
MIR은 HIR/Resolve/Types/TypeCheck에 의존하고 LLVM을 포함하지 않습니다.
Unicode 18.0.0의 XID 데이터는 고정된 unicode-ident 1.0.26을 vendor에 포함했습니다.
Rust 1.80 이상이 필요하며 `cargo test --workspace --offline`으로 빌드할 수 있습니다.

```powershell
cargo fmt --check
cargo clippy --workspace --all-targets -- -D warnings
cargo test --workspace
cargo check --workspace --all-features
```

## 구현 경계

`FileId`는 하나의 데이터베이스 안에서 파일 추가 순서에 따라 배정됩니다.
영구 캐시 ID나 경로/content hash가 아닙니다. 소스는 추가 후 수정할 수 없으므로
기존 ID, Span, 지연 LineIndex는 그대로 유지됩니다.

`Span::new`는 범위 순서를 검사하고 `SourceDatabase::slice`와 진단 Renderer는
파일 존재 여부, 범위, UTF-8 경계를 검사합니다. EOF의 빈 Span은 유효합니다.
잘못된 UTF-8 입력은 교체 문자 없이 `SourceError::InvalidUtf8`로 반환합니다.
Source 계층 오류를 Nxxxx 진단으로 매핑하는 작업은 향후 Driver에서 담당합니다.

LineIndex는 LF, CRLF, CR을 처리하고 원본 바이트를 보존합니다. 이는 소스 표시의
구현 선택이며 Lexer의 문장 종료/END 의미를 정의하지 않습니다.
표시 열은 NOVA-078에 따라 Unicode scalar 수이며 터미널 셀 폭과 다를 수 있습니다.
Snapshot은 ANSI 없이 Plain 형식을 사용하고 경로 구분자를 `/`로 표시합니다.
진단에는 바이트 범위와 원본 위치가 함께 남습니다.

NOVA-070의 `ReadOnlySpan`, `splitAt`, pointer+length ABI 및 region 계약은
NOVA-002와 사용자 Stage 순서에 따라 언어 수준 Stage C 작업으로 남깁니다.
이번 컴파일러 `Span`은 해당 언어 타입과 별개인 소스 위치 자료형입니다.

## 다음 단계

1. Lexer/Token/Stage A END 정규화 완료: [승인 기준](docs/development-v0.1/ACCEPTED_LEXER.md).
2. Stage A Parser/AST 완료: [P01 승인 범위와 전용 EBNF](docs/development-v0.1/PARSER_STAGE_A_PROPOSAL.md).
3. HIR·최소 이름/타입 검사와 frontend pass/fail harness 완료: [P02 승인 범위](docs/development-v0.1/SEMANTICS_STAGE_A_PROPOSAL.md).
4. MIR lowering/validation 완료: [구현·검증 경계](docs/development-v0.1/MIR_IMPLEMENTATION.md).
5. 최소 runtime/entry/print/host 계약 동결, Codegen Interface/LLVM Adapter와 Hello Nova E2E.

제공된 NOVA-014는 일반 요구사항을 담고 있지만 실제 EBNF Production은 없습니다.
Stage A는 별도로 사용자 승인된 `GRAMMAR_STAGE_A.ebnf`를 따릅니다.
전체 `GRAMMAR.ebnf`의 미래 Stage 구문은 여전히 Draft입니다.

현재 Stage A MIR까지 구현했습니다. CLI 및 LLVM Backend는 후속 단계입니다.
따라서 `nova check`와 `nova run`은 아직 제공하지 않습니다. 이번 테스트는 Rust
기반 계층의 UTF-8, 범위 오류, EOF, 혼합 줄바꿈, 대형 파일, 진단 Snapshot,
JSON escaping 및 Suggestion 위치 검증을 다룹니다.
Lexer lexical pass/fail fixture와 source reconstruction/중첩 mode/END/회귀 테스트도 포함합니다.
lexical pass는 프로그램 전체 타입 검사나 실행 성공을 뜻하지 않습니다.
Parser-pass 역시 구문 수용만 뜻합니다. 이름·타입·실행 결과는 보장하지 않습니다.
frontend-pass는 승인된 Stage A 이름·타입 검사 성공을 뜻하며 Native 실행 성공이 아닙니다.
Parser 입력은 normalized tokens여야 하며, 잘못된 API 입력은 ParseInputError로 반환합니다.
잘못된 Nova 구문은 N1101~N1103와 recovery AST로 반환합니다. 호출자는 Lexer와 Parser
오류를 모두 확인한 후 lowering해야 합니다. 기본 nesting limit은 128이며 1~128로 설정 가능합니다.

의미 분석 API/구현 경계는 [P02 구현 계약](docs/development-v0.1/SEMANTICS_IMPLEMENTATION.md)에 있습니다.
Stage A `print`는 `print(string) -> Unit`이며 Int32/Bool 출력은 보간 문자열을 사용합니다.
main 존재/signature는 아직 fragment 검사에서 강제하지 않으며 Native entry 단계에서 구현합니다.
MIR은 arithmetic과 interpolation을 abstract 연산으로 보존합니다. Runtime overflow/출력 형식/ABI는
Native 전에 승인해야 하며 MIR 검증 성공은 Native 실행 성공을 뜻하지 않습니다.
