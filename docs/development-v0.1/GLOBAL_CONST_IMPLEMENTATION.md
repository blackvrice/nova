# P06 단일 파일 전역 const·의존성 평가 구현·검증 기록

작성일: 2026-10-04. [사용자 승인 P06](GLOBAL_CONST_STAGE_B_PROPOSAL.md)과
[31-production 전용 EBNF](GRAMMAR_STAGE_B_GLOBAL_CONST.ebnf)를 구현했다.
승인 답변은 “P06 승인하고 전역 const 구현 진행”이다.
P02/P06 print 문장 충돌은 사용자 “기존 함수 print 허용, 전역 const print만 거부 (권장)” 답변으로 해결했다.
전체 D06/D09/Stage B, module·다중 파일이나 const function의 구현 완료가 아니다.

## 구현 경계와 API

- root는 function/초기값 필수 const/recovery Error item을 source-order로 가진다.
  `Binding.constant`를 재사용하며 top-level let/var/runtime statement는 N1102다.
  잘못된 const 뒤 다음 const/function 선언에서 복구하고 HIR은 root의 runtime binding 주입을 거부한다.
- Resolver는 root function과 `DefinitionKind::GlobalConst(HirId)`를 먼저 수집한다.
  전역 initializer와 함수는 선언 위치와 무관하게 전역 const를 볼 수 있다.
  local declaration-before/after·shadow/parameter scope 규칙은 기존 P02/P05다.
- builtin print는 기존 바깥 prelude scope를 유지한다. 사용자 func print/local print shadow는 허용하고
  새 전역 const print만 N2002다. 전역 const/function끼리 같은 root 이름 중복도 N2002다.
  root lookup은 최초 선언을 유지하고 duplicate global은 invalid로 표시한다.
- `nova-typecheck/src/global_consts.rs`는 private graph 분석이다. 전역 initializer의 모든 global-name
  reference를 수집하므로 skipped Bool RHS도 dependency다. 중복 edge는 첫 source reference를 사용한다.
  이름/구문/중복 오류가 있는 global의 outgoing edge는 평가 가능한 graph에서 제외하고 Invalid로 전파한다.
- explicit-stack DFS postorder와 reverse graph로 SCC를 구한다. cyclic SCC마다 N3202 하나를 선언 순서로 보고한다.
  earliest declaration에서 SCC 내부 source-order DFS의 첫 back edge로 실제 cycle path를 선택한다.
  closing reference가 primary, path의 선언/다른 reference가 secondary, note는 `A -> B -> A` chain이다.
  모든 단순 cycle 또는 path에 없는 SCC member를 diagnostic에 열거하지 않는다.
- 순환의 모든 member는 `Failed { code: 3202, nodes: 0 }`다. 이는 initializer를 평가하지 않은 상태이며
  성공한 0-node 식이 아니다. dependents는 Invalid/ErrorType로 전파해 파생 type/const 오류를 억제한다.
  type annotation이 있어도 순환은 거부한다. 다른 독립 전역 선언의 오류는 계속 검사한다.
- dependency-first로 전역 타입/ConstValue를 확정한 뒤 모든 function body를 검사한다.
  동일 입력의 DefId/dump/진단이 결정적이며 acyclic 값은 root item 재배치에 독립적이다.
  public Resolved 전체를 HIR에서 재검사하므로 종류·scope·reference·flag 변조를 CheckError로 거부한다.
- evaluator는 [P05](CONST_IMPLEMENTATION.md)를 재사용한다. Int32/Bool/String/Unit,
  permission/type/checked failure·N3201·actual Bool short-circuit와 initializer당 10,000-node budget은 같다.
  cached global name도 1 node이며 예산은 local/global initializer마다 reset한다.
  graph traversal도 host recursion을 사용하지 않고 파일 전체 memory/time cap은 별도로 약속하지 않는다.
- unused 전역 상수도 모두 평가한다. failed/Pending/Invalid/ErrorType는 성공 MIR/CodegenUnit에 들어가지 않는다.
  cyclic SCC의 initializer를 값처럼 다시 타입 추론하거나 평가하지 않는다.
- MIR은 global const를 runtime body·local Place·startup/global mutable memory로 만들지 않는다.
  함수 내 global read는 기존 Constant operand다. local const는 P05 lexical Place를 유지한다.
  일반 let/var가 global constant와 산술을 하면 그 연산은 기존 runtime checked 연산이다.
