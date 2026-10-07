# 문서 동결과 Nova 구현 로드맵

상태 Draft. Stage A~E 순서는 기존 Freeze를 유지한다. 문서 작성은 완료했어도 아래 승인/구현
gate가 자동 충족되지 않는다. 미래 문서를 전부 승인해야 첫 Lexer 작업을 할 수 있다는 뜻은 아니다.

| Gate | 먼저 동결할 결정/자료 | 구현 산출물 | 완료 증거 |
|---|---|---|---|
| 0 기반 | 기존 IDs/Source/Diagnostic, D02/D25 필요한 부분 | 현재 3 crates의 검증/오류 API 정리 | Cargo 4 checks, Unicode/Span/renderer tests |
| A Frontend | D01~D08 최소 부분, D11 call subset, EBNF/END | syntax/lexer/AST/parser/HIR/resolve/types/typecheck | production snapshots와 pass/fail |
| A Native | D16/D18/D19/D22/D23 print/D28 host pin | 최소 MIR/validation/backend/runtime/link CLI | Hello check/run, output/exit, invalid source codegen 차단 |
| B 기본 언어 | D06~D12 type/control/aggregate, D09 const | modules, primitives, aggregates, loops/match/try | T008~T019/T047~T048과 multi-file corpus |
| C 안전성 | D10/D20 predicate core/View API | initialization/Move/NLL/Drop/View | T020~T022/T041와 sanitizer corpus |
| D 추상화 | D11~D15/D23 generic APIs | Class/interface/generic/closure | T023~T025, specialization cache/inference |
| E 제품화 | D16~D26/D28~D30 | package/lock/cache/formatter/C FFI/std/Targets | locked clean builds, C ABI, tooling, release checklist |

## 상호 의존성 처리

Source Array/Option/Result 문법은 B에서 준비하되 Move element-safe execution은 C, 사용자
Generic/Closure 기반 std API 완성은 D에 의존한다. 특정 라이브러리 기능의 납품을 뒤 Stage로
미루고 안전성 검사를 생략하지 않는다. C/D를 Stage A 전에 선행 구현하는 방식은 금지한다.

## 첫 구현 재개 조건

1. Rust frontend toolchain 및 기존 Cargo fmt/clippy/test/check 실행 완료.
2. D01~D05 승인과 Token/Lexer/Stage A END 구현 완료. D07/D08의 Stage A numeric/condition 의미는 승인 필요.
3. Stage A parser fixtures의 source+expected diagnostic Span 구체화.
4. SourceInfo/Diagnostic API에 승인된 변화만 적용, Lexer부터 순서대로 구현.

현재 Source/Diagnostic과 Token/Lexer/END, P01 Stage A AST/Parser를 구현했다.
2026-10-04 사용자 승인 [P01](PARSER_STAGE_A_PROPOSAL.md)은 구문·복구 subset에 적용한다.
2026-10-04 사용자 승인 [P02](SEMANTICS_STAGE_A_PROPOSAL.md)에 따라 HIR lowering,
단일 파일 이름·타입 검사와 frontend pass/fail harness를 구현했다.
원본 NOVA-081~083/091과 P02에 따른 Stage A [MIR lowering/validation](MIR_IMPLEMENTATION.md)도 구현했다.
문서 검증과 compiler tests, Native 실행 완료는 구분한다. 다음은 Codegen Interface/LLVM Adapter다.
Native 실행 전 arithmetic runtime, panic/print, entry/CLI/LLVM host 계약의 필요한 부분을 별도 동결한다.
후속 [P03](NATIVE_STAGE_A_PROPOSAL.md) 승인으로 최소 계약을 동결하고 Windows x64
[Native/CLI/Runtime](NATIVE_IMPLEMENTATION.md)과 Hello E2E를 구현·검증했다.
다음은 Linux Native host 검증 및 Stage B 착수 범위 검토다.

2026-10-04 Stage B 첫 범위로 [P04](CONTROL_STAGE_B_PROPOSAL.md)의 var/direct-name 대입,
while/break/continue를 사용자 진행 요청에 따라 구현했다. 전용 EBNF, 진단·MIR와
Windows x64 O0/O2 증거는 [P04 구현 기록](CONTROL_IMPLEMENTATION.md)에 있다.
이 범위 완료는 Stage B 전체 완료가 아니다. 다음은 const/Primitive·aggregate/module 상세의 별도 동결이다.
현재 로컬에는 Linux 실행 환경이 없어 Linux Native 검증은 후속이다.

