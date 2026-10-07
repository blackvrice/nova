# 직접 실행하는 Nova 테스트

명령은 저장소 root의 PowerShell에서 실행한다. Rust/MSVC와 Native용 LLVM 21.1.8이 필요하다.
현재 개발 완료 기능은 P14 Copy Enum·statement match까지다.

## 현재 기능 실행 — Copy Enum·match

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

P14 기준 기본 tests 276개가 성공하고 실제 LLVM/Native tests 46개는 ignored로 표시된다.
이 46개를 실제 실행하려면 LLVM/Rust/MSVC 환경에서 다음을 별도로 실행한다.

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
