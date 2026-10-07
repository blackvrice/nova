# Stage B loop·정수 범위 for 최소 계약 — P19

작성일: 2026-10-07. 상태: **Draft / 사용자 승인 대기 / 미구현**.
P01~P18은 보존한다. 이 문서·[전용 EBNF](GRAMMAR_STAGE_B_RANGE_LOOP.ebnf)·
[수용 fixture](range-loop-proposal-fixtures/README.md)는 검토 자료이며 승인 전 Compiler 의미에 적용하지 않는다.

## Specification Change Proposal

- 관련 문서: [NOVA-004 Canonical](../00_Governance/NOVA-004_Nova_용어_키워드_Canonical_표.md),
  [원본 NOVA-043](../04_Functions_Control/NOVA-043_if_while_for_loop_사양서.md),
  [원본 NOVA-044](../04_Functions_Control/NOVA-044_break_continue_return_사양서.md),
  NOVA-014/026/073~078/081/083/091/136. [P04](CONTROL_STAGE_B_PROPOSAL.md)·
  [P07](INTEGER_STAGE_B_PROPOSAL.md)·[P16](TRY_STAGE_B_PROPOSAL.md)·[P18](DEFAULT_ARGUMENTS_STAGE_B_PROPOSAL.md)를 의존 계약으로 사용한다.
- 현재 사양: Stage B는 for/loop를 포함하고 Canonical은 until 끝 제외·through 끝 포함을 정했다.
  승인 P04의 while/bare break/continue는 구현했지만 for/loop·range type/protocol·종료 분석 상세는 미동결이다.
- 발견된 문제: binder scope/type, bound 평가 순서/횟수, 빈·역방향 범위,
  inclusive 정수 최댓값에서 overflow 없이 종료하는 방법을 Compiler가 추측할 수 없다.
- 제안 변경: statement loop와 정수 ascending range 전용 for를 아래 최소 계약으로 동결한다.
- 변경 이유: 기존 정수·scope·CFG·checked arithmetic·try를 사용해 Stage B 반복문을 확장한다.
- 영향 범위: Parser/AST/HIR/Resolver/TypeChecker, MIR CFG·검증, LLVM/Native 테스트와 문서.
  기존 Lexer keyword/END, Types 숫자 정책, private 함수 ABI·Runtime API는 보존한다.
- Backward Compatibility: 기존 유효 프로그램 의미·진단·출력을 보존한다.
  지금까지 미지원이던 for/loop 문장만 수용하며 기존 while의 보수적 return 분석도 유지한다.
- 대안: loop만 추가하거나 Array/iterator/일반 Range 값까지 함께 동결한다.
  정수 전용 for는 일반 iterable·Generic·Move/Drop 결정을 앞당기지 않는 실행 가능한 최소 범위다.

## 문법과 문장 의미

1. `loop { statements }`와 `for IDENT in start until end { statements }` /
   `for IDENT in start through end { statements }`를 Unit **문장**으로 지원한다.
   binder type annotation·let/var·destructuring·wildcard pattern은 이 단계에 없다. `_` binder는 N1102다.
   until/through는 for header에만 허용한다. Range 값/타입·대입·함수 전달·괄호로 묶은 전체 range는 미지원이다.
   start/end는 기존 P18 expression 전체다. range operator는 이 두 expression보다 낮은 header 구분자이며 연쇄는 N1103다.
2. loop는 body entry로 바로 진입한다. 정상 body 종료/continue는 다음 iteration의 body entry로 이동한다.
   break는 그 loop의 다음 문장으로, return/try Error는 함수 exit로 이동한다.
3. for는 **start를 한 번 평가·정수 변환·snapshot한 뒤 end를 한 번 평가·변환·snapshot**한다.
   body 밖 최초 진입에서 이 순서가 유지된다. 빈 범위도 두 bound를 평가한다.
   start 평가가 Abort/try Error이면 end·body를 실행하지 않는다. end 실패이면 body를 실행하지 않는다.
   body에서 bound가 참조한 outer var를 바꾸어도 저장한 시작/끝 값은 바뀌지 않는다.
4. ascending, step +1만 지원한다. until은 start < end일 때 start부터 end 직전까지,
   through는 start <= end일 때 start부터 end까지 방문한다. start > end는 둘 다 빈 범위다.
   start == end는 until 0회, through 1회다. 자동 descending/음의 step은 없다.
