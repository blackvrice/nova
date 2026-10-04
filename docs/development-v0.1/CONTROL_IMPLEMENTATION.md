# P04 가변 지역 변수·반복문 구현·검증 기록

작성일: 2026-10-04. [P04 최소 계약](CONTROL_STAGE_B_PROPOSAL.md)과
[30-production EBNF](GRAMMAR_STAGE_B_CONTROL.ebnf)을 구현했다.
전체 Stage B 완료가 아니다. 기존 P01/P02/P03 프로그램과 Native 계약은 유지한다.

## 현재 동작과 API

- `var name [: Type] = expression`: 초기값 필수. Int32/Bool/String/Unit의 타입을 추론하거나 annotation 검사.
- `name = expression`: 가변 지역 이름에만 대입. RHS 평가 후 같은 Place에 저장하는 Unit 문장.
  let, 기본 Read parameter, 함수/print에 대한 대입은 N3004다. 함수 인수 모드는 확장하지 않았다.
- `while Bool { ... }`: 매 반복 조건 재평가. false면 body 미실행. body는 독립 scope이고
  지역 initializer는 선언에 도달할 때마다 실행한다.
- bare break/continue는 가장 가까운 while의 exit/condition으로 이동한다. 밖에서는 N3002.
  return은 enclosing function으로 돌아간다. while은 필수 return 분석에서 fallthrough 가능으로 취급한다.
- `Binding.mutable`를 AST/HIR에서 보존하고 `Definition.mutable`에 기록한다.
  Assignment children은 target Name/RHS, While children은 condition/body다.
  ID는 기존 flat arena, SourceOrigin/byte Span은 각 단계에 남긴다.
- Parser는 IDENT 뒤 `=` lookahead로 assignment statement를 구분한다. expression에는 assignment를 추가하지 않았다.
  label/value jump, chained/field/index/group/call 대상 대입, const/for/loop는 거부한다.
  기존 nesting 128을 유지하며 P04 loop 안의 한도 초과는 N8901, 기존 P01 입력은 N1102다.
- Resolver는 initializer를 선언 전에 검사한다. 가까운 let이 바깥 var를 가리면 N3004다.
  var의 deterministic resolution dump에는 `mutable def ID`가 추가된다.
- Checker는 loop nesting context와 Fallthrough/Return/Jump flow를 별도로 추적한다.
  unreachable source도 이름·타입 검사하되 break/continue 뒤의 return을 필수 return의 증거로 쓰지 않는다.
  public resolution table의 변조된 mutable flag는 API 오류로 거부한다.
- MIR은 새 terminator 없이 기존 Assign/Goto/Branch/Call/Return을 재사용한다.
  lexical loop stack이 continue-condition/break-exit을 기록한다. 조건의 call/short-circuit도
  condition CFG 안에서 실행하므로 backedge와 continue에서 다시 실행된다.
  loop body return/jump 뒤의 문장은 CFG에 연결하지 않고, 양쪽 jump인 if의 orphan join은 Unreachable이다.
- 기존 cyclic CFG must-initialized validator와 immutable CodegenUnit gate를 재사용한다.
  loop body에서만 초기화해 zero-iteration path의 읽기를 허용하는 변조 MIR는 거부한다.
- LLVM Adapter/Runtime/CLI의 public 계약과 host 범위는 P03 그대로다. alloca는 entry에 있고
  값 갱신은 load/store이므로 반복마다 compiler stack allocation을 추가하지 않는다.
  dynamic String arena는 정상 entry 종료까지 유지한다. 반복 중 문자열 생성에 따라 memory가 증가할 수 있다.
  iteration별 Drop/해제, ownership/Move/NLL, memory bound는 이번 구현에 없다.

## 실행 예제

Workspace root의 PowerShell, LLVM 21.1.8과 Rust/MSVC를 준비한 Windows x64:

