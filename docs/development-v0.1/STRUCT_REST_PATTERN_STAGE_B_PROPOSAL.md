# P35 Copy struct trailing rest 패턴 계약

2026-10-19. 상태: **Accepted / 구현·검증 완료**.
사용자 “다음 작업 승인할테니 개발 진행해줘”의 다음 단계 승인으로 이 제한 계약을 구현 전에 기록한다.
P01~P34·D01~D05·Canonical·원본 148개를 보존한다. 전체 D06/D08/D09/D10/D12/D16/D25/D30 승인이 아니다.

구현·검증 완료: 2026-10-19. 승인일 2026-10-19.

## Specification Change Proposal

관련 NOVA-026/029/035/038/044/046/047/048/073/074/076/078/081/083/087/091/094/095/136 및 P12 Copy struct·P24 recursive pattern·P25 guard·P26~P29 binding·P33/P34 struct 패턴 계약.
문제: 원하는 field 외에도 모든 field를 wildcard로 나열해야 한다. 이름 순서가 다른 sparse projection과 omitted-field coverage를 명확히 정의해야 한다.
변경: struct pattern의 마지막 argument에만 .. marker를 허용하고 명시하지 않은 field를 wildcard로 취급한다.
이유: 기존 Source 순서/field 이름 대응·Copy snapshot·bounded matrix를 보존하며 부분 분해를 지원한다.
대안: shorthand·brace 문법·Tuple/Enum payload rest와 private field 생략 정책까지 확장한다. 이번에는 접근 가능한 전체 Copy struct의 trailing rest만 지원한다.
Backward Compatibility: rest 없는 P33/P34 전체 field 요구·N2201·평가 순서·coverage를 보존한다. 기존 Parser의 P(..) 거부 source를 현재 성공 assertion으로 옮기고 고정 fixture/grammar bytes는 보존한다.

## 승인 범위

1. struct_pattern_arguments production을 확장하고 struct_pattern_rest = ".."를 추가한다. 총 **67개 production / 기존 65개 보존**이다.
   Point(..), Point(x,..), Point(y:flag,..), Point(x,y:flag,..)와 선택적 trailing comma를 지원한다. Empty(..)와 모든 field를 이미 명시한 뒤 ..도 허용한다.
   rest는 한 번, 마지막에만 허용한다. rest 뒤 field/두 번째 rest·rest binder·field value/tuple/variant/root의 ..는 N1102로 거부한다.
   Lexer의 기존 인접 Dot token 두 개를 사용하고 새 Token/END 규칙은 추가하지 않는다. 두 점 사이 trivia는 허용하지 않는다.
   AST/HIR PatternRest는 정확한 .. Source Span과 child 없는 marker다. struct pattern의 직접 마지막 child에만 허용한다.
2. rest가 있으면 누락한 field는 wildcard이고 binding을 만들지 않는다. 없으면 모든 field를 한 번 명시하는 P34 규칙이다.
   위치 prefix·이름 label 대응·unknown/duplicate/overlap/이름 뒤 위치는 P34 N2201을 유지한다. rest는 위치 ordinal/field label이 아니다.
   nominal/import head·alias annotation·Copy layout을 유지한다. private field를 포함한 struct는 다른 module에서 **생략하더라도 N2004**다.
   rest를 private 접근 허용으로 해석하지 않는다. 같은 module의 private field 생략은 허용한다. 전체 값 binder/wildcard는 P30 그대로다.
3. AST/HIR와 binder 선언/타입 진단 방문은 Source 순서다. Checked struct children은 명시한 field만 선언 순서로 정렬하고 field_indices와 일대일 대응한다.
   rest marker는 Source pattern node 한 개로 센다. Unit 타입/Wildcard typed bookkeeping에만 남기며 값/binder/field projection으로 사용하지 않는다.
   implicit wildcard에 synthetic Source/HIR node를 만들지 않는다. tuple/variant/field wrapper의 기존 children 의미는 변경하지 않는다.
4. matrix는 struct constructor의 전체 field vector를 wildcard로 시작하고 명시한 field_indices 위치만 실제 패턴으로 채운다.
   omitted field를 무시한 정확한 N3101/N3102와 unguarded 합집합·guard 정책을 유지한다. witness는 P33 위치 형태다.
   let/var/local/global const의 irrefutability N3103는 명시한 component만 검사한다. 알려진 initializer 값으로 좁히지 않으며 첫 refutable secondary는 Source 순서다.
5. initializer/scrutinee 전체를 한 번 평가·snapshot한다. 생략한 field의 constructor operand도 Source 순서대로 평가되므로 effect·checked Abort·try 전파를 생략하지 않는다.
   pattern projection은 명시한 field만 선언 순서로 실행하고 guard는 모든 pattern 성공 뒤 한 번 평가한다.
   const는 전체 initializer 평가/기존 10,000-node 예산 후 실제 field_indices로 cached binder 값을 projection한다. 생략한 field의 const 실패도 N3201이다.
   전역 forward/순환·default·mutable binder copy·return/jump·Native ABI는 기존 계약이다.
6. Source .. spelling/adjacency·marker parent/shape/order·Checked sparse field_indices/child order/type/cache·MIR FieldId/Source/branch 변조를 원 HIR 재계산으로 거부한다.
   LLVM/Runtime production·ABI/layout·Lexer/END·Cargo dependency·diagnostic registry는 변경하지 않는다.
7. arm 1,025·source pattern nodes 10,000·matrix tasks 100,000·live cells 1,000,000·nesting/layout 한도를 유지한다.
   implicit wildcard 확장은 기존 field/layout·matrix 한도 안에서만 한다. 전체 value domain/Cartesian product를 열거하지 않는다.
   shorthand/brace·Tuple/Enum/sum payload rest·rest binder·field default·or/range·Array·String/Move field·parameter/for/assignment pattern·const 함수·일반 Move/borrow/Drop은 제외한다.

## 수용 계획

rest-only/empty/역순 label/위치 혼합·누락 허용/기존 누락 거부·가시성/alias/nominal·sparse 타입/const projection·omitted-field coverage·snapshot/guard/try/Abort를 검증한다.
고정 UTF-8 fixture/실제 code별 byte Span, Source/Checked/MIR 변조·rest 포함 10,000/10,001-node 경계·독립 Bool product oracle·LLVM COFF/ELF O0/O2·Windows Native debug/release·독립 예제/명령을 제공한다.
