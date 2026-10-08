# 문서 변경 기록

## 2026-10-08 — P19 loop·정수 범위 for 구현

사용자 승인 P19의 loop·정수 until/through for·불변 binder·source-order bound snapshots·inclusive MAX 종료·
nested jump/try·보수적 return 분석과 Source/MIR private body proof를 구현했다.
기본 10·LLVM 1·Native 2 테스트와 직접 실행 예제를 추가했다. [구현·검증 기록](RANGE_LOOP_IMPLEMENTATION.md).
Unit bound fixture의 기대 Span을 함수 괄호에서 실제 bound 22..24로 정정했다. 새 의미 정책 변경은 없다.


## 2026-10-07 — P19 loop·정수 범위 for 착수안 (Draft)

loop/정수 전용 until·through for·불변 binder·bound snapshot·inclusive MAX 종료·mixed jump·return 분석·Source/CFG 검증의
최소 계약·54-production EBNF·두 파일 정상/정상 2/부정 18/Runtime 2 fixture와 기대 byte Span을 준비했다.
기존 P01~P18 승인 사양과 Compiler 의미는 보존한다. 사용자 승인 전 구현하지 않는다.


## 2026-10-07 — P18 상수 표현식 함수 기본 인수 구현

- 사용자 “P18 승인하고 함수 기본 인수 구현 진행”으로 P18을 Accepted 처리했다.
- Parser/HIR default Source·declaration-module resolution·typed 상수 평가/예산·생략 대응·caller materialization·cross-file MIR 인증 구현.
- 기본 10개·LLVM 1개·Native 2개 테스트와 독립 예제/직접 실행 명령 추가. [구현 기록](DEFAULT_ARGUMENTS_IMPLEMENTATION.md)을 따른다.
- 기존 Compiler 의미·원본 148개 보존. runtime/parameter 의존 default·const function·overload·named constructor·일반 Move/Drop은 후속이다.

## 2026-10-07 — P18 상수 표현식 함수 기본 인수 착수안 (Draft)

- 선언 module scope·기존 const subset·10,000-node 예산·생략 대응·caller materialization·Source/MIR 검증 제안.
- P17 parameter 한 production만 확장한 52-production EBNF, 두 파일/정상 2개/부정 20개·UTF-8 Span·cascade·20줄 기대값 작성.
- Draft ledger/영향 문서/validator와 구현 후 실행 명령을 연결했다. Compiler/Runtime/기존 승인/원본 148개는 보존한다.
- runtime default·parameter 참조·overload·named constructor·일반 Move/Drop과 전체 D09/D11/D16/D25/D30은 제외한다.

## 2026-10-07 — P17 함수 이름 인수 구현

- 사용자 “P17 승인하고 이름 인수 구현 진행”으로 P17을 Accepted 처리했다.
- AST/HIR source metadata·parameter mapping/type context·MIR source-order snapshot/parameter-order 전달·try 보존과 위조 gate를 구현했다.
- 기본 9개·LLVM 1개·Native 2개 테스트와 독립 예제/명령을 추가했다. [구현 기록](NAMED_ARGUMENTS_IMPLEMENTATION.md)을 따른다.
- private import Draft 기대값은 기존 P11 N2004 전체 import Span 0..20으로 정정했다. resolver/LLVM/Runtime 의미는 보존했다.
- 기존 P01~P16/원본 148개 보존. 기본 인수·overload·named constructor·일반 Move/Drop과 전체 D11/D16/D25/D30은 후속이다.

## 2026-10-07 — P17 함수 이름 인수 착수안 (Draft)

