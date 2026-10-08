# Stage B 비제네릭 Type Alias 최소 계약 — P21

작성일: 2026-10-08. 상태: **Draft / 사용자 승인 대기 / 미구현**.
P01~P20·D01~D05·Canonical·원본 148개를 보존한다.
[전용 EBNF](GRAMMAR_STAGE_B_ALIAS.ebnf)·[수용 fixture](alias-proposal-fixtures/README.md)는 검토 자료다.

## Specification Change Proposal

- 관련 문서: [Canonical](CANONICAL.md), [MVP Freeze](../00_Governance/NOVA-002_Nova_0.1_MVP_기능_동결표.md),
  [원본 NOVA-030](../03_Types_Declarations/NOVA-030_Type_Alias_타입_정규화_사양서.md),
  [NOVA-030 보완 초안](specs/NOVA-030.md), NOVA-014/020/023/024/025/029/033/073~078/081/083/136.
  의존 계약은 [P11 Module](MODULE_STAGE_B_PROPOSAL.md), [P12 struct](STRUCT_STAGE_B_PROPOSAL.md),
  [P13 Tuple](TUPLE_STAGE_B_PROPOSAL.md), [P15 Option/Result](OPTION_RESULT_STAGE_B_PROPOSAL.md),
  [P18 default](DEFAULT_ARGUMENTS_STAGE_B_PROPOSAL.md), [P20 exists](EXISTS_STAGE_B_PROPOSAL.md)다.
- 현재 사양: D01의 type은 이미 hard keyword다. Primitive alias·T?·void는 정규화한다.
  NOVA-030 원본은 사용자 alias 선언 문법·순환·import 상세를 정의하지 않는다.
  NOVA-030 보완 초안과 전체 Draft EBNF는 type Name = Type와 transparent alias를 제안했다.
- 발견된 문제: 비제네릭 alias의 범위, type/value namespace, 사용 위치, forward/cycle,
  visibility, 자원 한도와 public AST/Checked metadata 검증이 미동결이다.
- 제안 변경: 아래 top-level 비제네릭 transparent type alias와 type-position 사용만 동결한다.
- 변경 이유: 기존 타입의 긴 표기를 재사용하고 Module·Tuple·Option·Result·const/default를 연결한다.
  owning Array의 성장·Drop·Move/borrow와 method receiver는 각각 후속 계약으로 남긴다.
- 영향 범위: alias RHS의 END type-context 추적, AST/Parser/HIR, Resolver의 type registry/import,
  TypeChecker 정규화·진단, MIR 입력 검증·tests·문서. Scanner/raw keyword·Runtime·LLVM ABI/API는 변경하지 않는다.
- Backward Compatibility: 기존 유효 소스와 D01~D05/P01~P20은 유지한다.
  type은 이미 예약돼 있으므로 새 예약어가 없다. 기존 unsupported alias 선언만 수용한다.
- 대안: generic alias·newtype·alias constructor/variant head까지 함께 구현할 수 있다.
  이번에는 type 위치만 지원해 범위를 한정한다. 이 제외 범위는 최종 언어의 영구 금지가 아니다.

## 문법·Source·END

1. top-level [visibility] type IDENT = type end를 허용한다. 기본 visibility는 internal이다.
   함수/block 내부 alias·generic parameter·qualified target type·alias의 type argument는 제외한다.
2. P20 program production에 alias_decl만 추가하고 새 alias_decl production 하나를 추가한다.
   총 **55개 production**, 기존 **53개 보존**이다. expression/postfix/type production은 그대로다.
3. type/IDENT/=와 target type child의 원 byte Span, visibility 및 whole declaration Span을 보존한다.
   AST/HIR alias는 target type child 하나다. 외부 AST의 child shape·spelling·token 경계·order·
   parent/UTF-8 포함·trivia gap·원 source 연결을 검증한다. 잘못된 입력은 Panic 없이 복구한다.
