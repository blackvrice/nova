# P15 Copy Option·Result·nullable 구현·검증 기록

승인·구현일: 2026-10-07. 사용자 “승인 할테니 다음 개발 작업 진행해줘” 답변으로
검토한 [P15 최소 계약](OPTION_RESULT_STAGE_B_PROPOSAL.md)과 [51-production EBNF](GRAMMAR_STAGE_B_OPTION_RESULT.ebnf)를 승인했다.
원본 148개 문서와 기존 승인 subset을 보존한다. 전체 D06/D08/D09/D10/D12/D15/D16/D23/D25/D30은 계속 Draft다.

## 구현

- Parser는 named type application·trailing comma·반복 nullable·none expression/pattern을 처리한다.
  >=의 type-close 위치만 원 1-byte >/= subspan으로 나누고 pending assignment token 하나를 유지한다.
  원 raw/normalized stream은 보존하며 긴 파일에서 토큰 배열을 반복 이동하지 않는다.
  END normalizer는 type argument close+assignment 상태만 바꾸며 expression >=는 유지한다.
  중첩 한도·UTF-8 truncation·missing close 복구를 검사한다. HIR은 외부 AST의 이름/shape와
  nullable/generic punctuation·none spelling을 별도로 확인하고 nested comment trivia도 허용한다.
- Resolver/type lookup의 module-local 선언·type import가 intrinsic fallback보다 우선한다.
  Local value shadow는 owner type을 바꾸지 않는다. nullable는 spelling lookup을 거치지 않는다.
  실패한 type import는 builtin fallback을 차단하며 원 import 진단과 cascade suppression을 유지한다.
- TypeChecker는 Option/Result family와 normalized payload Type key로 specialization을 intern한다.
  Type::Enum storage를 재사용하지만 user Enum과 별도 SumRegistry·최초 HirId origin을 유지한다.
  Synthetic IDs는 source DefId·Tuple·다른 sum에 충돌하지 않는다. User Enum/tuple/sum 예산은 독립이다.
  annotations 수집과 constructor inference 순서가 달라도 source byte 순서로 첫 초과 origin을 고른다.
- Some는 payload 기대 타입을 전달하고 문맥이 없으면 payload 실제/default 타입에서 추론한다.
  none/None 및 Result 생성자는 완성된 기대 타입이 필요하다. wrapping·sum payload widening·직접 보간/비교/산술/cast/projection은 거부한다.
  qualified constructor·arity·Copy·immutable binder·family·coverage/flow를 검사한다.
  Struct/Tuple/Enum payload와 혼합 가변 경로·private factory의 nominal identity/visibility는 기존 규칙을 재사용한다.
- const는 Enum value 생성 경로로 Some/Success/Error 및 none/None을 평가한다.
  constructor 1+payload nodes·cached reference·10,000-node 경계·checked N3201·static cycle N3202를 유지한다.
- mixed aggregate graph/layout의 기존 반복형 검사에 intrinsic variants를 연결한다.
  Option Some=tag 0/None=1, Result Success=0/Error=1은 private metadata이며 source/FFI 약속이 아니다.
  independent layout oracle 및 unused mixed cycle의 closing named type byte Span을 검사한다.
- MIR의 constructor/tag dispatch/active payload read·Copy snapshot·indirect parameter/out pointer ABI를 재사용한다.
  private immutable sum family/key/origin certificate와 exact enum/struct schema를 비교하고 별도 family arity·key uniqueness·원 source의 Span/origin·kind를 검사한다.
  user Enum lookalike로 kind를 바꾸거나 specialization을 합치거나 payload type/variant schema를 변경한 MIR은 거부한다.
  P14의 CFG active tag proof·overwrite/retarget·초기화·source provenance 검사를 유지한다.
  손상된 30,000-level nested sum constant는 64 KiB host stack에서 반복형 변환/검증/해제로 검사한다.
- LLVM은 검증한 tagged Enum storage/active typed view와 entry scratch를 재사용한다.
  Runtime·LLVM emit source는 변경하지 않았으며 COFF/ELF O0/O2와 Windows debug/release는 별도 실행한다.