- [최소 계약](NAMED_ARGUMENTS_STAGE_B_PROPOSAL.md): 사용자 함수 parameter mapping·mixed 순서·source-order snapshot/parameter-order 전달·mapped 문맥·try effect·Source/MIR 검증.
- [52-production EBNF](GRAMMAR_STAGE_B_NAMED_ARGUMENTS.ebnf)는 P16 arguments를 확장하고 argument 한 production을 추가하며 기존 50개를 보존한다.
- [두 파일·부정 16사례](named-arguments-proposal-fixtures/README.md)에 UTF-8 primary Span·cascade 금지·제안 24줄 stdout·구현 후 명령을 기록했다.
- Draft ledger/색인/영향 문서/validator를 연결한다. accepted P01~P16와 Compiler/Runtime은 보존한다.
- 기본 인수·overload·named constructor·Array·일반 Move/Drop과 전체 D11/D16/D25/D30은 제외한다.

## 2026-10-07 — P16 Copy try·Result 오류 전파 구현

- 사용자 “P16 승인하고 Copy try 구현 진행” 답변으로 P16을 Accepted 처리했다.
- Prefix AST/HIR·operand 문맥 격리·intrinsic Result/exact E·const N3201·flow를 연결했다.
- MIR 단일 Copy snapshot·active Success/Error·destination Error 조기 반환·private Source/control certificate를 구현했다.
- LLVM tag switch·기존 Copy/private ABI를 연결했다. Lexer/Runtime 정책은 유지한다.
- 기본 10개·실제 LLVM 1개·Native 2개를 추가했다. [구현·검증 기록](TRY_IMPLEMENTATION.md)과 [직접 실행 명령](../../TESTING.md).
- P15 try 거부 기대값은 P16 constructor 문맥 N2103으로 대체하고 기존 중첩 한도 code 표기만 정정했다.

## 2026-10-07 — P16 Copy try·Result 오류 전파 착수안 (Draft)

- [최소 계약](TRY_STAGE_B_PROPOSAL.md): prefix 결합·operand 문맥 격리·정확한 E·단일 평가/Copy snapshot·Error 조기 반환·const N3201·Source/CFG 검증.
- [51-production EBNF](GRAMMAR_STAGE_B_TRY.ebnf)는 P15 prefix_expr만 확장하며 기존 50개 production을 보존한다.
- [두 파일·부정 18사례](try-proposal-fixtures/README.md)에 UTF-8 byte Span·cascade 금지·제안 19줄 stdout·구현 후 실행 명령을 기록했다.
- Draft ledger/색인/영향 문서/validator를 연결했다. 사용자 미승인/미구현으로 accepted P01~P15와 Compiler/Runtime은 유지한다.
- Option try·error conversion·String/Move payload·일반 Drop과 전체 D08/D09/D10/D12/D16/D23/D25는 제외한다.

## 2026-10-07 — P15 Copy Option·Result·nullable 구현

- 사용자 “승인 할테니 다음 개발 작업 진행해줘” 답변으로 P15를 Accepted 처리했다.
- Generic type syntax·nullable·none·END와 >= parser adapter, builtin fallback/shadow·문맥·Copy specialization을 연결했다.
- Enum 기반 const/MIR/Native 경로를 재사용하고 intrinsic family/key/origin certificate를 별도로 검증한다.
- 4,096 고유 specialization·mixed cycle/layout·const 예산·coverage/flow/visibility와 손상 IR 회귀를 추가했다.
- standalone 예제와 check/debug/release 명령·8줄 출력은 [TESTING.md](../../TESTING.md)를 따른다.
- [구현·검증 기록](OPTION_RESULT_IMPLEMENTATION.md). 원본 148개 보존; try·Array·String/Move·사용자 Generic은 후속.

## 2026-10-07 — P15 Copy Option·Result·nullable 착수안 (Draft)

- Builtin Copy family·T?·none·생성 문맥·type namespace shadow·match·const·private tagged ABI 제안.
- 원 raw token을 보존하는 type-close >= adapter/END 문맥·bounded specialization 구체화.
- P14 세 production 확장/세 추가: 51-production EBNF와 두 파일·부정 21사례 제안 code/UTF-8 Span 작성.
- Draft ledger·validator·승인 후 실행 명령 추가. Compiler/Runtime/accepted ledger·원본 148개는 보존.
- try·Array·Move/Drop·사용자 Generic은 후속. 문서 검증과 실제 Compiler/Native 증거를 구분.

