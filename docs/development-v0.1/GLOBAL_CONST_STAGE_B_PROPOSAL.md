# Stage B 단일 파일 전역 const·의존성 평가 착수안 — P06

작성일: 2026-10-04. 상태: **Draft / 사용자 승인 대기**.
이 문서는 검토용 제안이며 현재 Compiler에 적용하지 않았다.
[P05 함수 내부 const](CONST_STAGE_B_PROPOSAL.md)의 값 타입·연산·checked 평가·예산을 재사용한다.
[P01](PARSER_STAGE_A_PROPOSAL.md)~[P04](CONTROL_STAGE_B_PROPOSAL.md)의 승인 범위도 유지한다.
전체 D06/D09/Stage B, module·다중 파일 또는 const function의 승인안은 아니다.

## Specification Change Proposal

- 관련 문서: [원본 NOVA-019 scope](../02_Names_Modules/NOVA-019_이름_해석_Scope_사양서.md),
  [상수 표현식 NOVA-032](../03_Types_Declarations/NOVA-032_상수_표현식_const_평가_사양서.md),
  [Const Engine NOVA-087](../08_MIR_Middleend/NOVA-087_Constant_Evaluation_Engine_사양서.md),
  [D06/D09](DECISIONS.md), [P05 구현 기록](CONST_IMPLEMENTATION.md), [로드맵](ROADMAP.md).
- 현재 사양: Stage B에 const가 포함되며 상수 평가는 컴파일 처리 순서에 독립적이어야 한다.
  P05는 함수 내부 const만 구현했고 top-level const는 N1102다. D09 보완 초안에는
  dependency cycle 진단 방향이 있지만 전역 선언 수집·forward 참조·순환 경계는 미동결이다.
- 발견된 문제: 전역 상수를 local binding처럼 순서대로 등록하면 뒤의 선언을 볼 수 없고,
  function body 검사 순서에 따라 결과가 달라진다. 타입 annotation만으로는 순환을 해결할 수 없다.
- 제안 변경: 단일 파일 top-level const를 먼저 수집하고 정적 의존성 graph를 검사한다.
  acyclic dependency를 먼저 타입 검사·평가한 뒤 모든 function body를 검사한다.
  순환은 N3202로 거부하며 실제 계산의 short-circuit와 정적 의존성 검사를 구분한다.
- 변경 이유: P05 ConstValue/evaluator를 확장하면서 module·새 primitive·runtime global storage 없이
  forward 참조와 안정적인 순환 진단을 검증할 수 있다.
- 영향 범위: [P06 전용 EBNF](GRAMMAR_STAGE_B_GLOBAL_CONST.ebnf), Parser/AST/HIR root item,
  Resolver의 전역 선언 종류와 scope, TypeChecker의 dependency/type/value 처리,
  MIR global name의 Constant lowering, API 대응 검증, CLI/Native tests와 문서.
  새 Lexer token·외부 dependency·Runtime ABI·startup initializer는 필요하지 않다.
- Backward Compatibility: 현재 승인된 Stage A/P04/P05 프로그램의 의미를 유지한다.
  기존에 unsupported였던 top-level const만 추가로 수용한다. local const의 선언 순서·shadow 정책은 유지한다.
- 대안: 전역 const도 source-order 참조만 허용하면 구현량은 줄지만 순환 검증을 뒤로 미루고
  function body 위치와 전역 선언 가시성의 추가 규칙이 필요하다. module까지 동시에 구현하면
  파일 경로·import·visibility 상세를 함께 동결해야 하므로 이번 범위에서 분리한다.

## 선언·scope·이름 계약

1. top-level에는 기존 function과 `const name [: Type] = expression`을 섞어 선언할 수 있다.
   초기값은 필수이며 기존 normalized END/선택 세미콜론을 사용한다.
   top-level let/var·expression statement·runtime initializer는 계속 N1102다.
