# 문서 변경 기록

## 2026-10-04 — P06 전역 const 착수안 (Draft)

- 단일 파일 전역 const·forward visibility·dependency-first 타입/상수 평가를 제안.
- 정적 skipped RHS edge를 포함한 SCC별 N3202/chain 진단, 전역 불변 대입과 P05 node budget 재사용.
- 31-production 전용 EBNF와 Parser→Native 수용 계획을 작성. P06은 승인 대기이며 Compiler는 유지.
- Draft ledger와 문서 validator에 초안/승인 경계 검사를 추가. D06/D09 전체·module/다중 파일은 후속.

## 2026-10-04 — P05 함수 내부 const 구현

- 사용자 답변 “P05 승인하고 const 구현 진행”을 Accepted subset으로 기록.
- local const 분류와 target-independent ConstValue, iterative permission/budget/checked evaluator 추가.
- 10,000-node limit과 N3201/N3202, short-circuit와 skipped RHS 검사, 실패 ErrorType 전파.
- 성공 값만 MIR Constant로 materialize. 일반 let/var Runtime 산술은 유지하고 변조된 값/count/table 차단.
- constants.nova와 20 tests 추가. 기본 143 + 실제 LLVM/Native 16, 총 159 tests.
- Windows x64 O0/O2 UTF-8/NUL·Unit·경계/음수 산술·shadow/loop 검증. 전역 상수/전체 D09는 Draft.

## 2026-10-04 — P05 const 착수안 (Draft)

- 함수 내부 초기값 필수 const, 현재 Int32/Bool/String/Unit 값과 제한된 표현식 계약 작성.
- 기존 checked 산술의 compile-time N3201, short-circuit 허용성/평가 구분, 10,000-node/N3202 예산 제안.
- 30-production 전용 EBNF, 의미/MIR/CLI/Native 수용 기준 추가. Compiler와 accepted ledger는 유지.

## 2026-10-04 — P04 가변 지역 변수·반복문 구현

- P04 문서/승인 질문 뒤 사용자 “다음 개발 진행ㅎ재ㅝ” 요청을 해당 최소 범위 진행 승인으로 기록.
- var 가변성, direct-name 대입, while/break/continue를 AST→HIR→이름·타입→MIR→Native로 연결.
- N3004 불변 대상 대입 진단 등록, loop nesting 및 Return/Jump/Fallthrough 분석, CFG cycle 검증.
- loops.nova 예제, 24 tests 추가: 기본 125와 실제 LLVM/Native 14, 총 139 tests.
- Windows x64 O0/O2 누적/nested jump/조건·initializer 횟수/zero iteration/return/문자열·Abort 검증.
- 전체 Stage B/ownership/Drop와 Linux Native는 후속. P03 Runtime ABI·String arena 유지.

## 2026-10-04 — P04 Stage B 착수안 (Draft)

- var/direct-name 대입/while/break/continue의 문법·의미·진단·MIR·Native 수용 기준 작성.
- P01을 확장하는 30-production 전용 EBNF 초안 추가. N3004 불변 대상 대입 진단 제안.
- 승인 ledger와 compiler는 유지. P04 사용자 승인 후 구현하며 전체 Stage B 승인이 아니다.

## 2026-10-04 — P03 Stage A Native

- 사용자 답변 “P03 승인하고 Stage A Native 구현 진행”을 Accepted subset으로 기록.
- nova-codegen/nova-codegen-llvm/nova-cli 추가. Verified CodegenUnit, LLVM IR verify/Object, private Runtime ABI.
- Windows x64 MSVC Runtime compile/link, main entry, check/build/run debug/release, 안전한 신규 출력 경로.
- Checked Int32/Abort, UTF-8+LF print, decimal/Bool 보간과 실행 단위 String arena 구현.
- LLVM 21.1.8 공식 배포 hash 고정. Hello와 Unicode/NUL/short-circuit/산술 실패 O0/O2 Native 검증.
- 기본 105 tests와 opt-in 실제 LLVM/Native 10 tests, 재현 가능한 Hello executable/기존 출력·I/O 실패 검증.
- Linux x64는 LLVM IR/ELF Object만 검증. Linux Native link/run, 전체 ABI/FFI/ownership/Package는 후속.

## 2026-10-04 — Stage A MIR Lowering·검증

- nova-mir 추가: non-SSA Place 기반 BasicBlock CFG와 typed operand/local/signature/source origin.
- Call Terminator, source-order lowering, short-circuit 분기, if/return/Unit fallthrough.
- 독립 MIR validator: ID/type/signature/source/terminator와 must-initialized CFG dataflow.
- upstream 오류와 변조된 의미 table 성공 차단. MIR 18 tests, 총 91 Rust tests.
- 원본 NOVA-081~083/091 및 P02 승인 평가 순서를 적용. 새 언어 의미 변경/추가 승인 없음.
- Arithmetic/보간은 abstract 연산. Runtime/print/entry/ABI, LLVM/CLI/Native 실행은 후속.

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
