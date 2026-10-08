# Nova 0.1 — 개발 문서 보완팩

작성일: 2026-10-03 · 언어: Nova 0.1 · 상태: **D01~D05/P01~P23 Accepted / 나머지 Draft**

다음 작업 [P24 중첩 Copy 패턴·Tuple match](NESTED_PATTERN_STAGE_B_PROPOSAL.md)는 **Draft / 승인 대기 / 미구현**이다.
[60-production 제안 EBNF](GRAMMAR_STAGE_B_NESTED_PATTERN.ebnf)·[수용 계획](nested-pattern-proposal-fixtures/README.md)을 검토한다.

기존 Documentation Pack의 148개 주제에 대해 구현 계약, 오류 조건, 검증 사례를 작성했다.
추가로 실제 EBNF, lexical/END/숫자 모델, 표준 API, 파일/schema 계약, 진단 코드,
결정 기록, conformance 계획, 개발/배포 지침을 제공한다. 범위는 Nova 0.1과 그 개발에
필요한 절차다. 비지원 기능은 제외/후속 문서로 명시하며 구현 범위를 확장하지 않는다.

## 먼저 읽기

1. [Canonical 확정 기준](CANONICAL.md): 기존 결정을 유지하는 경계.
2. [원본 사양 감사](SPEC_AUDIT.md): 빈 정의, 충돌, 주제 혼재.
3. [결정 기록 30건](DECISIONS.md): 새 규칙의 제안/대안/영향/승인 조건. [승인 Lexer 기준](ACCEPTED_LEXER.md).
4. [전체 문서 148개 색인](INDEX.md): 주제와 Stage별 계약.
5. [문법 설명](GRAMMAR_NOTES.md)과 [EBNF](GRAMMAR.ebnf), [Lexical](LEXICAL.md), [END](END_RULES.md).
6. [숫자 규칙](NUMERIC_RULES.md), [표준 API](STDLIB_API.md), [FFI 타입](FFI_TYPES.md).
7. [진단 schema](DIAGNOSTIC_SCHEMA.md), [Manifest/Lock/Artifact](MANIFEST_SCHEMA.md), [CLI](CLI_CONTRACTS.md).
8. [수용 테스트](CONFORMANCE.md), [개발 로드맵](ROADMAP.md), [기여/운영](CONTRIBUTING.md).
9. [기계 검증 결과](VALIDATION.md), [변경 기록](CHANGELOG.md).

Stage A Parser의 승인 기준: [Stage A Parser·AST 착수안 P01](PARSER_STAGE_A_PROPOSAL.md)과
[Stage A EBNF](GRAMMAR_STAGE_A.ebnf). P01 subset은 2026-10-04 사용자 승인으로 Accepted다.
현재 API와 검증 경계는 [Parser 구현 계약](PARSER_IMPLEMENTATION.md)에 기록했다.
Stage A 의미 검사는 [사용자 승인 P02](SEMANTICS_STAGE_A_PROPOSAL.md)를 따른다.
현재 API·검증 범위는 [의미 검사 구현 계약](SEMANTICS_IMPLEMENTATION.md)에 기록했다.
Stage A MIR은 원본 NOVA-081~083/091과 P02 평가 순서에 따라 구현했다.
현재 API·validator·runtime 미동결 경계는 [MIR 구현 기록](MIR_IMPLEMENTATION.md)에 있다.
Stage A Native 최소 계약은 [사용자 승인 P03](NATIVE_STAGE_A_PROPOSAL.md), CLI/LLVM/Runtime의 현재
지원·제약과 실제 검증 증거는 [Native 구현 기록](NATIVE_IMPLEMENTATION.md)을 따른다.

Stage B 최소 범위는 [P04 가변 지역 변수·반복문 계약](CONTROL_STAGE_B_PROPOSAL.md)과
[전용 EBNF](GRAMMAR_STAGE_B_CONTROL.ebnf)를 따른다. 2026-10-04 사용자 진행 요청으로 승인했고
현재 frontend/MIR와 Windows Native 검증은 [P04 구현 기록](CONTROL_IMPLEMENTATION.md)에 있다.

