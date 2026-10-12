# P32 고정 폭 정수 literal 패턴 계약

2026-10-12. 상태: **Accepted / 구현·검증 완료**.
사용자 “다음 작업 승인할태니 개발 진행해줘”의 다음 단계 승인으로 이 제한된 계약을 구현 전에 기록한다.
P01~P31·D01~D05·Canonical·원본 148개를 보존한다. 전체 D07/D08/D10/D12/D16/D25/D30 승인이 아니다.

## Specification Change Proposal

관련 NOVA-014/026/029/035/038/044/046/047/048/073/074/076/078/081/083/087/091/094/095/136 및 P07 정수·P24 recursive pattern·P25 guard·P26~P29 binding·P30 Copy 값 match·P31 char 패턴 계약.
현재 정수 root/nested 값은 binder/wildcard·guard로만 검사하며 literal은 N1102다.
문제: signed 최소/unsigned 최대·서로 다른 radix의 같은 값·유한 Int8/UInt8 domain의 완전성을 pattern 모델에 정의해야 한다.
변경: INT 또는 minus+INT 패턴을 component의 고정 폭 정수 타입으로 해석하고 bounded observed/complement matrix와 MIR equality에 연결한다.
이유: 기존 P07 checked 숫자 의미를 사용하며 LLVM이나 host 정수로 의미를 결정하지 않는다.
대안: float/string literal·cast/range/or/field pattern까지 확장한다. 이번에는 8종 정수 literal subset만 적용한다.
Backward Compatibility: P01~P31 의미·guard/snapshot·예산·opaque-only integer witness를 보존한다. 과거 INT 거부 source/metadata는 별도 성공 override로 보존한다.

## 승인 범위

1. P31 62-production pattern 하나에 integer_pattern을 추가하고 integer_pattern = [minus], INT production을 추가한다.
   총 **63개 production / 기존 61개 보존**이다. INT spelling·radix·underscore는 D01~D05/P07 그대로이며 plus prefix·일반 prefix/group/cast/float/string/range/or/field pattern은 제외한다.
   minus/INT 원 token Span·trivia·byte boundary·child 없는 AST/HIR PatternInteger를 보존한다. expression Integer/Prefix와 구분한다.
2. pattern 기대 타입은 scrutinee/component의 Int8/16/32/64·UInt8/16/32/64다. literal로 scrutinee 타입을 추론·승격하거나 기본 Int32/peer 문맥을 추가하지 않는다.
   정수 외 정상 타입은 N2101, ErrorType의 파생 진단은 억제한다. P07 parser로 range를 검사하며 초과는 pattern 전체 Span N2102다.
   signed MIN magnitude를 허용하고 unsigned의 minus 표기는 **-0 포함 N2102**다. 같은 signed 타입의 0/-0은 같은 값이다.
   decimal/hex/binary/octal/underscore의 같은 decoded 값은 같은 typed constructor다.
3. matrix는 관측한 unguarded typed integer values와 component 타입의 첫 missing 대표 하나만 partition한다.
   observed values를 숫자 순서로 정렬하고 타입 MIN부터 첫 gap을 찾는다. 전체 64-bit domain/Cartesian product를 열거하지 않는다.
   missing witness는 decimal이다. literal 없는 기존 opaque column의 _ witness를 보존한다.
   Int8/UInt8 모든 256개 값의 unguarded 합집합은 fallback 없이 완전할 수 있다. 다른 타입도 같은 정확한 domain 의미와 기존 자원 한도를 따른다.
4. guard 없는 literal만 coverage에 기여한다. duplicate decoded literal/catchall 뒤 literal은 N3102, 누락은 N3101이다.
   guarded literal 뒤 같은 literal은 도달 가능하며 guard true/false·known scrutinee 값으로 분석을 좁히지 않는다.
   nested Tuple/Enum/Option/Result 및 기존 Char/Bool 패턴을 조합한다. bare 이름은 계속 binder이며 const equality lookup을 하지 않는다.
5. integer literal을 포함한 let/var/local·global const compound binding은 refutable N3103이다. initializer 값으로 좁히지 않는다.
   첫 refutable literal에 secondary를 제공하고 const 계산/파생 const 오류를 억제한다. parameter/for/assignment pattern은 추가하지 않는다.
6. scrutinee를 한 번 snapshot하고 정확한 타입/폭의 equality를 검사한 뒤 기존 guard/body로 간다. 실패/false는 다음 arm이다.
   enum payload는 tag proof 아래 projection한다. guard print/try·checked Runtime Abort·Source·return/jump 의미는 기존 규칙이다.
7. MIR은 기존 fixed-width Constant·EqualEqual·Bool Branch를 사용하고 비교 SourceInfo는 원 literal pattern이다.
   AST sign/digits/source·typed value/kind/type·Checked cache·MIR constant/branch/projection proof 변조를 원 HIR에서 재계산하여 거부한다.
   LLVM/Runtime production·정수 ABI/layout·Lexer/END·diagnostic registry/Cargo dependency는 변경하지 않는다.
8. arm 1,025·pattern nodes 10,000·matrix tasks 100,000·live cells 1,000,000·nesting/layout 예산을 유지한다.
   Array·String/Move payload·float/string literal·field/or/range pattern·match expression·const 함수·일반 Move/borrow/Drop은 제외한다.

## 수용 계획

8종 min/MAX·radix/zero alias·Int8/UInt8 full domain/one missing·nested product/sum·guard/try/Abort·refutability·range/type/cascade를 검증한다.
고정 UTF-8 source·진단 code별 byte Span·AST/Checked/MIR forgery·LLVM COFF/ELF O0/O2·Windows Native debug/release·독립 예제를 제공한다.
