# Stage B Copy Enum·statement match 최소 계약 — P14

작성일: 2026-10-07. 상태: **Draft / 사용자 승인 대기 / 미구현**.
기존 D01~D05/P01~P13·Canonical·원본 148개 문서를 보존한다.
승인 범위는 아래 subset이며 전체 D06/D08/D09/D10/D12/D16/D25/D30 승인이 아니다.

## Specification Change Proposal

- 관련 문서: [Canonical](CANONICAL.md), [MVP Freeze](../00_Governance/NOVA-002_Nova_0.1_MVP_기능_동결표.md),
  [원본 Enum](../03_Types_Declarations/NOVA-027_Struct_Class_Enum_선언_사양서.md),
  [원본 완전성](../04_Functions_Control/NOVA-047_Match_완전성_도달_불가_Arm_분석서.md),
  [원본 lowering](../04_Functions_Control/NOVA-048_Match_Lowering_Decision_Tree_사양서.md),
  [Enum 초안](specs/NOVA-027.md), [완전성 초안](specs/NOVA-047.md), [결정](DECISIONS.md), [P13](TUPLE_STAGE_B_PROPOSAL.md).
- 현재 사양: Enum은 하나의 활성 variant와 payload를 가지며 활성 payload만 읽는다.
  match는 완전성과 도달 불가 arm을 검사하고 arm 순서를 보존한다. Option/Result는 일반 Enum 의미를 따른다.
  원본은 구체 선언·생성·pattern grammar, Copy binding·진단 severity·private ABI를 정의하지 않는다.
- 발견된 문제: product 값 이후 sum 값을 안전하게 생성·분기하려면 variant identity, coverage와 tag 검사를 먼저 동결해야 한다.
  현재 Lexer에는 enum/match/::/=>가 있으나 Parser 이후 계층은 이를 지원하지 않는다.
- 제안 변경: nominal Copy Enum·위치 payload·type import와 qualified variant 생성, Enum/Bool statement match,
  단순 Copy binder·wildcard, 완전성/도달성 오류, const 생성과 private tagged ABI를 아래처럼 한정한다.
- 변경 이유: Array allocation과 일반 Move/borrow/Drop을 앞당기지 않고 활성 payload 접근의 안전성을 검증한다.
- 영향 범위: AST/Parser/HIR/Resolver/Types/TypeChecker/const/MIR/validator/LLVM 및 compile/runtime tests.
  Lexer token·END·Runtime scalar formatting·CLI host 계약은 유지한다.
- Backward Compatibility: 기존 scalar/struct/Tuple 프로그램과 P11 import·가시성을 유지한다.
  P12/P13 Copy component 허용 목록에 이번 Enum을 추가하되 기존 타입 identity·경로 가변성은 보존한다.
- 대안: payload 없는 enum만 먼저 구현하거나 match를 뒤로 미룰 수 있다.
  본 제안은 payload를 match 외부에서 꺼내는 API 없이 생성과 검증된 분기를 함께 제공한다.

## 구문·이름·가시성

[전용 EBNF](GRAMMAR_STAGE_B_ENUM.ebnf)는 P13의 program/statement/primary_expr 세 production만 확장하고
enum_decl/variant_decl/payload_types/variant_path/match_stmt/match_arm/pattern/pattern_arguments **여덟** production을 추가한다. 총 48개다.
Lexer의 기존 token/최장 일치/END 정규화와 P13 numeric selector subspan 규칙은 바뀌지 않는다.

1. `enum Event { Empty; Data(int8, (int, bool)); }`처럼 선언한다. variant 사이 종료는 기존 END이며
   newline/semicolon을 허용한다. variant payload는 이름 없는 위치 타입 목록이며 trailing comma를 허용한다.
   `Empty()` 선언·빈 Enum·variant visibility modifier·custom discriminant·named payload·method/init/drop은 N1102다.
   variant 선언 구분자 comma는 이번 문법에 없으며 N1101로 복구한다. payload tuple 타입 `(int,)`는 한 component다.
2. nullary 생성은 `Event::Empty`, payload 생성은 `Event::Data(1, (2, true))`다.
   nullary의 `Event::Empty()`는 N2201이다. payload variant를 인수 없이 쓰거나 arity가 틀리면 N2201이다.
   생성자는 first-class function 값이 아니다. 인수 타입 문맥·순서·scalar coercion은 P12/P13과 같다.
