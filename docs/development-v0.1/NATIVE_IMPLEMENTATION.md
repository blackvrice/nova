# Stage A Native 구현·사용·검증 기록

작성일: 2026-10-04. [사용자 승인 P03](NATIVE_STAGE_A_PROPOSAL.md)의 최소 부분을 구현했다.
전체 D07~D28, Public ABI/FFI/ownership/Package 승인이나 전체 Nova 구현 완료가 아니다.

## 산출물과 의존 경계

- `nova-codegen`: immutable verified CodegenUnit, backend-neutral trait/options/target/artifact/error.
  공개 mutable MIR는 validator를 통과해야 하며 Unit에는 mutable MIR 접근이 없다.
  executable entry는 LLVM와 독립적으로 검사한다. missing main N2001, duplicate N2002,
  parameter N2201, non-Unit return N2101을 Source Span과 함께 출력한다.
- `nova-codegen-llvm`: LLVM 21.1.8 textual IR Adapter. 내부 함수는 deterministic private ID/fastcc,
  stack Place 기반이며 LLVM 타입/IR/프로세스 호출을 이 crate에 격리한다.
  Int32 i32, Bool i1, Unit value empty aggregate/return void, String `{ptr,i64}`다.
  C Runtime 경계는 scalar/pointer/out-pointer이며 public Nova/C FFI ABI가 아니다.
- `nova-cli`: `nova` executable, single-file frontend/MIR check와 Native build/run.
  Rust Runtime source를 embed하고 object와 rustc/MSVC로 링크한다. LLVM Rust binding 의존은 없다.
- Runtime은 `crates/nova-cli/runtime/stage_a.rs`. unsafe는 생성된 buffer/length/out-pointer 계약의
  private C boundary에 한정한다. panic=abort, 실행 단위 String arena는 정상 main 종료 후 해제한다.
  함수별 Drop/ownership 분석이나 peak memory 최적화는 후속이다.

## 도구와 명령

Compiler와 기본 tests에는 LLVM 설치가 필요 없다. Native는 LLVM/Clang **21.1.8**, Rust와
Windows x64 MSVC linker/SDK가 필요하다. 이번 환경은 Rust 1.99.0/MSVC 14.44.35207이다.
rust-version 1.80은 선언된 MSRV이며 해당 toolchain에서 별도 실행하지 않았다.

