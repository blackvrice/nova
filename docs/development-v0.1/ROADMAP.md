# 문서 동결과 Nova 구현 로드맵

현재 구현은 **P38 Copy Tuple 중간 rest 패턴까지 완료**다.
[P38 계약](TUPLE_MIDDLE_REST_STAGE_B_PROPOSAL.md)·[70-production EBNF](GRAMMAR_STAGE_B_TUPLE_MIDDLE_REST.ebnf)·[수용 fixture](tuple-middle-rest-proposal-fixtures/README.md)·[구현 기록](TUPLE_MIDDLE_REST_IMPLEMENTATION.md)·[직접 테스트](../../TESTING.md).
Tuple middle rest·prefix/suffix·실제 field_indices/annotation·전체 initializer 평가·const/coverage/MIR를 지원한다. struct/Enum middle rest·rest binder·Array·일반 Move/Drop은 후속 계약이다.


현재 구현은 **P37 Copy Enum·sum payload trailing rest 패턴까지 완료**다.
[P37 계약](PAYLOAD_REST_PATTERN_STAGE_B_PROPOSAL.md)·[69-production EBNF](GRAMMAR_STAGE_B_PAYLOAD_REST_PATTERN.ebnf)·[수용 fixture](payload-rest-pattern-proposal-fixtures/README.md)·[구현 기록](PAYLOAD_REST_PATTERN_IMPLEMENTATION.md)·[직접 테스트](../../TESTING.md).
Enum/Option/Result payload trailing rest·명시 prefix·tag/전체 initializer 평가·const/coverage/MIR를 지원한다. middle rest·named payload·Array·일반 Move/Drop은 후속 계약이다.


현재 구현은 **P36 Copy Tuple trailing rest 패턴까지 완료**다.
[P36 계약](TUPLE_REST_PATTERN_STAGE_B_PROPOSAL.md)·[68-production EBNF](GRAMMAR_STAGE_B_TUPLE_REST_PATTERN.ebnf)·[수용 fixture](tuple-rest-pattern-proposal-fixtures/README.md)·[구현 기록](TUPLE_REST_PATTERN_IMPLEMENTATION.md)·[직접 테스트](../../TESTING.md).
Tuple trailing rest·명시 prefix·Unit 구분·전체 initializer 평가·prefix const/coverage/MIR를 지원한다. middle rest·Enum payload rest·Array·일반 Move/Drop은 후속 계약이다.


현재 구현은 **P35 Copy struct trailing rest 패턴까지 완료**다.
[P35 계약](STRUCT_REST_PATTERN_STAGE_B_PROPOSAL.md)·[67-production EBNF](GRAMMAR_STAGE_B_STRUCT_REST_PATTERN.ebnf)·[수용 fixture](struct-rest-pattern-proposal-fixtures/README.md)·[구현 기록](STRUCT_REST_PATTERN_IMPLEMENTATION.md)·[직접 테스트](../../TESTING.md).
struct trailing rest·부분 field 대응·전체 initializer 평가·sparse const/coverage/MIR를 지원한다. shorthand·Tuple/Enum rest·Array·일반 Move/Drop은 후속 계약이다.


현재 구현은 **P34 Copy struct 이름 필드 패턴까지 완료**다.
[P34 계약](STRUCT_NAMED_PATTERN_STAGE_B_PROPOSAL.md)·[66-production EBNF](GRAMMAR_STAGE_B_STRUCT_NAMED_PATTERN.ebnf)·[수용 fixture](struct-named-pattern-proposal-fixtures/README.md)·[구현 기록](STRUCT_NAMED_PATTERN_IMPLEMENTATION.md)·[직접 테스트](../../TESTING.md).
역순/위치 prefix field label·전체 field 대응·정규화 plan·let/var/const·coverage/Source/MIR를 지원한다. rest/shorthand·Array·일반 Move/Drop은 후속 계약이다.


현재 구현은 **P33 Copy struct 위치 패턴까지 완료**다.
[P33 계약](STRUCT_PATTERN_STAGE_B_PROPOSAL.md)·[64-production EBNF](GRAMMAR_STAGE_B_STRUCT_PATTERN.ebnf)·[수용 fixture](struct-pattern-proposal-fixtures/README.md)·[구현 기록](STRUCT_PATTERN_IMPLEMENTATION.md)·[직접 테스트](../../TESTING.md).
nominal/import struct pattern·private field 검사·let/var/const 분해·bounded coverage·Source/MIR projection을 지원한다. 이름 field/rest 패턴·Array·일반 Move/Drop은 후속 계약이다.


