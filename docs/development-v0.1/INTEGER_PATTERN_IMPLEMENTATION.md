# P32 고정 폭 정수 literal 패턴 구현·검증

2026-10-12. Accepted / 구현·검증 완료.
사용자 “다음 작업 승인할태니 개발 진행해줘”에 따라 [제한 계약](INTEGER_PATTERN_STAGE_B_PROPOSAL.md)을 구현 전에 기록했다.
전체 D07/D08/D10/D12/D16/D25/D30 승인이 아니다.

## Implemented / Changed

Parser는 INT 또는 minus+INT를 child 없는 PatternInteger로 저장하고 sign/digits token Span을 보존한다.
HIR은 expression Integer/Prefix와 구분하고 원 digits spelling·minus·trivia·ASCII radix/underscore·token/byte boundaries를 검증한다.
Type Checker는 component의 8종 고정 폭 정수 타입과 기존 P07 parser로 literal을 검사한다. pattern에 default Int32/peer 승격을 적용하지 않는다.
정수 외 타입은 N2101, 범위 초과와 unsigned minus(-0 포함)는 N2102다. signed MIN·decimal/hex/binary/octal/underscore alias를 유지한다.
Matrix constructor는 exact kind와 값이다. 관측한 unguarded 값과 MIN부터 첫 gap 하나를 partition하며 전체 64-bit domain/Cartesian product를 열거하지 않는다.
Int8/UInt8의 256개 전체 합집합을 정확히 분석하고 one-missing witness를 decimal로 표시한다. literal 없는 opaque-only witness는 _다.
중복 alias/0/-0·catchall 뒤 literal은 N3102, 누락은 N3101이다. guard true/false·known scrutinee로 분석을 좁히지 않는다.
let/var/local·global const compound literal 패턴은 N3103이며 literal secondary와 const 계산/파생 진단 억제를 유지한다.
MIR은 기존 width-preserving Constant·EqualEqual·Bool Branch를 사용하고 enum tag proof 아래 payload projection·guard 순서를 유지한다.
Source sign/digits·typed value/kind/type·MIR constant/branch 변조를 기존 원 HIR 재계산 및 private full-body proof로 거부한다.
Grammar는 pattern 하나를 확장하고 integer_pattern 하나를 추가해 63 productions다. 다른 61개 production을 보존한다.
LLVM/Runtime production·ABI/layout·Lexer/END·Cargo dependencies·diagnostic registry는 변경하지 않았다.

## Tests Added / Validation

새 테스트 15개: 기본 12·LLVM opt-in 1·Native opt-in 2.
8종 min/MAX/범위 초과·unsigned -0·radix/zero alias·ErrorType cascade·nested tuple/sum·refutable binding/const 실패 억제·guard/try/return을 검사한다.
독립 finite-domain oracle와 비교해 Int8/UInt8 complete/one-missing coverage를 확인한다. I64/U64의 min/max/missing 대표와 작은 task 수를 확인한다.
Parser sign/digits/trivia·UTF-8 prefix 결정적 복구·AST sign/digits/shape/value-kind 변조를 검사한다.
Checked value/width/type 및 MIR comparison constant/failure branch 변조를 거부한다.

| 검사 | 실제 결과 |
|---|---|
| fmt / Runtime rustfmt / all-features check / all-targets all-features Clippy -D warnings | PASS |
| 전체 기본 tests | 495 PASS / 0 FAIL / 99 ignored |
| LLVM 전체 opt-in COFF/ELF O0/O2 | 28 PASS / 0 FAIL |
| Windows Native 전체 opt-in debug/release | 71 PASS / 0 FAIL |

총 594 PASS / 0 FAIL. Native는 P32 두 개를 먼저 실행하고 기존 69개를 --skip p32_로 실행하여 중복 없는 전체 71개를 검증했다.
[고정 fixture](integer-pattern-proposal-fixtures/README.md)는 두 파일·정상 3·부정 18 code별 byte Span·Runtime 1을 검사했다.
Native 두 profile에서 8종 경계·full UInt8 match·single scrutinee·nested signed/radix/Char·guard 순서/생략·try/loop·checked Abort를 관측했다.
LLVM에서는 i8/i64 equality와 양 target의 COFF/ELF O0/O2 object를 확인했다.
[독립 예제](../../examples/integer_patterns.nova)는 negative와 nested=true 두 줄/LF·빈 stderr·exit 0이다.
P24/P30 literal_pattern.nova·P31 integer_pattern.nova의 당시 N1102 source/metadata를 보존하고 현재 성공을 P32 historical overrides·Driver·Native에 명시했다.

## Specification / Remaining

관련 NOVA-014/026/029/035/038/044/046/047/048/073/074/076/078/081/083/087/091/094/095/136 보완 문서에 제한된 P32 bridge를 추가했다.
P01~P31 ledger·D01~D30 본문·Canonical·원본 148개·기존 fixtures/grammar bytes를 보존하고 문서 validator/generation idempotency를 별도 검증했다.
Rust 1.99.0·LLVM 21.1.8·Windows x64 MSVC에서 검증했다. MSRV 1.80·Linux Native host 실행은 이번에 검증하지 않았다.
CARGO_INCREMENTAL=0은 검증 프로세스에만 설정했다. Windows hard-link/junction 기본 검사는 sandbox 밖에서 실행했다.
float/string literal·field/or/range pattern·Array·String/Move payload·match expression·const 함수·일반 Move/borrow/Drop은 후속 계약이다.
[TESTING.md](../../TESTING.md)에 직접 실행 명령을 제공한다.
