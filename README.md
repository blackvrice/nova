# Nova 0.1 compiler

개발 전에 읽을 [전체 개발 문서 보완팩](docs/development-v0.1/README.md)을 작성했습니다.
148개 주제별 문서와 구체 EBNF, 30건 결정 초안, API/schema, 수용 테스트 계획을 포함합니다.
D01~D05 Lexer, P01 Parser, P02 이름·타입, P03 Native, P04 가변 변수·반복문, P05 지역 const, P06 전역 const, P07 고정 폭 정수·승격, P08 char, P09 float, P10 숫자 cast, P11 Module 최소 계약은 Accepted이며,
나머지 상세는 Draft입니다.

Nova 컴파일러의 Stage A와 Stage B 제어 흐름·지역/전역 const·고정 폭 정수·char·float·cast·Module 구현입니다. 언어 사양은 `docs/`의 원본
Documentation Pack과 사용자가 제공한 Canonical Decisions를 따릅니다.
원본 사양 파일은 보존하며, 사용자 승인된 상세 계약을 별도 문서로 추가했습니다.

## 현재 구현

- `nova-core-ids`: 데이터베이스 내에서 안정적인 정수 `FileId`.
- `nova-source`: append-only UTF-8 `SourceDatabase`, 반열린 바이트 `Span`,
  지연 계산되는 `LineIndex`, Unicode scalar 기준의 1-based 표시 위치.
- `nova-diagnostics`: `DiagnosticCode`, Severity, Primary/Secondary Label,
  Note, Suggestion 및 Plain/ANSI/JSON/Snapshot Renderer.
- `nova-syntax`: 공식/승인 키워드, token, END origin 등 단계 독립 자료형.
- `nova-lexer`: UTF-8 lossless scanning, Unicode XID, Literal/Escape, 중첩 주석/보간,
  오류 진단, 결정적 token dump, Stage A END 정규화.
- `nova-ast`: byte Span과 source-order 자식 ID를 보존하는 Arena, AstNodeId,
  Error Node, 반복형 Visitor와 결정적 dump. 의미 TypeId/DefId는 포함하지 않습니다.
- `nova-parser`: 승인된 Stage A 함수·let·return·if/else, typed parameter, positional call,
  기본 표현식·문자열 보간, P04 var·대입·while·break/continue, P05 함수 내부/P06 전역 const, P08 문자/P09 실수 리터럴, P10 cast, P11 import·visibility. 구문 복구 포함.
- `nova-hir`: AST와 분리된 flat HIR, SymbolId/SourceOrigin, Primitive/Unit 정규화와 String/char decode, 파일별 root/ownership/ImportEdge bundle.
- `nova-resolve`: ScopeTree/DefId/DefinitionRegistry/ResolutionMap, 함수·전역 const forward reference, 원 DefId import alias·visibility, 지역 Scope·가변성.
- `nova-types`: TypeInterner, 8종 정수·Float32/64·Bool/Char/String/Unit, IntegerValue/FloatValue·lossless conversion와 ConstValue.
- `nova-typecheck`: expected type/TypeTable, Literal 범위·인수·return·Bool 조건·불변 대입·loop jump,
  P05/P06 const checked 평가와 10,000-node budget·ConstEvaluation table, cross-file 전역 dependency/SCC 순환 진단, 기대/peer literal 문맥과 승격 metadata, char scalar 비교.
- `nova-mir`: 비SSA Place/Operand/Rvalue, BasicBlock CFG, source-order Call Terminator,
  명시적인 Widen·short-circuit/if/return/while·jump/const Lowering과 타입·초기화·순환 CFG 검증.
- `nova-codegen`: immutable verified CodegenUnit, Backend trait/Target/Options/Artifact/error 경계.
- `nova-codegen-llvm`: LLVM 21.1.8 textual IR, checked arithmetic/CFG, verify와 COFF/ELF Object 생성.
- `nova-driver`: root-relative 파일 discovery, exact spelling·canonical root·physical identity, 1,024-module 제한과 결정적 FileId.
- `nova-cli`: `.nova` entry 및 reachable Module bundle의 check/build/run과 Windows x64 MSVC Rust Runtime 링크·실행.