현재 구현은 **P32 고정 폭 정수 literal 패턴까지 완료**다.
[P32 계약](INTEGER_PATTERN_STAGE_B_PROPOSAL.md)·[63-production EBNF](GRAMMAR_STAGE_B_INTEGER_PATTERN.ebnf)·[수용 fixture](integer-pattern-proposal-fixtures/README.md)·[구현 기록](INTEGER_PATTERN_IMPLEMENTATION.md)·[직접 테스트](../../TESTING.md).
8종 checked literal·radix/zero alias·Int8/UInt8 전체 domain·nested/guard·MIR equality를 지원한다. float/string literal·field/range pattern·Array·일반 Move/Drop은 후속 계약이다.


현재 구현은 **P31 Unicode char literal 패턴까지 완료**다.
[P31 계약](CHAR_PATTERN_STAGE_B_PROPOSAL.md)·[62-production EBNF](GRAMMAR_STAGE_B_CHAR_PATTERN.ebnf)·[수용 fixture](char-pattern-proposal-fixtures/README.md)·[구현 기록](CHAR_PATTERN_IMPLEMENTATION.md)·[직접 테스트](../../TESTING.md).
root/nested char literal·escape alias·guard·bounded coverage·MIR equality를 지원한다. 정수/float/string literal·field pattern·Array·일반 Move/Drop은 후속 계약이다.


현재 구현은 **P30 숫자·char·Copy struct statement match까지 완료**다.
[P30 계약](COPY_VALUE_MATCH_STAGE_B_PROPOSAL.md)·[수용 fixture](copy-value-match-proposal-fixtures/README.md)·[구현 기록](COPY_VALUE_MATCH_IMPLEMENTATION.md)·[직접 테스트](../../TESTING.md).
whole Copy binder/wildcard·Bool guard·snapshot·보수적 완전성/도달성을 지원한다. Array·literal/field pattern·일반 Move/Drop은 후속 계약이다.


현재 구현은 **P29 전역 Copy const 구조 분해까지 완료**다.
[P29 계약](GLOBAL_CONST_DESTRUCTURING_STAGE_B_PROPOSAL.md)·[62-production EBNF](GRAMMAR_STAGE_B_GLOBAL_CONST_DESTRUCTURING.ebnf)·[수용 fixture](global-const-destructuring-proposal-fixtures/README.md)·[구현 기록](GLOBAL_CONST_DESTRUCTURING_IMPLEMENTATION.md)·[직접 테스트](../../TESTING.md).
component import/가시성·forward reference·공유 initializer 평가·정적 group 순환을 지원한다. Array·const 함수·일반 Move/Drop은 후속 계약이다.


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

[P16 Copy try·Result 오류 전파](TRY_STAGE_B_PROPOSAL.md)는 2026-10-07 사용자 승인 후 구현했다.
[51-production EBNF](GRAMMAR_STAGE_B_TRY.ebnf)·[두 파일·부정 18사례](try-proposal-fixtures/README.md)·
[구현 기록](TRY_IMPLEMENTATION.md)과 [사용자 실행 명령](../../TESTING.md)을 제공한다.
Prefix 결합·operand 문맥 격리·정확한 E·Copy snapshot·Error 조기 반환·const N3201·Source/CFG 검증을 지원한다.
Option try·error conversion·String/Move payload·일반 Drop와 전체 D08/D09/D10/D12/D16/D23/D25는 후속이다.

[P17 함수 이름 인수](NAMED_ARGUMENTS_STAGE_B_PROPOSAL.md)·[52-production EBNF](GRAMMAR_STAGE_B_NAMED_ARGUMENTS.ebnf)·
[두 파일·부정 16사례 수용 fixture](named-arguments-proposal-fixtures/README.md)는 **Accepted / 구현 완료**다.
[구현 기록](NAMED_ARGUMENTS_IMPLEMENTATION.md)을 따른다.
사용자 함수의 parameter 이름 대응·source-order snapshot/parameter-order 전달·mapped 타입/try 검증을 구현했다.
기본 인수·overload·named constructor·Array·일반 Move/Drop은 포함하지 않는다. 현재 실행 명령은 [TESTING.md](../../TESTING.md)를 따른다.