함수 내부 const는 [사용자 승인 P05](CONST_STAGE_B_PROPOSAL.md)와
[전용 EBNF](GRAMMAR_STAGE_B_CONST.ebnf)를 따른다. API·예산·진단과 실제 실행 증거는
[P05 구현 기록](CONST_IMPLEMENTATION.md)에 있다. 전체 D09 상세는 Draft다.

[P06 단일 파일 전역 const·의존성 평가](GLOBAL_CONST_STAGE_B_PROPOSAL.md)와
[31-production 전용 EBNF](GRAMMAR_STAGE_B_GLOBAL_CONST.ebnf)는 2026-10-04 사용자 승인으로 Accepted다.
print 충돌 정정도 사용자 승인했고 현재 API·진단·검증 증거는 [P06 구현 기록](GLOBAL_CONST_IMPLEMENTATION.md)에 있다.

[P07 고정 폭 정수 타입·손실 없는 승격](INTEGER_STAGE_B_PROPOSAL.md)은 사용자 승인으로 Accepted다.
8종 정수·literal 문맥·checked 산술·const·MIR 변환·보간의 [구현·검증 기록](INTEGER_IMPLEMENTATION.md).
float/cast 최소 범위는 P09/P10, module은 P11, Copy struct는 P12를 따른다. 나머지 aggregate는 후속이다.

[P08 char·Unicode scalar·보간 계약](CHAR_STAGE_B_PROPOSAL.md)과
[31-production EBNF](GRAMMAR_STAGE_B_CHAR.ebnf)는 2026-10-04 사용자 승인으로 Accepted다.
문자 비교·const·UTF-8 보간·private scalar ABI의 [구현·검증 기록](CHAR_IMPLEMENTATION.md).
기존 P06 primary에 CHAR만 추가하며 D04 Lexer/escape/END는 보존한다.

[P09 float·IEEE 결과·숫자 승격·보간](FLOAT_STAGE_B_PROPOSAL.md)과
[31-production EBNF](GRAMMAR_STAGE_B_FLOAT.ebnf)는 2026-10-04 사용자 승인으로 Accepted다.
IEEE rounding·NaN/Infinity/±0·const·private formatter의 [구현·검증 기록](FLOAT_IMPLEMENTATION.md).
숫자 cast는 P10을 따르고 float remainder는 후속이다.

[P10 명시적 숫자 cast](CAST_STAGE_B_PROPOSAL.md)와
[31-production EBNF](GRAMMAR_STAGE_B_CAST.ebnf)는 2026-10-05 사용자 승인으로 Accepted다.
정수 범위 검사·직접 float 반올림·float→int truncation·finite narrowing 실패·const/Runtime의
[구현·검증 기록](CAST_IMPLEMENTATION.md)을 따른다.
Bool/Char/String 변환·wrapping/saturating cast API와 전체 D07은 포함하지 않는다.

[P11 Module·다중 파일 최소 계약](MODULE_STAGE_B_PROPOSAL.md)과
[34-production EBNF](GRAMMAR_STAGE_B_MODULE.ebnf)는 2026-10-05 사용자 승인으로 Accepted다.
[구현·검증 기록](MODULE_IMPLEMENTATION.md)과 [두 파일 수용 fixture](module-proposal-fixtures/README.md)를 따른다.
root-relative 함수/const item import·alias·visibility·순환 함수 참조/const 순환 금지와
[두 파일 Draft fixture](module-proposal-fixtures/README.md)를 검토한다.
module alias/qualified value/reexport·package/aggregate와 전체 D06/D30은 제외한다.

## 상태와 효력

