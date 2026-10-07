# P14 Copy Enum·statement match 구현·검증 기록

승인·구현일: 2026-10-07. 사용자 “P14 승인하고 Copy Enum·match 구현 진행”으로 승인한
[최소 계약](ENUM_STAGE_B_PROPOSAL.md)과 [48-production EBNF](GRAMMAR_STAGE_B_ENUM.ebnf)를 적용했다.
기존 승인 의미와 원본 148개 NOVA 문서를 보존한다.

## 구현과 경계

- AST/Parser/HIR에 Enum·Variant·qualified variant path·statement match·arm·단순 pattern을 추가했다.
  nullary variant는 `E::A`, payload variant는 `E::B(args)`다. Match arm은 block을 가지며 END/comma로 구분한다.
  Lexer token/END 정책은 유지하고 UTF-8 원 byte Span과 복구·기존 중첩 한도를 보존한다.
  HIR은 외부 AST의 이름·pattern shape·Bool/wildcard spelling·metadata도 독립 검사한다.
- Resolver는 struct와 같은 type namespace에 nominal Enum을 등록한다. P11 type import alias는 원 EnumId를 유지한다.
  왼쪽 owner는 type namespace로 찾으므로 동명의 지역 값은 variant path를 shadow하지 않는다.
  binder는 불변 Copy 지역 정의이며 arm body의 root scope에서 선언한다. `_`는 정의를 만들지 않는다.
  private factory의 추론 payload type은 유지하며 실제 struct field visibility는 기존 규칙으로 검사한다.
- TypeChecker는 scalar/struct/Tuple/Enum Copy payload·기대 literal 문맥·정확한 arity를 검사한다.
  String/Move payload, Enum arithmetic/comparison/cast/interpolation/직접 projection은 거부한다.
  Enum·Bool match의 전체 유한 domain을 source order로 검사하며 누락은 Error N3101,
  이미 덮인 pattern은 Error N3102다. 첫 누락 case 목록과 covering pattern의 원 Span을 제공한다.
  모든 arm return이면 함수 flow가 return이고 break/continue는 기존 enclosing loop를 대상으로 한다.
- const는 nullary 및 payload Enum 생성과 이전 참조를 지원한다. 생성은 1 node + payload expression이며
  qualified path를 별도 node로 세지 않는다. checked N3201·정적 skipped RHS 포함 cycle N3202·10,000-node 예산을 유지한다.
- Enum variant layout과 struct/Tuple layout은 Core의 checked metadata를 공유하며 kind는 별도 registry로 구분한다.
  unused/all-variant by-value graph도 반복형 cycle 검사 대상이다. tag는 u32 선언 순서 0부터이며
  payload union은 variant별 natural layout의 최대 size/alignment다. Synthetic variant가 depth를 추가하지 않는다.
  Enum/variant/payload component 각 1,024, match arm 1,025, mixed depth 128, size 1 MiB,
  all-variant occurrence 65,536과 기존 Tuple shape 4,096 한도를 지킨다.
- MIR은 Enum constructor·exhaustive tag dispatch·active payload read를 명시한다. Scrutinee는 한 번 평가해
  별도 Copy snapshot에 저장한다. 스키마/종류/ID/arity/type/초기화/layout·source provenance·coverage를 독립 검증한다.
  tag별 successor edge의 proof를 교차 병합하고 receiver overwrite/Call destination에서 proof를 제거한다.
  proof 없는 payload read, case retarget·wrong variant·stale receiver, Enum을 product aggregate/projection으로 바꾼 MIR을 거부한다.
  nested ConstValue/Constant 변환과 해제는 반복형이며 손상된 30,000단계 값도 작은 host stack에서 해제한다.
- LLVM Adapter는 private tagged storage와 active variant typed view를 분리한다. Opaque 저장 공간은 초기화하고
  Copy에 사용하며 typed payload는 검증한 active branch에서 읽는다. snapshot pointer 인수/out pointer 반환 ABI를 확장했다.
  Enum scratch/call temporary는 entry에 배치하며 const 직접 return처럼 Enum local이 없는 경우에도 준비한다.
  LLVM scalar formatting과 Runtime checked Abort 정책은 유지한다. 공용 ABI/FFI/serialization은 추가하지 않는다.
- P14의 IDENT::IDENT 구문은 module-qualified value call과 표면적으로 같으므로 이전 P11 Parser의
  `lib::f()` 거부 테스트를 의미 검사로 옮겼다. Enum type/variant로 해석되지 않는 호출은 여전히 거부한다.

## 검증 증거

