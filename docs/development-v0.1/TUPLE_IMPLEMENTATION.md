# P13 Copy Tuple 구현·검증 기록

승인·구현일: 2026-10-07. 사용자 “P13 승인하고 Copy Tuple 구현 진행”으로 승인한
[최소 계약](TUPLE_STAGE_B_PROPOSAL.md)과 [40-production EBNF](GRAMMAR_STAGE_B_TUPLE.ebnf)를 적용했다.
기존 승인 의미와 원본 148개 NOVA 문서를 보존한다.

## 구현과 경계

- Parser/AST/HIR에 Tuple/TupleType/numeric projection을 추가했다. ()는 Unit, (x)는 grouping,
  (x,)는 one-tuple이며 trailing comma를 허용한다. Tuple 타입은 element별 별도 type node를 유지한다.
  expression/type nesting은 기존 Parser 상한을 따르고 오류 뒤 다음 element/선언으로 복구한다.
- longest-match Lexer와 END·token dump를 유지했다. Dot 뒤의 0.1 같은 canonical decimal token을
  Parser에서 원 byte subspan의 두 selector로 해석한다. 연속 selector는 왼쪽 결합하고 각 index Span을 보존한다.
  exponent/underscore/radix/leading zero/음수 selector는 N1102이며 거대한 decimal index는 out-of-range N2001이다.
  HIR lowering은 외부 AST가 넘긴 selector 철자·subspan·node shape도 독립 검사한다.
- Type::Tuple의 shape는 정규화 element Type 목록으로 intern한다. primitive alias는 같고 struct element의
  원 nominal ID는 유지한다. 익명 tuple shape와 nominal struct는 구분하지만 layout/field metadata는 공유한다.
  Compiler Core에는 LLVM 타입을 넣지 않는다. synthetic shape ID는 source 정의 ID와 겹치지 않는다.
  HashMap은 lookup에만 사용하며 출력/진단 순서는 별도 source-order 정렬과 결정적 ID로 유지한다.
- 기대 Tuple의 arity가 같으면 element별 기대 타입을 전파하고 scalar coercion을 명시한다.
  이미 만들어진 Tuple에는 element-wise 암묵 변환을 하지 않는다. String/Move element와 aggregate 연산/보간/cast를 거부한다.
  tuple binding·인수·return·const는 독립 Copy 값이다. private factory의 추론 값과 struct import alias를 보존한다.
- bare local var root의 numeric/named 혼합 경로를 읽고 갱신한다. 모든 struct field는 var여야 하며
  Tuple element는 var root 아래에서 변경 가능하다. let/const root·parameter·let field는 N3004다.
  RHS를 한 번 평가한 다음 root를 재구성하므로 저장 전 값을 읽고 snapshot은 독립적이다.
- const evaluator는 tuple 생성·projection을 허용하고 모든 element를 source order로 평가한다.
  tuple expression 1 node + element, projection 1 node + receiver, cached reference 1 node와 기존 group 규칙을 유지한다.
  initializer별 10,000-node 예산, checked N3201과 skipped RHS 포함 static dependency/cycle N3202를 보존한다.
- signature/local type annotation까지 수집한 뒤 struct/Tuple mixed graph를 반복형으로 검사한다.
  unused by-value cycle도 N2101이고 named type token·cycle edge의 원 파일 위치를 보존한다.
  직접 element 1,024·고유 shape 4,096·aggregate 깊이 128·size 1 MiB·transitive occurrence 65,536 상한은 N8901이다.
  구조 metadata의 등록은 최대 4,096개로 제한하며 초과 shape의 source origin을 추적해 첫 초과 source 위치를 보고한다.
  동일 shape 재사용은 예산을 늘리지 않는다. Unit/Empty ZST도 논리 초기화와 경로에 포함한다.
- MIR은 기존 Aggregate/Project/Update를 Tuple에도 사용한다. 원 schema·shape kind·constructor/projection/path·
  function signature·완전 초기화·layout provenance를 독립 검증한다. Struct/Tuple kind를 바꾼 손상 local/constant도 거부한다.
  HIR/Resolve/Checked 재계산 gate를 유지하고 nested payload 변환·검증·해제는 반복형으로 처리한다.
