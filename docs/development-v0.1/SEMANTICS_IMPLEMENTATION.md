# P02 Stage A 의미 검사 구현 계약

사양: [사용자 승인 P02](SEMANTICS_STAGE_A_PROPOSAL.md). Parser는 [P01](PARSER_STAGE_A_PROPOSAL.md).
지원 타입은 Int32/Bool/String/Unit. internal Function 표식은 call typing에만 쓰며 함수 값 기능이 아니다.

## API와 책임

SourceDatabase → Lexer → normalize_ends → Parser → HIR lower → resolve → typecheck 순서다.
Lexer 또는 Parser 오류가 있으면 harness가 성공 분석 경로를 차단한다.

- nova_hir::lower(sources, arena, root) → Result<Module, LoweringError>.
- nova_resolve::resolve(module) → Resolved.
- nova_typecheck::check(module, resolved) → Result<Checked, CheckError>.
- Checked.has_errors()는 이름/타입 diagnostics, recovery HIR, ErrorType/invalid TypeId를 확인한다.

사용자 의미 오류는 diagnostics와 partial tables로 반환한다. malformed AST shape/source metadata,
다른 HIR의 잘못된 resolution tables는 API 오류다. Syntax recovery를 조사할 때 Error HIR을
lowering할 수 있지만 upstream diagnostics를 버려서는 안 된다. 성공 codegen 조건에는
모든 upstream 오류와 Checked.has_errors()가 함께 포함되어야 한다.

HIR은 AST에 semantic TypeId/DefId를 넣지 않는다. Module은 flat immutable node arena와
deterministic symbol interner, SourceOrigin을 가진다. `void`/`()`/생략 반환은 UnitType,
Primitive alias는 type 구문에서만 canonical spelling으로 바꾼다. 같은 철자의 값 이름은 보존한다.
생략 반환은 함수 name 끝의 zero-width Span과 ImplicitReturn origin을 가진다.
String escape/brace를 decode해 NUL·Unicode를 포함한 UTF-8 내용을 저장한다. 보간 expression 순서는 유지한다.

## 이름·타입 side tables

HirId/SymbolId/DefId/ScopeId는 해당 Module/분석 안에서만 안정적이며 영구 cache ID가 아니다.
DefinitionRegistry에는 builtin/function/parameter/local과 name Span이 남는다. prelude → file →
function/branch Scope 순서이며 parameter와 함수 최외곽 body는 같은 Scope다.
초기값을 먼저 해석한 후 local을 등록한다. 중복을 overwrite하지 않고 이전 선언 secondary와 함께 거부한다.

ResolutionMap/TypeTable/signature/call target/return-path tables는 HIR과 분리한다.
이름은 현재 source-order의 가장 가까운 Scope에서 DefId로 해석한다. 파일 함수는 먼저 모두 수집한다.
타입 검사는 signature/annotation의 expected type을 argument/initializer/return에 전달한다.
ErrorType compatibility로 파생 진단을 억제하되 독립적인 이름·타입 오류는 계속 검사한다.

Int32 Literal은 magnitude accumulation 전에 range를 확인한다. 직접 `-2147483648`은 prefix
전체를 signed literal로 검사하며 IntegerValues의 값은 prefix HIR ID에 기록한다. 그 magnitude
child를 standalone positive constant로 codegen하면 안 된다. `-(2147483648)`은 range 오류다.
arithmetic expression을 평가/constant-fold하지 않으므로 `1/0`의 runtime 의미를 이 단계에서 실행하지 않는다.

if/else 양쪽 return, 이후 unconditional return을 반환 경로로 인정한다. unreachable code도 검사한다.
main 존재/signature는 fragment 검사에 포함하지 않는다. print는 String 인수 하나와 Unit 반환이며
실제 console output/formatting/newline/ABI는 다음 Native 단계의 계약이다.

## 검증

- HIR 6개: Unit/alias/source origin, String decode·순서, 값 이름 보존, malformed/recovery AST,
  모든 truncated prefix lowering, Hello HIR snapshot.
- TypeInterner 1개: 안정적 ID와 ErrorType compatibility.
- frontend 17개: Hello와 함수/재귀, 6개 fail fixture의 code/Span, scope/duplicate/shadow,
  MIN/MAX와 base/underscore, expected types, print/interpolation, all-return paths와 unreachable 오류.
- resolution/typed snapshots, 오류 syntax 성공 분석 차단, UTF-8/CRLF 위치, 결정성,
  10,000개 flat call statement 및 잘못된 resolution table의 API 오류.
- 모든 traversals와 storage/drop은 반복형이다. 검증은 frontend-pass/fail이며 Native 실행을 뜻하지 않는다.

다음 단계는 MIR lowering/validation이다. ownership, overload, 숫자 승격, module, LLVM/CLI 및
entry/runtime/print 실패 계약 전체는 아직 구현하지 않았다.
