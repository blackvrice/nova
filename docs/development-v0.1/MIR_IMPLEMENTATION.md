# Stage A MIR Lowering·검증 구현 기록

작성일: 2026-10-04. 이 문서는 현재 Rust API와 검증 범위를 기록한다.
새 언어 의미 승인 문서가 아니다. 근거는 원본 NOVA-081/082/083/091/092와
[사용자 승인 P02](SEMANTICS_STAGE_A_PROPOSAL.md)의 source-order/short-circuit 계약이다.
Arithmetic runtime/formatting/ABI/entry 정책과 미래 Stage 상세는 Draft를 유지한다.
후속 [P03 승인](NATIVE_STAGE_A_PROPOSAL.md)으로 Stage A Native 최소 runtime 부분을 동결했다.
아래는 MIR 단계의 기록이며 현재 Native 상태는 [후속 구현](NATIVE_IMPLEMENTATION.md)을 따른다.

## 구현 API와 의존 방향

- `nova_mir::lower(hir, resolved, checked, upstream_has_errors) -> Result<Module, LoweringError>`.
- `upstream_has_errors`에는 Lexer와 Parser의 오류를 모두 포함해야 한다.
  삽입 토큰만 있는 recovery는 HIR에 Error Node가 없을 수 있다. Driver가 이 값을 전달할 책임이 있다.
- `FrontendErrors`는 기존 frontend 진단이 있으므로 MIR 생성을 중단한다는 결과다.
  호출자는 원래 Nxxxx 진단을 보존한다. MIR가 같은 사용자 오류를 새 진단으로 중복 보고하지 않는다.
- 공개 mutable side table은 resolve/check를 재계산해 현재 HIR 결과와 정확히 비교한다.
  길이뿐 아니라 call/constant/definition/type 변조도 `InvalidAnalysis`로 거부한다.
  그 후 Lowering하고 독립 validator가 성공한 경우에만 Module을 반환한다.
- `validate(&Module) -> Vec<ValidationError>`는 향후 변환 전후에도 사용할 수 있다.
  오류에는 body/block/source와 Violation이 있으며 compiler/API 불변 조건 오류다.
- Production 의존은 MIR → HIR/Resolve/Types/TypeCheck/Source/Syntax다.
  Lexer/Parser 연동은 dev dependency의 harness에만 있다. LLVM 의존이 없다.

## MIR 모델

- Non-SSA Place 기반이며 Module은 self-contained Callee signature registry와 함수별 Body를 갖는다.
  `DefId`는 기존 Resolution ID를 유지한다. `CalleeId`/`LocalId`/`BlockId`는 해당 Module/Body 안의 ID다.
- Body는 typed Local/parameter/BasicBlockData/entry로 구성한다. Local 타입은 고정 Stage A Type이며
  HIR TypeInterner의 수명과 무관하다. Binding statement의 Unit 타입과 declared local 타입을 구분한다.
- Operand는 Place 또는 Int32/Bool/String/Unit Constant. Rvalue는 Use/Unary/Binary/Interpolate다.
- Statement는 Assign. Terminator는 Goto/Branch/Call/Return/Unreachable이다.
  Call은 continuation block과 destination Place를 가진 Terminator이며 expression statement가 아니다.
- 모든 Local/Statement/Terminator/Body에 HirId/Span/SourceOrigin을 유지한다.
  원본 HIR provenance table도 Module에 보존하고 validator가 각 SourceInfo와 대조한다.
- MVP String은 abstract 값이다. Copy/Move/Read/Drop/borrow 분석과 physical string representation은
  여기서 정하지 않는다. Drop Terminator/cleanup 삽입은 해당 Stage 승인·구현 시 추가한다.

## Lowering과 관찰 순서

- 반복형 작업 스택으로 함수 body, 표현식, control flow를 내린다. 재귀적인 MIR 트리는 만들지 않는다.
  표현식 결과 table은 Module당 한 번 할당해 다수 함수에서 반복 초기화하지 않는다.
- 인수와 일반 operand는 source order. 각 호출을 continuation으로 연결하여 중첩 call 순서를 드러낸다.
  Grouped direct callee와 forward/recursive call도 static CalleeId를 사용한다.
- `&&`/`||`는 RHS/short-path/join block으로 변환하고 양쪽 경로가 Bool destination을 초기화한다.
  logical operator를 eager Binary Rvalue로 남기지 않는다.
