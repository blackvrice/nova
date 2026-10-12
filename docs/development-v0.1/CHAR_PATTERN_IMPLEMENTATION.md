# P31 Unicode char literal 패턴 구현·검증

2026-10-12. Accepted / 구현·검증 완료.
사용자 “다음 작업 승인할태니 개발 진행해줘”에 따라 [제한 계약](CHAR_PATTERN_STAGE_B_PROPOSAL.md)을 구현 전에 기록했다.
전체 D08/D10/D12/D16/D25/D30 승인이 아니다.

## Implemented / Changed

Parser pattern 위치의 Character token을 별도 AST PatternCharacter로 저장한다. HIR은 기존 decoder로 하나의 Unicode scalar와 원 Span을 보존한다.
Pattern literal은 expression Character와 구분하고 child 없는 shape·quoted source·UTF-8 boundary를 검증한다.
Char 타입만 허용하고 raw/escape alias는 동일한 constructor다. 틀린 정상 타입은 N2101이며 ErrorType cascade는 억제한다.
Matrix는 관측한 unguarded literal들과 첫 누락 scalar 대표 하나로 Char domain을 partition한다. surrogate를 건너뛰고 scalar 순서를 유지한다.
Unicode 전체 값/Cartesian product를 열거하지 않는다. literal 없는 기존 opaque char column은 _ witness와 fast path를 보존한다.
중복 decoded literal/catchall 뒤 literal은 N3102, fallback 없는 유한 literal 합집합은 N3101이다. guard true/false·known scrutinee로 coverage를 좁히지 않는다.
let/var/local·global const compound literal 패턴은 N3103이며 literal secondary를 제공한다. const initializer 계산과 파생 오류를 억제한다.
MIR은 기존 Char constant·EqualEqual·Bool Branch로 literal을 검사한 뒤 guard/body로 들어간다. 기존 enum tag proof 뒤 payload projection을 유지한다.
Source/typed scalar/type/kind·MIR 비교값/branch 변조는 기존 원 HIR 재계산 및 private full-body proof로 거부한다.
P29/P30 62-production grammar의 pattern 하나만 확장하고 나머지 61개를 보존한다.
LLVM/Runtime production·Char private i32 ABI/layout·Lexer/END·Cargo dependency·diagnostic registry는 변경하지 않았다.

## Tests Added / Validation

새 테스트 16개: 기본 13·LLVM opt-in 1·Native opt-in 2.
Parser literal/value 구분·원 spelling·모든 UTF-8 prefix 결정적 복구·AST Source/shape 변조를 검사한다.
Raw/escape 중복·known-value 불완전성·nested tuple/sum union coverage·refutability·const 실패 억제·타입/cascade·guard/return/try·1,025/1,026 arm 경계를 검사한다.
독립 matrix tests는 observed/complement coverage·wildcard 전체 덮기·contiguous scalar의 surrogate 건너뛰기를 검사한다.
Checked scalar/type/kind 및 MIR Char constant/branch 변조를 거부한다.

| 검사 | 실제 결과 |
|---|---|
| fmt / Runtime rustfmt / all-features check / all-targets all-features Clippy -D warnings | PASS |
| 전체 기본 tests | 483 PASS / 0 FAIL / 96 ignored |
| LLVM 전체 opt-in COFF/ELF O0/O2 | 27 PASS / 0 FAIL |
| Windows Native 전체 opt-in debug/release | 69 PASS / 0 FAIL |

총 579 PASS / 0 FAIL. Native는 P31 두 개를 먼저 실행하고 기존 67개를 --skip p31_로 실행하여 중복 없는 전체 69개를 검증했다.
[고정 fixture](char-pattern-proposal-fixtures/README.md)는 두 파일·정상 2·부정 13 code별 실제 byte Span·Runtime 1을 검사했다.
NUL/Unicode MAX/D7FF/E000·newline/backslash/quotes/Hangul/emoji·raw/escape alias·single scrutinee·guard 순서/생략·try/Abort를 두 profile에서 관측했다.
[독립 예제](../../examples/char_patterns.nova)의 두 줄은 emoji와 nested=true이며 debug/release LF·빈 stderr·exit 0이다.
P30 char_pattern.nova의 당시 N1102 source/metadata를 보존하고 현재 성공을 P31 historical override·Driver·Native에 명시했다.

## Specification / Remaining

관련 NOVA-014/027/029/035/038/044/046/047/048/073/074/076/078/081/083/087/091/094/095/136 보완 문서에 제한된 P31 bridge를 추가했다.
P01~P30 ledger·D01~D30 본문·Canonical·원본 148개·기존 fixtures/grammar bytes를 보존하고 문서 validator/generation idempotency를 별도 검증했다.
Rust 1.99.0·LLVM 21.1.8·Windows x64 MSVC에서 검증했다. MSRV 1.80·Linux Native host 실행은 이번에 검증하지 않았다.
CARGO_INCREMENTAL=0은 검증 프로세스에만 설정했다. Windows hard-link/junction 기본 검사는 sandbox 밖에서 실행했다.
정수/float/string literal·field/or/range pattern·Array·String/Move payload·match expression·const 함수·일반 Move/borrow/Drop은 후속 계약이다.
[TESTING.md](../../TESTING.md)에 직접 실행 명령을 제공한다.