## 2026-10-07 — P14 승인과 Copy Enum·statement match 구현

- 사용자 “P14 승인하고 Copy Enum·match 구현 진행”을 계약·48-production EBNF·accepted ledger에 기록.
- nominal Copy Enum·type alias import·Enum/Bool coverage·불변 binder·const·private tagged ABI 구현.
- 독립 MIR active-tag CFG proof와 stale/wrong/retargeted payload gate, resource/layout oracle 검증.
- 기본 276개·실제 LLVM 10개·Windows Native 36개, 부정 16사례 code/UTF-8 Span 검증.
- [구현 기록](ENUM_IMPLEMENTATION.md), [예제](../../examples/enums.nova), [사용자 실행 명령](../../TESTING.md) 제공.
- 원본 148개 보존. Array·Option/Result·일반 Move/borrow/Drop·guard/nested pattern은 후속.

## 2026-10-07 — P14 Copy Enum·statement match 착수안 (Draft)

- nominal Copy Enum·qualified variant·type import·Enum/Bool statement match와 immutable Copy binder 제안.
- N3101/N3102 Error·coverage/flow·활성 tag proof·const 생성·private union layout/ABI·자원 한도 구체화.
- P13 세 production 확장/여덟 추가: 48-production EBNF, 두 파일·부정 16사례 proposed code/UTF-8 Span 작성.
- Draft ledger·기계 validator·승인 후 사용자 실행 명령 추가. Compiler/Runtime/기존 accepted ledger와 원본 148개 보존.
- 검증 결과는 VALIDATION.md와 P14 초안 준비 검증 기록으로 구분하며 제안 source의 실행 성공을 주장하지 않는다.

## 2026-10-07 — P13 승인과 Copy Tuple 구현

- 사용자 “P13 승인하고 Copy Tuple 구현 진행”을 계약·40-production EBNF·accepted ledger에 기록.
- structural Copy Tuple·numeric selector 원 byte subspan·혼합 경로·기대 타입·const·mixed layout graph 구현.
- 독립 MIR shape kind/ID/path/signature gate와 private snapshot/out ABI, 반복형 malformed payload 해제 확장.
- 기본 265개·실제 LLVM 9개·Windows Native 34개와 정확한 부정 10사례 code/Span을 검증.
- [구현 기록](TUPLE_IMPLEMENTATION.md), [예제](../../examples/tuples.nova), [사용자 실행 명령](../../TESTING.md) 갱신.
- 원본 148개 보존, Array·Enum/match·일반 Move element·전체 D09/D10/D12/D16은 후속.

## 2026-10-07 — P13 Copy Tuple 착수안 (Draft)과 사용자 테스트 안내

- structural Copy Tuple·위치 element·numeric projection·혼합 가변 경로·const·private ABI와 자원 제한 제안.
- P12의 네 production 확장·세 production 추가로 40-production Draft EBNF, 두 파일/부정 10사례 기대값 작성.
- longest-match Lexer를 보존하는 selector 문맥 token subspan 정책과 정확한 진단 위치 제안.
- Draft ledger/validator와 [직접 실행할 명령](../../TESTING.md) 추가. 승인 전 Compiler/accepted ledger에는 미적용.
- 문서 validator·기존 기본 252개 tests·fmt/clippy/all-features PASS. 기존 P12 안내 예제의 check/debug/release 실제 실행 확인.
  P13 수용 테스트는 미구현이며 opt-in LLVM/Native 전체는 이번 문서 변경에서 재실행하지 않았다.

## 2026-10-07 — P12 승인과 Copy struct 구현

