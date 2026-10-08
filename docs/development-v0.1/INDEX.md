# Nova 0.1 개발 문서 전체 색인

148개 원본 주제를 빠짐없이 보완했다. 전체 상세는 Draft이며 D01~D05 Lexer, [P01 Parser](PARSER_STAGE_A_PROPOSAL.md), [P02 의미 검사](SEMANTICS_STAGE_A_PROPOSAL.md), [P03 Native](NATIVE_STAGE_A_PROPOSAL.md), [P04 가변 변수·반복문](CONTROL_STAGE_B_PROPOSAL.md), [P05 const](CONST_STAGE_B_PROPOSAL.md) subset은 Accepted다. 승인/구현/테스트 통과 상태를 구분한다.

[P06 단일 파일 전역 const](GLOBAL_CONST_STAGE_B_PROPOSAL.md)와 [전용 EBNF](GRAMMAR_STAGE_B_GLOBAL_CONST.ebnf) subset도 Accepted이며 [구현·검증 기록](GLOBAL_CONST_IMPLEMENTATION.md)을 따른다.

[P07 고정 폭 정수·승격](INTEGER_STAGE_B_PROPOSAL.md) subset도 Accepted이며 [구현·검증 기록](INTEGER_IMPLEMENTATION.md)을 따른다. 기존 P06 source grammar를 재사용한다.

[P08 char·scalar 비교·UTF-8 보간](CHAR_STAGE_B_PROPOSAL.md)과 [CHAR primary 전용 EBNF](GRAMMAR_STAGE_B_CHAR.ebnf)는 Accepted이며 [구현·검증 기록](CHAR_IMPLEMENTATION.md)을 따른다.

[P09 float·IEEE 결과·숫자 승격·보간](FLOAT_STAGE_B_PROPOSAL.md)과 [전용 EBNF](GRAMMAR_STAGE_B_FLOAT.ebnf)는 Accepted이며 [구현·검증 기록](FLOAT_IMPLEMENTATION.md)을 따른다.

[P10 명시적 숫자 cast](CAST_STAGE_B_PROPOSAL.md)와 [전용 EBNF](GRAMMAR_STAGE_B_CAST.ebnf)는 Accepted이며 [구현·검증 기록](CAST_IMPLEMENTATION.md)을 따른다.

[P11 Module·다중 파일 최소 계약](MODULE_STAGE_B_PROPOSAL.md)과 [전용 EBNF](GRAMMAR_STAGE_B_MODULE.ebnf)는 Accepted이며 [구현·검증 기록](MODULE_IMPLEMENTATION.md)을 따른다.

[P12 Copy struct 최소 계약](STRUCT_STAGE_B_PROPOSAL.md)과 [37-production EBNF](GRAMMAR_STAGE_B_STRUCT.ebnf), [수용 fixture](struct-proposal-fixtures/README.md)는 Accepted이며 [구현·검증 기록](STRUCT_IMPLEMENTATION.md)을 따른다. 전체 D06/D10/D12/D16/D30 승인이 아니다.

[P13 Copy Tuple 최소 계약](TUPLE_STAGE_B_PROPOSAL.md), [40-production EBNF](GRAMMAR_STAGE_B_TUPLE.ebnf), [수용 fixture](tuple-proposal-fixtures/README.md)는 Accepted이며 [구현 기록](TUPLE_IMPLEMENTATION.md)을 따른다. 전체 D09/D10/D12/D16 승인이 아니다.

[P14 Copy Enum·statement match 최소 계약](ENUM_STAGE_B_PROPOSAL.md), [48-production EBNF](GRAMMAR_STAGE_B_ENUM.ebnf), [수용 fixture](enum-proposal-fixtures/README.md)는 Accepted / 구현 완료이며 [구현 기록](ENUM_IMPLEMENTATION.md)을 따른다. 전체 D06/D08/D09/D10/D12/D16/D25/D30 승인이 아니다.

[P15 Copy Option·Result·nullable 최소 계약](OPTION_RESULT_STAGE_B_PROPOSAL.md), [51-production EBNF](GRAMMAR_STAGE_B_OPTION_RESULT.ebnf), [수용 fixture](option-result-proposal-fixtures/README.md)는 Accepted / 구현 완료다. [구현·검증 기록](OPTION_RESULT_IMPLEMENTATION.md)을 따른다.

