# Nova 0.1 compiler

개발 전에 읽을 [전체 개발 문서 보완팩](docs/development-v0.1/README.md)을 작성했습니다.
148개 주제별 문서와 구체 EBNF, 30건 결정 초안, API/schema, 수용 테스트 계획을 포함합니다.
새 상세는 Draft이며 기존 Canonical 결정을 변경하지 않았습니다.

Nova 컴파일러의 첫 Stage A 기반 구현입니다. 언어 사양은 `docs/`의 원본
Documentation Pack과 사용자가 제공한 Canonical Decisions를 따릅니다.
사양 파일의 의미는 변경하지 않았습니다.

## 현재 구현

- `nova-core-ids`: 데이터베이스 내에서 안정적인 정수 `FileId`.
- `nova-source`: append-only UTF-8 `SourceDatabase`, 반열린 바이트 `Span`,
  지연 계산되는 `LineIndex`, Unicode scalar 기준의 1-based 표시 위치.
- `nova-diagnostics`: `DiagnosticCode`, Severity, Primary/Secondary Label,
  Note, Suggestion 및 Plain/ANSI/JSON/Snapshot Renderer.

의존 방향은 `nova-diagnostics → nova-source → nova-core-ids`입니다.
외부 Rust 라이브러리 의존성은 없습니다. Rust 1.80 이상이 필요합니다.

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

1. Lexer/Token/END 정규화: NOVA-009~013, NOVA-071에 맞춰 구현.
2. Parser/AST: 실제 EBNF Production 확인 후 구현.
3. HIR, 최소 이름/타입 검사, Compile-pass/fail Harness.
4. MIR와 LLVM Adapter, Hello Nova E2E.

제공된 NOVA-014는 일반 요구사항을 담고 있지만 실제 EBNF Production은 없습니다.
Parser 구현 전에 문법을 보완하거나 별도 공식 문법 자료를 받아야 하며, 구현으로
언어 문법을 임의로 결정하지 않습니다.

현재 Lexer, Parser, CLI 및 LLVM Backend는 구현하지 않았습니다.
따라서 `nova check`와 `nova run`은 아직 제공하지 않습니다. 이번 테스트는 Rust
기반 계층의 UTF-8, 범위 오류, EOF, 혼합 줄바꿈, 대형 파일, 진단 Snapshot,
JSON escaping 및 Suggestion 위치 검증을 다룹니다.