2. builtin print와 모든 top-level function/const를 단일 root value namespace에 먼저 수집한다.
   전역 const의 initializer와 모든 function body는 파일 내 위치와 무관하게 모든 전역 const를 참조할 수 있다.
   전역 initializer에서 parameter/local binding은 보이지 않는다.
3. 전역 const/function/print의 같은 이름 충돌은 N2002다. 뒤 선언 name Span을 primary,
   앞 선언을 secondary로 삼고 builtin에는 기존처럼 source declaration Span을 만들지 않는다.
   오류 복구의 namespace lookup은 최초 선언을 유지하며 중복 선언도 성공으로 처리하지 않는다.
4. local parameter/let/var/const는 기존 P02 lexical shadow 규칙을 따른다.
   local initializer는 local declaration 등록 전 해석하므로 같은 이름의 전역 const가 있으면 이를 참조한다.
   `const X = 2; func f(){ const X = X + 1 }`의 local X는 3이다.
   전역이 없는 local forward/self 참조는 기존 N2001이다.
5. 전역 const에 direct-name 대입하면 N3004와 target name/전역 선언 Span을 제공한다.
   전역 이름을 shadow한 local var 대입은 허용한다. 전역 저장소·주소·수정 API는 제공하지 않는다.
6. `nova check`는 main이 없는 전역 const 전용 fragment도 검사한다.
   build/run의 main 요구·exit/output 보호와 단일 파일 CLI 계약은 P03을 유지한다.

## 타입·허용 표현식·상수값

- 전역 const는 P05의 Int32/Bool/String/Unit과 annotation/추론, literal/group/unary/binary,
  const name만 허용한다. 새 primitive·승격·cast·aggregate·보간·사용자 함수 실행은 추가하지 않는다.
- global name은 성공한 전역 ConstValue를 사용한다. global initializer가 local 값을 참조하는 방식은 없다.
  local const는 앞서 선언된 local const뿐 아니라 파일 어느 위치의 전역 const도 사용할 수 있다.
- 함수 호출/print/보간과 let/var/parameter 값의 const 사용은 P05 N3201이다.
  함수 값 자체는 기존 N1102, 미정의 이름은 N2001, 일반 타입/literal 오류는 N2101/N2102다.
  acyclic initializer의 타입·literal 오류가 있으면 파생 평가 오류를 억제하는 P05 정책을 유지한다.
- dependency의 타입과 값을 확정한 뒤 해당 initializer를 검사한다. 명시 annotation이 있어도
  dependency cycle을 허용하지 않는다. 성공한 모든 전역 값은 function body 검사 전에 준비한다.
- checked Int32 산술·실패 N3201·실제 Bool short-circuit·String UTF-8/NUL·Unit 의미는 P05와 같다.
  `const SAFE = false && (1 / 0 == 0)`은 false로 성공한다.
- 참조되지 않은 전역 const도 모두 검사·평가한다. `const BAD = 1 / 0`이 있으면
  main이나 다른 const가 BAD를 실행하지 않아도 파일 검사 전체가 실패한다.
- 기존 P05 local const의 unreachable/while false body 검사와 initializer-before-declaration은 유지한다.
  실패 dependency는 ErrorType/실패 또는 upstream-invalid 상태로 전파하고 후속 참조의 중복 오류를 억제한다.

## 정적 dependency와 순환 계약

1. 전역 initializer에서 전역 const로 해석된 **모든 name reference**를 dependency edge로 수집한다.
   logical RHS를 실행하지 않아도 graph에 포함하며 각 edge에 원본 reference Span을 보존한다.
   한 initializer의 같은 dependency가 반복되면 graph 탐색에는 최초 source-order reference를 사용한다.
   function 선언/parameter/local binding은 이 graph의 node가 아니다.
2. self edge 또는 둘 이상 const가 서로 의존하는 cyclic strongly connected component(SCC)는 N3202다.
   `const A = A`와 `const A = false && A`는 모두 실패한다.
   `const A = false && B; const B = A`도 실제 Bool 계산과 무관하게 실패한다.
   skipped RHS의 구조·타입을 검사하는 P05와 같이 정적 graph에서 순환을 숨기지 않는다.
