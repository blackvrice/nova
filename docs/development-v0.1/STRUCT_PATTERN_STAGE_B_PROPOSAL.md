# P33 Copy struct 위치 패턴 계약

2026-10-12. 상태: **Accepted / 구현·검증 완료**.
사용자 “다음 작업 승인할태니 개발 진행해줘”의 다음 단계 승인으로 이 제한된 계약을 구현 전에 기록한다.
P01~P32·D01~D05·Canonical·원본 148개를 보존한다. 전체 D06/D08/D09/D10/D12/D16/D25/D30 승인이 아니다.

## Specification Change Proposal

관련 NOVA-026/029/035/038/044/046/047/048/073/074/076/078/081/083/087/091/094/095/136 및 P12 Copy struct·P24 recursive pattern·P25 guard·P26~P29 binding·P30 Copy 값 match·P31/P32 scalar 패턴 계약.
문제: Copy struct는 binder/wildcard로만 검사할 수 있어 Tuple/Enum처럼 field 구조를 pattern으로 표현할 수 없다.
변경: 기존 위치 생성자와 대응하는 IDENT(patterns) 형태를 nominal struct pattern으로 정의한다.
이유: P12 필드 순서/가시성·Copy layout과 기존 bounded matrix/Source/MIR projection을 재사용한다.
대안: 이름 필드·shorthand·rest 패턴을 함께 지원한다. 이번에는 전체 필드를 명시하는 위치 subset만 구현한다.
Backward Compatibility: 기존 bare IDENT는 binder이며 상수/타입 equality lookup을 하지 않는다. 기존 생성자/메서드·P01~P32 의미와 opaque-only witness를 보존한다.

## 승인 범위

1. struct_pattern = IDENT, "(", [pattern_arguments], ")"를 추가하고 pattern/destructuring_pattern에 연결한다.
   총 **64개 production / 기존 61개 보존**이다. 빈 struct는 Empty(), trailing comma·trivia는 기존 tuple/variant 패턴 규칙을 따른다.
   AST/HIR PatternStruct는 원 head token Span·source-ordered pattern children을 보존한다. expression Call과 구분한다.
   이름 필드·shorthand·.. 생략·qualified module head·alias constructor/pattern head는 제외한다.
2. head는 선언 module의 type namespace에서 실제 Struct 또는 P11 item import alias로 찾는다. 값 binder/function과 다른 namespace를 유지한다.
   P21 transparent type alias는 annotation에 허용하지만 pattern head에는 N1102다. 등록된 Enum head 또는 nominal mismatch는 N2101, undefined type은 N2001이다.
   실패한 import/initializer ErrorType의 파생 진단을 억제한다. 유효 Copy layout을 가진 P12 struct만 분해한다.
3. 선언 순서의 모든 field마다 정확히 한 pattern을 요구한다. arity mismatch는 N2201이다. 메서드는 field arity에 포함하지 않는다.
   P12 생성자와 동일하게 모든 필드에 접근할 수 있어야 한다. 다른 module의 private field는 head Span N2004와 원 field secondary로 거부한다.
   wildcard도 private field를 노출하지 않는다. 같은 module의 private field는 허용한다.
4. field의 기대 타입을 기존 recursive pattern에 전달한다. Tuple/Enum/Option/Result/struct·Bool/Char/정수 literal을 중첩할 수 있다.
   binder는 기존 match/let 불변·var 가변·const 상수 규칙과 중복 N2002/initializer-before-local-binding/전역 forward·순환 검사를 따른다.
   let/var/local·global const는 field 패턴 모두 irrefutable일 때만 허용한다. scalar/다중 variant의 refutable component는 기존 N3103·secondary이며 const 계산/파생 오류를 억제한다.
   const initializer는 한 번 계산하고 Struct ConstValue field를 projection하여 cached binder 값을 저장한다. 기존 10,000-node 예산을 유지한다.
5. matrix는 nominal struct 단일 constructor와 선언 순서 field types로 specialize한다. guard 없는 합집합의 완전성 N3101·도달성 N3102를 기존 bounded usefulness로 검사한다.
   concrete struct patterns이 있으면 missing witness는 Type(field patterns)다. concrete 없는 기존 opaque-only _ witness/shortcut을 보존한다.
   struct의 Bool field true/false 합집합과 nested sum/product를 검사하며 Cartesian domain을 열거하지 않는다. guard truth/known scrutinee로 coverage를 좁히지 않는다.
6. scrutinee/initializer를 한 번 snapshot한 뒤 기존 FieldId 순서로 projection한다. pattern 성공 후 guard를 한 번 평가하고 false 시 다음 arm으로 간다.
   Enum payload는 tag proof 아래에서만 접근한다. try/return/jump·checked Abort·const/default·Native private struct ABI는 기존 계약이다.
7. Source head token·delimiter/trivia/child order·typed nominal ID/type/arity·Checked caches·MIR field/type/source/projection proof 변조를 원 HIR 재계산으로 거부한다.
   LLVM/Runtime production·ABI/layout·Lexer/END·Cargo dependency·diagnostic registry는 변경하지 않는다.
8. arm 1,025·pattern nodes 10,000·matrix tasks 100,000·live cells 1,000,000·nesting/layout 예산을 유지한다.
   field 이름/rest/or/range 패턴·Array·String/Move field·parameter/for/assignment pattern·const 함수·일반 Move/borrow/Drop은 제외한다.

## 수용 계획

정상·부정 고정 UTF-8 fixture와 실제 code별 byte Span을 제공한다. 빈/중첩/다중 파일 struct·type import alias·private·nominal/arity·coverage·refutability·const 실패/순환·snapshot·guard/try/loop·자원 한도를 검사한다.
AST/Checked/MIR 변조·LLVM COFF/ELF O0/O2·Windows Native debug/release·독립 예제와 사용자 실행 명령을 검증한다.
