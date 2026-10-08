# Nova 0.1 compiler

개발 전에 읽을 [전체 개발 문서 보완팩](docs/development-v0.1/README.md)을 작성했습니다.
148개 주제별 문서와 구체 EBNF, 30건 결정 초안, API/schema, 수용 테스트 계획을 포함합니다.
D01~D05 Lexer, P01 Parser, P02 이름·타입, P03 Native, P04 가변 변수·반복문, P05 지역 const, P06 전역 const, P07 고정 폭 정수·승격, P08 char, P09 float, P10 숫자 cast, P11 Module, P12 Copy struct, P13 Copy Tuple, P14 Copy Enum·match, P15 Copy Option·Result, P16 Copy try, P17 함수 이름 인수, P18 상수 표현식 기본 인수, P19 loop·정수 범위 for, P20 Copy Option exists, P21 비제네릭 Type Alias, P22 Copy struct Read 메서드, P23 Copy struct 생성자 이름 인수 최소 계약은 Accepted이며,
나머지 상세는 Draft입니다. [P17 함수 이름 인수](docs/development-v0.1/NAMED_ARGUMENTS_STAGE_B_PROPOSAL.md)는 구현·검증 완료입니다. [P18 함수 기본 인수](docs/development-v0.1/DEFAULT_ARGUMENTS_STAGE_B_PROPOSAL.md)와 [P20 Copy Option exists](docs/development-v0.1/EXISTS_STAGE_B_PROPOSAL.md)도 구현·검증 완료입니다. [P21 Type Alias](docs/development-v0.1/ALIAS_STAGE_B_PROPOSAL.md)도 구현·검증 완료입니다. [P22 Read 메서드](docs/development-v0.1/METHOD_STAGE_B_PROPOSAL.md)도 구현·검증 완료입니다. 직접 실행할 [예제·테스트 명령](TESTING.md)을 제공합니다.

Nova 컴파일러의 Stage A와 Stage B 제어 흐름·지역/전역 const·고정 폭 정수·char·float·cast·Module·Copy struct·Copy Tuple·Copy Enum/match 구현입니다. 언어 사양은 `docs/`의 원본
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
  기본 표현식·문자열 보간, P04 var·대입·while·break/continue, P05 함수 내부/P06 전역 const, P08 문자/P09 실수 리터럴, P10 cast, P11 import·visibility, P12 struct·field 경로, P13 Tuple·numeric projection, P14 Enum·statement match, P21 top-level type alias, P22 struct Read instance method. 구문 복구 포함.
- `nova-hir`: AST와 분리된 flat HIR, SymbolId/SourceOrigin, Primitive/Unit 정규화와 String/char decode, 파일별 root/ownership/ImportEdge bundle.
- `nova-resolve`: ScopeTree/DefId/DefinitionRegistry/ResolutionMap, 함수·전역 const forward reference, 원 DefId import alias·visibility, 지역 Scope·가변성, P12 분리 type namespace·원자 import, P22 nominal member registry.
- `nova-types`: TypeInterner, 8종 정수·Float32/64·Bool/Char/String/Unit, IntegerValue/FloatValue·lossless conversion와 ConstValue, nominal StructId/FieldId, structural Tuple shape·nominal Enum/Variant와 checked mixed layout.
- `nova-typecheck`: expected type/TypeTable, Literal 범위·인수·return·Bool 조건·불변 대입·loop jump,
  P05/P06 const checked 평가와 10,000-node budget·ConstEvaluation table, cross-file 전역 dependency/SCC 순환 진단, 기대/peer literal 문맥과 승격 metadata, char scalar 비교, Copy struct/Tuple 생성·projection·가변 경로·const, Copy Enum 생성·binder·coverage 검사, P21 선언 scope의 iterative alias 정규화·순환/한도 검사, P22 immutable receiver·member visibility·named/default offset.
- `nova-mir`: 비SSA Place/Operand/Rvalue, BasicBlock CFG, source-order Call Terminator,
  명시적인 Widen·short-circuit/if/return/while·jump/const Lowering과 타입·초기화·순환 CFG 검증, aggregate 생성/읽기/갱신과 원 ID/path/layout 독립 검증, Enum 생성·tag dispatch·active payload CFG proof, P22 receiver-first snapshot·static callee·method full-body proof.
