# P41 float literal 패턴 계약

2026-10-20. 상태: **Accepted / 구현·검증 완료**.
사용자 “다음 작업 승인할태니 개발 해줘”의 다음 단계 승인으로 이 제한 계약을 구현 전에 기록한다.
P01~P40·D01~D05·Canonical·원본 148개를 보존한다. 전체 D07/D08/D09/D10/D12/D16/D25/D30 승인이 아니다.

구현·검증 완료: 2026-10-20. 승인일 2026-10-20.

## Specification Change Proposal

관련 NOVA-010/025/026/029/035/038/044/046/047/048/073/074/076/078/081/083/087/091/094/095/136 및 P09 float·P24 recursive pattern·P25 guard·P26~P29 binding·P30 Copy value match·P32 정수 literal 패턴 계약.
문제: float 값은 이미 IEEE 비교가 가능하지만 literal match는 Parser에서 거부한다. decimal alias·직접 반올림·signed zero·NaN을 coverage와 실행에서 일치시켜야 한다.
변경: root/nested pattern에 FLOAT 또는 minus FLOAT를 허용한다.
이유: 기존 float 비교·Copy snapshot·bounded matrix를 재사용하며 별도 숫자 변환을 도입하지 않는다.
대안: 정수 범위·or·string literal·일반 Move 패턴까지 확장한다. 이번에는 기존 Float32/64의 finite literal만 지원한다.
Backward Compatibility: INT 패턴은 정수 전용이며 float로 승격하지 않는다. 기존 char/Bool/product/sum·rest·alias/import·guard/try·평가 순서/ABI를 유지한다. P32 float_pattern.nova source와 당시 N1102 metadata는 보존하고 현재 성공 override를 별도 기록한다.

## 승인 범위

1. pattern에 float_pattern을 추가하고 float_pattern = [ "-" ], "FLOAT" production을 추가한다. 총 **71개 production / 기존 69개 보존**이다.
   1.5, 1e3, -0.0, - /*trivia*/ 1.5를 root match·Tuple/struct/Enum/Option/Result 안에서 허용한다. digits와 minus subspan을 보존한다.
   unary plus·grouped pattern·arithmetic/cast/이름 상수 패턴·NaN/Infinity 전용 literal은 추가하지 않는다. bare IDENT는 기존 binder다. Lexer/END는 변경하지 않는다.
2. 기대 component의 Float32/Float64 종류로 decimal에서 직접 round-to-nearest ties-to-even한다. Float64를 Float32 경유하지 않는다. 기대 타입 없는 기본값/peer inference는 pattern에 없다.
   다른 component 타입은 N2101, 반올림 후 Infinity overflow는 N2102다. N2102 primary는 FLOAT digits token, annotation이 있으면 실제 component 선언 secondary다.
   subnormal/underflow-to-zero를 허용한다. minus는 rounded finite 값의 sign bit를 반전해 negative zero를 보존한다. Type::Error에는 파생 오류를 추가하지 않는다.
   HIR는 Source spelling과 digits/minus를 보존하고 타입 검사 후 float pattern value는 RecursivePatternKind에 저장한다. expression float_literals cache와 혼동하지 않는다.
3. match는 P09 IEEE Equal이다. +0/-0는 같으며 NaN scrutinee는 어떤 finite literal과도 같지 않다. Infinity scrutinee도 finite literal과 같지 않다.
   typed pattern/const metadata는 signed-zero bits를 보존하지만 coverage key는 zero 부호를 합친다. decimal/exponent/underscore alias와 같은 타입에서 동일하게 반올림되는 literal도 같은 region이다.
   matrix는 관측 finite Float bits(0 정규화)와 항상 남는 Opaque complement 하나로 분할한다. complement에는 다른 finite 값·Infinity·NaN이 포함된다. 전체 float domain을 열거하지 않는다.
   guarded arm은 coverage를 늘리지 않는다. finite literal만으로는 exhaustiveness를 만족하지 못한다. root에서는 unguarded binder/wildcard, nested에서는 complement를 덮는 기존 패턴이 필요하다. N3101/N3102와 기존 witness 순서를 유지한다.
   관측값 witness는 rounded 값의 roundtrip decimal이며 complement는 _다. float 패턴 없는 기존 opaque-only witness는 유지한다.
4. let/var/local/global const의 nested float literal은 refutable이며 N3103다. 알려진 initializer 값으로 irrefutable 처리하지 않는다. 첫 refutable secondary는 Source 순서이며 initializer const 평가/파생 실패를 억제한다.
   binder scope·중복·가시성·local/global const cache/forward/cycle/default는 기존 규칙이다.
5. scrutinee 전체 단일 평가·snapshot 뒤 literal 비교를 실행한다. prefix/suffix projection·선언 순서·guard 성공 후 단일 평가·false 시 다음 arm·try 전파·Abort Source를 유지한다.
   MIR은 기존 float Constant와 Binary Equal/Branch를 사용한다. LLVM은 기존 ordered fcmp oeq이며 fast-math·ABI·Runtime 변경이 없다.
6. Source FLOAT spelling·minus adjacency/trivia·subspan/parent/child 없는 shape, Checked float bits/kind/type와 MIR 같은 타입 float bits/Source/branch 변조를 원 HIR 재계산과 private full-body proof로 거부한다.
   arm 1,025·source pattern nodes 10,000·matrix tasks 100,000·live cells 1,000,000·nesting 128·layout 한도를 유지한다. 기존 float host/environment 검사를 pattern-only 코드에도 적용한다.
   Core는 LLVM-free다. LLVM/Runtime production·private ABI/layout·Lexer/END·Cargo dependency·diagnostic registry는 변경하지 않는다.
7. 정수/float 혼합 pattern 승격·float range/or·string literal·NaN/Infinity literal·const 이름 참조 패턴·struct shorthand·Array·String/Move payload·일반 Move/borrow/Drop은 제외한다.

## 수용 계획

Float32/64 직접 반올림·decimal alias·signed zero·subnormal/underflow·overflow/type 진단·NaN/Infinity fallback·중첩 product/sum/rest·guard/try·single snapshot·refutable const 억제를 검증한다.
독립 bit/IEEE 비교 oracle·bounded matrix complement·자원 경계·Source/Checked/MIR 변조·고정 UTF-8/LF hash/code별 byte Span·P32 historical override·LLVM COFF/ELF O0/O2·Windows Native debug/release·사용자 예제/명령을 제공한다.
