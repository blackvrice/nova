# Stage B 가변 지역 변수·반복문 착수안 — P04

작성일: 2026-10-04. 상태: **Draft / 사용자 승인 대기**.
이 문서는 구현 승인을 위한 제안이다. 현재 compiler는 아래 확장 문법을 지원하지 않는다.
[P01](PARSER_STAGE_A_PROPOSAL.md), [P02](SEMANTICS_STAGE_A_PROPOSAL.md),
[P03](NATIVE_STAGE_A_PROPOSAL.md)의 기존 subset은 유지한다.
D06/D08의 필요한 부분만 제안하며 Stage B 전체나 D06~D30 전체 승인이 아니다.

## Specification Change Proposal

- 관련 문서: [원본 제어 흐름](../04_Functions_Control/NOVA-043_if_while_for_loop_사양서.md),
  [원본 jump](../04_Functions_Control/NOVA-044_break_continue_return_사양서.md),
  [전체 문법 초안](GRAMMAR.ebnf), [로드맵](ROADMAP.md).
- 현재 사양: Stage B에 var/반복문이 포함되고 할당은 문장이다. 승인된 Stage A는
  초기값 필수 let과 if/return만 구현했다. 가변성, 대입 대상, 반복문 상세는 미동결이다.
- 발견된 문제: 지역 상태를 갱신하거나 반복할 수 없어 기본 누적·탐색 프로그램을 작성할 수 없다.
  Rust나 LLVM의 가변성·loop 규칙을 언어 규칙으로 대신 적용할 수 없다.
- 제안 변경: 아래 var/direct-name assignment/while/break/continue 최소 계약과
  [전용 문법](GRAMMAR_STAGE_B_CONTROL.ebnf)을 채택한다.
- 변경 이유: 기존 Int32/Bool/String/Unit과 verified MIR 위에서 작은 실행 가능한 Stage B 단계를 완성한다.
- 영향 범위: AST/Parser/HIR, resolver의 선언 가변성·block scope, typecheck의 대입·jump 검사,
  MIR의 loop CFG와 validation 회귀 시험, LLVM/CLI Native harness. 기존 Lexer token/END 계약은 유지한다.
- Backward Compatibility: 기존 정상 Stage A 프로그램의 출력·진단·실행 계약은 유지한다.
  지금까지 unsupported였던 아래 구문을 추가로 허용한다. let과 기본 Read parameter는 가변으로 바꾸지 않는다.
- 대안: 가변 변수 없이 while만 구현하거나, 전체 Stage B를 한 번에 동결한다.
  이번 제안은 실제 누적 프로그램을 실행할 수 있는 작은 단위를 선택한다.

## 제안하는 언어 계약

1. **선언**: `var name [: Type] = expression`은 가변 지역 변수다. 초기값은 필수이고
   type inference/annotation은 P02의 let과 동일하다. let은 불변이다. 함수 parameter는
   기본 Read이므로 대입할 수 없다. 현재 네 가지 값 타입을 모두 허용한다.
   선언은 initializer를 검사한 뒤 scope에 등록하며 동일 scope 중복과 내부 scope shadow는 P02를 따른다.
2. **대입**: `name = expression`은 Unit 문장이다. 대상은 현재 scope에서 해석되는
   가변 지역 var 하나여야 한다. 가까운 let이 바깥 var를 가리면 그 let에 대한 대입을 거부한다.
   RHS는 한 번 평가한 뒤 기존 local에 저장하고, 선언 때 정한 타입과 정확히 같아야 한다.
   Unit var에 `()` 대입도 허용한다. 대입은 표현식이 아니며 `a = b = value`,
   call/field/index/group 대상, compound assignment, increment/decrement는 지원하지 않는다.
3. **while**: `while condition { statements }`은 Unit 문장이다. condition은 Bool만 허용한다.
   첫 body 진입 전에 검사하고, 매 정상 반복 종료 및 continue 뒤에 다시 평가한다.
   false면 body를 실행하지 않고 다음 문장으로 진행한다. body는 독립 lexical scope다.
   body의 var/let initializer는 해당 선언에 도달할 때마다 실행한다. body local은 밖에서 참조할 수 없다.
4. **jump**: bare `break`는 가장 가까운 enclosing while의 다음 문장으로,
   bare `continue`는 그 while의 condition으로 이동한다. if 안에서도 같은 규칙이다.
   loop 밖 jump는 오류이며 label/value jump는 지원하지 않는다. return은 기존처럼 enclosing
   function을 종료한다. break/continue 뒤의 같은 block 문장도 이름·타입 오류는 검사한다.
