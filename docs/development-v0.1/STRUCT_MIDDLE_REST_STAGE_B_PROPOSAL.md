# P40 Copy struct 중간 rest 패턴 계약

2026-10-20. 상태: **Accepted / 구현·검증 완료**.
사용자 “다음 작업 승인할태니 개발 해줘”의 다음 단계 승인으로 이 제한 계약을 구현 전에 기록한다.
P01~P39·D01~D05·Canonical·원본 148개를 보존한다. 전체 D06/D08/D09/D10/D12/D16/D25/D30 승인이 아니다.

구현·검증 완료: 2026-10-20. 승인일 2026-10-20.

## Specification Change Proposal

관련 NOVA-026/029/035/038/044/046/047/048/073/074/076/078/081/083/087/091/094/095/136 및 P33~P35 struct·P38/P39 rest 계약.
문제: struct 위치 패턴의 suffix만 선택할 때 중간 field마다 wildcard가 필요하다. 이름 label·Source 방문·canonical checked 순서를 혼동하면 잘못된 타입/값을 연결한다.
변경: struct pattern에서 하나의 ..를 시작/중간/끝에 허용한다.
이유: 기존 이름 대응·Copy snapshot·bounded coverage를 유지하면서 positional prefix/suffix를 선택한다.
대안: shorthand·rest binder·Array·일반 Move/borrow를 함께 도입한다. 이번에는 기존 Copy struct의 rest 위치만 확장한다.
Backward Compatibility: rest 없는 full field·unknown/duplicate/이름 뒤 위치 N2201·private 생략 N2004·Tuple/Enum rest·alias head 금지를 유지한다. P35/P38/P39 고정 중간 rest 거부 source/metadata는 보존하고 현재 성공 overrides를 별도로 기록한다.

## 승인 범위

1. struct_pattern_arguments 한 production만 확장한다. 총 **70개 production / 기존 69개 보존**, 새 production은 없다.
   Point(first,..,last), Point(..,last), Point(first,..,a,b,)와 기존 trailing/rest-only/empty struct를 지원한다.
   marker는 하나이고 두 점은 인접해야 한다. 중복/spaced dots·rest binder·field value/root rest는 N1102다. Lexer/END는 변경하지 않는다.
2. 위치 prefix p·suffix s·전체 field n에서 prefix는 0..p, suffix는 n-s..n이다. suffix 수는 marker 뒤의 직접 위치 인수 수이며 named field는 세지 않는다. 빈 생략을 허용한다.
   이름 field는 marker 양쪽 어디서나 선언 이름으로 대응한다. Point(..,y:b,x:a)와 Point(a,..,last,y:b)도 허용한다.
   이름 뒤 위치는 marker가 사이에 있어도 N2201이다. marker는 이 순서 규칙을 초기화하지 않는다.
   unknown/중복/위치-name overlap/explicit 초과/marker 없는 누락은 N2201이다. suffix가 named field를 피해 다른 index로 이동하지 않는다.
   private field를 포함한 struct는 다른 module에서 생략하더라도 N2004다. nominal/import·alias annotation을 유지하고 alias head는 N1102다.
3. AST/HIR와 binder/타입 진단 방문은 marker 포함 Source 순서다. Checked struct children은 marker 제외·field 선언 순서로 정렬하며 field_indices와 일대일 대응한다.
   각 Source child의 타입/secondary annotation은 실제 field 선언으로 대응하며 marker는 Unit/None이다. component 불일치 N2101·범위 초과 N2102·중복 binder N2002·불변 N3004를 구분한다.
   marker는 source pattern node 한 개를 센다. implicit wildcard에 synthetic Source/HIR node·DefId를 만들지 않는다.
4. coverage는 전체 struct field vector의 wildcard 중 explicit indices만 채운다. N3101/N3102·guard 없는 합집합·witness/순서/예산을 유지한다.
   let/var/local/global const는 explicit irrefutable component만 허용한다. refutable이면 N3103·Source 순서 첫 refutable secondary이며 const 평가/파생 오류를 억제한다.
5. 전체 initializer/scrutinee 단일 평가·snapshot 뒤 선언 순서의 explicit field만 projection한다. 생략한 중간 field operand의 effect·checked Abort·try 전파도 실행한다.
   guard는 패턴 성공 뒤 한 번 평가한다. const 전체 평가·실제 index binder cache·whole value·forward/static cycle/default·10,000-node 예산을 보존한다.
6. Source marker spelling/shape/order/parent·중복 marker·Checked children/indices/type/cache·MIR 같은 타입 FieldId/Source/guard 변조를 기존 gate/private proof로 거부한다.
   arm 1,025·source pattern nodes 10,000·matrix tasks 100,000·live cells 1,000,000·nesting 128·layout 한도를 유지한다. 전체 Cartesian product를 열거하지 않는다.
   LLVM/Runtime production·private ABI/layout·Lexer/END·Cargo dependency·diagnostic registry는 변경하지 않는다.
7. shorthand/brace·rest binder·named Enum payload·field default·Array·String/Move field·parameter/for/assignment pattern·const 함수·일반 Move/borrow/Drop은 제외한다.

## 수용 계획

prefix/suffix 모든 조합·named marker 양쪽/역순/overlap·실제 field annotation·Source 진단 순서·coverage 독립 oracle·const cache/forward/default/cycle/실패·visibility/import·initializer 효과/Abort/try를 검증한다.
고정 UTF-8/LF hash·code별 byte Span·역사 overrides·Source/Checked/MIR 변조·10,000/10,001 nodes·128/129 nesting·LLVM COFF/ELF O0/O2·Windows Native debug/release·독립 예제/명령을 제공한다.