**작성 완료와 사양 승인, 구현 완료, 테스트 통과는 서로 다른 상태다.** 원본 Canonical
결정은 유지한다. D01~D05는 사용자 승인으로 Accepted이며 Lexer 구현에 적용한다.
P01 Stage A Parser 구문·복구는 2026-10-04 사용자 승인으로 Accepted다.
P02 단일 파일 HIR·이름·타입 최소 계약도 같은 날짜 사용자 승인으로 Accepted다.
P03 Native와 P04 가변 변수·반복문 최소 계약도 해당 문서의 사용자 승인/진행 요청 범위에서 Accepted다.
P05 함수 내부 const 최소 계약은 2026-10-04 사용자 승인으로 Accepted다.
P06 단일 파일 전역 const 최소 계약과 print 이름 경계도 2026-10-04 사용자 승인으로 Accepted다.
P07 정수·승격과 P08 char 최소 계약도 같은 날짜 사용자 승인으로 Accepted다.
P09 float 최소 계약도 같은 날짜 사용자 “P09 승인하고 float 구현 진행”으로 Accepted다.
P10 숫자 cast도 2026-10-05 사용자 “P10 승인하고 숫자 cast 구현 진행”으로 Accepted다.
나머지 새 문법·언어 의미·공용 API·ABI·Package 형식은 Draft다. D번호 승인
기록 없이 이 보완팩을 확정 사양으로 구현하지 않는다. 본문에서 '제안'이 생략된 구현
설명도 문서 상태는 Draft다. 기존 코드와 달라지는 규칙 역시 코드에 자동 적용하지 않았다.

구현에 필요한 정의를 빈 칸으로 남기는 대신 하나의 일관된 기본안을 작성하고 대안을
DECISIONS에 기록했다. 원본에 있는 '확정' 상태를 새 내용에 그대로 복사하지 않았다.
단계별 구현 시작 때 해당 Stage의 결정만 먼저 승인·동결할 수 있다.

## 관리

- 원본: 부모 docs의 155개 파일. 이번 작업에서는 보존.
- 보완: 이 디렉터리의 148개 specs와 부속 문서.
- MANIFEST.json: 원본/보완 경로, Stage, 상태, SHA-256 추적.
- topics 작성 원천: tools/docs/topics*.mjs. build-pack.mjs로 specs/index/audit를 재생성.
- 기계 검사: node tools/docs/validate-pack.mjs.

자료구조/API/schema는 설계 계약 예시이며 실행 가능한 현재 compiler API 목록이 아니다.
Nova 예제와 conformance fixture 역시 향후 수용 사례다. 현 compiler는 Source/Diagnostic
기반과 Token/Lexer/END, P01 AST/Parser, P02 HIR/이름·타입 검사 및 frontend pass/fail harness를
구현했다. Stage A MIR lowering/validation과 P03 Windows x64 CLI/Native 실행도 구현·검증했다.
P04 var·대입·while·break/continue와 Windows x64 O0/O2 실행도 구현·검증했다.
P05 함수 내부 const 평가와 Windows x64 O0/O2 실행도 구현·검증했다.
P06 전역 const·forward dependency/cycle와 Windows x64 O0/O2 실행도 구현·검증했다.
P07 정수·승격과 P08 char·비교·보간 및 Windows x64 O0/O2 실행도 구현·검증했다.
P09 float·상수 평가·숫자 승격·보간과 Windows x64 O0/O2 실행도 구현·검증했다.
P10 숫자 cast·범위 검사·const 평가와 Windows x64 O0/O2 실행도 구현·검증했다.
Linux Native link/run과 일반 ABI/ownership/Package는 후속이다.
frontend/MIR pass는 Native 실행 성공이 아니다. P03 Windows x64 Stage A Native는 실제 검증했다.
일반 Arithmetic/출력/ABI 전체 정책은 Draft이며 P03 subset만 승인했다.