## 테스트와 검증

P15 추가 **14개**: Lexer 1, Parser 1, HIR 1, TypeChecker 5, MIR 2, CLI 1, LLVM opt-in 1, Native opt-in 2.
기본 workspace **287개** PASS, ignored **49개**다. 실제 LLVM **11개** PASS.
Windows Native **38개** PASS. 총 **336개** 고유 tests를 실제 실행했다.
기본 실행에서 ignored인 LLVM/Native 49개도 별도로 전부 확인했다.
MIR 원 source certificate를 강화한 최종 source에서 기본 287·LLVM 11·Native 38개를 모두 재실행했다.
최종 Native 실행은 342.55초이며 실패·누락·waived test가 없다.
fmt/Runtime rustfmt/clippy(-D warnings)/all-features check·문서 validator·git diff --check PASS를 확인했다.
Standalone 예제 check/debug/release도 최종 CLI로 실제 실행했다.

- [21개 부정 fixture](option-result-proposal-fixtures/README.md)의 진단 code·정확한 UTF-8 primary byte Span.
  초안 raw_wrapping 기대값이 주석 P15의 숫자 1을 가리켜 실제 initializer 1의 byte 위치로 정정했다.
  언어 의미 변경 없이 기대값 metadata와 independent hardcoded test를 함께 바로잡았다.
- alias/nullable identity, user type/import/value shadow, failed-import fallback, all-arm return/loop flow,
  nominal/private factory, wrapping/widening·N2103·arity·Copy·unknown variant·coverage·binder 오류.
- 4,096/4,097 unique specialization, independent Tuple/user Enum 예산, 최초 source origin,
  mixed depth 128/129·expanded occurrence 65,536 전후·unused by-value cycle, const 10,000-node 전후.
  size 1 MiB tag padding은 기존 shared Enum layout의 독립 cached-dependency oracle 회귀를 유지한다.
- layout oracle: Option<Unit> 4/4, Option<(int8,uint64)> 24/8,
  Option<Option<(int8,uint64)>> 32/8, Result<(),uint64> 16/8.
- CLI check/build/run은 source 오류를 missing native tools 호출 전에 거부하고 기존 output bytes를 보존한다.
- 기존 Parser의 none 거부는 승인된 P15 syntax test로 이동했다. 모든 named generic applications를 파싱하는
  P15에 따라 Array<int>의 Parser 거부도 의미 검사로 이동했다. 미선언 Array는 N2001이고
  기존 non-generic 사용자/primitive type application은 N1102이며 Array·사용자 Generic을 구현한 것은 아니다.
- actual LLVM regression 11개는 Windows COFF/Linux ELF 및 O0/O2를 검사한다.
  P15 corpus는 Unit/nullable/nested Option/Result·Tuple·all-scalar·private ABI를 포함한다.
- Native P15는 두 파일 alias/private factory fixture, all numeric/Bool/Char/Unit payload,
  left-to-right effect·single evaluation·snapshot·mixed mutation·nested match/while jump·all-return,
  checked overflow의 정확한 UTF-8 file/byte Span을 debug/release에서 검사한다.

## 직접 실행

[TESTING.md](../../TESTING.md)와 [standalone 예제](../../examples/option_result.nova)를 따른다.
두 파일 fixture와 standalone 예제의 stdout은 8줄 + 각 LF, stderr 없음, exit 0이다.

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

환경: Rust 1.99.0, LLVM 21.1.8, Windows x64 MSVC 14.44.35207.
Rust 1.80 MSRV 실행과 Linux Native host 실행은 별도 검증하지 않았다. ELF object는 Linux 실행 증거가 아니다.
문서 validator는 original hash/link/grammar/approval ledger/fixture metadata 검사이며 Compiler/Native 실행은 별도 증거다.

## 후속 경계

try·exists·unwrap/메서드, Array·String/Move payload·사용자 Generic·general borrow/Drop,
중첩 pattern/guard·match expression·niche·public ABI/FFI는 후속 최소 계약과 사용자 승인을 따른다.
