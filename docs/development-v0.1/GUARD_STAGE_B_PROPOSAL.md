# Stage B Copy match guard 최소 계약 — P25

작성일: 2026-10-12. 상태: **Draft / 사용자 승인 대기 / 미구현**.
P01~P24·D01~D05·Canonical·원본 148개 문서를 보존한다.
[61-production 제안 EBNF](GRAMMAR_STAGE_B_GUARD.ebnf)·[고정 수용 계획](guard-proposal-fixtures/README.md)을 함께 검토한다.
이 문서와 proposed_result는 승인 또는 Compiler/Native 실행 증거가 아니다.
[착수 준비 기록](GUARD_PREPARATION.md)에 현재 회귀 검사와 미구현 진단을 기록했다.

## Specification Change Proposal

- 관련 문서: [Canonical](CANONICAL.md), [MVP Freeze](../00_Governance/NOVA-002_Nova_0.1_MVP_기능_동결표.md),
  [Pattern](specs/NOVA-046.md), [완전성](specs/NOVA-047.md), [Lowering](specs/NOVA-048.md), [D08/D10](DECISIONS.md).
  의존 계약은 [P24 중첩 Copy 패턴](NESTED_PATTERN_STAGE_B_PROPOSAL.md), [P16 try](TRY_STAGE_B_PROPOSAL.md),
  [P19 loop](RANGE_LOOP_STAGE_B_PROPOSAL.md), [P22 Read method](METHOD_STAGE_B_PROPOSAL.md)다.
- 현재 사양: Bool/Copy sum/Tuple/Unit statement match·recursive immutable Copy binder·단일 snapshot·bounded matrix coverage를 구현했다.
  guard는 P24의 명시적 제외 범위다. 기존 Parser는 pattern 뒤 if를 N1102로 거부한다.
- 발견된 문제: payload에 대한 Bool 조건과 함수 효과를 한 arm에서 검사하려면 guard의 scope·평가 순서·false continuation·coverage 기여를 정의해야 한다.
- 제안 변경: `pattern if expression => block` 형태의 선택적 Bool guard를 추가한다. Copy pattern과 statement match만 확장한다.
- 변경 이유: 기존 Stage B match/try/loop를 연결하며 Array의 ownership/Drop 선행 조건은 유지한다.
- 영향 범위: Parser/AST/HIR/Source·Resolver/Type Checker/flow/coverage·MIR/validator·LLVM 회귀·문서/fixture.
  Lexer token/keyword/END·const/default 평가 의미·aggregate ABI/layout·scalar Runtime API는 변경하지 않는다.
- Backward Compatibility: P01~P24 유효 프로그램과 guard 없는 match의 진단/Span/출력·기존 flat 경로·기존 자원 예산을 보존한다.
  P25 guarded match 전체만 아래 분석 경로를 사용한다. 전체 D08/D10/D12/D16/D25/D30 승인이 아니다.
- 대안: pure/constant guard만 허용하거나 guard truth를 symbolically 증명한다. 이번에는 일반 Bool expression과 보수적 coverage를 제안한다.

## Grammar·END·복구

1. P24 60개 production의 match_arm 하나만 확장하고 match_guard 하나를 추가한다.
   총 **61개 production / 기존 59개 보존**이다. 기존 if keyword를 재사용하며 `when` 등 새 어휘는 없다.
2. guard는 pattern 뒤, fat arrow 앞에서 한 번만 허용한다. assignment/statement block/guard chain은 expression이 아니므로 허용하지 않는다.
   값 tuple·nested pattern·생성자 인수·함수/메서드 문법은 P24 그대로다.
3. P24 END를 그대로 사용한다. pattern과 if 사이에 END가 있으면 다른 논리 줄이므로 N1101이다.
   guard expression 내부의 괄호·binary continuation과 expression 뒤 => 앞의 newline은 기존 정규화 규칙을 따른다.
   `x if
 x > 0
 => { ... }`는 END가 삽입되지 않는 기존 token 규칙의 범위에서 가능하다. explicit semicolon은 guard 안에 넣을 수 없다.