총 **322개** 고유 tests: 기본 workspace **276개**, 별도 실제 LLVM **10개**, Windows Native **36개**다.
기본 실행의 ignored 46개는 별도 실제 실행 결과로 확인했다.
첫 전체 Native 실행은 35개 PASS, 기존 P07 한 개는 Windows os error 5로 프로그램 실행이 거부됐다.
그 한 개는 단독 재실행에서 모든 width/debug/release 사례가 PASS해 고유 Native 36개를 확인했다.
동시 재빌드의 실행 중 test exe 잠금(LNK1104)은 검사가 끝난 뒤 순차 재빌드로 해소했다.
P14 추가 14개: Parser 1, HIR 1, TypeChecker 5, Types 1, MIR 2, CLI 1, LLVM opt-in 1, Native opt-in 2.

- [부정 fixture 16개](enum-proposal-fixtures/README.md)의 지정 진단 code와 정확한 UTF-8 primary byte Span을 검사했다.
  nominal/alias/private factory·cross-file visibility와 unused mixed cycle·cross-file const cycle도 검증했다.
- 독립 layout oracle: `Empty | (uint8,uint64,Empty,Unit) | (char,bool)`의 전체 size 24, align 8, depth 2, occurrence 6.
  payload offsets는 [0,8,16,16]과 [0,4]다. Cached dependency layout으로 tag padding 포함 1 MiB 직전/초과도 검사했다.
  1,024/1,025 선언·variant·component, depth 128/129, all-variant occurrence, arm 한도와 const 10,000-node 경계를 검사했다.
- Parser의 모든 UTF-8 truncation·unsupported syntax·중첩 복구, malformed 외부 AST/MIR tag/coverage/schema gate를 확인했다.
  CLI check/build/run은 source 오류를 missing tool 호출 전에 거부하며 기존 output bytes를 보존한다.
- LLVM 21.1.8은 기존 회귀와 Enum/mixed/ZST/all-scalar private ABI를 Windows COFF/Linux ELF O0/O2로 검증했다.
- Windows Native debug/release는 두 파일 fixture·standalone 예제, left-to-right effect·scrutinee 단일 평가·snapshot,
  payload binder Copy·원 변수 변경·nested match/while jump·모든 arm return·checked failure의 정확한 file/byte Span을 검사했다.
  모든 8종 정수·Float32/64·Bool·Char·Unit·nullary/mixed Enum을 실행했다.

두 파일 fixture와 standalone 예제의 stdout은 다음 네 줄 + 각 LF, stderr 없음, exit 0이다.

```text
make
sum=30, code=7, flag=true
original=empty
ok
```

전체 scalar payload/private ABI의 검증 출력:

```text
-128 255 -32768 65535 -2147483648 4294967295 -9223372036854775808 18446744073709551615 0.5 -0 true 🙂
```

기존 전체 회귀와 fmt/Runtime rustfmt/clippy(-D warnings)/all-features, 문서 hash/link/grammar/ledger validator를 검사했다.
환경은 Rust 1.99.0, LLVM 21.1.8, Windows x64 MSVC 14.44.35207이다.
Rust 1.80 MSRV와 Linux Native host 실행은 별도 검증하지 않았다. ELF object는 Linux host 실행 증거가 아니다.

```powershell
cargo fmt --check
rustfmt --check --edition 2021 crates/nova-cli/runtime/stage_a.rs
cargo clippy --workspace --all-targets --offline -- -D warnings
cargo test --workspace --offline
cargo check --workspace --all-features --offline
$env:NOVA_CLANG = (Resolve-Path target/toolchains/llvm-21.1.8/bin/clang.exe).Path
cargo test -p nova-codegen-llvm --test emission --offline -- --ignored --test-threads=1
cargo test -p nova-cli --test native --offline -- --ignored --test-threads=1
node tools/docs/build-pack.mjs
node tools/docs/validate-pack.mjs
```

문서 validator는 승인 ledger/hash/link/grammar/fixture 데이터만 검사하며 Compiler/Native는 별도 Cargo 실행 증거다.
[사용자 실행 명령](../../TESTING.md), [예제](../../examples/enums.nova), [두 파일 fixture](enum-proposal-fixtures/README.md)를 제공한다.

## 후속 범위

Array·Option/Result·generic/try·guard/nested pattern·match expression·String/Move payload·일반 borrow/Drop·public ABI는 제외한다.
전체 D06/D08/D09/D10/D12/D16/D25/D30은 계속 Draft이며 후속 확장은 별도 최소 계약과 사용자 승인을 따른다.