- If는 then/else/join CFG. 양쪽이 return하면 orphan join에 Unreachable을 표시하며 entry에서 도달하지 않는다.
  이 Unreachable은 panic/trap 연산이 아니다. return 뒤의 source는 이미 frontend에서 검사했으므로
  실행 MIR에서는 생략한다. Unit fallthrough는 implicit Return(Unit)이다.
- `-2147483648`은 checker의 signed prefix constant를 사용하며 child magnitude를 별도로 내리지 않는다.
- 보간은 이미 평가된 Int32/Bool/String component의 순서를 유지하는 abstract Interpolate다.

## Validator 불변 조건

- 유효한 Callee/Body/parameter/Local/Block ID, unique definition ID, signature와 함수 Body 대응.
- Stage A value type만 허용하며 Error/Function value type, wrong call arity/type/destination,
  wrong return/condition/assignment type와 허용하지 않은 operator를 거부한다.
- 모든 block은 terminator를 갖고 모든 successor는 Body 안에 있어야 한다.
  source는 provenance와 일치하며 owner function Span 안에 있어야 한다.
- entry에서 도달 가능한 CFG에 must-initialized dataflow를 수행한다.
  predecessor 사실을 교집합으로 합치고 parameter는 entry boundary로 초기화한다.
  Call destination은 continuation edge에서만 초기화된다. 각 statement RHS를 assignment 전에 검사한다.
  한 branch에만 할당된 join 값, 초기화 전 argument/return/condition 읽기와 reachable Unreachable을 거부한다.
- Bitset fixed point이므로 cycle에도 종료한다. 이것은 MIR 검증이며 Nova loop 구문/Stage C Move 분석 구현이 아니다.
  Block 수 B, Local 수 L일 때 initialization facts 메모리는 O(B × ceil(L/64))다.
  unreachable block도 ID/type/source/terminator는 검사하며 초기화 검사는 reachable block에 적용한다.
- provenance는 Module 내부 정합성 검사다. 외부에서 Module과 provenance를 함께 위조한 경우 원본 파일의
  진위를 증명하지 않는다. 원본 byte/UTF-8 경계는 SourceDatabase와 HIR lowering에서 검사한다.

## Runtime·완료 경계

`+ - * / %`는 abstract Nova 연산으로 보존한다. LLVM wrapping/nsw/poison/checked policy를 선택하거나
arithmetic constant folding을 수행하지 않는다. `1/0`, `2147483647+1`, `MIN/-1`은 타입이 맞으면
MIR에 그대로 남는다. 실제 실행 시 결과/overflow/zero-division 정책은 Native 단계 전에 동결해야 한다.
print newline/number·Bool formatting/I/O failure/ABI/main 계약도 승인 전이다.
MIR validation 성공은 실행 가능한 Native code나 전체 ownership safety 보장이 아니다.
다음 단계는 위 최소 runtime/entry/host 계약의 구체안과 nova-codegen interface/LLVM Adapter다.
현재 `nova check/run`, LLVM object, linker, optimizer, cache/query, parallel compiler는 제공하지 않는다.

## 검증 증거

- MIR 18 tests: Hello snapshot/결정성, 호출 순서, short circuit call 생략, 양 branch 반환/fallthrough,
  scope shadow/initializer outer lookup, signed MIN, abstract arithmetic 보존, 보간 component/call 순서,
  fragment/forward/recursive/grouped call/Unicode CRLF, upstream 오류 차단과 table 변조 거부.
- Validator corruption tests: target/terminator/source/Local, call arity/type, ErrorType/operator,
  join definite assignment, duplicate definitions/parameters, missing body, reachable Unreachable,
  CFG cycle/Call-edge definition, 잘못된 return/condition/callee.
- Truncated UTF-8 source prefixes와 10,000항 flat expression, 10,000 binding의 안전한 처리.
- Bounded test oracle는 CFG/call trace와 Bool/Unit/in-range addition만 검사한다.
  문자열 보간 formatting/overflow/ABI/Native runtime 구현이나 실행 증거가 아니다.
- Windows Rust 1.99.0에서 전체 91 tests, fmt/clippy/workspace test/all-features check와 문서 validator를 확인한다.
  rust-version 1.80은 선언된 MSRV이며 해당 toolchain에서 별도 실행하지 않았다.
