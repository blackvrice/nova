# Stage B Copy struct Read 메서드 최소 계약 — P22

작성일: 2026-10-08. 상태: **Accepted / 2026-10-08 사용자 승인 / 구현 완료**.
D01~D05/P01~P21·Canonical·원본 148개를 보존한다.
[58-production EBNF](GRAMMAR_STAGE_B_METHOD.ebnf)·[수용 fixture](method-proposal-fixtures/README.md)는 승인된 최소 계약이다.

## Specification Change Proposal

- 관련 문서: [Canonical](CANONICAL.md), [MVP Freeze](../00_Governance/NOVA-002_Nova_0.1_MVP_기능_동결표.md),
  [원본 Struct](../03_Types_Declarations/NOVA-027_Struct_Class_Enum_선언_사양서.md),
  [원본 Receiver](../04_Functions_Control/NOVA-035_함수_메서드_호출_규약_Receiver_사양서.md),
  [Receiver 보완 초안](specs/NOVA-035.md), [D11/D12](DECISIONS.md), NOVA-014/020/023/024/025/029/030/031/033/036/037/040/073~078/081/083/136.
  의존 계약은 [P12 struct](STRUCT_STAGE_B_PROPOSAL.md), [P17 named](NAMED_ARGUMENTS_STAGE_B_PROPOSAL.md),
  [P18 default](DEFAULT_ARGUMENTS_STAGE_B_PROPOSAL.md), [P21 alias](ALIAS_STAGE_B_PROPOSAL.md)다.
- 현재 사양: Struct는 값 의미, 기본 인수 모드는 Read, 인수는 source order, default는 caller에서 평가한다.
  P12는 method/self/init/drop를 N1102로 거부한다. D11은 func m(self, ...)를 제안했지만 receiver 상세는 미승인이다.
- 발견된 문제: member namespace·receiver binding·lookup/visibility·평가 순서·named/default 대응·const·Source/MIR 계약이 미동결이다.
  Array는 길이/용량/초기화·정확한 Drop이 필요하므로 일반 ownership 없이 먼저 실행 구현하는 범위를 선택하지 않는다.
- 제안 변경: nominal Copy struct의 explicit self Read instance method와 직접 expr.method(args) 호출만 동결한다.
- 변경 이유: 기존 Copy snapshot·함수 ABI·named/default·try를 연결해 Stage B Method를 검증한다.
  일반 Move/borrow/Drop·change/take receiver는 Stage C 계약으로 남긴다.
- 영향 범위: EBNF·AST/Parser·HIR/Source·Resolver/Method registry·TypeChecker/signature/call plan·MIR/validation·tests·문서.
  Scanner/keyword/raw token·D05 END·Runtime API·공용 ABI는 변경하지 않는다. LLVM은 canonical 함수 호출을 사용한다.
- Backward Compatibility: D01~D05/P01~P21의 유효 소스·field/constructor·module/import·call/default·entry 동작을 보존한다.
  self는 계속 IDENT다. 기존 top-level 함수/변수/field의 self 이름은 허용한다. 현재 unsupported인 struct method만 추가한다.
- 대안: Array 또는 change/take method·explicit init·overload·Enum method까지 함께 구현한다.
  이번에는 Read Copy struct instance method만 제안하며 나머지는 별도 계약으로 남긴다.

## 선언·Grammar·Source

1. struct body에 [visibility] func IDENT(self [, typed ordinary parameters]) [-> type] block를 추가한다.
   첫 receiver는 source spelling이 정확히 self인 contextual IDENT, 별도 type/default/mode가 없는 하나의 receiver다.
   self만 있는 괄호와 self 뒤 trailing comma를 허용한다. ordinary parameter·trailing comma·return은 P01/P18을 따른다.
   receiver 없는 method, typed/default receiver, 첫 위치가 아닌 receiver, change/take는 N1102다.