- 사용자 “P12 승인하고 Copy struct 구현 진행”의 2026-10-05 승인을 계약/37-production EBNF/ledger에 기록.
- nominal Copy struct·분리 type namespace/원자 import·위치 생성·field 읽기/가변 경로·const·checked layout 구현.
- 독립 MIR 원 ID/path/layout/signature 검증과 private Native snapshot/out ABI, malformed payload 반복형 해제 보강.
- 기본 252개·실제 LLVM 8개·Windows Native 32개 검증 및 최종 gate 보강 후 Core/LLVM/P12·Hello 재검증.
- [구현·검증 기록](STRUCT_IMPLEMENTATION.md), [두 파일 fixture](struct-proposal-fixtures/README.md), [예제](../../examples/structs.nova).
- 원본 148개 문서를 보존하며 String field·init/Drop·일반 Move/borrow·전체 D06/D10/D12/D16/D30은 후속.

## 2026-10-05 — P12 Copy struct 착수안 (Draft)

- 원본 value semantics·명시적 let/var·재귀 값 금지·namespace 분리를 보존하는 [P12 최소 계약](STRUCT_STAGE_B_PROPOSAL.md) 작성.
- Copy field·위치 인수 생성·type import·가시성·field 경로 대입·const·private layout/ABI와 자원 제한 제안.
- P11의 세 production만 변경하고 세 production을 추가하는 37-production Draft EBNF 작성.
- 두 파일 Copy/const/함수 예제와 부정 10사례의 예상 진단·UTF-8 Span, Draft ledger와 validator 작성.
- 승인 전 Compiler/accepted ledger는 보존. String field·init/Drop·일반 Move/borrow와 전체 D06/D10/D12/D16/D30은 후속.

## 2026-10-05 — P11 승인과 Module·다중 파일 구현

- 사용자 “P11 승인하고 Module·다중 파일 구현 진행”으로 계약/34-production EBNF/ledger를 Accepted로 전환.
- Driver의 reachable discovery·exact spelling·canonical root·hard link/junction physical identity·1,024-module 제한.
- 파일별 AST root/SourceOrigin, bundle HIR/DefId·import alias/visibility와 cross-file const·entry/MIR 연결.
- private/duplicate/undefined/const/root 오류의 원 FileId/Span과 frontend gate, Native private symbol/entry isolation 검증.
- [현재 구현·검증 기록](MODULE_IMPLEMENTATION.md), [두 파일 fixture](module-proposal-fixtures/README.md).
- 원본 148개 문서와 D01~D05/P01~P10 의미를 보존하며 Package/aggregate/qualified value/reexport는 제외.

## 2026-10-05 — P11 Module·다중 파일 착수안 (Draft)

- 원본 파일 경로 module·순환 참조 허용/순환 초기화 금지 기준을 보존하는 [P11 최소 계약](MODULE_STAGE_B_PROPOSAL.md) 작성.
- root-relative 직접 함수/const item import·alias·visibility·source root CLI·cross-file const/entry/source identity 제안.
- 기존 P10 production을 유지하고 top-level use/visibility만 추가하는 34-production Draft EBNF와 두 파일 fixture 작성.
- Draft ledger/미구현 상태·grammar 경계·fixture 데이터를 문서 validator에 추가. Compiler/accepted ledger에는 미적용.
- module alias/qualified value/reexport·Package/aggregate·전체 D06/D30은 후속.

## 2026-10-05 — P10 숫자 cast 구현

- 사용자 “P10 승인하고 숫자 cast 구현 진행”을 Accepted subset/전용 postfix EBNF에 기록.
- AST/HIR source origin·타입 문맥 격리·Core checked 변환·const N3201/예산·MIR CheckedCast 구현.
- LLVM 원 width RN·truncation 후 ordered guard·안전한 fptoi·Runtime reason 4/full cast Span 구현.
- 독립 rational oracle 9,000개, const/Native 957쌍, 숫자 100조합 O0/O2 COFF/ELF 및 실패/회귀 검증.
- [구현·검증 기록](CAST_IMPLEMENTATION.md), [수용 예제](../../examples/casts.nova).
- Bool/Char/unsafe cast·float remainder/math API·aggregate/module·전체 D07은 후속.