5. 정상 body 종료/continue는 advance 단계로 이동한다. through는 current == saved_end이면
   **증가시키지 않고 종료**한다. 나머지 경우만 +1 후 header로 이동한다.
   until은 current < end guard가 참인 iteration 뒤 +1이 표현 가능하므로 overflow하지 않는다.
   signed/unsigned 최댓값을 through로 마지막 방문한 뒤의 정상 종료·continue는 모두 Abort/wrap 없이 끝나야 한다.
   사용자 body/bound 산술은 기존 checked arithmetic/Abort를 그대로 적용한다.

## 정수 타입·scope·진단

6. bound는 P07의 8종 정수와 alias만 지원한다. Bool/Char/Float/String/Unit/aggregate/Option/Result는 N2101이다.
   두 operand의 literal-only 부분식과 peer integer 문맥은 **P07 정수 비교와 동일**하다.
   typed peer 하나면 반대 literal-only bound가 그 타입을 사용하고, 둘 다 literal-only이면 Int32다.
   cast의 operand 문맥 격리와 literal range는 P10/P07을 따른다. Bool/함수 반환 타입/바깥 선언 기대 타입은 전달하지 않는다.
   타입 확정 뒤 P07 손실 없는 공통 정수 타입으로 두 bound를 변환한다. 공통 타입이 없으면 N2101이다.
   binder는 그 공통 타입이다. 예: int8와 uint8 typed bound는 int16, int64와 uint64는 거부한다.
   peer 추론 방문 순서와 실제 source-order 실행은 구분한다. ErrorType 파생 타입 오류를 억제한다.
7. bounds는 binder 선언 이전 outer scope에서 해석한다. body마다 값이 새로 설정되는 **불변 Copy binder**다.
   body top-level binding과 동일 scope이므로 같은 이름 재선언은 N2002, binder 대입은 N3004다.
   outer 같은 이름은 shadow하고 inner block은 binder를 다시 shadow할 수 있다. body 밖 binder는 보이지 않는다.
   hidden mutable induction temporary와 공개 불변 binder를 분리하고 사용자 DefId를 hidden counter로 재사용하지 않는다.
8. bare break/continue는 while/for/loop 중 가장 가까운 loop에 대응한다. if/match block에서도 동일하다.
   label/value jump는 기존 미지원 정책을 유지한다. jump 뒤 unreachable code도 이름·타입 검사는 수행한다.
   **for/loop도 while처럼 fallthrough 가능으로 보수적으로 분석**한다. non-Unit 함수의 loop body return만으로
   필수 return을 충족하지 않는다. 뒤의 명시적 return 등 기존 P02 return 규칙이 필요하다.
   Never/무한 반복 증명·상수 조건별 termination 추론을 추가하지 않는다.
9. 신규 diagnostic code는 없다. type mismatch N2101: 비정수 bound expression,
   no-common-integer N2101: start에서 end까지 header range Span. literal range N2102: 기존 literal/prefix Span.
   undefined N2001: reference, duplicate N2002: 나중 선언 name, immutable N3004: assignment target/binder secondary.
   loop 밖 jump N3002: keyword. missing return N3003: function body/return signature.
   unsupported N1102: 해당 binder/iterable/range operator/token. 연쇄 N1103: 두 번째 until/through.
   missing token N1101: unexpected/zero-width token. nesting limit 128과 반복 construct 초과 N8901은 P04를 유지한다.
10. D05 END·newline/semicolon·brace boundary와 P01 recovery를 유지한다. until/through 주변 newline은
    기존 continuation을 사용한다. body recovery 뒤 다음 함수 분석을 계속한다. general Drop/iteration cleanup은 약속하지 않는다.
    dynamic String arena는 기존 entry 종료 수명이며 loop 생성량에 비례해 늘어날 수 있다.

## MIR·검증·수용 계획

- AST/HIR는 for/loop keyword·binder name·in/range operator·각 bound/whole header/body·원 FileId/byte Span을 보존한다.
  외부 AST child shape/order·token spelling·UTF-8/parent 포함을 검사한다. [54-production EBNF](GRAMMAR_STAGE_B_RANGE_LOOP.ebnf)는
  P18 statement 한 production 확장과 for_stmt/loop_stmt 두 production 추가만 허용하며 기존 나머지 51개를 보존한다.