[P16 Copy try·Result 오류 전파 최소 계약](TRY_STAGE_B_PROPOSAL.md), [51-production EBNF](GRAMMAR_STAGE_B_TRY.ebnf), [두 파일·부정 18사례 수용 fixture](try-proposal-fixtures/README.md)는 Accepted / 구현 완료다. [구현 기록](TRY_IMPLEMENTATION.md)을 따른다.

[P17 함수 이름 인수 최소 계약](NAMED_ARGUMENTS_STAGE_B_PROPOSAL.md), [52-production EBNF](GRAMMAR_STAGE_B_NAMED_ARGUMENTS.ebnf), [두 파일·부정 16사례 수용 fixture](named-arguments-proposal-fixtures/README.md)는 Accepted / 구현 완료다. [구현 기록](NAMED_ARGUMENTS_IMPLEMENTATION.md)을 따른다. 기본 인수·overload·named constructor는 포함하지 않는다.

[P18 상수 표현식 함수 기본 인수 최소 계약](DEFAULT_ARGUMENTS_STAGE_B_PROPOSAL.md), [52-production EBNF](GRAMMAR_STAGE_B_DEFAULT_ARGUMENTS.ebnf), [두 파일·부정 20사례 수용 fixture](default-arguments-proposal-fixtures/README.md)는 Accepted / 구현 완료다. [구현 기록](DEFAULT_ARGUMENTS_IMPLEMENTATION.md)을 따른다. runtime default·parameter 참조·overload·named constructor는 포함하지 않는다.

[P19 loop·정수 범위 for](RANGE_LOOP_STAGE_B_PROPOSAL.md), [54-production EBNF](GRAMMAR_STAGE_B_RANGE_LOOP.ebnf), [두 파일·부정 18/Runtime 2사례 fixture](range-loop-proposal-fixtures/README.md)는 Accepted / 구현 완료다. [구현 기록](RANGE_LOOP_IMPLEMENTATION.md)을 따른다.

[P20 Copy Option postfix exists](EXISTS_STAGE_B_PROPOSAL.md), [54-production EBNF](GRAMMAR_STAGE_B_EXISTS.ebnf), [두 파일·정상 2/부정 20/Runtime 1사례](exists-proposal-fixtures/README.md)는 Accepted / 구현 완료다. [구현 기록](EXISTS_IMPLEMENTATION.md)을 따른다.

[P21 비제네릭 Type Alias](ALIAS_STAGE_B_PROPOSAL.md), [55-production EBNF](GRAMMAR_STAGE_B_ALIAS.ebnf), [두 파일·부정 16사례](alias-proposal-fixtures/README.md)는 Accepted / 구현 완료다. [구현·검증 기록](ALIAS_IMPLEMENTATION.md)을 따른다.

[시작 문서](README.md) · [결정](DECISIONS.md) · [문법](GRAMMAR.ebnf) · [검증 사례](CONFORMANCE.md)

