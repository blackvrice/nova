# Stage A HIR·이름·타입 검사 착수안 — P02

작성일: 2026-10-04. 상태: **Accepted / 2026-10-04 사용자 승인**.
승인 근거: 사용자 답변 “P02 승인하고 이름·타입 검사까지 진행”.
기존 D01~D05와 [P01](PARSER_STAGE_A_PROPOSAL.md)은 유지한다.
이 안은 D06/D07/D08/D11/D23/D25의 아래 최소 부분만 대상으로 하며 전체 승인이 아니다.

## Specification Change Proposal

- 관련 문서: NOVA-019/024/025/026/035/043/044/074/075/076/077/136.
- 현재 사양: 가장 가까운 Scope, 선언 수집 후 이름 해석, 안정적 ID, Unit 정규화,
  resolution/type side table, 양방향 타입 검사, ErrorType 연쇄 오류 억제가 확정되어 있다.
- 발견된 문제: 지역 선언 시점·중복, Stage A literal 범위, 조건/return 검사와 print signature가
  상세하게 정의되지 않았다. P01 승인은 구문·복구이며 타입·실행 의미의 승인이 아니다.
- 제안 변경: 아래 Stage A 단일 파일 의미 계약을 채택한다.
- 변경 이유: Hello Nova와 함수·분기를 정적으로 검사하고 잘못된 프로그램의 후속 codegen을 차단한다.
- 영향 범위: HIR Lowering, name resolver, type table/checker, frontend pass/fail harness.
- Backward Compatibility: 기존 Lexer/Parser 구문은 유지한다. Parser-pass였던 일부 프로그램이
  아래 의미 오류로 거부된다. Primitive alias와 기존 Canonical 의미는 바꾸지 않는다.
- 대안: HIR Lowering만 먼저 완료하고 의미 검사는 다음 단계로 미룬다.

## 지원 의미 계약

1. Stage A 타입은 `int`/`int32` → Int32, `bool`, `string`, `void`/`()`/생략 반환 → Unit이다.
   다른 공식 Primitive는 현재 Stage 미지원 진단, 알 수 없는 타입은 undefined-name 진단이다.
   일반 타입 승격·float·Generic·ownership는 뒤 Stage로 남긴다.
2. Integer Literal의 기본/기대 타입은 Int32. decimal/base/underscore 철자를 해석하며
   [-2147483648, 2147483647] 범위를 검사한다. 직접 unary minus의 magnitude를 함께 검사해
   `-2147483648`을 허용한다. `2147483648`과 `-(2147483648)`은 range 오류다.
   host integer parse overflow도 사용자 range 진단이며 compiler panic이 아니다.
3. 함수는 파일 전체에서 forward call/재귀를 허용한다. 동일 파일의 같은 함수 이름은 Stage A에서
   중복 오류다. Overload는 후속 Stage이며 반환 타입만으로 overload 불허라는 Canonical을 유지한다.
   지역 값은 초기값 검사 후 이름을 등록한다. 같은 Scope 중복은 오류, 안쪽 Scope shadow는 허용한다.
   parameter와 함수 최외곽 body는 같은 Scope, 각 if/else block은 별도 Scope다.
4. 인수 개수와 각 인수·return 타입은 정확히 일치해야 한다. let annotation은 initializer를 검사하고,
   생략 시 initializer 타입을 사용한다. 초기값 없는 let은 P01대로 syntax 오류다.
   함수 참조는 직접 call에만 사용할 수 있다. 함수 값/closure/default/named arguments는 미지원이다.
5. 산술 `+ - * / %`와 순서 비교 `< <= > >=`는 Int32, `+ -` prefix는 Int32, `!`는 Bool.
   `== !=`는 같은 Int32 또는 같은 Bool. `&& ||`와 if 조건은 Bool이며 logical HIR을 보존한다.
   String/Unit 비교·String concat·자동 String 변환은 이번 subset에 없다.
   인수/operand는 source order, logical short circuit을 뒤 MIR에서 유지해야 한다.
   이 단계는 실행·constant folding을 하지 않으며 arithmetic runtime overflow 정책은 별도 동결한다.
6. 반환 타입이 Unit이면 bare return/Unit 값을 허용한다. non-Unit 함수는 모든 경로에서 같은 타입을
   return해야 한다. if/else 양쪽 반환은 인정, else 없는 if는 fallthrough 가능으로 처리한다.
   unreachable code도 이름/타입 오류를 검사한다. if는 statement이며 branch value를 만들지 않는다.
7. prelude의 builtin `print(string) -> Unit`을 제공한다. 사용자 함수/지역 값은 가까운 Scope에서
   print를 shadow할 수 있다. overload/implicit stringify는 제공하지 않는다.
   String 보간에는 Int32/Bool/String만 허용한다. String escape/brace decode는 D04를 따른다.
   실제 출력 newline/formatting/I/O failure/ABI는 D18/D23/Backend 단계에서 별도 동결한다.
8. 이번 검사 API는 fragment/file frontend 검사다. main 존재와 entry signature는 강제하지 않는다.
   실행 파일을 만드는 단계에서 D19 entry 계약을 승인·구현한다. Modules, visibility, MIR,
   LLVM, `nova check/run`, ownership checker는 이번 작업에 포함하지 않는다.

## 구현·진단·검증

- nova-hir는 AST와 분리된 flat arena, source origin, SymbolId와 Unit 정규화를 제공한다.
- nova-resolve는 ScopeTree/DefId/DefinitionRegistry/ResolutionMap, nova-types는 고정 Stage A
  TypeId/interner, nova-typecheck는 TypeTable/call/return 검사와 diagnostics를 제공한다.
- N2001/N2002, N2101/N2102, N2201, N3001/N3002/N3003 및 기존 N1102를 필요한 부분만 채택한다.
  primary/secondary Span을 유지하고 ErrorType에서 파생된 추가 오류는 억제한다.
- 오류 HIR/TypeTable을 성공 Codegen 입력으로 사용하지 않는다. harness는 upstream Lexer/Parser 오류도
  검사한다. malformed internal AST/HIR는 별도 API 오류이며 사용자 의미 오류와 구분한다.
- HIR normalization snapshot, forward/recursive call, lexical Scope/duplicate/shadow, int MIN/MAX,
  initializer/call/return/condition 오류, return-path 검사, invalid syntax 차단, 결정성·대형 입력을 검증한다.
- cargo fmt/clippy/test/check 및 문서 검증 후 GitHub에 반영한다. frontend-pass는 Native 실행 증거가 아니다.

## 승인 질문

P02의 Stage A 의미 계약은 승인되어 이 단계에 적용한다. 나머지 의미·runtime 정책은 Draft다.
