# 직접 실행하는 Nova 테스트

명령은 저장소 root의 PowerShell에서 실행한다. Rust/MSVC와 Native용 LLVM 21.1.8이 필요하다.
현재 개발 완료 기능은 P17 함수 이름 인수까지다.

## 다음 개발 준비 — 상수 표현식 함수 기본 인수 (P18 Draft)

[P18 계약](docs/development-v0.1/DEFAULT_ARGUMENTS_STAGE_B_PROPOSAL.md)·[제안 fixture/구현 후 명령/20줄 기대 출력](docs/development-v0.1/default-arguments-proposal-fixtures/README.md)을 준비했다.
기본 인수는 미승인·미구현이다. 현재 실행 가능한 명령은 아래 P17 예제다.
문서 metadata 검사는 `node tools/docs/validate-pack.mjs`로 실행하며 기대 결과는 PASS다.
이 검사는 P18 source의 compile/Native 성공 검사가 아니다.

## 현재 기능 실행 — 함수 이름 인수

```powershell
cargo run -p nova-cli --offline -- check examples/named_arguments.nova
$env:NOVA_CLANG = (Resolve-Path target/toolchains/llvm-21.1.8/bin/clang.exe).Path
cargo run -p nova-cli --offline -- run examples/named_arguments.nova --profile debug
cargo run -p nova-cli --offline -- run examples/named_arguments.nova --profile release
```

check는 출력 없이 exit 0이며 두 Native 실행은 다음 24줄과 각 LF, stderr 없음·exit 0이다.

```text
right
left
reverse=702
positional
named
mixed=102
second
first
text=first/second
unit=3
unicode=304
minimum=-12500
copy=4/5
option=7
leaf
later
after
success=102
leaf
error=-1
before
leaf
prior-error=-1
short=false
```

[P17 계약](docs/development-v0.1/NAMED_ARGUMENTS_STAGE_B_PROPOSAL.md)·[구현 기록](docs/development-v0.1/NAMED_ARGUMENTS_IMPLEMENTATION.md)·[두 파일 수용 예제](docs/development-v0.1/named-arguments-proposal-fixtures/README.md)를 제공한다.
이름은 원 함수 parameter와 대응하며 label과 value 이름 공간은 별개다. source-order 평가 후 parameter-order로 전달한다.
기본 인수·named constructor·overload는 지원하지 않는다.

```powershell
cargo test --workspace --offline p17_
cargo test -p nova-codegen-llvm --test emission --offline p17_ -- --ignored --test-threads=1
cargo test -p nova-cli --test native --offline p17_ -- --ignored --test-threads=1
cargo run -p nova-cli --offline -- check docs/development-v0.1/named-arguments-proposal-fixtures/unknown_label.nova
```

집중 검사는 기본 9개·LLVM 1개·Native 2개다. 마지막 명령은 unknown label `other`에 N2201·exit 1이 예상된다.
문서 metadata 검사는 `node tools/docs/validate-pack.mjs`로 실행하며 기대 결과는 PASS다.
Compiler/Native 실행 검증과 구분한다.

## 기존 기능 실행 — Copy try·Result 오류 전파

```powershell
$env:NOVA_CLANG = (Resolve-Path target/toolchains/llvm-21.1.8/bin/clang.exe).Path
cargo run -p nova-cli --offline -- check examples/try_result.nova
cargo run -p nova-cli --offline -- run examples/try_result.nova --profile debug
cargo run -p nova-cli --offline -- run examples/try_result.nova --profile release
```

check는 출력 없이 exit 0이다. debug/release는 다음 19줄과 각 LF, stderr 없음·exit 0이다.

```text
start
leaf
second
after
ok=8
start
leaf
error=-1
leaf
nested=7
leaf
nested-error=-1
ping
unit=success
snapshot=9
short=false
leaf
leaf
loop=7
```

[계약](docs/development-v0.1/TRY_STAGE_B_PROPOSAL.md)·[구현 기록](docs/development-v0.1/TRY_IMPLEMENTATION.md)·
[두 파일 import/alias fixture](docs/development-v0.1/try-proposal-fixtures/README.md)를 제공한다.
P16 집중 검사는 기본 10개·실제 LLVM 1개·Native 2개이며 opt-in 검사는 O0/O2를 포함한다.

```powershell
cargo test --workspace --offline p16_
cargo test -p nova-codegen-llvm --test emission --offline p16_ -- --ignored --test-threads=1
cargo test -p nova-cli --test native --offline p16_ -- --ignored --test-threads=1
cargo run -p nova-cli --offline -- check docs/development-v0.1/try-proposal-fixtures/error_width.nova
```

마지막 check는 int8/Int16 Error 타입이 정확히 같지 않아 try keyword의 **N2101·exit 1**이다.

## 현재 기능 실행 — Copy Option·Result·nullable

```powershell
$env:NOVA_CLANG = (Resolve-Path target/toolchains/llvm-21.1.8/bin/clang.exe).Path
cargo run -p nova-cli --offline -- check examples/option_result.nova
cargo run -p nova-cli --offline -- run examples/option_result.nova --profile debug
cargo run -p nova-cli --offline -- run examples/option_result.nova --profile release
```

