# Stage B irrefutable Copy let 구조 분해 최소 계약 — P26

작성일: 2026-10-12. 상태: **Draft / 사용자 승인 대기 / 미구현**.
P01~P25·D01~D05·Canonical·원본 148개 문서를 보존한다.
[62-production 제안 EBNF](GRAMMAR_STAGE_B_DESTRUCTURING.ebnf)·[고정 수용 계획](destructuring-proposal-fixtures/README.md)을 함께 검토한다.
이 문서·proposed_result·독립 oracle는 승인 또는 Compiler/Native 실행 성공의 증거가 아니다.
[착수 준비 기록](DESTRUCTURING_PREPARATION.md)에 현재 회귀 검사와 미구현 진단을 기록했다.

## Specification Change Proposal

- 관련 문서: [Canonical](CANONICAL.md), [MVP Freeze](../00_Governance/NOVA-002_Nova_0.1_MVP_기능_동결표.md),
  [binding](specs/NOVA-035.md), [scope](specs/NOVA-038.md), [Pattern](specs/NOVA-046.md), [완전성](specs/NOVA-047.md),
  [Lowering](specs/NOVA-048.md), [D08/D10](DECISIONS.md).
  의존 계약은 [P13 Tuple](TUPLE_STAGE_B_PROPOSAL.md), [P24 Copy pattern](NESTED_PATTERN_STAGE_B_PROPOSAL.md),
  [P25 guard](GUARD_STAGE_B_PROPOSAL.md), [P16 try](TRY_STAGE_B_PROPOSAL.md), [P19 loop](RANGE_LOOP_STAGE_B_PROPOSAL.md)다.
- 현재 사양: 일반 let/var/const IDENT binding, Copy Tuple/Enum·recursive statement match·immutable pattern binder를 구현했다.
  let 구조 분해는 P24의 명시적 제외 범위다. 기존 Parser는 let 뒤 Tuple/Unit pattern을 거부한다.
- 발견된 문제: Copy product/single-variant sum 값을 여러 local로 분해하려면 선언 시점·타입 문맥·irrefutability·단일 평가·payload proof를 정의해야 한다.
- 제안 변경: 함수 block의 `let destructuring_pattern [: type] = expression`을 추가한다.
  irrefutable Copy pattern만 허용하고 initializer는 필수다. 실패할 수 있는 binding을 runtime 분기/Abort로 대체하지 않는다.
- 변경 이유: Stage B의 기존 Copy 값·pattern·local binding을 연결하며 Array의 ownership/Drop 선행 조건은 유지한다.
- 영향 범위: Parser/AST/HIR/Source·Resolver/Type Checker/irrefutability·MIR/validator·LLVM 회귀·진단 등록·문서/fixture.
  Lexer/keyword/END·const/default 평가 의미·aggregate ABI/layout·Runtime API는 변경하지 않는다.
- Backward Compatibility: P01~P25 유효 프로그램과 일반 IDENT binding·match/guard의 진단/Span/출력·기존 자원 예산을 보존한다.
  `let _ = expression`은 기존 IDENT binding이며 새 discard 문법으로 재해석하지 않는다. `_` discard는 새 compound pattern 안에서만 적용한다.
- 대안: var/const·parameter·for·assignment 구조 분해 또는 refutable let-else/guard까지 확장한다.
  이번에는 local let의 irrefutable Copy subset만 제안한다. 전체 D08/D10/D12/D16/D25/D30 승인이 아니다.

## Grammar·END·복구

1. P25 61개 production의 binding 하나만 확장하고 destructuring_pattern 하나를 추가한다.
   총 **62개 production / 기존 60개 보존**이다. 기존 pattern/tuple_pattern/unit_pattern/variant_path를 재사용한다.
2. root는 Tuple·Unit·qualified Enum variant pattern이다. bare IDENT/`_` root는 일반 binding 경로를 유지한다.
   nested component는 P24 binder/`_`·Bool·Unit·Tuple·qualified variant·none을 재사용한다.
   그룹 `(x)`은 Tuple pattern이 아니며 N1102다. `(x,)`는 singleton Tuple다.
3. var/const compound pattern·global binding·parameter/for pattern·destructuring assignment·struct field pattern·or/range/general literal을 추가하지 않는다.
   guard/if/else·실패 대체 block도 binding에는 없다. 이 제외 문법은 N1102와 원 token/pattern Span으로 복구한다.
4. optional annotation은 **initializer 전체 값 타입**이다. component별 annotation 문법은 없다.
   annotation/`=`/initializer/END는 기존 binding/type normalization을 따른다. let keyword 뒤 pattern이 있어도 Lexer END를 변경하지 않는다.
   generic type의 >= split·Unicode·comment·prefix recovery를 보존한다.
5. AST/HIR는 let keyword·pattern·optional type·initializer·colon/equals SourceInfo를 명시한다.
   Source gate는 keyword/XID·child 순서/containment·punctuation·원 initializer와 UTF-8 byte boundaries를 검사한다.
   복구 Error node는 성공 Codegen 입력이 될 수 없다.

## Binding·기대 타입·irrefutability

1. initializer를 먼저 기존 scope에서 해석·검사한다. 같은 문장의 새 binder는 initializer에서 보이지 않는다.
   성공 후 pattern 전체 binder를 현재 block scope에 source depth-first order로 선언한다.
   새 binder는 const/function/type 이름을 조회하지 않는 immutable Copy local이다. 반복 이름/동일 scope 충돌 N2002,
   ancestor shadow 허용·변경 N3004·scope escape/initializer self-reference N2001은 기존 계약을 유지한다.