3. qualified variant 경로는 정확히 `IDENT :: IDENT`다. 왼쪽은 lexical value가 아니라 현재 파일 type namespace의 Enum이다.
   enum type·struct type은 같은 type namespace에서 중복 N2002다. 기존 별도 value namespace 정책을 유지한다.
   local value가 같은 철자여도 이 경로의 type lookup을 shadow하지 않는다.
   module-qualified value/variant (`module::Event::Data`), bare variant import/호출은 이번 범위 밖이다.
4. Enum은 P11 type item으로 `use events::Event as E`를 import할 수 있다.
   type 선언의 기본 internal/private/public과 alias·원 nominal ID·cross-file 진단은 P11/P12를 따른다.
   variant의 접근 범위는 Enum과 같다. private payload type의 추론 값은 P12 factory 정책을 따르며 field visibility는 유지한다.
5. `match expression { pattern => { statements } ... }`는 statement다. arm body는 block이고
   arm 종료는 END·comma·바깥 closing brace 중 하나다. 쉼표 뒤 END도 허용한다.
   body의 return/break/continue는 기존 enclosing function/loop에 속한다. match가 새 loop boundary를 만들지 않는다.
   match expression·guard·nested/literal/tuple/struct/or pattern·range·binding mode change/take는 N1102다.
6. 허용 pattern은 `E::Empty`, `E::Data(x, _)`, Bool `true`/`false`, wildcard `_`다.
   payload variant pattern은 모든 component에 IDENT binder 또는 `_`를 써야 한다. trailing comma는 허용한다.
   nullary pattern의 괄호는 N2201이다. `_`는 discard/wildcard 문맥에서만 특별하고 기존 value identifier 정책은 바뀌지 않는다.
   Parser는 arm·variant·중첩 block 오류 뒤 다음 arm/variant/선언으로 복구하고 nesting 기본 128을 유지한다.

## Copy·pattern binding·coverage·흐름

1. Enum은 원 선언 ID에 따른 nominal 타입이다. 모든 variant payload가 Copy여야 하며
   허용 component는 8종 정수·Float32/64·Bool·Char·Unit·P12 struct·P13 Tuple·P14 Enum이다.
   String/Array/Class/Function/Move/borrow payload는 N1102다. undefined named type은 N2001이다.
   모든 variant edge를 검사하므로 사용되지 않는 재귀 값도 N2101이다. Enum 간 structural conversion은 제공하지 않는다.
2. binding/대입/인수/return/const는 독립 Copy다. var root의 struct/Tuple 가변 경로에 Enum 전체를 저장할 수 있다.
   Enum에는 `.field`/`.index` payload 접근과 직접 payload 대입을 제공하지 않는다. 해당 selector는 N2101이다.
   Enum 전체 산술·비교·보간·cast는 N2101이다. match에서 scalar component를 Copy한 뒤 기존 연산을 사용할 수 있다.
3. scrutinee는 Bool 또는 Enum이어야 한다. 다른 타입은 scrutinee 전체에 N2101이다.
   시작 시 한 번 평가하고 독립 snapshot을 보관한다. arm에서 원 var를 재대입해도 선택 variant/payload는 바뀌지 않는다.
   matching enum type은 scrutinee와 정확히 같아야 한다. 다른 Enum의 variant는 qualified path 전체에 N2101이다.
   variant 이름이 없으면 해당 variant IDENT에 N2001이다. visibility 오류는 N2004다.
4. payload binder는 arm에만 존재하는 초기화된 immutable local Copy 값이다.
   arm body의 root scope와 같은 scope이며 같은 scope 재선언/중복 binder는 뒤 IDENT에 N2002다.
   outer local shadow는 P02처럼 허용한다. discard는 DefId를 만들지 않는다. 원 binding·payload type의 source를 보존한다.
   binder/parameter 대입은 N3004이고, `var local = binder`로 별도 변경 가능한 Copy를 만들 수 있다.
   Enum pattern binders는 implicit conversion이나 annotation을 추가하지 않는다.
5. pattern은 payload 값을 제한하지 않으므로 variant pattern 하나는 그 variant의 모든 값을 덮는다.
   Bool은 true/false, Enum은 선언 순서의 모든 variant가 coverage domain이다. wildcard는 남은 모든 경우를 덮는다.
   누락은 **Error N3101**, primary는 match keyword, note는 누락 variant/Bool을 결정적인 선언 순서로 나열한다.
   완전히 덮인 arm(중복 variant/Bool, wildcard 뒤 arm, 이미 전체 coverage 뒤 wildcard)은 **Error N3102**다.
   primary는 뒤 pattern 전체, secondary는 덮은 앞 pattern(여러 개면 source order)이다. warning-only 정책은 적용하지 않는다.
