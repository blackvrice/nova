# P10 숫자 cast 구현·검증 기록

구현·검증일: 2026-10-05. 사용자 “P10 승인하고 숫자 cast 구현 진행”으로 승인한
[숫자 변환 계약](CAST_STAGE_B_PROPOSAL.md)과 [31-production 전용 EBNF](GRAMMAR_STAGE_B_CAST.ebnf)를 적용했다.
D01~D05/P01~P09의 기존 의미와 원본 148개 NOVA 문서를 유지한다.

## 구현

- `as`는 call과 같은 postfix 층에서 source order로 왼쪽 결합하며 prefix보다 강하다.
  AST/HIR은 value/type child와 전체 cast Span, keyword Span, type alias 원본 SourceOrigin을 보존한다.
  malformed public AST의 keyword spelling/위치/child shape는 거부한다.
  cast 뒤 `<`는 공백과 무관하게 비교 연산자다. 기존 선언의 미지원 generic 진단은 유지한다.
- 8종 정수와 Float32/64, 기존 숫자 alias의 100가지 source/target 조합을 허용한다.
  cast operand는 target/consumer expected context를 받지 않고 기존 literal 기본/peer 타입으로 검사한다.
  결과에는 기존 lossless consumer 승격을 적용할 수 있다. 함수 값의 cast는 N2101이다.
  nonnumeric source/target은 N2101, undefined target은 N2001, source literal 범위 초과는 N2102다.
- Core `ConstValue::checked_cast`와 CastError는 LLVM에 의존하지 않는다.
  integer→integer는 원 수학적 값의 범위를 검사한다. UInt64는 양수 i128 값으로 보존한다.
  integer→float는 원 signed/unsigned i64/u64에서 목적 f32/f64로 직접 RN ties-even 반올림한다.
  f64 중간값을 거쳐 f32로 바꾸지 않는다.
- float→integer는 IEEE bits를 정수 significand/exponent로 해석해 소수 부분을 제거한 뒤 목적 범위를 검사한다.
  rounded float MAX 경계나 Rust saturating float-to-int 결과를 사용하지 않는다.
  NaN/Infinity는 실패, 음수 소수의 truncated zero와 ±0는 unsigned 0도 허용한다.
- Float32→Float64는 정확한 확장이다. Float64→Float32는 직접 RN이며 finite source가 Infinity로
  반올림되면 실패한다. source Infinity는 허용하고 NaN은 P09 canonical bits로 정규화한다.
  zero sign/subnormal/underflow를 보존한다. identity numeric cast도 허용한다.
- float 계산은 P09의 MXCSR RN/FTZ·DAZ off/masked exceptions 제어와 control/status 복원을 유지한다.
  black_box input/output은 conversion을 환경 제어 안에 유지한다. host 지원은 현재 x86_64다.
- const permission/type/cycle는 skipped RHS도 검사한다. 값 평가의 logical short-circuit는 유지한다.
  evaluated cast 실패는 N3201이며 primary는 전체 실패 cast, secondary는 const 선언이다.
  cast는 1 node + operand subtree로 계산하고 target type syntax와 implicit widening은 예산에서 제외한다.
  initializer 10,000-node 예산 초과는 N3202다. 일반 let/var/return/call의 값 실패는 compile error로 접지 않는다.
- MIR `CheckedCast(Operand, Type)`는 legacy integer `Widen`/lossless `NumericConvert`와 구분된다.
  independent validator는 numeric pair/result type/initialization/SourceInfo를 검사한다.
  Resolve/Checked 재계산 gate는 cast raw target/operand coercion·const 값/count와 누락 table 변조를 거부한다.
- LLVM integer cast는 sext/zext to i128 scratch → 수학적 범위 guard → 목적 폭 trunc 순서다.
  i128은 LLVM 구현용이며 새 Nova semantic type이 아니다. integer→float는 원 폭의 sitofp/uitofp다.
  float→int는 llvm.trunc.f32/f64 → ordered [MIN, exclusive upper power-of-two) guard → 안전 edge의
  fptosi/fptoui다. invalid 경로의 poison 결과를 소비하지 않는다.
  Float64→Float32는 fptrunc 후 source finite/output infinity bits를 검사한다.
  NaN canonicalization을 유지하고 fast-math/reassociation을 사용하지 않는다.
- Runtime reason 4는 `numeric cast out of range`다. 첫 실패에서 Abort하며 전체 cast SourceInfo를 보고한다.
  operand는 한 번 평가하고 이후 효과를 실행하지 않는다. 기존 reason 1~3/ABI/formatter는 유지한다.

## 검증

