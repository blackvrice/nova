# P12 Copy struct 구현·검증 기록

승인일: 2026-10-05. 구현 완료일: 2026-10-07. 검증 기간: 2026-10-05~2026-10-07.
사용자 “P12 승인하고 Copy struct 구현 진행”으로 승인한 [최소 계약](STRUCT_STAGE_B_PROPOSAL.md)과
[37-production EBNF](GRAMMAR_STAGE_B_STRUCT.ebnf)를 적용했다. 기존 승인 의미와 원본 148개 NOVA 문서를 보존한다.

## 구현과 경계

- AST/Parser와 HIR에 top-level struct, let/var field, postfix projection, bare local root의 field 경로 대입을 추가했다.
  field END·Unicode·기존 call/as 결합을 유지한다. 잘린 입력과 누락된 닫는 brace는 진단 후 다음 선언으로 복구한다.
- Resolver는 type/value namespace를 분리하고 원 StructDefId로 forward type과 type import alias를 연결한다.
  같은 이름의 type/value를 함께 import할 때 두 binding을 원자적으로 검사한다. primitive 예약 이름,
  private type/field, failed import의 파생 오류 억제와 value-first constructor lookup을 적용한다.
  공개 factory가 반환한 private nominal type을 추론하는 것은 허용하며 field 접근에는 원 visibility를 검사한다.
- Core Types의 StructId/FieldId, StructRegistry와 TypeChecker는 nominal identity, Copy 구성,
  위치 인수 생성·기대 literal/손실 없는 승격·field 읽기·경로 가변성을 검사한다.
  모든 경로 field와 local root가 var여야 변경 가능하다. RHS를 한 번 평가한 뒤 저장하므로 이전 root 값을 읽을 수 있다.
  let snapshot·인수·return·const 복사는 원본과 독립된 값이다. String field와 aggregate 연산/보간/cast는 거부한다.
- 지역/전역 const에 순수 constructor/projection을 연결했다. constructor head와 field 이름은 예산 node로 세지 않는다.
  cached reference는 기존 1 node, initializer 예산은 10,000 node다. 모든 생성 인수와 생략 logical RHS의 정적
  dependency를 검사한다. scalar checked 실패 N3201과 const cycle/budget N3202를 보존한다.
- by-value cycle은 사용 여부와 무관하게 N2101이다. 반복형 graph 처리 후 checked x64 layout을 계산한다.
  선언 순서·자연 alignment/padding, Unit/빈 struct의 0-byte 논리 field를 유지한다.
  bundle 1,024 struct, 타입당 1,024 직접 field, 깊이 128, size 1 MiB, transitive field occurrence 65,536 상한은 N8901이다.
  malformed child metadata의 잘못된 alignment/depth/size와 arithmetic overflow를 오류로 반환한다.
- MIR은 Aggregate/Project/Update로 생성·읽기·root 갱신을 표현한다. Codegen 전 HIR/Resolve/Checked 재계산 gate를 유지하고,
  MIR validator가 원 schema·constructor/callee signature·projection FieldId·mutation root/path·가변성·완전 초기화·layout을
  별도 보존된 provenance와 대조한다. 손상 metadata나 오류 타입으로 CodegenUnit을 만들 수 없다.
  malformed nested ConstValue/Constant의 변환·검증·해제는 반복형으로 처리해 깊은 payload의 host stack overflow를 막는다.
- LLVM Adapter는 원 ID별 aggregate type과 insert/extractvalue를 사용한다. struct 인수는 caller snapshot을 가리키는
  readonly private 간접 인수, 반환은 caller-owned out pointer다. call용 alloca를 entry에 배치해 loop마다 stack을 늘리지 않는다.
  scalar ABI와 source Abort Span은 그대로이며 field byte offset이나 padding을 언어 API로 노출하지 않는다.
  CLI source root·entry·exit code·source 오류의 도구/출력 gate는 기존 계약을 따른다.

## 검증 증거

총 **292개** 고유 tests를 검증했다. 기본 workspace 252개와 별도 실행한 LLVM 8개·Windows Native 32개다.
기본 실행의 ignored 40개를 성공 개수에 합산하지 않고 실제 별도 실행 결과로 확인했다.

