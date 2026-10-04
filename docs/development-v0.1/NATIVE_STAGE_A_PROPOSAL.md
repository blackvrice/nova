# Stage A Native 실행 착수안 — P03

작성일: 2026-10-04. 상태: **Accepted / 2026-10-04 사용자 승인**.
승인 근거: 사용자 답변 “P03 승인하고 Stage A Native 구현 진행”.
[P01](PARSER_STAGE_A_PROPOSAL.md)/[P02](SEMANTICS_STAGE_A_PROPOSAL.md)와 원본 MIR 규칙은 유지한다.
이 문서는 D07/D16/D18/D19/D22/D23/D28의 아래 Stage A 부분만 제안한다. 전체 결정 승인이 아니다.

## Specification Change Proposal

- 관련 문서: NOVA-025/026/093~101/114/116/125/128/136/139/140.
- 현재 사양: Rust compiler, LLVM Adapter 격리, verified MIR 입력, Panic Abort,
  Stage A main/console/object/executable 목표는 확정되어 있다. Int32/Bool/String/Unit 타입과
  print(String) signature 및 source-order/short-circuit은 P02가 승인되어 있다.
- 발견된 문제: arithmetic 실행 실패, 출력 줄바꿈/보간 formatting, executable entry와
  internal representation/ABI, toolchain 범위가 미정이다. LLVM이나 host Rust 정책으로 추측할 수 없다.
- 제안 변경: 아래 Stage A Native 최소 계약을 채택한다.
- 변경 이유: Hello Nova와 현재 Stage A subset을 Native로 실행하고 정상/실패 결과를 검증한다.
- 영향 범위: nova-codegen/nova-codegen-llvm/runtime/driver, 실행 파일용 entry 검사,
  LLVM object/link 및 Native harness. Source/Lexer/Parser/기존 fragment 의미 검사는 유지한다.
- Backward Compatibility: syntax와 frontend-pass 범위는 유지한다. Executable 생성 시에만 main을 요구한다.
  미정이었던 runtime 실패와 출력 바이트가 정의된다. 일반 ABI/FFI/ownership 정책을 확정하지 않는다.
- 대안: 이번에는 backend interface/toolchain 오류 처리까지만 구현하고 실제 Native 연산을 미룬다.

## 제안하는 실행 계약

1. **정수**: Int32 unary minus와 `+ - * / %`는 모든 profile에서 checked다.
   overflow, division/remainder by zero, `MIN / -1` 및 `MIN % -1`은 Abort panic이다.
   정상 division은 0 방향 truncate, remainder는 dividend 부호다. Unary plus는 값 유지.
   Bool/Int32 비교와 logical short circuit은 P02 의미를 유지한다.
   이 단계는 constant folding/const semantics/widening/float를 추가하지 않는다.
2. **출력**: `print(string)`은 UTF-8 bytes 뒤에 LF 한 바이트를 출력하고 flush한다.
   Windows에서도 byte 결과는 LF이며 embedded NUL을 문자열 끝으로 보지 않는다.
   I/O 실패는 best-effort stderr 메시지와 call Span을 남기고 Abort한다.
3. **보간**: Int32는 locale-independent decimal(음수는 `-`, plus/공백/구분자 없음),
   Bool은 `true`/`false`, String은 원본 UTF-8 bytes다. 평가 순서는 기존 MIR 순서를 유지한다.
   String allocation/length overflow/OOM은 Abort이며 부수 효과를 재평가하지 않는다.
4. **실패**: unwind 없음. Runtime panic은 best-effort stderr에 이유와 FileId/byte Span을 기록하고
   프로세스를 Abort한다. Native 실패는 nonzero 종료로 검사하며 OS별 정확한 abort exit/signal은 고정하지 않는다.
   call stack trace hook/full symbolization과 recoverable I/O API는 후속이다.
5. **Entry**: executable은 파일 범위의 사용자 함수 `main() -> Unit` 하나를 요구한다.
   인수 있는 main/non-Unit main/missing main은 실행 파일 생성 전에 오류로 거부한다.
   C-compatible startup wrapper가 Nova main을 호출하며 정상 종료는 exit 0이다.
   frontend check와 library/object-only fragment에는 main을 강제하지 않는다.

## Backend/Runtime 구현 경계 제안

- verified immutable CodegenUnit 뒤에 `CodegenBackend`/TargetSpec/CodegenOptions/ObjectArtifact를 둔다.
  LLVM 타입, IR 문자열 구성과 LLVM process invocation은 Adapter에 격리한다.
