# Nova Documentation Pack v0.1 — Master Index

Nova 0.1 언어·Compiler·Runtime·표준 라이브러리·도구 개발을 위한 148개 문서다.

## 가장 먼저 읽을 문서

1. NOVA-002 MVP 기능 동결표
2. NOVA-004 Canonical 표
3. NOVA-068 Compiler 전체 아키텍처
4. NOVA-069 저장소·Crate 구조
5. NOVA-070 Source Manager
6. NOVA-071 Lexer
7. NOVA-072 Parser
8. NOVA-073 AST
9. NOVA-074 HIR
10. NOVA-078 진단 시스템
11. NOVA-136 테스트 전략

## 전체 목록

| 번호 | 분류 | 문서 | 시점 | 상태 |
|---:|---|---|---|---|
| 001 | 00_Governance | [Nova 언어 설계 원칙·목표·비목표](00_Governance/NOVA-001_Nova_언어_설계_원칙_목표_비목표.md) | 착수 전 | 확정 |
| 002 | 00_Governance | [Nova 0.1 MVP 기능 동결표](00_Governance/NOVA-002_Nova_0.1_MVP_기능_동결표.md) | 착수 전 | 확정 |
| 003 | 00_Governance | [Nova 0.1 비지원 기능 목록](00_Governance/NOVA-003_Nova_0.1_비지원_기능_목록.md) | 착수 전 | 확정 |
| 004 | 00_Governance | [Nova 용어·키워드 Canonical 표](00_Governance/NOVA-004_Nova_용어_키워드_Canonical_표.md) | 착수 전 | 확정 |
| 005 | 00_Governance | [Nova 문서 인덱스·의존 관계도](00_Governance/NOVA-005_Nova_문서_인덱스_의존_관계도.md) | 착수 전 | 확정 |
| 006 | 00_Governance | [Nova 언어 변경 제안·결정 기록 절차](00_Governance/NOVA-006_Nova_언어_변경_제안_결정_기록_절차.md) | 구현 병행 | 확정 |
| 007 | 00_Governance | [Nova 버전 정책·호환성 원칙](00_Governance/NOVA-007_Nova_버전_정책_호환성_원칙.md) | MVP 후반 | 확정 |
| 008 | 01_Source_Syntax | [소스 인코딩·Unicode·줄바꿈 사양서](01_Source_Syntax/NOVA-008_소스_인코딩_Unicode_줄바꿈_사양서.md) | 착수 전 | 확정 |
| 009 | 01_Source_Syntax | [Token 종류·예약어·연산자 사양서](01_Source_Syntax/NOVA-009_Token_종류_예약어_연산자_사양서.md) | 착수 전 | 확정 |
| 010 | 01_Source_Syntax | [숫자·문자·문자열 Literal 사양서](01_Source_Syntax/NOVA-010_숫자_문자_문자열_Literal_사양서.md) | 착수 전 | 확정 |
| 011 | 01_Source_Syntax | [문자열 보간 Lexer Mode 사양서](01_Source_Syntax/NOVA-011_문자열_보간_Lexer_Mode_사양서.md) | Lexer 전 | 확정 |
| 012 | 01_Source_Syntax | [주석·중첩 주석 사양서](01_Source_Syntax/NOVA-012_주석_중첩_주석_사양서.md) | Lexer 전 | 확정 |
| 013 | 01_Source_Syntax | [줄바꿈·문장 종료·연속 줄 규칙](01_Source_Syntax/NOVA-013_줄바꿈_문장_종료_연속_줄_규칙.md) | Parser 전 | 확정 |
| 014 | 01_Source_Syntax | [문법 명세 및 EBNF 사양서](01_Source_Syntax/NOVA-014_문법_명세_및_EBNF_사양서.md) | Parser 전 | 확정 |
| 015 | 01_Source_Syntax | [연산자 우선순위·결합 방향 사양서](01_Source_Syntax/NOVA-015_연산자_우선순위_결합_방향_사양서.md) | Parser 전 | 확정 |
| 016 | 01_Source_Syntax | [Parser·AST 사양서](01_Source_Syntax/NOVA-016_Parser_AST_사양서.md) | 착수 전 | 확정 |
| 017 | 01_Source_Syntax | [Parser 오류 복구 사양서](01_Source_Syntax/NOVA-017_Parser_오류_복구_사양서.md) | 착수 전 | 확정 |
| 018 | 01_Source_Syntax | [소스 포매팅 기준서](01_Source_Syntax/NOVA-018_소스_포매팅_기준서.md) | Formatter 전 | 확정 |
| 019 | 02_Names_Modules | [이름 해석·Scope 사양서](02_Names_Modules/NOVA-019_이름_해석_Scope_사양서.md) | 의미 분석 전 | 확정 |
| 020 | 02_Names_Modules | [Module·Import 사양서](02_Names_Modules/NOVA-020_Module_Import_사양서.md) | 의미 분석 전 | 확정 |
| 021 | 02_Names_Modules | [접근 제한·가시성 사양서](02_Names_Modules/NOVA-021_접근_제한_가시성_사양서.md) | 의미 분석 전 | 확정 |
| 022 | 02_Names_Modules | [Symbol·Definition ID 사양서](02_Names_Modules/NOVA-022_Symbol_Definition_ID_사양서.md) | HIR 전 | 확정 |
| 023 | 02_Names_Modules | [Package 간 이름 해석 사양서](02_Names_Modules/NOVA-023_Package_간_이름_해석_사양서.md) | 패키지 기능 전 | 확정 |
| 024 | 02_Names_Modules | [이름 충돌·Shadowing 진단 기준서](02_Names_Modules/NOVA-024_이름_충돌_Shadowing_진단_기준서.md) | 의미 분석 전 | 확정 |
| 025 | 03_Types_Declarations | [타입 시스템·타입 추론·형변환 사양서](03_Types_Declarations/NOVA-025_타입_시스템_타입_추론_형변환_사양서.md) | 타입 검사 전 | 확정 |
| 026 | 03_Types_Declarations | [Primitive·Literal 기본 타입 결정 사양서](03_Types_Declarations/NOVA-026_Primitive_Literal_기본_타입_결정_사양서.md) | 타입 검사 전 | 확정 |
| 027 | 03_Types_Declarations | [Struct·Class·Enum 선언 사양서](03_Types_Declarations/NOVA-027_Struct_Class_Enum_선언_사양서.md) | 타입 검사 전 | 확정 |
| 028 | 03_Types_Declarations | [Nullable·Option·Result 타입 규칙](03_Types_Declarations/NOVA-028_Nullable_Option_Result_타입_규칙.md) | 타입 검사 전 | 확정 |
| 029 | 03_Types_Declarations | [Tuple·Array·Function 타입 사양서](03_Types_Declarations/NOVA-029_Tuple_Array_Function_타입_사양서.md) | 타입 검사 전 | 확정 |
| 030 | 03_Types_Declarations | [Type Alias·타입 정규화 사양서](03_Types_Declarations/NOVA-030_Type_Alias_타입_정규화_사양서.md) | 타입 검사 전 | 확정 |
| 031 | 03_Types_Declarations | [생성자·필드 초기화 사양서](03_Types_Declarations/NOVA-031_생성자_필드_초기화_사양서.md) | 생성자 구현 전 | 확정 |
| 032 | 03_Types_Declarations | [상수 표현식·const 평가 사양서](03_Types_Declarations/NOVA-032_상수_표현식_const_평가_사양서.md) | Const 구현 전 | 확정 |
| 033 | 03_Types_Declarations | [타입 레이아웃·정렬·Niche 사양서](03_Types_Declarations/NOVA-033_타입_레이아웃_정렬_Niche_사양서.md) | Backend 전 | 확정 |
| 034 | 03_Types_Declarations | [Class 객체 모델·정체성·배치 사양서](03_Types_Declarations/NOVA-034_Class_객체_모델_정체성_배치_사양서.md) | Class 구현 전 | 확정 |
| 035 | 04_Functions_Control | [함수·메서드·호출 규약·Receiver 사양서](04_Functions_Control/NOVA-035_함수_메서드_호출_규약_Receiver_사양서.md) | 함수 구현 전 | 확정 |
| 036 | 04_Functions_Control | [위치·이름·기본 인수 사양서](04_Functions_Control/NOVA-036_위치_이름_기본_인수_사양서.md) | 함수 구현 전 | 확정 |
| 037 | 04_Functions_Control | [Overload 해석·변환 비용 사양서](04_Functions_Control/NOVA-037_Overload_해석_변환_비용_사양서.md) | 호출 해석 전 | 확정 |
| 038 | 04_Functions_Control | [함수 타입·Lambda·Closure 사양서](04_Functions_Control/NOVA-038_함수_타입_Lambda_Closure_사양서.md) | Lambda 구현 전 | 확정 |
| 039 | 04_Functions_Control | [Closure Capture·호출 Receiver 사양서](04_Functions_Control/NOVA-039_Closure_Capture_호출_Receiver_사양서.md) | Lambda 구현 전 | 확정 |
| 040 | 04_Functions_Control | [main·프로그램 진입점 사양서](04_Functions_Control/NOVA-040_main_프로그램_진입점_사양서.md) | 실행 파일 전 | 확정 |
| 041 | 04_Functions_Control | [Effect·pure·noPanic 사양서](04_Functions_Control/NOVA-041_Effect_pure_noPanic_사양서.md) | Effect 구현 전 | 확정 |
| 042 | 04_Functions_Control | [Native Calling Convention 사양서](04_Functions_Control/NOVA-042_Native_Calling_Convention_사양서.md) | Backend 전 | 확정 |
| 043 | 04_Functions_Control | [if·while·for·loop 사양서](04_Functions_Control/NOVA-043_if_while_for_loop_사양서.md) | Parser·MIR 전 | 확정 |
| 044 | 04_Functions_Control | [break·continue·return 사양서](04_Functions_Control/NOVA-044_break_continue_return_사양서.md) | MIR 전 | 확정 |
| 045 | 04_Functions_Control | [Range until·through 사양서](04_Functions_Control/NOVA-045_Range_until_through_사양서.md) | Parser 전 | 확정 |
| 046 | 04_Functions_Control | [Pattern 문법·Binding 사양서](04_Functions_Control/NOVA-046_Pattern_문법_Binding_사양서.md) | Match 구현 전 | 확정 |
| 047 | 04_Functions_Control | [Match 완전성·도달 불가 Arm 분석서](04_Functions_Control/NOVA-047_Match_완전성_도달_불가_Arm_분석서.md) | Match 구현 전 | 확정 |
| 048 | 04_Functions_Control | [Match Lowering·Decision Tree 사양서](04_Functions_Control/NOVA-048_Match_Lowering_Decision_Tree_사양서.md) | MIR 전 | 확정 |
| 049 | 04_Functions_Control | [try·Result 전파 Lowering 사양서](04_Functions_Control/NOVA-049_try_Result_전파_Lowering_사양서.md) | MIR 전 | 확정 |
| 050 | 04_Functions_Control | [using Lowering 사양서](04_Functions_Control/NOVA-050_using_Lowering_사양서.md) | MIR 전 | 확정 |
| 051 | 05_Ownership_Safety | [메모리·소유권 모델 사양서](05_Ownership_Safety/NOVA-051_메모리_소유권_모델_사양서.md) | 타입 검사 전 | 확정 |
| 052 | 05_Ownership_Safety | [초기화·이동 상태 분석 사양서](05_Ownership_Safety/NOVA-052_초기화_이동_상태_분석_사양서.md) | MIR 분석 전 | 확정 |
| 053 | 05_Ownership_Safety | [부분 이동 정책 사양서](05_Ownership_Safety/NOVA-053_부분_이동_정책_사양서.md) | MIR 분석 전 | 확정 |
| 054 | 05_Ownership_Safety | [빌림·수명·Place 충돌 사양서](05_Ownership_Safety/NOVA-054_빌림_수명_Place_충돌_사양서.md) | Borrow Checker 전 | 확정 |
| 055 | 05_Ownership_Safety | [Drop 정교화·Drop Flag 사양서](05_Ownership_Safety/NOVA-055_Drop_정교화_Drop_Flag_사양서.md) | Drop 구현 전 | 확정 |
| 056 | 05_Ownership_Safety | [사용자 drop·Drop Glue 사양서](05_Ownership_Safety/NOVA-056_사용자_drop_Drop_Glue_사양서.md) | Drop 구현 전 | 확정 |
| 057 | 05_Ownership_Safety | [Shared·Weak·참조 횟수 모델 사양서](05_Ownership_Safety/NOVA-057_Shared_Weak_참조_횟수_모델_사양서.md) | Runtime 전 | 확정 |
| 058 | 05_Ownership_Safety | [Raw Pointer·Unsafe Capability 사양서](05_Ownership_Safety/NOVA-058_Raw_Pointer_Unsafe_Capability_사양서.md) | FFI 전 | 확정 |
| 059 | 05_Ownership_Safety | [Pinning·자기 참조 타입 사양서](05_Ownership_Safety/NOVA-059_Pinning_자기_참조_타입_사양서.md) | MVP 이후 | 후속 |
| 060 | 05_Ownership_Safety | [Thread 이동·공유 안전성 사양서](05_Ownership_Safety/NOVA-060_Thread_이동_공유_안전성_사양서.md) | Thread 구현 전 | 확정 |
| 061 | 06_Interfaces_Generics | [Interface·구현·정적 디스패치 사양서](06_Interfaces_Generics/NOVA-061_Interface_구현_정적_디스패치_사양서.md) | Interface 전 | 확정 |
| 062 | 06_Interfaces_Generics | [Generic 제약 증명 사양서](06_Interfaces_Generics/NOVA-062_Generic_제약_증명_사양서.md) | Generic 전 | 확정 |
| 063 | 06_Interfaces_Generics | [Generic 타입 추론 사양서](06_Interfaces_Generics/NOVA-063_Generic_타입_추론_사양서.md) | Generic 전 | 확정 |
| 064 | 06_Interfaces_Generics | [Monomorphization·특수화 사양서](06_Interfaces_Generics/NOVA-064_Monomorphization_특수화_사양서.md) | Generic 전 | 확정 |
| 065 | 06_Interfaces_Generics | [Generic Cache·코드 팽창 제어 사양서](06_Interfaces_Generics/NOVA-065_Generic_Cache_코드_팽창_제어_사양서.md) | 최적화 전 | 확정 |
| 066 | 06_Interfaces_Generics | [Associated Type·Dynamic Interface Object 사양서](06_Interfaces_Generics/NOVA-066_Associated_Type_Dynamic_Interface_Object_사양서.md) | MVP 이후 | 후속 |
| 067 | 06_Interfaces_Generics | [VTable·Object Safety 사양서](06_Interfaces_Generics/NOVA-067_VTable_Object_Safety_사양서.md) | MVP 이후 | 후속 |
| 068 | 07_Compiler_Frontend | [Compiler 전체 아키텍처 사양서](07_Compiler_Frontend/NOVA-068_Compiler_전체_아키텍처_사양서.md) | 착수 전 | 확정 |
| 069 | 07_Compiler_Frontend | [저장소·Crate·모듈 구조 사양서](07_Compiler_Frontend/NOVA-069_저장소_Crate_모듈_구조_사양서.md) | 착수 전 | 확정 |
| 070 | 07_Compiler_Frontend | [Source Manager·File ID·Span 사양서](07_Compiler_Frontend/NOVA-070_Source_Manager_File_ID_Span_사양서.md) | 착수 전 | 확정 |
| 071 | 07_Compiler_Frontend | [Lexer 구현 사양서](07_Compiler_Frontend/NOVA-071_Lexer_구현_사양서.md) | 착수 전 | 확정 |
| 072 | 07_Compiler_Frontend | [Parser 구현 사양서](07_Compiler_Frontend/NOVA-072_Parser_구현_사양서.md) | 착수 전 | 확정 |
| 073 | 07_Compiler_Frontend | [AST Node 구조서](07_Compiler_Frontend/NOVA-073_AST_Node_구조서.md) | 착수 전 | 확정 |
| 074 | 07_Compiler_Frontend | [HIR Node 구조서](07_Compiler_Frontend/NOVA-074_HIR_Node_구조서.md) | 착수 전 | 확정 |
| 075 | 07_Compiler_Frontend | [AST→HIR Lowering 사양서](07_Compiler_Frontend/NOVA-075_AST_HIR_Lowering_사양서.md) | HIR 전 | 확정 |
| 076 | 07_Compiler_Frontend | [Symbol Table·Definition Registry 사양서](07_Compiler_Frontend/NOVA-076_Symbol_Table_Definition_Registry_사양서.md) | 이름 해석 전 | 확정 |
| 077 | 07_Compiler_Frontend | [타입 검사기·소유권 검사기 아키텍처 사양서](07_Compiler_Frontend/NOVA-077_타입_검사기_소유권_검사기_아키텍처_사양서.md) | 타입 검사 전 | 확정 |
| 078 | 07_Compiler_Frontend | [진단 시스템·Error Code 관리 사양서](07_Compiler_Frontend/NOVA-078_진단_시스템_Error_Code_관리_사양서.md) | 착수 전 | 확정 |
| 079 | 07_Compiler_Frontend | [Compiler Query·의존성·Cache 사양서](07_Compiler_Frontend/NOVA-079_Compiler_Query_의존성_Cache_사양서.md) | 증분 기능 전 | 확정 |
| 080 | 07_Compiler_Frontend | [Compiler 내부 오류·ICE 처리 사양서](07_Compiler_Frontend/NOVA-080_Compiler_내부_오류_ICE_처리_사양서.md) | 구현 병행 | 확정 |
| 081 | 08_MIR_Middleend | [MIR 구조·CFG·평가 순서 사양서](08_MIR_Middleend/NOVA-081_MIR_구조_CFG_평가_순서_사양서.md) | MIR 전 | 확정 |
| 082 | 08_MIR_Middleend | [HIR→MIR Lowering 사양서](08_MIR_Middleend/NOVA-082_HIR_MIR_Lowering_사양서.md) | MIR 전 | 확정 |
| 083 | 08_MIR_Middleend | [MIR 검증기 사양서](08_MIR_Middleend/NOVA-083_MIR_검증기_사양서.md) | MIR 전 | 확정 |
| 084 | 08_MIR_Middleend | [초기화·Move 분석 구현서](08_MIR_Middleend/NOVA-084_초기화_Move_분석_구현서.md) | MIR 분석 전 | 확정 |
| 085 | 08_MIR_Middleend | [Borrow Checker 구현서](08_MIR_Middleend/NOVA-085_Borrow_Checker_구현서.md) | Borrow 전 | 확정 |
| 086 | 08_MIR_Middleend | [Drop Elaboration 구현서](08_MIR_Middleend/NOVA-086_Drop_Elaboration_구현서.md) | Drop 전 | 확정 |
| 087 | 08_MIR_Middleend | [Constant Evaluation Engine 사양서](08_MIR_Middleend/NOVA-087_Constant_Evaluation_Engine_사양서.md) | Const 전 | 확정 |
| 088 | 08_MIR_Middleend | [Match Lowering 구현서](08_MIR_Middleend/NOVA-088_Match_Lowering_구현서.md) | Match 전 | 확정 |
| 089 | 08_MIR_Middleend | [Generic 특수화 Pass 구현서](08_MIR_Middleend/NOVA-089_Generic_특수화_Pass_구현서.md) | Generic 전 | 확정 |
| 090 | 08_MIR_Middleend | [MIR 최적화 Pass 목록·순서 사양서](08_MIR_Middleend/NOVA-090_MIR_최적화_Pass_목록_순서_사양서.md) | 최적화 전 | 확정 |
| 091 | 08_MIR_Middleend | [SSA 변환 여부 결정서](08_MIR_Middleend/NOVA-091_SSA_변환_여부_결정서.md) | 최적화 전 | 확정 |
| 092 | 08_MIR_Middleend | [Debug MIR 출력 형식 사양서](08_MIR_Middleend/NOVA-092_Debug_MIR_출력_형식_사양서.md) | 구현 병행 | 확정 |
| 093 | 09_Backend_Runtime | [Backend 선택·LLVM 연동 결정서](09_Backend_Runtime/NOVA-093_Backend_선택_LLVM_연동_결정서.md) | Backend 전 | 확정 |
| 094 | 09_Backend_Runtime | [Nova 타입→Backend 타입 Mapping 사양서](09_Backend_Runtime/NOVA-094_Nova_타입_Backend_타입_Mapping_사양서.md) | Backend 전 | 확정 |
| 095 | 09_Backend_Runtime | [Backend 타입 레이아웃·정렬·Niche 구현서](09_Backend_Runtime/NOVA-095_Backend_타입_레이아웃_정렬_Niche_구현서.md) | Backend 전 | 확정 |
| 096 | 09_Backend_Runtime | [함수 ABI·인수·반환 Lowering 사양서](09_Backend_Runtime/NOVA-096_함수_ABI_인수_반환_Lowering_사양서.md) | Backend 전 | 확정 |
| 097 | 09_Backend_Runtime | [Name Mangling·Symbol 사양서](09_Backend_Runtime/NOVA-097_Name_Mangling_Symbol_사양서.md) | Link 전 | 확정 |
| 098 | 09_Backend_Runtime | [Object File·Linker 연동 사양서](09_Backend_Runtime/NOVA-098_Object_File_Linker_연동_사양서.md) | Link 전 | 확정 |
| 099 | 09_Backend_Runtime | [Runtime Startup·main 호출 사양서](09_Backend_Runtime/NOVA-099_Runtime_Startup_main_호출_사양서.md) | 실행 전 | 확정 |
| 100 | 09_Backend_Runtime | [메모리 Allocator·OOM 정책 사양서](09_Backend_Runtime/NOVA-100_메모리_Allocator_OOM_정책_사양서.md) | Heap 사용 전 | 확정 |
| 101 | 09_Backend_Runtime | [Panic Runtime·Stack Trace 사양서](09_Backend_Runtime/NOVA-101_Panic_Runtime_Stack_Trace_사양서.md) | Panic 구현 전 | 확정 |
| 102 | 09_Backend_Runtime | [Shared·Weak Runtime 사양서](09_Backend_Runtime/NOVA-102_Shared_Weak_Runtime_사양서.md) | Shared 구현 전 | 확정 |
| 103 | 09_Backend_Runtime | [Thread·Atomic·Lock Runtime 사양서](09_Backend_Runtime/NOVA-103_Thread_Atomic_Lock_Runtime_사양서.md) | MVP 후반 | 확정 |
| 104 | 09_Backend_Runtime | [Platform Abstraction Layer 사양서](09_Backend_Runtime/NOVA-104_Platform_Abstraction_Layer_사양서.md) | 다중 OS 전 | 확정 |
| 105 | 10_FFI | [foreign 선언 문법·공통 모델 사양서](10_FFI/NOVA-105_foreign_선언_문법_공통_모델_사양서.md) | FFI 전 | 확정 |
| 106 | 10_FFI | [C ABI Import·Export 사양서](10_FFI/NOVA-106_C_ABI_Import_Export_사양서.md) | FFI 전 | 확정 |
| 107 | 10_FFI | [C Header Parser·Binding Generator 사양서](10_FFI/NOVA-107_C_Header_Parser_Binding_Generator_사양서.md) | FFI 후반 | 확정 |
| 108 | 10_FFI | [C++ Bridge·Opaque Handle 사양서](10_FFI/NOVA-108_C_Bridge_Opaque_Handle_사양서.md) | MVP 이후 가능 | 후속 |
| 109 | 10_FFI | [외부 예외→Result 변환 사양서](10_FFI/NOVA-109_외부_예외_Result_변환_사양서.md) | FFI 전 | 확정 |
| 110 | 10_FFI | [Foreign Ownership·Drop 계약 사양서](10_FFI/NOVA-110_Foreign_Ownership_Drop_계약_사양서.md) | FFI 전 | 확정 |
| 111 | 10_FFI | [.NET Hosted Adapter 사양서](10_FFI/NOVA-111_.NET_Hosted_Adapter_사양서.md) | MVP 이후 | 후속 |
| 112 | 10_FFI | [JVM Hosted Adapter 사양서](10_FFI/NOVA-112_JVM_Hosted_Adapter_사양서.md) | MVP 이후 | 후속 |
| 113 | 10_FFI | [Python Hosted Adapter 사양서](10_FFI/NOVA-113_Python_Hosted_Adapter_사양서.md) | MVP 이후 | 후속 |
| 114 | 11_Standard_Library | [Core Prelude·기본 Symbol 사양서](11_Standard_Library/NOVA-114_Core_Prelude_기본_Symbol_사양서.md) | 첫 실행 전 | 확정 |
| 115 | 11_Standard_Library | [Primitive 메서드 API 사양서](11_Standard_Library/NOVA-115_Primitive_메서드_API_사양서.md) | Core 구현 전 | 확정 |
| 116 | 11_Standard_Library | [string·UTF-8 API 사양서](11_Standard_Library/NOVA-116_string_UTF-8_API_사양서.md) | 문자열 구현 전 | 확정 |
| 117 | 11_Standard_Library | [Array<T> API·메모리 모델 사양서](11_Standard_Library/NOVA-117_Array_T_API_메모리_모델_사양서.md) | Array 구현 전 | 확정 |
| 118 | 11_Standard_Library | [Span<T>·ReadOnlySpan<T> 사양서](11_Standard_Library/NOVA-118_Span_T_ReadOnlySpan_T_사양서.md) | View 구현 전 | 확정 |
| 119 | 11_Standard_Library | [Option<T> API 사양서](11_Standard_Library/NOVA-119_Option_T_API_사양서.md) | Match 구현 전 | 확정 |
| 120 | 11_Standard_Library | [Result<T,E>·try API 사양서](11_Standard_Library/NOVA-120_Result_TE_try_API_사양서.md) | Error 처리 전 | 확정 |
| 121 | 11_Standard_Library | [List<T>·Map<K,V> 사양서](11_Standard_Library/NOVA-121_List_T_Map_KV_사양서.md) | 기본 실행 후 | 확정 |
| 122 | 11_Standard_Library | [파일·Stream·Console I/O 사양서](11_Standard_Library/NOVA-122_파일_Stream_Console_I_O_사양서.md) | 실행 프로그램 전 | 확정 |
| 123 | 11_Standard_Library | [시간·난수·환경 변수 사양서](11_Standard_Library/NOVA-123_시간_난수_환경_변수_사양서.md) | MVP 후반 | 확정 |
| 124 | 11_Standard_Library | [Thread·Lock·Atomic API 사양서](11_Standard_Library/NOVA-124_Thread_Lock_Atomic_API_사양서.md) | MVP 이후 가능 | 후속 |
| 125 | 12_Tooling_Packaging | [Compiler CLI 명령·옵션 사양서](12_Tooling_Packaging/NOVA-125_Compiler_CLI_명령_옵션_사양서.md) | 첫 실행 전 | 확정 |
| 126 | 12_Tooling_Packaging | [Package Manifest 사양서](12_Tooling_Packaging/NOVA-126_Package_Manifest_사양서.md) | 다중 패키지 전 | 확정 |
| 127 | 12_Tooling_Packaging | [의존성 해석·Lock File 사양서](12_Tooling_Packaging/NOVA-127_의존성_해석_Lock_File_사양서.md) | 패키지 관리자 전 | 확정 |
| 128 | 12_Tooling_Packaging | [Build Profile·Target 사양서](12_Tooling_Packaging/NOVA-128_Build_Profile_Target_사양서.md) | Backend 전 | 확정 |
| 129 | 12_Tooling_Packaging | [Package Artifact·Metadata 형식 사양서](12_Tooling_Packaging/NOVA-129_Package_Artifact_Metadata_형식_사양서.md) | 외부 패키지 전 | 확정 |
| 130 | 12_Tooling_Packaging | [증분 컴파일 Cache 형식 사양서](12_Tooling_Packaging/NOVA-130_증분_컴파일_Cache_형식_사양서.md) | 증분 기능 전 | 확정 |
| 131 | 12_Tooling_Packaging | [Formatter 사양·구현서](12_Tooling_Packaging/NOVA-131_Formatter_사양_구현서.md) | Parser 안정 후 | 확정 |
| 132 | 12_Tooling_Packaging | [Linter 규칙 사양서](12_Tooling_Packaging/NOVA-132_Linter_규칙_사양서.md) | MVP 후반 | 확정 |
| 133 | 12_Tooling_Packaging | [Language Server·LSP 사양서](12_Tooling_Packaging/NOVA-133_Language_Server_LSP_사양서.md) | MVP 이후 | 후속 |
| 134 | 12_Tooling_Packaging | [API 문서 생성기 사양서](12_Tooling_Packaging/NOVA-134_API_문서_생성기_사양서.md) | MVP 이후 | 후속 |
| 135 | 12_Tooling_Packaging | [Package Registry 사양서](12_Tooling_Packaging/NOVA-135_Package_Registry_사양서.md) | MVP 이후 | 후속 |
| 136 | 13_Testing_Release | [Compiler 테스트 전략서](13_Testing_Release/NOVA-136_Compiler_테스트_전략서.md) | 착수 전 | 확정 |
| 137 | 13_Testing_Release | [Lexer·Parser Snapshot 테스트 사양서](13_Testing_Release/NOVA-137_Lexer_Parser_Snapshot_테스트_사양서.md) | Lexer와 병행 | 확정 |
| 138 | 13_Testing_Release | [Compile-pass·Compile-fail 규격](13_Testing_Release/NOVA-138_Compile-pass_Compile-fail_규격.md) | 타입 검사 전 | 확정 |
| 139 | 13_Testing_Release | [MIR Snapshot 규격](13_Testing_Release/NOVA-139_MIR_Snapshot_규격.md) | MIR 전 | 확정 |
| 140 | 13_Testing_Release | [Runtime Test Harness 사양서](13_Testing_Release/NOVA-140_Runtime_Test_Harness_사양서.md) | Runtime 전 | 확정 |
| 141 | 13_Testing_Release | [Lexer·Parser·MIR Fuzzing 사양서](13_Testing_Release/NOVA-141_Lexer_Parser_MIR_Fuzzing_사양서.md) | 각 단계 병행 | 확정 |
| 142 | 13_Testing_Release | [Property·Differential Testing 사양서](13_Testing_Release/NOVA-142_Property_Differential_Testing_사양서.md) | MVP 후반 | 확정 |
| 143 | 13_Testing_Release | [성능 Benchmark·회귀 기준서](13_Testing_Release/NOVA-143_성능_Benchmark_회귀_기준서.md) | Backend 후 | 확정 |
| 144 | 13_Testing_Release | [메모리 안전성 검증 전략서](13_Testing_Release/NOVA-144_메모리_안전성_검증_전략서.md) | Ownership 구현 후 | 확정 |
| 145 | 13_Testing_Release | [Unsafe·FFI 보안 검토 기준서](13_Testing_Release/NOVA-145_Unsafe_FFI_보안_검토_기준서.md) | FFI 전 | 확정 |
| 146 | 13_Testing_Release | [0.1 Release Checklist](13_Testing_Release/NOVA-146_0.1_Release_Checklist.md) | 배포 전 | 확정 |
| 147 | 13_Testing_Release | [언어 호환성 Test Suite 사양서](13_Testing_Release/NOVA-147_언어_호환성_Test_Suite_사양서.md) | 배포 전 | 확정 |
| 148 | 13_Testing_Release | [Self-hosting 계획서](13_Testing_Release/NOVA-148_Self-hosting_계획서.md) | MVP 이후 | 후속 |