- 기본 252개: 기존 240개와 P12 12개(Parser 2, TypeChecker 5, MIR 2, CLI 1, Types 2).
  모든 UTF-8 truncation·구문 복구, nominal mismatch·생성·const·Copy·private factory·dual namespace 원자 import,
  [부정 fixture 10개](struct-proposal-fixtures/README.md)의 정확한 code/byte Span, 도구/출력 gate와 회귀를 포함한다.
- const budget 10,000-node 경계, 직접 field/struct 개수, layout 깊이 128/129와 ZST transitive occurrence 한도를 검사했다.
  혼합 layout의 독립 기대값은 offsets [0,0,0,2,4,8,16,24], size 32, alignment 8, depth 2, occurrences 8이다.
  손상 child layout과 30,000단계 malformed aggregate의 변환/검증/해제를 검사했다.
  ConstValue 해제는 64 KiB stack thread에서도 완료했다.
- LLVM 21.1.8 opt-in 8개: Windows x64 COFF/Linux x64 ELF의 O0/O2 실제 object 생성·검증.
  새 aggregate/ZST/함수 ABI corpus와 기존 scalar/숫자 cast/invalid IR/output 보존을 포함한다.
- Windows x64 Rust/MSVC Native opt-in 32개: 전체 순차 회귀와 P12 2개.
  원 main.nova/geometry.nova 두 파일 fixture를 격리된 경로에 그대로 복사해 debug/release에서 실행했다.
  stdout은 `original=21, snapshot=20, shifted=21, tag=🙂` + LF, stderr 없음, exit 0이다.
  인수/return 복사·loop field 변경·Empty/Unit/Bool/Char/Double·left-to-right 단일 effect 평가를 확인했다.
  8종 정수 경계와 float/double struct const/private ABI 출력은 다음과 같다.

```text
-128 255 -32768 65535 -2147483648 4294967295 -9223372036854775808 18446744073709551615 0.5 -0
```

- struct int8 field checked overflow는 두 profile에서 stdout 없음과 정확한 UTF-8 `file#0:start..end` Abort를 확인했다.
- 전체 Native 회귀 후 MIR provenance와 반복형 payload 해제를 보강했다. 이후 Core 전체·LLVM 전체와 P12/Hello Native를 재검증했다.
  마지막으로 모든 정수 width의 Native fixture를 확장하고 해당 P12 Native test를 두 profile에서 다시 통과시켰다.
- fmt, Runtime rustfmt, clippy(-D warnings), offline workspace tests/all-features check와 문서 validator PASS.
  환경은 Rust 1.99.0, LLVM 21.1.8, Windows x64 MSVC 14.44.35207이다.
  Rust 1.80 MSRV와 Linux Native host 실행은 별도 검증하지 않았다. ELF object 검증은 Linux 실행 검증이 아니다.

```powershell
cargo fmt --check
rustfmt --check --edition 2021 crates/nova-cli/runtime/stage_a.rs
cargo clippy --workspace --all-targets --offline -- -D warnings
cargo test --workspace --offline
cargo check --workspace --all-features --offline
$env:NOVA_CLANG = (Resolve-Path target/toolchains/llvm-21.1.8/bin/clang.exe).Path
cargo test -p nova-codegen-llvm --test emission -- --ignored --test-threads=1
cargo test -p nova-cli --test native -- --ignored --test-threads=1
node tools/docs/build-pack.mjs
node tools/docs/validate-pack.mjs
```

문서 validator는 ledger/hash/link/grammar/fixture 데이터를 검사한다. Compiler/Native 수용은 별도 Cargo 실행으로 확인했다.
[두 파일 fixture 실행 명령](struct-proposal-fixtures/README.md)과 [standalone 예제](../../examples/structs.nova)를 제공한다.

## 후속 범위

String/Move field·명시적 init/Drop·일반 borrow/ownership·Enum/Tuple/Array/Class·method/generic·Package·public FFI는 포함하지 않는다.
전체 D06/D10/D12/D16/D30은 계속 Draft다. 후속 언어 확장은 별도의 최소 계약과 사용자 승인을 따른다.
