# Stage B char 구현·검증 기록 — P08

2026-10-04 사용자 “P08 승인하고 char 구현 진행” 답변으로
[P08 최소 계약](CHAR_STAGE_B_PROPOSAL.md)을 Accepted로 적용했다.
기존 승인 EBNF는 보존하고 [P08 전용 EBNF](GRAMMAR_STAGE_B_CHAR.ebnf)의 primary에 CHAR만 추가했다.
D04 Lexer/escape/END 정책과 전체 D07의 Draft 상태는 유지한다.

## 구현과 API

- Parser는 기존 Character token을 source Span/quote를 포함한 AST Character leaf로 수용한다.
  HIR Character는 검증된 Rust `char`와 SourceOrigin을 보존한다. shared escape decoder의 char mode는
  String brace doubling을 적용하지 않고 raw quote/newline·잘못된 scalar·복수 문자를 거부한다.
  malformed public AST는 LoweringError이며 사용자 오류는 기존 Lexer/Parser 진단으로 복구한다.
- Types의 `Type::Char`, `ConstValue::Char(char)`는 정수 타입/값과 구분한다.
  지역/전역 const, let/var·대입·인수·return은 같은 Char 타입만 수용한다.
  6종 비교는 scalar 값 순서의 Bool이며 정수/Bool/String/Unit 변환·혼용, 산술·unary/logical 연산은 없다.
  `print`는 String 인수만 받는다. Char 출력은 문자열 보간을 사용한다.
- const engine은 char 값/그룹/const 참조·비교를 평가한다. 기존 Bool short-circuit,
  skipped RHS의 타입/허용성 검사, 10,000 HIR node 예산과 global static cycle 검사는 유지한다.
  함수 print의 builtin shadow와 지역 print는 허용하고 전역 const print만 N2002다.
- MIR `Constant::Char(char)`와 Char Place/Call/Return을 연결했다.
  lower의 Resolve/Checked 정확한 재계산 gate는 값·type/coercion/count 변조를 거부한다.
  independent validator는 Char 비교와 보간을 허용하고 Char arithmetic·Widen·혼합 타입은 거부한다.
  기존 SourceInfo와 definite initialization/CFG 검증을 유지한다.
- LLVM private ABI는 Char를 i32 scalar로 전달/저장한다. Core의 Int32 의미와 별개다.
  equality는 eq/ne, 순서 비교는 ult/ule/ugt/uge다. Char 보간은
  `nova_format_char(ptr, i32, i32, i32, i32)`를 사용한다.
  signature/local뿐 아니라 interpolation의 global Char constant도 formatter declaration 탐지에 포함한다.
  Char가 없는 기존 Hello IR snapshot은 변경하지 않았다.
- Runtime은 private `u32`를 `char::from_u32`로 검증하고 UTF-8 1~4 bytes를 기존 String arena에 보존한다.
  NUL·줄바꿈·탭은 실제 바이트로 출력한다. invalid scalar는
  `Nova panic: invalid char scalar at file#F:S..E`와 Abort다.
  unchecked char/UTF-8 생성은 없으며 기존 integer panic reason/print LF·flush·OOM/I/O 정책은 유지한다.

## 검증

P07 대비 17개 tests 추가. 기본 workspace 187개와 opt-in LLVM 4개/Native 22개,
총 213개 tests 통과. Cargo fmt/clippy/all-features, standalone Runtime rustfmt와 문서 validator도 통과했다.
Native 전체 검사 첫 실행에서 기존 P07 정수 Abort matrix의 한 사례가 Runtime compile/link 실패로 중단됐다.
해당 실패는 stderr 상세가 비어 원인을 확정하지 못했다. 테스트 실패 출력에 문맥을 추가하고
P07 matrix 전체 104개 O0/O2 실행을 다시 검증해 통과했다. 나머지 21개 Native tests는 첫 실행에 통과했다.

