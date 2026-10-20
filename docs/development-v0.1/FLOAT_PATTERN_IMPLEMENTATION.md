# P41 float literal 패턴 구현·검증

2026-10-20. Accepted / 구현·검증 완료.
사용자 “다음 작업 승인할태니 개발 해줘”의 다음 단계 승인으로 [제한 계약](FLOAT_PATTERN_STAGE_B_PROPOSAL.md)을 구현 전에 기록했다.
전체 D07/D08/D09/D10/D12/D16/D25/D30 승인이 아니다.

## Implemented / Changed

root match와 기존 Tuple/struct/Enum/Option/Result nested pattern에서 FLOAT와 minus FLOAT를 지원한다. sign trivia·digits/minus Source subspan·END를 보존한다.
AST/HIR PatternFloat는 child 없는 pattern이며 expression Float와 구분한다. FLOAT spelling은 기존 validator를 재사용하고 sign/spelling/boundary/parent/shape 변조를 거부한다.
기대 Float32/Float64로 decimal에서 직접 반올림한다. Float64가 Float32를 경유하지 않는다. 다른 타입은 N2101, finite overflow는 FLOAT digits primary N2102와 실제 선언 field/annotation secondary다.
subnormal/underflow는 허용하고 minus는 rounded finite 값의 sign bit를 반전하므로 negative zero를 보존한다. Type::Error 파생 진단을 억제한다.
RecursivePatternKind::Float에 원래 bits/kind를 저장하며 expression float_literals cache는 사용하지 않는다. 기존 unsupported-float-host gate를 pattern-only 코드에도 적용했다.
coverage는 같은 타입에서 동일 rounded value를 하나의 region으로 취급하고 zero sign만 합친다. typed/MIR metadata는 zero sign을 보존한다.
관측 finite bits와 항상 남는 Opaque complement로 분할한다. complement에는 다른 finite 값·Infinity·NaN이 포함된다. float pattern 없는 기존 opaque-only witness는 유지하며 전체 domain을 열거하지 않는다.
finite literal만으로 exhaustive하지 않다. N3101/N3102·unguarded 합집합·guard 정책·기존 witness 순서를 유지한다. 관측값 witness는 roundtrip decimal, complement는 _다.
nested let/var/local/global const의 float literal은 N3103이며 알려진 initializer로 좁히지 않는다. Source 순서의 첫 refutable secondary·const 평가/파생 오류 억제를 유지한다.
MIR은 scrutinee 단일 snapshot 뒤 기존 float Constant·Binary Equal·Branch를 사용한다. 기존 LLVM의 ordered fcmp oeq가 zero equality/NaN 불일치/fallback을 수행한다.
기존 field/payload projection 순서·guard 성공 후 단일 평가·try 전파·checked Abort Source를 보존한다. const binder cache/forward/cycle/default는 기존 규칙이다.
Checked float bits/sign/kind/type/cache와 MIR 같은 타입 float constant/Source/branch 변조는 HIR 재계산과 private full-body proof로 거부한다.
Grammar는 pattern을 확장하고 float_pattern production을 추가했다. 총 71개이며 기존 다른 69개를 보존했다.
Production 변경은 AST/Parser/HIR·float host gate·타입 검사/coverage·MIR literal equality다. LLVM/Runtime production·private ABI/layout·Lexer/END·Cargo dependencies·diagnostic registry는 변경하지 않았다. Core는 LLVM-free다.

## Tests Added / Validation

새 테스트 22개: 기본 18·LLVM opt-in 1·Native opt-in 3.
독립적인 hardcoded bits 15개로 Float32/64의 0.1·16777217·halfway/just-above-halfway·최대 finite·최소 subnormal·signed zero·negative underflow와 expression-cache 분리를 검사했다.
zero/decimal/exponent/underscore/rounded alias 중복, Float64의 16777216/16777217 구별, finite-only/guard-only 불완전성과 NaN/Infinity complement를 검사했다.
(Float64,Bool)의 관측 1.5·zero·나머지(complement) 여섯 region과 네 pattern의 16개 부분집합을 독립 oracle로 비교했다.
내부 matrix 테스트는 관측 finite 4개+항상 남는 complement, opaque-only fast path와 roundtrip witness를 확인했다.
같은 의미의 sparse guarded struct 패턴에서 10,000/10,001 source-node 경계를 검사했다. 별도의 dense Float product는 기존 1,000,000 live matrix cell 제한으로 N8901을 반환한다. 큰 concrete product가 node limit 전에 matrix limit에 도달할 수 있으며 한도를 늘리지 않았다.
128/129 nesting·모든 UTF-8 prefix 복구·Source/Checked/MIR 변조·negative-zero raw Constant·single make call을 검증했다.
imported alias Float64 bits와 다른 파일의 실제 Float32 field secondary, Unicode sign trivia·overflow digits primary를 확인했다.
P32 float_pattern.nova의 source·당시 N1102 metadata는 byte 보존하고 현재 성공 override를 추가했다.

| 검사 | 실제 결과 |
|---|---|
| fmt / Runtime rustfmt / all-features check / all-targets all-features Clippy -D warnings | PASS |
| 전체 기본 tests | 636 PASS / 0 FAIL / 127 ignored |
| LLVM 전체 opt-in COFF/ELF O0/O2 | 37 PASS / 0 FAIL |
| Windows Native 전체 opt-in debug/release | 90 PASS / 0 FAIL |

총 763 PASS / 0 FAIL. Native는 P41 세 개를 먼저 실행하고 기존 87개를 --skip p41_로 실행해 중복 없이 전체 90개를 검증했다.
[고정 fixture](float-pattern-proposal-fixtures/README.md)는 두 파일·정상 3·부정 21 code별 byte Span·Runtime 2 및 P32 override를 검증했다.
Native 독립 bit bank는 각 Float32/64의 finite·zero 양쪽 sign·rounding 경계·subnormal·Infinity 양쪽 sign·NaN 9개(총 18개)를 Rust primitive IEEE equality와 hardcoded pattern bits로 분류했다. Nova FloatValue/coverage helper를 oracle로 재사용하지 않았다. 두 profile을 실행했다.
LLVM은 결정적 ordered fcmp oeq IR·fast/ueq 부재·COFF/ELF O0/O2 object를 확인했다.
[독립 예제](../../examples/float_patterns.nova)는 literal=1, zero=2, nan=3 세 줄/LF·빈 stderr·exit 0이다.

## Specification / Remaining

NOVA-010/025/026/029/035/038/044/046/047/048/073/074/076/078/081/083/087/091/094/095/136 보완 문서 21개에 제한된 P41 bridge를 추가했다.
P01~P40 ledger·D01~D30 본문·Canonical·원본 148개·기존 fixtures/grammar bytes·사용자 변경을 보존하고 문서 validator/generation idempotency를 별도 검증했다.
Rust 1.99.0·LLVM 21.1.8·Windows x64 MSVC에서 검증했다. MSRV 1.80·Linux Native host 실행은 이번에 검증하지 않았다.
CARGO_INCREMENTAL=0은 검증 프로세스에만 설정했다. Windows hard-link/junction 기본 검사는 sandbox 밖에서 실행했다.
integer/float pattern 승격·float range/or·string literal·NaN/Infinity literal·const 이름 참조 패턴·struct shorthand·Array·일반 Move/borrow/Drop은 후속 계약이다.
[TESTING.md](../../TESTING.md)에 직접 실행 명령을 제공한다.
