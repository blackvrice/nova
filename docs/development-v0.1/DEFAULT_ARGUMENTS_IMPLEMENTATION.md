# P18 상수 표현식 함수 기본 인수 구현·검증 기록

승인일/구현일: 2026-10-07. 사용자 “P18 승인하고 함수 기본 인수 구현 진행” 답변으로
[최소 계약](DEFAULT_ARGUMENTS_STAGE_B_PROPOSAL.md)·[52-production EBNF](GRAMMAR_STAGE_B_DEFAULT_ARGUMENTS.ebnf)를 승인했다.
원본 148개와 D01~D05/P01~P17의 승인 의미를 보존한다.

## 구현

- Parser는 parameter type child 뒤 optional DefaultValue wrapper/한 expression을 저장한다.
  AST/HIR는 `=` Span·initializer/parameter Span·원 AstId/SourceOrigin을 보존한다. EBNF는 parameter 한 production만 확장했다.
  외부 AST의 wrapper spelling·child shape/order·parent 포함·trivia gap·UTF-8 경계를 검사한다. Lexer/END 변경은 없다.
- Resolver는 initializer를 함수의 declaration-module root scope에서 방문한다. parameter/body/caller scope를 사용하지 않는다.
  forward global/import alias·private global/type 접근과 동일 철자 global/parameter/caller shadow를 처리한다.
- TypeChecker는 global const 검사 후 모든 parameter default를 한 번 검사·평가한다. 사용하지 않거나 override해도 실패를 숨기지 않는다.
  parameter 기대 타입·기존 숫자/Copy/Option/Result 문맥과 const evaluator를 재사용한다. initializer당 10,000-node 예산과 skipped RHS legality를 유지한다.
  try는 keyword N3201이며 파생 N3002를 억제한다. upstream name/type 실패와 global const 실패는 파생 평가 오류를 억제한다.
- Checked.defaults는 callee/parameter identity·index·initializer HirId·ConstEvaluation 상태/값/node 수를 보관한다.
  mapped call metadata는 제공 source argument→parameter 대응과 declaration-order omitted defaults를 구분한다.
  default 없는 함수의 P17 bijection/진단을 유지하고 필수/default parameter의 어느 선언 순서든 허용한다.
  default 전체 override·순수 positional·mixed named·all-default 빈 호출을 지원한다.
- MIR은 제공 인수를 소스 순서로 평가·변환·snapshot한 다음 생략 default Constant를 declaration order로 caller temporary에 저장한다.
  모든 값을 parameter order로 기존 Call에 전달한다. callee ABI/prologue·LLVM/Runtime production/API는 바꾸지 않았다.
  default는 runtime effect/산술을 만들지 않으며 제공 인수의 try Error는 default write와 callee를 건너뛴다.
- MIR private certificate는 default origin/value/index·snapshot 위치·단일 write·인수 slot·callee signature·전체 caller Body/CFG를 결합한다.
  declaration initializer가 caller body 밖/다른 FileId에 있어도 private default_sources로 인증한 원 SourceInfo만 허용한다.
  제공 인수/Call은 caller SourceInfo, default write는 원 선언 SourceInfo다. 일반 source 범위 검사를 전역으로 완화하지 않았다.
  public Checked 값을 재검사하며 같은 타입 값 swap·default 값/source/write/order·CFG/try/effect 위조를 Adapter 전에 거부한다.
  미래 MIR transform은 인증 Body/certificate를 함께 갱신해야 한다. 현재 optimizer pass는 없다.
- signature default 여부와 label lookup을 재사용하며 default declaration은 caller 수만큼 재평가하지 않는다.
  새 인수 cap은 없다. 1,024개 default와 빈 호출을 256 KiB host stack에서 분석·lower·검증·해제했다.

## 추가 테스트

기본 10개·실제 LLVM 1개·Windows Native 2개, 총 13개를 추가했다.

