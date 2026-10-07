# 직접 실행하는 Nova 테스트

명령은 저장소 root의 PowerShell에서 실행한다. Rust/MSVC와 Native용 LLVM 21.1.8이 필요하다.
현재 개발 완료 기능은 P13 Copy Tuple까지다.

## 현재 기능 실행 — Copy Tuple

```powershell
cargo run -p nova-cli --offline -- check examples/tuples.nova
cargo run -p nova-cli --offline -- run examples/tuples.nova --profile debug
cargo run -p nova-cli --offline -- run examples/tuples.nova --profile release
```

두 실행의 예상 출력은 다음 한 줄과 LF이며 exit 0이다.

```text
original=21, snapshot=20, shifted=21, one=7, tag=🙂
```

LLVM이 PATH에 없다면 실행 전에 설치된 clang 경로를 설정한다. 저장소 로컬 toolchain이 있을 때의 명령은 다음과 같다.

```powershell
$env:NOVA_CLANG = (Resolve-Path target/toolchains/llvm-21.1.8/bin/clang.exe).Path
```

기존 Copy struct 예제는 `examples/structs.nova`로 실행할 수 있다.

자신의 파일은 예제 경로를 `.nova` 파일 경로로 바꾸면 된다. 예를 들어 기존 `examples/function.nova`를
`cargo run -p nova-cli --offline -- check examples/function.nova`로 검사할 수 있다. 파일의 동작/통과 여부는 별도로 확인한다.

## 자동 테스트와 품질 검사

```powershell
cargo fmt --check
cargo clippy --workspace --all-targets --offline -- -D warnings
cargo test --workspace --offline
cargo check --workspace --all-features --offline
```

P13 기준 기본 tests 265개가 성공하고 실제 LLVM/Native tests 43개는 ignored로 표시된다.
이 43개를 실제 실행하려면 LLVM/Rust/MSVC 환경에서 다음을 별도로 실행한다.

```powershell
cargo test -p nova-codegen-llvm --test emission -- --ignored --test-threads=1
cargo test -p nova-cli --test native -- --ignored --test-threads=1
```

P13만 빠르게 확인하려면 다음 명령을 사용한다. LLVM 1개·Native 2개 테스트 안에서 O0/O2를 함께 검사한다.

```powershell
cargo test -p nova-typecheck --test frontend --offline p13_
cargo test -p nova-codegen-llvm --test emission p13_ -- --ignored --test-threads=1
cargo test -p nova-cli --test native p13_ -- --ignored --test-threads=1
```

Native tests는 격리된 target 경로에 실행 파일을 만들고 실행한다. ELF object 검증은 Linux Native 실행 검증이 아니다.
앞으로 기능 구현 완료 보고에는 새 예제의 check/debug/release 명령·예상 출력·기능별 테스트 명령을 함께 제공한다.

## 다음 제안 — P14 Copy Enum·statement match

[계약](docs/development-v0.1/ENUM_STAGE_B_PROPOSAL.md)과
[수용 예제·예상 출력](docs/development-v0.1/enum-proposal-fixtures/README.md)는 Draft / 미구현이다.
P14 승인·구현 전에는 이 예제가 check/run에 성공하지 않는다.
현재는 위 P13 예제와 자동 테스트를 실행할 수 있다.
P14 승인 후 실행할 check/debug/release 명령은 수용 예제 README에 미리 작성했다.
