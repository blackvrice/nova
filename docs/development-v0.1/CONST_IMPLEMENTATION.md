# P05 함수 내부 const·상수 평가 구현·검증 기록

작성일: 2026-10-04. [사용자 승인 P05](CONST_STAGE_B_PROPOSAL.md)와
[전용 30-production EBNF](GRAMMAR_STAGE_B_CONST.ebnf)의 최소 subset을 구현했다.
전체 D09/Stage B, 전역 상수·const function 승인이 아니다.

## 구현 경계와 API

- `const name [: Type] = expression`은 함수와 if/while body의 불변 binding이다.
  초기값 필수, 기존 Int32/Bool/String/Unit type inference/annotation을 사용한다.
  전역 const는 N1102. 내부 const 대입은 P04 N3004다.
- AST/HIR `Binding.constant`와 `Definition.constant`가 let/var/const를 구분한다.
  mutable+constant를 동시에 지정한 malformed AST는 HIR API 오류로 거부한다.
  initializer-before-declaration/scope/shadow/duplicate 규칙은 P02를 유지한다.
- `nova-types::ConstValue`는 Int32/Bool/String/Unit의 target-independent 값이다.
  String은 decoded immutable UTF-8 bytes, NUL을 포함한다. LLVM/Runtime 타입에 의존하지 않는다.
- Const Engine은 `nova-typecheck/src/const_eval.rs`에 격리했다. 별도 외부 dependency가 없다.
  typecheck 뒤 허용성/예산 검사를 source-order preorder work stack으로 수행하고,
  그다음 허용 표현식을 평가 work/value stack으로 계산한다. 긴 flat expression도 host recursion을 쓰지 않는다.
- 허용 연산은 P05의 현재 primitive subset과 앞서 성공한 const 참조뿐이다.
  let/var/parameter, call/보간은 N3201이며 skipped logical RHS도 허용성 검사를 받는다.
  함수 값의 기존 N1102, literal/type/name 오류는 그대로 유지한다.
- 실제 계산은 source-order·Bool short-circuit이다. false && div0 / true || div0의
  Bool 조건식은 계산 성공하지만 skipped RHS의 범위·타입 오류는 계속 거부한다.
- checked Int32 연산은 P03 의미와 같다. overflow/div0/remainder0/MIN/-1은 N3201,
  operation Span과 const 선언 name Span을 함께 기록한다. 사용자 코드를 실행하거나 LLVM를 호출하지 않는다.
- `CONST_NODE_LIMIT=10_000`. root 및 skipped subtree를 포함한 initializer HIR expression node를 센다.
  10,001번째 preorder node에서 N3202와 Span/declaration/limit note를 제공한다.
  annotation/binding은 제외하고 cached const name은 1 node다. initializer마다 예산을 재시작한다.
  parser nesting 128은 별도이며 파일 전체 memory/time cap 또는 CLI budget override는 없다.
- `Checked.const_values`는 DefId별 NotConstant/Pending/Invalid/Value{value,nodes}/Failed{code,nodes}다.
  Pending은 분석 중 상태이고 완료된 성공 입력에는 남지 않는다. Invalid는 upstream 오류로 평가를 생략한 상태다.
  평가 실패는 definition ErrorType로 전파하여 뒤의 const 참조에서 파생 오류를 억제한다.
  dump는 const def별 값·count/state를 안정적으로 기록한다.
- 모든 source const를 검사하므로 unreachable/zero-iteration body의 실패도 숨기지 않는다.
  CLI는 syntax 오류를 의미 분석 전에 거부한다. HIR API만 호출하는 사용자는 기존처럼
  Lexer/Parser diagnostics를 보존하여 MIR의 upstream error gate에도 전달해야 한다.
- MIR는 successful const initializer를 기존 `Constant`/Assign으로 materialize하고 그 subtree를
  runtime expression으로 lowering하지 않는다. name 사용은 lexical local Place를 통해 값을 읽는다.
  loop마다 precomputed 값 저장은 가능하지만 const 산술을 runtime에서 다시 수행하지 않는다.
  일반 let/var 산술은 기존 Runtime checked 연산으로 남는다.
- SourceOrigin과 initializer SourceInfo table은 보존한다. MIR correspondence gate는 값을 포함한
  public semantic table을 재검사하므로 constant 값/count/상태/누락 table 변조를 거부한다.
  검증 중 const 계산을 다시 확인할 수 있으며 compiler 내부 평가 횟수를 API로 고정하지 않는다.