- Typed analysis는 range common type·bound conversions·binder identity를 저장한다. public typed metadata를 재검사한다.
  MIR은 preheader의 source-order bound snapshots, header guard, body binder copy, advance, exit를 명시한다.
  body가 guard/initialization을 우회하거나 종료한 inclusive 값의 increment로 갈 수 없어야 한다.
  nested target stack·return/try early exit·기존 definite-initialization fixed point를 보존한다.
- MIR validator는 원 SourceInfo/DefId/type·원 bound 순서/단일 평가·signedness/guard·inclusive equality-before-increment·
  hidden counter/immutable binder 구분·continue/break target을 확인한다. private proof 또는 동일 수준의 독립 검증으로
  value/source/type/CFG/guard/advance/entry/try/effect 위조를 CodegenUnit/LLVM 전에 거부한다.
  Backend는 range 의미를 추론하지 않고 검증된 CFG·기존 integer instruction을 lowering한다.
- Parser/recovery/Unicode prefix·scope·정수 8종/alias·peer literal·promotion/no-common·0/1회/역방향·MAX/MIN·bound mutation·
  nested mixed loops·match jump·return/try·default/named/import·checked failure Source Span을 시험한다.
  const initializer/default 안 loop/for statement를 값으로 받아들이지 않고 기존 const subset을 보존한다.
- 두 파일 정상·별도 정상 2개·부정 18개 exact byte Span/cascade·Runtime 실패 2개를 [fixture](range-loop-proposal-fixtures/README.md)로 준비했다.
  18줄 stdout은 **제안 기대값**이다. Compiler 구현·Native 성공 기록은 아직 없다.
- 승인 후 Windows Native debug/release, real LLVM COFF/ELF O0/O2, 전체 fmt/clippy/test/all-features와 Runtime fmt·문서 검사를 실행한다.
  flat 대량 loops·128/129 nesting·작은 host stack·일반 CFG fixed point 성능 회귀를 확인한다.
  Linux Native 실행·MSRV 실행은 별도 환경 검증이다.

## 승인 경계

P19 loop·정수 range 전용 for subset만 승인 대상이다. Array/iterable protocol·Range 값·step/descending·
pattern binder·labeled/value jump·Never/termination inference·일반 Move/borrow/Drop·사용자 Generic·
공용 ABI/FFI·전체 D08/D10/D12/D16/D23/D25는 승인하지 않는다.
기존 P01~P18·원본 148개·Canonical은 보존한다. 승인 전에는 draft ledger·검토 문서·fixture만 작성한다.

## 초안 준비 검증 — 2026-10-07

- 문서 build/validator PASS: 148개 원본 hash·로컬 링크·Draft ledger·54-production EBNF와 P18의 기존 51개 production 보존,
  두 파일/정상 2/부정 18/Runtime 2 fixture의 UTF-8 byte Span·cascade·제안 18줄 metadata를 검사했다.
- HEAD 대비 D01~D05/P01~P18 accepted ledger·승인 문서/EBNF·Canonical/Accepted Lexer·원본 148개 SHA-256을 보존했다.
  Compiler/Runtime/Cargo source 변경은 없다. 사용자 예제와 IDE 변경도 이번 작업에서 제외한다.
- 기존 전체 `cargo test --workspace --offline`: **316 PASS / 0 FAIL / 58 ignored**.
  이번 문서 작업에서 실제 LLVM/Native opt-in 58개 전체는 재실행하지 않았다. 이전 전체 증거는 [P18 구현 기록](DEFAULT_ARGUMENTS_IMPLEMENTATION.md)을 따른다.
- `cargo fmt --check`, 별도 Runtime `rustfmt --check --edition 2021 crates/nova-cli/runtime/stage_a.rs`,
  `cargo clippy --workspace --all-targets --offline -- -D warnings`, `cargo check --workspace --all-features --offline` PASS.
- ignored target의 **기존 P18 문법·while/명시적 counter만 사용하는 대조 프로그램**을 check 및 Windows Native debug/release로 실행했다.
  check는 출력 없이 exit 0, Native는 제안과 같은 18줄 UTF-8/LF stdout·빈 stderr·exit 0이다.
  이는 기존 숫자/출력 값 검산이며 P19 구문·새 binder scope·range promotion·단일 평가·guard/advance CFG의 구현 증거가 아니다.
  tracked P19 fixture는 Draft / implementation_verified:false를 유지한다.
