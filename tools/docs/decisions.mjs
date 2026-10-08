export const decisions = [
['D01','추가/문맥 키워드','A~E','004/009/020/030/038/041/105','use는 Parser에 있으나 공식 목록에 없고 type/lambda/noPanic/library/self 분류가 없다.','use/type/lambda를 추가 keyword 후보, self/c/library를 contextual, noPanic은 보류한다. 공식 제외 어휘를 alias로 재해석하지 않는다.','use만 추가하고 alias/lambda/effect 구문을 후속으로 미룬다.','T001'],
['D02','Unicode와 줄바꿈','A','008/070/133','UTF-8만 정해졌고 identifier Unicode/BOM/CR 정책이 없다.','고정 Unicode XID+_, NFC 변환 없음, initial BOM trivia, LF/CRLF/CR event, byte/scalar/UTF-16 위치 분리.','ASCII identifier만 시작하거나 NFC 정규화를 명시한다.','T002'],
['D03','연산자와 결합성','A~B','009/014/015','우선순위 계층만 있고 정확한 철자/결합성/cast 층이 부족하다.','GRAMMAR의 operator 집합, 산술 left, prefix right, compare/range nonassoc, cast postfix, assignment statement. ::와 @는 경로/attribute token.','cast 별도 낮은 층, bitwise/shift/compound assignment 추가를 별도 제안.','T007'],
['D04','Literal/escape/comment','A~B','010/011/012/071','nested comment/interpolation 방향은 있으나 literal 철자/escape/복구가 불완전하다.','LEXICAL의 base/underscore/decimal float/Unicode escape, char 1 scalar, nested /* */, brace interpolation, raw/multiline 제외.','decimal-only Stage A, raw string 별도 지원안.','T003'],
['D05','END 상세','A','013/014/071','operator/header/return/EOF 경계의 충돌 우선순위가 없다.','END_RULES 우선순위와 header state, return newline bare return, boundary before }/EOF. generic type delimiter는 comparison과 구분한 type context에서 처리한다.','Parser가 soft newline을 직접 처리하되 normalized END 구조는 유지한다.','T006'],
['D06','Scope/Module/Visibility','A~B~E','019~024','namespace 방향 외 import grammar/default visibility/경로 충돌 정책이 없다.','qualified :: path, use alias, src 경로 module, same-scope duplicate 오류/nested shadow 허용, public/internal/private 경계.','dot module path, private 기본, wildcard import 제한 도입.','T011'],
['D07','숫자 모델','A~B','025/026/037/087/115','lossless 승격은 있으나 기본 literal/overflow/cast/float가 미정이다.','NUMERIC_RULES: int32/float32 default, 전체 범위 lossless widening, checked Abort 모든 profile, float IEEE, fast-math off.','기본 float64, explicit-only mixed numeric, wrapping 별도 operator.','T008'],
['D08','표현식/제어 흐름/Pattern','A~B','028/043~050','if/match/loop의 value 여부, bool 조건, guard, range, try E 변환이 미정이다.','statement control flow, bool-only 조건, left-to-right, short circuit, integer step1 range, explicit pattern mode, exact-E try, using→Drop.','if/match value expression 허용, explicit error-conversion protocol 추가.','T012'],
['D09','const subset','B','032/087','허용 연산과 evaluation budget이 미정이다.','순수 primitive/aggregate const subset, allocation/I/O/FFI/user const function 제외, target semantics와 budget.','const func 표기를 추가하되 별도 effect/termination 설계.','T018'],
['D10','Move/Loan/Drop 상세','C','051~056/070/084~086/118','핵심 모드는 있으나 Copy 조건/암묵 move/return view/drop body 순서가 부족하다.','primitive/component Copy, owned binding/return move, explicit take call, no partial move, NLL, user body→역순 field Drop, local view return 금지.','모든 move에 take 필수, 별도 lifetime annotation syntax.','T020'],
['D11','호출/Receiver/Overload','A~D','035~039','caller default 외 순서/receiver 철자/비용 tie-break가 없다.','self/change self/take self, named 뒤 positional 금지, provided source order 후 default declaration order, identity0/widen1 Pareto.','argument order 제한 강화, default는 const-only, specificity tie-break 추가.','T014'],
['D12','Aggregate/alias/constructor','B~D','027/029~034/056','완전 declaration grammar 및 equality/constructor/drop 표기가 없다.','GRAMMAR init/drop/members/tuple/Array, field let/var, explicit init, enum tuple payload. type alias와 Class identity API를 구분.','struct literal 생성, named enum payload, 별도 nominal newtype.','T016'],
['D13','Closure 구문/캡처','D','038/039','capture 분류 외 source 구문/default capture/call receiver가 없다.','lambda + optional capture list, implicit capture는 실제 use의 요구 mode로 검사하되 take/change 요구를 명시, owned environment와 once-call.','암묵 capture를 모두 금지하고 explicit capture만 허용.','T024'],
['D14','pure/noPanic','B~D','041/074','pure 제한 범위와 noPanic 표기가 미정이다.','pure 외부 effect 보수적 금지, noPanic 지원/철자 보류, transitive EffectTable.','pure를 후속 기능으로 제외하거나 보다 좁은 observable-state 계약.','T030'],
['D15','Generic/Interface coherence','D','061~065/089','constraint 철자/orphan/overlap/default methods 정책이 없다.','explicit implements/where, concrete type inference, unique pair, local type 또는 interface 소유, conservative overlap, no default method 최초안.','orphan 없이 global graph coherence, default method 별도 허용.','T023'],
['D16','Layout/ABI/Mangling','A~E','033/042/094~097/106','Target ABI 세부와 stable Nova ABI가 없다.','target layout query, bool memory byte/char32 후보, direct/indirect pass, versioned length-prefixed mangling, foreign ABI 별도.','초기 ABI 전면 unstable + 모든 artifact rebuild; C-repr subset 확장.','T027'],
['D17','C FFI/Pointer/Annotation','E','058/105~110','foreign example 외 raw pointer 철자와 wrapper 계약이 불완전하다.','*T/*change T 후보, explicit unsafe deref API, contextual library/@symbol, ownership/length/null/thread annotation manifest, C shim exceptions.','raw pointer source syntax를 숨기고 builtin opaque handle API부터 제공.','T026'],
['D18','OOM/Panic 실패 정책','A~E','100/101','Abort는 정해졌지만 OOM/print 실패와 trace 방식은 미정이다.','OOM Abort, stderr best effort panic+Span, no unwind, optional trace, recoverable I/O는 Result.','recoverable allocator API subset과 infallible allocation을 분리.','T028'],
['D19','Entry/Main/Startup','A~E','040/099','main() 외 허용 signature/argv/exit 코드가 없다.','최초 parameterless Unit main, exit0, args std API, platform wrapper.','int/Result main를 함께 지원하고 startup mapping 명시.','T013'],
['D20','Thread/Atomic 안전성','C~E','057/060/102~104/124','thread predicates/order/Shared payload mutation 정책이 미정이다.','internal Sendable/Shareable, explicit atomic order, conservative foreign handles, guard Drop unlock, Shared clone explicit.','thread API 전체 후속화 또는 source interface predicates 설계.','T029'],
['D21','Package/Lock/Artifact/Cache','E','126~130/135','필드 방향만 있고 schema/source/resolve/atomic write가 없다.','MANIFEST_SCHEMA v1, path+pinned git, locked/offline, SHA256 content, versioned artifact/cache, no registry/build hooks.','path-only 첫 release 또는 registry client를 별도 scope 승인.','T031'],
['D22','CLI I/O와 child exit','A~E','125','명령과 compiler exit는 있으나 run child/output/clean 경계가 없다.','CLI_CONTRACTS, check 무산출물, build output dir, run 성공 build 뒤 child 상태 전달, JSON diagnostics stream 구분.','child exit를 envelope JSON로만 전달하는 mode.','T032'],
['D23','Core/std 공개 API','A~E','114~124','UTF-8/Array/Enum 개념 외 exact API/print newline/List 역할이 없다.','STDLIB_API의 모드/Result/panic/cost, print line output, List=Array convenience, Move Shared explicit clone.','print non-newline+println 별도, List 별개 추상 container.','T033'],
['D24','Formatter/LSP/문서','E','018/131/133/134','100열 외 줄바꿈/오류 파일/position/doc comment 규칙이 부족하다.','4-space/LF/final newline, parse error write 거부, UTF-16 LSP 기본, stale version discard, /// doc, sanitized HTML.','프로젝트 formatting config 허용, negotiated LSP encodings.','T034'],
['D25','진단/Lint/Test schema','A~E','024/078/132/136~140','범위만 있고 individual code/schema/lint suppression/baseline 정책이 없다.','DIAGNOSTICS.csv 제안 code, structured schema v1, sidecar exact Span tests, approved baseline updates, code reuse 금지.','원본 annotation-only harness 또는 richer multi-edit suggestions schema.','T035'],
['D26','Optimization/Profiles/성능','A~E','090/128/143','pass order/default options/performance threshold가 미정이다.','validator 전후, conservative folds/CFG, O0/O2/Os, checked semantics 유지, fast-math off, baseline 후 threshold 승인.','초기 MIR optimize 없이 LLVM opt만, LTO 기본 사용 별도 실험.','T036'],
['D27','Hosted adapters 제외 경계','E/후속','111~113','동결표는 C FFI만인데 hosted 문서가 확정으로 표시된다.','C만 0.1 구현, .NET/JVM/Python은 후속 검토로 분리.','0.1 scope 확장 proposal로 한 adapter를 별도 승인.','T037'],
['D28','Toolchain/Target 지원표','A~E','069/093/104/128/146','우선 Target은 있으나 최소 release matrix/LLVM pin이 없다.','Windows x64 MSVC/Linux x64 GNU release 필수 후보; AArch64 experimental; backend 착수 때 Rust/LLVM/binding version pin과 clean-machine 검증.','한 host target만 first preview, Linux-first release를 별도 승인.','T038'],
['D29','버전/호환성 정책','전 Stage','007/129/146/147','세부 compatibility 기준과 bug fix 예외 절차가 없다.','언어/compiler/runtime ABI/schema 독립 version, patch 의미 유지 원칙, bug-fix impact/migration, incompatible artifact rebuild.','0.1 동안 매 release 언어 breaking 허용하되 명시 edition 지원.','T039'],
['D30','Resource budgets','전 Stage','017/065/079/087/141/145','깊은 입력/특수화/query budgets 수치와 failure class가 미정이다.','모든 budget를 config/toolchain 기록, user limit diagnostic과 ICE 구분, corpus 측정 후 수치 승인.','제한 없는 iterative 알고리즘 가능한 곳 우선, IDE/batch 별도 default.','T040'],
];