## 2026-10-05 — P10 명시적 숫자 cast 착수안 (Draft)

- P07/P09 다음 최소 범위로 [숫자 as 계약](CAST_STAGE_B_PROPOSAL.md)을 작성.
- postfix/prefix 우선순위·literal 문맥 격리·정수 범위·RN float 변환·truncation 후 정수 범위·finite narrowing 실패 제안.
- const N3201/node budget·MIR CheckedCast·Runtime Abort reason 4/SourceInfo와 독립 oracle/Native 수용 계획 정리.
- P09 postfix에 as type만 추가하는 [31-production 검토 EBNF](GRAMMAR_STAGE_B_CAST.ebnf), Draft ledger와 validator 검사 추가.
- 승인 전 Compiler/accepted ledger/기존 accepted EBNF 보존. Bool/Char/unsafe cast·float remainder·전체 D07은 후속.

## 2026-10-05 — P09 float 구현

- 사용자 “P09 승인하고 float 구현 진행”을 Accepted subset/전용 FLOAT primary EBNF로 기록.
- Float32/64의 직접 literal 반올림·whole-type lossless 승격·IEEE 산술/비교·canonical NaN·±0를 AST→Native에 연결.
- const와 MIR NumericConvert/독립 validator·재계산 gate·LLVM scalar/bit formatter ABI 구현.
- MXCSR 제어/복원, Runtime entry 환경 초기화, fallible fixed decimal 출력과 정확한 decimal-even 동률 보정.
- Python 정수/유리수 독립 oracle: literal 24개·연산 1,599개·formatter bits 2,474개, Native const/runtime 229쌍 검증.
- [구현·검증 기록](FLOAT_IMPLEMENTATION.md), [실행 예제](../../examples/floats.nova).
- source cast·float remainder/math API·aggregate/module·전체 D07은 후속.

## 2026-10-04 — P09 float 착수안 (Draft)

- binary32/64·REAL literal 문맥·직접 ties-to-even·전체 범위 기반 numeric 승격 제안.
- IEEE overflow/div0·canonical NaN/±0, const/MIR·LLVM/private formatter와 decimal 보간 검증 경계 명시.
- [P09 초안](FLOAT_STAGE_B_PROPOSAL.md), P08 primary에 FLOAT만 추가하는 [검토 EBNF](GRAMMAR_STAGE_B_FLOAT.ebnf) 작성.
- 승인 대기/미구현 Draft ledger와 validator 검사 추가. source cast·float remainder·전체 D07은 후속.
- 기존 Compiler·accepted ledger/EBNF·원본 사양은 보존.
- 문서 validator와 기존 기본 187 tests, Cargo fmt/clippy/all-features check 통과.
  Compiler 변경이 없어 기존 opt-in LLVM/Native tests는 이번 초안 작업에서 다시 실행하지 않았다.

## 2026-10-04 — P08 char 구현

- 사용자 “P08 승인하고 char 구현 진행” 답변을 Accepted subset/전용 CHAR primary EBNF로 기록.
- Character AST/HIR·Char 타입/const/MIR, 동일 타입 6종 비교·함수/대입·UTF-8 보간/private Runtime 연결.
- String brace와 character brace를 구분하고 invalid Runtime scalar는 SourceInfo와 Abort로 거부.
- [구현·검증 기록](CHAR_IMPLEMENTATION.md), [예제](../../examples/characters.nova), 17개 tests 추가.
- 기본 187 + 실제 LLVM 4/Native 22 = 총 213 tests, fmt/clippy/all-features·문서 검증 통과.
  첫 Native 실행의 기존 P07 Runtime 링크 실패와 104-case 재검증 통과는 구현 기록에 명시.