5. **종료 분석**: non-Unit 함수의 reachable fallthrough는 계속 오류다.
   while은 보수적으로 fallthrough 가능으로 분석한다. `while true`나 body의 return만으로
   함수의 필수 return을 충족하지 않는다. 반복 횟수/상수 조건/termination 추론은 추가하지 않는다.
6. **실행과 한계**: P03 checked arithmetic/Abort, source-order/short-circuit, print와 String arena는 유지한다.
   String var 대입은 값의 pointer/length를 바꾸고 이전 dynamic String 저장소도 entry 종료까지 보존한다.
   반복 중 dynamic String 생성에 비례해 arena memory가 증가할 수 있다. 이 단계에서 ownership/Drop,
   매 iteration 해제, borrow checker나 memory bound를 약속하지 않는다.
7. **END와 복구**: 기존 D05 newline/semicolon/boundary 계약을 사용한다.
   break/continue 바로 다음 newline은 문장을 끝낸다. P01 기본 nesting limit 128을 반복문에도 적용하고
   malformed initializer/assignment/loop를 복구하되 다음 함수 분석을 계속한다.

## 진단 계약 제안

| 상황 | 코드 | Primary / 보조 위치 |
|---|---|---|
| 대입 대상 이름 미정의 | N2001 | target identifier |
| let/parameter/function/print 등 가변 지역 var가 아닌 이름에 대입 | **N3004 (신규 제안)** | target identifier / 선언 위치가 있으면 secondary |
| RHS 타입 불일치 | N2101 | RHS / target 선언 위치 |
| while 조건이 Bool 아님 | N3001 | condition |
| loop 밖 break/continue | N3002 | jump keyword |
| non-Unit 함수의 fallthrough | N3003 | function body / return signature |
| 초기값/필수 token 누락 | N1101 | 누락·예상 token 위치 |
| 표현식 대입, 비-name 대상, compound/label/value jump | N1102 | 미지원 construct |
| 중첩 한도 초과 | N8901 | 초과 construct |

N3004는 승인 후 diagnostic registry에 등록한다. 오류 입력은 기존처럼 CodegenUnit 생성 전에 차단한다.
ErrorType에서 비롯된 중복 타입 진단은 억제한다.

## 목표 프로그램과 예상 결과

아래는 승인 후 구현할 수용 사례이며 현재 실행 검증 결과가 아니다.

```nova
func main() {
    var index: int32 = 0
    var sum: int32 = 0
    while index < 6 {
        index = index + 1
        if index == 2 { continue }
        if index == 5 { break }
        sum = sum + index
    }
    print("sum={sum}, index={index}")
}
```

예상 stdout bytes: `sum=8, index=5\n`, exit 0. continue에서도 index 갱신은 보존한다.

## 구현·검증 순서와 완료 기준

1. AST/HIR에 가변 binding, direct-name assignment, while 및 jump를 명시적으로 표현하고
   identifier/keyword/whole-construct byte Span을 보존한다. 기존 flat arena 구조를 유지한다.
2. Resolver는 loop body scope와 declaration mutability를 기록한다. Typecheck는 정확한
   target/type/loop nesting/return flow를 검사하고 unresolved Error를 backend에 전달하지 않는다.
3. MIR은 기존 Place assignment와 CFG Goto/Branch로 condition/body/exit 및 backedge를 구성한다.
   nested loop target stack을 사용한다. break/continue/return 뒤의 코드를 실행 경로에 연결하지 않는다.
   MIR validator의 순환 CFG must-initialized fixed point를 재사용하되 회귀 사례로 검증한다.
4. Parser snapshot/recovery, shadow·불변 대입·미정의·type·jump Span compile-fail,
   zero iteration/condition 재평가/nested jumps/return/초기값 반복 실행 compile-pass를 추가한다.
   모든 실패는 toolchain 호출 전에 거부하는 CLI 시험도 추가한다.
5. Windows x64 Native O0/O2에서 누적, zero iteration, nested break/continue, 함수 return,
   Bool/String/Unit var와 반복 중 checked failure를 검증한다. stdout/exit/부수 효과 횟수를 대조한다.
   infinite loop를 실제 시험으로 실행하지 않고 종료하는 입력을 사용한다.
6. Cargo fmt/clippy/workspace tests/all-features check, runtime rustfmt,
   문서 validator와 기존 Stage A 실제 LLVM/Native 회귀 시험을 통과시킨다.

## 승인 경계

승인 대상은 이 문서의 P04 최소 계약과 전용 EBNF, N3004 등록이다.
const/지연 초기화/숫자 승격/for/loop/range/match/aggregate/module/multi-file,
Read·change·take 확장, Drop/Move/NLL, 일반 ABI와 Linux Native는 후속이다.
승인 전에는 compiler 의미 변경과 승인 ledger 등록을 수행하지 않는다.
