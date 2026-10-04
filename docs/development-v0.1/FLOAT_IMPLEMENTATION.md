# P09 float 구현·검증 기록

구현·검증 기록: 2026-10-05. 2026-10-04 사용자 “P09 승인하고 float 구현 진행”에 따라
[승인 계약](FLOAT_STAGE_B_PROPOSAL.md)과 [31-production EBNF](GRAMMAR_STAGE_B_FLOAT.ebnf)를 적용했다.
기존 D04 Float token/END/escape와 P01~P08 동작, 원본 148개 문서를 유지한다.

## 구현

- AST Float leaf와 HIR 원 spelling·Span/SourceOrigin을 연결했다. HIR은 public AST의
  잘못된 float spelling도 거부한다. grammar 변경은 P08 primary에 FLOAT 추가뿐이다.
- Core FloatKind/FloatValue는 binary32/64 bits를 보존한다. metadata Eq는 bits equality로
  +0/-0를 구분하고 NaN sign/payload를 canonical 7FC00000/7FF8000000000000으로 정규화한다.
- 직접 decimal→f32/f64 parser, 기대 float·typed float peer·Float32 기본 규칙을 적용했다.
  INT의 기본/정수 peer 문맥은 유지한다. Float32+Int32는 Float64이고 Float32+Int16은 Float32다.
  소비 문맥의 승격이 원 arithmetic 폭을 바꾸지 않도록 raw type/coercion을 별도 기록한다.
  binding/assignment/call/return/const의 whole-type lossless conversion을 동일하게 검사한다.
- float unary ±, 4종 산술과 6종 비교를 지원한다. IEEE overflow/zero division 결과는
  Infinity/NaN이며 const에서도 값이다. NaN !=는 true, 나머지 비교는 false다.
  Float64→Float32·Float→int·Char/Bool 혼용·float remainder를 거부한다.
- const 엔진은 P05/P06 permission/dependency/cycle/10,000-node 예산을 유지한다.
  implicit conversion은 별도 node를 소비하지 않는다. skipped RHS도 정적 검사를 받는다.
- float 평가 host는 현재 x86_64만 지원한다. 각 Core 연산/parse/conversion/comparison이 MXCSR를
  RN ties-even·gradual underflow·masked exceptions로 설정하고 이전 control/status를 정확히 복원한다.
  asm memory barrier와 black_box input/output으로 연산을 환경 제어 안에 유지한다.
  미지원 host의 float HIR/type 검사는 명시적 UnsupportedFloatHost로 거부하며 CLI exit 3이다.
- MIR Constant::Float와 NumericConvert를 추가했다. 기존 Widen은 integer-only다.
  independent validator는 float operator/conversion·동일 operand 타입·definite initialization을 검사한다.
  Resolve/Checked 재계산 gate가 raw type/coercion·literal bits·const ±0/NaN/값/타입/count와 누락 table 변조를 거부한다.
- LLVM float/double private fastcc ABI, exact integer-bitcast constant, fadd/sub/mul/div/fneg,
  fpext/sitofp/uitofp, ordered 비교와 une를 사용한다. 연산/변환 결과 NaN은 fcmp uno/select로 정규화한다.
  fast-math/FMA contraction/reassociation을 사용하지 않는다. global-only constant 보간도 formatter declaration을 만든다.
- Runtime entry는 MXCSR 1F80을 설정한다. nova_format_f32/f64는 u32/u64 bits를 받아
  원래 폭의 shortest fixed decimal을 출력한다. 지수 표기와 불필요한 소수점/0을 없애고
  `0`, `-0`, `inf`, `-inf`, `NaN` 철자를 유지한다. UTF-8 String arena·LF/flush·OOM/I/O 정책은 그대로다.
  Vec/std::fmt::Write의 fallible reserve로 allocation 실패에 SourceInfo를 유지한다.

## 출력 동률 보정

Rust typed Display의 shortest candidate는 일부 정확한 decimal 동률에서 위쪽 숫자를 선택한다.
예: binary32 bits 4A000001은 정확히 2097152.25이며 Rust 출력은 2097152.3이다.
P09은 2097152.2를 요구한다. binary64의 562949953421312.25도 같은 경계다.