check는 성공 시 출력 없이 exit 0, 두 실행은 다음 8줄과 각 LF, stderr 없음·exit 0이다.

```text
maybe
some=7
original=none
success=8, flag=true
error=-1
nested=none
private=9
unit=success
```

[계약](docs/development-v0.1/OPTION_RESULT_STAGE_B_PROPOSAL.md),
[구현 기록](docs/development-v0.1/OPTION_RESULT_IMPLEMENTATION.md),
[두 파일 import·private factory fixture 실행](docs/development-v0.1/option-result-proposal-fixtures/README.md)을 제공한다.
P15의 빠른 검증은 기본 회귀 11개, 실제 LLVM 1개, Native 2개다. O0/O2는 opt-in 테스트 안에서 검사한다.

```powershell
cargo test --workspace --offline p15_
cargo test -p nova-codegen-llvm --test emission --offline p15_ -- --ignored --test-threads=1
cargo test -p nova-cli --test native --offline p15_ -- --ignored --test-threads=1
cargo run -p nova-cli --offline -- check docs/development-v0.1/option-result-proposal-fixtures/untyped_none.nova
```

마지막 명령은 `none`의 타입 문맥 부족으로 N2103·exit 1이 예상된다.
문맥 없이 `Option::Some(7)`은 Option<int32>지만 `none` 및 Result 생성자는 완성된 기대 타입이 필요하다.
`let x:int8?=Option::Some(7)`처럼 타입을 지정할 수 있다. String/Move payload·사용자 Generic은 후속이다. try는 P16을 따른다.

## 기존 기능 실행 — Copy Enum·match

LLVM이 PATH에 없다면 저장소 로컬 toolchain의 clang 경로를 먼저 설정한다.

```powershell
$env:NOVA_CLANG = (Resolve-Path target/toolchains/llvm-21.1.8/bin/clang.exe).Path
cargo run -p nova-cli --offline -- check examples/enums.nova
cargo run -p nova-cli --offline -- run examples/enums.nova --profile debug
cargo run -p nova-cli --offline -- run examples/enums.nova --profile release
```

두 실행의 예상 출력은 다음 네 줄과 각 LF이며 stderr 없음, exit 0이다.

```text
make
sum=30, code=7, flag=true
original=empty
ok
```

[Enum·match 계약](docs/development-v0.1/ENUM_STAGE_B_PROPOSAL.md),
[구현·검증 기록](docs/development-v0.1/ENUM_IMPLEMENTATION.md),
[import alias·private factory의 두 파일 실행 명령](docs/development-v0.1/enum-proposal-fixtures/README.md)을 제공한다.
기존 Copy Tuple/struct 예제는 `examples/tuples.nova`, `examples/structs.nova`로 실행할 수 있다.
Tuple 예상 출력은 `original=21, snapshot=20, shifted=21, one=7, tag=🙂` 한 줄과 LF다.

자신의 파일은 예제 경로를 `.nova` 파일 경로로 바꾸면 된다. 예를 들어 기존 `examples/function.nova`를
`cargo run -p nova-cli --offline -- check examples/function.nova`로 검사할 수 있다. 파일의 동작/통과 여부는 별도로 확인한다.

## 자동 테스트와 품질 검사

```powershell
cargo fmt --check
rustfmt --check --edition 2021 crates/nova-cli/runtime/stage_a.rs
cargo clippy --workspace --all-targets --offline -- -D warnings
cargo test --workspace --offline
cargo check --workspace --all-features --offline
```

P17 기준 기본 tests 306개가 성공하고 실제 LLVM/Native tests 55개는 ignored로 표시된다.
이 55개(LLVM 13개·Native 42개)를 실제 실행하려면 LLVM/Rust/MSVC 환경에서 다음을 별도로 실행한다.

```powershell
cargo test -p nova-codegen-llvm --test emission --offline -- --ignored --test-threads=1
cargo test -p nova-cli --test native --offline -- --ignored --test-threads=1
```

P14만 빠르게 확인하려면 다음 명령을 사용한다. LLVM 1개·Native 2개 테스트 안에서 O0/O2를 함께 검사한다.

```powershell
cargo test --workspace --offline p14_
cargo test -p nova-codegen-llvm --test emission --offline p14_ -- --ignored --test-threads=1
cargo test -p nova-cli --test native --offline p14_ -- --ignored --test-threads=1
```

누락 variant 검사를 직접 확인하면 `N3101`과 exit 1이 나온다.

```powershell
cargo run -p nova-cli --offline -- check docs/development-v0.1/enum-proposal-fixtures/missing_variant.nova
```

Native tests는 격리된 target 경로에 실행 파일을 만들고 실행한다. ELF object 검증은 Linux Native host 실행 검증이 아니다.
앞으로 기능 구현 완료 보고에는 새 예제의 check/debug/release 명령·예상 출력·기능별 테스트 명령을 함께 제공한다.