- 원본·기존 EBNF·D04 Lexer/END·Hello snapshots·전체 D07 Draft 경계 보존.

## 2026-10-04 — P08 char 착수안 (Draft)

- char의 scalar 범위·동일 타입 비교·const·UTF-8 보간·private i32/u32 ABI와 formatter 제안.
- D04 Lexer/escape/END 보존. character brace에 String brace doubling을 적용하지 않는 decode 경계 명시.
- [P08 초안](CHAR_STAGE_B_PROPOSAL.md), [검토용 EBNF](GRAMMAR_STAGE_B_CHAR.ebnf) 작성.
- Draft ledger/validator에 승인 대기·미구현·CHAR primary만의 변경 경계 검사 추가.
- 기존 Compiler·P01~P07 accepted ledger·accepted EBNF·원본 사양은 보존.

## 2026-10-04 — P07 정수 타입·승격 구현

- 사용자 “P07 승인하고 정수 타입·승격 구현 진행” 답변에 따라 P07 subset을 Accepted로 기록.
- 8종 정수와 alias, 기대/peer literal 문맥, 전체 범위 기반 widening/common type 구현.
- checked const/Runtime·MIR Widen·LLVM 폭/부호별 연산·정수 보간/private wide formatter 연결.
- P06 grammar와 기존 Int32 진단·panic reason·formatter·Hello snapshots 보존.
- [구현·검증 기록](INTEGER_IMPLEMENTATION.md), [예제](../../examples/integers.nova). 전체 D07·후속 타입은 Draft.
- 15 tests 추가, 기본 174 + 실제 LLVM/Native 22 = 총 196 tests와 fmt/clippy/all-features·문서 검증 통과.

## 2026-10-04 — P07 정수 타입·승격 착수안 (Draft)

- int8~int64/uint8~uint64·Canonical alias, expected/peer literal 문맥과 전체 범위 lossless widening 제안.
- 공통 정수 타입·폭별 checked Runtime/const·명시적 MIR conversion·정수 보간/private ABI 수용 계획 작성.
- 기존 P06 source grammar 재사용, float/char/cast/전체 D07은 후속. Compiler·accepted ledger는 유지.
- Draft ledger와 validator에 P07 승인 경계 검사 추가.

## 2026-10-04 — P06 단일 파일 전역 const 구현

- 사용자 “P06 승인하고 전역 const 구현 진행” 답변을 Accepted subset으로 기록.
- P02/P06 print 문장 충돌을 보고했고 “기존 함수 print 허용, 전역 const print만 거부 (권장)” 정정 승인 기록.
- global 선언 수집·forward 이름과 dependency-first 타입/상수 평가, iterative SCC/첫 back-edge N3202 chain 구현.
- global 값은 MIR Constant operand로 사용하고 startup/global mutable storage 없이 기존 LLVM Adapter를 재사용.
- global_constants.nova와 22 tests 추가. 512가지 3-node graph·10,000-link chain·budget/cascade/변조 gate 검증.
- 실제 Windows O0/O2 예제·Int32 경계·Unicode/NUL·Unit·local shadow와 LLVM COFF/ELF 검증 통과.
- 기본 163 + 실제 LLVM/Native 18, 총 181 tests와 fmt/clippy/all-features·문서 검증 통과.

## 2026-10-04 — P06 전역 const 착수안 (Draft)

- 단일 파일 전역 const·forward visibility·dependency-first 타입/상수 평가를 제안.
- 정적 skipped RHS edge를 포함한 SCC별 N3202/chain 진단, 전역 불변 대입과 P05 node budget 재사용.
- 31-production 전용 EBNF와 Parser→Native 수용 계획을 작성. P06은 승인 대기이며 Compiler는 유지.
- Draft ledger와 문서 validator에 초안/승인 경계 검사를 추가. D06/D09 전체·module/다중 파일은 후속.

