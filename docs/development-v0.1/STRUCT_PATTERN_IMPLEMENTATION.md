# P33 Copy struct 위치 패턴 구현·검증

2026-10-12. Accepted / 구현·검증 완료.
사용자 “다음 작업 승인할태니 개발 진행해줘”에 따라 [제한 계약](STRUCT_PATTERN_STAGE_B_PROPOSAL.md)을 구현 전에 기록했다.
전체 D06/D08/D09/D10/D12/D16/D25/D30 승인이 아니다.

## Implemented / Changed

Parser는 IDENT 뒤 괄호 lookahead로 위치 struct pattern을 구분한다. bare 이름은 기존 binder다.
AST/HIR PatternStruct는 원 head token Span과 source-ordered pattern children을 저장한다. HIR은 head token boundary·delimiter/trivia·child 순서/shape를 검증한다.
다중 파일 HIR merge는 struct pattern head의 SymbolId를 remap한다. 이름은 선언 module의 type namespace에서 실제 Struct/P11 item import alias로 찾는다.
P21 type alias는 annotation에만 허용하고 pattern head는 N1102다. nominal mismatch/Enum head는 N2101, undefined type은 N2001, arity는 N2201이다.
P12 필드 순서/types/Source와 유효 Copy layout·전체 필드 가시성을 재사용한다. 메서드는 필드 수에 포함하지 않는다. 다른 module의 private field는 wildcard여도 N2004다.
기존 binder scope·중복·mutability·전역 dependency/순환을 유지한다. struct 내부 refutable component는 N3103이며 const 평가/파생 오류를 억제한다.
const initializer는 한 번 계산하고 Struct ConstValue field를 projection하여 binder별 상수 cache를 저장한다. 생성/산술·10,000-node 예산은 기존 규칙이다.
Matrix는 nominal struct 단일 constructor와 field types로 specialize한다. nested product/sum·unguarded Bool 합집합·N3101/N3102를 bounded usefulness로 검사한다.
concrete struct pattern의 witness는 Type(patterns), Empty는 Empty()다. 기존 opaque-only _ shortcut과 guard coverage 정책을 보존한다.
MIR은 scrutinee/initializer 한 번 snapshot 후 기존 FieldId로 projection한다. enum payload tag proof·pattern 뒤 guard/try 순서를 보존한다.
projection Source/provenance 및 private full-body proof를 연결하고 Checked nominal kind/type/child order·cached 값과 MIR field index/Source/branch 변조를 원 HIR 재계산으로 거부한다.
Grammar는 pattern/destructuring_pattern을 확장하고 struct_pattern을 추가해 64 productions다. 나머지 61개를 보존한다.
LLVM/Runtime production·ABI/layout·Lexer/END·Cargo dependencies·diagnostic registry는 변경하지 않았다.

## Tests Added / Validation

새 테스트 17개: 기본 14·LLVM opt-in 1·Native opt-in 2.
Bool 두 필드의 네 값에 대한 16개 부분집합을 독립 finite-domain oracle와 비교해 completeness를 확인한다. Empty/중첩 struct·Tuple/Enum/Option·guard union을 검사한다.
private/nominal/alias head/arity/type·value/type namespace·initializer scope·immutable match/let·mutable var·local/global const projection·forward/실패/순환·return/try를 검사한다.
패턴 root/head/children을 포함한 정확한 10,000/10,001-node 경계, 모든 UTF-8 prefix의 결정적 Parser 복구, AST head/shape/순서 변조를 검사한다.
Checked nominal kind/type/child order/유효하지만 다른 상수 값과 MIR 같은 타입 field index/Source/guard branch 변조를 거부한다.
const는 Runtime aggregate 생성 없이 materialize하고 실제 make 호출은 match/let마다 한 번만 실행하는 MIR을 확인했다.

| 검사 | 실제 결과 |
|---|---|
| fmt / Runtime rustfmt / all-features check / all-targets all-features Clippy -D warnings | PASS |
| 전체 기본 tests | 509 PASS / 0 FAIL / 102 ignored |
| LLVM 전체 opt-in COFF/ELF O0/O2 | 29 PASS / 0 FAIL |
| Windows Native 전체 opt-in debug/release | 73 PASS / 0 FAIL |

총 611 PASS / 0 FAIL. Native는 P33 두 개를 먼저 실행하고 기존 71개를 --skip p33_로 실행하여 중복 없는 전체 73개를 검증했다.
[고정 fixture](struct-pattern-proposal-fixtures/README.md)는 두 파일·정상 3·부정 20 code별 byte Span·Runtime 1을 검증했다.
Native 두 profile에서 scrutinee 1회·nested field/Unicode·let/var/const·Empty·guard 순서/생략·try/loop·checked Abort를 관측했다.
LLVM은 struct projection/equality의 결정적 IR과 양 target의 COFF/ELF O0/O2 object를 확인했다.
[독립 예제](../../examples/struct_patterns.nova)는 point=3/true, mutable=5/true, match=7 세 줄/LF·빈 stderr·exit 0이다.

## Specification / Remaining

관련 NOVA-026/029/035/038/044/046/047/048/073/074/076/078/081/083/087/091/094/095/136 보완 문서에 제한된 P33 bridge를 추가했다.
P01~P32 ledger·D01~D30 본문·Canonical·원본 148개·기존 fixtures/grammar bytes를 보존하고 문서 validator/generation idempotency를 별도 검증했다.
Rust 1.99.0·LLVM 21.1.8·Windows x64 MSVC에서 검증했다. MSRV 1.80·Linux Native host 실행은 이번에 검증하지 않았다.
CARGO_INCREMENTAL=0은 검증 프로세스에만 설정했다. Windows hard-link/junction 기본 검사는 sandbox 밖에서 실행했다.
이름 field/rest/or/range 패턴·Array·String/Move field·parameter/for/assignment pattern·const 함수·일반 Move/borrow/Drop은 후속 계약이다.
[TESTING.md](../../TESTING.md)에 직접 실행 명령을 제공한다.