3. cyclic SCC당 N3202 하나를 보고한다. component 순서는 가장 앞선 선언의 source-order로 정한다.
   component 내부에서는 가장 앞선 선언부터 source-order edge를 따라 DFS한 첫 back edge로
   cycle path를 선택한다. closing reference가 primary이며 path의 선언과 나머지 reference가 secondary다.
   note에는 `A -> B -> A` 같은 실제 cycle chain을 남긴다. 한 SCC의 모든 단순 cycle을 열거하지 않는다.
4. unresolved reference·중복 등 선행 이름 오류가 있는 initializer는 성공 node로 취급하지 않는다.
   유효하게 해석된 dependency graph의 순환 검사는 initializer 값의 추론·평가 전에 수행한다.
   cyclic component와 그 dependents의 파생 inference/평가 오류는 억제한다.
   다른 독립 선언의 오류는 계속 보고한다. annotated cycle도 동일하게 거부한다.
5. acyclic component는 dependency-first로 검사·평가한다. 독립 root와 sibling dependency의
   처리 순서는 declaration/source-order로 정한다. source 순서를 바꾸어도 같은 acyclic 식의 값은 같고,
   동일 입력의 DefId/dump/진단은 반복 실행에서 결정적이어야 한다.
   DefId를 파일 간 영구 ID나 재배치 후 동일 ID로 약속하지 않는다.
6. graph 탐색·dependency ordering·상수 평가에는 explicit work stack을 사용한다.
   긴 dependency chain이 host recursion overflow를 일으키지 않아야 한다.
   실제 parallel compilation/query/cache와 cross-module cycle 처리는 이번에 구현하지 않는다.

## 예산·진단·Codegen 차단

- P05의 **initializer당 10,000 HIR expression nodes** 제한을 전역에도 동일하게 적용한다.
  root/group/skipped RHS를 세고 10,001번째 preorder node의 Span에 N3202를 제공한다.
  전역 const name은 이미 계산한 값 참조 1 node이며 참조한 initializer의 node 수를 다시 더하지 않는다.
  local/global initializer마다 예산을 새로 시작한다. parser nesting 128은 별도다.
- N3202의 cycle과 node budget은 message·secondary·note로 구별한다. cycle diagnostic에는 chain,
  budget에는 declaration과 10,000 limit note를 제공한다. 새 diagnostic code는 추가하지 않는다.
- 제한은 initializer node 수 계약이다. dependency graph 전체 node 수나 파일 전체 memory/time,
  CLI budget override·전역 cache를 새로 약속하지 않는다.
- cyclic/failed/invalid/Pending 상태의 값을 성공한 MIR 또는 CodegenUnit에 넘기지 않는다.
  해석된 전역 선언 종류·scope·reference·type·ConstValue와 public table의 모듈 대응 관계를 검증한다.

| 상황 | 진단 | 위치/복구 |
|---|---|---|
| 전역 const 초기값/END 누락 | N1101 | parser 예상 위치; 다음 const/function 선언에서 복구 |
| top-level let/var/기타 미지원 | N1102 | 미지원 construct |
| 미정의/중복/타입/literal 오류 | N2001/N2002/N2101/N2102 | 기존 P02 정책 |
| 전역 const 대입 | N3004 | target name / 전역 선언 |
| 불허 표현식/checked 산술 실패 | N3201 | expression / const 선언 |
| 정적 dependency cycle | N3202 | closing reference / 실제 chain의 선언·reference |
| initializer 예산 초과 | N3202 | 10,001번째 node / const 선언·limit note |

## MIR·Native 계약

- top-level const 자체는 runtime body·startup 작업·mutable global memory를 만들지 않는다.
  function 내 global const name 사용은 성공값을 기존 MIR Constant operand로 materialize한다.
  local const는 기존 P05 Constant/lexical Place lowering을 유지한다.
