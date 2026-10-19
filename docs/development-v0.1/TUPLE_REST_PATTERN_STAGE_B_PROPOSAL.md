# P36 Copy Tuple trailing rest 패턴 계약

2026-10-19. 상태: **Accepted / 구현·검증 완료**.
사용자 “다음 작업 승인할테니 개발 진행해줘”의 다음 단계 승인으로 이 제한 계약을 구현 전에 기록한다.
P01~P35·D01~D05·Canonical·원본 148개를 보존한다. 전체 D06/D08/D09/D10/D12/D16/D25/D30 승인이 아니다.

구현·검증 완료: 2026-10-20. 승인일 2026-10-19.

## Specification Change Proposal

관련 NOVA-026/029/035/038/044/046/047/048/073/074/076/078/081/083/087/091/094/095/136 및 P13 Copy Tuple·P24 recursive pattern·P25 guard·P26~P29 binding·P35 struct rest 계약.
문제: tuple의 필요 없는 suffix를 wildcard로 모두 나열해야 한다. prefix projection·omitted-element coverage와 Source 없는 implicit wildcard를 정의해야 한다.
변경: Tuple pattern의 마지막 .. marker로 남은 요소를 생략한다. P35 product plan과 기존 Tuple/Unit 구별을 유지한다.
이유: 전체 initializer 평가·Copy snapshot·bounded matrix·const/MIR 실제 field 번호를 재사용한다.
대안: 중간 rest/뒤 요소·rest binder·Enum payload rest·Array까지 확장한다. 이번에는 structural Copy Tuple의 prefix+trailing rest만 지원한다.
Backward Compatibility: rest 없는 기존 Tuple arity/Source/쉼표·Unit ()·P35 struct rest를 보존한다. 과거 고정 fixture 거부 source/metadata는 새 성공 override로 보존한다.

## 승인 범위

1. tuple_pattern 하나를 확장하고 tuple_pattern_rest = ".."를 추가한다. 총 **68개 production / 기존 66개 보존**이다.
   (first,..), (first,second,..), (..), (..,)와 선택적 trailing comma를 지원한다. explicit prefix가 전체 요소 수와 같아도 rest를 허용한다.
   rest는 한 번, 마지막에만 허용한다. rest 뒤 요소/두 번째 rest·rest binder·field value/root·Enum/sum payload rest는 N1102다.
   기존 인접 Dot token 두 개와 P35 PatternRest/정확한 .. Span을 재사용한다. 두 점 사이 trivia는 허용하지 않는다. Lexer/END 규칙은 변경하지 않는다.
   AST/HIR tuple children은 Source 순서이며 marker는 직접 마지막 child에만 허용한다. 일반 단일 tuple (x,)의 필수 comma는 유지한다.
2. (..)도 Type::Tuple만 요구한다. Unit ()·struct·scalar에 적용하면 N2101이며 Unit은 계속 ()로 매칭한다.
   rest 없이 요소 수가 다르거나 rest가 있어도 explicit prefix가 tuple arity보다 길면 N2201이다. annotation/alias·nested struct/Tuple/Enum/Option/Result는 기존 규칙이다.
   prefix binder에만 이름/타입/가변성/const 값이 생긴다. initializer 이전 local scope·전역 forward/순환·중복 N2002·불변 N3004를 보존한다.
3. Checked tuple children은 explicit prefix만 저장하고 field_indices=0..prefix 길이와 일대일 대응한다. struct의 P35 sparse indices는 그대로다.
   marker는 Unit/Wildcard bookkeeping으로 source node 한 개를 세며 값/projection/binder로 사용하지 않는다. implicit wildcard에 synthetic Source/HIR node를 만들지 않는다.
4. matrix는 Tuple constructor의 전체 field vector를 wildcard로 시작하고 explicit prefix 위치만 채운다. N3101/N3102·unguarded 합집합·guard 정책·기존 위치 witness·예산을 유지한다.
   let/var/local/global const는 prefix가 모두 irrefutable일 때만 허용한다. scalar/여러 variant prefix는 기존 N3103·첫 refutable Source secondary이며 const 계산/파생 오류를 억제한다.
5. initializer/scrutinee 전체를 한 번 평가·snapshot한다. 생략한 tuple suffix의 effect·checked Abort·try 전파를 건너뛰지 않는다.
   MIR projection은 explicit prefix만 실행하고 guard는 패턴 성공 후 한 번이다. const도 전체 initializer 평가·기존 10,000-node 예산 후 prefix binder 값을 cache한다.
   Native Tuple ABI/layout·mutable copy·default·return/jump는 기존 계약이다.
6. Source marker spelling/adjacency·parent/shape/order·단일 tuple comma·Checked indices/children/type/cache·MIR FieldId/Source/branch 변조를 원 HIR 재계산으로 거부한다.
   기존 pattern nesting 한도 128이 struct constructor head에서 누락된 것을 함께 수정한다. 승인된 한도 적용의 버그 수정이며 새 중첩 의미를 추가하지 않는다.
   LLVM/Runtime production·ABI/layout·Lexer/END·Cargo dependency·diagnostic registry는 변경하지 않는다.
7. arm 1,025·source pattern nodes 10,000·matrix tasks 100,000·live cells 1,000,000·nesting/layout 한도를 유지한다.
   implicit wildcard 확장은 기존 field/layout·matrix 한도 안에서만 한다. 전체 value domain/Cartesian product를 열거하지 않는다.
   middle rest·rest binder·Enum/sum payload rest·Tuple 이름 label·shorthand/brace·Array·String/Move element·parameter/for/assignment pattern·const 함수·일반 Move/borrow/Drop은 제외한다.

## 수용 계획

rest-only/한 요소/전체 prefix·Unit 거부/기존 Unit 허용·arity/type/range·nested product/sum·coverage·const cache/forward/실패/순환·scope/mutability·snapshot/omitted suffix effect/Abort/try를 검증한다.
UTF-8 fixture와 code별 byte Span·역사 override·Source/Checked/MIR 변조·10,000/10,001-node 및 struct/tuple nesting 128/129·독립 Bool coverage oracle·LLVM COFF/ELF O0/O2·Windows Native debug/release·예제/사용자 명령을 제공한다.