의존 방향은 `nova-diagnostics → nova-source → nova-core-ids`입니다.
Lexer의 의존 방향은 `nova-lexer → nova-syntax/nova-source/nova-diagnostics`입니다.
Parser의 production 의존 방향은 `nova-parser → nova-ast/nova-syntax/nova-source/nova-diagnostics`입니다.
Lexer 연동은 Parser 테스트의 dev dependency로만 사용합니다.
HIR은 AST/Source/Syntax에, Resolver는 HIR에, TypeChecker는 HIR/Resolver/Types에 의존합니다.
Driver가 Source/AST/HIR discovery를 제공하며 Core는 파일 I/O에 의존하지 않습니다.
HIR→TypeChecker, Types→LLVM 의존은 없습니다.
MIR은 HIR/Resolve/Types/TypeCheck에 의존하고 LLVM을 포함하지 않습니다.
Unicode 18.0.0의 XID 데이터는 고정된 unicode-ident 1.0.26을 vendor에 포함했습니다.
Rust 1.80 이상이 필요하며 `cargo test --workspace --offline`으로 빌드할 수 있습니다.

```powershell
cargo fmt --check
cargo clippy --workspace --all-targets -- -D warnings
cargo test --workspace
cargo check --workspace --all-features
```

## 구현 경계

`FileId`는 하나의 데이터베이스 안에서 파일 추가 순서에 따라 배정됩니다.
영구 캐시 ID나 경로/content hash가 아닙니다. 소스는 추가 후 수정할 수 없으므로
기존 ID, Span, 지연 LineIndex는 그대로 유지됩니다.

`Span::new`는 범위 순서를 검사하고 `SourceDatabase::slice`와 진단 Renderer는
파일 존재 여부, 범위, UTF-8 경계를 검사합니다. EOF의 빈 Span은 유효합니다.
잘못된 UTF-8 입력은 교체 문자 없이 `SourceError::InvalidUtf8`로 반환합니다.
Driver는 입력/읽기/UTF-8/root 오류를 N8001 source event 또는 유효한 import Span으로 보고합니다.

LineIndex는 LF, CRLF, CR을 처리하고 원본 바이트를 보존합니다. 이는 소스 표시의
구현 선택이며 Lexer의 문장 종료/END 의미를 정의하지 않습니다.
표시 열은 NOVA-078에 따라 Unicode scalar 수이며 터미널 셀 폭과 다를 수 있습니다.
Snapshot은 ANSI 없이 Plain 형식을 사용하고 경로 구분자를 `/`로 표시합니다.
진단에는 바이트 범위와 원본 위치가 함께 남습니다.

NOVA-070의 `ReadOnlySpan`, `splitAt`, pointer+length ABI 및 region 계약은
NOVA-002와 사용자 Stage 순서에 따라 언어 수준 Stage C 작업으로 남깁니다.
이번 컴파일러 `Span`은 해당 언어 타입과 별개인 소스 위치 자료형입니다.

## 다음 단계

1. Lexer/Token/Stage A END 정규화 완료: [승인 기준](docs/development-v0.1/ACCEPTED_LEXER.md).
2. Stage A Parser/AST 완료: [P01 승인 범위와 전용 EBNF](docs/development-v0.1/PARSER_STAGE_A_PROPOSAL.md).
3. HIR·최소 이름/타입 검사와 frontend pass/fail harness 완료: [P02 승인 범위](docs/development-v0.1/SEMANTICS_STAGE_A_PROPOSAL.md).
4. MIR lowering/validation 완료: [구현·검증 경계](docs/development-v0.1/MIR_IMPLEMENTATION.md).
5. [P03](docs/development-v0.1/NATIVE_STAGE_A_PROPOSAL.md) 승인과 Windows x64 Stage A Native/Hello E2E 완료:
   [명령·지원·검증 기록](docs/development-v0.1/NATIVE_IMPLEMENTATION.md).
6. [P04 가변 변수·반복문](docs/development-v0.1/CONTROL_STAGE_B_PROPOSAL.md)과 Windows Native 검증 완료:
   [구현·검증 기록](docs/development-v0.1/CONTROL_IMPLEMENTATION.md).
7. [P05 함수 내부 const](docs/development-v0.1/CONST_STAGE_B_PROPOSAL.md)와 Windows Native 검증 완료:
   [구현·검증 기록](docs/development-v0.1/CONST_IMPLEMENTATION.md).
