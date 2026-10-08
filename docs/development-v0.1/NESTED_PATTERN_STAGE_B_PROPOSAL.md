# Stage B 중첩 Copy 패턴·Tuple match 최소 계약 — P24

작성일·승인일: 2026-10-08. 상태: **Accepted / 구현·검증 완료**.
사용자 “P24 승인하고 중첩 Copy 패턴·Tuple match 구현 진행” 답변으로 이 범위를 승인했다.
D01~D05/P01~P23·Canonical·원본 148개 문서를 보존한다.
[60-production EBNF](GRAMMAR_STAGE_B_NESTED_PATTERN.ebnf)·[수용 fixture](nested-pattern-proposal-fixtures/README.md)·[구현·검증 기록](NESTED_PATTERN_IMPLEMENTATION.md)을 따른다.
[착수 준비 기록](NESTED_PATTERN_PREPARATION.md)은 승인 전의 역사이며 현재 구현 결과는 별도 기록한다.

## Specification Change Proposal

- 관련 문서: [Canonical](CANONICAL.md), [MVP Freeze](../00_Governance/NOVA-002_Nova_0.1_MVP_기능_동결표.md),
  [원본 Pattern](../04_Functions_Control/NOVA-046_Pattern_문법_Binding_사양서.md),
  [원본 완전성](../04_Functions_Control/NOVA-047_Match_완전성_도달_불가_Arm_분석서.md),
  [원본 lowering](../04_Functions_Control/NOVA-048_Match_Lowering_Decision_Tree_사양서.md),
  [Pattern 초안](specs/NOVA-046.md), [완전성 초안](specs/NOVA-047.md), [D08/D10](DECISIONS.md).
  의존 계약은 [P13 Tuple](TUPLE_STAGE_B_PROPOSAL.md), [P14 Enum](ENUM_STAGE_B_PROPOSAL.md),
  [P15 Option/Result](OPTION_RESULT_STAGE_B_PROPOSAL.md), [P16 try](TRY_STAGE_B_PROPOSAL.md), [P19 loop](RANGE_LOOP_STAGE_B_PROPOSAL.md)다.
- 현재 사양: flat Enum/Option/Result/Bool statement match·immutable Copy payload binder·단일 scrutinee snapshot과
  N3101/N3102를 구현했다. Tuple scrutinee와 recursive pattern은 P14의 N1102 경계다.
- 발견된 문제: `Result<Option<(int8,bool)>,E>`를 한 match에서 검사하려면 product/sum 조합의 완전성,
  부분 중첩과 union coverage, recursive binder scope, 각 tag에 지배되는 payload 접근을 정의해야 한다.
- 제안 변경: 기존 Copy sum payload에 recursive pattern과 Copy Tuple/Unit statement match를 추가한다.
  guard·일반 literal·struct destructuring·Move/loan은 추가하지 않는다.
- 변경 이유: Stage B pattern/coverage를 기존 Copy 값으로 확장하고, Array 실행의 소유권·Drop 선행 조건은 보존한다.
- 영향 범위: Parser/AST/HIR·Source validation·Resolver/Checked pattern plan·coverage·MIR/validator·LLVM와 회귀 테스트.
  Lexer/keyword/END·타입/aggregate layout·scalar Runtime·CLI·const 평가 의미는 변경하지 않는다.
- Backward Compatibility: P01~P23 유효 프로그램과 flat match의 출력·진단·Span·기존 자원 경계를 보존한다.
  새 subset에서는 아래 matrix 검사를 사용한다. P14/P15 flat-only match는 기존 검사와 한도를 유지한다.
- 대안: guard·정수/문자/string literal·or pattern·Move binding까지 함께 구현하거나 Array를 먼저 구현한다.
  이번에는 유한 constructor 기반 Copy pattern만 제안한다. 전체 D08/D10/D12/D16/D25/D30 승인이 아니다.

## Grammar·Source·복구