6. 각 arm은 source order로 검사·실행하며 선택된 body만 실행한다. statement match의 정상 완료는 Unit이다.
   모든 유효 arm이 return하면 enclosing 함수의 그 뒤 경로는 fallthrough가 아니다.
   한 arm이라도 정상 완료하면 join이 필요하며 기존 missing-return N3003을 적용한다.
   while 안의 break/continue·nested match·unreachable source checking은 기존 P02/P04 정책을 보존한다.

## Const·layout·자원 제한

1. const에 순수 Enum 생성만 추가한다. 생성 node 1 + 실제 평가한 모든 payload expression이며
   nullary 생성은 1 node다. variant path/qualified name의 synthetic node는 따로 세지 않는다.
   payload 평가 순서는 왼쪽부터 한 번씩이다. 기존 cached reference 1·group 1·initializer 10,000-node 제한을 유지한다.
   N3201 checked 평가·skipped RHS 포함 static dependency/cross-file cycle N3202도 유지한다.
   match는 statement이므로 const initializer expression에는 허용하지 않으며 const function은 계속 범위 밖이다.
2. private x64 layout은 32-bit unsigned tag와 aligned union payload다. tag는 선언 순서의 0-based discriminant다.
   variant payload의 field 순서와 natural alignment는 P12/P13을 따른다. nullary payload size 0/align 1이다.
   `A = max(4, 모든 variant payload align)`, `O = align_up(4, 모든 payload align의 최댓값)`이며
   `S = align_up(O + 모든 payload size의 최댓값, A)`다. Unit/Empty도 논리 component/초기화에 포함한다.
   nullary-only Enum size/align은 4/4다. niche·public layout/FFI/serialization 약속은 제공하지 않는다.
   discriminant와 layout model은 Core 자료형으로 정의하며 LLVM 타입은 Adapter 안에 둔다.
3. bundle당 Enum 1,024개·Enum당 variant 1,024개·variant당 payload component 1,024개·match당 arm 1,025개다.
   type/layout의 기존 mixed aggregate 깊이 128·size 1 MiB·transitive occurrence 65,536 상한에 Enum을 포함한다.
   깊이는 Enum 1 + 모든 payload aggregate child 깊이의 최댓값이며 variant 자체는 합성 깊이를 추가하지 않는다.
   occurrence는 모든 variant의 각 payload edge에 `1 + child aggregate occurrence`를 합산한다.
   size는 활성 variant가 아니라 최대 payload size를 사용한다. struct/tuple 한도와 고유 Tuple shape 4,096은 유지한다.
   named recursion은 unused 여부와 무관하게 자원 한도보다 먼저 N2101이며 closing type와 cycle edge Span을 보존한다.
4. 각 상한 초과는 N8901에 한도 note와 source-order 첫 초과 construct를 표시한다.
   선언 개수는 초과 Enum 이름, variant 개수는 초과 variant 이름, payload 개수는 초과 type,
   arm 개수는 초과 pattern, aggregate layout은 원인 named type/tuple 또는 선언과 원인 edge를 표시한다.
   coverage는 이 subset의 유한 tag set 검사로 제한하며 일반 pattern matrix recursion은 도입하지 않는다.
   graph/layout/const payload/MIR 검증·손상 payload 해제는 반복형·checked 처리한다.

## MIR·Native 안전성 계약

- stable EnumId/VariantId·discriminant·payload component 타입과 source를 분리한다.
  생성·tag test·검증된 arm의 payload Copy·CFG join/return/jump를 MIR에 명시한다.
  scrutinee를 snapshot local에 한 번 저장하며 payload read는 일치하는 tag 검사가 성공한 경로에서만 허용한다.
- MIR validator는 원 schema/nominal kind/variant ID·tag range·payload arity/type·완전 초기화·layout,
  source provenance·signature·coverage certificate·CFG tag-test dominance를 독립 검사한다.
  scrutinee가 덮어써진 뒤 오래된 tag proof를 payload read에 사용하는 손상 MIR도 거부한다.
  오류 type·불완전 coverage·잘못된 inactive read가 CodegenUnit을 만들지 못한다.