2. P21 struct_decl 한 production만 확장하고 struct_member/method_decl/receiver를 추가한다.
   총 **58개 production**, 기존 **54개 보존**이다. postfix/field/argument/type/alias/Enum production은 그대로다.
   expr.method(args)는 기존 projection 뒤 direct Call 구문을 사용한다. 새 token·END 규칙은 없다.
3. AST/HIR은 method func/name/whole span·self identifier·paren/comma와 ordinary parameter/default/body의 원 byte Span을 보존한다.
   receiver는 별도 child이며 원 struct owner를 연결한다. implicit receiver ABI slot에 synthetic source 철자를 만들지 않는다.
   external AST는 child shape·spelling·XID 양쪽 경계·source order·parent/UTF-8 포함·trivia/punctuation gap을 검증한다.
   bundle은 원 FileSource/owner와 SymbolId를 재배치한다. prefix truncation/잘못된 header/member/다음 item을 panic 없이 복구한다.
4. field와 method는 선언 source order로 AST/HIR에 남는다. struct constructor/layout은 field만 선언 순서로 센다.
   methods는 storage field·tuple component·constructor argument를 추가하지 않는다. 빈 field struct도 method를 가질 수 있다.

## Member namespace·scope·visibility

1. 원 StructDefId별 field/method의 단일 member 이름 공간을 사용한다. 같은 owner의 field/field·field/method·method/method 중복은
   N2002, 뒤 선언 이름 primary·앞 선언 이름 secondary다. method return/parameter만 다른 overload도 중복이다.
   서로 다른 struct의 같은 이름과 top-level 함수의 같은 이름은 허용한다. method는 top-level value/type binding을 만들지 않는다.
2. reachable bundle의 모든 method/signature를 body 검사 전에 수집한다. 선언 전 호출·재귀·상호 호출을 허용한다.
   method owner는 원 nominal StructDefId, method는 안정적 DefId와 선언 FileId/Span을 유지한다.
   Module/Struct import alias·P21 transparent alias를 통해 얻은 값도 원 owner의 method를 조회한다.
3. self binding의 타입은 원 owner struct이며 immutable Read parameter다. 본문 self는 일반 lexical name lookup을 따른다.
   ordinary parameter self의 중복은 N2002이며 별도 receiver를 제공한 것으로 해석하지 않는다.
   body local shadow는 기존 P02 scope 규칙을 따른다. 쓰기 제한은 철자 self가 아닌 원 receiver DefId의 불변성으로 검사한다.
   self 또는 self.field/tuple 경로 대입은 N3004, 기존 P12대로 root self Span이 primary다.
   var copy=self 이후 copy의 var field를 수정할 수 있으며 원 receiver/caller 값은 바뀌지 않는다.
4. method body의 global 이름·type·private 접근은 선언 module scope다. receiver/parameters/body local은 기존 함수 scope 규칙을 따른다.
   default는 P18대로 **선언 module top-level scope**이며 self/ordinary parameters/body/caller local을 implicit하게 참조하지 않는다.
   module global self라는 이름이 존재하면 기존 P18처럼 그 global을 조회한다. 없는 self 참조는 N2001이다.
5. method visibility 기본은 internal이다. private은 정의 module, internal/public은 기존 P11 compilation bundle 규칙이다.
   명시적 struct/type import와 method 접근을 각각 검사한다. private method N2004는 member 이름 primary·선언 이름 secondary다.
   private field는 외부 caller가 직접 읽을 수 없어도 정의 module의 method body가 읽을 수 있다.
   public opaque factory의 추론된 struct 값에는 접근 가능한 method를 호출할 수 있다. 새 API leak/Package export 정책은 추가하지 않는다.
6. 직접 method item import·associated/static function·type-qualified 호출·extension·Enum/Tuple/Primitive method·Generic·interface·
   first-class bound method는 제외한다. 기존 type/value lookup 실패 진단을 보존한다.
   p.m을 method value로 읽거나 (p.m)(...)처럼 grouping으로 bound callee를 추출하면 해당 projection 전체 N1102다.
   일반 field 읽기/grouped function callee는 P12/P17 그대로다. bare m(...)은 module/lexical 함수 조회이며 implicit self 호출은 없다.