- `nova-codegen`: immutable verified CodegenUnit, Backend trait/Target/Options/Artifact/error 경계.
- `nova-codegen-llvm`: LLVM 21.1.8 textual IR, checked arithmetic/CFG, verify와 COFF/ELF Object 생성, struct snapshot 인수/out 반환 private ABI.
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
14. [P12 Copy struct 최소 계약](docs/development-v0.1/STRUCT_STAGE_B_PROPOSAL.md)·[전용 EBNF](docs/development-v0.1/GRAMMAR_STAGE_B_STRUCT.ebnf)·[수용 fixture](docs/development-v0.1/struct-proposal-fixtures/README.md) 구현 완료: [검증 기록](docs/development-v0.1/STRUCT_IMPLEMENTATION.md), [예제](examples/structs.nova).
15. [P13 Copy Tuple 최소 계약](docs/development-v0.1/TUPLE_STAGE_B_PROPOSAL.md)·[전용 EBNF](docs/development-v0.1/GRAMMAR_STAGE_B_TUPLE.ebnf) 구현 완료: [검증 기록](docs/development-v0.1/TUPLE_IMPLEMENTATION.md), [예제](examples/tuples.nova).
16. [P14 Copy Enum·statement match 계약](docs/development-v0.1/ENUM_STAGE_B_PROPOSAL.md)·[전용 EBNF](docs/development-v0.1/GRAMMAR_STAGE_B_ENUM.ebnf) 구현 완료: [검증 기록](docs/development-v0.1/ENUM_IMPLEMENTATION.md), [예제](examples/enums.nova).
17. 후속 Move field·init/Drop·Array·float remainder/math API·Package·Linux Native host 검증.

제공된 NOVA-014는 일반 요구사항을 담고 있지만 실제 EBNF Production은 없습니다.
Stage A는 별도로 사용자 승인된 `GRAMMAR_STAGE_A.ebnf`를 따릅니다.
P04 확장은 `GRAMMAR_STAGE_B_CONTROL.ebnf`를 따릅니다.
P05 함수 내부 const 확장은 `GRAMMAR_STAGE_B_CONST.ebnf`를 따릅니다.
P06 단일 파일 전역 const 확장은 `GRAMMAR_STAGE_B_GLOBAL_CONST.ebnf`를 따릅니다.
P08 문자 리터럴 확장은 `GRAMMAR_STAGE_B_CHAR.ebnf`를 따릅니다.
P09 실수 리터럴 확장은 `GRAMMAR_STAGE_B_FLOAT.ebnf`를 따릅니다.
P10 명시적 숫자 변환 확장은 `GRAMMAR_STAGE_B_CAST.ebnf`를 따릅니다.
P11 Module/import/visibility 확장은 `GRAMMAR_STAGE_B_MODULE.ebnf`를 따릅니다.
P12 Copy struct/field 확장은 `GRAMMAR_STAGE_B_STRUCT.ebnf`를 따릅니다.
P13 Tuple/type/numeric field 확장은 `GRAMMAR_STAGE_B_TUPLE.ebnf`를 따릅니다.
P14 Enum/match 확장은 `GRAMMAR_STAGE_B_ENUM.ebnf`를 따릅니다.
전체 `GRAMMAR.ebnf`의 미래 Stage 구문은 여전히 Draft입니다.

현재 Windows x64 Stage A/P04~P14 CLI/LLVM/Runtime을 제공합니다. LLVM 21.1.8과 Rust/MSVC가 필요합니다.
기본 Cargo tests에는 실제 LLVM/Native tests가 ignored이며 별도 명령으로 실행합니다. 기반 테스트는 Rust
기반 계층의 UTF-8, 범위 오류, EOF, 혼합 줄바꿈, 대형 파일, 진단 Snapshot,
JSON escaping 및 Suggestion 위치 검증을 다룹니다.
Lexer lexical pass/fail fixture와 source reconstruction/중첩 mode/END/회귀 테스트도 포함합니다.
lexical pass는 프로그램 전체 타입 검사나 실행 성공을 뜻하지 않습니다.
Parser-pass 역시 구문 수용만 뜻합니다. 이름·타입·실행 결과는 보장하지 않습니다.
frontend-pass는 승인된 Stage A/P04~P14 이름·타입·const 검사 성공을 뜻하며 Native 실행 성공이 아닙니다.
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

[P14 Copy Enum·statement match](docs/development-v0.1/ENUM_STAGE_B_PROPOSAL.md)는 Accepted / 구현 완료다.
[구현 기록](docs/development-v0.1/ENUM_IMPLEMENTATION.md), [standalone 예제](examples/enums.nova),
[두 파일 수용 예제](docs/development-v0.1/enum-proposal-fixtures/README.md)를 제공한다.