4. AST/HIR는 arm의 pattern·선택적 guard·body를 명시적으로 구분하고 if keyword Span과 guard expression Span을 보존한다.
   누락 expression/arrow·잘린 guard는 Error node로 복구하며 다음 arm/item·prefix HIR를 결정적으로 보존한다.
   Source gate는 원 if spelling/XID 경계·pattern/guard/body 순서·=>·END와 UTF-8 byte subspan을 검사한다.

## Binding·타입·실행 의미

1. 기존 P24 root type·pattern 종류·nominal identity·Copy 제한을 사용한다. 숫자·char·String·Copy struct root 또는 새로운 pattern 문법은 추가하지 않는다.
   패턴이 맞으면 immutable Copy binder를 해당 arm scope에 준비한다. guard와 body가 같은 scope를 사용한다.
   body의 top-level let/var/const와 binder 이름 충돌은 N2002, binder 변경은 N3004, arm 밖 사용은 N2001이다.
2. guard는 Bool이어야 한다. 일반 Bool expression·함수/Read 메서드·단락 논리·exists·숫자 연산·P16 try를 사용할 수 있다.
   숨은 truthiness나 숫자→Bool 변환은 없다. guard type N2101의 primary는 guard expression 전체, secondary는 필요한 기대/선언 source다.
   이름·인수·literal 범위·private 접근·try Error identity의 기존 진단과 Source Span은 유지한다.
3. scrutinee는 한 번 평가하고 Copy snapshot한다. 각 arm은 source order로 pattern을 검사한다.
   **pattern mismatch에서는 guard를 평가하지 않는다**. 모든 constructor 검사와 ancestor tag proof가 성공한 뒤에만 guard용 Copy binder를 읽는다.
   guard는 해당 arm 시도에서 한 번 평가한다. true면 body, false면 다음 arm으로 진행한다.
   한 arm의 guard가 false여도 뒤 arm에서 같은 snapshot을 다시 검사할 수 있다. scrutinee를 재평가하지 않는다.
4. guard의 effect는 source order로 관측된다. mismatch·단락 skipped RHS·선택되지 않은 body의 effect는 없다.
   false guard의 String 임시값은 기존 P03 Runtime/함수 lifetime 규칙을 유지하며 일반 Move/loan/Drop을 추가하지 않는다.
   guard 안에는 local 대입 문장을 넣을 수 없다. guard의 호출/try/Abort를 임의로 제거하거나 앞뒤 arm으로 이동하지 않는다.
5. guard의 `try` Error는 enclosing Result 함수에서 즉시 반환한다. 뒤 guard/arm/body/후속 statement를 실행하지 않는다.
   성공 Bool(false)는 다음 arm으로 진행한다. guard 안의 checked Abort도 다음 arm으로 대체하지 않는다.
   body return/try는 enclosing 함수, body break/continue는 enclosing loop를 따른다.
6. statement match 결과는 Unit이다. guard 없는 arms만으로 완전성이 성립하고 도달 가능한 모든 body가 return/jump해야 기존 all-return/jump 결론을 낸다.
   guard expression의 상수 true·try early return만으로 missing-return 분석을 강화하지 않는다. guard 안의 return/break/continue statement를 추가하지 않는다.
   match expression·const function·runtime default는 추가하지 않는다. body의 기존 const/default 의미는 보존한다.

## 완전성·도달성·자원

1. usefulness는 **앞의 guard 없는 유효 arms의 합집합**으로 계산한다. guarded candidate 자체도 이 합집합에 완전히 덮이면 N3102다.
   앞 guarded arm은 뒤 arm의 coverage를 늘리지 않는다. 같은 pattern의 두 guarded arms는 모두 도달 가능할 수 있다.
   guard가 literal true/false·상수 이름이어도 coverage에서는 Bool 값을 추론하지 않는다. false guard를 이유로 새로운 unreachable 진단을 내지 않는다.