7. method main은 entry가 아니다. P03/P11 entry는 root module의 top-level main()→Unit만 검사한다.

## 호출·평가 순서·named/default·try

1. direct Call callee인 expr.member에서 receiver가 canonical nominal struct면 member를 조회한다.
   method가 있으면 원 method DefId/signature로 정적 dispatch한다. field면 기존 field 읽기와 non-callable 검사를 유지한다.
   unknown member는 N2001 member 이름, non-struct receiver의 method 선택은 기존 projection 전체 N2101이다.
   receiver ErrorType/미정의 이름에서는 파생 member/mapping/type 진단을 억제하고 독립 인수 오류는 보존한다.
2. receiver는 **한 번 먼저 평가 → 기존 Copy snapshot**한다. 이어 제공 ordinary 인수를 source order로 한 번씩 평가·변환·snapshot한다.
   다음으로 생략 default를 ordinary parameter 선언 순서로 caller materialize한다. 마지막에 receiver slot과 완전한 parameter order로 호출한다.
   Read Copy snapshot은 기존 값 복사다. receiver에 쓰기·소유권 이전·지속되는 loan·새 Drop obligation을 만들지 않는다.
3. source named/positional 인수 index 0은 첫 ordinary parameter다. self는 source 제공 인수/label/default 목록에 없다.
   self: label은 unknown label N2201이며 receiver를 덮어쓰거나 두 번째로 전달하지 않는다.
   P17/P18의 unknown/duplicate/positional-after-named·count/missing·expected type·checked literal/cast 진단·Span을 그대로 적용한다.
   receiver slot offset 때문에 literal 기대 타입·default index·try error type가 다른 parameter로 밀리지 않아야 한다.
4. (p).m(), make().m(), tuple.0.m(), alias-typed value의 m(), 결과 struct의 chained method를 허용한다.
   receiver의 기대 문맥은 ordinary argument/return 기대 타입과 격리한다. enclosing caller의 try 의미는 P16 그대로다.
   receiver나 제공 인수에서 try Error/Abort/return이 발생하면 이후 인수/default/call/body effect를 실행하지 않는다.
5. method body는 기존 return/if/match/loop/try/print/String arena·checked 숫자 규칙을 따른다.
   return은 기존 scalar/String/Copy aggregate와 원 nominal identity를 사용한다. method 자체를 const function으로 만들지 않는다.

## Const·한도·MIR/Native

1. 사용 여부와 무관하게 모든 method signature/default/body를 검사한다. local/global const·parameter default에서 사용자 method call은
   전체 call Span N3201이다. skipped logical RHS도 permission 검사한다. 기존 type/name ErrorType 파생 억제·10,000-node·N3202는 보존한다.
   receiver에 const 값이 들어가도 호출 허용성을 바꾸지 않는다. field projection·위치 struct const 생성은 기존대로 허용한다.
2. method 상한은 **bundle 1,024개**이며 source/bundle 선언 순서의 첫 1,025번째 method 이름에 N8901과 note를 제공한다.
   fields 1,024/struct·struct count·mixed depth 128·size 1 MiB·expanded occurrence 65,536·tuple/sum specialization 한도는 그대로다.
   method는 layout depth/size/occurrence 또는 const node를 추가하지 않는다. body/ordinary parameter는 기존 함수 한도·복구를 따른다.
3. public Resolved/Checked의 method owner/DefId/visibility/receiver binding·canonical signature·ordinary mapping/default offset·
   call source/receiver evaluation plan을 재계산 검증한다. 임의 owner/field/alias/import substitution은 MIR 전에 거부한다.
