# Stage B 함수 내부 const·상수 평가 착수안 — P05

작성일: 2026-10-04. 상태: **Accepted / 2026-10-04 사용자 승인**.
승인 근거: 사용자 답변 “P05 승인하고 const 구현 진행”.
이 최소 계약을 구현에 적용했다. 현재 API와 실제 검증 증거는 [P05 구현 기록](CONST_IMPLEMENTATION.md)을 따른다.
[P01](PARSER_STAGE_A_PROPOSAL.md)/[P02](SEMANTICS_STAGE_A_PROPOSAL.md)/
[P03](NATIVE_STAGE_A_PROPOSAL.md)/[P04](CONTROL_STAGE_B_PROPOSAL.md)의 기존 subset을 유지한다.
D09의 아래 최소 부분만 제안하며 전체 Stage B/D09 승인이 아니다.

## Specification Change Proposal

- 관련 문서: [원본 상수 표현식](../03_Types_Declarations/NOVA-032_상수_표현식_const_평가_사양서.md),
  [원본 Const Engine](../08_MIR_Middleend/NOVA-087_Constant_Evaluation_Engine_사양서.md),
  [D09](DECISIONS.md), [로드맵](ROADMAP.md).
- 현재 사양: Stage B에 const가 포함되고 고정 폭 primitive와 target semantics를 따라야 한다.
  현재 let은 불변 runtime binding이다. const의 허용 표현식·실패·평가량 제한은 미동결이다.
- 발견된 문제: 불변 let을 compile-time 상수로 추정할 수 없고,
  LLVM folding이나 host Rust overflow 정책이 const의 언어 의미를 대신 결정해서도 안 된다.
- 제안 변경: 함수 내부 초기값 필수 const, 현재 네 값 타입의 제한된 표현식,
  compile-time checked 평가와 아래 고정 node budget을 채택한다.
  구문은 [P05 전용 EBNF](GRAMMAR_STAGE_B_CONST.ebnf)를 따른다.
- 변경 이유: 기존 frontend/verified MIR 위에서 상수를 먼저 구현하고 aggregate/module 의존은 분리한다.
- 영향 범위: AST/Parser/HIR의 binding 분류, resolver/typed const value table,
  const evaluator, MIR constant lowering과 upstream gate, CLI/Native harness·문서.
  Lexer/END, 기존 runtime arithmetic/ABI와 새로운 package 의존은 변경하지 않는다.
- Backward Compatibility: 기존 Stage A/P04 프로그램의 동작을 유지한다.
  지금까지 unsupported인 함수 내부 const만 추가로 허용하며 let을 const로 바꾸지 않는다.
- 대안: 전역 상수·forward dependency/cycle·const function·aggregate를 함께 동결한다.
  이번에는 기존 lexical scope 안의 최소 표현식부터 구현한다.

## 제안하는 선언·이름·타입 계약

1. **선언**: `const name [: Type] = expression`을 함수 body와 if/while body에서 허용한다.
   초기값은 필수다. 타입 추론/annotation은 P02의 Int32/Bool/String/Unit 규칙을 따른다.
   const는 불변이며 대입 시 P04 N3004, declaration Span을 secondary로 제공한다.
2. **Scope**: initializer는 declaration 등록 전에 해석한다. 앞서 선언된 const만 initializer에서
   참조할 수 있다. 바깥 const를 같은 이름으로 shadow하면 initializer는 그 바깥 const를 참조한다.
   같은 scope 중복은 N2002, 뒤의 const나 바깥 const 없는 self-reference는 N2001이다.
   함수 밖/top-level const는 N1102다. forward global reference/cycle resolution은 이번 범위가 아니다.
3. **시점**: 각 source const 선언은 compile-time에 평가한다. `return` 뒤나 `while false` body의
   const도 검사·평가하며 실패를 숨기지 않는다. Runtime의 반복 횟수에 따라 const 산술을 재평가하지 않는다.
   local storage에 precomputed 값을 넣는 MIR는 가능하지만 runtime 부수 효과는 없다.

## 허용 표현식과 평가 의미

| 표현식 | P05 허용 범위 |
|---|---|
| literal | P02 범위의 Int32, Bool, decoded String literal, Unit `()` |
| name | 앞서 해석·평가가 성공한 const의 값 |
| group | 허용 표현식의 괄호 |
| unary | Int32 `+ -`, Bool `!` |
| binary | 기존 Int32 `+ - * / %`, Int32 비교, Bool/Int32 `== !=`, Bool `&& ||` |

String literal은 UTF-8 bytes/NUL을 그대로 가진 immutable value다. 사용자 heap allocation이나
String 보간/concat/format은 허용하지 않는다. String name/group의 단순 복사는 허용한다.
현재 P02에서 거부하는 String/Unit equality나 다른 타입 연산은 그대로 타입 오류다.

- 허용 여부는 initializer의 **모든 표현식**을 source-order로 검사한다.
  let/var/parameter 값 참조, call(사용자 함수와 print 모두), 보간은 N3201이다.
  함수 값 자체의 사용은 기존 P02 N1102를 유지한다.
  short-circuit로 실행하지 않는 RHS도 이 구조 제한을 만족해야 한다.
  예: `false && predicate()`는 호출을 실행하지 않아도 const에서 허용하지 않는다.
- 허용 표현식의 실제 평가는 소스 순서와 기존 Bool short-circuit을 따른다.
  `false && (1 / 0 == 0)` 및 `true || (1 / 0 == 0)`은 각각 false/true로 성공한다.
  실행하지 않는 RHS의 **타입·literal 범위**는 계속 검사한다. `false && (2147483648 == 0)`은 N2102다.