2. 최종 완전성도 guard 없는 arms만으로 판정한다. guarded wildcard만 있으면 N3101이다.
   pattern·guard 이름/타입/arity/binding 오류가 있으면 그 match의 파생 N3101/N3102를 억제한다. 독립 body 오류는 계속 검사한다.
   N3101 primary는 match keyword·P24 순서의 최대 8개 witness와 생략 note다.
   N3102 primary는 pattern 전체이며 guard/if/body는 포함하지 않는다. covering secondary는 guard 없는 앞 pattern만 최대 8개/source order+생략 note다.
3. guard가 하나라도 있으면 그 match 전체를 P24의 bounded constructor/default matrix 경로로 분석한다.
   canonical key·constructor/column/source order·cache-hit 0·arm usefulness와 최종 exhaustiveness/witness 공유 예산을 그대로 사용한다.
   guard truth/effect/이름은 coverage key에 넣지 않는다. guard 없는 기존 flat P14/P15 경로를 변경하지 않는다.
4. match arm 1,025·pattern node **10,000개**·matrix task **100,000개**·live canonical cells **1,000,000개**·기존 type/layout 한도를 유지한다.
   if token/guard expression/body는 source pattern node 수에 포함하지 않는다. guard expression 자체는 기존 expression nesting/AST 한도를 따른다.
   guard용 임의 새 truth-enumeration 예산을 만들지 않는다. 초과 N8901과 파생 coverage 억제는 P24와 같다.

## Checked·MIR·Native 검증

- Checked plan은 원 match/arm/pattern/guard/body HirId·if SourceInfo·canonical Bool type·binder DefIds·snapshot/projection identity를 가진다.
  mutable public tables와 별개로 원 HIR/Resolved에서 재계산하며 guard 삭제/arm 이동/소유권·타입 변조를 거부한다.
- MIR은 pattern-success → binder Copy → guard expression/branch → body 또는 next-arm 경로를 보존한다.
  false edge의 임시 local을 뒤 arm binder로 재사용하지 않는다. inactive payload read는 모든 ancestor active tag에 지배돼야 한다.
  tag/Bool 검사·snapshot single write·guard single evaluation·source-order effect·body/join·try Error/Abort edges를 독립 검증한다.
  같은 타입 guard operand/arm 교환·guard bypass·false edge→body·snapshot overwrite·inactive read·effect/try/jump/body 삭제를 거부한다.
- 기존 private source/full-body certificates와 aggregate ABI를 유지한다. LLVM COFF/ELF O0/O2·Windows Native debug/release를 승인 후 검증한다.
  Linux object 생성과 Linux Native host 실행 증거를 구분한다.

## 수용 계획·승인 경계

- 두 파일 main/types·정상 3·부정 16·Runtime 1의 고정 UTF-8/LF source·proposed_result·byte Span을 제공한다.
  main 제안 출력 16줄은 mismatch skip·guard order·first match·snapshot·try false/Error·loop·Unit을 관측한다.
  [독립 유한 reference](../../tools/tests/guard-oracle.mjs)는 6개 vector의 unguarded coverage/usefulness와 runtime 선택/trace만 열거한다. Compiler 검증을 대신하지 않는다.
- 승인 후 Parser/END/prefix·Source tampering·scope/type/cascade·literal guard coverage·union·예산 경계·Checked/MIR forgery와 실제 Native tests를 구현한다.
  guard 없는 P01~P24 회귀와 실제 LLVM/Native 검사도 유지한다. 성공 전 implementation_verified를 true로 바꾸지 않는다.
- or/range/general literal·struct/let destructuring·match expression·guard refinement/predicate proving·Array·Read loan/take·일반 Move/Drop·전체 D08/D10/D12/D16/D25/D30은 별도 계약이다.
