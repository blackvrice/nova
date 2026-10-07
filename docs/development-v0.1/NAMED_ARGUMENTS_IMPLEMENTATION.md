# P17 함수 이름 인수 구현·검증 기록

승인일/구현일: 2026-10-07. 사용자 “P17 승인하고 이름 인수 구현 진행” 답변으로
[최소 계약](NAMED_ARGUMENTS_STAGE_B_PROPOSAL.md)·[52-production EBNF](GRAMMAR_STAGE_B_NAMED_ARGUMENTS.ebnf)를 승인했다.
원본 148개와 D01~D05/P01~P16 승인 의미를 보존한다.

## 구현

- Parser/AST/HIR는 `label: expression`을 source-order Call의 한 child wrapper로 보존한다.
  label·colon·expression·wrapper의 byte Span과 원 AstId를 유지하며 외부 AST의 shape/철자/순서/UTF-8 경계를 검사한다.
  Lexer/END 변경은 없다. HIR bundle은 label symbol도 remap한다.
- TypeChecker는 signature별 parameter 이름 lookup과 source argument→parameter 대응을 생성한다.
  leading positional 후 named 순서를 허용하고 unknown·duplicate·collision·missing·excess·positional-after-named를 N2201로 거부한다.
  duplicate는 첫 인수/선언, missing은 첫 누락 parameter, unknown은 제한된 알려진 이름 hint를 제공한다.
  builtin print 및 Struct/Enum/Option/Result constructor는 named label을 거부한다. 사용자 print shadow는 유지한다.
- expected type은 대응한 parameter를 따른다. integer boundary/승격·nullable/Result·Tuple/Struct/Enum·String·Unit을 지원한다.
  잘못된 mapping은 추측한 expected type을 쓰지 않으며 independent expression 오류를 유지한다.
  unresolved callee의 ErrorType에서는 파생 N2201/N2101을 억제한다. const 함수 호출은 전체 Call N3201이다.
- MIR은 각 인수를 소스 순서로 한 번 평가·변환·snapshot한 후 operand만 parameter 순서로 배치한다.
  기존 Call/private ABI/String arena를 사용한다. try Error는 이후 인수와 callee를 실행하지 않는다.
  LLVM/Runtime production 변경이나 새 cleanup/API는 없다.
- public typed mapping은 재검사 결과와 비교한다. MIR의 private certificate는 mapping·signature·source·snapshot 위치/단일 write·전달 순서와
  해당 함수 Body의 entry/CFG/statements/terminators를 검증한다. 동일 타입 swap이나 effect/try 우회도 Adapter 진입 전에 차단한다.
  현재 MIR 변환 pass는 없다. 미래 최적화 pass는 인증된 Body와 certificate를 함께 갱신해야 한다.
- parameter lookup과 snapshot certificate 위치는 직접 인덱싱한다. 새 언어 인수 cap은 없다.
  평평한 1,024-parameter named call을 256 KiB host stack에서 분석·lower·검증했다. 기존 parser/module/type/const 한도는 유지한다.

## 추가 테스트

기본 9개, 실제 LLVM 1개, Windows Native 2개로 총 12개를 추가했다.

- Parser/HIR: source spans·trailing comma·보간/중첩·Unicode prefix recovery·외부 AST 위조.
- TypeChecker: 두 파일 alias·forward/recursive/grouped callee·print shadow·label/value 분리·exact spelling·mapped contexts·const/cascade.
- MIR: source-order snapshot/parameter-order 전달·public mapping 위조·같은 타입 operand swap·snapshot write/source/CFG/effect/try 변조.
- CLI: main/별도 정상 2개 check, private import exact N2004, 오류 check/build/run gate와 기존 output 보존.
  [부정 fixture 16개](named-arguments-proposal-fixtures/expected.json)의 code·정확한 primary Span·cascade를 검사한다.
- LLVM: deterministic 기존 private Call ABI·Windows COFF/Linux ELF O0/O2 object 생성.
- Native O0/O2: 24줄 effect/try fixture·정상 2개·숫자 10종/Bool/Char/Unit/String/Copy payload·mutable place snapshot·
  early return 이후 String 생존·short-circuit·overflow의 정확한 Span과 이후 effect 미실행.

## 기존 계약 보존과 정정

P17 Draft의 private import 기대 Span 4..19는 기존 P11의 실제 N2004 Span과 달랐다.
resolver 의미를 바꾸지 않고 END를 포함한 전체 import 0..20 (`use helpers::hidden\n`)으로 수용 데이터를 정정했다.
기본 테스트 oracle가 지원하지 않던 곱셈은 새 MIR oracle fixture에서 구별 가능한 덧셈으로 대체했다.
실제 Native fixture의 곱셈/702 결과는 유지한다. 기존 테스트를 삭제하거나 완화하지 않았다.

## 검증 결과

- 기본 전체 회귀: **306 PASS / 0 FAIL / 55 ignored**.
- 실제 LLVM 전체: **13 PASS**, Windows COFF/Linux ELF O0/O2 포함.
- Windows Native 전체: **42 PASS / 0 FAIL** (376.79 s), debug/release 포함.
- 총 **361개 고유 테스트**를 실제 실행했다. ignored 55개는 별도 LLVM/Native 실행으로 확인했다.
- fmt·Runtime rustfmt·clippy `-D warnings`·all-features·문서 build/validator·git diff --check PASS.
- 독립 예제 check/debug/release: check 출력 없음·exit 0, Native 두 실행 정확한 24줄 UTF-8/LF·빈 stderr·exit 0.
  안내한 unknown label 명령도 N2201·exit 1로 확인했다. 두 파일 수용 fixture와 별도 정상 2개는 Native tests에서 두 profile로 실행했다.
- HEAD 대비 accepted D01~D05/P01~P16 ledger·승인 문서/grammar와 원본 148개 SHA-256 보존을 확인했다.
- 사용자 `examples/enums.nova`·`examples/try_result.nova` 수정, `.idea/`·`examples/function.nova`는 보존하고 커밋에서 제외했다.

환경: Rust 1.99.0, LLVM 21.1.8, Windows x64 MSVC. MSRV 1.80은 이번 실행에서 별도 검증하지 않았다.
Linux host Native 실행은 후속이며 ELF object 생성과 구분한다.

## 직접 실행과 남은 범위

[독립 예제](../../examples/named_arguments.nova)·[TESTING.md](../../TESTING.md)를 따른다.
check는 출력 없이 exit 0, debug/release는 [수용 fixture의 24줄](named-arguments-proposal-fixtures/README.md)·LF·빈 stderr·exit 0이다.
기본 인수·외부 label 문법·overload·메서드/receiver·사용자 Generic·함수 값·Closure·named constructor·Array·
일반 Move/borrow/Drop·public ABI/FFI와 전체 D11/D16/D25/D30은 후속이다.