| ID | 분야 | 작성 문서 | Stage | 상태 |
|---|---|---|---|---|
| NOVA-001 | 00_Governance | [Nova 언어 설계 원칙·목표·비목표](specs/NOVA-001.md) | 전 Stage | Draft |
| NOVA-002 | 00_Governance | [Nova 0.1 MVP 기능 동결표](specs/NOVA-002.md) | 전 Stage | Draft |
| NOVA-003 | 00_Governance | [Nova 0.1 비지원 기능 목록](specs/NOVA-003.md) | 전 Stage | Draft |
| NOVA-004 | 00_Governance | [Nova 용어·키워드 Canonical 표](specs/NOVA-004.md) | 전 Stage | Draft |
| NOVA-005 | 00_Governance | [Nova 문서 인덱스·의존 관계도](specs/NOVA-005.md) | 전 Stage | Draft |
| NOVA-006 | 00_Governance | [Nova 언어 변경 제안·결정 기록 절차](specs/NOVA-006.md) | 전 Stage | Draft |
| NOVA-007 | 00_Governance | [Nova 버전 정책·호환성 원칙](specs/NOVA-007.md) | 전 Stage | Draft |
| NOVA-008 | 01_Source_Syntax | [소스 인코딩·Unicode·줄바꿈 사양서](specs/NOVA-008.md) | A~B | Draft |
| NOVA-009 | 01_Source_Syntax | [Token 종류·예약어·연산자 사양서](specs/NOVA-009.md) | A~B | Draft |
| NOVA-010 | 01_Source_Syntax | [숫자·문자·문자열 Literal 사양서](specs/NOVA-010.md) | A~B | Draft |
| NOVA-011 | 01_Source_Syntax | [문자열 보간 Lexer Mode 사양서](specs/NOVA-011.md) | A~B | Draft |
| NOVA-012 | 01_Source_Syntax | [주석·중첩 주석 사양서](specs/NOVA-012.md) | A~B | Draft |
| NOVA-013 | 01_Source_Syntax | [줄바꿈·문장 종료·연속 줄 규칙](specs/NOVA-013.md) | A~B | Draft |
| NOVA-014 | 01_Source_Syntax | [문법 명세 및 EBNF 사양서](specs/NOVA-014.md) | A~B | Draft |
| NOVA-015 | 01_Source_Syntax | [연산자 우선순위·결합 방향 사양서](specs/NOVA-015.md) | A~B | Draft |
| NOVA-016 | 01_Source_Syntax | [Parser·AST 사양서](specs/NOVA-016.md) | A~B | Draft |
| NOVA-017 | 01_Source_Syntax | [Parser 오류 복구 사양서](specs/NOVA-017.md) | A~B | Draft |
| NOVA-018 | 01_Source_Syntax | [소스 포매팅 기준서](specs/NOVA-018.md) | E | Draft |
| NOVA-019 | 02_Names_Modules | [이름 해석·Scope 사양서](specs/NOVA-019.md) | B~E | Draft |
| NOVA-020 | 02_Names_Modules | [Module·Import 사양서](specs/NOVA-020.md) | B~E | Draft |
| NOVA-021 | 02_Names_Modules | [접근 제한·가시성 사양서](specs/NOVA-021.md) | B~E | Draft |
| NOVA-022 | 02_Names_Modules | [Symbol·Definition ID 사양서](specs/NOVA-022.md) | B~E | Draft |
| NOVA-023 | 02_Names_Modules | [Package 간 이름 해석 사양서](specs/NOVA-023.md) | B~E | Draft |
| NOVA-024 | 02_Names_Modules | [이름 충돌·Shadowing 진단 기준서](specs/NOVA-024.md) | B~E | Draft |
| NOVA-025 | 03_Types_Declarations | [타입 시스템·타입 추론·형변환 사양서](specs/NOVA-025.md) | A~B | Draft |
| NOVA-026 | 03_Types_Declarations | [Primitive·Literal 기본 타입 결정 사양서](specs/NOVA-026.md) | A~B | Draft |
| NOVA-027 | 03_Types_Declarations | [Struct·Class·Enum 선언 사양서](specs/NOVA-027.md) | B~E | Draft |
| NOVA-028 | 03_Types_Declarations | [Nullable·Option·Result 타입 규칙](specs/NOVA-028.md) | B~E | Draft |
| NOVA-029 | 03_Types_Declarations | [Tuple·Array·Function 타입 사양서](specs/NOVA-029.md) | B~E | Draft |
| NOVA-030 | 03_Types_Declarations | [Type Alias·타입 정규화 사양서](specs/NOVA-030.md) | B~E | Draft |
| NOVA-031 | 03_Types_Declarations | [생성자·필드 초기화 사양서](specs/NOVA-031.md) | B~E | Draft |
| NOVA-032 | 03_Types_Declarations | [상수 표현식·const 평가 사양서](specs/NOVA-032.md) | B~E | Draft |
| NOVA-033 | 03_Types_Declarations | [타입 레이아웃·정렬·Niche 사양서](specs/NOVA-033.md) | B~E | Draft |
| NOVA-034 | 03_Types_Declarations | [Class 객체 모델·정체성·배치 사양서](specs/NOVA-034.md) | D | Draft |
| NOVA-035 | 04_Functions_Control | [함수·메서드·호출 규약·Receiver 사양서](specs/NOVA-035.md) | A~B | Draft |
| NOVA-036 | 04_Functions_Control | [위치·이름·기본 인수 사양서](specs/NOVA-036.md) | A~B | Draft |
| NOVA-037 | 04_Functions_Control | [Overload 해석·변환 비용 사양서](specs/NOVA-037.md) | A~B | Draft |
| NOVA-038 | 04_Functions_Control | [함수 타입·Lambda·Closure 사양서](specs/NOVA-038.md) | D | Draft |
| NOVA-039 | 04_Functions_Control | [Closure Capture·호출 Receiver 사양서](specs/NOVA-039.md) | D | Draft |
| NOVA-040 | 04_Functions_Control | [main·프로그램 진입점 사양서](specs/NOVA-040.md) | A~B | Draft |
| NOVA-041 | 04_Functions_Control | [Effect·pure·noPanic 사양서](specs/NOVA-041.md) | A~B | Draft |
| NOVA-042 | 04_Functions_Control | [Native Calling Convention 사양서](specs/NOVA-042.md) | A~B | Draft |
| NOVA-043 | 04_Functions_Control | [if·while·for·loop 사양서](specs/NOVA-043.md) | A~B | Draft |
| NOVA-044 | 04_Functions_Control | [break·continue·return 사양서](specs/NOVA-044.md) | A~B | Draft |
| NOVA-045 | 04_Functions_Control | [Range until·through 사양서](specs/NOVA-045.md) | A~B | Draft |
| NOVA-046 | 04_Functions_Control | [Pattern 문법·Binding 사양서](specs/NOVA-046.md) | A~B | Draft |
| NOVA-047 | 04_Functions_Control | [Match 완전성·도달 불가 Arm 분석서](specs/NOVA-047.md) | A~B | Draft |
| NOVA-048 | 04_Functions_Control | [Match Lowering·Decision Tree 사양서](specs/NOVA-048.md) | A~B | Draft |
| NOVA-049 | 04_Functions_Control | [try·Result 전파 Lowering 사양서](specs/NOVA-049.md) | A~B | Draft |
| NOVA-050 | 04_Functions_Control | [using Lowering 사양서](specs/NOVA-050.md) | A~B | Draft |
| NOVA-051 | 05_Ownership_Safety | [메모리·소유권 모델 사양서](specs/NOVA-051.md) | C | Draft |
| NOVA-052 | 05_Ownership_Safety | [초기화·이동 상태 분석 사양서](specs/NOVA-052.md) | C | Draft |
| NOVA-053 | 05_Ownership_Safety | [부분 이동 정책 사양서](specs/NOVA-053.md) | C | Draft |
| NOVA-054 | 05_Ownership_Safety | [빌림·수명·Place 충돌 사양서](specs/NOVA-054.md) | C | Draft |
| NOVA-055 | 05_Ownership_Safety | [Drop 정교화·Drop Flag 사양서](specs/NOVA-055.md) | C | Draft |
| NOVA-056 | 05_Ownership_Safety | [사용자 drop·Drop Glue 사양서](specs/NOVA-056.md) | C | Draft |
| NOVA-057 | 05_Ownership_Safety | [Shared·Weak·참조 횟수 모델 사양서](specs/NOVA-057.md) | C~E | Draft |
| NOVA-058 | 05_Ownership_Safety | [Raw Pointer·Unsafe Capability 사양서](specs/NOVA-058.md) | C~E | Draft |
| NOVA-059 | 05_Ownership_Safety | [Pinning·자기 참조 타입 사양서](specs/NOVA-059.md) | 제외/후속 | Draft |
| NOVA-060 | 05_Ownership_Safety | [Thread 이동·공유 안전성 사양서](specs/NOVA-060.md) | C~E | Draft |
| NOVA-061 | 06_Interfaces_Generics | [Interface·구현·정적 디스패치 사양서](specs/NOVA-061.md) | D | Draft |
| NOVA-062 | 06_Interfaces_Generics | [Generic 제약 증명 사양서](specs/NOVA-062.md) | D | Draft |
| NOVA-063 | 06_Interfaces_Generics | [Generic 타입 추론 사양서](specs/NOVA-063.md) | D | Draft |
| NOVA-064 | 06_Interfaces_Generics | [Monomorphization·특수화 사양서](specs/NOVA-064.md) | D | Draft |
| NOVA-065 | 06_Interfaces_Generics | [Generic Cache·코드 팽창 제어 사양서](specs/NOVA-065.md) | D | Draft |
| NOVA-066 | 06_Interfaces_Generics | [Associated Type·Dynamic Interface Object 사양서](specs/NOVA-066.md) | 제외/후속 | Draft |
| NOVA-067 | 06_Interfaces_Generics | [VTable·Object Safety 사양서](specs/NOVA-067.md) | 제외/후속 | Draft |
| NOVA-068 | 07_Compiler_Frontend | [Compiler 전체 아키텍처 사양서](specs/NOVA-068.md) | A~E | Draft |
| NOVA-069 | 07_Compiler_Frontend | [저장소·Crate·모듈 구조 사양서](specs/NOVA-069.md) | A~E | Draft |
| NOVA-070 | 07_Compiler_Frontend | [Source Manager·File ID·Span 사양서](specs/NOVA-070.md) | A~E | Draft |
| NOVA-071 | 07_Compiler_Frontend | [Lexer 구현 사양서](specs/NOVA-071.md) | A~E | Draft |
| NOVA-072 | 07_Compiler_Frontend | [Parser 구현 사양서](specs/NOVA-072.md) | A~E | Draft |
| NOVA-073 | 07_Compiler_Frontend | [AST Node 구조서](specs/NOVA-073.md) | A~E | Draft |
| NOVA-074 | 07_Compiler_Frontend | [HIR Node 구조서](specs/NOVA-074.md) | A~E | Draft |
| NOVA-075 | 07_Compiler_Frontend | [AST→HIR Lowering 사양서](specs/NOVA-075.md) | A~E | Draft |
| NOVA-076 | 07_Compiler_Frontend | [Symbol Table·Definition Registry 사양서](specs/NOVA-076.md) | A~E | Draft |
| NOVA-077 | 07_Compiler_Frontend | [타입 검사기·소유권 검사기 아키텍처 사양서](specs/NOVA-077.md) | A~E | Draft |
| NOVA-078 | 07_Compiler_Frontend | [진단 시스템·Error Code 관리 사양서](specs/NOVA-078.md) | A~E | Draft |
| NOVA-079 | 07_Compiler_Frontend | [Compiler Query·의존성·Cache 사양서](specs/NOVA-079.md) | A~E | Draft |
| NOVA-080 | 07_Compiler_Frontend | [Compiler 내부 오류·ICE 처리 사양서](specs/NOVA-080.md) | A~E | Draft |
| NOVA-081 | 08_MIR_Middleend | [MIR 구조·CFG·평가 순서 사양서](specs/NOVA-081.md) | A~E | Draft |
| NOVA-082 | 08_MIR_Middleend | [HIR→MIR Lowering 사양서](specs/NOVA-082.md) | A~E | Draft |
| NOVA-083 | 08_MIR_Middleend | [MIR 검증기 사양서](specs/NOVA-083.md) | A~E | Draft |
| NOVA-084 | 08_MIR_Middleend | [초기화·Move 분석 구현서](specs/NOVA-084.md) | C | Draft |
| NOVA-085 | 08_MIR_Middleend | [Borrow Checker 구현서](specs/NOVA-085.md) | C | Draft |
| NOVA-086 | 08_MIR_Middleend | [Drop Elaboration 구현서](specs/NOVA-086.md) | C | Draft |
| NOVA-087 | 08_MIR_Middleend | [Constant Evaluation Engine 사양서](specs/NOVA-087.md) | B~E | Draft |
| NOVA-088 | 08_MIR_Middleend | [Match Lowering 구현서](specs/NOVA-088.md) | B~E | Draft |
| NOVA-089 | 08_MIR_Middleend | [Generic 특수화 Pass 구현서](specs/NOVA-089.md) | D | Draft |
| NOVA-090 | 08_MIR_Middleend | [MIR 최적화 Pass 목록·순서 사양서](specs/NOVA-090.md) | B~E | Draft |
| NOVA-091 | 08_MIR_Middleend | [SSA 변환 여부 결정서](specs/NOVA-091.md) | B~E | Draft |
| NOVA-092 | 08_MIR_Middleend | [Debug MIR 출력 형식 사양서](specs/NOVA-092.md) | B~E | Draft |
| NOVA-093 | 09_Backend_Runtime | [Backend 선택·LLVM 연동 결정서](specs/NOVA-093.md) | A~E | Draft |
| NOVA-094 | 09_Backend_Runtime | [Nova 타입→Backend 타입 Mapping 사양서](specs/NOVA-094.md) | A~E | Draft |
| NOVA-095 | 09_Backend_Runtime | [Backend 타입 레이아웃·정렬·Niche 구현서](specs/NOVA-095.md) | A~E | Draft |
| NOVA-096 | 09_Backend_Runtime | [함수 ABI·인수·반환 Lowering 사양서](specs/NOVA-096.md) | A~E | Draft |
| NOVA-097 | 09_Backend_Runtime | [Name Mangling·Symbol 사양서](specs/NOVA-097.md) | A~E | Draft |
| NOVA-098 | 09_Backend_Runtime | [Object File·Linker 연동 사양서](specs/NOVA-098.md) | A~E | Draft |
| NOVA-099 | 09_Backend_Runtime | [Runtime Startup·main 호출 사양서](specs/NOVA-099.md) | A~E | Draft |
| NOVA-100 | 09_Backend_Runtime | [메모리 Allocator·OOM 정책 사양서](specs/NOVA-100.md) | A~E | Draft |
| NOVA-101 | 09_Backend_Runtime | [Panic Runtime·Stack Trace 사양서](specs/NOVA-101.md) | A~E | Draft |
| NOVA-102 | 09_Backend_Runtime | [Shared·Weak Runtime 사양서](specs/NOVA-102.md) | A~E | Draft |
| NOVA-103 | 09_Backend_Runtime | [Thread·Atomic·Lock Runtime 사양서](specs/NOVA-103.md) | A~E | Draft |
| NOVA-104 | 09_Backend_Runtime | [Platform Abstraction Layer 사양서](specs/NOVA-104.md) | A~E | Draft |
| NOVA-105 | 10_FFI | [foreign 선언 문법·공통 모델 사양서](specs/NOVA-105.md) | A~E | Draft |
| NOVA-106 | 10_FFI | [C ABI Import·Export 사양서](specs/NOVA-106.md) | A~E | Draft |
| NOVA-107 | 10_FFI | [C Header Parser·Binding Generator 사양서](specs/NOVA-107.md) | A~E | Draft |
| NOVA-108 | 10_FFI | [C++ Bridge·Opaque Handle 사양서](specs/NOVA-108.md) | A~E | Draft |
| NOVA-109 | 10_FFI | [외부 예외→Result 변환 사양서](specs/NOVA-109.md) | A~E | Draft |
| NOVA-110 | 10_FFI | [Foreign Ownership·Drop 계약 사양서](specs/NOVA-110.md) | A~E | Draft |
| NOVA-111 | 10_FFI | [.NET Hosted Adapter 사양서](specs/NOVA-111.md) | 제외/후속 | Draft |
| NOVA-112 | 10_FFI | [JVM Hosted Adapter 사양서](specs/NOVA-112.md) | 제외/후속 | Draft |
| NOVA-113 | 10_FFI | [Python Hosted Adapter 사양서](specs/NOVA-113.md) | 제외/후속 | Draft |
| NOVA-114 | 11_Standard_Library | [Core Prelude·기본 Symbol 사양서](specs/NOVA-114.md) | B~E (print: A) | Draft |
| NOVA-115 | 11_Standard_Library | [Primitive 메서드 API 사양서](specs/NOVA-115.md) | B~E (print: A) | Draft |
| NOVA-116 | 11_Standard_Library | [string·UTF-8 API 사양서](specs/NOVA-116.md) | B~E (print: A) | Draft |
| NOVA-117 | 11_Standard_Library | [Array<T> API·메모리 모델 사양서](specs/NOVA-117.md) | B~E (print: A) | Draft |
| NOVA-118 | 11_Standard_Library | [Span<T>·ReadOnlySpan<T> 사양서](specs/NOVA-118.md) | B~E (print: A) | Draft |
| NOVA-119 | 11_Standard_Library | [Option<T> API 사양서](specs/NOVA-119.md) | B~E (print: A) | Draft |
| NOVA-120 | 11_Standard_Library | [Result<T,E>·try API 사양서](specs/NOVA-120.md) | B~E (print: A) | Draft |
| NOVA-121 | 11_Standard_Library | [List<T>·Map<K,V> 사양서](specs/NOVA-121.md) | B~E (print: A) | Draft |
| NOVA-122 | 11_Standard_Library | [파일·Stream·Console I/O 사양서](specs/NOVA-122.md) | B~E (print: A) | Draft |
| NOVA-123 | 11_Standard_Library | [시간·난수·환경 변수 사양서](specs/NOVA-123.md) | B~E (print: A) | Draft |
| NOVA-124 | 11_Standard_Library | [Thread·Lock·Atomic API 사양서](specs/NOVA-124.md) | B~E (print: A) | Draft |
| NOVA-125 | 12_Tooling_Packaging | [Compiler CLI 명령·옵션 사양서](specs/NOVA-125.md) | A~E | Draft |
| NOVA-126 | 12_Tooling_Packaging | [Package Manifest 사양서](specs/NOVA-126.md) | E | Draft |
| NOVA-127 | 12_Tooling_Packaging | [의존성 해석·Lock File 사양서](specs/NOVA-127.md) | E | Draft |
| NOVA-128 | 12_Tooling_Packaging | [Build Profile·Target 사양서](specs/NOVA-128.md) | E | Draft |
| NOVA-129 | 12_Tooling_Packaging | [Package Artifact·Metadata 형식 사양서](specs/NOVA-129.md) | E | Draft |
| NOVA-130 | 12_Tooling_Packaging | [증분 컴파일 Cache 형식 사양서](specs/NOVA-130.md) | E | Draft |
| NOVA-131 | 12_Tooling_Packaging | [Formatter 사양·구현서](specs/NOVA-131.md) | E | Draft |
| NOVA-132 | 12_Tooling_Packaging | [Linter 규칙 사양서](specs/NOVA-132.md) | E | Draft |
| NOVA-133 | 12_Tooling_Packaging | [Language Server·LSP 사양서](specs/NOVA-133.md) | E | Draft |
| NOVA-134 | 12_Tooling_Packaging | [API 문서 생성기 사양서](specs/NOVA-134.md) | E | Draft |
| NOVA-135 | 12_Tooling_Packaging | [Package Registry 사양서](specs/NOVA-135.md) | 제외/후속 | Draft |
| NOVA-136 | 13_Testing_Release | [Compiler 테스트 전략서](specs/NOVA-136.md) | 전 Stage | Draft |
| NOVA-137 | 13_Testing_Release | [Lexer·Parser Snapshot 테스트 사양서](specs/NOVA-137.md) | 전 Stage | Draft |
| NOVA-138 | 13_Testing_Release | [Compile-pass·Compile-fail 규격](specs/NOVA-138.md) | 전 Stage | Draft |
| NOVA-139 | 13_Testing_Release | [MIR Snapshot 규격](specs/NOVA-139.md) | 전 Stage | Draft |
| NOVA-140 | 13_Testing_Release | [Runtime Test Harness 사양서](specs/NOVA-140.md) | 전 Stage | Draft |
| NOVA-141 | 13_Testing_Release | [Lexer·Parser·MIR Fuzzing 사양서](specs/NOVA-141.md) | 전 Stage | Draft |
| NOVA-142 | 13_Testing_Release | [Property·Differential Testing 사양서](specs/NOVA-142.md) | 전 Stage | Draft |
| NOVA-143 | 13_Testing_Release | [성능 Benchmark·회귀 기준서](specs/NOVA-143.md) | 전 Stage | Draft |
| NOVA-144 | 13_Testing_Release | [메모리 안전성 검증 전략서](specs/NOVA-144.md) | 전 Stage | Draft |
| NOVA-145 | 13_Testing_Release | [Unsafe·FFI 보안 검토 기준서](specs/NOVA-145.md) | 전 Stage | Draft |
| NOVA-146 | 13_Testing_Release | [0.1 Release Checklist](specs/NOVA-146.md) | 전 Stage | Draft |
| NOVA-147 | 13_Testing_Release | [언어 호환성 Test Suite 사양서](specs/NOVA-147.md) | 전 Stage | Draft |
| NOVA-148 | 13_Testing_Release | [Self-hosting 계획서](specs/NOVA-148.md) | 제외/후속 | Draft |
