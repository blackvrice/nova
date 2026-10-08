# Stage B Copy struct 생성자 이름 인수 최소 계약 — P23

작성일: 2026-10-08. 상태: **Draft / 미승인 / 미구현**.
D01~D05/P01~P22·Canonical·원본 148개를 보존한다.
[기존 P22 58-production EBNF](GRAMMAR_STAGE_B_METHOD.ebnf)를 그대로 재사용하며 새 문법 파일은 만들지 않는다.
[수용 계획](struct-named-arguments-proposal-fixtures/README.md)의 결과는 제안값이며 실행 검증 결과가 아니다.

## Specification Change Proposal

- 관련 문서: [Canonical](CANONICAL.md), [MVP Freeze](../00_Governance/NOVA-002_Nova_0.1_MVP_기능_동결표.md),
  [원본 Struct](../03_Types_Declarations/NOVA-027_Struct_Class_Enum_선언_사양서.md),
  [원본 Receiver](../04_Functions_Control/NOVA-035_함수_메서드_호출_규약_Receiver_사양서.md),
  NOVA-014/020/023/024/029/030/031/032/033/036/037/040/073~078/081/083/087/091/093~097/136와 [D09/D11/D12/D16](DECISIONS.md).
  의존 계약: [P12 struct](STRUCT_STAGE_B_PROPOSAL.md), [P17 named](NAMED_ARGUMENTS_STAGE_B_PROPOSAL.md),
  [P18 default](DEFAULT_ARGUMENTS_STAGE_B_PROPOSAL.md), [P21 alias](ALIAS_STAGE_B_PROPOSAL.md), [P22 method](METHOD_STAGE_B_PROPOSAL.md).
- 현재 사양: generated Copy struct 생성자는 field 선언 순서의 위치 인수를 요구한다. 함수/Read 메서드의 이름 인수는 지원한다.
  P12/P17의 named constructor 제외와 N2201을 이번 subset에서만 확장한다. explicit init은 별도 계약이다.
- 발견된 문제: 생성자의 label 대상·타입 문맥·privacy·const 예산·평가 순서·aggregate provenance가 미동결이다.
- 제안 변경: `Point(y:2,x:1)`과 위치 prefix 뒤 이름 인수를 nominal Copy struct generated constructor에 허용한다.
  label은 field 이름이며 methods는 constructor slot이 아니다. 새 token·keyword·END·production은 없다.
- 변경 이유: 기존 Stage B constructor와 P17 source-order 인수 대응을 연결하며 const/default까지 같은 의미를 검증한다.
- 영향 범위: Resolver constructor 선택·TypeChecker field mapping/expected type·const/default/dependency·MIR aggregate lowering/validation·tests·문서.
  기존 AST/HIR NamedArgument 구조와 source proof를 재사용한다. LLVM은 승인된 aggregate ABI를 재사용한다.
- Backward Compatibility: 유효한 P01~P22 위치 생성·함수/메서드 호출·동일 이름 value 우선·가시성·layout·오류 코드를 보존한다.
  기존에 N2201이던 Copy struct 이름 생성만 허용한다. Enum/Option/Result/builtin print는 위치 인수 전용이다.
- 대안: named constructor 계속 제외, explicit init/field default까지 확장, 또는 Array/ownership부터 구현.
  이번 제안은 Copy struct generated constructor만 다룬다. 전체 D09/D11/D12/D16/D30 승인이 아니다.

## Constructor 선택·field 대응·가시성

1. P12의 bare IDENT 호출 선택을 유지한다. lexical value가 있으면 기존 value 호출을 선택한다.
   함수면 P17/P18을 따르고 non-callable value면 N2101이다. type 생성자로 재시도하지 않는다.
   value가 없을 때만 nominal StructDefId를 선택한다. module import alias는 원 struct/field ID를 보존한다.
   transparent type alias는 type 위치 전용이며 alias constructor head는 기존 N1102다.
2. label은 원 struct의 storage field identifier다. field의 let/var·기본/internal/public·이름 self 여부와 무관하게
   declaration order slot으로 대응한다. method 이름은 알려진 field 목록이나 slot에 포함하지 않는다.
   XID 철자·대소문자·Unicode 비정규화는 P17/D02 그대로다. label을 lexical value reference로 해석하지 않는다.
