# 원본 사양 감사와 보완 경계

원본 148개 NOVA 문서를 기준으로 새 계약을 작성했다. '공통 양식 포함'은 자동 탐지한 구조 분류이며 그 문서 전체가 무효라는 판정은 아니다. 원본 파일은 수정하지 않았다.

## 구현 전 핵심 문제

- NOVA-014: 실제 EBNF Production 부재 → GRAMMAR 초안 작성, D01~D05 승인 완료; 이후 Parser 관련 결정은 승인 필요.
- NOVA-004 vs 072: use keyword 누락 → D01; alias/lambda/noPanic contextual 표기도 검토.
- NOVA-070: compiler source Span과 runtime Span<T> 항목 혼재 → 담당 문서 070/118 구분(D10).
- NOVA-026/037: numeric widening/default/overload 비용 불완전 → D07/D11.
- NOVA-029/117/121: growable Array와 List 역할 중복 → D23.
- NOVA-059/066/067/135/148: 비지원 기능의 문서 존재는 구현 허가가 아님.
- NOVA-111~113: hosted adapters는 C FFI 동결표에 없음 → D27 후속 제안.
- 사양 날짜는 원본 값 그대로; 새 작성일은 2026-10-03.

| 원본 | 조사 분류 | 보완 문서 |
|---|---|---|
| [NOVA-001](../00_Governance/NOVA-001_Nova_언어_설계_원칙_목표_비목표.md) | 공통 양식 포함; 주제별 정의 보완 | [작성 문서](specs/NOVA-001.md) |
| [NOVA-002](../00_Governance/NOVA-002_Nova_0.1_MVP_기능_동결표.md) | 전용 요약; 상세 계약/검증 보완 | [작성 문서](specs/NOVA-002.md) |
| [NOVA-003](../00_Governance/NOVA-003_Nova_0.1_비지원_기능_목록.md) | 공통 양식 포함; 주제별 정의 보완 | [작성 문서](specs/NOVA-003.md) |
| [NOVA-004](../00_Governance/NOVA-004_Nova_용어_키워드_Canonical_표.md) | 전용 요약; 상세 계약/검증 보완 | [작성 문서](specs/NOVA-004.md) |
| [NOVA-005](../00_Governance/NOVA-005_Nova_문서_인덱스_의존_관계도.md) | 공통 양식 포함; 주제별 정의 보완 | [작성 문서](specs/NOVA-005.md) |
| [NOVA-006](../00_Governance/NOVA-006_Nova_언어_변경_제안_결정_기록_절차.md) | 공통 양식 포함; 주제별 정의 보완 | [작성 문서](specs/NOVA-006.md) |
| [NOVA-007](../00_Governance/NOVA-007_Nova_버전_정책_호환성_원칙.md) | 공통 양식 포함; 주제별 정의 보완 | [작성 문서](specs/NOVA-007.md) |
| [NOVA-008](../01_Source_Syntax/NOVA-008_소스_인코딩_Unicode_줄바꿈_사양서.md) | 공통 양식 포함; 주제별 정의 보완 | [작성 문서](specs/NOVA-008.md) |
| [NOVA-009](../01_Source_Syntax/NOVA-009_Token_종류_예약어_연산자_사양서.md) | 공통 양식 포함; 주제별 정의 보완 | [작성 문서](specs/NOVA-009.md) |
| [NOVA-010](../01_Source_Syntax/NOVA-010_숫자_문자_문자열_Literal_사양서.md) | 공통 양식 포함; 주제별 정의 보완 | [작성 문서](specs/NOVA-010.md) |
| [NOVA-011](../01_Source_Syntax/NOVA-011_문자열_보간_Lexer_Mode_사양서.md) | 전용 요약; 상세 계약/검증 보완 | [작성 문서](specs/NOVA-011.md) |
| [NOVA-012](../01_Source_Syntax/NOVA-012_주석_중첩_주석_사양서.md) | 공통 양식 포함; 주제별 정의 보완 | [작성 문서](specs/NOVA-012.md) |
| [NOVA-013](../01_Source_Syntax/NOVA-013_줄바꿈_문장_종료_연속_줄_규칙.md) | 공통 양식 포함; 주제별 정의 보완 | [작성 문서](specs/NOVA-013.md) |
| [NOVA-014](../01_Source_Syntax/NOVA-014_문법_명세_및_EBNF_사양서.md) | 공통 양식 포함; 주제별 정의 보완 | [작성 문서](specs/NOVA-014.md) |
| [NOVA-015](../01_Source_Syntax/NOVA-015_연산자_우선순위_결합_방향_사양서.md) | 공통 양식 포함; 주제별 정의 보완 | [작성 문서](specs/NOVA-015.md) |
| [NOVA-016](../01_Source_Syntax/NOVA-016_Parser_AST_사양서.md) | 전용 요약; 상세 계약/검증 보완 | [작성 문서](specs/NOVA-016.md) |
| [NOVA-017](../01_Source_Syntax/NOVA-017_Parser_오류_복구_사양서.md) | 전용 요약; 상세 계약/검증 보완 | [작성 문서](specs/NOVA-017.md) |
| [NOVA-018](../01_Source_Syntax/NOVA-018_소스_포매팅_기준서.md) | 공통 양식 포함; 주제별 정의 보완 | [작성 문서](specs/NOVA-018.md) |
| [NOVA-019](../02_Names_Modules/NOVA-019_이름_해석_Scope_사양서.md) | 전용 요약; 상세 계약/검증 보완 | [작성 문서](specs/NOVA-019.md) |
| [NOVA-020](../02_Names_Modules/NOVA-020_Module_Import_사양서.md) | 전용 요약; 상세 계약/검증 보완 | [작성 문서](specs/NOVA-020.md) |
| [NOVA-021](../02_Names_Modules/NOVA-021_접근_제한_가시성_사양서.md) | 공통 양식 포함; 주제별 정의 보완 | [작성 문서](specs/NOVA-021.md) |
| [NOVA-022](../02_Names_Modules/NOVA-022_Symbol_Definition_ID_사양서.md) | 공통 양식 포함; 주제별 정의 보완 | [작성 문서](specs/NOVA-022.md) |
| [NOVA-023](../02_Names_Modules/NOVA-023_Package_간_이름_해석_사양서.md) | 전용 요약; 상세 계약/검증 보완 | [작성 문서](specs/NOVA-023.md) |
| [NOVA-024](../02_Names_Modules/NOVA-024_이름_충돌_Shadowing_진단_기준서.md) | 전용 요약; 상세 계약/검증 보완 | [작성 문서](specs/NOVA-024.md) |
| [NOVA-025](../03_Types_Declarations/NOVA-025_타입_시스템_타입_추론_형변환_사양서.md) | 전용 요약; 상세 계약/검증 보완 | [작성 문서](specs/NOVA-025.md) |
| [NOVA-026](../03_Types_Declarations/NOVA-026_Primitive_Literal_기본_타입_결정_사양서.md) | 전용 요약; 상세 계약/검증 보완 | [작성 문서](specs/NOVA-026.md) |
| [NOVA-027](../03_Types_Declarations/NOVA-027_Struct_Class_Enum_선언_사양서.md) | 전용 요약; 상세 계약/검증 보완 | [작성 문서](specs/NOVA-027.md) |
| [NOVA-028](../03_Types_Declarations/NOVA-028_Nullable_Option_Result_타입_규칙.md) | 전용 요약; 상세 계약/검증 보완 | [작성 문서](specs/NOVA-028.md) |
| [NOVA-029](../03_Types_Declarations/NOVA-029_Tuple_Array_Function_타입_사양서.md) | 전용 요약; 상세 계약/검증 보완 | [작성 문서](specs/NOVA-029.md) |
| [NOVA-030](../03_Types_Declarations/NOVA-030_Type_Alias_타입_정규화_사양서.md) | 전용 요약; 상세 계약/검증 보완 | [작성 문서](specs/NOVA-030.md) |
| [NOVA-031](../03_Types_Declarations/NOVA-031_생성자_필드_초기화_사양서.md) | 공통 양식 포함; 주제별 정의 보완 | [작성 문서](specs/NOVA-031.md) |
| [NOVA-032](../03_Types_Declarations/NOVA-032_상수_표현식_const_평가_사양서.md) | 공통 양식 포함; 주제별 정의 보완 | [작성 문서](specs/NOVA-032.md) |
| [NOVA-033](../03_Types_Declarations/NOVA-033_타입_레이아웃_정렬_Niche_사양서.md) | 전용 요약; 상세 계약/검증 보완 | [작성 문서](specs/NOVA-033.md) |
| [NOVA-034](../03_Types_Declarations/NOVA-034_Class_객체_모델_정체성_배치_사양서.md) | 전용 요약; 상세 계약/검증 보완 | [작성 문서](specs/NOVA-034.md) |
| [NOVA-035](../04_Functions_Control/NOVA-035_함수_메서드_호출_규약_Receiver_사양서.md) | 전용 요약; 상세 계약/검증 보완 | [작성 문서](specs/NOVA-035.md) |
| [NOVA-036](../04_Functions_Control/NOVA-036_위치_이름_기본_인수_사양서.md) | 전용 요약; 상세 계약/검증 보완 | [작성 문서](specs/NOVA-036.md) |
| [NOVA-037](../04_Functions_Control/NOVA-037_Overload_해석_변환_비용_사양서.md) | 공통 양식 포함; 주제별 정의 보완 | [작성 문서](specs/NOVA-037.md) |
| [NOVA-038](../04_Functions_Control/NOVA-038_함수_타입_Lambda_Closure_사양서.md) | 전용 요약; 상세 계약/검증 보완 | [작성 문서](specs/NOVA-038.md) |
| [NOVA-039](../04_Functions_Control/NOVA-039_Closure_Capture_호출_Receiver_사양서.md) | 전용 요약; 상세 계약/검증 보완 | [작성 문서](specs/NOVA-039.md) |
| [NOVA-040](../04_Functions_Control/NOVA-040_main_프로그램_진입점_사양서.md) | 공통 양식 포함; 주제별 정의 보완 | [작성 문서](specs/NOVA-040.md) |
| [NOVA-041](../04_Functions_Control/NOVA-041_Effect_pure_noPanic_사양서.md) | 전용 요약; 상세 계약/검증 보완 | [작성 문서](specs/NOVA-041.md) |
| [NOVA-042](../04_Functions_Control/NOVA-042_Native_Calling_Convention_사양서.md) | 공통 양식 포함; 주제별 정의 보완 | [작성 문서](specs/NOVA-042.md) |
| [NOVA-043](../04_Functions_Control/NOVA-043_if_while_for_loop_사양서.md) | 공통 양식 포함; 주제별 정의 보완 | [작성 문서](specs/NOVA-043.md) |
| [NOVA-044](../04_Functions_Control/NOVA-044_break_continue_return_사양서.md) | 공통 양식 포함; 주제별 정의 보완 | [작성 문서](specs/NOVA-044.md) |
| [NOVA-045](../04_Functions_Control/NOVA-045_Range_until_through_사양서.md) | 공통 양식 포함; 주제별 정의 보완 | [작성 문서](specs/NOVA-045.md) |
| [NOVA-046](../04_Functions_Control/NOVA-046_Pattern_문법_Binding_사양서.md) | 공통 양식 포함; 주제별 정의 보완 | [작성 문서](specs/NOVA-046.md) |
| [NOVA-047](../04_Functions_Control/NOVA-047_Match_완전성_도달_불가_Arm_분석서.md) | 전용 요약; 상세 계약/검증 보완 | [작성 문서](specs/NOVA-047.md) |
| [NOVA-048](../04_Functions_Control/NOVA-048_Match_Lowering_Decision_Tree_사양서.md) | 전용 요약; 상세 계약/검증 보완 | [작성 문서](specs/NOVA-048.md) |
| [NOVA-049](../04_Functions_Control/NOVA-049_try_Result_전파_Lowering_사양서.md) | 전용 요약; 상세 계약/검증 보완 | [작성 문서](specs/NOVA-049.md) |
| [NOVA-050](../04_Functions_Control/NOVA-050_using_Lowering_사양서.md) | 공통 양식 포함; 주제별 정의 보완 | [작성 문서](specs/NOVA-050.md) |
| [NOVA-051](../05_Ownership_Safety/NOVA-051_메모리_소유권_모델_사양서.md) | 전용 요약; 상세 계약/검증 보완 | [작성 문서](specs/NOVA-051.md) |
| [NOVA-052](../05_Ownership_Safety/NOVA-052_초기화_이동_상태_분석_사양서.md) | 공통 양식 포함; 주제별 정의 보완 | [작성 문서](specs/NOVA-052.md) |
| [NOVA-053](../05_Ownership_Safety/NOVA-053_부분_이동_정책_사양서.md) | 공통 양식 포함; 주제별 정의 보완 | [작성 문서](specs/NOVA-053.md) |
| [NOVA-054](../05_Ownership_Safety/NOVA-054_빌림_수명_Place_충돌_사양서.md) | 전용 요약; 상세 계약/검증 보완 | [작성 문서](specs/NOVA-054.md) |
| [NOVA-055](../05_Ownership_Safety/NOVA-055_Drop_정교화_Drop_Flag_사양서.md) | 전용 요약; 상세 계약/검증 보완 | [작성 문서](specs/NOVA-055.md) |
| [NOVA-056](../05_Ownership_Safety/NOVA-056_사용자_drop_Drop_Glue_사양서.md) | 전용 요약; 상세 계약/검증 보완 | [작성 문서](specs/NOVA-056.md) |
| [NOVA-057](../05_Ownership_Safety/NOVA-057_Shared_Weak_참조_횟수_모델_사양서.md) | 전용 요약; 상세 계약/검증 보완 | [작성 문서](specs/NOVA-057.md) |
| [NOVA-058](../05_Ownership_Safety/NOVA-058_Raw_Pointer_Unsafe_Capability_사양서.md) | 전용 요약; 상세 계약/검증 보완 | [작성 문서](specs/NOVA-058.md) |
| [NOVA-059](../05_Ownership_Safety/NOVA-059_Pinning_자기_참조_타입_사양서.md) | 전용 요약; 상세 계약/검증 보완 | [작성 문서](specs/NOVA-059.md) |
| [NOVA-060](../05_Ownership_Safety/NOVA-060_Thread_이동_공유_안전성_사양서.md) | 공통 양식 포함; 주제별 정의 보완 | [작성 문서](specs/NOVA-060.md) |
| [NOVA-061](../06_Interfaces_Generics/NOVA-061_Interface_구현_정적_디스패치_사양서.md) | 전용 요약; 상세 계약/검증 보완 | [작성 문서](specs/NOVA-061.md) |
| [NOVA-062](../06_Interfaces_Generics/NOVA-062_Generic_제약_증명_사양서.md) | 전용 요약; 상세 계약/검증 보완 | [작성 문서](specs/NOVA-062.md) |
| [NOVA-063](../06_Interfaces_Generics/NOVA-063_Generic_타입_추론_사양서.md) | 전용 요약; 상세 계약/검증 보완 | [작성 문서](specs/NOVA-063.md) |
| [NOVA-064](../06_Interfaces_Generics/NOVA-064_Monomorphization_특수화_사양서.md) | 공통 양식 포함; 주제별 정의 보완 | [작성 문서](specs/NOVA-064.md) |
| [NOVA-065](../06_Interfaces_Generics/NOVA-065_Generic_Cache_코드_팽창_제어_사양서.md) | 전용 요약; 상세 계약/검증 보완 | [작성 문서](specs/NOVA-065.md) |
| [NOVA-066](../06_Interfaces_Generics/NOVA-066_Associated_Type_Dynamic_Interface_Object_사양서.md) | 전용 요약; 상세 계약/검증 보완 | [작성 문서](specs/NOVA-066.md) |
| [NOVA-067](../06_Interfaces_Generics/NOVA-067_VTable_Object_Safety_사양서.md) | 공통 양식 포함; 주제별 정의 보완 | [작성 문서](specs/NOVA-067.md) |
| [NOVA-068](../07_Compiler_Frontend/NOVA-068_Compiler_전체_아키텍처_사양서.md) | 전용 요약; 상세 계약/검증 보완 | [작성 문서](specs/NOVA-068.md) |
| [NOVA-069](../07_Compiler_Frontend/NOVA-069_저장소_Crate_모듈_구조_사양서.md) | 전용 요약; 상세 계약/검증 보완 | [작성 문서](specs/NOVA-069.md) |
| [NOVA-070](../07_Compiler_Frontend/NOVA-070_Source_Manager_File_ID_Span_사양서.md) | 전용 요약; 상세 계약/검증 보완 | [작성 문서](specs/NOVA-070.md) |
| [NOVA-071](../07_Compiler_Frontend/NOVA-071_Lexer_구현_사양서.md) | 전용 요약; 상세 계약/검증 보완 | [작성 문서](specs/NOVA-071.md) |
| [NOVA-072](../07_Compiler_Frontend/NOVA-072_Parser_구현_사양서.md) | 전용 요약; 상세 계약/검증 보완 | [작성 문서](specs/NOVA-072.md) |
| [NOVA-073](../07_Compiler_Frontend/NOVA-073_AST_Node_구조서.md) | 전용 요약; 상세 계약/검증 보완 | [작성 문서](specs/NOVA-073.md) |
| [NOVA-074](../07_Compiler_Frontend/NOVA-074_HIR_Node_구조서.md) | 전용 요약; 상세 계약/검증 보완 | [작성 문서](specs/NOVA-074.md) |
| [NOVA-075](../07_Compiler_Frontend/NOVA-075_AST_HIR_Lowering_사양서.md) | 전용 요약; 상세 계약/검증 보완 | [작성 문서](specs/NOVA-075.md) |
| [NOVA-076](../07_Compiler_Frontend/NOVA-076_Symbol_Table_Definition_Registry_사양서.md) | 공통 양식 포함; 주제별 정의 보완 | [작성 문서](specs/NOVA-076.md) |
| [NOVA-077](../07_Compiler_Frontend/NOVA-077_타입_검사기_소유권_검사기_아키텍처_사양서.md) | 전용 요약; 상세 계약/검증 보완 | [작성 문서](specs/NOVA-077.md) |
| [NOVA-078](../07_Compiler_Frontend/NOVA-078_진단_시스템_Error_Code_관리_사양서.md) | 전용 요약; 상세 계약/검증 보완 | [작성 문서](specs/NOVA-078.md) |
| [NOVA-079](../07_Compiler_Frontend/NOVA-079_Compiler_Query_의존성_Cache_사양서.md) | 전용 요약; 상세 계약/검증 보완 | [작성 문서](specs/NOVA-079.md) |
| [NOVA-080](../07_Compiler_Frontend/NOVA-080_Compiler_내부_오류_ICE_처리_사양서.md) | 전용 요약; 상세 계약/검증 보완 | [작성 문서](specs/NOVA-080.md) |
| [NOVA-081](../08_MIR_Middleend/NOVA-081_MIR_구조_CFG_평가_순서_사양서.md) | 전용 요약; 상세 계약/검증 보완 | [작성 문서](specs/NOVA-081.md) |
| [NOVA-082](../08_MIR_Middleend/NOVA-082_HIR_MIR_Lowering_사양서.md) | 전용 요약; 상세 계약/검증 보완 | [작성 문서](specs/NOVA-082.md) |
| [NOVA-083](../08_MIR_Middleend/NOVA-083_MIR_검증기_사양서.md) | 전용 요약; 상세 계약/검증 보완 | [작성 문서](specs/NOVA-083.md) |
| [NOVA-084](../08_MIR_Middleend/NOVA-084_초기화_Move_분석_구현서.md) | 공통 양식 포함; 주제별 정의 보완 | [작성 문서](specs/NOVA-084.md) |
| [NOVA-085](../08_MIR_Middleend/NOVA-085_Borrow_Checker_구현서.md) | 공통 양식 포함; 주제별 정의 보완 | [작성 문서](specs/NOVA-085.md) |
| [NOVA-086](../08_MIR_Middleend/NOVA-086_Drop_Elaboration_구현서.md) | 전용 요약; 상세 계약/검증 보완 | [작성 문서](specs/NOVA-086.md) |
| [NOVA-087](../08_MIR_Middleend/NOVA-087_Constant_Evaluation_Engine_사양서.md) | 공통 양식 포함; 주제별 정의 보완 | [작성 문서](specs/NOVA-087.md) |
| [NOVA-088](../08_MIR_Middleend/NOVA-088_Match_Lowering_구현서.md) | 전용 요약; 상세 계약/검증 보완 | [작성 문서](specs/NOVA-088.md) |
| [NOVA-089](../08_MIR_Middleend/NOVA-089_Generic_특수화_Pass_구현서.md) | 전용 요약; 상세 계약/검증 보완 | [작성 문서](specs/NOVA-089.md) |
| [NOVA-090](../08_MIR_Middleend/NOVA-090_MIR_최적화_Pass_목록_순서_사양서.md) | 전용 요약; 상세 계약/검증 보완 | [작성 문서](specs/NOVA-090.md) |
| [NOVA-091](../08_MIR_Middleend/NOVA-091_SSA_변환_여부_결정서.md) | 전용 요약; 상세 계약/검증 보완 | [작성 문서](specs/NOVA-091.md) |
| [NOVA-092](../08_MIR_Middleend/NOVA-092_Debug_MIR_출력_형식_사양서.md) | 전용 요약; 상세 계약/검증 보완 | [작성 문서](specs/NOVA-092.md) |
| [NOVA-093](../09_Backend_Runtime/NOVA-093_Backend_선택_LLVM_연동_결정서.md) | 전용 요약; 상세 계약/검증 보완 | [작성 문서](specs/NOVA-093.md) |
| [NOVA-094](../09_Backend_Runtime/NOVA-094_Nova_타입_Backend_타입_Mapping_사양서.md) | 전용 요약; 상세 계약/검증 보완 | [작성 문서](specs/NOVA-094.md) |
| [NOVA-095](../09_Backend_Runtime/NOVA-095_Backend_타입_레이아웃_정렬_Niche_구현서.md) | 전용 요약; 상세 계약/검증 보완 | [작성 문서](specs/NOVA-095.md) |
| [NOVA-096](../09_Backend_Runtime/NOVA-096_함수_ABI_인수_반환_Lowering_사양서.md) | 전용 요약; 상세 계약/검증 보완 | [작성 문서](specs/NOVA-096.md) |
| [NOVA-097](../09_Backend_Runtime/NOVA-097_Name_Mangling_Symbol_사양서.md) | 공통 양식 포함; 주제별 정의 보완 | [작성 문서](specs/NOVA-097.md) |
| [NOVA-098](../09_Backend_Runtime/NOVA-098_Object_File_Linker_연동_사양서.md) | 공통 양식 포함; 주제별 정의 보완 | [작성 문서](specs/NOVA-098.md) |
| [NOVA-099](../09_Backend_Runtime/NOVA-099_Runtime_Startup_main_호출_사양서.md) | 공통 양식 포함; 주제별 정의 보완 | [작성 문서](specs/NOVA-099.md) |
| [NOVA-100](../09_Backend_Runtime/NOVA-100_메모리_Allocator_OOM_정책_사양서.md) | 공통 양식 포함; 주제별 정의 보완 | [작성 문서](specs/NOVA-100.md) |
| [NOVA-101](../09_Backend_Runtime/NOVA-101_Panic_Runtime_Stack_Trace_사양서.md) | 전용 요약; 상세 계약/검증 보완 | [작성 문서](specs/NOVA-101.md) |
| [NOVA-102](../09_Backend_Runtime/NOVA-102_Shared_Weak_Runtime_사양서.md) | 전용 요약; 상세 계약/검증 보완 | [작성 문서](specs/NOVA-102.md) |
| [NOVA-103](../09_Backend_Runtime/NOVA-103_Thread_Atomic_Lock_Runtime_사양서.md) | 공통 양식 포함; 주제별 정의 보완 | [작성 문서](specs/NOVA-103.md) |
| [NOVA-104](../09_Backend_Runtime/NOVA-104_Platform_Abstraction_Layer_사양서.md) | 공통 양식 포함; 주제별 정의 보완 | [작성 문서](specs/NOVA-104.md) |
| [NOVA-105](../10_FFI/NOVA-105_foreign_선언_문법_공통_모델_사양서.md) | 전용 요약; 상세 계약/검증 보완 | [작성 문서](specs/NOVA-105.md) |
| [NOVA-106](../10_FFI/NOVA-106_C_ABI_Import_Export_사양서.md) | 전용 요약; 상세 계약/검증 보완 | [작성 문서](specs/NOVA-106.md) |
| [NOVA-107](../10_FFI/NOVA-107_C_Header_Parser_Binding_Generator_사양서.md) | 전용 요약; 상세 계약/검증 보완 | [작성 문서](specs/NOVA-107.md) |
| [NOVA-108](../10_FFI/NOVA-108_C_Bridge_Opaque_Handle_사양서.md) | 공통 양식 포함; 주제별 정의 보완 | [작성 문서](specs/NOVA-108.md) |
| [NOVA-109](../10_FFI/NOVA-109_외부_예외_Result_변환_사양서.md) | 전용 요약; 상세 계약/검증 보완 | [작성 문서](specs/NOVA-109.md) |
| [NOVA-110](../10_FFI/NOVA-110_Foreign_Ownership_Drop_계약_사양서.md) | 전용 요약; 상세 계약/검증 보완 | [작성 문서](specs/NOVA-110.md) |
| [NOVA-111](../10_FFI/NOVA-111_.NET_Hosted_Adapter_사양서.md) | 공통 양식 포함; 주제별 정의 보완 | [작성 문서](specs/NOVA-111.md) |
| [NOVA-112](../10_FFI/NOVA-112_JVM_Hosted_Adapter_사양서.md) | 공통 양식 포함; 주제별 정의 보완 | [작성 문서](specs/NOVA-112.md) |
| [NOVA-113](../10_FFI/NOVA-113_Python_Hosted_Adapter_사양서.md) | 공통 양식 포함; 주제별 정의 보완 | [작성 문서](specs/NOVA-113.md) |
| [NOVA-114](../11_Standard_Library/NOVA-114_Core_Prelude_기본_Symbol_사양서.md) | 공통 양식 포함; 주제별 정의 보완 | [작성 문서](specs/NOVA-114.md) |
| [NOVA-115](../11_Standard_Library/NOVA-115_Primitive_메서드_API_사양서.md) | 공통 양식 포함; 주제별 정의 보완 | [작성 문서](specs/NOVA-115.md) |
| [NOVA-116](../11_Standard_Library/NOVA-116_string_UTF-8_API_사양서.md) | 전용 요약; 상세 계약/검증 보완 | [작성 문서](specs/NOVA-116.md) |
| [NOVA-117](../11_Standard_Library/NOVA-117_Array_T_API_메모리_모델_사양서.md) | 전용 요약; 상세 계약/검증 보완 | [작성 문서](specs/NOVA-117.md) |
| [NOVA-118](../11_Standard_Library/NOVA-118_Span_T_ReadOnlySpan_T_사양서.md) | 전용 요약; 상세 계약/검증 보완 | [작성 문서](specs/NOVA-118.md) |
| [NOVA-119](../11_Standard_Library/NOVA-119_Option_T_API_사양서.md) | 전용 요약; 상세 계약/검증 보완 | [작성 문서](specs/NOVA-119.md) |
| [NOVA-120](../11_Standard_Library/NOVA-120_Result_TE_try_API_사양서.md) | 전용 요약; 상세 계약/검증 보완 | [작성 문서](specs/NOVA-120.md) |
| [NOVA-121](../11_Standard_Library/NOVA-121_List_T_Map_KV_사양서.md) | 공통 양식 포함; 주제별 정의 보완 | [작성 문서](specs/NOVA-121.md) |
| [NOVA-122](../11_Standard_Library/NOVA-122_파일_Stream_Console_I_O_사양서.md) | 공통 양식 포함; 주제별 정의 보완 | [작성 문서](specs/NOVA-122.md) |
| [NOVA-123](../11_Standard_Library/NOVA-123_시간_난수_환경_변수_사양서.md) | 공통 양식 포함; 주제별 정의 보완 | [작성 문서](specs/NOVA-123.md) |
| [NOVA-124](../11_Standard_Library/NOVA-124_Thread_Lock_Atomic_API_사양서.md) | 공통 양식 포함; 주제별 정의 보완 | [작성 문서](specs/NOVA-124.md) |
| [NOVA-125](../12_Tooling_Packaging/NOVA-125_Compiler_CLI_명령_옵션_사양서.md) | 전용 요약; 상세 계약/검증 보완 | [작성 문서](specs/NOVA-125.md) |
| [NOVA-126](../12_Tooling_Packaging/NOVA-126_Package_Manifest_사양서.md) | 전용 요약; 상세 계약/검증 보완 | [작성 문서](specs/NOVA-126.md) |
| [NOVA-127](../12_Tooling_Packaging/NOVA-127_의존성_해석_Lock_File_사양서.md) | 전용 요약; 상세 계약/검증 보완 | [작성 문서](specs/NOVA-127.md) |
| [NOVA-128](../12_Tooling_Packaging/NOVA-128_Build_Profile_Target_사양서.md) | 공통 양식 포함; 주제별 정의 보완 | [작성 문서](specs/NOVA-128.md) |
| [NOVA-129](../12_Tooling_Packaging/NOVA-129_Package_Artifact_Metadata_형식_사양서.md) | 공통 양식 포함; 주제별 정의 보완 | [작성 문서](specs/NOVA-129.md) |
| [NOVA-130](../12_Tooling_Packaging/NOVA-130_증분_컴파일_Cache_형식_사양서.md) | 공통 양식 포함; 주제별 정의 보완 | [작성 문서](specs/NOVA-130.md) |
| [NOVA-131](../12_Tooling_Packaging/NOVA-131_Formatter_사양_구현서.md) | 전용 요약; 상세 계약/검증 보완 | [작성 문서](specs/NOVA-131.md) |
| [NOVA-132](../12_Tooling_Packaging/NOVA-132_Linter_규칙_사양서.md) | 전용 요약; 상세 계약/검증 보완 | [작성 문서](specs/NOVA-132.md) |
| [NOVA-133](../12_Tooling_Packaging/NOVA-133_Language_Server_LSP_사양서.md) | 공통 양식 포함; 주제별 정의 보완 | [작성 문서](specs/NOVA-133.md) |
| [NOVA-134](../12_Tooling_Packaging/NOVA-134_API_문서_생성기_사양서.md) | 공통 양식 포함; 주제별 정의 보완 | [작성 문서](specs/NOVA-134.md) |
| [NOVA-135](../12_Tooling_Packaging/NOVA-135_Package_Registry_사양서.md) | 공통 양식 포함; 주제별 정의 보완 | [작성 문서](specs/NOVA-135.md) |
| [NOVA-136](../13_Testing_Release/NOVA-136_Compiler_테스트_전략서.md) | 전용 요약; 상세 계약/검증 보완 | [작성 문서](specs/NOVA-136.md) |
| [NOVA-137](../13_Testing_Release/NOVA-137_Lexer_Parser_Snapshot_테스트_사양서.md) | 전용 요약; 상세 계약/검증 보완 | [작성 문서](specs/NOVA-137.md) |
| [NOVA-138](../13_Testing_Release/NOVA-138_Compile-pass_Compile-fail_규격.md) | 공통 양식 포함; 주제별 정의 보완 | [작성 문서](specs/NOVA-138.md) |
| [NOVA-139](../13_Testing_Release/NOVA-139_MIR_Snapshot_규격.md) | 전용 요약; 상세 계약/검증 보완 | [작성 문서](specs/NOVA-139.md) |
| [NOVA-140](../13_Testing_Release/NOVA-140_Runtime_Test_Harness_사양서.md) | 공통 양식 포함; 주제별 정의 보완 | [작성 문서](specs/NOVA-140.md) |
| [NOVA-141](../13_Testing_Release/NOVA-141_Lexer_Parser_MIR_Fuzzing_사양서.md) | 전용 요약; 상세 계약/검증 보완 | [작성 문서](specs/NOVA-141.md) |
| [NOVA-142](../13_Testing_Release/NOVA-142_Property_Differential_Testing_사양서.md) | 공통 양식 포함; 주제별 정의 보완 | [작성 문서](specs/NOVA-142.md) |
| [NOVA-143](../13_Testing_Release/NOVA-143_성능_Benchmark_회귀_기준서.md) | 공통 양식 포함; 주제별 정의 보완 | [작성 문서](specs/NOVA-143.md) |
| [NOVA-144](../13_Testing_Release/NOVA-144_메모리_안전성_검증_전략서.md) | 공통 양식 포함; 주제별 정의 보완 | [작성 문서](specs/NOVA-144.md) |
| [NOVA-145](../13_Testing_Release/NOVA-145_Unsafe_FFI_보안_검토_기준서.md) | 전용 요약; 상세 계약/검증 보완 | [작성 문서](specs/NOVA-145.md) |
| [NOVA-146](../13_Testing_Release/NOVA-146_0.1_Release_Checklist.md) | 전용 요약; 상세 계약/검증 보완 | [작성 문서](specs/NOVA-146.md) |
| [NOVA-147](../13_Testing_Release/NOVA-147_언어_호환성_Test_Suite_사양서.md) | 공통 양식 포함; 주제별 정의 보완 | [작성 문서](specs/NOVA-147.md) |
| [NOVA-148](../13_Testing_Release/NOVA-148_Self-hosting_계획서.md) | 전용 요약; 상세 계약/검증 보완 | [작성 문서](specs/NOVA-148.md) |