```powershell
$env:NOVA_CLANG = (Resolve-Path 'target/toolchains/llvm-21.1.8/bin/clang.exe').Path
cargo run --offline -p nova-cli -- check examples/loops.nova
cargo run --offline -p nova-cli -- run examples/loops.nova
cargo run --offline -p nova-cli -- run examples/loops.nova --profile release
```

[loops.nova](../../examples/loops.nova)는 O0/O2 모두 stdout `sum=8, index=5\n`, exit 0으로 검증했다.
check에는 LLVM/entry가 필요 없고 산출물도 생성하지 않는다. 자세한 도구/CLI 경계는
[Native 기록](NATIVE_IMPLEMENTATION.md)을 따른다.

## 검증 증거

- 이번 추가 **24 tests**: Parser 4, HIR 2, frontend semantics 7, MIR 5, CLI 2, 실제 Native 4.
- 기본 workspace **125 pass**, 별도 opt-in 실제 LLVM **2 pass** 및 Windows Native **12 pass**,
  합계 **139 pass**. 기본 Cargo test는 실제 LLVM/Native tests를 ignored로 남긴다.
  이는 P04 완료 당시 숫자이며 이후 local const는 [P05 구현 기록](CONST_IMPLEMENTATION.md),
  global const와 현재 tests는 [P06 구현 기록](GLOBAL_CONST_IMPLEMENTATION.md)을 따른다.
- Parser: source-order, END/newline jump, 미지원 대상/label/value, 다음 함수 복구,
  모든 ASCII prefix와 deep loop nesting의 결정성·한도.
- HIR: 모든 loop prefix의 recovery lowering/Span/SourceOrigin, malformed assignment/loop AST API 거부.
- Frontend: 모든 현재 값 타입 var, initializer visibility/duplicate/shadow, 정확한 target/RHS/declaration
  진단 Span, Bool condition, loop 밖 jump, 보수적인 return flow와 unreachable 오류, ErrorType 억제,
  변조된 declaration 가변성 거부.
- MIR: bounded control-flow oracle로 누적 8/nested sum 33, condition 4회·initializer 3회,
  zero iteration/return/jump 뒤 부수 효과 누락을 확인했다. cycles의 초기화 경로 검증도 회귀 시험했다.
- CLI: check/build/run의 잘못된 대입/조건/jump가 missing-clang보다 먼저 source exit1로 거부된다.
  target directory를 만들지 않는다. 정상 loops check도 LLVM 없이 통과한다.
- Windows Native O0/O2: 누적/nested break·continue, 조건과 initializer의 정확한 print 횟수,
  zero iteration/return과 unreachable effects, Bool/String/Unit 갱신, Unicode/NUL/기존 String 값,
  반복 중 overflow와 재평가된 조건의 division-by-zero Abort·후속 출력 차단.
  test-owned CLI/Native 프로세스는 반복문 회귀가 걸리면 40초 뒤 종료하여 무한 대기를 막는다.
- 기존 Stage A actual LLVM/Native 회귀 시험도 함께 실행했다. LLVM/Clang 21.1.8,
  Rust 1.99.0/MSVC 14.44.35207 Windows x64 환경이며 MSRV 1.80은 별도 검증하지 않았다.
- Cargo fmt, clippy `-D warnings`, workspace test, all-features check,
  standalone Runtime rustfmt, docs validator 및 git diff whitespace 검사를 모두 통과했다.
  실제 host Clang 실행은 Codex 제한 때문에 승인된 host process 권한으로 수행했다.

## 다음 범위

const/지연 초기화/Primitive 확장·승격/aggregate/module/multi-file,
for/loop/range/match/Option/Result/try는 다음 Stage B 단계의 별도 상세 승인 대상이다.
Linux는 P03처럼 IR/ELF Object만 검증했으며 Native link/run은 미지원이다.
기존 Stage A와 P04가 구현된 상태이며, Stage C ownership/Drop/borrow를 선행 완료했다고 주장하지 않는다.