2. annotation이 있으면 P07~P23의 기존 initializer 기대 타입/checked literal/손실 없는 coercion을 적용한다.
   없으면 initializer의 기존 타입을 추론한다. **pattern으로 initializer 타입을 추론하지 않는다**.
   initializer가 ErrorType이면 dependent component/irrefutability 오류를 억제하고 독립 이름/스코프 오류는 유지한다.
3. Typed pattern은 P24의 canonical component type·arity·nominal/import identity·Copy 제한·private 접근 규칙을 따른다.
   Copy struct는 opaque component binder/`_`로만 받을 수 있다. String/Move element·struct destructuring·alias constructor/variant head는 추가하지 않는다.
   단일 variant user Enum은 허용하지만 Option/Result는 두 constructors이므로 특정 Some/None/Success/Error pattern은 refutable이다.
4. **타입의 전체 domain**을 덮는 pattern만 irrefutable이다. initializer가 특정 literal/variant/const임을 이유로 판정을 좁히지 않는다.
   Tuple/Unit·single-variant Enum과 그 children이 모두 irrefutable이면 성공한다.
   Bool true/false·multi-variant Enum·Option/Result constructor를 포함하면 refutable이다. binder/`_`는 Copy component 전체를 덮는다.
5. P24 bounded constructor/default matrix로 one-pattern coverage를 검증한다. concrete cartesian product를 미리 생성하지 않는다.
   Bool false/true·Enum 선언 순서·Option None/Some·Result Success/Error·opaque `_` witness와 결정성을 유지한다.
   값/guard truth 추론을 추가하지 않는다. 기존 match/guard의 분석 경로·예산은 변경하지 않는다.
6. 새 Error **N3103 — refutable binding pattern**을 제안한다. primary는 pattern 전체,
   secondary는 가능한 첫 refutable component의 원 Source Span, notes는 최대 8개 missing witness+생략 표시다.
   이 code는 **아직 공용 DIAGNOSTICS.csv/Runtime/Compiler에 등록하지 않는다**. P26 승인·구현 시 등록한다.
   pattern/initializer의 type/name/arity/binding 오류 또는 resource 실패가 있으면 파생 N3103을 억제한다.
   기존 N3101/N3102는 statement match에 그대로 사용한다.

## 실행·MIR·자원

1. initializer는 소스 위치에서 **한 번 평가**하고 coercion 후 한 Copy snapshot을 만든다.
   snapshot의 component를 source depth-first 순서로 projection/Copy해 local을 초기화한다. `_`는 DefId/local을 만들지 않는다.
   Unit/모든 wildcard pattern도 initializer의 effect를 생략하지 않는다. 원 value의 이후 변경은 binder 값에 영향을 주지 않는다.
2. initializer의 try Error는 enclosing Result 함수에서 먼저 반환한다. snapshot·binder·뒤 statement는 실행하지 않는다.
   checked Abort도 binding 실패 대체 경로로 바꾸지 않는다. loop 안에서는 도달한 각 iteration에서 평가한다.
   일반 Move/borrow/Drop·partial move/partial initialization·새 runtime failing destructuring은 추가하지 않는다.
3. Checked plan은 원 statement/pattern/type/initializer HirId·canonical type·component path·binder DefIds·SourceInfo·irrefutability proof를 가진다.
   public tables와 별개로 원 HIR/Resolved에서 재계산하며 삭제/owner/type/path/source/initializer/DefId 변조를 거부한다.
4. MIR은 initializer/coercion/snapshot/Copy binding 순서를 명시한다. single-variant payload도 기존 active-tag proof를 제공하며 필요한 단일-variant tag edge를 명시한다.
   private source/full-body proof로 same-type tuple slot/payload path·snapshot receiver·effect/try/control edge·overwrite/body 삭제를 거부한다.
   LLVM COFF/ELF O0/O2·Windows Native debug/release를 승인 후 검증한다. Linux object와 Linux Native host 실행은 별도 증거다.
5. 새 binding statement당 source pattern **10,000개**·canonical matrix task **100,000개**·live cells **1,000,000개**를 허용한다.
   binder/wildcard/Bool/Unit/tuple/variant 각각 1, punctuation/qualified head/annotation/initializer/body는 pattern 예산에 포함하지 않는다.
   첫 10,001번째 pattern node 전체에 N8901, task/cell 초과는 let keyword N8901이며 파생 N3103을 억제한다.
   cache-hit 0·초과 전 allocation 중단·반복형 traversal/Drop·pattern nesting 128·기존 mixed type/layout/declaration 한도를 유지한다.

## 수용 계획·승인 경계

- 두 파일 main/types·정상 3·부정 22·Runtime 1의 고정 UTF-8/LF source·proposed_result·byte Span을 제공한다.
  main 제안 출력 17줄은 단일 평가·중첩 single Enum/Tuple/Unit·opaque Copy struct·snapshot·shadow·기존 `_` binding·loop·try를 관측한다.
- 독립 유한 oracle 6개는 타입 domain의 one-pattern coverage와 initializer-independent irrefutability를 검사한다. Compiler 성공을 대신하지 않는다.
- 승인 후 Parser/END/prefix·scope/annotation/cascade·irrefutability·자원 경계·Source/Checked/MIR forgery·LLVM/Native 회귀를 구현한다.
  실제 진단/Span/출력 검증 전에 implementation_verified를 true로 바꾸지 않는다.
- var/const/global/parameter/for/assignment destructuring·let-else/guard·struct/or/range/general literal pattern·String/Move payload·Array·일반 Move/borrow/Drop·전체 D08/D10/D12/D16/D25/D30은 별도 계약이다.