3. 위치 인수 prefix는 field 0부터 채운다. 첫 이름 인수 이후 위치 인수는 N2201이다.
   나머지 이름 인수는 어떤 순서라도 가능하지만 모든 field를 정확히 한 번 채워야 한다.
   unknown/duplicate/위치와 이름 중복/초과/누락은 N2201이다. `_`도 실제 field 이름일 때만 대응한다.
   빈 struct의 `Empty()`는 유지하며 method만 있는 Empty에 이름 인수를 주면 unknown label이다.
4. field initializer/default·생략 field·spread·shorthand·struct literal·external label·overload는 추가하지 않는다.
   mixed field/method 선언은 field만 세며 receiver slot·method default를 생성자에 가져오지 않는다.
5. type 선택과 constructor 전체 field 접근 권한은 P12/P11을 유지한다. private field가 있으면 외부 module은
   public 이름 인수라도 constructor를 호출할 수 없다. N2004 primary는 call 전체, secondary는 private field 선언이다.
   같은 module의 factory는 이름 생성이 가능하다. opaque return type/API leak 정책을 새로 도입하지 않는다.
   constructor 접근 실패가 확인된 call에는 field 대응 N2201/known-label note를 추가하지 않는다.
   인수 내부의 독립 name/type 오류는 기존처럼 검사한다. private type import의 기존 진단은 그대로다.

## 타입·평가 순서·const/default

1. 유효하게 대응한 값 expression의 기대 타입과 secondary annotation Span은 해당 FieldId의 선언에서 얻는다.
   source ordinal이 아닌 field mapping으로 literal 문맥·P07/P09 승격·P13 tuple·P15 sum 문맥을 적용한다.
   int8 field의 `128`은 N2102이며 Bool에 정수 값 등은 N2101이다. nominal identity/완성 aggregate widening 규칙은 그대로다.
   잘못된 label에는 임의 field 기대 타입을 주지 않는다. 그 오류로 파생 N2101/N2102를 만들지 않는다.
2. caller는 제공 값 expression을 source order로 정확히 한 번 평가·필요한 lossless 변환·Copy snapshot한다.
   모든 값이 성공한 뒤 snapshot을 field declaration order로 aggregate에 배치한다.
   뒤 인수가 앞 값을 overwrite하거나 재평가하지 않는다. nested aggregate/call/short-circuit도 기존 CFG 의미를 따른다.
   try Error/checked Abort는 즉시 기존 경로로 나가며 앞 effect만 남긴다. 뒤 인수·aggregate 생성·후속 statement는 실행하지 않는다.
3. P05/P06/P12의 순수 const 생성·projection에 이름 생성도 허용한다. 함수 parameter default는 P18의 declaration module scope다.
   field label은 값 scope를 만들지 않는다. 값 expression의 const dependency만 SCC에 포함하며 label 자체는 포함하지 않는다.
   순환은 N3202이고 user function/method 호출은 이름 생성 내부에서도 N3201이다. skipped logical RHS도 legality/dependency 검사를 받는다.
4. initializer당 **10,000-node** static permission tree 한도는 유지한다. constructor Call은 1 node,
   값 expression은 기존 P05~P22 규칙으로 센다. callee head와 label/colon은 제외한다.
   NamedArgument wrapper는 값 의미가 없는 표기이므로 **0 node**로 투명하게 통과해 값 child만 센다.
   같은 값 표현식의 위치/이름 생성은 같은 node 비용이다. conversion metadata는 새 expression node가 아니다.
   각 인수의 모든 값 child·skipped RHS를 source order로 센다. cached const reference는 기존 1 node다.
   정확히 10,000은 허용, 10,001번째 값 node는 N3202와 그 원 value Span이다. name/type 오류의 파생 const 오류는 억제한다.
5. const evaluator도 값들을 source order로 평가·변환하고 ConstValue::Struct.fields는 field declaration order로 저장한다.
   checked 실패의 첫 source 값과 원 Span은 Runtime/const가 일치해야 한다. compile-time 실패는 기존 N3201이다.
   새 const function·runtime default·field 의존 default·Move/Drop·String field는 추가하지 않는다.

## 진단·Source·MIR 검증·자원