[P15 Copy Option·Result·nullable](docs/development-v0.1/OPTION_RESULT_STAGE_B_PROPOSAL.md)는 Accepted / 구현 완료다.
[구현 기록](docs/development-v0.1/OPTION_RESULT_IMPLEMENTATION.md), [standalone 예제](examples/option_result.nova),
[두 파일 수용 예제](docs/development-v0.1/option-result-proposal-fixtures/README.md)와 [직접 실행 명령](TESTING.md)을 제공한다.
Copy payload·T?·문맥/none·match·const·private ABI를 지원한다. try는 P16이며 Array·String/Move payload·사용자 Generic은 후속이다.

[P16 Copy try·Result 오류 전파](docs/development-v0.1/TRY_STAGE_B_PROPOSAL.md)는 Accepted / 구현 완료다.
[구현 기록](docs/development-v0.1/TRY_IMPLEMENTATION.md)·[51-production 문법](docs/development-v0.1/GRAMMAR_STAGE_B_TRY.ebnf)·[두 파일·부정 18사례](docs/development-v0.1/try-proposal-fixtures/README.md)와
[독립 예제](examples/try_result.nova)·[직접 실행 명령](TESTING.md)을 제공한다.

[P17 함수 이름 인수](docs/development-v0.1/NAMED_ARGUMENTS_STAGE_B_PROPOSAL.md)는 Accepted / 구현 완료다.
`f(right:2,left:1)`과 leading positional/named 혼합, import alias·Unicode label·mapped 타입 문맥을 지원한다.
인수는 소스 순서로 한 번씩 평가·snapshot한 뒤 매개변수 순서로 전달한다. try Error는 뒤 인수와 호출을 건너뛴다.
[구현 기록](docs/development-v0.1/NAMED_ARGUMENTS_IMPLEMENTATION.md)·[독립 예제](examples/named_arguments.nova)·[직접 실행 명령](TESTING.md)을 제공한다.
기본 인수는 P18, Read 메서드는 P22, Copy struct 이름 생성자는 P23을 따른다. overload·Enum/sum 이름 생성자는 후속이다.

[P18 상수 표현식 함수 기본 인수](docs/development-v0.1/DEFAULT_ARGUMENTS_STAGE_B_PROPOSAL.md)는 Accepted / 구현 완료다.
`func f(a:int8=1,b:int8=2)`의 생략 인수를 declaration-module scope의 typed 상수로 채운다.
제공 인수를 소스 순서로 평가·snapshot한 뒤 caller에서 생략 기본값을 declaration order로 materialize한다.
[구현 기록](docs/development-v0.1/DEFAULT_ARGUMENTS_IMPLEMENTATION.md)·[독립 예제](examples/default_arguments.nova)·[직접 실행 명령](TESTING.md)을 제공한다.
Copy struct 이름 생성자는 P23을 따른다. runtime/parameter 의존 default·const function·overload·Enum/sum 이름 생성자는 후속이다.

[P19 loop·정수 범위 for](docs/development-v0.1/RANGE_LOOP_STAGE_B_PROPOSAL.md)는 Accepted / 구현·검증 완료다.

[P20 Copy Option postfix exists](docs/development-v0.1/EXISTS_STAGE_B_PROPOSAL.md). Accepted / 구현·검증 완료다.
`value exists`는 intrinsic Option의 Some이면 true, None이면 false다. operand를 한 번 평가하고 const/default에서도 지원한다.
[구현 기록](docs/development-v0.1/EXISTS_IMPLEMENTATION.md)·[독립 예제](examples/exists.nova)·[직접 실행 명령](TESTING.md)을 제공한다.


[P23 Copy struct 생성자 이름 인수](docs/development-v0.1/STRUCT_NAMED_ARGUMENTS_STAGE_B_PROPOSAL.md)는 **Accepted / 구현·검증 완료**다.
`Point(y:2,x:1)`의 field 대응·source-order 평가/snapshot·const/default·private 접근·Source/MIR 검증을 지원한다.
[구현 기록](docs/development-v0.1/STRUCT_NAMED_ARGUMENTS_IMPLEMENTATION.md)·[독립 예제](examples/struct_named_arguments.nova)·[직접 실행 명령](TESTING.md)을 제공한다.
현재 실행 구현은 P23까지다. explicit init·field default·Enum/sum named payload·Array·일반 Move/Drop은 후속이다.

[P24 중첩 Copy 패턴·Tuple match](docs/development-v0.1/NESTED_PATTERN_STAGE_B_PROPOSAL.md)는 **Draft / 승인 대기 / 미구현**이다.
[수용 예제](docs/development-v0.1/nested-pattern-proposal-fixtures/README.md)와 독립 coverage oracle을 준비했다.
현재 문서 검사는 `node tools/docs/validate-pack.mjs`, oracle 검사는 `node tools/tests/nested-pattern-oracle.mjs`로 실행한다.