[P18 상수 표현식 함수 기본 인수](DEFAULT_ARGUMENTS_STAGE_B_PROPOSAL.md)·[52-production EBNF](GRAMMAR_STAGE_B_DEFAULT_ARGUMENTS.ebnf)·
[두 파일·부정 20사례 수용 fixture](default-arguments-proposal-fixtures/README.md)는 **Accepted / 구현 완료**다.
[구현 기록](DEFAULT_ARGUMENTS_IMPLEMENTATION.md)을 따른다.
선언 module의 상수 default·필수/default 혼합·생략 대응·caller materialization·Source/CFG 검증을 구현했다.
runtime default·parameter 참조·overload·named constructor·일반 Move/Drop은 후속이다. 현재 실행 명령은 [TESTING.md](../../TESTING.md)를 따른다.

[P19 loop·정수 범위 for](RANGE_LOOP_STAGE_B_PROPOSAL.md)·[54-production EBNF](GRAMMAR_STAGE_B_RANGE_LOOP.ebnf)·
[두 파일/부정 18/Runtime 2사례](range-loop-proposal-fixtures/README.md)는 **Accepted / 구현 완료**다. [구현 기록](RANGE_LOOP_IMPLEMENTATION.md)을 따른다.
기존 while·정수 승격·try·P18 default를 연결하는 승인 Stage B 최소 범위다.
단일 bound 평가·immutable binder·until/through·MAX 종료·nested jump·보수적 return 분석을 구현했다.
Array/iterable protocol·Range 값·step/descending·일반 Move/Drop은 후속이다.

[P20 Copy Option postfix exists](EXISTS_STAGE_B_PROPOSAL.md)·[54-production EBNF](GRAMMAR_STAGE_B_EXISTS.ebnf)·
[두 파일/정상 2/부정 20/Runtime 1사례](exists-proposal-fixtures/README.md)는 **Accepted / 구현 완료**다.
Bool·const/default·단일 평가·Source/MIR 검증을 구현했고 P01~P19를 보존했다.
[구현 기록](EXISTS_IMPLEMENTATION.md)·[독립 예제/실행 명령](../../TESTING.md)을 제공한다.
Result exists·flow narrowing·Move/Drop·Array는 후속이다.


[P21 비제네릭 Type Alias](ALIAS_STAGE_B_PROPOSAL.md)·[55-production EBNF](GRAMMAR_STAGE_B_ALIAS.ebnf)·
[두 파일·부정 16사례](alias-proposal-fixtures/README.md)는 **Accepted / 구현 완료**다.
type 위치·forward/import·선언 scope·순환/한도·Source/MIR 검증을 구현했다. P01~P20과 원본을 보존한다.
[구현 기록](ALIAS_IMPLEMENTATION.md)·[독립 예제/실행 명령](../../TESTING.md)을 제공한다.
generic alias·newtype·alias constructor/variant head·API leak/export 정책·Array·일반 Move/Drop은 후속이다.


사용자 승인한 [P22 Copy struct Read 메서드](METHOD_STAGE_B_PROPOSAL.md)·[58-production EBNF](GRAMMAR_STAGE_B_METHOD.ebnf)·
[두 파일·정상 1/부정 20/Runtime 1사례](method-proposal-fixtures/README.md)는 **Accepted / 구현 완료**다.
contextual self·member scope/visibility·receiver-first snapshot·named/default·Source/MIR 검증을 구현했다. P01~P21을 보존한다.
[구현 기록](METHOD_IMPLEMENTATION.md)·[독립 예제/실행 명령](../../TESTING.md)을 제공한다.
change/take·일반 Move/borrow/Drop·init·overload·bound method·Enum method·Array는 후속이다.