| 상황 | code · primary / secondary |
|---|---|
| unknown/method label | N2201 · label / struct 선언과 알려진 field note |
| field 중복 충족 | N2201 · 두 번째 label / 첫 인수와 field 선언 |
| 이름 뒤 위치 인수 | N2201 · 뒤 값 expression 전체 / 첫 label |
| mixed 초과 위치 인수 | N2201 · 초과 값 expression / struct 선언 |
| 대응 오류 없는 이름 call의 누락 | N2201 한 개 · call 전체 / 누락 field 선언과 이름 note |
| constructor private field | N2004 · call 전체 / private field 선언; 파생 mapping 진단 억제 |
| undefined/non-callable head | 기존 N2001/N2101 · head / 파생 mapping 오류 억제 |
| Enum/sum/builtin 이름 인수 | 기존 N2201 · 첫 label / 위치 인수 전용 note |
| alias constructor head | 기존 N1102 · call 전체 |
| 값 타입/범위 | 기존 N2101/N2102 · 값 expression/literal / 대응 field annotation |
| const/default 불허 함수 호출 | 기존 N3201 · 내부 call 전체 / initializer 선언 |
| const cycle/budget | 기존 N3202 · reference/초과 값 node / dependency/initializer |

- 대응 오류는 source order로 보고하고 이미 대응 오류가 있는 call의 파생 누락 진단은 억제한다.
  알려진 field note는 선언 순서 첫 8개와 생략 표시로 제한한다. P17 함수 진단은 변경하지 않는다.
- 원 AST/HIR NamedArgument의 name/colon/whole/value Span·정확한 source spelling·양쪽 XID 경계·같은 파일·UTF-8 경계·
  parent 포함·source order·trivia gap을 유지한다. label은 값 child가 아니며 value는 정확히 하나다.
  source proof·bundle 재배치·ErrorType cascade 규칙을 유지한다. 승인 grammar의 58개 production은 전부 보존한다.
- typed 생성 계획은 원 StructDefId/FieldId·source argument ID·source-index→field-index 대응을 가진다.
  완전성·유일성·bounds·원 field registry/annotation·field-only order·expected type를 Resolved/Checked gate에서 독립 재계산한다.
  mutable public table을 신뢰해 잘못된 mapping을 backend에 넘기지 않는다. 함수 NamedCall/default/receiver 계획과 구분한다.
- MIR lowering은 source-order snapshot과 declaration-order aggregate Operand를 연결한다.
  private provenance와 caller full-body proof는 동일 타입 operand 교환·다른 constructor/field ID·effect 재배열/복제/생략·
  snapshot overwrite·try Error 후 뒤 인수/aggregate 생성·잘못된 layout·source/label 변조를 거부해야 한다.
  arity/type/layout 검사만으로 source 평가 순서를 증명했다고 보고하지 않는다. 기존 함수/메서드 provenance는 보존한다.
- 새 ABI/LLVM 의미는 없다. aggregate register/stack ABI·zero-size 값·checked 실패·Scalar source Span 전달은 기존 계약이다.
  Core에 LLVM 자료형을 도입하지 않는다. constructor label 대응은 argument+field 수에 선형으로 구축한다.
  struct/field 각각 1,024·layout depth 128·1 MiB·expansion 65,536·module 1,024·기존 parser 한도를 유지한다.

## 수용 계획과 완료 조건

- 두 파일 main/types, 추가 정상 2·부정 24·Runtime 1사례: Unicode label·field self·method interleave·import alias·
  mapped int8/int16/Bool/Char/Tuple/Option/Result·const/default scope·source effect order·try early return·checked Abort.
  expected.json의 모든 결과는 **proposed_result**, implementation_verified=false다. 원 LF/UTF-8 byte Span을 고정한다.
- 승인 후 Parser/AST/HIR source 변조·Resolver 선택/alias/visibility·Checked mapping 변조와 const/default/dependency·
  10,000/10,001 비용 경계·단계별 자원 경계·MIR same-type operand/effect/try forgery 회귀를 추가한다.
  이름/위치 생성의 동등 node 비용과 source-order 첫 overflow를 별도 검사한다.
- 네 Cargo gate·runtime rustfmt·문서 validator·P01~P22 regression을 통과시킨다.
  LLVM COFF/ELF O0/O2 aggregate 경로와 Windows x64 Native debug/release에서 제안 출력/진단/Span을 검증한다.
  문서 검사는 compiler/Native 실행을 대신하지 않는다. 통과하기 전 Accepted/implementation_verified로 바꾸지 않는다.
- Array·explicit init/Drop·field default·Enum/Option/Result named payload·generic constructor·bound method·change/take·
  일반 Move/borrow·전체 D09/D11/D12/D16/D30은 별도 제안으로 남긴다.