- MIR source table은 전역 선언/initializer/reference의 SourceOrigin·byte Span을 보존한다.
  분석 table은 reference에서 DefId/decl Span/value로 연결한다. 값/count/type/kind/scope/누락 table 변조는
  기존 MIR correspondence gate에서 거부한다. 전역 값도 LLVM-specific 타입을 compiler core에 노출하지 않는다.
- String은 기존 static UTF-8/NUL bytes 표현이며 Runtime ABI·arena·entry/print/Abort 정책은 P03을 유지한다.
  새 crate/dependency/toolchain 설치나 일반 global ABI는 추가하지 않았다.

## 실행

Windows x64, LLVM 21.1.8/Rust/MSVC가 준비된 workspace PowerShell:

```powershell
$env:NOVA_CLANG = (Resolve-Path 'target/toolchains/llvm-21.1.8/bin/clang.exe').Path
cargo run --offline -p nova-cli -- check examples/global_constants.nova
cargo run --offline -p nova-cli -- run examples/global_constants.nova
cargo run --offline -p nova-cli -- run examples/global_constants.nova --profile release
```

[global_constants.nova](../../examples/global_constants.nova)는 뒤의 BASE를 먼저 평가해 LIMIT를 만들고,
global TITLE을 local const에 넣어 loop 결과와 SAFE를 출력한다.
실제 Windows x64 O0/O2 stdout UTF-8 bytes는 `합계: 15, skipped=false\n`, exit 0이다.
check는 main 없는 전역 const fragment도 받고 LLVM 없이 검사하며 산출물을 만들지 않는다.

## 검증

- 추가 **22 tests**: Parser 2, HIR 2, frontend 10, MIR 3, LLVM IR 1, CLI 2, 실제 Native 2.
- Frontend: 네 타입·forward/name/type/annotation·함수보다 뒤의 global·root 재배치·local shadow와
  initializer lookup, global/function/builtin 충돌, immutable 대입의 정확한 code/byte Span.
- Dependency: self/annotated/skipped-edge cycle, SCC 한 진단·first back edge/chain·독립 오류·failed cascade,
  10,000-link acyclic chain·2,048-node cycle·1,024개 self-cycle component와 deterministic 결과.
  별도 transitive reachability oracle로 모든 512가지 3-node graph의 cycle/value/invalid 상태를 대조했다.
- Const: P05 checked 경계·signed division/remainder·short-circuit permission/type/evaluation,
  global node 10,000/10,001·cached/reset/skipped count·unused 실패·String Unicode/NUL·Unit.
  기존 Draft const-divzero source fixture의 실제 N3201/17..22 byte Span도 frontend test에서 확인했다.
  fixture sidecar 전체 T018/D09 상태는 Draft로 보존했다.
- Parser/HIR: mixed root/END·복구·모든 ASCII prefix lowering·origin과 malformed root runtime binding 거부.
- MIR/IR: global read Constant와 source table·no initializer body/global startup,
  let runtime arithmetic 보존·failure gate·값/type/count/kind/scope/누락 table 변조 차단,
  Windows/Linux x64 deterministic IR와 실제 COFF/ELF object 생성·기존 output 보존/invalid IR 거부.
- CLI: check/build/run cycle/budget/unused arithmetic/read-only failure는 missing clang보다 먼저
  source exit 1로 거부하고 target directory를 만들지 않는다. 성공 fragment check와 기존 main requirement도 검사했다.
- 실제 Windows Native **16 pass**, 실제 LLVM **2 pass**. Native는 기존 Stage A/P04/P05 회귀도 포함하며
  새 예제·MIN/MAX·global 값을 이용한 runtime arithmetic·함수 return·local shadow·Unicode/NUL·Unit을 O0/O2로 확인했다.
- 기본 workspace **163 pass**, 별도 실제 LLVM/Native **18 pass**, 합계 **181 pass**다.
  기본 Cargo 실행에서 ignored인 실제 도구 시험은 위 별도 실행으로 확인했다.
- Cargo fmt, clippy `-D warnings`, workspace test, all-features check, standalone Runtime rustfmt,
  문서 validator와 diff whitespace 검사를 모두 통과했다.
  실제 LLVM/Clang 21.1.8, Rust 1.99.0/MSVC 14.44.35207 Windows x64 환경이며 declared MSRV 1.80은 별도 시험하지 않았다.

## 후속

Primitive 확장·숫자 승격·aggregate, module/import/visibility·multi-file, const function,
전역 let/var/runtime initializer·optimizer/query/cache는 다음 최소 상세 승인 범위다.
Linux Native link/run과 ownership/Drop/NLL, 전체 Stage B 완료는 이번 작업 범위가 아니다.