## 2026-10-04 — P05 함수 내부 const 구현

- 사용자 답변 “P05 승인하고 const 구현 진행”을 Accepted subset으로 기록.
- local const 분류와 target-independent ConstValue, iterative permission/budget/checked evaluator 추가.
- 10,000-node limit과 N3201/N3202, short-circuit와 skipped RHS 검사, 실패 ErrorType 전파.
- 성공 값만 MIR Constant로 materialize. 일반 let/var Runtime 산술은 유지하고 변조된 값/count/table 차단.
- constants.nova와 20 tests 추가. 기본 143 + 실제 LLVM/Native 16, 총 159 tests.
- Windows x64 O0/O2 UTF-8/NUL·Unit·경계/음수 산술·shadow/loop 검증. 전역 상수/전체 D09는 Draft.

## 2026-10-04 — P05 const 착수안 (Draft)

- 함수 내부 초기값 필수 const, 현재 Int32/Bool/String/Unit 값과 제한된 표현식 계약 작성.
- 기존 checked 산술의 compile-time N3201, short-circuit 허용성/평가 구분, 10,000-node/N3202 예산 제안.
- 30-production 전용 EBNF, 의미/MIR/CLI/Native 수용 기준 추가. Compiler와 accepted ledger는 유지.

## 2026-10-04 — P04 가변 지역 변수·반복문 구현

- P04 문서/승인 질문 뒤 사용자 “다음 개발 진행ㅎ재ㅝ” 요청을 해당 최소 범위 진행 승인으로 기록.
- var 가변성, direct-name 대입, while/break/continue를 AST→HIR→이름·타입→MIR→Native로 연결.
- N3004 불변 대상 대입 진단 등록, loop nesting 및 Return/Jump/Fallthrough 분석, CFG cycle 검증.
- loops.nova 예제, 24 tests 추가: 기본 125와 실제 LLVM/Native 14, 총 139 tests.
- Windows x64 O0/O2 누적/nested jump/조건·initializer 횟수/zero iteration/return/문자열·Abort 검증.
- 전체 Stage B/ownership/Drop와 Linux Native는 후속. P03 Runtime ABI·String arena 유지.

## 2026-10-04 — P04 Stage B 착수안 (Draft)

- var/direct-name 대입/while/break/continue의 문법·의미·진단·MIR·Native 수용 기준 작성.
- P01을 확장하는 30-production 전용 EBNF 초안 추가. N3004 불변 대상 대입 진단 제안.
- 승인 ledger와 compiler는 유지. P04 사용자 승인 후 구현하며 전체 Stage B 승인이 아니다.

## 2026-10-04 — P03 Stage A Native

- 사용자 답변 “P03 승인하고 Stage A Native 구현 진행”을 Accepted subset으로 기록.
- nova-codegen/nova-codegen-llvm/nova-cli 추가. Verified CodegenUnit, LLVM IR verify/Object, private Runtime ABI.
- Windows x64 MSVC Runtime compile/link, main entry, check/build/run debug/release, 안전한 신규 출력 경로.
- Checked Int32/Abort, UTF-8+LF print, decimal/Bool 보간과 실행 단위 String arena 구현.
- LLVM 21.1.8 공식 배포 hash 고정. Hello와 Unicode/NUL/short-circuit/산술 실패 O0/O2 Native 검증.
- 기본 105 tests와 opt-in 실제 LLVM/Native 10 tests, 재현 가능한 Hello executable/기존 출력·I/O 실패 검증.
- Linux x64는 LLVM IR/ELF Object만 검증. Linux Native link/run, 전체 ABI/FFI/ownership/Package는 후속.

## 2026-10-04 — Stage A MIR Lowering·검증

