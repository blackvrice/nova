# P07 고정 폭 정수 타입·승격 구현 기록

작성일: 2026-10-04. 기준: 사용자 승인 [P07](INTEGER_STAGE_B_PROPOSAL.md).
사용자 답변: “P07 승인하고 정수 타입·승격 구현 진행”.
이 기록은 현재 Compiler API와 검증 증거이며 전체 D07/Stage B 완료 선언이 아니다.

## 구현과 의미

- `nova-types`: 8종 IntKind/Type, 검증된 IntegerValue, 전체 범위 containment 기반 widening/common type.
  값의 private fields와 checked constructor가 범위를 보장한다. i128은 Core 계산용이며 Nova 타입/ABI가 아니다.
  uint64 최대값과 곱셈 overflow도 host wrap 없이 검사한다. 기존 ConstValue::Int32는 보존한다.
- `nova-typecheck`: annotation/assignment/call/return의 integer expectation, 직접 signed MIN,
  구문 기반 literal-only subtree와 peer 문맥, 최소 공통 타입 및 명시적인 use coercion을 계산한다.
  `type_table`은 원래 타입, `coercions[HirId]`는 destination TypeId다. source 타입은 type_table,
  source 위치는 같은 HIR의 Span/SourceOrigin으로 연결된다. 이미 선언된 이름의 타입은 덮어쓰지 않는다.
  Int32 `integer_values` API는 보존하고 전체 폭 payload는 `integer_literals`에 기록한다.
  debug dump는 추가 정수 payload와 source→destination 승격을 표시한다.
- `const_eval`: 폭·부호별 checked 계산과 implicit conversion을 수행한다. 변환은 stack work이며
  HIR budget에 추가하지 않는다. P05 10,000 nodes, skipped permission/type 검사와 Bool short-circuit,
  P06 dependency/cycle·print shadow 정책을 유지한다. initializer 실행 실패는 N3201이다.
- `nova-mir`: Integer Constant, `Rvalue::Widen(Operand, Type)`와 conversion temporary를 추가했다.
  연산/call/store/return 전 operand 타입을 일치시키며 SourceInfo를 유지한다.
  lower는 Resolver/Checked를 정확히 재계산·비교해 public payload/type/coercion 변조를 거부한다.
  독립 validator는 whole-range 변환, signed unary minus, 동일 타입 binary와 초기화 read를 검사한다.
- LLVM Adapter: i8/i16/i32/i64, signed/unsigned overflow intrinsic·comparison·division/remainder,
  sign/zero extension을 선택한다. division 전에 zero와 signed MIN/-1 guard를 둔다.
  failed path는 noreturn panic/Abort며 nsw/nuw 가정은 사용하지 않는다.
- private Runtime: 기존 `nova_format_int`/Int32 reason 1과 zero reason 2를 보존했다.
  `nova_format_i64`/`nova_format_u64`, 다른 정수의 reason 3/`integer overflow`를 추가했다.
  locale-independent decimal ASCII와 signed MIN/unsigned MAX를 정확히 출력한다.
  기존 print LF·String arena·OOM/I/O·Bool 출력 계약은 유지한다.

`a:int8`, `b:int8`의 `let r:int64=a+b`는 Int8 checked 연산 후 Widen이다.
`let r:int64=a+1`은 literal 1의 Int64 문맥 때문에 Int64 연산이다.
큰 unconstrained literal이나 작은 const 이름의 narrowing은 수용하지 않는다.
변환에 대한 검증은 현재 값이 아닌 source 타입의 전체 범위를 기준으로 한다.

## 검증 범위

기본 workspace 174개와 opt-in 실제 LLVM 3개/Native 19개, 총 196개 tests 통과.
P06 대비 15개 test를 추가했다. Native 실패 matrix는 52개 사례를 O0/O2에서 각각 실행해
104개 Abort 결과를 대조한다. Cargo fmt/clippy/all-features, standalone Runtime rustfmt와
문서 validator도 통과했다. 문서는 148개 원본 SHA-256, 1,702개 로컬 링크와 승인 ledger를 검사했다.

- Core 8×8 변환/common type을 독립 range oracle로 대조하고 모든 폭의 min/max, ±1,
  checked add/sub/mul/div/rem/neg 및 signed 몫/나머지를 검사한다.
- Frontend는 8×8 조합을 binding/return/call/assignment/const와 arithmetic/compare/equality에서 검사한다.
  decimal/base/underscore/alias·MIN 괄호 경계·unsigned -0·긴 literal의 정확한 N2102/Span,
  기대 타입/peer subtree·raw operation 폭·N2101·N3201·cycle/short-circuit을 검증한다.
- 10,000 HIR nodes에 2,000개 이상의 implicit conversions를 넣은 const가 통과하고
  10,001 nodes는 N3202다. 변환 자체를 budget에 세지 않는다.
- MIR의 explicit Widen·SourceInfo, illegal narrowing/signedness/비정수 변환과
  literal payload·raw type·coercion 변조의 no-codegen을 검사한다.
- LLVM 전체 폭의 IR이 결정적이고 sign/width/guard/formatter가 일치한다.
  실제 LLVM 21.1.8이 O0/O2 Windows x64 COFF와 Linux x64 ELF object를 생성·검증했다.
- Windows Native O0/O2에서 모든 폭의 함수 ABI·정상 산술·const 대조·signed 몫/나머지·MIN/MAX 보간,
  unsigned MAX만 보간하는 프로그램, source-order effects와 결과 widening 시 연산 폭을 검사한다.
  폭별 add/sub/mul/div0/rem0/signed MIN/-1/negation Abort의 이유·정확한 바이트 Span을 대조한다.
  기존 CLI의 Windows process status 부가 출력은 유지한다.
- CLI의 check/build/run은 정수 source 오류를 LLVM 호출 및 output 생성 전에 exit 1로 거부한다.
  기존 Stage A/P04/P05/P06 tests와 Hello typed/MIR/LLVM snapshots를 유지한다.

수용 예제 [integers.nova](../../examples/integers.nova)의 stdout UTF-8 bytes는 두 Native profile에서
`mixed=254, wide=253, min=-9223372036854775808, max=18446744073709551615\n`, exit 0다.

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

## 승인·후속 경계

P07은 [P06의 31-production EBNF](GRAMMAR_STAGE_B_GLOBAL_CONST.ebnf)를 재사용한다.
Lexer/AST/Parser 문법과 진단 code registry는 확장하지 않았다.
추가 상세의 전체 D07, float/char/never·source cast/bitwise/shift/wrapping API·overload,
aggregate/module/const function·ownership/Drop·public FFI·Linux Native는 후속이다.
Rust 1.99.0에서 검증했고 선언된 MSRV 1.80의 별도 실행 증거는 없다.
Linux IR/object 검증은 Linux Native 실행 증거가 아니다.