4. MIR은 receiver snapshot과 ordinary snapshots/defaults를 명시적으로 만든 뒤 static callee ID를 사용한다.
   private ABI receiver slot은 기존 Copy struct parameter 표현, 그 뒤 기존 ordinary argument/return 표현이다.
   SourceInfo는 call·receiver·인수·default 및 method body의 원 file/span/origin을 유지한다.
   full-body proof 등 기존 수준의 typed provenance/CFG 검증으로 receiver 단일 평가·slot 순서·snapshot·try bypass·effect 위조를 차단한다.
   LLVM/Core에 이름 기반 method lookup·bound object·vtable·공용 FFI ABI·새 Runtime helper를 넣지 않는다.

## 진단·수용 기준

| 상황 | code와 primary |
|---|---|
| 누락/잘못된 receiver | N1102, 첫 비지원 token 또는 닫는 paren |
| duplicate member/parameter | N2002, 뒤 이름 / 이전 선언 secondary |
| receiver 쓰기 | N3004, root self / receiver 선언 secondary |
| private/unknown method | N2004/N2001, member 이름 |
| non-struct method 선택 | N2101, callee projection 전체 |
| bound method value/grouped bound callee | N1102, projection 전체 |
| mapping/type/default | 기존 P17/P18 code와 정확한 Span |
| const/default method call | N3201, call 전체 |
| method count 초과 | N8901, 첫 초과 method 이름 / 한도 note |

- 두 파일·정상 1·부정 20·Runtime 1 fixture의 기대 Span/출력을 제공한다. **제안 15줄**은 실행 증거가 아니다.
- 승인 후 tests: UTF-8 prefix/recovery·trivia/token boundary·위조 AST/HIR·alias/import/nominal owner·private/opaque·member duplicate,
  immutable receiver와 mutable local Copy·all scalar/aggregate/empty struct ABI·unused body/default·10,000/10,001·1,024/1,025,
  receiver-first/단일 평가·named/default offset·String lifetime·try Error·checked Abort file/span·MIR owner/slot/effect forgery,
  deterministic IR·COFF/ELF O0/O2·Windows Native debug/release·Cargo 4 gates를 검증한다.

## 승인 경계

P22 Copy struct의 explicit self Read instance method subset만 승인 대상이다.
change/take·일반 Move/borrow/Drop·explicit init·overload·bound method value·Enum method·Array·Class/Generic/interface·
Package/API export·전체 D06/D09/D10/D11/D12/D16/D25/D30 승인은 아니다.
붙여넣은 개발 지침의 “사용자의 승인을 받기 전에는 해당 사양 변경을 적용하지 마십시오”에 따라 승인 전 compiler source를 변경하지 않는다.

## 초안 준비 검증 (당시 기록) — 2026-10-08

- 문서 build/validator PASS: 148개 원본 hash·Draft ledger·58-production EBNF·P21 기존 54개 production 보존·
  두 파일/정상 1/부정 20/Runtime 1 fixture의 UTF-8 Span·제안 15줄 metadata를 확인했다.
- 기존 P21 기본 workspace **348 PASS / 0 FAIL / 67 ignored**, fmt·clippy -D warnings·all-features PASS.
  이 결과는 기존 compiler의 회귀 기준이며 P22 구현 검증이 아니다.
- 현재 compiler의 두 파일 entry check는 **N1102 / exit 1**이다. struct method 미지원 진단을 확인했다.
- P01~P21 승인 ledger·계약/EBNF·D01~D05·Canonical·원본 148개·Compiler/Runtime/Cargo를 보존했다.
  실제 LLVM/Native 전체 회귀는 문서 작업에서 다시 실행하지 않았다. 이전 P21 완료 증거와 구분한다.

## 승인·구현 반영 — 2026-10-08

사용자 “승인하고 다음개발 진행해줘”로 앞서 준비한 P22 계약을 승인했다.
[구현·검증 기록](METHOD_IMPLEMENTATION.md)·[직접 실행](../../TESTING.md)을 따른다.
P22 Read Copy struct subset만 적용하며 D01~D05/P01~P21·Canonical·원본은 보존한다.
