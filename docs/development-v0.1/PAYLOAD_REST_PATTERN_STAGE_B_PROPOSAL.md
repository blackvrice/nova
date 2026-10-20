# P37 Copy Enum·sum payload trailing rest 패턴 계약

2026-10-20. 상태: **Accepted / 구현·검증 완료**.
사용자 “다음 작업 승인할태니 개발 해줘”의 다음 단계 승인으로 이 제한 계약을 구현 전에 기록한다.
P01~P36·D01~D05·Canonical·원본 148개를 보존한다. 전체 D06/D08/D09/D10/D12/D16/D25/D30 승인이 아니다.

구현·검증 완료: 2026-10-20. 승인일 2026-10-20.

## Specification Change Proposal

관련 NOVA-026/029/035/038/044/046/047/048/073/074/076/078/081/083/087/091/094/095/136 및 P14/P15 Enum/sum·P24 recursive pattern·P25 guard·P26~P29 binding·P35/P36 rest 계약.
문제: Enum/sum payload의 필요 없는 suffix에 wildcard를 전부 나열해야 한다. 생략이 tag 검사·전체 initializer 평가를 건너뛰어서는 안 된다.
변경: payload가 있는 qualified variant pattern에 prefix+trailing rest를 허용한다. nullary variant의 기존 괄호 금지를 유지한다.
이유: Source marker·bounded matrix·prefix const/MIR를 재사용하며 활성 variant의 payload만 읽는다.
대안: middle rest·named payload·rest binder·nullary 괄호 허용·Array를 함께 확장한다. 이번에는 Copy payload suffix 생략만 지원한다.
Backward Compatibility: rest 없는 arity·nullary parentheses N2201·intrinsic specialization·nominal/import head·alias head 금지·struct/Tuple rest를 보존한다. P36 enum_rest.nova와 당시 N1102 metadata는 수정하지 않고 현재 성공 override를 별도로 기록한다.

## 승인 범위

1. pattern_arguments 하나를 확장하고 payload_pattern_rest = ".."를 추가한다. 총 **69개 production / 기존 67개 보존**이다.
   E::Data(first,..), E::Data(..), E::Data(first,second,..,)를 허용한다. 명시 prefix가 전체 payload 수와 같아도 rest를 허용한다.
   Option::Some(..), Option::Some(value,..), Result::Success(..), Result::Error(..)는 scrutinee의 기존 concrete Copy specialization을 따른다.
   E::End(..), E::End(), Option::None(..), Option::None() 등 nullary variant의 괄호는 N2201이다. E::End·Option::None·none은 기존대로다.
   rest는 한 번, 마지막이며 두 Dot token이 인접해야 한다. middle/duplicate/spaced rest·rest binder·named payload·root rest는 N1102다. Lexer/END는 변경하지 않는다.
2. AST/HIR는 Source 순서의 명시 prefix와 정확한 .. Span의 마지막 PatternRest를 저장한다. source spelling/shape/order/parent·punctuation/trivia를 검증한다.
   Checked Variant children은 명시 prefix만 가지며 ordinal=0..prefix 길이로 payload에 대응한다. field_indices는 기존 non-product 규칙대로 비어 있다.
   marker는 Unit/Wildcard bookkeeping으로 source pattern node 한 개를 센다. implicit wildcard에 synthetic Source/HIR node·DefId를 만들지 않는다.
3. nonempty payload의 prefix가 arity보다 길면 N2201이다. 다른 family/nominal component는 N2101·undefined variant/name과 alias head는 기존 진단이다.
   binder 타입/정수 문맥·scope·중복 N2002·불변 N3004·module/import/가시성을 보존한다. 생략만으로 private nominal type나 head 접근을 허용하지 않는다.
4. matrix는 선택한 Variant의 전체 payload vector를 wildcard로 만들고 explicit prefix만 채운다. tag는 생략하지 않는다.
   N3101/N3102·guard 없는 합집합·guard 정책·기존 witness/순서/예산을 유지한다. E::Data(..)는 다른 variant를 덮지 않는다.
   let/var/local/global const는 단일 variant Enum과 irrefutable explicit prefix에만 허용한다. Option/Result/여러 variant 또는 refutable prefix는 N3103·첫 refutable Source secondary이며 const 평가/파생 오류를 억제한다.
5. initializer/scrutinee 전체를 한 번 평가·snapshot한다. 생략한 payload operand의 effect·checked Abort·try 전파를 건너뛰지 않는다.
   MIR은 tag 검사 뒤 활성 payload의 explicit prefix만 ordinal로 읽는다. rest-only에도 tag 검사와 guard의 성공 후 단일 평가를 유지한다.
   const도 전체 initializer를 평가한 뒤 prefix binder cache를 저장한다. whole value·forward/static cycle·default·10,000-node const 예산을 보존한다.
6. Source marker/parent/shape·Checked children/type/cache/field_indices·MIR payload index/Source/tag/guard branch 변조는 원 HIR 재계산과 기존 private proof로 거부한다.
   arm 1,025·source pattern nodes 10,000·matrix tasks 100,000·live cells 1,000,000·nesting 128·기존 layout 한도를 유지한다.
   전체 value domain/Cartesian product를 열거하지 않는다. LLVM/Runtime production·private ABI/layout·Lexer/END·Cargo dependency·diagnostic registry는 변경하지 않는다.
7. middle rest·rest binder·named Enum payload·nullary 괄호 완화·shorthand/brace·Array·String/Move payload·parameter/for/assignment pattern·const 함수·일반 Move/borrow/Drop은 제외한다.

## 수용 계획

rest-only/전체 prefix·nullary 거부/기존 bare nullary·다른 variant tag·nested product/sum·coverage·const cache/forward/실패/순환·scope/mutability·omitted payload effect/Abort/try를 검증한다.
UTF-8 source hash·code별 byte Span·P36 historical override·Source/Checked/MIR 변조·10,000/10,001-node 및 128/129 nesting·독립 Bool coverage oracle·LLVM COFF/ELF O0/O2·Windows Native debug/release·예제/사용자 명령을 제공한다.