8. [P06 단일 파일 전역 const](docs/development-v0.1/GLOBAL_CONST_STAGE_B_PROPOSAL.md)와
   [전용 EBNF](docs/development-v0.1/GRAMMAR_STAGE_B_GLOBAL_CONST.ebnf) 구현 완료:
   [구현·검증 기록](docs/development-v0.1/GLOBAL_CONST_IMPLEMENTATION.md).
9. [P07 고정 폭 정수 타입·손실 없는 승격](docs/development-v0.1/INTEGER_STAGE_B_PROPOSAL.md) 구현 완료:
   [타입·const·MIR·Native 검증 기록](docs/development-v0.1/INTEGER_IMPLEMENTATION.md), [예제](examples/integers.nova).
10. [P08 char·scalar 비교·UTF-8 보간](docs/development-v0.1/CHAR_STAGE_B_PROPOSAL.md)과
    [전용 EBNF](docs/development-v0.1/GRAMMAR_STAGE_B_CHAR.ebnf) 구현 완료:
    [구현·검증 기록](docs/development-v0.1/CHAR_IMPLEMENTATION.md), [예제](examples/characters.nova).
11. [P09 float·IEEE 결과·숫자 승격·보간](docs/development-v0.1/FLOAT_STAGE_B_PROPOSAL.md)과
    [전용 EBNF](docs/development-v0.1/GRAMMAR_STAGE_B_FLOAT.ebnf) 구현 완료:
    [구현·검증 기록](docs/development-v0.1/FLOAT_IMPLEMENTATION.md), [예제](examples/floats.nova).
12. [P10 명시적 숫자 cast](docs/development-v0.1/CAST_STAGE_B_PROPOSAL.md)와
    [전용 EBNF](docs/development-v0.1/GRAMMAR_STAGE_B_CAST.ebnf) 구현 완료:
    [checked 변환·const·Native 검증 기록](docs/development-v0.1/CAST_IMPLEMENTATION.md), [예제](examples/casts.nova).
13. [P11 Module·다중 파일 최소 계약](docs/development-v0.1/MODULE_STAGE_B_PROPOSAL.md)과
    [전용 EBNF](docs/development-v0.1/GRAMMAR_STAGE_B_MODULE.ebnf) 구현 완료:
    [구현·검증 기록](docs/development-v0.1/MODULE_IMPLEMENTATION.md), [두 파일 수용 fixture](docs/development-v0.1/module-proposal-fixtures/README.md).
14. [P12 Copy struct 최소 계약](docs/development-v0.1/STRUCT_STAGE_B_PROPOSAL.md)·[전용 EBNF](docs/development-v0.1/GRAMMAR_STAGE_B_STRUCT.ebnf)·[수용 fixture](docs/development-v0.1/struct-proposal-fixtures/README.md) 작성: Draft/승인 대기, 미구현.
15. 후속 Move field·init/Drop·Enum/Tuple/Array·float remainder/math API·Package·Linux Native host 검증.

제공된 NOVA-014는 일반 요구사항을 담고 있지만 실제 EBNF Production은 없습니다.
Stage A는 별도로 사용자 승인된 `GRAMMAR_STAGE_A.ebnf`를 따릅니다.
P04 확장은 `GRAMMAR_STAGE_B_CONTROL.ebnf`를 따릅니다.
P05 함수 내부 const 확장은 `GRAMMAR_STAGE_B_CONST.ebnf`를 따릅니다.
P06 단일 파일 전역 const 확장은 `GRAMMAR_STAGE_B_GLOBAL_CONST.ebnf`를 따릅니다.
P08 문자 리터럴 확장은 `GRAMMAR_STAGE_B_CHAR.ebnf`를 따릅니다.
P09 실수 리터럴 확장은 `GRAMMAR_STAGE_B_FLOAT.ebnf`를 따릅니다.
P10 명시적 숫자 변환 확장은 `GRAMMAR_STAGE_B_CAST.ebnf`를 따릅니다.
P11 Module/import/visibility 확장은 `GRAMMAR_STAGE_B_MODULE.ebnf`를 따릅니다.
전체 `GRAMMAR.ebnf`의 미래 Stage 구문은 여전히 Draft입니다.