1. P22 58-production EBNF의 `pattern`과 `pattern_arguments`만 확장하고 `tuple_pattern`·`unit_pattern` 두 production을 추가한다.
   총 **60개 production / 기존 56개 보존**이다. statement/arm/END/값 tuple/함수/메서드/생성자 문법은 바뀌지 않는다.
2. 허용 pattern은 `_`, bare IDENT binder, `true`/`false`, `()`, `(p,)`/`(p,q,...)`,
   `E::Empty`, `E::Data(p,...)`, `Option::Some(p)`, `Option::None`/`none`, `Result::Success(p)`/`Result::Error(p)`다.
   nested tuple/variant는 재귀적으로 조합한다. trailing comma는 기존 payload/tuple 정책을 따른다.
   `(p)` grouped pattern·`x: p`·`p as x`·or/range/guard·정수/float/char/string literal·struct pattern·change/take는 N1102다.
   value grouping `(x)`나 value expression/statement의 기존 처리를 pattern 문맥으로 확장하지 않는다.
3. bare IDENT는 항상 새 binder이며 같은 이름의 const/함수/type를 lookup하거나 value equality로 해석하지 않는다.
   `_`는 DefId 없는 discard다. qualified path의 head만 기존 type namespace를 조회한다.
   nominal type import alias는 허용하고 P21 transparent alias를 variant head로 쓰는 것은 계속 N1102다.
4. 원 pattern/tuple/variant/각 IDENT·괄호·쉼표·`::`/arm `=>`·body와 scrutinee byte Span을 보존한다.
   AST/HIR child 순서·arity·source spelling·양쪽 XID 경계·같은 파일·parent 포함·UTF-8 경계·punctuation/trivia gap을 검사한다.
   bundle 재배치에서도 원 FileId/owner/nominal ID를 유지한다. immutable public metadata의 임의 신뢰를 피한다.
5. parser nesting 기본 128과 기존 prefix recovery를 유지한다. 잘못된 nested pattern 뒤 다음 arm/선언을 복구하며
   recursive pattern 처리는 한도 검사 후 수행한다. 손상 AST/HIR를 재귀 탐색하다 stack overflow하지 않도록 반복형 gate를 사용한다.

## 타입·Copy binding·실행

1. root scrutinee는 기존 Bool/Copy Enum/Option/Result에 **Copy Tuple 또는 Unit**을 추가한다.
   primitive integer/float/char·String·struct root는 계속 N2101이다. tuple 안의 scalar/Copy struct는 binder 또는 `_`로 받을 수 있다.
   String/Move element·Array·Class·Function·View를 Copy subset에 추가하지 않는다.
2. 각 child는 parent의 정확한 component type을 따른다. Tuple arity는 정확히 같고 Unit은 `()`로만 생성자를 검사한다.
   Bool literal은 Bool, `none`은 Option, variant path는 동일한 nominal Enum 또는 해당 intrinsic specialization이어야 한다.
   variant/payload arity와 source annotation을 기존처럼 검사한다. implicit numeric conversion·binder annotation을 추가하지 않는다.
3. 같은 arm 전체를 왼쪽부터 depth-first source order로 binder 수집한다. 중복은 뒤 IDENT N2002 / 앞 IDENT secondary다.
   body root와 binder는 같은 scope다. 외부 shadow는 P02를 따른다. body 밖에서는 binder가 없다.
   binder는 초기화된 immutable Copy local이며 root 대입은 N3004다. `var copy=binder`는 독립 Copy를 만든다.
4. scrutinee는 한 번 먼저 평가하고 기존 방식으로 독립 snapshot을 만든다. pattern test에 side effect는 없다.
   arm은 source order로 선택하고 첫 matching body만 실행한다. snapshot에 대해 tests를 끝낸 후 선택 arm의 binders만 Copy한다.
   body가 원 var를 바꿔도 binder/다른 component의 snapshot은 바뀌지 않는다.
   outer/inner inactive payload는 해당 원 snapshot/tag 검사가 성공한 CFG 경로에서만 읽는다.