4. 기존 D05 END 의미를 유지하되 normalizer에 새 alias RHS type-context를 연결한다.
   alias = 이후의 <...>는 TypeArguments delimiter이며 내부 줄바꿈을 억제하고 closing >는 type 종료로 처리한다.
   = 뒤 줄바꿈은 operator continuation, target 종료 뒤 줄바꿈/semicolon은 END다.
   type 선언은 target 종료 뒤 다음 {를 declaration-body continuation으로 취급하지 않는다.
   END/semicolon/EOF 및 오류 복구에서 alias context가 다음 함수·comparison으로 유출되지 않게 한다.
   Option<int8>=none 같은 기존 P15 annotation의 >= 분할과 일반 비교 token은 그대로다. generic alias와 local type은 N1102,
   =/target 누락은 기존 N1101이며 다음 함수 선언으로 복구한다.

## Namespace·import·사용 위치

1. 모든 reachable module의 alias를 먼저 type namespace에 수집한다. forward alias 참조를 허용한다.
   alias 이름은 같은 namespace의 struct/Enum/alias/import 및 Primitive 예약 이름과 중복이면 N2002다.
   Option/Result 이름은 P15처럼 예약하지 않고 module type binding이 builtin fallback보다 우선한다.
   alias는 자체 value binding을 만들지 않는다. 같은 spelling의 함수/const/지역 값은 별도로 허용한다.
2. P11/P12 use module::Alias as Name으로 직접 alias item을 import한다. 원 Alias DefId와 정의 파일을 보존한다.
   type/value가 함께 존재하면 기존 원자 import 규칙을 따른다. 실패하면 둘 다 설치하지 않는다.
   private item import는 N2004다. module alias·qualified type·재export·Package는 추가하지 않는다.
   단, 직접 선언한 type B = ImportedA는 새로운 alias 선언이므로 B를 import할 수 있다.
3. RHS는 반드시 선언 module의 type scope에서 해석한다. caller의 같은 이름이나 value shadow를 사용하지 않는다.
   Primitive/String/Unit, structural Tuple, intrinsic concrete Option/Result, nominal Copy struct/Enum,
   다른 alias가 target이 될 수 있다. bare Option/Result는 기존 N2101 concrete-type 오류다.
4. alias 사용 위치는 parameter/return/local/global annotation, struct field/Enum payload,
   Tuple/type arguments/nullable와 numeric as target이다. 기대 타입·literal·승격·const/default·try·exists는
   정규화된 타입을 사용한다. String alias 자체는 가능하지만 Copy 밖 field/payload는 기존 N1102다.
5. alias는 새 nominal type·layout·specialization·runtime value가 아니다. 같은 canonical TypeId와
   원 StructId/EnumId/family key를 재사용한다. alias가 있을 때와 원 타입을 직접 쓸 때 assignability는 같다.
6. alias 이름을 value로 읽는 경우 값 binding이 없으면 N2001이다. 같은 이름의 실제 함수가 있으면 호출한다.
   값 binding이 없는 alias constructor A(...)와 alias variant/pattern head A::V는 이 단계 N1102다.
   생성/패턴에는 원 struct/Enum 이름이나 기존 intrinsic Option/Result head를 사용한다.
   local value를 type 생성자로 재시도하지 않는 P12 정책을 유지한다.

## Visibility·정규화·순환·한도

1. 명시적 alias 이름/import의 가시성을 P11/P12대로 검사한다. 원 nominal identity와 field/variant 가시성은 유지한다.
   public/internal alias가 private nominal target을 포함하는 것을 별도로 금지하는 API leak 정책은 추가하지 않는다.
   이는 P12의 opaque factory signature와 동일한 경계다. 이 단계는 alias constructor/variant head를 지원하지 않아
   별칭으로 private 생성/필드/패턴 접근을 우회할 수 없다. NOVA-030 보완 초안의 public-alias leak 거부안은
   이 계약의 승인 범위에 넣지 않으며 전체 API export 정책은 후속이다.
2. annotation 사용 여부와 무관하게 모든 alias RHS를 검사한다. alias-to-alias edge는 Tuple/nullable/
   Option/Result 안의 참조까지 포함하되 nominal struct/Enum 자체는 정규화의 terminal identity다.
   반복형 dependency/SCC와 memoization으로 forward chain을 정규화한다.
3. 각 cyclic alias 선언에 N2103 하나, primary는 해당 target type 전체 Span, secondary는 SCC의 alias 이름을
   bundle 선언 순서로 제공한다. 직접/상호/중첩/다중 파일 순환도 같다. cyclic target은 ErrorType으로 캐시하고
   사용 지점의 파생 N2101/N2001은 억제한다. 알 수 없는 target은 원 type reference N2001이다.
4. type A = Node; struct Node { let next:A }는 alias graph 순환이 아니라 기존 by-value layout cycle N2101이다.
   모든 unused nominal cycle·Copy 제한·mixed layout 검증을 보존한다. alias로 cycle을 숨기지 않는다.
5. bundle alias 선언 상한은 **1,024개**다. 첫 초과 alias 이름에 N8901과 한도 note를 제공한다.
   alias chain 깊이 자체는 host recursion을 쓰지 않으며 1,024개 chain을 허용한다. 정규화 후 Parser/type/aggregate
   depth 128·size 1 MiB·all-variant occurrence 65,536·P15 specialization 한도를 그대로 적용한다.
   alias wrapper는 layout depth나 const expression node를 추가하지 않는다. 실제 target 구조의 확장은 한도를 검사한다.
   10,000-node const/default budget·static const cycle N3202는 독립이며 그대로다.

## Compiler·MIR·수용 기준

- Resolver/Checked public metadata를 재계산 검증하고 alias 원 DefId/owner/source target와 canonical TypeId를 연결한다.
  외부 AST/HIR 위조·type binding/import mapping·target scope·cycle state·TypeTable/registry/family/layout 위조를
  MIR/CodegenUnit 전에 거부한다. backend는 Alias Type variant나 이름 기반 판정을 받지 않는다.
- MIR에는 기존 canonical 타입/상수/명시적 numeric cast/nominal field ID가 내려간다.
  alias declaration은 runtime statement·function·local·effect를 만들지 않는다. source annotation Span은 원 철자로 남는다.
- [두 파일 fixture](alias-proposal-fixtures/README.md)는 forward chain·import alias·value/type 동명·String/Unit·
  Tuple·nullable·Result·Copy struct/Enum·checked cast·default·try·exists의 **제안 8줄**, 정상 1·부정 16사례를 제공한다.
- 승인 후 tests: exact diagnostics/cascade/source, import atomicity/private/opaque factory, same canonical TypeId,
  모든 정수/float alias·Option/Result shadow·중첩 sum cache·unused/다중 파일 cycle, layout cycle,
  1,024/1,025·작은 host stack·prefix truncation, typed forgery·MIR proof·COFF/ELF O0/O2·Windows Native.
  original annotation 철자·call/cast/Abort Span, source order·named/default·try·String arena 수명을 검증한다.

## 승인 경계

P21 비제네릭 transparent alias·type 위치·forward/import·cycle/자원·Source/MIR 검증 subset만 별도 승인 대상이다.
generic alias·newtype·alias constructor/variant head·API leak/export 정책·Array·method·일반 Move/Drop·
전체 D06/D10/D11/D12/D16/D30 승인은 아니다. 승인 전 compiler source를 변경하지 않는다.
문서/fixture 검증 성공은 alias compiler/Native 실행 성공을 뜻하지 않는다.


## 초안 준비 검증 — 2026-10-08

- 문서 build/validator PASS: 148개 원본 hash·링크·Draft ledger·55-production EBNF,
  P20 기존 53개 production 보존·두 파일/정상 1/부정 16 fixture의 UTF-8 Span·제안 8줄 metadata를 확인했다.
- 기존 P20 기본 workspace 336 PASS / 0 FAIL / 64 ignored, fmt·clippy -D warnings·all-features PASS.
  이 결과는 기존 compiler의 회귀 기준이며 P21 구현 검증이 아니다.
- 현재 compiler로 두 파일 entry check는 N1102 / exit 1이다. alias 미지원 진단을 확인했으며 승인 전 실행 성공을 주장하지 않는다.
- P01~P20/D01~D05 승인 ledger·계약/EBNF, Canonical·원본 148개를 보존했고 Compiler/Runtime/Cargo는 변경하지 않았다.
  실제 LLVM/Native 전체 회귀는 이번 문서 작업에서 다시 실행하지 않았다. 이전 P20 완료 기록과 구분한다.
