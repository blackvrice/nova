# P19 loop·정수 범위 for 구현·검증 기록

승인 반영/구현일: 2026-10-08. 사용자 “P19 승인하고 loop·정수 범위 for 구현 진행” 답변으로
[P19 계약](RANGE_LOOP_STAGE_B_PROPOSAL.md)·[54-production EBNF](GRAMMAR_STAGE_B_RANGE_LOOP.ebnf)를 적용했다.
기존 D01~D05/P01~P18과 원본 148개·Canonical을 보존한다.

## 구현

- AST/HIR For는 Binder/start/end/body의 source-order child와 for/in/until·through/whole-range Span을 보관한다.
  Loop는 keyword·body를 보관한다. 외부 AST child shape/order·spelling·trivia gap·UTF-8/parent 포함을 검사한다.
  Parser의 keyword/END는 변경하지 않았으며 128 nesting/P04 N8901·복구를 유지한다.
  잘린 입력의 빈 binder는 ErrorNode로 복구한다. HIR 신규 metadata 검사도 child 수 확인 후 indexing한다.
- Resolver는 bounds를 outer scope에서 해석하고 body scope에 불변 Copy binder DefId를 선언한다.
  동일 body scope 재선언·outer shadow·inner shadow·body 밖 참조와 기존 가변 대입 진단을 보존한다.
- TypeChecker는 iterative Work/Peer로 P07 정수 literal 문맥을 재사용하고 정수 8종의 lossless common type을 계산한다.
  Checked.ranges에 binder identity/common TypeId를 기록하며 기존 coercion table에 bound 변환을 저장한다.
  non-integer/N2101·no-common/header Span·literal/N2102·ErrorType cascade를 검사한다.
  loop/for의 return flow는 while처럼 보수적 fallthrough다. 범위/반복을 const expression으로 확장하지 않았다.
- MIR은 start 변환/snapshot → end 변환/snapshot → header guard → immutable binder copy/body → advance → exit를 구성한다.
  until은 <, through는 <=다. through advance는 current == saved_end이면 increment 없이 exit한다.
  hidden mutable induction temporary와 사용자 binder local/DefId를 분리한다. continue는 advance, loop continue는 body entry로 간다.
  mixed while/for/loop target stack·return/try 조기 exit와 definite-initialization fixed point를 보존한다.
- public Resolver/Checked table은 재계산으로 검증한다. private loop_bodies는 P19 반복문이 lowering된 각 함수 Body의
  entry·locals·statements·terminators·SourceInfo를 한 번 저장한다. 독립 validator는 full-body 동일성을 검사하므로
  bounds 평가/순서·snapshot·guard/advance·signedness·binder write·nested targets·try/effect/source 위조를 Adapter 전에 거부한다.
  미래 MIR transform은 이 private proof도 함께 갱신해야 한다. 현재 optimizer pass는 없다.
  P17/P18 named/default proof와 함께 있는 함수는 각각의 기존 certificate를 유지한다.
- LLVM/Runtime production/API·함수 ABI를 변경하지 않았다. 기존 integer compare·checked add·CFG로 lowering한다.

## 추가 테스트

기본 10개·실제 LLVM 1개·Windows Native 2개, 총 13개를 추가했다.

- Parser 2: 원 source Span/order·until 주변 newline·Unicode prefix truncation·잘못된 syntax의 다음 함수 복구,
  nesting 한도와 256 KiB host stack에서 flat 1,024개 for/loop.
- HIR 1: prefix recovery/원 SourceOrigin·child shape/order·keyword/in/operator/range spelling 및 외부 AST 9개 위조.
- TypeChecker 2: 부정 18 fixture exact code/UTF-8 byte Span/cascade, 정수 8종 MIN/MAX·peer/context·공통 타입·scope·불변 binder·return 분석.
- MIR 3: bound snapshot/continue·MAX exit oracle, public RangeInfo 위조·try bypass·same-type value/Source/entry/CFG 변조,
  256 KiB host stack의 flat 256개 loop 쌍과 cyclic definite-initialization 검사.
- CLI 1: 정상 두 파일/정상 2/Runtime 2 check, invalid check/build/run 도구 미실행·기존 output 보존.
- LLVM 2: deterministic signedness/guard CFG와 Windows COFF/Linux ELF O0/O2 객체.
- Native 2: 수용 18줄/정상 2·8종 MIN/MAX·정상 종료와 continue의 inclusive MAX·빈/역방향·mixed promotion·
  bound mutation snapshot·named/default/import·mixed loops/match·return/try start·end 실패·String arena/Copy/Unit body,
  start/end/body checked failure의 cross-file UTF-8 exact Span·Abort 전 effect.

## 검증 결과

- 기본 전체 회귀: **326 PASS / 0 FAIL / 61 ignored**.
- 실제 LLVM 전체: **15 PASS** (11.14 s), COFF/ELF O0/O2 포함.
- P19 Native 집중: **2 PASS** (26.61 s), debug/release 포함.
- Windows Native 전체: **46 PASS / 0 FAIL** (526.93 s), debug/release 포함. 기본·LLVM·Native 합계 **387 PASS**.
- fmt·별도 Runtime rustfmt·clippy -D warnings·all-features PASS.
- 문서 validator PASS. 독립 예제 check/debug/release의 정확한 18줄 UTF-8/LF 출력·빈 stderr·exit 0을 확인했다.
  Runtime 부정 2 fixture의 debug/release Abort 전 출력·정확한 Source Span과 불변 binder N3004·exit 1도 확인했다.
- D01~D05/P01~P18 승인 ledger·문서·EBNF, Canonical, 원본 148개 SHA-256 보존을 확인했다.
  사용자 작업 파일은 이번 커밋 범위에서 제외한다.

검증 중 빈 for binder recovery 오류를 수정했다. Unit bound fixture는 실제 bound 대신 함수 괄호를 가리켜
기대 Span을 22..24로 정정했다. 의미/진단 규칙 변경은 아니다.
테스트 생성 도구의 Windows cp949 오류는 UTF-8로, 테스트의 Checked clone 사용은 재검사 API로 수정했다.
대형 CFG 테스트가 기존 oracle의 1,000-step 한도를 초과해 해당 사례만 10,000-step 유한 예산을 사용한다.
기존 사례의 한도는 유지하며 primitive <=를 oracle에 추가했다. 기존 테스트를 삭제하거나 완화하지 않았다.

환경: Rust 1.99.0, LLVM 21.1.8, Windows x64 MSVC. MSRV 1.80은 별도 실행하지 않았다.
Linux Native host는 미검증이며 ELF 객체 생성과 구분한다.

## 직접 실행과 남은 범위

[독립 예제](../../examples/range_loops.nova)·[TESTING.md](../../TESTING.md)를 따른다.
check는 출력 없이 exit 0, debug/release는 [수용 fixture의 18줄](range-loop-proposal-fixtures/README.md)·LF·빈 stderr·exit 0이다.
Array/iterable protocol·Range 값·step/descending·pattern binder·labeled/value jump·Never/termination inference·
일반 Move/borrow/Drop·사용자 Generic·공용 ABI/FFI와 전체 D08/D10/D12/D16/D23/D25는 후속이다.
