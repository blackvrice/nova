# P30 숫자·char·Copy struct statement match 계약

작성일 2026-10-12. 상태: **Accepted / 구현·검증 완료**.
사용자 “다음 개발 승인할테니 개발 진행해줘”의 다음 단계 승인으로 이 제한된 계약을 구현 전에 기록한다.
P01~P29·D01~D05·Canonical·원본 148개를 보존한다. 전체 D08/D10/D12/D16/D25/D30 승인이 아니다.

## Specification Change Proposal

관련 NOVA-026/027/029/035/038/044/046/047/048/074/076/078/081/083/087/091/094/095/136 및 P12 Copy struct·P24 recursive pattern·P25 guard 계약.
현재 root statement match는 Bool/Copy Enum/Option/Result/Tuple/Unit을 지원한다. 숫자·char·Copy struct는 nested binder로만 받는다.
문제: 정수 상태 값이나 Copy struct 전체를 guard로 분기하려면 Bool/Enum/Tuple wrapper를 만들어야 한다.
변경: 8종 정수·Float32/64·Char·Copy Struct 값을 root statement match의 binder/wildcard와 Bool guard로 검사한다.
이유: 이미 정의된 Copy snapshot·pattern binder·guard·bounded coverage 모델을 연결한다.
대안: literal/range/or/field pattern과 Array/Move match를 함께 확장한다. 이번에는 기존 문법의 Copy 값 subset만 적용한다.
영향: Type Checker root domain·기존 recursive MIR/LLVM 검증·테스트·문서. Lexer/Parser/AST/HIR/Runtime production을 확장하지 않는다.
Backward Compatibility: 기존 P01~P29 의미/진단/평가 순서/예산을 유지하며 P14 invalid_scrutinee.nova 및 P24/P25 root_scalar.nova의 당시 N2101은 별도 역사로 보존한다.

## 승인 범위

1. 새 root 타입은 Int8/16/32/64·UInt8/16/32/64·Float32/64·Char·등록된 Copy Struct다.
   alias는 기존 정규화 타입을 사용한다. 기존 Bool/Enum/Tuple/Unit domain은 그대로다. String·Function·Move 타입은 제외한다.
2. 새 root의 유효한 pattern은 기존 bare binder/`_`뿐이다. binder는 immutable whole-value Copy local이고 해당 arm의 guard/body에서만 보인다.
   다른 타입용 Bool/Unit/Tuple/variant pattern은 기존 N2101·N2201 등으로 거부한다. 숫자/char literal·struct field·or/range pattern은 추가하지 않는다.
   같은 이름의 outer local을 shadow할 수 있으며 binder 대입은 N3004, arm 밖 참조는 N2001이다.
3. 기존 statement match의 scrutinee를 한 번 평가하고 snapshot한다. binder는 이 값을 Copy한다.
   arm은 소스 순서로 검사한다. pattern 성공 후 Bool guard를 한 번 평가하고 false이면 다음 arm으로 간다.
   guard의 기존 print/function/method/try·checked 산술·float IEEE·Source Span/Abort 의미를 그대로 적용한다.
4. 완전성과 도달성은 기존 bounded matrix의 opaque Copy domain을 사용한다. 구체 값 domain을 열거하지 않는다.
   guard 없는 binder/`_`가 전체 domain을 덮는다. guarded arm만으로는 N3101이며 missing witness는 `_`다.
   guard 없는 전체 pattern 뒤 arm은 N3102다. guard true/false·NaN·상수 scrutinee를 근거로 분석을 좁히지 않는다.
   return/jump 분석은 기존 statement match 정책을 유지한다. match는 표현식이나 const initializer가 아니다.
5. MIR은 기존 recursive pattern snapshot/guard CFG를 사용한다. 새 tag/payload extraction이나 runtime 타입 분기는 필요 없다.
   타입·immutable binder·Source/Checked plan·Operand/CFG proof 변조는 원 HIR에서 재계산하여 거부한다.
   copy struct private field 접근·Read method·snapshot ABI/layout은 기존 계약이다.
6. 기존 arm 1,025·pattern node 10,000·matrix task 100,000·live cell 1,000,000·nesting/layout 예산을 유지한다.
   P29 62-production EBNF를 **바이트 그대로 재사용**하며 grammar_change=false다. 새로운 diagnostic code는 없다.
7. P14 invalid_scrutinee.nova 및 P24/P25 root_scalar.nova의 source/원 N2101 metadata를 변경하지 않고 현재 성공을 P30 historical override로 기록한다.
   모든 다른 과거 fixture는 보존한다. Array·String/Move·literal/field/or/range pattern·match expression·일반 Move/borrow/Drop은 제외한다.

## 수용 계획

8종 정수 경계·float NaN/Inf/zero·Unicode char·import/alias/private struct·whole snapshot·guard order/try/Abort·return/loop를 검증한다.
고정 UTF-8 fixture·부정 진단 Span·Source/Checked/MIR forgery·실제 LLVM COFF/ELF O0/O2·Windows Native debug/release 결과를 기록한다.