[공식 LLVM 배포](https://github.com/llvm/llvm-project/releases/tag/llvmorg-21.1.8)의
`LLVM-21.1.8-win64.exe` SHA-256:
`7a5386c26497db1691f320121e5b113364dd0274b98e55f15f4dbc00c0450113`.
현재 workspace는 도구를 `target/toolchains/llvm-21.1.8/bin`에 압축 해제했다.
전역 설치/PATH는 바꾸지 않았다. binary는 ignored target 안에 있고 GitHub에는 포함되지 않는다.
다른 환경은 공식 배포를 준비하고 NOVA_CLANG을 설치된 clang.exe로 지정한다.

Workspace root의 PowerShell:

```powershell
$env:NOVA_CLANG = (Resolve-Path 'target/toolchains/llvm-21.1.8/bin/clang.exe').Path
cargo run --offline -p nova-cli -- check examples/hello.nova
cargo run --offline -p nova-cli -- run examples/hello.nova
cargo run --offline -p nova-cli -- build examples/hello.nova -o hello.exe
cargo run --offline -p nova-cli -- run examples/hello.nova --profile release
```

일반 설치는 해당 clang 경로 또는 PATH의 clang을 사용한다. NOVA_RUSTC도 지정할 수 있으며
기본은 PATH의 rustc다. 다른 Clang 버전은 거부한다. Codex의 제한된 process 환경에서는
Clang이 permission denied로 끝날 수 있다. 이번 실제 시험은 승인된 host execution 권한으로 수행했다.

## CLI와 출력 보존

- check: UTF-8/frontend/MIR 검사, main/LLVM 불필요, object/link/build directory 없음.
  성공 stdout/stderr는 비어 있고 exit0이다.
- build: main()→Unit 검사 후 verify/object/link. executable 경로를 stdout에 출력한다.
  `-o <new-path>`는 기존 파일을 덮어쓰지 않는다.
- run: build 성공 뒤 child stdout/stderr 연결, 0..255 exit 전달.
  OS abort/Windows 큰 종료 값은 stderr에 상태를 기록하고 nonzero(1)로 전달한다.
- debug는 Nova Object O0, release는 O2. Runtime은 양쪽 모두 Rust O0/abort이다.
  checked 실패/source order는 profile과 무관하다. size/custom target/argv/JSON/dump/manifest/package/clean은 미지원이다.
- 내부 산출물은 target/nova-stage-a의 고유 directory에 보존한다. source는 수정/삭제하지 않는다.
  human diagnostic은 stderr, build 경로는 stdout, run stdout은 child bytes다.
- success0/source1/CLI2/toolchain-link3/invariant101. invalid syntax/type/main은 output/tool invocation 전에 거부한다.
- argument array 사용. 큰 IR stdin과 tool stdout/stderr를 동시에 처리해 pipe 교착을 피한다.
  Object create_new와 신규 -o만 사용하며 failed object/copy의 새 파일은 제거한다.
- `/Brepro`와 Runtime source path remap으로 반복 Hello executable bytes 일치를 확인했다.

## LLVM·Runtime 의미

- [LLVM Language Reference](https://releases.llvm.org/21.1.0/docs/LangRef.html)를 기준으로 IR 생성.
  version probe→IR parse/verify→별도 object emission, COFF x64/ELF x64 header 검사를 수행한다.
- Local/보간 array alloca는 function entry. MIR CFG/short-circuit/source-order Call을 유지한다.
  Bool/Int32 비교, Unit parameters, forward/recursive/grouped call을 지원한다.
- `+ - *`/unary minus는 signed overflow intrinsic/guard. `/ %`는 zero와 MIN/-1 guard 후 sdiv/srem.
  remainder MIN/-1도 P03대로 Abort. signed UB/nsw를 언어 의미로 삼지 않으며 frontend constant folding은 추가하지 않았다.
- print는 length 기반 UTF-8 bytes+LF/flush. NUL을 잘라내지 않는다. I/O 실패는 best-effort stderr 이유+Span 후 Abort.
- 보간은 decimal Int32/true·false/원본 String. String 반환/인수/nested 보간도 private representation과 arena로 유지한다.
  allocation/count/length overflow 검사. OOM은 try_reserve 오류에서 Abort하며 실제 OOM 강제 시험은 하지 않았다.
- SourceInfo는 IR comments와 IrArtifact source table, runtime에는 FileId/byte Span을 전달한다.
  filename/line-column/stack trace symbolization, DWARF/PDB debugger 연동은 없다.

## 검증·지원표

| 대상 | 증거 | 상태 |
|---|---|---|
| Windows x64 MSVC | verify/COFF/Object/Runtime link/Native O0·O2 | 이번 host 검증 |
| Linux x64 GNU | verify/ELF x64 Object O2 | cross-object 검증, Native link/run 미검증 |
| AArch64/기타 | 없음 | 명시적 미지원 |

P03 완료 당시 기본 **105 tests**, opt-in 실제 LLVM/Native **10 tests**, 총 **115 tests**였다.
후속 반복문 Native 증거는 [P04 구현 기록](CONTROL_IMPLEMENTATION.md),
P05 const Native 증거는 [P05 구현 기록](CONST_IMPLEMENTATION.md), 이후 P06 포함 현재 숫자와
전역 const Native 증거는 [P06 구현 기록](GLOBAL_CONST_IMPLEMENTATION.md)을 따른다.
기본 Cargo test만으로 ignored Native tests가 실행됐다고 보고하지 않는다.

```powershell
cargo fmt --check
rustfmt --check --edition 2021 crates/nova-cli/runtime/stage_a.rs
cargo clippy --workspace --all-targets --offline -- -D warnings
cargo test --workspace --offline
cargo check --workspace --all-features --offline
node tools/docs/validate-pack.mjs
cargo test -p nova-codegen-llvm --offline --test emission -- --ignored --test-threads=1
cargo test -p nova-cli --offline --test native -- --ignored --test-threads=1
```

- 기본 14 추가: CodegenUnit/target/error, deterministic IR/source/snapshot, guard/string escaping/entry,
  unsupported target/missing tool/version/Unit-Bool ABI, CLI usage/check/no-artifact/type/main errors.
- 실제 LLVM 2: invalid IR rejection, x64 COFF/ELF verify/object, existing object preservation.
- 실제 Native 8: Hello LF/exit, Unicode/NUL/Int32 boundaries/Bool, call order/short-circuit/recursion/branch,
  signed division/remainder, 8 checked failure cases O0/O2, optimized String/control 동등성,
  deterministic executable/repeated -o 보호, read-only stdout 실패/unchanged sink/exact call Span.
- CI는 Linux default checks와 isolated Windows real LLVM/Native job을 갖는다.
  Local 증거와 새 GitHub Actions run의 완료는 별개다.

선택된 Stage A Native subset은 실행 가능하다. Linux Native/full ABI/foreign/modules,
ownership/Drop/borrow/package/query/cache와 나머지 Stage B 기능은 후속이다.
P04 가변 변수·반복문은 별도 승인 범위로 구현했다.
P05 함수 내부 const도 별도 승인 범위로 구현했다.