- Int32 산술은 P03과 동일한 checked 의미다. overflow/zero division 또는 remainder,
  MIN/-1 division 및 remainder는 **compile-time N3201**이며 Abort 프로세스를 만들지 않는다.
  정상 division은 0 방향 truncate, remainder는 dividend 부호다. MIN literal의 기존 signed magnitude 규칙도 유지한다.
- 외부 도구/LLVM/사용자 함수를 호출해 const를 평가하지 않는다.
  host width·debug/release wrap·UB에 의존하지 않고 typed Int32 값과 checked 연산으로 결과를 정한다.
  let/var 일반 initializer의 arithmetic은 P03 Runtime checked 연산으로 남긴다.

## 계산량 제한 제안

- 한 const initializer의 HIR expression subtree는 **최대 10,000 nodes**다.
  root, group, unary/binary, literal/name 등 모든 expression node를 센다.
  short-circuit로 평가를 생략하는 subtree도 허용성 검사와 이 node 수에 포함한다.
  binding/type annotation 자체는 제외한다. const name은 이미 계산한 값 참조 한 node로 센다.
- 매 initializer마다 counter를 새로 시작한다. source-order preorder의 10,001번째 node에서
  N3202와 해당 Span, const declaration 위치 및 limit note를 보고한다.
  이 예산은 parser의 기존 nesting limit 128과 독립적이며 CLI override는 이번에 추가하지 않는다.
- 허용성 검사와 계산은 host recursion 대신 work stack으로 진행한다.
  const 참조는 저장된 값을 사용하며 사용자 함수를 실행하거나 dependency를 재귀 계산하지 않는다.
  이 제한은 initializer node 수 계약이며 파일 전체 memory/time 제한을 약속하지 않는다.

## 진단·오류 입력 차단

| 상황 | 코드 | Primary / secondary |
|---|---|---|
| literal 범위/일반 타입 불일치/미정의/중복 | N2102/N2101/N2001/N2002 | 기존 P02 규칙 |
| 불변 const에 대입 | N3004 | target name / const 선언 |
| const에서 불허한 이름·call·보간 | N3201 | 불허 표현식 / const 선언 |
| checked 산술 실패 | N3201 | 실패 operation의 expression Span / const 선언 |
| initializer node budget 초과 | N3202 | 10,001번째 node / const 선언·limit note |
| 초기값 누락/전역 const | N1101/N1102 | parser 위치 |

N3201/N3202는 이미 registry에 있으며 새 code를 추가하지 않는다.
기존 syntax/name/type 오류가 있는 initializer는 const 성공값을 만들지 않고 파생 평가 오류는 억제한다.
const 평가에 실패한 선언은 ErrorType/실패 상태로 전파하여 뒤의 참조에서 중복 실패를 생성하지 않는다.
허용성/예산 검사에서 첫 오류 하나를 보고한 뒤 해당 initializer 계산을 중단한다.
실제 평가에서도 첫 산술 실패 하나를 보고한다. 오류가 있는 const value는 MIR/CodegenUnit에 들어가지 않는다.

## 목표 프로그램과 수용 기준

아래는 P05 수용 예제다. 실제 Windows x64 O0/O2 결과는 구현 기록에 있다.

```nova
func main() {
    const limit: int32 = 3 * 2
    const title = "합계"
    const skipped = false && (1 / 0 == 0)
    var index = 0
    var sum = 0
    while index < limit {
        sum = sum + index
        index = index + 1
    }
    print("{title}: {sum}, skipped={skipped}")
}
```

예상 stdout UTF-8 bytes: `합계: 15, skipped=false\n`, exit 0.

1. Parser/HIR는 const와 let/var를 구분하고 declaration/initializer SourceOrigin·byte Span을 보존한다.
2. typed const value는 DefId와 연결한다. 계산 성공/실패/budget을 별도로 기록하고,
   public semantic table을 변조한 입력은 기존 MIR correspondence gate에서 거부한다.
3. MIR는 성공한 const initializer를 Constant로 materialize한다. Arithmetic/Call/format Runtime 평가를
   생성하지 않는지 확인하며 source origin을 잃지 않는다. 일반 let/var는 기존 lowering을 유지한다.
4. 모든 허용 타입/연산/Int32 경계, 상수 참조와 shadow/duplicate/forward, short-circuit과
   skipped RHS type·permission, unreachable const 실패, 불변 대입과 정확한 code/Span을 검사한다.
5. budget 경계/큰 flat expression/결정성/복구/변조 table와 no-codegen을 검증한다.
   CLI check/build/run의 const 실패는 missing LLVM보다 먼저 source 오류로 끝나고 산출물 없이 거부한다.
6. Windows x64 Native O0/O2에서 예제·Int32 boundary·String Unicode/NUL·loop와 const 조합을 검증하고
   기존 Stage A/P04 LLVM/Native 회귀 시험을 실행한다.
7. Cargo fmt/clippy/workspace tests/all-features check, Runtime rustfmt, 문서 validator를 통과시킨다.

## 승인 경계

승인 대상은 이 문서의 함수 내부 const/허용 표현식/compile-time 실패/10,000-node 제한과 전용 EBNF다.
전역 const·forward dependency/cycle, const function, aggregate/float·승격/module/array-size generic,
보간 상수화, const optimizer·incremental cache, ownership/Drop와 Linux Native는 후속이다.
보완팩의 top-level `const-divzero` fixture는 후속 범위의 Draft 예제이며 P05 검증으로 승인하거나 수정하지 않는다.
이 최소 계약만 accepted ledger와 구현에 적용했다. 나머지 Stage B/D09 상세는 Draft다.