- [독립 oracle](../../tools/tests/cast_oracle.py)은 Python 정수/Fraction과 P09 rational IEEE encoder를 사용한다.
  host float 연산/변환/format나 Rust 출력에서 기대값을 얻지 않는다. 9,000개 deterministic TSV 사례를
  Core debug/release에서 bits 또는 수학적 정수로 대조했다. 숫자 100조합, MIN/MAX/±1, 2^63/2^64,
  NaN/±Infinity/±0/subnormal, finite narrowing overflow와 midpoint/underflow, 직접 f32 double-rounding 반례를 포함한다.
- Core hostile MXCSR에서 직접 integer→float와 Float64→Float32 정상/실패 경로의 값과 정확한 환경 복원을
  debug/release에서 검사했다. nonnumeric/identity 계약도 검사했다.
- Parser precedence/call·cast chain/as 전후 newline/비교/bare return/모든 ASCII truncation 및 malformed target
  recovery, HIR alias/origin/public AST 검사를 수행했다.
- frontend numeric 100조합과 문맥 격리, literal overflow, nonnumeric/undefined type, consumer widening,
  const skipped permission/cycle/full primary·secondary와 10,000/10,001-node 경계를 검사했다.
- MIR illegal CheckedCast/result mismatch/uninitialized read와 cast target/coercion/const 값/count/누락 table gate를 검사했다.
  기존 Hello AST/HIR/typed/MIR/LLVM snapshots는 그대로 통과한다.
- 실제 LLVM 21.1.8에서 숫자 100조합을 Windows COFF/Linux ELF O0/O2 객체로 검증했다.
  emitter의 ordered guards와 안전 edge의 fptoi, 직접 integer RN, NaN 정규화와 기존 LLVM 회귀도 통과했다.
- Windows Native O0/O2에서 독립 oracle 957개 const/runtime 쌍의 stdout bytes가 일치했다.
  수용 예제와 signed zero/subnormal/Infinity/NaN·64-bit safe edge·cast chain·var 재대입을 검사했다.
  30가지 실패 프로그램을 두 profile에서 실행해 한 번의 효과·Abort reason 4·UTF-8 전체 cast byte Span·후속 효과 중단을 검사했다.
- CLI check/build/run의 type/literal/const 실패는 도구 실행 전에 exit 1이며 기존 output을 보존한다.
  수용 예제와 Runtime 값 실패를 포함한 일반 let cast는 LLVM 없이 check가 성공한다.
- 전체 기본 workspace **217개**, opt-in LLVM **6개**, Windows Native **28개**, 총 **251개** tests 통과.
  Core release cast 3개도 별도 통과했다. fmt/clippy/all-features/standalone Runtime rustfmt와 문서 validator를 수행했다.
  병렬 전체 Native 첫 실행은 기존 테스트 4개에서 MSVC link 중복/라이브러리 손상 오류가 났고,
  전체 `--test-threads=1` 재실행은 28/28 통과했다. 테스트 요구사항을 완화하지 않았다.

```powershell
cargo fmt --check
rustfmt --check --edition 2021 crates/nova-cli/runtime/stage_a.rs
cargo clippy --workspace --all-targets --offline -- -D warnings
cargo test --workspace --offline
cargo check --workspace --all-features --offline
cargo test -p nova-types --release --test casts --offline
$env:NOVA_CLANG=(Resolve-Path 'target/toolchains/llvm-21.1.8/bin/clang.exe').Path
cargo test -p nova-codegen-llvm --test emission -- --ignored
cargo test -p nova-cli --test native -- --ignored --test-threads=1
python tools/tests/cast_oracle.py
node tools/docs/build-pack.mjs
node tools/docs/validate-pack.mjs
```

실제 환경은 Rust 1.99.0·LLVM 21.1.8·Windows x64/MSVC 14.44.35207이다.
선언된 Rust MSRV 1.80은 별도 toolchain으로 검사하지 않았다. Linux ELF 객체 생성은 Linux Native link/run의 증거가 아니다.
검증 로그는 로컬 target/p10-*.log에 있으며 target은 Git artifact로 저장하지 않는다.

## 수용 예제

[examples/casts.nova](../../examples/casts.nova)의 stdout은 다음 bytes + LF 두 개이며 exit 0이다.

```text
small=127, whole=127, zero=0, skipped=false
rounded=16777216, value=2.25, back=2
```

## 남은 범위

Bool/Char/String/Unit 변환·unsafe/bitcast·wrapping/saturating/Option cast API·public FFI,
float remainder/math API·aggregate/module·const function·Linux Native와 전체 D07은 승인/구현 범위가 아니다.
후속 기능은 별도 최소 계약을 작성해 검토한다. P10의 승인으로 다른 Draft가 자동 승인되지는 않는다.