다음 [P05 함수 내부 const](CONST_STAGE_B_PROPOSAL.md)는 사용자 “P05 승인하고 const 구현 진행”
답변에 따라 구현했다. 현재 값 타입의 제한된 상수 표현식·checked 평가·node budget과
전용 EBNF를 적용했고 [구현 기록](CONST_IMPLEMENTATION.md)에 Windows O0/O2 증거를 남겼다.
다음은 전역 상수·Primitive 확장·aggregate/module의 상세 범위 검토다. 전체 Stage B 동결과는 별도다.

다음 최소 범위를 [P06 단일 파일 전역 const·forward dependency/cycle](GLOBAL_CONST_STAGE_B_PROPOSAL.md)로
구체화했고 사용자 “P06 승인하고 전역 const 구현 진행” 답변으로 Accepted다.
P02의 기존 함수 print shadow를 보존하고 전역 const print만 N2002로 거부하는 정정도 승인했다.
[전용 EBNF](GRAMMAR_STAGE_B_GLOBAL_CONST.ebnf)와 root 선언 수집·iterative dependency/SCC 평가,
MIR Constant lowering을 구현했다. [구현·검증 기록](GLOBAL_CONST_IMPLEMENTATION.md)에 실제 Native 증거를 남겼다.
다음은 Primitive 확장·숫자 승격·aggregate/module의 최소 상세 범위 검토다. const function도 별도 후속이다.

다음 최소 범위를 [P07 고정 폭 정수·lossless 승격](INTEGER_STAGE_B_PROPOSAL.md)으로 구체화했다.
8종 정수와 alias·literal 문맥·checked 산술·const·MIR 변환·보간/private Runtime 상세와 수용 계획은
사용자 “P07 승인하고 정수 타입·승격 구현 진행” 답변으로 Accepted다.
타입 검사·const·MIR·LLVM·Runtime과 Windows O0/O2 검증은 [P07 구현 기록](INTEGER_IMPLEMENTATION.md)에 있다.
P07은 기존 P06 grammar를 사용하며 float/cast·aggregate/module은 별도 후속으로 남긴다.

[P08 char·scalar 비교·UTF-8 보간](CHAR_STAGE_B_PROPOSAL.md)은
사용자 “P08 승인하고 char 구현 진행” 답변으로 Accepted다.
D04의 기존 char token/escape를 AST·HIR·타입·const·MIR·Native로 연결했다.
[전용 EBNF](GRAMMAR_STAGE_B_CHAR.ebnf)는 P06 primary에 CHAR만 추가한다.
[구현·검증 기록](CHAR_IMPLEMENTATION.md)에 Windows O0/O2와 Runtime scalar 경계 검증을 기록했다.
float/숫자 cast 최소 계약은 아래 P09/P10을 따르고 aggregate/module은 별도 후속이다.

다음 Primitive 최소 범위 [P09 float·IEEE 결과·숫자 승격·보간](FLOAT_STAGE_B_PROPOSAL.md)은
사용자 “P09 승인하고 float 구현 진행”으로 Accepted다. D04 Float token을 AST→Native까지 연결했다.
[전용 EBNF](GRAMMAR_STAGE_B_FLOAT.ebnf)는 P08 primary에 FLOAT만 추가한다.
[구현 기록](FLOAT_IMPLEMENTATION.md)에 literal/연산/출력의 독립 유리수 oracle과 Windows O0/O2 검증을 기록했다.
숫자 cast는 P10, float remainder/math API·aggregate/module과 전체 D07은 별도 후속이다.

## Backlog 경계

[P10 명시적 숫자 cast](CAST_STAGE_B_PROPOSAL.md)는 사용자 승인으로 Accepted이며 구현했다.
[전용 EBNF](GRAMMAR_STAGE_B_CAST.ebnf)는 P09 postfix에 `as type`만 추가한다.
checked narrowing·직접 RN 반올림·truncation 후 범위 검사·finite narrowing 실패와 const/Runtime
진단의 [구현·검증 기록](CAST_IMPLEMENTATION.md). 후속은 aggregate 최소 계약의 정의·검토와
float remainder/math API, Linux Native host 실행 검증이다. 전체 D07 승인을 의미하지 않는다.