- Lexer의 정확한 token spelling과 source reconstruction, escape·brace·Unicode 경계,
  empty/multi-scalar·surrogate·범위 초과·raw newline/EOF recovery, END를 검사했다.
  raw newline의 Error token은 D04대로 줄바꿈 직전에 끝나며 나머지 입력은 보존한다.
- AST/HIR 문자 leaf, quote 포함 Span, SourceOrigin, String과 다른 brace 처리,
  malformed public AST와 Unicode 소스의 모든 UTF-8 경계 truncation을 검사했다.
- Core 전체 Unicode scalar 범위의 유효성과 UTF-8 결과를 독립 range/bit encoder oracle로 대조했다.
  surrogate와 범위 초과를 거부하며 미할당/noncharacter를 임의로 제외하지 않는다.
- frontend는 U+0000/007F/0080/07FF/0800/D7FF/E000/FFFF/10000/10FFFF/0378,
  6종 비교·forward/local/global const, mutable/loop/function 경계를 검사했다.
  8종 정수·Bool/String/Unit과 binding/assignment/return/call/const/compare 혼용을 거부했다.
  const permission/type/cycle의 skipped RHS와 10,000/10,001 node 경계를 검사했다.
- MIR의 Char ABI와 materialized constant·SourceInfo, Char arithmetic/mixed type/illegal Widen,
  raw type/coercion·const 값/타입/count 변조의 no-codegen을 검사했다.
- 실제 LLVM 21.1.8이 char 함수·비교·보간 IR을 O0/O2 Windows x64 COFF와 Linux x64 ELF 객체로 생성했다.
  기존 정수 폭/부호/guard 검사와 Hello snapshots도 통과했다.
- Windows Native debug/release에서 수용 예제, global Char만의 보간, 1~4-byte UTF-8·NUL·모든 escape,
  brace·미할당/noncharacter, 6종 const/runtime 비교, 인수/return·대입·loop와 함수 effect 순서를 대조했다.
  Char 재대입과 arena 확장 후에도 이전 보간 String의 bytes가 유지됐다.
- 별도 Rust harness는 실제 Runtime 소스를 module로 컴파일하고 test-owned entry bridge만 공급했다.
  Runtime O0/O2에서 10개 valid boundary의 bytes와 D800/DFFF/110000/FFFFFFFF의 Abort·SourceInfo를 검증했다.
  사용자 소스에 cast/FFI를 추가하지 않았다.
- CLI check/build/run은 잘못된 char 소스를 LLVM 호출·output 생성 전에 exit 1로 거부했다.
  예제의 check는 LLVM이 없는 환경에서도 통과했다. 새 진단 code는 없다.

[characters.nova](../../examples/characters.nova)의 debug/release stdout UTF-8 bytes는
`letter=가, face=🙂, brace={, ordered=true\n`, exit 0이며 stderr는 비어 있다.

```powershell
cargo fmt --check
rustfmt --check --edition 2021 crates/nova-cli/runtime/stage_a.rs
cargo clippy --workspace --all-targets --offline -- -D warnings
cargo test --workspace --offline
cargo check --workspace --all-features --offline
$env:NOVA_CLANG = (Resolve-Path 'target/toolchains/llvm-21.1.8/bin/clang.exe').Path
cargo test -p nova-codegen-llvm --test emission --offline -- --ignored --test-threads=1
cargo test -p nova-cli --test native --offline -- --ignored --test-threads=1
node tools/docs/build-pack.mjs
node tools/docs/validate-pack.mjs
```

## 후속 경계

float/never·source cast·char arithmetic·String equality/indexing·Unicode collation API,
aggregate/module/const function·ownership/Drop·public FFI와 전체 D07은 후속이다.
Rust 1.99.0에서 검증했고 선언된 MSRV 1.80의 별도 실행 증거는 없다.
Linux IR/object 검증은 Linux Native link/run 증거가 아니다.
