# 문서 변경 기록

## 2026-10-04 — P02 Stage A HIR/이름·타입 검사

- 사용자 답변 “P02 승인하고 이름·타입 검사까지 진행”과 최소 의미 계약을 Accepted로 기록.
- nova-hir/nova-resolve/nova-types/nova-typecheck 추가. Primitive/Unit 정규화와 SourceOrigin 보존.
- 단일 파일 forward/recursive call, lexical Scope/중복/shadow, Int32 range, call/return/condition 검사.
- print(String)과 보간 타입 검사, ErrorType 연쇄 진단 억제, recovery HIR 성공 차단.
- HIR·resolution·typed table snapshot, 정확한 code/Span frontend pass/fail harness 추가.
- MIR/Runtime/CLI/LLVM Native 실행과 미래 Stage 전체 정책은 후속 작업.
## 2026-10-04 — P01 Stage A AST/Parser

- 사용자 답변 “P01 승인하고 Stage A Parser 구현 진행” 기록. 전용 EBNF와 구문·복구 subset Accepted.
- nova-ast/nova-parser 추가. Arena/ID/byte Span, source-order Visitor, 결정적 AST dump.
- 함수/typed parameter/return type, let/return/if/else, positional call, 기본 표현식·문자열 보간.
- N1101~N1103, Synthetic Token, delimiter 동기화와 next-function 복구, nesting limit 적용.
- AST 3개 및 Parser 16개 테스트 추가. 전체 문법 초안과 Stage A 문법을 함께 기계 검증.
- 이름/타입 검사, HIR, CLI, LLVM 실행은 후속 단계. D06~D30 전체 정책은 Draft 유지.

## 2026-10-04 — 승인된 Lexer 구현 및 검증

- 사용자 승인 D01~D05를 Accepted로 기록. D06~D30은 Draft 유지.
- nova-syntax/nova-lexer 추가: lossless token, Unicode 식별자, Literal/Escape,
  중첩 주석/보간, N1001~N1003 진단, Stage A END 정규화.
- Unicode 18.0.0 데이터와 공식 unicode-ident 1.0.26 배포본·라이선스·checksum 고정.
- 초기화 없는 generic type annotation의 종료와 comparison continuation을 구분.
- Rust 테스트 30개 및 fmt/Clippy/check 통과. 문서 기계 검증 PASS.
- 전체 Parser, 의미 검사, CLI, LLVM 실행과 나머지 상세 사양 승인은 후속 작업.

## 2026-10-03 — 개발 문서 보완 Draft

- 원본 148개 NOVA 주제 각각에 전용 계약/오류/검증 문서 작성.
- 실제 Parser EBNF와 lexical/END/numeric 상세안 작성.
- 30건 semantic/tooling 결정 제안과 대안/승인 상태 기록.
- Core/std API, FFI type/ownership, CLI 및 package/diagnostic schema 초안 작성.
- 60개 conformance 묶음, 진단 코드 제안 registry, review fixtures 작성.
- 색인/감사/추적 manifest/SHA-256, 생성과 기계 검증 도구, Stage gates 추가.
- 원본 docs와 compiler crates는 수정하지 않음. 기존 Rust/LLVM/ownership/비지원 Canonical 유지.

승인/언어 규칙 적용/컴파일러 tests 통과는 이 기록의 작성 완료에 포함되지 않는다.
