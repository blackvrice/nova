# P30 숫자·char·Copy struct statement match 구현·검증

작성일 2026-10-12. Accepted / 구현·검증 완료.
사용자 “다음 개발 승인할테니 개발 진행해줘”에 따라 [최소 계약](COPY_VALUE_MATCH_STAGE_B_PROPOSAL.md)을 구현 전에 기록했다.
전체 D08/D10/D12/D16/D25/D30 승인이 아니다.

## Implemented / Changed

Type Checker의 root match domain을 8종 정수·Float32/64·Char·Copy Struct로 확장했다.
새 root는 기존 recursive pattern engine으로 보내고 bare binder/wildcard·Bool guard만 허용한다.
Binder의 exact type·immutable whole Copy·arm scope·ErrorType cascade suppression을 기존 규칙으로 검사한다.
Guard 없는 binder/wildcard가 전체 domain을 덮고 guarded arm만으로는 N3101이다. 뒤 unreachable arm은 N3102다.
상수 scrutinee·true/false guard·NaN을 근거로 완전성/도달성을 좁히지 않는다.
기존 opaque matrix domain을 사용하므로 숫자/char의 값 domain이나 struct field 조합을 열거하지 않는다.
MIR은 기존 recursive snapshot·binder copy·guard CFG·private full-body proof로 컴파일한다.
Lexer/Parser/AST/HIR/MIR/LLVM/Runtime production·ABI/layout·Cargo dependency·diagnostic registry를 변경하지 않았다.
P29 62-production EBNF를 byte 그대로 재사용하며 grammar_change=false다.

## Tests Added / Validation

새 테스트 13개: 기본 10·LLVM opt-in 1·Native opt-in 2.
8종 정수/float/char exact binder type·shadow/scope·immutable binder/field·guard 타입·잘못된 타입 pattern·String 제외를 검사한다.
모든 새 domain의 guarded-only N3101·catchall 뒤 N3102·constant/true/false guard의 보수적 분석을 검사한다.
return/loop jump/try·Read method·alias/import/private whole Copy 및 private field 거부·1,025/1,026 arm 경계를 검사한다.
Checked recursive pattern type/nested domain/guard plan 변조와 MIR guard condition/branch target 변조를 거부한다.
MIR에서 scrutinee 함수 호출 횟수를 확인하고 Native에서 한 번 평가·guard 순서·whole-value snapshot을 관측한다.

| 검사 | 실제 결과 |
|---|---|
| cargo fmt --check / Runtime rustfmt | PASS |
| all-features check / all-targets all-features Clippy -D warnings | PASS |
| 전체 기본 tests | 470 PASS / 0 FAIL / 93 ignored |
| LLVM 전체 opt-in COFF/ELF O0/O2 | 26 PASS / 0 FAIL |
| Windows Native 전체 opt-in debug/release | 67 PASS / 0 FAIL |

총 563 PASS / 0 FAIL. Native는 P30 두 개를 먼저 실행한 뒤 기존 65개를 --skip p30_로 실행했다. 합계는 중복 없는 전체 67개다.
[고정 fixture](copy-value-match-proposal-fixtures/README.md)는 두 파일·scalar 정상 1·부정 12 실제 byte Span·Runtime 1을 검증했다.
Int8/16/32/64 MIN·UInt8/16/32/64 MAX·Float32 NaN·Float64 NaN/negative zero·Unicode/NUL char·import/alias/private whole struct·Read method·try/loop를 Native 두 profile에서 관측했다.
Runtime guard overflow는 기존 Int32 전용 panic 문자열·file#0:41..53 operation Span·before-only 출력·exit 1을 유지한다.
[독립 예제](../../examples/copy_value_match.nova)의 두 줄은 `snapshot=-2/current=4`와 `char=🙂`이며 debug/release의 LF·빈 stderr·exit 0이다.
P14 invalid_scrutinee.nova·P24/P25 root_scalar.nova의 source/원 N2101 metadata는 보존했다. 현재 성공을 P30 historical override 및 Compiler/Native 테스트에 명시했다.

## Specification / Remaining

관련 NOVA-026/027/029/035/038/044/046/047/048/074/076/078/081/083/087/091/094/095/136 보완 문서에 P30 bridge를 추가했다.
P01~P29 ledger·D01~D30 본문·Canonical·원본 148개·기존 fixture hash·원 grammar bytes를 보존하고 생성 idempotency·문서 validator를 별도 확인했다.
Rust 1.99.0·LLVM 21.1.8·Windows x64 MSVC에서 검증했다. MSRV 1.80·Linux Native host 실행은 이번에 검증하지 않았다.
CARGO_INCREMENTAL=0은 검증 프로세스에만 설정했다. Windows hard-link/junction 기본 검사는 sandbox 밖에서 실행했다.
Array·String/Move·literal/struct field/or/range pattern·match expression·const 함수·일반 Move/borrow/Drop은 후속 계약이다.
[TESTING.md](../../TESTING.md)에 직접 실행 명령을 제공한다.