- LLVM/Runtime ABI, String arena, host 범위와 CLI exit/output 보호는 P03/P04를 유지한다.
  const String은 static IR bytes이고 runtime 보간의 dynamic 저장소는 기존 arena 정책을 따른다.

## 실행

Windows x64, LLVM 21.1.8와 Rust/MSVC가 준비된 workspace root PowerShell:

```powershell
$env:NOVA_CLANG = (Resolve-Path 'target/toolchains/llvm-21.1.8/bin/clang.exe').Path
cargo run --offline -p nova-cli -- check examples/constants.nova
cargo run --offline -p nova-cli -- run examples/constants.nova
cargo run --offline -p nova-cli -- run examples/constants.nova --profile release
```

[constants.nova](../../examples/constants.nova)의 예상·검증 stdout은
`합계: 15, skipped=false\n`, exit 0이다. O0/O2에서 같은 UTF-8 bytes를 출력한다.
check에는 LLVM/entry가 필요 없고 산출물을 생성하지 않는다.

## 검증

- 이번 추가 **20 tests**: Parser 2, HIR 1, frontend const 8, MIR 4, LLVM IR 1, CLI 2, 실제 Native 2.
- 기본 workspace **143 pass**, 별도 실제 LLVM **2 pass**와 Windows Native **14 pass**,
  합계 **159 pass**. 기본 tests의 ignored 결과와 실제 도구 시험은 별도 기록한다.
- Parser/HIR: 선언 분류/END/초기값·전역 경계/다음 함수 복구, 모든 ASCII const prefix의
  recovery lowering, conflicting flags의 malformed AST API 오류.
- Const frontend: 네 타입·허용 연산·Int32 경계/음수 division·remainder,
  dependency/shadow/duplicate/forward/불변 대입, 8 checked 실패의 정확한 code/Span,
  short-circuit permission/type/evaluation 구분, forbidden 이름/call/보간 위치,
  unreachable const 실패/실패 상태·ErrorType 파생 오류 억제/결정적 dump·resolution flags.
- Budget: 10,000-node 성공, 10,001-node 실패와 limit note/정확한 leaf Span,
  initializer별 reset/cached name 1-node/skipped subtree 포함/반복 결과 결정성.
- MIR/IR: const 산술·skipped division을 runtime 연산/guard로 생성하지 않음,
  일반 let 산술은 보존, loop·String·Unit 값과 SourceInfo, failure gate,
  값/count/상태/누락 table 변조 차단, Windows/Linux x64 deterministic IR.
- CLI: check/build/run의 N3201/N3202/N3004/전역 N1102가 missing-clang보다 먼저 source exit1로 끝나고
  target directory를 만들지 않는다. 정상 check도 외부 도구 없이 통과한다.
- Native O0/O2: 예제, MIN/MAX·const 이름 unary 연산·signed division/remainder,
  String alias/Unicode/NUL·Unit parameter·nested shadow/loop·short-circuit을 확인했다.
  기존 Stage A/P04 실제 LLVM/Native regression도 같은 실행 묶음에서 통과했다.
- Cargo fmt, clippy `-D warnings`, workspace test, all-features check,
  standalone Runtime rustfmt, 문서 validator와 diff whitespace 검사를 모두 통과했다.
  LLVM/Clang 21.1.8, Rust 1.99.0/MSVC 14.44.35207 Windows x64 환경에서 검증했으며,
  declared MSRV 1.80은 별도 시험하지 않았다. Host Clang 실행은 Codex 제한 때문에 승인된 host 권한을 사용했다.

## 후속

단일 파일 전역 const·forward dependency/cycle은 이후 사용자 승인 [P06](GLOBAL_CONST_STAGE_B_PROPOSAL.md)과
[P06 구현 기록](GLOBAL_CONST_IMPLEMENTATION.md)에서 구현했다. 위 P05 검증 수치는 P05 완료 당시 기록이다.

const function, 숫자 승격/다른 primitive/aggregate,
module·multi-file/array-size generic·const cache/optimizer는 다음 별도 승인 범위다.
보완팩의 top-level const-divzero fixture와 전체 D09는 Draft로 유지했다.
Linux Native link/run, ownership/Drop/NLL과 전체 Stage B 완료는 이번 작업의 결과가 아니다.
