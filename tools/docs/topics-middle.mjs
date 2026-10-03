export const topicsMiddle = {
51: `## Ownership 기준
Move 타입은 하나의 owner를 가지며 Read는 owner를 소비하지 않는다. take 이후 원본은 재초기화까지 사용할 수 없다. Copy/Move 판정과 local initialization/assignment의 암묵 이동 여부는 D10 초안으로 구분한다.

## 초안 — D10
Copy는 primitive, Unit, 모든 component가 Copy인 tuple/struct/enum 및 사용자 drop 없는 타입에 한정한다. string/Array/Class/Shared는 기본 Move다. let y=x와 return x는 owned value 문맥에서 Move를 허용하고 호출의 take parameter는 take x를 명시한다.

## 검증
Copy x 재사용 pass, Move x 재사용 N4101, Read 호출 뒤 재사용 pass, take parameter에 modifier 누락 fail. Shared refcount increment는 암묵 복사가 아닌 clone API로 제안한다.`,
52: `## 상태 lattice
각 local/field는 Uninitialized, Initialized, Moved, MaybeInitialized를 가진다. entry에서 parameter는 initialized, 미초기화 local은 uninitialized다. predecessor 상태가 다르면 merge에서 불확정 상태가 된다.

## transfer
읽기/빌림/take는 initialized를 요구한다. take는 moved로 전이, 유효 대입은 initialized로 전이한다. var 재대입은 기존 initialized 값을 먼저 Drop한 뒤 교체하지만 RHS는 기존 값 파괴 전에 평가하는 D10 제안이다.

## 검증
분기 한쪽 이동 뒤 읽기 fail, 모든 branch 재초기화 후 읽기 pass, loop backedge 이동, constructor field partial init, RHS 실패 시 기존 값 처리 의미를 검사한다. unknown/ErrorType 상태에서 연쇄 move 오류를 억제한다.`,
53: `## Canonical 금지
사용자 partial move는 지원하지 않는다. aggregate의 field만 take하여 나머지를 다시 사용하는 표현은 거부한다. whole-value take와 Copy field read는 허용 범주다.

## 구체 경계 — D10
take value.moveField, destructuring으로 일부 Move payload만 소유하는 pattern, closure의 일부 field take capture를 같은 규칙으로 검사한다. take 전체 Enum을 consume한 후 활성 payload 처리하는 내부 lowering은 사용자 partial move와 구별한다.

## 내부 구현
compiler의 constructor/Array 부분 초기화와 drop glue field 상태는 허용된다. 사용자 제한을 이유로 double-drop 방지용 field state를 제거하지 않는다.

## 검증
struct Move field extraction fail, Copy integer projection pass, whole struct take pass, partially initialized cleanup은 initialized field만 역순으로 Drop한다.`,
54: `## Loan 의미
Read loan은 동시 공유 가능, change loan은 독점이다. loan은 마지막 실제 사용까지 유지하며 owner의 Drop/Move/변경과 충돌을 검사한다. disjoint를 증명하지 못하면 충돌로 본다.

## Place 관계
root가 다르면 disjoint; 같은 root의 서로 다른 struct field는 disjoint 증명 가능; 같은/dynamic array index는 보수적으로 overlap이다. dereference/shared backing storage는 alias 정보를 고려한다. projection prefix 관계는 overlap이다.

## Region 계약 초안 — D10
view의 사용 지점 집합과 owner storage-live 집합을 비교한다. read/change 재빌림은 원 loan을 suspend/제한하고 child 종료 뒤 복귀한다. view return은 source parameter에 연결되는 origin 계약이 필요하다.

## 검증
last-use 뒤 mutation pass, 동시에 live read/change fail, local view 반환 fail, 다른 field change pass, dynamic index split은 NOVA-118의 증명된 API만 허용한다.`,
55: `## Drop 의미
local과 field는 선언 역순 Drop한다. moved 원본은 Drop하지 않으며 조건부 초기화에는 Drop Flag를 쓴다. 정상 scope exit, return, break, continue, try를 모두 cleanup으로 연결한다.

## 순서 초안 — D10
return value/RHS는 먼저 평가하여 목적지에 안전하게 보존하고, 이후 떠나는 scope를 안쪽부터 정리한다. 사용자 drop body와 field glue 순서는 NOVA-056의 제안을 따른다. Abort는 cleanup 경로가 없다.

## 검증
Drop trace의 exact order, branch move flag, var replacement, early return, constructor 실패, inactive enum payload를 확인한다. Drop flag 최적화는 관측되는 Drop 횟수를 바꾸지 않아야 한다.`,
56: `## 사용자 drop 초안 — D10/D12
drop { ... } 또는 drop receiver 문법은 D12에서 최종 선정한다. 초안 grammar는 drop { ... }를 제안한다. body는 자기 객체의 제한된 change 접근을 갖지만 owner를 외부로 이동하거나 부활시키지 못한다.

## Glue 순서 제안
사용자 drop body → field 선언 역순 Drop → Class storage free. destructor panic은 Abort하며 다른 field cleanup을 약속하지 않는다. drop을 가진 타입은 자동 Copy가 아니다.

## 검증
user body와 field trace 순서, recursive Drop cycle/stack budget, destructor의 self escape 거부, Class free 정확히 한 번, zero-sized field의 논리 Drop 호출을 확인한다.`,
57: `## Shared/Weak
마지막 Strong 해제에서 payload를 Drop하고 Weak는 control block만 연장한다. strong cycle은 자동 수집하지 않으며 Weak로 끊는다. Weak upgrade는 살아 있는 Strong이 있을 때만 성공한다.

## 소유권 초안 — D23
Shared는 명시 clone으로 refcount 증가하는 Move handle이다. shared와 weak는 ownership wrapper type로 제안한다. Shared의 T mutation은 자동 change loan을 허용하지 않고 별도 동기화/interior-mutation API가 필요하다.

## 검증
strong 1→0와 weak 마지막 free, upgrade/last-drop race, count overflow Abort, 순환 누수 예제, mutable Shared alias 거부. control block이 해제된 뒤 upgrade가 읽지 않는지 sanitizer로 검사한다.`,
58: `## unsafe 경계
unsafe block은 위험 연산 허용 capability이며 소유권·빌림 검사를 자동 해제하지 않는다. raw pointer는 안전 view보다 약한 보장을 가지며 safe code가 invalid pointer를 직접 생성/역참조할 수 없어야 한다.

## 초안 — D17
pointer type 철자, address-of/deref, nullable pointer, alignment/validity 계약은 FFI_TYPES에서 제안한다. 각 unsafe operation은 pointer 유효성, allocation origin, alignment, initialized range, alias, thread 조건을 기록한다.

## 검증
safe raw deref fail, pointer escape wrapper fail, foreign pointer null/length/UTF-8 검사, owner Drop 뒤 view 재사용 거부. unsafe가 ErrorType/codegen 검증을 우회하지 않아야 한다.`,
59: `## 0.1 제외 결정
Pinning와 self-reference는 Canonical 비지원이다. stable address를 이유로 owner 내부를 가리키는 안전 self view를 허용하지 않는다. Class heap allocation도 self-reference 안전성의 자동 증명이 아니다.

## 진단
노출된 pin syntax/API, 자신의 이동 가능한 storage를 가리키는 view field는 제외 기능 또는 lifetime 오류로 보고한다. unsafe raw pointer 패턴은 안전 self-reference 타입 지원이라는 뜻이 아니며 FFI 계약의 책임을 남긴다.

## 후속 연구 조건
Stage C의 Move/Drop/View를 먼저 검증한 뒤 address stability, projection, destructor 계약, ABI 영향을 별도 버전 제안한다.

## 검증
owner 내부 reference 저장/반환 fail, move 이후 dangling 포인터를 safe program이 만들 수 없는지 corpus로 검사한다.`,
60: `## Thread safety 초안 — D20
타입의 thread 이동 가능과 shared read 가능을 별도 compiler predicate로 관리한다. 이름/소스 trait 문법은 추가하지 않고 internal Sendable/Shareable 판정을 제안한다.

## 전파
primitive/owned immutable data는 component 조건을 따른다. raw pointer/borrowed local view/비동기화 mutable state는 기본 reject다. Shared<T>가 thread-safe라고 T의 동시 변경까지 허용하지 않는다.

## 검증
소유 값 thread 이전 pass, stack view escape fail, non-shareable foreign handle spawn fail, atomic refcount와 payload access 별도 race 검증. Thread API를 추가해 async/await를 선행 구현하지 않는다.`,
61: `## Interface
implements는 명시적이며 같은 type/interface 조합 구현은 하나다. 0.1 dispatch는 static이다. Interface는 compile-time constraint이며 dynamic owned object type가 아니다.

## 초안 — D15
interface body의 method signature와 receiver mode/return/effect를 정확히 구현해야 한다. default method와 interface 간 관계는 첫 승인안에서 보류한다. Generic interface는 Associated Type 대신 명시 type parameter를 사용한다.

## 검증
누락 method, change/read receiver mismatch, duplicate impl, ambiguous method, foreign package orphan 정책은 D15에 연결한다. Class 구현 상속을 interface 구현 관계로 우회하지 않는다.`,
62: `## Constraint 증명
Generic declaration은 type parameter와 where/interface constraint를 가진다. 구체 call에서 inferred type가 각 constraint를 만족하는지 registry로 증명한다. constraint를 unchecked LLVM cast로 대신하지 않는다.

## 초안 — D15
coherence는 같은 type/interface pair에 유일 implementation, orphan 규칙은 type 또는 interface 중 하나가 current package 소유여야 함을 제안한다. blanket/conditional implementation overlap은 보수적으로 오류다.

## 검증
충족/누락 impl, generic body에서 constraint 없는 method 사용 fail, 조건부 impl cycle, package 간 overlap을 다룬다. 진단은 generic 선언 constraint와 call-site type를 함께 표시한다.`,
63: `## Inference
타입 인수는 argument/receiver/expected type에서 추론한다. 명시 generic call arguments는 금지다. constraint solving은 type inference를 돕되 임의 구현 하나를 골라 type를 결정하지 않는다.

## 알고리즘 초안 — D15
fresh inference variables → parameter/actual unify → expected return unify → occurs check → substitutions normalize → interface obligations 검증. 해결되지 않은 변수가 있으면 annotation help를 제안한다.

## 검증
identity(1), expected return에 따른 empty container, incompatible repeated T, recursive T=Array<T> occurs failure, f<int>(x) 제외 진단. 실패 inference를 기본 int로 덮지 않는다.`,
64: `## Monomorphization
Generic 구현은 구체 type substitution별 코드 생성이다. key=(DefId,canonical type arguments,Target ABI,semantic options)를 사용하고 동일 key는 한 번만 생성한다.

## 작업 queue
reachable root에서 specialization 요구를 수집하고 stable key 순서로 처리한다. 동일 key 재귀는 허용하되 T→Array<T>처럼 무한 성장하는 chain은 오류다. declaration/query origin을 함께 보존한다.

## 검증
alias 동일 key dedup, recursive 동일 specialization pass, growing specialization fail, module order 변경에도 symbol/key 같음, unused generic code 미생성을 검사한다. generic Error HIR는 codegen에 넣지 않는다.`,
65: `## Cache 경계
in-memory specialization registry와 disk artifact cache를 구분한다. key는 canonical types와 body/dependency fingerprint, compiler/runtime/Target/options를 포함한다.

## 예산 초안 — D30
specialization depth/count/총 IR budget을 설정하고 초과는 가장 긴 확장 chain과 함께 진단한다. dedup와 dead code 제거를 우선하고 ABI를 바꾸는 type erasure는 0.1에 추가하지 않는다.

## 검증
body 변경 invalidation, unchanged alias reuse, Target 차이 miss, 손상 cache safe rebuild, 동일 input/options에서 count/order 같음을 확인한다. 성능 경고와 의미 오류를 구분한다.`,
66: `## 비지원 경계
Associated Type과 dynamic interface object는 0.1 제외다. interface 자체를 runtime field/parameter/return 값 타입으로 쓰거나 type-erased vtable handle을 암묵 생성하지 않는다.

## 허용 대안
Generic T where T implements I, concrete type에 implements, explicit generic interface parameters는 static dispatch 범위에서 검토 가능하다. 이것이 Associated Type syntax 지원을 뜻하지 않는다.

## 검증
interface value storage fail, associated type declaration fail, constrained concrete generic pass. error message는 concrete generic 사용 대안을 설명하되 자동 source rewrite는 하지 않는다.`,
67: `## VTable/Object safety
0.1에는 dynamic interface dispatch가 없으므로 vtable layout/object-safety 알고리즘은 납품 범위가 아니다. runtime Class handle에도 interface vtable을 자동 넣지 않는다.

## 후속 검토 항목
object ownership, lifetime, receiver mode, generic method, associated type, ABI stability, drop dispatch를 한 묶음으로 새 버전에서 설계한다. 현재 Interface signature constraints는 NOVA-061의 static 검증만 수행한다.

## 검증
backend dump에 dynamic vtable이 불필요하게 생기지 않는지 검사한다. 미래용 TODO는 release 완료 조건이나 코드 구현 요청이 아니다.`,
68: `## Pipeline 계약
Source → Lexer → END → Parser → AST → HIR → Resolution/Types → MIR → Validation → Move/Borrow/Drop → Optimization → nova-codegen → LLVM Adapter → Object → Link.

## 각 경계
결과는 데이터+진단+SourceInfo이며 user error를 Result/diagnostic으로 전달한다. AST와 HIR/Typed tables를 분리하고 Backend는 verified concrete MIR만 받는다. LLVM type는 core에 노출하지 않는다.

## Query
source_file/lex/parse/lower_hir/resolve_names/type_check/build_mir/analyze_moves/analyze_borrows/elaborate_drops/codegen_unit 경계를 유지한다. Stage A에서는 query를 일반 함수로 구현하고 아직 disk cache/병렬 scheduler를 만들 필요는 없다.

## 검증
단계별 dump, invalid source의 codegen 차단, frontend test의 LLVM 설치 불필요, Target 독립 type 판단을 확인한다.`,
69: `## Workspace 책임
core-ids/source/diagnostics/syntax/lexer/parser/ast/hir/resolve/types/typecheck/mir/analysis/codegen/codegen-llvm/package/cli가 각각 ID, source, errors, tokens, scanning, parsing, syntax tree, lowering, lookup, canonical types, semantics, CFG, safety, backend API, LLVM, packages, driver를 담당한다.

## 의존 규칙
Lexer→Parser, HIR→Typecheck, Types→LLVM, Analysis→CLI 금지. public structs에 llvm_sys/inkwell type를 넣지 않는다. 현재 필요한 crate부터 만들고 비어 있는 미래 crate를 선행 추가하지 않는다.

## 품질
cargo fmt --check; cargo clippy --workspace --all-targets -- -D warnings; cargo test --workspace; cargo check --workspace --all-features를 CI에서 실행한다. Rust/LLVM 버전은 Stage backend 시작 때 toolchain policy D28로 고정한다.

## 검증
cargo metadata graph로 금지 edge와 cycle을 검사한다. unit test, fixtures, std/runtime source의 배치를 CONTRIBUTING에서 관리한다.`,
70: `## Compiler Source 모델
FileId는 database 내 stable integer, Span은 [start,end) byte range, LineIndex는 lazy다. immutable SourceFile과 append-only SourceDatabase를 첫 계약으로 제안한다. runtime Span<T>는 NOVA-118로 분리한다.

## API
add(path,text)/add_bytes(path,bytes) → FileId 또는 input error; file(id) → source; slice(span) → checked &str; location(offset) → 1-based line/scalar column. UTF-8 boundary, start≤end, end≤length, file 존재를 확인한다.

## 현재 코드와 사양
현재 세 crate는 이 기반을 구현했지만 Rust 실행 검증은 미완료다. file ID 안정성과 persistent cache fingerprint는 별개다. source revision을 지원할 때 기존 Span을 새 text에 자동 적용하지 않는다.

## 검증
Unicode/CRLF/CR/BOM/EOF/빈 파일/큰 파일/unknown FileId, lazy index 1회 계산, invalid span에서 panic 없음. mixed 문서 항목의 region/ABI는 Stage C로 추적한다.`,
71: `## Lexer 인터페이스
lex(SourceFile) → RawTokens,Trivia,Diagnostics. 원문 byte 범위가 token/trivia에서 겹치거나 누락되지 않으며 EOF는 length 위치의 빈 Span이다.

## 실행 구조
cursor는 UTF-8 char boundary를 지킨다. identifier/number/operator scanning은 최장 일치, string/comment/interpolation은 mode stack. 잘못된 char는 최소 한 scalar를 소비한 Error token이다. number range는 type 단계에서 검사한다.

## 복잡도
각 byte를 상수 횟수 방문하여 O(n)을 목표로 한다. 매 identifier마다 suffix 전체 복사, nested comment 재검색, 반복 문자열 concat을 피한다. depth limit은 D30 진단으로 처리한다.

## 검증
LEXICAL/END_RULES fixture, source reconstruction, Unicode boundary, fuzz timeout/crash, 동일 raw input의 stable dump.`,
72: `## Parser 인터페이스
parse(NormalizedTokens) → AST arena/root/diagnostics. declaration/statement/type은 Recursive Descent, expression은 Pratt며 source trivia를 type checker로 보내지 않는다.

## 상태
cursor, delimiter stack(open Span 포함), diagnostics, recovery budget, grammar version을 가진다. 함수 boundary가 return 분석과 delimiter 복구의 경계다. generic >는 type context에서만 닫힘으로 처리한다.

## 검증
GRAMMAR.ebnf의 모든 production positive/negative fixture, chained compare/assignment-expression 거부, truncated IDE input, token consumption progress. Recovery AST는 debug dump는 되지만 codegen 성공으로 표시하지 않는다.`,
73: `## AST 데이터
Arena node는 AstNodeId/Span/NodeKind/token anchor를 가진다. item은 Func/Struct/Class/Enum/Interface/Foreign/Use/Const/TypeAlias, statement는 Binding/Assign/Expr/Return/If/While/For/Loop/Match/Using/Unsafe/Error를 제안한다.

## Expression
Literal/Name/Prefix/Binary/Call/Member/Index/Cast/Exists/Tuple/Array/Lambda/Interpolation/Error. semantic TypeId와 DefId는 저장하지 않는다. grammar가 지원하지 않는 미래 node를 미리 구현하지 않는다.

## Visitor
source-order walking, immutable visit와 controlled transform을 분리한다. Error/synthetic node도 방문하고 Span/source origin을 잃지 않는다.

## 검증
node ID 유일성, 부모 Span child 포함, trivia anchor, ErrorNode visitor, arena dump의 deterministic order.`,
74: `## HIR 데이터
HirItemId/HirExprId/HirStmtId/HirPatternId/ScopeId와 source origin을 사용한다. T?→Option<T>, void/생략→Unit, ()→UnitValue로 정규화한다.

## 유지하는 구문
try/exists/using/for는 전용 HIR로 보존한다. 이름 인수는 Label+SourceOrder, 할당은 HirStatement다. early lowering으로 argument reorder/cleanup 의미를 감추지 않는다.

## Side tables
ResolutionMap, TypeTable, CoercionTable, OwnershipTable, EffectTable을 독립 산출물로 관리한다. HIR은 trivia를 참조하지 않으며 Error HIR로 분석을 계속할 수 있다.

## 검증
AST sugar와 canonical HIR 동등성, synthetic source origin, source-order 유지, ErrorType/codegen 차단.`,
75: `## Lowering 단계
AST arena → HIR arena와 lexical ScopeTree. 선언 수집과 body lowering을 분리하여 forward reference를 지원한다. identifier spelling은 SymbolId로 intern하되 아직 unresolved reference일 수 있다.

## Normalize 계약
Primitive alias/nullable/Unit sugar는 한 곳에서 처리한다. implicit default call argument 삽입은 resolution 이후 하며 SourceOrigin=DefaultArgument를 남긴다. ownership-sensitive for/try/using은 semantic 전용 node로 보존한다.

## 검증
같은 AST의 반복 lowering 결정성, Error AST→Error HIR, synthetic node note 위치, named argument 평가 순서, alias source spelling 진단을 검사한다.`,
76: `## Registry
DefinitionRegistry는 DefId→name,kind,owner,visibility,Span,signature source를 보유한다. ScopeTree는 parent/namespace bindings/import edges를 보유한다. Symbol interner는 session 문자열만 관리한다.

## mutation 경계
declare 단계 종료 뒤 registry의 identity/owner를 고정하고 resolution/type side table은 별도로 생성한다. duplicate 선언은 임의 overwrite하지 않고 충돌을 기록한다. LocalId는 function body 소유다.

## 검증
중복 선언 보존/secondary label, cross-module forward call, import order permutation, root namespace 격리, unresolved reference→Error resolution 처리.`,
77: `## Type/Ownership 분리
type checker는 typed expressions/call resolution/coercion과 요구 ownership mode를 계산한다. MIR analysis가 concrete CFG에서 initialization/move/borrow/drop 검사를 수행한다. source 수준 검사와 CFG 수준 검사를 중복 구현하더라도 책임을 명확히 한다.

## 데이터 계약
TypedBody는 TypeTable/CallResolution/OwnershipAction/Effects/Diagnostics를 포함한다. generic body는 symbolic constraint 상태를 구체 body와 구분한다. LLVM pointer type를 타입 안전성 판단에 쓰지 않는다.

## 검증
type mismatch에서 파생 borrow 오류 억제, mode-aware call, branch merge, ErrorType 차단, stage가 지원하는 타입 subset 선언.`,
78: `## Diagnostic 구조
code/severity/message/primary/secondary/notes/suggestions를 보존한다. N1xxx syntax, N2xxx name/type, N3xxx control/const, N4xxx ownership, N5xxx ABI/runtime, N8xxx tooling/package, N9xxx ICE다.

## Renderer
Plain/ANSI/JSON/Snapshot을 제공한다. byte Span과 Unicode scalar 표시 열은 다르다. 다중 파일/다중 줄/EOF label, note/help, suggestion applicability를 잃지 않는다. JSON schema는 DIAGNOSTIC_SCHEMA에 제안한다.

## 코드 운영
DIAGNOSTICS.csv의 각 코드에는 category, trigger, primary/secondary 의미, Stage를 기록한다. proposal을 승인 전 안정 API라고 부르지 않는다. code는 폐기 후 재사용하지 않는 D25 제안이다.

## 검증
renderer metadata 보존, control character escaping, invalid internal Span은 ICE 또는 명시 error, 연쇄 오류 suppression, sorting 결정성.`,
79: `## Query 계약
key/value/dependency fingerprint/diagnostics를 분리한다. query identity는 compiler/schema/options/Target과 관련 input을 포함한다. LLVM backend 결과만 Target key가 필요하다고 가정하지 않는다; layout/const도 Target 의존이다.

## 무효화
source text, imported export signature, generic body, runtime ABI, options가 바뀌면 해당 dependency path 결과를 무효화한다. query cycle은 user semantic cycle과 compiler dependency bug를 구분한다.

## 단계적 도입
Stage A는 pure function 호출과 session memoization으로 충분하다. disk cache는 E에서 checksum, atomic write, trust boundary를 검증하고 도입한다.

## 검증
unchanged reuse, body-only change 영향, public signature change, corrupted disk entry miss/rebuild, serial/parallel 동일 diagnostics.`,
80: `## ICE 경계
잘못된 Nova source는 user diagnostic이며 panic 원인이 아니다. verified MIR invariant/LLVM verify 실패처럼 compiler 내부 불변 조건이 깨지면 ICE다. system I/O/toolchain 실패와도 분리한다.

## 보고 계약
compiler version/build ID, Target, command options, query stack, source locations, reproduction dump 경로를 포함한다. CLI exit=101을 따른다. source/환경 비밀값 업로드는 자동 수행하지 않는다.

## 검증
fault-injected invariant fail은 N9xxx와 exit101, user malformed source는 N1xxx/exit1, missing linker는 exit3. 모든 재현 가능한 ICE는 최소 corpus regression을 추가한다.`,
81: `## MIR 구조
Body는 typed locals/scopes/basic blocks를 가지며 block은 Statements와 하나의 Terminator다. Place=root+projection, Operand=Copy/Move/Constant, Rvalue=Use/Binary/Aggregate/Ref 등으로 제안한다.

## Terminator
Goto,Switch,Call,Drop,Return,Abort,Unreachable를 구분한다. Call/Drop은 원본 규칙대로 Terminator이며 successor를 명시한다. SourceInfo에 Span과 scope를 저장한다.

## 평가
left-to-right 임시값과 CFG로 source order를 고정한다. short circuit은 Switch, try는 variant branch, return은 return place+cleanup이다. unwind edge는 0.1 Abort 의미에 추가하지 않는다.

## 검증
typed operands, block target, place projections, cleanup path, no ErrorType/unresolved generic, exact Drop trace.`,
82: `## HIR→MIR
typed HIR/call resolution/layout-independent type info를 입력으로 받는다. Error body는 lowering 성공이 아니며 backend unit에서 제외한다. temporary/local IDs는 source traversal에 따라 결정적으로 할당한다.

## lowering 계약
값을 한 번 평가해 temporary에 저장하고 source-order operation을 이어간다. if/loop/short-circuit은 CFG, match/try는 tag Switch, named/default arguments는 평가 temp와 전달 order를 분리한다.

## 검증
side-effect function trace, assignment RHS-before-old-drop, early return cleanup, loop continue target, view lifetime source origin. MIR dump가 source 의미를 역추적할 수 있어야 한다.`,
83: `## Validator
pre-analysis와 post-drop/optimization validator를 분리한다. block terminator 존재, successor/local/type IDs 유효, projection type 일치, operand/rvalue type, return/call signature, switch tag 범위를 확인한다.

## 안전 입력
codegen 전에는 ErrorType, unresolved generic, uninitialized read, illegal Move/Loan, unelaborated Drop를 허용하지 않는다. early MIR에서는 아직 분석 전 정보가 있다는 점을 validator phase로 표현한다.

## 검증
deliberately malformed MIR corpus를 만들고 mismatch마다 stable internal code와 source origin을 보고한다. optimizer 전후 validator 모두 통과해야 한다. invalid CFG의 cycle 자체를 오류로 보지는 않는다.`,
84: `## Dataflow 구현
CFG predecessor/successor worklist로 initialization/move 상태의 least fixed point를 계산한다. finite lattice와 monotone transfer를 사용하고 loop에서도 종료해야 한다.

## 단위
Move path는 root와 field initialization 상태를 추적한다. 사용자 partial move 금지는 별도 semantic check지만 constructor/Array 내부 init tracking은 유지한다. use 지점과 move origin의 secondary label을 연결한다.

## 검증
diamond merge, loop-carried owner, unreachable block 제외, reinitialize then read pass, moved then borrowed fail, analysis iteration 순서 바꿔 동일 결과.`,
85: `## NLL 구현
Ref creation에서 Loan(place,mode,origin)을 만들고 use points, CFG reachability와 region constraints로 live range를 계산한다. lexical scope 전체를 무조건 lifetime으로 사용하지 않는다.

## conflict
loan live point에서 overlapping Read/change, mutation, take, Drop을 검사한다. struct field disjoint 증명, reborrow suspend, external alias uncertainty를 PlaceRelation에 분리한다. dynamic index는 보수적이다.

## 검증
last-use 직후 mutation pass, branch-specific loan, loop NLL, returning local view fail, reborrow parent access fail, SplitAt non-overlap pass. soundness 반례를 최소 regression으로 보존한다.`,
86: `## Drop elaboration
Move analysis의 initialized state를 읽어 Drop sites/flags를 삽입한다. 함수 lexical scopes와 edge exit scopes를 사용해 cleanup blocks를 공유하되 순서를 바꾸지 않는다.

## 계약
StorageLive/Dead와 논리 ownership를 구분한다. moved flag는 false, successful init은 true, flag-guarded Drop 뒤 false다. Enum은 active payload만, Class는 user body/fields/free 순서를 따른다.

## 검증
conditional return, loop break/continue, try Error path, var overwrite, field init failure, optimizer 뒤 exact once. panic Abort에는 fictitious cleanup edge를 넣지 않는다.`,
87: `## Const engine
typed const IR/MIR의 허용 subset을 interpreter로 평가한다. Nova target primitive semantics를 구현하고 host arithmetic을 그대로 호출해 wrap하거나 UB를 만들지 않는다.

## 입력/출력
ConstKey=(DefId,substitution,Target,semantics version) → ConstValue 또는 Source diagnostic. recursive evaluation stack은 dependency cycle chain을 남긴다. allocation/I/O/FFI는 D09 subset에서 거부한다.

## 검증
타입검사 숫자 모델과 동등, bounds/div-zero compile fail, large integer parsing, float exact rounding corpus, deterministic step budget, cycle diagnostics.`,
88: `## Match pass
Pattern matrix에서 coverage와 decision tree를 만들되 NOVA-047 분석과 동일 constructor set을 사용한다. payload projection은 verified tag branch 아래에만 놓는다.

## 보존 조건
guard 순서/side effect, binding mode/loan region, scrutinee once, arm source order, cleanup을 유지한다. payload destructive extraction을 guard 전에 수행하여 실패 후 다음 arm에서 읽을 수 없게 만들지 않는다.

## 검증
nested enum tests 공유, wildcard unreachable, guarded fallback, moving whole scrutinee, partial move fail, same trace with/without tree optimization.`,
89: `## Specialization pass
type substitution를 typed HIR/MIR에 적용하고 concrete type/obligation를 재검증한다. call graph에서 필요한 specialization을 queue에 추가한다. generic body마다 새 syntax parser를 실행하지 않는다.

## key/symbol
canonical type와 owning package identity를 사용하고 mangling NOVA-097과 일치시킨다. 같은 specialization의 concurrent 요청은 한 결과를 공유한다.

## 검증
cross-module dedup, inference failure source label, recursive growth detection, foreign ABI generic exclusion, cache invalidation/serial-parallel equivalence.`,
90: `## Optimization 초안 — D26
Stage A는 검증 → 상수 folding(동일 숫자 의미) → unreachable block 제거 → 단순 CFG 정리 → 검증을 제안한다. C 이후에는 Drop elaboration 완료 뒤 dead temporary/copy propagation을 제한적으로 추가한다.

## 금지
side effect/Drop/abort 가능 operation을 제거하거나 reorder하지 않는다. integer checked arithmetic를 LLVM unchecked add로 바꾸지 않는다. fast-math/reassociation은 의미 변경이므로 기본 제외다.

## 검증
O0/O2 관찰 결과 비교: stdout/exit/Drop trace/error, overflow/bounds, short-circuit. 성능 개선은 correctness suite 통과 이후 측정한다.`,
91: `## 비SSA 결정
MVP MIR은 Place 기반 비SSA다. Move/Borrow/Drop 분석은 SSA 이전에 수행한다. phi node 요구 때문에 source ownership 모델을 바꾸지 않는다.

## Backend 경계
LLVM의 memory-to-register promotion을 사용할 수 있으나 Nova cleanup/alias 계약은 먼저 MIR에 확정한다. 별도 optimization SSA는 후속 work item이다.

## 검증
loop-carried locals, conditional Drop flags, branch merge return, address-taken local을 O0/O2로 비교한다. compiler core에 LLVM SSA Value type를 노출하지 않는다.`,
92: `## MIR dump schema 초안
header: schema/compiler/Target/pass/body ID. locals: _N:type,mode,scope,source. blocks: bbN { statements; terminator }. projections는 field/index/deref와 byte Span을 별도 필드로 출력한다.

## 결정성
memory address/hash-map iteration/time를 출력하지 않는다. local/block 번호는 canonical traversal order로 normalize하되 semantics-affecting operand order는 유지한다. before/after pass 이름을 기록한다.

## 검증
같은 입력 두 번 byte 동일, parallel jobs 동일, phi 없는 loop CFG, Call/Drop successor와 SourceInfo 표시. snapshot schema 변경은 version과 승인된 baseline update를 요구한다.`,
93: `## Backend 결정
Rust compiler와 LLVM Adapter를 유지한다. nova-codegen의 CodegenBackend는 verified CodegenUnit/TargetSpec/Options를 받아 ObjectArtifact 또는 CodegenError를 반환한다.

## Pipeline
MIR concrete verify → LLVM Module → LLVM verify → optimize → object → linker. LLVM version/binding 선택은 D28에서 toolchain 잠금으로 정한다. frontend build는 LLVM 없이 가능해야 한다.

## Target 범위
원본 우선순위는 Windows x86_64 MSVC, Linux x86_64 GNU, Linux AArch64, Windows AArch64다. release 필수 첫 두 Target과 나머지 experimental 여부는 D28에서 제안한다. JIT/Wasm/GPU/direct machine code는 제외다.

## 검증
adapter isolation, LLVM verify failure→ICE, wrong toolchain→exit3, Hello object/link/run.`,
94: `## Mapping 계약
bool register i1, memory bool representation은 Layout; intN/uintN→iN; float/double→float/double; char→32-bit scalar storage를 D16에서 제안한다. string/Array/Enum/Class/view는 Layout/ABI 결과에 따라 lowered aggregate다.

## 금지 가정
Unit가 모든 ABI에서 1-byte인지, Never가 return register를 갖는지, pointer가 64-bit인지 가정하지 않는다. checked arithmetic는 overflow intrinsic/guard, bounds는 conditional abort로 표현한다.

## 검증
Target endian/align, unsigned comparison, sign extension vs zero extension, Unit return, Never call 뒤 unreachable, option niche와 일반 enum 같은 source 의미.`,
95: `## Layout lowering
core Layout를 LLVM DataLayout와 대조한다. field offsets/align/padding/tag/payload를 하나의 Target query 결과로 계산하고 backend의 임의 재배치를 금지한다.

## Niche
valid bit pattern 밖의 representation만 사용한다. optimization enable/disable이 discriminant test/Drop 의미를 바꾸지 않아야 한다. foreign representation은 explicit ABI-safe mapping으로 제한한다.

## 검증
sizeof/alignof/offset golden, LLVM structural type padding, over-aligned allocation, zero-sized element, enum maximum payload, C roundtrip harness.`,
96: `## Call lowering
semantic Signature → AbiSignature(args PassMode,return,destination,calling convention). source evaluation temp와 physical register/stack position을 분리한다.

## modes
Read scalar direct 또는 aggregate readonly pointer, change exclusive pointer, take owned representation 전달을 제안한다. readonly/noalias/nocapture는 분석으로 증명된 범위만 부착한다. Shared pointer와 raw FFI에는 추측 noalias를 넣지 않는다.

## 검증
caller/callee ABI 일치, named source order, large return destination, foreign callback context lifetime, stack alignment/aggregate classification across first two Targets.`,
97: `## Symbol 초안 — D16
Nova symbol은 language ABI version, package identity, module/item path, canonical signature/type args를 length-prefix encoding으로 포함한다. delimiter만 연결하여 이름 충돌을 만들지 않는다.

## 외부 이름
C export/import는 explicit symbol를 사용하며 Nova mangling을 붙이지 않는다. Entry wrapper C main과 user Nova main을 분리한다. Unicode identifiers는 UTF-8 byte length 또는 stable encoded spelling으로 처리한다.

## 검증
overload/generic/package revision 구별, alias canonical dedup, 동일 build의 symbol order, ambiguous concatenation 반례, external symbol duplicate linker diagnostic. ABI version 변경 시 rebuild한다.`,
98: `## Object/Link 계약
ObjectArtifact는 path,Target,format,compiler/runtime ABI,checksum을 가진다. COFF/ELF는 Target에 맞추며 output/temp path는 workspace output 아래로 제한한다.

## Linker invocation
argument list API를 사용하고 source/package 값을 shell command로 연결하지 않는다. required runtime/system libs, search paths, entry/export를 명시한다. toolchain discovery 실패, missing symbol, unsupported Target은 사용자 코드 오류와 분리한다.

## 검증
공백/한글 경로, object format mismatch, missing library/symbol, response-file 큰 command, temporary cleanup, partial output를 정상 executable로 오인하지 않음을 확인한다.`,
99: `## Startup
OS entry → platform runtime initialization → Nova main → 정상 shutdown/exit. user main return/argv 의미는 D19를 따른다. source-level main을 platform ABI로 직접 노출하지 않는다.

## cleanup 경계
정상 종료는 main local cleanup 후 runtime resource 종료다. Abort panic은 unwinding/global destructor 보장을 하지 않는다. module runtime 초기화 순환은 금지하고 deterministic startup order를 유지한다.

## 검증
빈 main exit0, print UTF-8, allocation/panic startup path, runtime symbol link, repeated process invocation independence, library target에 entry 미생성.`,
100: `## Allocation 초안 — D18
allocator API는 size/align/checked capacity multiplication을 받는다. allocation failure는 Abort로 제안하며 일부 API의 recoverable OOM 여부는 별도 승인 없이는 추가하지 않는다.

## 불변 조건
allocation provenance를 유지하고 alloc/free pair 및 align을 맞춘다. Array 성장 시 이전 initialized values가 새 storage로 안전하게 이전된 뒤 old storage를 해제한다. zero-sized allocation policy는 Layout/Runtime 계약에 명시한다.

## 검증
fault-injected OOM, capacity overflow, over-alignment, realloc 실패, double-free 없음, host/Target pointer width 차이. OOM 테스트는 실제 시스템 메모리 고갈 대신 allocator injection으로 수행한다.`,
101: `## Panic
0.1 panic은 Abort다. unwind/catch를 도입하지 않고 FFI 경계를 넘어 예외를 전파하지 않는다. message와 Source 위치를 기록한다.

## Runtime 초안 — D18
runtime panic entry는 code/message/file/line을 받고 stderr best-effort 출력 후 abort한다. recursive panic/allocator failure 상황에서도 무한 재귀를 피한다. stack trace는 optional hook이며 실패해도 abort가 진행된다.

## 검증
explicit panic, bounds, integer overflow, division error, no cleanup-on-abort, stderr 위치. OS별 abort exit 값은 numeric 하나로 고정하지 않고 abnormal termination class로 검사한다.`,
102: `## Control block
atomic strong/weak counts, payload lifetime flag, drop/free function과 allocation metadata를 가진다. strong 0은 payload dead, weak 0은 control block free다. implicit weak bookkeeping 여부는 내부 구현이며 수명 의미를 바꾸지 않는다.

## Atomic protocol 초안 — D20
clone overflow checked increment, final decrement acquire/release synchronization, upgrade CAS nonzero strong을 제안한다. publication/payload access와 refcount ordering을 별도 증명한다.

## 검증
upgrade vs final drop interleavings, exactly one payload drop, last weak free, overflow, concurrency stress/model checking. refcount 자체의 atomicity가 T data race를 해결한다고 주장하지 않는다.`,
103: `## Thread/Lock/Atomic runtime 초안 — D20
thread spawn/join, mutex lock guard, fixed-width atomic operations를 platform layer 뒤에 제공한다. join 결과와 worker panic Abort는 별개의 failure class다.

## 안전 계약
lock guard는 scope Drop로 unlock하며 lifetime은 mutex보다 짧다. Mutex<T> 공유는 T predicate를 검사한다. atomic order는 Relaxed/Acquire/Release/AcqRel/SeqCst를 명시하고 불가능한 load/store order는 거부한다.

## 검증
guard early return unlock, deadlock timeout test, worker ownership transfer, relaxed counter vs publication 예제, OS resource close. async runtime/condition-free spin을 기본 구현으로 추가하지 않는다.`,
104: `## Platform layer
file/console/env/clock/random/thread/allocator/dynamic library의 최소 OS 경계를 모듈화한다. 언어 의미와 API error category는 플랫폼에 독립, 구체 OS error code는 note/source error payload다.

## Target 계약
Windows wide path conversion과 Linux byte path 차이는 public Path API에서 정의한다. Console는 UTF-8 contract이며 Windows terminal conversion을 runtime에서 수행한다. newline text translation은 API가 명시한 경우만 한다.

## 검증
Unicode path/output, invalid OS encoding, permission error, monotonic time, secure random failure, resource leak counts across Windows/Linux. backend Target support와 std API support를 별도 표로 관리한다.`,
};