[P12 Copy struct 최소 계약](STRUCT_STAGE_B_PROPOSAL.md)·[전용 EBNF](GRAMMAR_STAGE_B_STRUCT.ebnf)·
[두 파일 수용 fixture](struct-proposal-fixtures/README.md)는 2026-10-05 사용자 승인으로 Accepted다.
nominal Copy field·위치 인수 생성·type import·field 읽기/대입·const·private ABI와 제한을 구현했다.
[구현·검증 기록](STRUCT_IMPLEMENTATION.md)을 따른다. String field·init/Drop·일반 Move/borrow는 후속이다.

[P13 Copy Tuple](TUPLE_STAGE_B_PROPOSAL.md)·[40-production EBNF](GRAMMAR_STAGE_B_TUPLE.ebnf)·
[두 파일 fixture](tuple-proposal-fixtures/README.md)는 2026-10-07 사용자 승인으로 Accepted다.
구조적 Copy 타입·numeric selector subspan·혼합 가변 경로·const·private ABI와 자원 한도를 구현했다.
[구현·검증 기록](TUPLE_IMPLEMENTATION.md)과 [사용자 실행 명령](../../TESTING.md)을 제공한다. Array·일반 Move element는 후속이다.

[P14 Copy Enum·statement match](ENUM_STAGE_B_PROPOSAL.md)는 2026-10-07 사용자 승인 후 구현했다.
[48-production EBNF](GRAMMAR_STAGE_B_ENUM.ebnf), [두 파일·부정 16사례 fixture](enum-proposal-fixtures/README.md),
[구현·검증 기록](ENUM_IMPLEMENTATION.md)과 [사용자 실행 명령](../../TESTING.md)을 제공한다.
Enum/Bool의 유한 coverage·Copy payload·const·private tagged ABI를 지원한다.
Array·Option/Result·guard/nested pattern·일반 Move/borrow/Drop과 전체 D06/D08/D09/D10/D12/D16/D25/D30은 후속이다.

[P15 Copy Option·Result·nullable](OPTION_RESULT_STAGE_B_PROPOSAL.md)는 2026-10-07 사용자 승인 후 구현했다.
[51-production EBNF](GRAMMAR_STAGE_B_OPTION_RESULT.ebnf), [두 파일·부정 21사례](option-result-proposal-fixtures/README.md),
[구현·검증 기록](OPTION_RESULT_IMPLEMENTATION.md)과 [사용자 실행 명령](../../TESTING.md)을 제공한다.
Builtin Copy specialization·T?·생성 문맥·none·match·const·private ABI와 제한을 지원한다.
try·exists·method API·Array·String/Move payload·사용자 Generic·일반 borrow/Drop은 후속이다.

[P16 Copy try·Result 오류 전파](TRY_STAGE_B_PROPOSAL.md)는 2026-10-07 사용자 승인 후 구현했다.
[51-production EBNF](GRAMMAR_STAGE_B_TRY.ebnf)·[두 파일·부정 18사례](try-proposal-fixtures/README.md)·
[구현 기록](TRY_IMPLEMENTATION.md)과 [사용자 실행 명령](../../TESTING.md)을 제공한다.
Prefix 결합·operand 문맥 격리·정확한 E·Copy snapshot·Error 조기 반환·const N3201·Source/CFG 검증을 지원한다.
Option try·error conversion·String/Move payload·일반 Drop와 전체 D08/D09/D10/D12/D16/D23/D25는 후속이다.

[P17 함수 이름 인수](NAMED_ARGUMENTS_STAGE_B_PROPOSAL.md)·[52-production EBNF](GRAMMAR_STAGE_B_NAMED_ARGUMENTS.ebnf)·
[두 파일·부정 16사례 수용 fixture](named-arguments-proposal-fixtures/README.md)는 **Accepted / 구현 완료**다.
[구현 기록](NAMED_ARGUMENTS_IMPLEMENTATION.md)을 따른다.
사용자 함수의 parameter 이름 대응·source-order snapshot/parameter-order 전달·mapped 타입/try 검증을 구현했다.
기본 인수·overload·named constructor·Array·일반 Move/Drop은 포함하지 않는다. 현재 실행 명령은 [TESTING.md](../../TESTING.md)를 따른다.

