# P16 Copy try·Result 오류 전파 구현·검증 기록

승인일/구현일: 2026-10-07. 사용자 “P16 승인하고 Copy try 구현 진행” 답변으로
[최소 계약](TRY_STAGE_B_PROPOSAL.md)과 [51-production EBNF](GRAMMAR_STAGE_B_TRY.ebnf)를 승인했다.
원본 148개와 기존 D01~D05/P01~P15의 승인 의미를 보존한다.

## 구현

- Parser/AST/HIR에 prefix try와 한 operand·전체 Span·3-byte keyword Span·원 AstId를 연결했다.
  call/as/projection이 먼저, arithmetic/logical이 나중에 결합한다. 새 Lexer token이나 END 규칙은 없다.
  외부 AST shape·keyword spelling·식별자 경계·child order를 검사하며 UTF-8 truncation에서 recovery가 가능하다.
- TypeChecker는 바깥 기대 타입을 operand에 전달하지 않는다. intrinsic Result family만 허용하고
  source/destination의 E를 정규화된 Type identity로 비교한다. 추출한 T의 기존 scalar widening은 유지한다.
  사용자 Enum lookalike·Option·primitive는 N2101, 잘못된 enclosing return은 N3002,
  불완전한 Result constructor 문맥은 N2103이다. ErrorType 파생 진단은 억제한다.
- const declaration 문맥을 전달해 전역/지역·생략된 논리 RHS의 try를 keyword N3201로 거부한다.
  const 선언 secondary를 유지하고 N3002를 추가하지 않는다. 정적 const cycle·기존 평가 예산은 유지한다.
  try는 Success 경로의 fallthrough이므로 known Error const도 mandatory return을 대신하지 않는다.
- MIR은 operand를 한 번 Copy snapshot하고 명시적 `TerminatorKind::Try`로 두 active tag 경로를 만든다.
  Success payload를 독립 temporary에 복사한다. Error payload는 destination Result::Error로 구성해 즉시 Return한다.
  기존 variant payload/constructor·private out-pointer ABI·String 실행 단위 arena를 재사용한다.
- private try certificate는 source/destination family·callee·SourceInfo/keyword·snapshot·Success read·Error block을 보존한다.
  snapshot과 추출 Success temporary의 static write 수가 각각 하나임을 검사한다. loop의 반복 실행은 허용한다.
  별도 control certificate가 함수 entry와 terminator 전체를 보존해 operand call 중복·분기 retarget·Error return 우회를 거부한다.
  현재 try 함수의 CFG/call/return 변환은 certificate와 일치해야 한다. 향후 optimizer는 이 검증 경계를 보존해야 한다.
  기존 must-initialized와 active-payload proof를 두 try edge에도 적용한다.
- LLVM Adapter는 검증한 Result tag를 switch하고 기존 typed payload view/Copy/return을 방출한다.
  Error는 Abort가 아니며 이후 sibling/statement를 실행하지 않는다. Runtime 소스/API 변경은 없다.

## 테스트

P16 추가 **13개**: 기본 10개(Parser 1·HIR 1·TypeChecker 3·MIR 2·Driver 1·CLI 1·LLVM text 1),
실제 LLVM opt-in 1·Windows Native opt-in 2.

- [부정 18사례](try-proposal-fixtures/expected.json)의 code·정확한 UTF-8 byte Span·const secondary·cascade 금지를 실제 검사했다.
- exact E/alias/nullable/nominal·constructor 문맥 격리·raw T/coercion·nested try/return try·all-arm return·const cycle를 검사했다.
- Parser 결합·cast/projection·bare return END·모든 UTF-8 character-boundary prefix·누락 operand·기존 중첩 제한을 검사했다.
- MIR 변조 13종: Success/Error edge 교환·Source 위조·Error return 우회·같은 타입의 다른 Error 값·variant retarget·
  snapshot 값 교체/overwrite·잘못된 active payload·다른 receiver·operand call 중복·entry 우회·try dispatch 제거·Success 값 overwrite.
- 256 KiB host stack에서 flat try 512개의 분석·CFG·검증·해제를 검사했다. 새 try language cap은 없다.
- CLI check/build/run이 의미 오류에서 Native tools를 호출하지 않고 기존 output 파일을 보존함을 검사했다.
- Native O0/O2: 두 파일 alias·19줄 effect order·nested try·Unit/ZST·snapshot·short-circuit·while continue,
  숫자 10종/Bool/Char의 mixed Copy payload, private factory와 private struct Error payload,
  String temporary의 early return 이후 생존, while condition의 Error return·break, nested return try를 실행했다.
  Success 이후 checked overflow의 정확한 UTF-8 byte Span도 두 profile에서 검사했다.