- LLVM Adapter는 P12 caller snapshot 간접 인수/out pointer 반환 ABI를 Tuple로 확장했다.
  call alloca는 entry에 배치하고 scalar ABI·Runtime formatting·checked Abort 위치는 유지한다.
  mixed Tuple/struct/ZST와 모든 숫자 width를 O0/O2로 검증한다. 공개 layout/FFI/serialization API는 추가하지 않는다.

## 검증 증거

총 **308개** 고유 tests: 기본 workspace **265개**, 별도 실제 LLVM **9개**, Windows Native **34개**다.
기본 실행의 ignored 43개는 실제 별도 실행 결과로 확인했다.

- P13 추가 16개: Parser 2, HIR 1, TypeChecker 6, MIR 2, CLI 1, Types 1, LLVM opt-in 1, Native opt-in 2.
- Parser: Unit/group/one-tuple·type/trailing comma·numeric token 유지·selector subspan,
  모든 UTF-8 truncation·bad selector·제한 target·type nesting 복구.
- TypeChecker: structural/nominal identity·expected literal/명시적 scalar coercion·Copy·private factory·multi-file const/import,
  [부정 fixture 10개](tuple-proposal-fixtures/README.md)의 정확한 code/UTF-8 byte Span, mixed cycle과 const checked 실패.
  10,000-node 경계, 1,024/1,025 element, 4,096/4,097 고유 shape·재사용·pass 순서와 무관한 첫 초과 source Span,
  mixed 깊이와 ZST occurrence 상한을 검사했다.
  혼합 Tuple layout의 독립 기대값은 offsets [0,0,0,2,4,8,16,24,40], size 48, align 8, depth 2, occurrences 11이다.
- HIR/MIR: 외부 selector metadata와 schema/path/kind/arity 손상, 30,000단계 malformed Tuple constant 변환/검증/해제.
  혼합 Tuple/struct ConstValue 해제는 64 KiB thread stack에서도 완료했다.
- CLI: 모든 check/build/run에서 source 오류가 도구 호출보다 먼저 거부되며 기존 keep.exe bytes를 보존한다.
- LLVM 21.1.8: 기존 회귀와 새 Tuple/ZST/mixed ABI를 Windows COFF/Linux ELF O0/O2로 실제 검증했다.
- Windows Native: 원 두 파일 fixture, 독립 Copy·인수/return·loop mutation·left-to-right effect 단일 평가,
  Unit/Empty/Bool/Char/Float·모든 8종 정수 경계, source checked failure를 debug/release로 검사했다.
  원 fixture와 standalone 예제의 stdout은 다음 한 줄 + LF이며 stderr 없음, exit 0이다.

```text
original=21, snapshot=20, shifted=21, one=7, tag=🙂
```

전체 숫자 Tuple const/private ABI의 검증 출력:

```text
-128 255 -32768 65535 -2147483648 4294967295 -9223372036854775808 18446744073709551615 0.5 -0
```

Tuple int8 element overflow는 stdout 없음과 정확한 UTF-8 `file#0:start..end` Abort를 두 profile에서 확인했다.
기존 전체 Core/LLVM/Native 회귀와 fmt/Runtime rustfmt/clippy(-D warnings)/all-features·문서 validator를 검증했다.
환경은 Rust 1.99.0, LLVM 21.1.8, Windows x64 MSVC 14.44.35207이다.
Rust 1.80 MSRV와 Linux Native host 실행은 별도 검증하지 않았다. ELF object 검증은 Linux Native 실행 검증이 아니다.

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

문서 validator는 승인 ledger/hash/link/grammar/fixture 데이터만 검사한다. Compiler와 Native 증거는 별도 Cargo 실행으로 확인한다.
[직접 실행할 명령](../../TESTING.md), [standalone 예제](../../examples/tuples.nova), [두 파일 fixture](tuple-proposal-fixtures/README.md)를 제공한다.

## 후속 범위

Array·Enum/match·destructuring·named tuple·type alias·String/Move element·일반 borrow·init/Drop·public ABI는 포함하지 않는다.
전체 D09/D10/D12/D16은 계속 Draft이며 후속 확장은 별도 최소 계약과 사용자 승인을 따른다.