- private ABI는 P12/P13 caller-owned snapshot 간접 인수/out pointer 반환이다.
  LLVM Adapter는 tag와 실제 active payload만 쓰고 읽는다. inactive storage를 typed value로 읽지 않는다.
  padding/inactive byte 값은 source에서 관측할 수 없으며 uninitialized/poison byte를 scalar 계산에 사용하지 않는다.
  aggregate Copy 구현은 logical active value를 보존하고 snapshot/source-order를 유지한다.
- COFF/ELF object O0/O2와 Windows Native debug/release로 mixed/ZST/alignment·all scalar width,
  inactive payload 미접근·scrutinee 한 번 실행·arm effect/Copy·checked Abort의 정확한 file/byte Span을 검사한다.
  Linux object 검증과 Linux Native host 실행 증거는 별도로 보고한다.

## 수용 기준·진단·승인 경계

[두 파일·부정 fixture](enum-proposal-fixtures/README.md)에 source와 proposed stdout/code/UTF-8 byte Span을 작성했다.
현재 기대값은 미구현 제안 데이터이며 Compiler 통과/Native 실행 증거가 아니다.

| 상황 | 진단·primary |
|---|---|
| 누락 구문 / 범위 밖 syntax·Copy payload | N1101/N1102, 해당 token/type |
| duplicate type/variant/binder | N2002, 뒤 이름 + 앞 정의 |
| undefined type/variant / visibility | N2001/N2004, 해당 이름 + 정의 |
| 다른 Enum pattern·scrutinee/인수 타입·cycle | N2101, path/expression/type와 근거 |
| 생성·pattern payload arity | N2201, 생성 expression 또는 pattern 전체 |
| 불변 binder/경로 대입 | N3004, 첫 불변 이름 + 정의 |
| 누락 coverage | Error N3101, match keyword + 누락 cases |
| 도달 불가 arm | Error N3102, 뒤 pattern 전체 + 덮은 앞 pattern |
| const checked/cycle/budget | 기존 N3201/N3202 |
| 자원 상한 | N8901, 첫 초과 construct + 한도 |

수용 테스트는 UTF-8 truncation·복구·구문 경계, nominal/alias/private factory·multi-file identity,
Copy snapshot·binder scope/불변성·source-order/단일 실행, coverage/flow/while jump, const budget/cycle,
unused mixed recursion·layout 독립 oracle·모든 상한의 직전/초과, 손상 schema/CFG/tag proof gate를 포함한다.
기존 workspace/LLVM/Native 회귀, fmt/Runtime rustfmt/clippy/all-features와 원본 hash·문서 validator도 검사한다.

미포함: Array·Option/Result/generic/try·String/Move payload·일반 borrow·Drop·match expression·guard·nested pattern·
destructuring assignment·variant 직접 import·module alias/qualified value·public ABI·niche optimization.
Bool match 외 scalar pattern과 전체 D08/D12/D25 정책은 동결하지 않는다.

승인 전 Compiler/Runtime/accepted ledger에는 이 의미를 적용하지 않는다.
현재 구현된 P13 직접 실행 명령과 이번 제안의 승인 후 실행 명령은 [TESTING.md](../../TESTING.md)에 구분한다.

## 초안 준비 검증 기록

2026-10-07 문서 validator PASS: 원본 148개 SHA-256 보존, 48개 production의 중복/참조/도달성,
Draft ledger와 16개 부정 source의 UTF-8 byte Span·기대값 데이터 유효성을 확인했다.
EBNF 무모호성 증명이나 P14 Compiler/Native 실행 검증은 아니다.
기존 P13 기본 workspace 265개 tests PASS (LLVM/Native 43개 ignored), fmt/Runtime rustfmt,
clippy(-D warnings)/all-features check와 git diff --check PASS를 확인했다.
`examples/tuples.nova` check/debug/release도 실제 실행했고 원 P13 stdout·exit 0을 확인했다.
변경은 문서/문서 도구뿐이며 Compiler/Runtime source는 변경하지 않았다.
기존 opt-in LLVM/Native 전체 43개는 이번 문서 작업에서 다시 실행하지 않았다.
환경은 Rust 1.99.0, LLVM 21.1.8, Windows x64다. 승인 후 P14 자체 회귀와 Native 검증이 필요하다.
