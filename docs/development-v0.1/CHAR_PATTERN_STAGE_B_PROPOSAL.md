# P31 Unicode char literal 패턴 계약

작성일 2026-10-12. 상태: **Accepted / 구현·검증 완료**.
사용자 “다음 작업 승인할태니 개발 진행해줘”의 다음 단계 승인으로 이 제한된 계약을 구현 전에 기록한다.
P01~P30·D01~D05·Canonical·원본 148개를 보존한다. 전체 D08/D10/D12/D16/D25/D30 승인이 아니다.

## Specification Change Proposal

관련 NOVA-014/027/029/035/038/044/046/047/048/073/074/076/078/081/083/087/091/094/095/136 및 P08 char·P24 recursive pattern·P25 guard·P26~P29 destructuring·P30 Copy 값 match 계약.
현재 char는 root/nested binder·wildcard와 guard로 검사한다. literal 패턴은 N1102다.
문제: Unicode scalar 값의 직접 선택과 escape alias 중복·누락 검사를 pattern 모델에 정의해야 한다.
변경: pattern 위치의 CHAR token을 지원하고 기존 char decoder·타입/Source/coverage/MIR 검증에 연결한다.
이유: char equality의 기존 의미를 사용하며 Unicode domain을 모든 값으로 펼치지 않는다.
대안: 정수·float·string literal 및 field/or/range 패턴까지 확장한다. 이번에는 char literal만 허용한다.
영향: Parser/AST/HIR Source·typed pattern·bounded matrix·MIR equality branch·테스트·문서.
Backward Compatibility: P01~P30 의미·guard 순서·scrutinee snapshot·예산·opaque-only char witness를 보존한다.
P30 char_pattern.nova의 source와 당시 N1102 metadata는 보존하고 현재 성공을 별도 override로 기록한다.

## 승인 범위

1. P29/P30의 62-production EBNF 중 pattern 하나에 CHAR를 추가하고 나머지 **61개 보존**한다.
   root match 및 Tuple/Enum/Option/Result nested pattern에서 기존 P08 Unicode scalar literal/escape를 사용한다.
   새 AST/HIR PatternCharacter는 값 expression Character와 구분하고 token byte Span·single scalar decoding·child 없는 shape를 검사한다.
   integer/float/string literal·grouped/or/range/field pattern은 계속 제외한다. Lexer/END 규칙은 변경하지 않는다.
2. char literal은 Char component에만 허용한다. 다른 정상 타입은 literal Span에 N2101이며 ErrorType의 파생 오류는 억제한다.
   숫자 cast·일반 이름 const equality pattern·Unicode normalization은 추가하지 않는다.
   같은 decoded scalar인 raw/escape 표기는 같은 constructor다. 예: 'A'와 '\u{41}'은 같은 값이다.
3. existing bounded constructor matrix에 Char(scalar)를 추가한다. wildcard candidate에서 관측한 unguarded char literals와 첫 누락 Unicode scalar 하나만 partition한다.
   observed literals는 scalar 순서로 정렬하고 surrogate를 건너뛰어 첫 missing representative를 찾는다. 구체 Unicode domain/Cartesian product를 열거하지 않는다.
   literal이 없는 기존 opaque-only char column은 Opaque/_ witness를 유지한다. missing char witness는 Rust escape_default 기반 quoted literal로 표시한다.
   guard 없는 literal 합집합만 coverage에 기여한다. 현재 자원 한도 안의 literal 목록만으로 Char 전체가 완전해지지 않으며 binder/`_` fallback이 필요하다. 불완전성은 기존 N3101이다.
   같은 decoded literal 중복·catchall 뒤 literal은 N3102다. guarded literal 뒤 동일 literal은 도달 가능하며 기존 guard truth 분석은 하지 않는다.
4. char literal이 들어간 let/var/local·global const compound binding은 refutable이므로 N3103이다.
   initializer가 같은 값을 가진다는 이유로 성공시키지 않는다. N3103 secondary는 해당 literal을 포함한 첫 refutable component다.
   실패한 const pattern은 initializer 계산과 파생 const 진단을 억제한다. parameter/for/assignment pattern은 추가하지 않는다.
5. scrutinee를 한 번 snapshot하고 char equality를 검사한다. 성공 후 기존 guard를 한 번 평가하며 실패/false면 다음 arm으로 간다.
   Enum payload는 기존 tag proof 아래 projection한 뒤 literal equality를 검사한다. Unicode/NUL/escape·guard print/try·Runtime checked Abort 의미는 기존 규칙이다.
6. MIR은 Char constant·기존 EqualEqual Rvalue/Bool branch로 표현하고 비교의 SourceInfo는 literal pattern이다.
   AST Source·decoded typed pattern 값·Checked cache·MIR constant/branch/projection proof 변조를 원 HIR에서 재계산하여 거부한다.
   LLVM/Runtime production·Char private i32 ABI·layout·새 diagnostic code는 변경하지 않는다.
7. 기존 arm 1,025·pattern node 10,000·matrix task 100,000·live cell 1,000,000·nesting/layout 예산을 유지한다.
   Array·String/Move payload·정수/float/string literal·struct field/or/range pattern·match expression·const 함수·일반 Move/borrow/Drop은 제외한다.

## 수용 계획

raw/escape alias·Unicode/NUL/control/min/max scalar·nested tuple/sum·guard order·known-value 불완전성/irrefutability·타입/cascade를 검증한다.
고정 UTF-8 fixture·진단 byte Span·Source/Checked/MIR forgery·독립 예제·LLVM COFF/ELF O0/O2·Windows Native debug/release를 제공한다.