- 최초 Native host는 Windows x86_64 MSVC. Linux x86_64 GNU는 명시적 target의 IR/object 시험 후보이며
  실제 link/run 검증 전에는 지원 완료라고 하지 않는다. AArch64/cross run은 미지원 오류다.
- LLVM/Clang 21.1.8을 첫 Adapter 검증 pin으로 제안한다. 외부 tool 경로를 명시할 수 있으며
  version/target mismatch, missing tool, verify/object/link 실패를 분리한다.
  compiler build/fmt/tests 자체는 LLVM 설치 없이 가능해야 한다. LLVM-linked Rust binding은 사용하지 않고
  versioned textual IR을 LLVM tool로 verify/object 생성하는 초기 Adapter를 제안한다.
- Internal scalar lowering: Int32 i32, Bool i1, Unit은 값이 필요할 때 empty aggregate, return은 void.
  Stage A String은 immutable UTF-8 pointer+u64 byte length, x64에서 두 필드 representation이다.
  compiler-generated internal call만 사용하고 runtime C boundary는 scalar/pointer/out-pointer로 전달한다.
  Source function symbol은 단일 CodegenUnit 안의 deterministic private ID symbol이며 public Nova ABI가 아니다.
- Stage A dynamic String 저장소는 Runtime의 실행 단위 arena에 보존하고 정상 entry 종료 시 해제한다.
  이는 현재 destructor/observable Drop가 없는 subset의 임시 구현이다. Ownership checker/Move/Drop 의미를
  선행 구현하거나 memory usage 최적화를 약속하지 않는다. 영구 public String ABI를 승인하지 않는다.
- 출력/format/panic/allocation Runtime은 Rust로 구현하고 unsafe boundary는 생성된 pointer/length 계약에 한정한다.
  언어 `foreign`/user C FFI는 이번에 추가하지 않는다.
- Optimizer는 이번 단계에서 O0를 기본으로 한다. O2 검증은 checked 실패/평가 순서가 보존됨을 확인하기 위한
  backend 시험이며 advanced Nova optimizer나 SSA 변환 구현이 아니다.
- 최초 driver는 단일 .nova의 check/build/run. check는 산출물 없이 기존 frontend/MIR를 검사한다.
  build/run은 entry 검사와 성공적인 verify/object/link 뒤에만 실행한다. Human 진단은 stderr,
  child stdout/stderr는 그대로 전달하고 정상 child exit를 전달한다. OS abort를 성공으로 바꾸지 않는다.
  CLI/manifest/user/toolchain/ICE exit 분류는 원본 NOVA-125를 따르고 전체 D22/JSON/package는 후속이다.
  출력은 명시 경로 또는 target/nova-stage-a의 고유 작업 디렉터리를 사용하며 source 파일을 덮어쓰지 않는다.

## 검증과 단계 완료

- CodegenUnit 오류 입력 차단, Target/options/tool invocation 오류·결정성 tests.
- LLVM IR verify, object magic/target, private call signature, SourceInfo map와 deterministic IR snapshot.
- Hello stdout `Hello, Nova\n`와 exit 0, 함수/branch/short-circuit/call order/return/Unicode/NUL/보간 Native tests.
- Int32 경계 정상 산술과 모든 checked 실패를 별도 child에서 검사하고 O0/O2를 대조한다.
- malformed source/missing or wrong main은 object/link/run 전에 거부한다.
- fmt/clippy/workspace tests/all-features check, 문서 validator와 실제 host toolchain evidence를 기록한다.
  LLVM tool이 없는 환경에서 기본 tests 성공은 LLVM verify/Native E2E 증거가 아니다.

## 근거와 승인 상태

- [원본 Backend 결정](../09_Backend_Runtime/NOVA-093_Backend_선택_LLVM_연동_결정서.md).
- [LLVM 21.1.8 공식 배포](https://github.com/llvm/llvm-project/releases/tag/llvmorg-21.1.8).
- [LLVM 21.1 Language Reference](https://releases.llvm.org/21.1.0/docs/LangRef.html).

위 Stage A subset은 사용자 승인으로 구현에 적용한다. 나머지 D07~D28 전체 정책은 Draft다.
승인과 LLVM 설치/호스트 검증 상태는 별도로 기록한다. 현재 시스템에서는 MSVC linker는 확인했으며
착수 시 Clang/LLVM IR verify 도구는 발견하지 못했으나 후속 구현 중 workspace에 준비했다.
현재 도구·사용·실제 검증 상태는 [Native 구현 기록](NATIVE_IMPLEMENTATION.md)을 따른다.