5. statement match 완료는 Unit이다. 모든 유효 reachable arm이 return/해당 jump이면 기존 CFG 분석을 따른다.
   arm의 break/continue는 enclosing loop, try는 enclosing 함수에 속한다. fallback 뒤 effect·join과 missing-return N3003을 보존한다.
   match expression/const function은 추가하지 않는다. statement match 안의 const/default 표현식 의미는 P05~P23과 같다.

## 완전성·도달성·결정성

1. wildcard/binder는 현재 component 전체를 덮는다. Bool constructors는 `false`, `true`, Unit은 하나,
   tuple은 arity/type로 결정된 하나의 product, Enum은 선언 순서 variant, Option은 P15 None/Some,
   Result는 P15 Success/Error를 사용한다. Copy struct/기타 scalar leaf는 검사 불가능한 한 opaque domain이며 `_`/binder만 허용한다.
   Bool ordering은 새 matrix witness 순서에만 적용한다. 기존 flat diagnostic 순서를 바꾸지 않는다.
2. source order로 앞 유효 arms의 **합집합**에 대해 다음 arm의 usefulness를 검사한다.
   완전히 덮이면 Error **N3102**, 뒤 pattern 전체 primary·덮는 앞 patterns를 source order secondary로 제공한다.
   부분 overlap만 있으면 유효하며 뒤 arm이 아직 남은 경우를 처리한다. 한 앞 arm만 비교하는 방식으로 완전성을 판정하지 않는다.
   모든 arms의 합집합이 root domain을 덮지 않으면 Error **N3101**, match keyword primary다.
3. recursive constructor specialization/default matrix로 usefulness와 누락 witness를 계산한다.
   열은 source product/payload 순서, branches는 위 constructor 순서, rows는 source arm 순서다.
   domain의 전체 cartesian product 또는 구체 값들을 사전 전개하지 않는다. hash iteration으로 진단/CFG 순서를 정하지 않는다.
   witness는 matching source 형식으로 최대 8개 notes와 추가 경우 생략 표시를 제공한다.
   scalar/struct opaque component는 `_`로 표시한다. 앞 covering secondary도 새 matrix 경로에서는 최대 8개+생략 note다.
4. pattern 타입/이름/arity/binding 오류가 있으면 그 match의 파생 N3101/N3102를 억제한다.
   유효 binder와 독립 body/name/type 오류는 검사하고 ErrorType 파생 진단은 억제한다.
   invalid path를 임의 variant/tuple child 타입으로 대체하지 않는다.

## 자원·진단

- 기존 match arm 1,025·mixed type/layout depth 128·size 1 MiB·occurrence 65,536·module/type/specialization 한도는 보존한다.
- 새 pattern 경로에서는 **match 전체 source pattern node 10,000개**를 허용한다.
  wildcard/binder/Bool/Unit/tuple/variant 각각 1, qualified head·punctuation은 0이며 component child를 source depth-first로 합산한다.
  첫 10,001번째 node 전체에 N8901 / 한도 note를 준다. body expression은 이 예산에 포함하지 않는다.
- 새 coverage 경로에서는 **match당 matrix subproblem 100,000개**를 허용한다.
  canonical `(typed columns, ordered rows, candidate)` key의 cache miss 작업에 1을 센다.
  source-order arm usefulness와 최종 wildcard exhaustiveness의 예산을 공유한다. cache hit는 0이다.
  각 task는 column/constructor 순서로 처리하고 memoization에서도 witness/covering source order를 유지한다.
  100,001번째 새 task 전에 match keyword N8901 / 예산 note를 보고하고 N3101/N3102는 억제한다.
  row/cell 저장은 총 1,000,000개로 별도 제한한다(동시에 live matrix의 pattern cells; 각 wildcard도 1).
  초과 전 같은 N8901로 종료하고 할당을 계속하지 않는다. matrix/cache/decision-tree Drop도 반복형으로 처리한다.