- nova-mir 추가: non-SSA Place 기반 BasicBlock CFG와 typed operand/local/signature/source origin.
- Call Terminator, source-order lowering, short-circuit 분기, if/return/Unit fallthrough.
- 독립 MIR validator: ID/type/signature/source/terminator와 must-initialized CFG dataflow.
- upstream 오류와 변조된 의미 table 성공 차단. MIR 18 tests, 총 91 Rust tests.
- 원본 NOVA-081~083/091 및 P02 승인 평가 순서를 적용. 새 언어 의미 변경/추가 승인 없음.
- Arithmetic/보간은 abstract 연산. Runtime/print/entry/ABI, LLVM/CLI/Native 실행은 후속.

## 2026-10-04 — P02 Stage A HIR/이름·타입 검사

- 사용자 답변 “P02 승인하고 이름·타입 검사까지 진행”과 최소 의미 계약을 Accepted로 기록.
- nova-hir/nova-resolve/nova-types/nova-typecheck 추가. Primitive/Unit 정규화와 SourceOrigin 보존.
- 단일 파일 forward/recursive call, lexical Scope/중복/shadow, Int32 range, call/return/condition 검사.
- print(String)과 보간 타입 검사, ErrorType 연쇄 진단 억제, recovery HIR 성공 차단.
- HIR·resolution·typed table snapshot, 정확한 code/Span frontend pass/fail harness 추가.
- MIR/Runtime/CLI/LLVM Native 실행과 미래 Stage 전체 정책은 후속 작업.
## 2026-10-04 — P01 Stage A AST/Parser

- 사용자 답변 “P01 승인하고 Stage A Parser 구현 진행” 기록. 전용 EBNF와 구문·복구 subset Accepted.
- nova-ast/nova-parser 추가. Arena/ID/byte Span, source-order Visitor, 결정적 AST dump.
- 함수/typed parameter/return type, let/return/if/else, positional call, 기본 표현식·문자열 보간.
- N1101~N1103, Synthetic Token, delimiter 동기화와 next-function 복구, nesting limit 적용.
- AST 3개 및 Parser 16개 테스트 추가. 전체 문법 초안과 Stage A 문법을 함께 기계 검증.
- 이름/타입 검사, HIR, CLI, LLVM 실행은 후속 단계. D06~D30 전체 정책은 Draft 유지.

## 2026-10-04 — 승인된 Lexer 구현 및 검증

- 사용자 승인 D01~D05를 Accepted로 기록. D06~D30은 Draft 유지.
- nova-syntax/nova-lexer 추가: lossless token, Unicode 식별자, Literal/Escape,
  중첩 주석/보간, N1001~N1003 진단, Stage A END 정규화.
- Unicode 18.0.0 데이터와 공식 unicode-ident 1.0.26 배포본·라이선스·checksum 고정.
- 초기화 없는 generic type annotation의 종료와 comparison continuation을 구분.
- Rust 테스트 30개 및 fmt/Clippy/check 통과. 문서 기계 검증 PASS.
- 전체 Parser, 의미 검사, CLI, LLVM 실행과 나머지 상세 사양 승인은 후속 작업.

## 2026-10-03 — 개발 문서 보완 Draft

- 원본 148개 NOVA 주제 각각에 전용 계약/오류/검증 문서 작성.
- 실제 Parser EBNF와 lexical/END/numeric 상세안 작성.
- 30건 semantic/tooling 결정 제안과 대안/승인 상태 기록.
- Core/std API, FFI type/ownership, CLI 및 package/diagnostic schema 초안 작성.
- 60개 conformance 묶음, 진단 코드 제안 registry, review fixtures 작성.
- 색인/감사/추적 manifest/SHA-256, 생성과 기계 검증 도구, Stage gates 추가.
- 원본 docs와 compiler crates는 수정하지 않음. 기존 Rust/LLVM/ownership/비지원 Canonical 유지.

승인/언어 규칙 적용/컴파일러 tests 통과는 이 기록의 작성 완료에 포함되지 않는다.
