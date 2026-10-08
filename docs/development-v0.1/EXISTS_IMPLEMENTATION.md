# P20 Copy Option postfix exists 구현·검증 기록

승인 반영/구현일: 2026-10-08. 사용자 “P20 승인하고 Copy Option exists 구현 진행” 답변으로
[P20 계약](EXISTS_STAGE_B_PROPOSAL.md)·[54-production EBNF](GRAMMAR_STAGE_B_EXISTS.ebnf)를 적용했다.
D01~D05/P01~P19·원본 148개·Canonical을 보존한다.

## 구현

- AST/HIR Exists는 원 operand child 하나와 keyword/whole expression byte Span을 보존한다.
  Parser postfix 층은 반복형이며 call/selector/as와 왼쪽 결합, prefix보다 강하다. Scanner/END는 그대로다.
- HIR은 child 수·spelling·XID 토큰 양쪽 경계·parent/UTF-8 포함·operand start/order·whole end·trivia gap을 검사한다.
  xexists·exists_extra·exists1의 일부를 keyword로 위조한 외부 AST를 거부하고 ()exists는 허용한다.
- TypeChecker는 operand expected context를 격리하고 intrinsic Option specialization만 Bool로 검사한다.
  user Enum Option/Result·non-Option N2101, unresolved None N2103·ErrorType cascade 억제를 유지한다.
  Resolver/Types registry·layout·nominal identity·private visibility를 변경하지 않았다.
- Const evaluator는 Exists 1 node와 전체 operand subtree를 기존 반복형 permission/count/evaluation에 연결한다.
  cached reference·10,000-node·static dependency/SCC·checked failure·skipped RHS permission은 그대로다.
  outer discriminant를 검사하고 Some(false/0/Unit/None)은 true다. payload 생성 자체는 생략하지 않는다.
- MIR은 source-order operand 평가 → private Copy snapshot → 원 intrinsic Some/None tag dispatch →
  양 edge의 Bool destination 초기화 → join으로 lowering한다. payload read·Runtime helper·ABI 변경은 없다.
  기존 Match terminator와 typed provenance를 재사용하고 LLVM/Runtime production은 변경하지 않았다.
- public Resolver/Checked table은 재계산 검증한다. private exists_bodies는 Exists가 lowering된 함수 Body의
  entry/locals/writes/terminators/SourceInfo를 한 번 보존한다. full-body 비교와 기존 family/schema/source/CFG 검증으로
  snapshot·tag direction·단일 평가·named/default/loop·short-circuit·try bypass·effect 위조를 LLVM 전에 거부한다.
  미래 MIR transform은 이 private proof도 함께 갱신해야 한다. 현재 별도 optimizer pass는 없다.

## 추가 테스트

기본 10개·실제 LLVM 1개·Windows Native 2개, 총 13개를 추가했다.

- Parser 2: prefix/postfix 결합·원 Span·D05 newline/semicolon·UTF-8 prefix recovery·뒤 함수 복구,
  256 KiB host stack flat 8,192 postfix와 loop 내부 nesting 한도.
- HIR 1: 원 source/origin·prefix truncation·외부 AST shape/order/spelling/gap·토큰 경계·공백 없는 postfix.
- TypeChecker 3: 부정 20개 exact UTF-8 Span/cascade, builtin/nominal/shadow/import/private·Copy payload 종류,
  Some(false/0/Unit/None)·Bool contexts/loops·const/default·forward/cross-file cycles·skipped failure/permission,
  local/global/default의 10,000/10,001-node 한도.
- MIR 2: 독립 discriminant oracle·Some/None Bool 결과·print 단일 호출·payload read 부재,
  CFG/write/source/entry/snapshot/tag/family·public typed metadata·try bypass 위조,
  작은 host stack flat 512 predicate CFG와 definite-initialization.
- CLI 1: 두 파일/정상 2/Runtime 1 check와 invalid check/build/run의 도구 미실행·output 보존.
- LLVM 2: deterministic tag/Bool CFG와 COFF/ELF O0/O2 객체.
- Native 2: 수용 18줄·정상 2·정수 8종 MIN/MAX/None 함수 ABI·zero/char NUL/nested Result/Copy Enum·
  while/loop/for/Bool match·single effect·snapshot·named/default·try Error early exit·short-circuit,
  cross-file UTF-8 checked Abort의 정확한 file/span과 Abort 전 effect.

## 검증 결과

- 기본 전체 회귀: **336 PASS / 0 FAIL / 64 ignored**.
- 실제 LLVM 전체: **16 PASS / 0 FAIL**, 11.96초. COFF/ELF 객체·O0/O2를 검증했다.
- Windows Native 전체: **48 PASS / 0 FAIL**, 584.62초. debug/release 실제 실행을 검증했다.
- 세 suite 합계 **400 PASS / 0 FAIL**이며 opt-in 64개도 LLVM 16개·Native 48개로 별도 실행했다.
- fmt·별도 Runtime rustfmt·clippy -D warnings·all-features PASS.
- 문서 build/validator·P01~P19 승인 문서/EBNF/ledger·D01~D05·Canonical 보존·원본 148개 SHA-256 PASS.
- 독립 examples/exists.nova check와 debug/release의 18줄 UTF-8 LF·빈 stderr·exit 0 PASS.
  Runtime operand_abort의 debug/release는 before LF만 출력하며 원 file#0:91..98의 checked Abort를 확인했다.
  undefined_operand check는 N2001·exit 1이며 파생 N2101을 억제했다.

진단 fixture의 try_precedence는 Result fetch()에서 exists N2101이 먼저 발생해 primary를 153..160으로 정정했다.
const/default runtime call 3사례는 함수 선언의 동일 철자 대신 실제 call Span으로 정정했다.
기존 P02 ErrorType 억제·P05/P18 permission 진단을 보존하는 기대값 수정이며 새 의미 정책 변경은 아니다.
테스트 oracle의 call 기록은 문자열 인수가 아닌 callee 이름(print)으로 확인하며, flat 입력에는 고유 binder 이름을 썼다.
nesting 테스트는 기존 P04 loop 내부 N8901을 검사한다. 일반 P01 expression 한도 N1102는 보존했다.
외부 AST의 keyword 경계를 보완하며 초기 Native 회귀는 중단하고 최종 코드로 재실행했다.
기존 테스트를 삭제하거나 완화하지 않았다.

환경: Rust 1.99.0, LLVM 21.1.8, Windows x64 MSVC. MSRV 1.80은 별도 실행하지 않았다.
Linux Native host는 미검증이며 ELF 객체 생성과 구분한다.

## 직접 실행과 후속

[독립 예제](../../examples/exists.nova)·[TESTING.md](../../TESTING.md)를 따른다.
check는 출력 없이 exit 0, debug/release는 [18줄](exists-proposal-fixtures/README.md)·LF·빈 stderr·exit 0이다.
Result exists·flow narrowing·unwrap·Option try·String/Move payload·일반 borrow/Drop·Array·methods·
공용 ABI/FFI·전체 D08/D09/D10/D16/D23/D25/D30은 후속이다.