[P18 상수 표현식 함수 기본 인수](DEFAULT_ARGUMENTS_STAGE_B_PROPOSAL.md)·[52-production EBNF](GRAMMAR_STAGE_B_DEFAULT_ARGUMENTS.ebnf)·
[두 파일·부정 20사례 수용 fixture](default-arguments-proposal-fixtures/README.md)는 **Accepted / 구현 완료**다.
[구현 기록](DEFAULT_ARGUMENTS_IMPLEMENTATION.md)을 따른다.
선언 module의 상수 default·필수/default 혼합·생략 대응·caller materialization·Source/CFG 검증을 구현했다.
Copy struct 이름 생성자는 P23을 따른다. runtime default·parameter 참조·overload·Enum/sum 이름 생성자·일반 Move/Drop은 후속이다. 현재 실행 명령은 [TESTING.md](../../TESTING.md)를 따른다.

현재 Stage B 구현은 [P19 loop·정수 범위 for](RANGE_LOOP_STAGE_B_PROPOSAL.md)·
[54-production 문법](GRAMMAR_STAGE_B_RANGE_LOOP.ebnf)·[수용 fixture](range-loop-proposal-fixtures/README.md)다.
Accepted / 구현 완료이며 [구현 기록](RANGE_LOOP_IMPLEMENTATION.md)을 따른다.

[P20 Copy Option postfix exists](EXISTS_STAGE_B_PROPOSAL.md)·[54-production EBNF](GRAMMAR_STAGE_B_EXISTS.ebnf)·
[두 파일/정상 2/부정 20/Runtime 1사례](exists-proposal-fixtures/README.md)는 **Accepted / 구현 완료**다.
Bool·const/default·단일 평가·Source/MIR 검증을 구현했고 P01~P19를 보존했다.
[구현 기록](EXISTS_IMPLEMENTATION.md)·[독립 예제/실행 명령](../../TESTING.md)을 제공한다.
Result exists·flow narrowing·Move/Drop·Array는 후속이다.


[P21 비제네릭 Type Alias](ALIAS_STAGE_B_PROPOSAL.md)·[55-production EBNF](GRAMMAR_STAGE_B_ALIAS.ebnf)·
[두 파일·부정 16사례](alias-proposal-fixtures/README.md)는 **Accepted / 구현 완료**다.
type 위치·forward/import·선언 scope·순환/한도·Source/MIR 검증을 구현했다. P01~P20과 원본을 보존한다.
[구현 기록](ALIAS_IMPLEMENTATION.md)·[독립 예제/실행 명령](../../TESTING.md)을 제공한다.
generic alias·newtype·alias constructor/variant head·API leak/export 정책·Array·일반 Move/Drop은 후속이다.


사용자 승인한 [P22 Copy struct Read 메서드](METHOD_STAGE_B_PROPOSAL.md)·[58-production EBNF](GRAMMAR_STAGE_B_METHOD.ebnf)·
[두 파일·정상 1/부정 20/Runtime 1사례](method-proposal-fixtures/README.md)는 **Accepted / 구현 완료**다.
contextual self·member scope/visibility·receiver-first snapshot·named/default·Source/MIR 검증을 구현했다. P01~P21을 보존한다.
[구현 기록](METHOD_IMPLEMENTATION.md)·[독립 예제/실행 명령](../../TESTING.md)을 제공한다.
change/take·일반 Move/borrow/Drop·init·overload·bound method·Enum method·Array는 후속이다.


[P23 Copy struct 생성자 이름 인수](STRUCT_NAMED_ARGUMENTS_STAGE_B_PROPOSAL.md)·[수용 fixture](struct-named-arguments-proposal-fixtures/README.md)는 Accepted / 구현·검증 완료다.
새 문법 없이 승인 P22 EBNF를 재사용하며 [구현 기록](STRUCT_NAMED_ARGUMENTS_IMPLEMENTATION.md)·[독립 예제/명령](../../TESTING.md)을 제공한다.