사용자 승인한 [P23 Copy struct 생성자 이름 인수](STRUCT_NAMED_ARGUMENTS_STAGE_B_PROPOSAL.md)·[수용 fixture](struct-named-arguments-proposal-fixtures/README.md)는 **Accepted / 구현·검증 완료**다.
P22 문법을 재사용하며 field 대응·가시성·source-order snapshot·const/default·Source/MIR를 구현했다.
[구현 기록](STRUCT_NAMED_ARGUMENTS_IMPLEMENTATION.md)·[독립 예제/명령](../../TESTING.md)을 제공한다.
explicit init·field default·Enum/sum named payload·Array·일반 Move/Drop은 별도 계약으로 남긴다.

[P24 중첩 Copy 패턴·Tuple match](NESTED_PATTERN_STAGE_B_PROPOSAL.md)는 **Accepted / 구현·검증 완료**다.
중첩 Enum/Option/Result·Copy Tuple/Unit 패턴, 불변 binder와 합집합 완전성·도달성 검사, 자원·Source/MIR 검증을 지원한다.
[60-production EBNF](GRAMMAR_STAGE_B_NESTED_PATTERN.ebnf)·[수용 fixture](nested-pattern-proposal-fixtures/README.md)·[구현 기록](NESTED_PATTERN_IMPLEMENTATION.md)·[독립 예제/명령](../../TESTING.md)을 제공한다.
guard·or/range·일반 literal·struct destructuring·Array·Move/Drop은 후속 계약이다.


[P25 Copy match guard](GUARD_STAGE_B_PROPOSAL.md)는 **Accepted / 구현·검증 완료**다.
Bool guard·pattern-success 단일 평가·false continuation·unguarded 합집합 coverage·try·Source/MIR 검증을 지원한다.
[61-production EBNF](GRAMMAR_STAGE_B_GUARD.ebnf)·[수용 fixture](guard-proposal-fixtures/README.md)·[구현 기록](GUARD_IMPLEMENTATION.md)·[독립 예제/명령](../../TESTING.md)을 제공한다.
Array·일반 Move/Drop·guard truth/refinement 증명은 후속 계약이다.


[P26 irrefutable Copy let 구조 분해](DESTRUCTURING_STAGE_B_PROPOSAL.md)는 **Accepted / 구현·검증 완료**다.
Tuple/Unit/single-variant Enum·불변 Copy binder·initializer 단일 평가/snapshot·선언 시점·N3103·Source/MIR 검증을 지원한다.
[62-production EBNF](GRAMMAR_STAGE_B_DESTRUCTURING.ebnf)·[수용 fixture](destructuring-proposal-fixtures/README.md)·[구현 기록](DESTRUCTURING_IMPLEMENTATION.md)·[직접 예제/명령](../../TESTING.md)을 제공한다.
var/const/parameter/for/assignment 구조 분해·Array·일반 Move/Drop은 후속 계약이다.


[P27 가변 Copy var 구조 분해](VAR_DESTRUCTURING_STAGE_B_PROPOSAL.md)는 **Accepted / 구현·검증 완료**다.
Tuple/Unit/single-variant Enum의 mutable Copy binder·직접/기존 경로 대입·initializer 단일 평가/snapshot·Source/Resolved/Checked/MIR 검증을 지원한다.
[62-production EBNF](GRAMMAR_STAGE_B_VAR_DESTRUCTURING.ebnf)·[수용 fixture](var-destructuring-proposal-fixtures/README.md)·[구현 기록](VAR_DESTRUCTURING_IMPLEMENTATION.md)·[직접 예제/명령](../../TESTING.md)을 제공한다.
let/match binder는 immutable이다. const/Array·일반 Move/Drop 등은 후속 계약이다.


[P28 함수 내부 Copy const 구조 분해](CONST_DESTRUCTURING_STAGE_B_PROPOSAL.md)는 **Accepted / 구현·검증 완료**다.
Tuple/Unit/single-variant Enum·불변 상수 binder·initializer 한 번 평가·cached component 값·실패 전파·Source/Checked/MIR 검증을 지원한다.
[62-production EBNF](GRAMMAR_STAGE_B_CONST_DESTRUCTURING.ebnf)·[수용 fixture](const-destructuring-proposal-fixtures/README.md)·[구현 기록](CONST_DESTRUCTURING_IMPLEMENTATION.md)·[직접 예제/명령](../../TESTING.md)을 제공한다.
전역 구조 분해·const 함수·Array·일반 Move/Drop은 후속 계약이다.
