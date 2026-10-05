# P11 Module·다중 파일 구현·검증 기록

구현·검증일: 2026-10-05. 사용자 “P11 승인하고 Module·다중 파일 구현 진행”으로 승인한
[Module 계약](MODULE_STAGE_B_PROPOSAL.md)과 [34-production EBNF](GRAMMAR_STAGE_B_MODULE.ebnf)를 적용했다.
D01~D05/P01~P10의 의미와 원본 148개 NOVA 문서를 보존한다.

## 구현과 경계

- `nova-driver::load`가 파일 I/O를 담당한다. entry 부모 또는 `--source-root` 기준으로 직접 item
  import가 도달한 `.nova`만 읽는다. root-relative 경로의 실제 철자를 검사하며 fallback/NFC 변환은 없다.
  canonical root를 벗어난 junction/symlink는 N8001이다. ASCII case-only 충돌과 동일 physical file의
  다른 논리 경로는 N2002다. Windows file handle의 volume/file index, Unix dev/inode로 hard link도 구분한다.
  discovery는 반복형 queue를 사용하고 1,024개까지만 등록한다. 초과 use는 N8901이다.
- entry FileId는 0, 나머지는 normalized relative filename의 UTF-8 byte 순서다. import 순서에 따른
  identity 변화가 없으며 unused broken file을 읽지 않는다. 파일별 AST arena/root와 원본 bytes를 보존한다.
  `Module::bundle`은 파일별 HIR root/ownership/ImportEdge와 공통 symbol table을 제공한다.
  multi-file SourceOrigin은 `(FileId, AstNodeId)`이며 합친 가상 소스나 cross-file AST parent를 만들지 않는다.
  no-import 단일 파일 dump/snapshot은 그대로다.
- top-level `use module::item [as alias]`와 함수/전역 const의 선택적 visibility를 구현했다.
  기본 internal이며 public/internal은 bundle, private는 원 module에서만 접근 가능하다.
  모든 module의 직접 선언을 먼저 수집하고 import binding을 원 DefId에 연결한다.
  aliases를 재import하는 reexport는 N1102이며 private는 N2004에 원 선언 secondary를 붙인다.
  동일 target의 다른 alias는 허용하고 동일 이름의 중복은 원 두 binding 위치의 N2002다.
  함수 print shadow와 nearest local/initializer-before-binding은 기존 규칙을 따른다.
  const print alias는 N2002다. failed import의 후속 이름 오류를 억제한다.
- reachable 모든 함수와 전역 const를 검사한다. imported signature의 expected/return label은 원 파일이다.
  전역 dependency/SCC는 원 DefId로 연결되며 skipped logical RHS의 cross-file cycle도 N3202다.
  cached const reference는 initializer 예산 1 node, 각 initializer의 10,000-node 예산은 독립이다.
  P10 cast를 포함한 const 실패의 원 파일/Span을 보존한다.
- MIR은 bundle 전체의 원 DefId body/callee를 하나의 CodegenUnit/object로 만든다.
  HIR와 resolve/type/const/call side table을 재계산하는 gate를 유지한다.
  MIR의 원 entry DefId/signature/source provenance는 외부에서 변경할 수 없으며 callee/body/source table과 독립 검증한다.
  mutable MIR의 이름 변경으로 가짜 entry를 만들거나 원 main을 바꾸는 경우도 거부한다.
  Native entry는 entry 파일에 직접 선언한 main()→Unit만 허용한다.
  imported alias main이나 다른 파일 main은 entry를 대체하지 않는다. 동명 함수는 private ID symbol로 공존한다.
- check/build/run은 `--source-root <directory>`를 지원한다. usage 오류는 2, source/graph/name/type/const 오류는 1,
  toolchain/unsupported float host는 3, ICE는 101이다. source 오류는 도구 호출/output 생성보다 먼저 거부한다.
  root 읽기/UTF-8 실패는 실제 path/offset의 N8001 event이며 존재하지 않는 source의 가짜 Span은 없다.

## 검증 증거

- 기본 workspace **240개** tests: 기존 217개와 P11 23개.
  Parser 3개(use/alias/visibility/newline/END·제외 구문·복구·모든 UTF-8 truncation), HIR 2개(malformed AST/bundle),
  Driver 15개(graph/alias/visibility/print/const/budget/source/entry/gate·hard link·Windows junction),
  CLI 2개(모든 command의 도구/출력 gate·source-root usage), LLVM emission 1개(private symbol/entry isolation).
- 실제 LLVM 21.1.8 **7개** opt-in tests: Windows x64 COFF/Linux x64 ELF의 O0/O2 object 검증,
  새 bundle과 기존 scalar/cast/invalid IR/output preservation 회귀. ELF object 검증은 Linux 실행 검증이 아니다.
- Windows x64 Rust/MSVC Native **30개** opt-in tests: 기존 28개와 P11 2개를 순차 실행했다.
  원 두 파일 fixture는 debug/release stdout `value=42` + LF, stderr 없음, exit 0.
  nested entry/명시적 root·파일 간 재귀·effect source order·String lifetime·다른 파일 main도 검증했다.
  다른 파일의 arithmetic/cast Abort는 Unicode/CRLF bytes를 포함한 정확한 `file#1:start..end`와
  후속 print 중단을 두 profile에서 확인했다.
- 전체 회귀 후 entry DefId/signature/source gate를 보강했고 Core 전체·LLVM 전체·P11/Hello Native를 재검증했다.
  LLVM P11 fixture 출력 경로는 실행별로 격리해 기존 object의 덮어쓰기 거부 정책을 유지한다.
- fmt/clippy(-D warnings)/offline workspace tests/all-features, Runtime rustfmt와 문서 validator PASS.
  검사 환경은 Rust 1.99.0, LLVM 21.1.8, Windows x64 MSVC 14.44.35207이다.
  선언된 Rust 1.80 MSRV와 Unix filesystem branch는 이 Windows 환경에서 별도 실행하지 않았다.

```powershell
cargo fmt --check
rustfmt --check --edition 2021 crates/nova-cli/runtime/stage_a.rs
cargo clippy --workspace --all-targets --offline -- -D warnings
cargo test --workspace --offline
cargo check --workspace --all-features --offline
$env:NOVA_CLANG = (Resolve-Path target/toolchains/llvm-21.1.8/bin/clang.exe).Path
cargo test -p nova-codegen-llvm --test emission -- --ignored --test-threads=1
cargo test -p nova-cli --test native -- --ignored --test-threads=1
```

문서 validator는 ledger/hash/link/grammar/fixture 데이터를 검사한다. Compiler·Native 수용은 위
Cargo 실행 증거로 별도 확인했다. [두 파일 예제와 직접 실행 명령](module-proposal-fixtures/README.md).

## 후속 범위

module alias/qualified value·type import·wildcard/group imports·reexport·Package manifest/dependency,
aggregate/ownership·public FFI·incremental cache·Linux Native host 실행과 전체 D06/D30 정책은 포함하지 않는다.
다음 Stage B aggregate 작업은 구현 전에 최소 계약을 정리해 승인받는다.
