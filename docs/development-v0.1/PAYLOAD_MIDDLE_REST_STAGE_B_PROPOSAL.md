# P39 Copy Enum·sum payload 중간 rest 패턴 계약

2026-10-20. 상태: **Accepted / 구현·검증 완료**.
사용자 “다음 작업 승인할태니 개발 해줘”의 다음 단계 승인으로 이 제한 계약을 구현 전에 기록한다.
P01~P38·D01~D05·Canonical·원본 148개를 보존한다. 전체 D06/D08/D09/D10/D12/D16/D25/D30 승인이 아니다.

구현·검증 완료: 2026-10-20. 승인일 2026-10-20.

## Specification Change Proposal

관련 NOVA-026/029/035/038/044/046/047/048/073/074/076/078/081/083/087/091/094/095/136 및 P14/P15 Enum/sum·P24 recursive pattern·P25 guard·P26~P29 binding·P37/P38 rest 계약.
문제: Enum payload suffix를 선택하려면 생략할 중간 요소를 모두 wildcard로 나열해야 한다. suffix 타입·실제 Source annotation·MIR 활성 tag와 payload index를 정확히 대응해야 한다.
변경: qualified Copy Enum/Option/Result payload에서 하나의 ..를 시작/중간/끝에 허용한다. struct rest는 trailing-only를 유지한다.
이유: P38의 prefix/suffix 대응을 공유하고, 실제 index를 const·coverage·MIR의 단일 계약으로 사용한다.
대안: struct middle rest·rest binder·named payload·Array를 함께 확장한다. 이번에는 Copy variant payload의 prefix/suffix 선택만 지원한다.
Backward Compatibility: rest 없는 arity·nullary parentheses N2201·family/nominal identity·alias head 금지·P38 Tuple middle rest·P35 struct trailing rest를 보존한다. P37 middle_rest.nova와 P38 payload_middle.nova의 source/당시 N1102 metadata는 보존하고 현재 성공 override를 기록한다.

## 승인 범위

1. pattern_arguments 한 production만 확장한다. 총 **70개 production / 기존 69개 보존**이며 새 production은 없다.
   E::Data(first,..,last), E::Data(..,last), E::Data(first,..,a,b,)를 허용한다. P37 trailing/rest-only도 유지한다.
   concrete intrinsic Option/Result에서도 Option::Some(..,value), Result::Success(..,value), Result::Error(..,error)를 허용한다. 1개 payload에서 rest가 생략하는 요소는 0개일 수 있다.
   marker는 하나만 허용한다. 중복/spaced dots·rest binder·named payload·root rest·struct middle rest는 N1102다. Lexer/END는 변경하지 않는다.
   nullary E::End(..)/E::End()/Option::None(..)/Option::None()는 기존 N2201이다. E::End·Option::None·none은 기존대로다.
2. payload arity n, explicit prefix p, suffix s에서 p+s<=n이어야 한다. 초과하면 N2201이고 rest 없는 정확한 arity도 기존대로다.
   prefix index는 0..p, suffix는 n-s..n이다. AST/HIR는 marker 포함 Source 순서, Checked children은 marker 제외 Source 순서다.
   field_indices는 이제 Struct/Tuple/Variant의 모든 explicit component와 일대일 대응한다. rest 없는 Variant도 natural indices를 저장한다. P37의 non-product empty 규칙에서 이 Variant 부분만 확장한다.
   component 타입 불일치는 N2101, 정수 범위 초과는 N2102다. suffix 타입/secondary annotation은 실제 variant 선언의 해당 요소를 가리킨다. annotation이 없으면 None이며 가상의 Source를 만들지 않는다.
   marker는 Unit/Wildcard bookkeeping으로 source pattern node 한 개를 센다. implicit wildcard에 synthetic Source/HIR node·DefId를 만들지 않는다.
3. matrix는 선택한 variant의 전체 payload vector를 wildcard로 만들고 field_indices만 채운다. tag를 생략하지 않는다.
   N3101/N3102·guard 없는 합집합·기존 witness/순서/예산을 유지한다. E::Data(..,last)는 다른 variant를 덮지 않는다.
   let/var/local/global const는 단일 variant Enum과 irrefutable explicit component에만 허용한다. 여러 variant/Option/Result 또는 refutable prefix/suffix는 N3103·첫 refutable Source secondary이며 const 평가/파생 오류를 억제한다.
   binder 타입/정수 문맥·scope·중복 N2002·불변 N3004·import/visibility·alias head 금지·nested product/sum은 기존 규칙이다.
4. initializer/scrutinee 전체를 한 번 평가·snapshot한다. 생략한 중간 operand의 effect·checked Abort·try 전파를 건너뛰지 않는다.
   MIR은 tag 검사 뒤 활성 payload의 실제 prefix/suffix index만 읽는다. rest-only에도 tag 검사와 guard의 성공 후 단일 평가를 유지한다.
   const도 전체 initializer 평가 후 실제 index의 binder cache를 저장한다. whole value·forward/static cycle·default·10,000-node const 예산을 보존한다.
5. Tuple/Variant rest 대응 helper를 공유한다. Source marker spelling/shape/order/parent·중복 marker·Checked children/indices/type/cache·MIR 같은 타입 payload index/Source/tag/guard 변조는 기존 gate와 private proof로 거부한다.
   arm 1,025·source pattern nodes 10,000·matrix tasks 100,000·live cells 1,000,000·nesting 128·기존 layout 한도를 유지한다.
   전체 value domain/Cartesian product를 열거하지 않는다. LLVM/Runtime production·private ABI/layout·Lexer/END·Cargo dependency·diagnostic registry는 변경하지 않는다.
6. struct middle rest·rest binder·named Enum payload·nullary 괄호 완화·shorthand/brace·Array·String/Move payload·parameter/for/assignment pattern·const 함수·일반 Move/borrow/Drop은 제외한다.

## 수용 계획

leading/middle/trailing/rest-only·빈 생략·intrinsic 1개 payload·nullary 거부·다른 variant tag·실제 suffix type/annotation/index·nested product/sum·coverage·const cache/forward/실패/순환·scope/mutability·omitted middle effect/Abort/try를 검증한다.
UTF-8 source hash·code별 byte Span·P37/P38 historical override·Source/Checked/MIR 변조·10,000/10,001-node 및 128/129 nesting·독립 Bool coverage oracle·LLVM COFF/ELF O0/O2·Windows Native debug/release·예제/사용자 명령을 제공한다.