- 실제 LLVM은 Windows COFF/Linux ELF object를 O0/O2에서 생성했다. ELF object 생성은 Linux host 실행 증거가 아니다.

## 기존 계약 보존과 정정

- P16 초안의 중첩 초과 code 표기는 기존 Parser 정책에 맞게 정정했다: P01 N1102, loop 내부 P04 N8901.
  N1103은 기존 비교 연산 chain 진단이다. Parser의 실제 중첩 정책은 변경하지 않았다.
- P15 `try_unsupported.nova`의 과거 Parser N1102는 승인된 P16이 대체한다.
  현재는 격리된 operand의 Result::Success constructor 전체에 N2103을 검증하며 P15의 나머지 20사례는 유지한다.
- Native 추가 fixture가 예약어 view/loop를 함수 이름으로 사용한 오류를 describe/tryLoop로 정정했다.
  Lexer keyword 계약은 변경하지 않았다.

## 검증 결과

- 기본 전체 회귀: **297 PASS / 0 FAIL / 52 ignored**.
- 실제 LLVM 전체: **12 PASS**, COFF/ELF O0/O2 생성 포함.
- Windows Native 전체: **40 PASS** (476.80 s), debug/release 실행 포함.
- 총 **349개 고유 테스트**를 실제 실행했다. ignored 52개는 위 LLVM/Native 실행으로 별도 확인했다.
- 전체 Native 실행 중 마지막 검토에서 Success temporary overwrite gate를 보강했다.
  그 최종 소스로 기본 297개·clippy·all-features와 P16 실제 LLVM 1개·Native 2개를 다시 실행해 통과했다.
  기존 P01~P15 Native/LLVM 경로에는 이 추가 try gate가 적용되지 않는다.
- 독립 `examples/try_result.nova`와 두 파일 `main.nova`의 check/debug/release를 직접 실행했다.
  check는 출력 없이 exit 0, Native 4회는 정확한 19줄 LF·빈 stderr·exit 0이었다.
  안내한 `error_width.nova` check도 N2101·정확한 try keyword byte Span·exit 1을 확인했다.
- fmt·Runtime rustfmt·clippy `-D warnings`·전체 feature check·문서 build/validator·git diff --check PASS.
- 기존 accepted P01~P15와 원본 148개 SHA-256을 HEAD와 비교해 보존했음을 확인했다.
  사용자의 `examples/enums.nova` 수정과 `.idea/`·`examples/function.nova`는 보존/커밋 제외했다.

환경: Rust 1.99.0, LLVM 21.1.8, Windows x64 MSVC 14.44.35207.

## 직접 실행

[독립 예제](../../examples/try_result.nova)와 [TESTING.md](../../TESTING.md)를 따른다.
독립/두 파일 예제의 check는 출력 없이 exit 0이다. Native debug/release는 [fixture의 19줄](try-proposal-fixtures/README.md),
각 LF·stderr 없음·exit 0이다.

```powershell
cargo fmt --check
rustfmt --check --edition 2021 crates/nova-cli/runtime/stage_a.rs
cargo clippy --workspace --all-targets --offline -- -D warnings
cargo test --workspace --offline
cargo check --workspace --all-features --offline
$env:NOVA_CLANG = (Resolve-Path target/toolchains/llvm-21.1.8/bin/clang.exe).Path
cargo test -p nova-codegen-llvm --test emission --offline -- --ignored --test-threads=1
cargo test -p nova-cli --test native --offline -- --ignored --test-threads=1
cargo run -p nova-cli --offline -- check examples/try_result.nova
cargo run -p nova-cli --offline -- run examples/try_result.nova --profile debug
cargo run -p nova-cli --offline -- run examples/try_result.nova --profile release
node tools/docs/build-pack.mjs
node tools/docs/validate-pack.mjs
```

## 후속 경계

Option try·error conversion·try block/catch/exception·String/Move payload·일반 borrow/Drop,
Array·사용자 Generic·메서드 API·public ABI/FFI·Result main은 제외한다.
전체 D08/D09/D10/D12/D16/D23/D25 승인이 아니다.
Rust 1.80 MSRV 실행과 Linux Native host 실행은 별도 검증하지 않았다.