- Parser/HIR: default/equal/source metadata·nested type `>=` split·trailing comma·Unicode prefix recovery·외부 AST 위조.
- TypeChecker: 부정 fixture 19개 exact code/primary Span/cascade·사용하지 않거나 override한 default 실패·선언 scope/forward/import/private/shadow·
  숫자 10종/Bool/Char/Unit/String/Copy aggregate/nullable/Result·const/checked cast·global cycle 실패 억제.
  10,000-node 경계·10,001 초과·skipped RHS budget 및 parameter별 독립 예산을 확인했다.
- MIR: public default value/mapping 위조·default origin/type·제공 snapshot/생략 순서·같은 타입 swap·value/source/local/statement/entry/CFG 변조·
  try Error의 default/callee 우회·작은 host stack 대량 flat defaults를 검사했다.
- CLI: 정상 두 파일/별도 정상 2개, 20번째 private import N2004 전체 import Span 0..20,
  invalid check/build/run에서 도구를 호출하지 않고 기존 output을 보존하는 gate.
- LLVM: deterministic private ABI/try, Windows COFF/Linux ELF O0/O2 객체 생성.
- Native O0/O2: 수용 fixture 20줄과 정상 2개·숫자 전체 경계·음의 zero·UTF-8/NUL String·Char·Unit·Copy tuple/struct/Enum/Result·override·
  private type/global·default String 생존·try early return·callee overflow의 정확한 cross-file UTF-8 Span과 effect order.

## 검증 결과

- 기본 전체 회귀: **316 PASS / 0 FAIL / 58 ignored**.
- 실제 LLVM 전체: **14 PASS**, COFF/ELF O0/O2 포함.
- P18 Native 집중: **2 PASS** (10.73 s), debug/release 포함.
- Windows Native 전체: **44 PASS / 0 FAIL** (349.88 s), debug/release 포함. 기본·LLVM·Native 합계 **374 PASS**.
- fmt·별도 Runtime rustfmt·clippy `-D warnings`·all-features·문서 validator·git diff whitespace 검사 PASS.
- 독립 예제 check와 debug/release Native를 직접 실행해 20줄의 UTF-8/LF stdout·빈 stderr·exit 0을 byte 단위로 확인했다.
  사용하지 않는 default의 checked overflow도 N3201·exit 1로 확인했다.
- 기존 D01~D05/P01~P17 승인 문서·EBNF·Canonical/Accepted Lexer와 원본 148개 SHA-256을 이전 HEAD와 대조해 보존을 확인했다.
  사용자가 수정한 examples/enums.nova·examples/try_result.nova 및 .idea/·examples/function.nova는 이번 변경에서 제외한다.

초기 연결 검사에서는 declaration SourceInfo를 기존 caller body 범위 검사가 거부했다.
private source certificate를 추가해 원 FileId/Span을 보존하며 해결했다. 새 테스트의 dump API/slice inference 오류도 수정했다.
기존 unsupported default Parser 사례는 승인 P18 수용/집중 회귀로 대체했고 다른 기존 테스트는 완화하지 않았다.

환경: Rust 1.99.0, LLVM 21.1.8, Windows x64 MSVC. MSRV 1.80은 별도 실행하지 않았다.
Linux host Native 실행은 후속이며 ELF 객체 검증과 구분한다.

## 직접 실행과 남은 범위

[독립 예제](../../examples/default_arguments.nova)·[TESTING.md](../../TESTING.md)를 따른다.
check는 출력 없이 exit 0, debug/release는 [수용 fixture의 20줄](default-arguments-proposal-fixtures/README.md)·LF·빈 stderr·exit 0이다.
runtime/parameter 의존 default·const function·overload·method/receiver·named constructor·사용자 Generic·Array·
일반 Move/borrow/Drop·public ABI/FFI·별도 컴파일/cache 제품화와 전체 D09/D11/D16/D25/D30은 후속이다.
