# P38 Copy Tuple 중간 rest 패턴 계약

2026-10-20. 상태: **Accepted / 구현·검증 완료**.
사용자 “다음 작업 승인할태니 개발 해줘”의 다음 단계 승인으로 이 제한 계약을 구현 전에 기록한다.
P01~P37·D01~D05·Canonical·원본 148개를 보존한다. 전체 D06/D08/D09/D10/D12/D16/D25/D30 승인이 아니다.

구현·검증 완료: 2026-10-20. 승인일 2026-10-20.

## Specification Change Proposal

관련 NOVA-026/029/035/038/044/046/047/048/073/074/076/078/081/083/087/091/094/095/136 및 P13 Tuple·P24 recursive pattern·P25 guard·P26~P29 binding·P35~P37 rest 계약.
문제: Tuple suffix를 선택하려면 중간 요소를 모두 wildcard로 나열해야 한다. suffix 타입·Source annotation·실제 field ordinal을 정확히 대응해야 한다.
변경: Copy Tuple의 하나의 .. marker를 시작/중간/끝에 허용한다. struct/Enum payload의 trailing-only 정책은 바꾸지 않는다.
이유: P36 explicit field_indices와 기존 const/MIR projection 경로를 사용해 선택한 앞뒤 요소만 검사·바인딩한다.
대안: middle struct/Enum rest·rest binder·Array를 함께 확장한다. 이번에는 structural Copy Tuple의 prefix/suffix 선택만 지원한다.
Backward Compatibility: rest 없는 정확한 arity·한 요소 Tuple comma·Unit ()·P35/P37 trailing-only rest·평가 순서/예산을 보존한다. P36 middle_rest.nova와 당시 N1102 metadata는 수정하지 않고 현재 성공 override를 기록한다.

## 승인 범위

1. tuple_pattern 하나를 확장하고 tuple_pattern_tail을 추가한다. 총 **70개 production / 기존 68개 보존**이다.
   (first,..,last), (..,last), (first,..,a,b), (..,a,b,)를 허용한다. P36 (..)/(..,)/(first,..)도 그대로다.
   marker는 하나만 허용한다. rest는 0개 이상의 요소를 생략할 수 있다. (a,..,b)가 2-element Tuple에 적용되면 생략 요소는 0개다.
   중복 rest·spaced dots·rest binder·Tuple 이름 label·root rest는 N1102다. struct/Enum payload middle rest도 계속 N1102다.
   기존 인접 Dot token·정확한 .. Span을 재사용한다. Lexer/END는 변경하지 않는다.
2. Type::Tuple만 허용한다. Unit/scalar/struct는 N2101이다. Unit은 () 패턴을 사용한다.
   explicit prefix 수 p, suffix 수 s, Tuple arity n에 대해 p+s<=n이어야 한다. 초과하면 N2201, rest 없는 arity 불일치도 기존 N2201이다.
   prefix field index는 0..p, suffix는 n-s..n이다. Checked children은 marker를 제외한 Source 순서이고 field_indices가 실제 index를 일대일 저장한다.
   AST/HIR는 marker를 포함한 Source 순서를 보존한다. suffix 타입과 secondary annotation은 실제 선언 요소를 가리키며 가상의 field Source를 만들지 않는다.
   marker는 Unit/Wildcard bookkeeping으로 source pattern node 한 개를 센다. implicit wildcard에 synthetic Source/HIR node·DefId를 만들지 않는다.
3. matrix는 전체 Tuple vector를 wildcard로 만들고 explicit prefix/suffix의 field_indices만 채운다. N3101/N3102·guard 없는 합집합·기존 witness/순서/예산을 유지한다.
   let/var/local/global const는 explicit component가 모두 irrefutable일 때만 허용한다. N3103·Source 순서의 첫 refutable secondary·const 평가/파생 오류 억제를 유지한다.
   binder 타입/정수 문맥·scope·중복 N2002·불변 N3004·alias/import·nested struct/Tuple/Enum/Option/Result는 기존 규칙이다.
4. initializer/scrutinee 전체를 한 번 평가·snapshot한다. 생략한 중간 요소의 effect·checked Abort·try 전파를 건너뛰지 않는다.
   기존 MIR은 실제 prefix/suffix index만 projection한다. guard는 패턴 성공 후 한 번 평가한다.
   const는 전체 initializer 평가 후 실제 index의 binder cache를 저장한다. whole value·forward/static cycle·default·10,000-node const 예산을 보존한다.
5. Source marker spelling/shape/order/parent·중복 marker·단일 Tuple comma와 Checked children/field_indices/type/cache·MIR 같은 타입 field index/Source/guard branch 변조를 기존 gate와 private proof로 거부한다.
   arm 1,025·source pattern nodes 10,000·matrix tasks 100,000·live cells 1,000,000·nesting 128·기존 layout 한도를 유지한다.
   전체 value domain/Cartesian product를 열거하지 않는다. const/MIR/LLVM/Runtime production·private ABI/layout·Lexer/END·Cargo dependency·diagnostic registry는 변경하지 않는다.
6. struct/Enum payload middle rest·rest binder·named Enum payload·shorthand/brace·Array·String/Move element·parameter/for/assignment pattern·const 함수·일반 Move/borrow/Drop은 제외한다.

## 수용 계획

leading/middle/trailing/rest-only·빈 생략·한 요소/Unit·suffix type/annotation·nonzero field_indices·nested product/sum·coverage·const cache/forward/실패/순환·scope/mutability·omitted middle effect/Abort/try를 검증한다.
UTF-8 source hash·code별 byte Span·P36 historical override·Source/Checked/MIR 변조·10,000/10,001-node 및 128/129 nesting·독립 Bool coverage oracle·LLVM COFF/ELF O0/O2·Windows Native debug/release·예제/사용자 명령을 제공한다.