export function decisionMarkdown() {
  let text = '# Nova 0.1 사양 결정 기록 — Accepted 5 / Draft 25\n\n';
  text += '작성일 2026-10-03. **D01~D05는 2026-10-03 사용자 승인으로 Accepted이며, D06~D30은 Draft다.** ' +
    '승인 기록: 사용자 답변 “D01~D05 승인하고 Lexer 진행”. 적용 상세는 [승인 Lexer 기준](ACCEPTED_LEXER.md). ' +
    'Parser 전체/타입/소유권/Backend의 미승인 제안을 함께 승인한 것으로 해석하지 않는다.\n\n';
  text += '2026-10-04 사용자 답변 “P01 승인하고 Stage A Parser 구현 진행”으로 ' +
    '[Stage A Parser P01](PARSER_STAGE_A_PROPOSAL.md)과 [전용 EBNF](GRAMMAR_STAGE_A.ebnf)를 승인했다. ' +
    '해당 구문/복구 subset에만 적용하며 아래 D06~D30 전체 정책은 Draft를 유지한다.\n\n';
  text += '2026-10-04 사용자 답변 “P02 승인하고 이름·타입 검사까지 진행”으로 ' +
    '[Stage A 의미 계약 P02](SEMANTICS_STAGE_A_PROPOSAL.md)를 승인했다. ' +
    'Int32/Bool/String/Unit, 단일 파일 scope/call/return/print typing subset에만 적용한다. ' +
    '숫자 승격·runtime/ABI·미래 Stage와 D06~D30 전체 정책은 Draft다.\n\n';
  text += '2026-10-04 사용자 “P09 승인하고 float 구현 진행”으로 [float 최소 계약 P09](FLOAT_STAGE_B_PROPOSAL.md)을 승인했다. ' +
    '[구현 기록](FLOAT_IMPLEMENTATION.md)과 MANIFEST의 accepted_proposals가 해당 subset에 우선한다. ' +
    '숫자 cast는 P10, float remainder/math API 및 전체 D07은 Draft를 유지한다.\n\n';
  text += '2026-10-05 사용자 “P10 승인하고 숫자 cast 구현 진행”으로 [명시적 숫자 cast P10](CAST_STAGE_B_PROPOSAL.md)을 승인했다. ' +
    '[구현 기록](CAST_IMPLEMENTATION.md)과 [postfix as 전용 EBNF](GRAMMAR_STAGE_B_CAST.ebnf)를 따른다. ' +
    '숫자 10종의 checked 변환 subset에만 적용하며 Bool/Char/unsafe cast와 전체 D07은 Draft다.\n\n';
  text += '2026-10-05 사용자 “P12 승인하고 Copy struct 구현 진행”으로 [P12 Copy struct 최소 계약](STRUCT_STAGE_B_PROPOSAL.md)을 승인했다. [구현 기록](STRUCT_IMPLEMENTATION.md)을 따른다. ' +
    '생성·type import·Copy field·const·private layout/ABI와 제한의 subset이며 전체 D06/D10/D12/D16/D30은 계속 Draft다.\n\n';
  text += '2026-10-07 사용자 “P13 승인하고 Copy Tuple 구현 진행”으로 [P13 Copy Tuple 최소 계약](TUPLE_STAGE_B_PROPOSAL.md)을 승인했다. [구현 기록](TUPLE_IMPLEMENTATION.md)을 따른다. Structural identity·numeric selector subspan·혼합 가변 경로·const·private ABI와 자원 한도 subset에만 적용하며 전체 D09/D10/D12/D16은 계속 Draft다.\n\n';
  text += '2026-10-07 사용자 “P14 승인하고 Copy Enum·match 구현 진행”으로 [P14 Copy Enum·statement match 최소 계약](ENUM_STAGE_B_PROPOSAL.md)을 승인했다. [구현 기록](ENUM_IMPLEMENTATION.md)을 따른다. Qualified variant·Copy binder·Enum/Bool coverage·const 생성·private tagged ABI와 자원 한도 subset이며 전체 D06/D08/D09/D10/D12/D16/D25/D30 승인이 아니다.\n\n';
  text += '2026-10-07 사용자 “승인 할테니 다음 개발 작업 진행해줘” 답변으로 검토한 [P15 Copy Option·Result·nullable 최소 계약](OPTION_RESULT_STAGE_B_PROPOSAL.md)을 승인했다. [구현 기록](OPTION_RESULT_IMPLEMENTATION.md)을 따른다. Builtin family·T?·문맥/none·Copy match·const·private ABI와 한도 subset이며 try·Move/Drop·사용자 Generic·전체 D06/D08/D09/D10/D12/D15/D16/D23/D25/D30 승인은 아니다.\n\n';
  text += '2026-10-07 사용자 “P16 승인하고 Copy try 구현 진행” 답변으로 [P16 Copy try·Result 오류 전파 최소 계약](TRY_STAGE_B_PROPOSAL.md)을 승인했다. Prefix 결합·operand 문맥 격리·정확한 E·Copy snapshot·Error 조기 반환·const N3201·Source/CFG 검증 subset을 구현했다. [51-production EBNF](GRAMMAR_STAGE_B_TRY.ebnf)·[수용 fixture](try-proposal-fixtures/README.md)·[구현 기록](TRY_IMPLEMENTATION.md)을 따른다. Option try·error conversion·String/Move payload·일반 Drop와 전체 D08/D09/D10/D12/D16/D23/D25 승인은 아니다.\n\n';
  text += '2026-10-07 사용자 “P17 승인하고 이름 인수 구현 진행” 답변으로 [P17 함수 이름 인수](NAMED_ARGUMENTS_STAGE_B_PROPOSAL.md)를 승인했다. [52-production EBNF](GRAMMAR_STAGE_B_NAMED_ARGUMENTS.ebnf)·[수용 fixture](named-arguments-proposal-fixtures/README.md)·[구현 기록](NAMED_ARGUMENTS_IMPLEMENTATION.md)을 따른다. parameter 이름 대응·소스 순서 snapshot 후 선언 순서 전달·mapped 타입·try effect·Source/MIR 검증 subset을 구현했다. 기본 인수·overload·named constructor와 전체 D11/D16/D25/D30 승인이 아니다.\n\n';
  text += '2026-10-07 사용자 “P18 승인하고 함수 기본 인수 구현 진행” 답변으로 [P18 상수 표현식 함수 기본 인수](DEFAULT_ARGUMENTS_STAGE_B_PROPOSAL.md)를 승인했다. [52-production EBNF](GRAMMAR_STAGE_B_DEFAULT_ARGUMENTS.ebnf)·[수용 fixture](default-arguments-proposal-fixtures/README.md)·[구현 기록](DEFAULT_ARGUMENTS_IMPLEMENTATION.md)을 따른다. declaration-module scope·checked compile-time default·생략 인수 대응·제공 인수 평가 후 caller materialization·Source/MIR 검증 subset을 구현했다. runtime/parameter 의존 default·overload·named constructor와 전체 D09/D11/D16/D25/D30 승인이 아니다.\n\n';
  text += '2026-10-08 사용자 “P19 승인하고 loop·정수 범위 for 구현 진행” 답변을 승인 기록으로 반영했다. [P19 loop·정수 범위 for](RANGE_LOOP_STAGE_B_PROPOSAL.md)·[54-production EBNF](GRAMMAR_STAGE_B_RANGE_LOOP.ebnf)·[수용 fixture](range-loop-proposal-fixtures/README.md)·[구현 기록](RANGE_LOOP_IMPLEMENTATION.md)을 따른다. integer peer/common type·불변 binder·단일 bound 평가·inclusive 최댓값 종료·mixed jump/try·보수적 return·Source/CFG 검증 subset을 구현했다. Array/iterable·Range 값·step/descending·일반 Move/Drop와 전체 D08/D10/D12/D16/D23/D25 승인은 아니다.\n\n';
  text += '2026-10-08 사용자 “P20 승인하고 Copy Option exists 구현 진행” 답변으로 [P20 Copy Option postfix exists](EXISTS_STAGE_B_PROPOSAL.md)·[54-production EBNF](GRAMMAR_STAGE_B_EXISTS.ebnf)·[수용 fixture](exists-proposal-fixtures/README.md)를 승인했다. [구현 기록](EXISTS_IMPLEMENTATION.md)을 따른다. Bool·const/default·단일 평가·기존 END·Source/MIR 검증 subset을 구현했다. P01~P19와 원본을 보존하며 Result exists·flow narrowing·Move/Drop·Array와 전체 D08/D09/D10/D16/D23/D25/D30 승인은 아니다.\n\n';
  text += '## 공통 승인 계약\n\n원본 Canonical 변경은 제안하지 않는다. 제안은 구체 정의 누락/충돌을 보완한다. ' +
    '최종 승인 시 실제 승인 날짜/증거/선택 대안을 기록하고 영향 NOVA 원본·grammar·fixture를 동시 갱신한다. ' +
    'Backward compatibility는 아래 각 항목에 공통으로: 현재 미완성 compiler에 지원이 없더라도 새로운 ' +
    '표기/의미를 공식 API로 동결하는 변경이다. 기존 확정 의미와 충돌하면 기존 의미가 우선이며 재제안한다.\n\n';
  for (const [id,title,stage,docs,gap,proposal,alternative,test] of decisions) {
    text += `## ${id} — ${title}\n\n` +
      `- 상태/승인: ${Number(id.slice(1)) <= 5 ? 'Accepted / 2026-10-03 사용자 승인' : 'Draft / 미승인'}. Stage: ${stage}.\n` +
      `- 관련 문서: NOVA-${docs}.\n` +
      `- 현재 사양과 발견된 문제: ${gap}\n` +
      `- 제안 변경: ${proposal}\n` +
      `- 변경 이유: 구현과 테스트가 같은 결과를 판정하도록 모호한 경계를 하나의 계약으로 정의한다.\n` +
      `- 영향 범위: 관련 언어/Grammar, Token/AST/HIR, semantic 분석 또는 Runtime/Tooling, conformance fixture. Backend가 의미를 추정하지 못하도록 검증한다.\n` +
      `- Backward Compatibility: 기존 Canonical 보존. 공통 승인 계약 적용; 새 상세를 구현 전 동결하고 기존 예제 및 negative corpus를 비교한다.\n` +
      `- 대안: ${alternative}\n` +
      `- 수용 사례: [${test}](CONFORMANCE.md).\n\n`;
  }
  return text.trimEnd() + '\n';
}