현재 Windows x64 Stage A/P04~P11 CLI/LLVM/Runtime을 제공합니다. LLVM 21.1.8과 Rust/MSVC가 필요합니다.
기본 Cargo tests에는 실제 LLVM/Native tests가 ignored이며 별도 명령으로 실행합니다. 기반 테스트는 Rust
기반 계층의 UTF-8, 범위 오류, EOF, 혼합 줄바꿈, 대형 파일, 진단 Snapshot,
JSON escaping 및 Suggestion 위치 검증을 다룹니다.
Lexer lexical pass/fail fixture와 source reconstruction/중첩 mode/END/회귀 테스트도 포함합니다.
lexical pass는 프로그램 전체 타입 검사나 실행 성공을 뜻하지 않습니다.
Parser-pass 역시 구문 수용만 뜻합니다. 이름·타입·실행 결과는 보장하지 않습니다.
frontend-pass는 승인된 Stage A/P04~P11 이름·타입·const 검사 성공을 뜻하며 Native 실행 성공이 아닙니다.
Parser 입력은 normalized tokens여야 하며, 잘못된 API 입력은 ParseInputError로 반환합니다.
잘못된 Nova 구문은 N1101~N1103와 recovery AST로 반환합니다. 호출자는 Lexer와 Parser
오류를 모두 확인한 후 lowering해야 합니다. 기본 nesting limit은 128이며 1~128로 설정 가능합니다.
P04 loop 내부의 nesting limit 초과는 N8901, 가변 지역 var 외 대상 대입은 N3004입니다.
P05 const의 허용성/checked 산술 실패는 N3201, initializer 예산 초과는 N3202입니다.
P06 전역 const도 같은 예산을 따르고 정적 순환은 N3202입니다. 실행을 생략하는 RHS도 dependency graph에 포함합니다.
사용자 함수 print의 builtin shadow는 허용하며 전역 const print는 N2002로 거부합니다.

의미 분석 API/구현 경계는 [P02 구현 계약](docs/development-v0.1/SEMANTICS_IMPLEMENTATION.md)에 있습니다.
Stage A `print`는 `print(string) -> Unit`이며 정수/float/Bool/Char 출력은 보간 문자열을 사용합니다.
P09 float는 IEEE 결과·canonical NaN·±0·점진적 underflow와 최단 fixed decimal 출력을 보존합니다.
float 상수 평가의 host 환경 제어는 현재 x86_64를 지원합니다.
P10 숫자 `as`는 원래 operand 타입을 유지하고 변환 값의 범위를 검사합니다.
실패는 const에서 N3201, Native에서 전체 cast Span의 `numeric cast out of range` Abort입니다.
main 존재/signature는 fragment 검사에서 강제하지 않으며 Native entry 단계에서 검사합니다.
MIR은 arithmetic과 interpolation을 abstract 연산으로 보존하고 LLVM/Runtime이 P03 정책으로 구현합니다.
MIR 검증 성공은 Native 실행 성공을 뜻하지 않습니다. 실제 Native tests는 별도로 실행합니다.

## Hello Nova 실행 (Windows x64)

```powershell
$env:NOVA_CLANG = (Resolve-Path 'target/toolchains/llvm-21.1.8/bin/clang.exe').Path # 또는 설치된 clang.exe
cargo run -p nova-cli -- check examples/hello.nova
cargo run -p nova-cli -- run examples/hello.nova
cargo run -p nova-cli -- build examples/hello.nova -o hello.exe
cargo run -p nova-cli -- run examples/hello.nova --profile release
cargo run -p nova-cli -- run examples/loops.nova
cargo run -p nova-cli -- run examples/loops.nova --profile release
cargo run -p nova-cli -- run examples/constants.nova
cargo run -p nova-cli -- run examples/constants.nova --profile release
cargo run -p nova-cli -- run examples/global_constants.nova
cargo run -p nova-cli -- run examples/global_constants.nova --profile release
cargo run -p nova-cli -- run examples/characters.nova
cargo run -p nova-cli -- run examples/characters.nova --profile release
cargo run -p nova-cli -- run examples/floats.nova
cargo run -p nova-cli -- run examples/floats.nova --profile release
```

출력은 `Hello, Nova` 뒤 LF이며 정상 종료는 0입니다. `-o`는 기존 파일을 덮어쓰지 않습니다.
GitHub clone에는 LLVM binary가 포함되지 않습니다. 설치·시험 방법은 Native 구현 기록을 확인하세요.