[P11 Module·다중 파일 최소 계약](MODULE_STAGE_B_PROPOSAL.md)은 2026-10-05 사용자 승인 후 구현했다.
[전용 EBNF](GRAMMAR_STAGE_B_MODULE.ebnf)는 top-level use/visibility만 추가한다.
root-relative 함수/전역 const import·alias·가시성·reachable graph·cross-file const/entry/source identity와
[구현·검증 증거](MODULE_IMPLEMENTATION.md)를 따른다. module alias/qualified value/reexport/Package와 전체 D06/D30은 후속이다.
[P12 Copy struct](STRUCT_STAGE_B_PROPOSAL.md)는 2026-10-05 사용자 승인 후 구현했다.
위치 인수 생성·type import·field 읽기/가변 경로·const·private layout/ABI와 자원 한도를 제공한다.
[37-production EBNF](GRAMMAR_STAGE_B_STRUCT.ebnf)와 [두 파일 fixture](struct-proposal-fixtures/README.md)는 Accepted이며
[구현·검증 기록](STRUCT_IMPLEMENTATION.md)을 따른다. String field·명시적 init/Drop·일반 Move/borrow·Enum/Tuple/Array는 별도 후속 계약이다.

Pin/self-reference, dynamic objects/vtable/associated types, async/generator, registry server,
hosted .NET/JVM/Python, self-hosting은 별도 버전/범위 검토. Stage A의 Advanced Optimization,
borrow checker/generalized factory 등은 앞당겨 구현하지 않는다.

[P13 Copy Tuple](TUPLE_STAGE_B_PROPOSAL.md)·[40-production EBNF](GRAMMAR_STAGE_B_TUPLE.ebnf)·
[두 파일 fixture](tuple-proposal-fixtures/README.md)는 2026-10-07 사용자 승인으로 Accepted다.
구조적 Copy 타입·numeric selector subspan·혼합 가변 경로·const·private ABI와 자원 한도를 구현했다.
[구현·검증 기록](TUPLE_IMPLEMENTATION.md)과 [사용자 실행 명령](../../TESTING.md)을 제공한다. Array·일반 Move element는 후속이다.

[P14 Copy Enum·statement match](ENUM_STAGE_B_PROPOSAL.md)는 2026-10-07 사용자 승인 후 구현했다.
[48-production EBNF](GRAMMAR_STAGE_B_ENUM.ebnf), [두 파일·부정 16사례 fixture](enum-proposal-fixtures/README.md),
[구현·검증 기록](ENUM_IMPLEMENTATION.md)과 [사용자 실행 명령](../../TESTING.md)을 제공한다.
Enum/Bool의 유한 coverage·Copy payload·const·private tagged ABI를 지원한다.
Array·Option/Result·guard/nested pattern·일반 Move/borrow/Drop과 전체 D06/D08/D09/D10/D12/D16/D25/D30은 후속이다.

[P15 Copy Option·Result·nullable](OPTION_RESULT_STAGE_B_PROPOSAL.md)는 2026-10-07 사용자 승인 후 구현했다.
[51-production EBNF](GRAMMAR_STAGE_B_OPTION_RESULT.ebnf), [두 파일·부정 21사례](option-result-proposal-fixtures/README.md),
[구현·검증 기록](OPTION_RESULT_IMPLEMENTATION.md)과 [사용자 실행 명령](../../TESTING.md)을 제공한다.
Builtin Copy specialization·T?·생성 문맥·none·match·const·private ABI와 제한을 지원한다.
try·exists·method API·Array·String/Move payload·사용자 Generic·일반 borrow/Drop은 후속이다.

다음 착수 후보는 [P16 Copy try·Result 오류 전파](TRY_STAGE_B_PROPOSAL.md)다.
상태는 Draft / 사용자 승인 대기 / 미구현이며 [51-production EBNF](GRAMMAR_STAGE_B_TRY.ebnf)와
[두 파일·부정 18사례](try-proposal-fixtures/README.md)의 prefix·exact E·Copy snapshot·조기 반환·const 금지·Source/CFG 기준을 검토한다.
승인 후 Parser→HIR→TypeChecker→MIR/검증→LLVM/Native 순으로 연결하고 실행 명령·예상 출력과 실제 검증 결과를 제공한다.
현재 승인 P01~P15와 원본 148개는 유지한다. Option try·error conversion·일반 Move/Drop·Array는 별도 후속이다.