- flat-only P14/P15 형태(기존 variant의 IDENT/`_` payload 또는 root Bool/none/`_`)는 기존 검사를 유지한다.
  새 Tuple/Unit/root binder 또는 제한적인 nested constructor가 하나라도 있으면 해당 match 전체가 새 경로다.
  새 자원 제한이 기존 승인 flat 프로그램을 거부하지 않아야 한다.

| 상황 | code · primary / secondary |
|---|---|
| 다른 root scrutinee type | N2101 · scrutinee 전체 |
| tuple/payload arity·nullary parentheses | N2201 · 해당 tuple/variant pattern 전체 / component 선언 |
| constructor-kind/type 불일치 | N2101 · 잘못된 leaf/tuple 또는 qualified path / 해당 component annotation |
| unknown variant/type·visibility | 기존 N2001/N2004 · 해당 이름 / 원 선언 |
| 같은 arm recursive binder 중복 | N2002 · 뒤 IDENT / 앞 IDENT |
| immutable binder 대입 / 밖 참조 | N3004 · root IDENT / N2001 · 참조 IDENT |
| nested 경우 누락 / union에 의해 완전히 덮인 arm | N3101 · match keyword / N3102 · 뒤 pattern 전체 |
| resource 초과 | N8901 · 위 node 또는 match keyword / 한도 note |
| 제외 pattern·alias variant head | N1102 · 제외 construct 전체 또는 qualified path |

## Checked·MIR·Native 검증

- Checked plan은 원 scrutinee HirId/TypeId·원 pattern IDs·component projection paths·nominal variant IDs·binder DefIds/type/source를 가진다.
  coverage/usefulness를 public table과 별개로 재계산하고 누락 child/타입/중복 binder/잘못된 import owner를 거부한다.
- MIR은 단일 snapshot과 recursive tag/Bool 검사·tuple component projection·arm-local Copy·정적 CFG를 명시한다.
  각 payload read의 원 snapshot identity, 모든 ancestor active tag의 dominance, branch/arm 순서, binder type/path/source를 검증한다.
  동일 타입의 tuple slot/inner payload path 교환·tag proof 위조·snapshot overwrite·inactive read·effect 재배열·try/jump 변조를 거부한다.
  arity/type만 맞는 forged MIR를 안전하다고 판정하지 않는다. private source/full-body proof도 기존 gate를 유지한다.
- aggregate ABI/tag/layout·Core/LLVM 분리·padding/inactive bytes 미관측은 P12~P16 그대로다.
  LLVM COFF/ELF O0/O2와 Windows Native debug/release에서 단일 평가·snapshot·nested inactive 접근·선택된 body/try/jump·Abort Span을 검증한다.
  Linux object와 Linux Native host 실행 증거는 구분한다.

## 수용 계획·승인 경계

- 두 파일 main/types·추가 정상 3·부정 18·Runtime 1의 고정 UTF-8/LF source와 validated_result/byte Span을 검증했다.
  finite-domain coverage oracle 6개로 product/sum·union·partial overlap·누락/unreachable의 기대 결과를 독립 계산한다.
  oracle는 문서 기대값 검증이며 실제 Compiler/Native 검증을 대신하지 않는다.
- Parser prefix/recovery·Source tampering·nested scope/type/cascade·자원 경계·Checked/MIR forgery 회귀를 구현했다.
  node 10,000/10,001·task 100,000/100,001·live cell 1,000,000/1,000,001 경계와 기존 flat 경로 회귀를 분리 검증한다.
- 기존 Cargo fmt/check/clippy/test·runtime rustfmt·원본 hash·P01~P23 승인 ledger/fixture를 보존한다.
  fixture Compiler 진단/Span과 Native debug/release 출력을 검증한 뒤 implementation_verified=true를 기록했다.
- guard/or/range·일반 literal·struct destructuring·irrefutable let destructuring·match expression·Read loan/take pattern·
  Array·Move/Drop·전체 D08/D10/D12/D16/D25/D30은 별도 계약으로 남긴다.