- global initializer의 arithmetic/call/format은 runtime에서 실행하지 않는다.
  일반 let/var initializer가 전역 const와 산술을 하면 그 산술은 기존 P03 runtime checked 연산이다.
- source table에는 전역 선언/initializer의 SourceOrigin·byte Span을 보존한다.
  global name operand 사용의 SourceInfo는 해당 reference를 가리키며 선언 provenance도 분석 table에 남긴다.
- Int32/Bool/Unit/String constant는 기존 target-independent 값과 LLVM Adapter를 사용한다.
  String literal은 기존 immutable static bytes 표현이며 새로운 global ABI나 ownership/Drop 정책을 정하지 않는다.
- CLI check/build/run은 전역 const 오류를 LLVM 탐색/호출보다 먼저 source exit 1로 거부하고 산출물을 만들지 않는다.
  Windows x64 Native O0/O2에서 값·UTF-8 출력·exit가 같아야 한다. Linux x64 IR 검증은 기존 범위다.

## 목표 프로그램·수용 계획

아래는 **승인 후 구현할 수용 예제**다. 현재 Compiler의 실행 성공을 주장하지 않는다.

```nova
const LIMIT: int32 = BASE * 2

func main() {
    const title = TITLE
    var index = 0
    var sum = 0
    while index < LIMIT {
        sum = sum + index
        index = index + 1
    }
    print("{title}: {sum}, skipped={SAFE}")
}

const BASE = 3
const TITLE = "합계"
const SAFE = false && (1 / 0 == 0)
```

예상 stdout UTF-8 bytes: `합계: 15, skipped=false\n`, exit 0.

1. Parser/HIR: mixed root item/END/annotation/초기값, root에서 let/var 거부,
   잘못된 const 이후 다음 const/function 복구, truncated prefixes와 malformed AST 차단.
2. Resolver/type: forward global dependency·function-before-global·네 타입/추론,
   global/function/builtin 충돌·undefined·local shadow/initializer lookup·불변 대입의 정확한 code/Span.
3. dependency: self/two-node/긴 cycle·annotated cycle·skipped RHS cycle, 다중 독립 cyclic SCC,
   chain secondary와 한 SCC 한 진단·실패 dependent 억제·독립 오류 보존·긴 acyclic chain/determinism.
4. const: P05 모든 checked 연산·short-circuit permission/type/evaluation 경계,
   global node 10,000/10,001·cached dependency/reset·unused 실패·Unicode/NUL/Unit과 local/global 조합.
5. MIR/LLVM: initializer runtime 연산·startup/global storage 없음, global read Constant와 SourceInfo,
   local let/var runtime 산술 보존, failed value·변조 scope/종류/type/value/table/누락 차단과 deterministic IR.
6. CLI/Native: 정상 check의 no-LLVM/no-output, cycle·budget·산술 오류의 no-codegen/source exit 1,
   main 없는 fragment check와 기존 entry 오류, Windows x64 O0/O2 예제·경계값·shadow·String/Unit 실행.
7. 기존 Stage A/P04/P05 회귀와 Cargo fmt/clippy/workspace test/all-features check,
   standalone Runtime rustfmt 및 문서 기계 검증을 수행한다.

## 승인 경계

승인 대상은 단일 파일 전역 const·forward visibility·dependency-first 타입/값 평가,
정적 skipped-edge 포함 cycle N3202·기존 initializer budget·불변 대입과 전용 EBNF다.
전역 let/var/runtime initializer, module/import/visibility·다중 파일, const function,
새 primitive/승격/aggregate, 보간 상수화·optimizer/query/cache, ownership/Drop 및 Linux Native는 후속이다.
승인 전에는 accepted ledger와 Compiler/P05 grammar를 변경하지 않는다.
보완팩의 top-level const-divzero Draft fixture는 승인 후 P06 구현 시험에서 별도로 다루며
기존 fixture의 전체 D09 수용 상태를 자동으로 Accepted로 바꾸지 않는다.