formatter는 Display candidate를 얻은 뒤 정확한 midpoint 등식
`M * 2^(E+1) == (2*C ± 1) * 10^K`를 정수로 검사해 짝수 decimal coefficient로 보정한다.
2/5의 인자를 따로 소거하므로 부동소수점 비교나 임의정밀도 라이브러리가 필요 없다.
이 계약은 Rust 출력 자체에 맡기지 않는다. 별도 rational shortest oracle로 결과를 대조했다.

## 검증

- [독립 oracle 생성기](../../tools/tests/float_oracle.py)는 Python 정수/Fraction만 사용한다.
  host float parser/연산/format·Rust 출력에 의존하지 않는다. exact rational IEEE RN encoder와
  shortest digit count→closest exact value→even decimal tie 순서로 TSV를 생성한다.
- 직접 literal midpoint·perturbation·double rounding 24개와 산술 1,599개를 bits로 대조했다.
  finite/subnormal/max·underflow/overflow·NaN/±0와 whole-type 승격 matrix를 검사했다.
- Core hostile MXCSR RC/FTZ/DAZ에서 값과 정상/오류 경로의 정확한 복원을 debug/release로 검사했다.
- 실제 Runtime formatter에 boundary·power neighbor·decimal tie·고정 seed random의 2,474개 bits를 공급했다.
  기본/release 테스트와 별도 Rust O0/O2 harness에서 rational shortest oracle stdout bytes와 일치했다.
  harness는 host 환경을 의도적으로 바꾸고 Runtime entry initializer가 1F80을 설정하는 것도 검사했다.
- frontend의 모든 conversion site·기대/peer/raw operation 폭·부적합 연산/조건과 token primary/secondary를 검사했다.
  const 허용성/skipped RHS/cycle와 10,000/10,001-node 경계도 검사했다.
- public AST/HIR spelling, 모든 ASCII truncation의 Parser 복구, MIR 불법 Widen/NumericConvert/float remainder와
  float metadata·signed zero·NaN·count/table 변조가 성공 MIR로 통과하지 않는 것을 검사했다.
- 실제 LLVM 21.1.8: float ABI/산술/비교/변환/보간을 Windows COFF·Linux ELF O0/O2 객체로 생성했다.
  integer guard·char·Hello snapshots와 기존 output 보존 검사도 통과했다.
- Windows Native O0/O2에서 독립 oracle의 const/runtime 연산 229쌍, NaN 6종 비교·±0·Infinity·subnormal,
  정수 혼합·함수 인수/return·effect 순서·loop/대입·String lifetime·FMA/reassociation 반례를 검증했다.
- 잘못된 float source는 check/build/run에서 도구 실행 전에 exit 1로 거부하고 기존 output을 보존한다.
  수용 예제 check는 LLVM 없이도 통과한다. 새 diagnostic code는 없다.

전체 기본 workspace 205개, opt-in LLVM 5개와 Native 25개, 총 235개 tests 통과.
Cargo fmt/clippy/all-features, standalone Runtime rustfmt와 문서 validator도 통과했다.
실제 Rust 1.99.0·LLVM 21.1.8·MSVC 14.44.35207 환경이다.

```powershell
cargo fmt --check
rustfmt --check --edition 2021 crates/nova-cli/runtime/stage_a.rs
cargo clippy --workspace --all-targets --offline -- -D warnings
cargo test --workspace --offline
cargo check --workspace --all-features --offline
cargo test -p nova-types --test float --release --offline
cargo test -p nova-cli --test runtime_float --release --offline
$env:NOVA_CLANG = (Resolve-Path 'target/toolchains/llvm-21.1.8/bin/clang.exe').Path
cargo test -p nova-codegen-llvm --test emission --offline -- --ignored --test-threads=1
cargo test -p nova-cli --test native --offline -- --ignored --test-threads=1
python tools/tests/float_oracle.py
node tools/docs/build-pack.mjs
node tools/docs/validate-pack.mjs
```

[floats.nova](../../examples/floats.nova)의 O0/O2 stdout은 아래와 같고 exit 0, stderr는 비어 있다.

```text
value=0.75, mixed=2.75, total=3.75
inf=inf, nan=NaN, negzero=-0, eq=false, ne=true
```

## 후속

source cast·float remainder/math API·never·aggregate/module·const function·ownership/Drop·public FFI와
전체 D07은 후속이다. x86_64 이외 float const host 제어는 미지원이다.
Linux IR/object는 Linux Native link/run 증거가 아니다. 선언된 Rust MSRV 1.80은 별도 실행하지 않았다.
