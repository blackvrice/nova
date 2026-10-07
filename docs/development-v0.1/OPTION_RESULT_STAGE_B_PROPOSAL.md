# Stage B Copy Option·Result·nullable 최소 계약 — P15

작성일: 2026-10-07. 상태: **Accepted / 사용자 승인 완료 / 구현 완료**.
기존 D01~D05/P01~P14·Canonical·원본 148개 문서를 보존한다.
전체 D06/D08/D09/D10/D12/D15/D16/D23/D25/D30 승인이 아니다.

## Specification Change Proposal

- 관련 문서: [Canonical](CANONICAL.md), [MVP Freeze](../00_Governance/NOVA-002_Nova_0.1_MVP_기능_동결표.md),
  [원본 Nullable/Option/Result](../03_Types_Declarations/NOVA-028_Nullable_Option_Result_타입_규칙.md),
  [원본 Option](../11_Standard_Library/NOVA-119_Option_T_API_사양서.md),
  [원본 Result/try](../11_Standard_Library/NOVA-120_Result_TE_try_API_사양서.md),
  [타입/const/layout](specs/NOVA-025.md), [const](specs/NOVA-032.md), [layout](specs/NOVA-033.md),
  [테스트 전략](../13_Testing_Release/NOVA-136_Compiler_테스트_전략서.md), [P14](ENUM_STAGE_B_PROPOSAL.md), [결정](DECISIONS.md).
- 현재 사양: Option은 Some/None, T?는 Option<T>, Result는 Success/Error다. Success(())는 Unit 성공이다.
  try는 Error 조기 반환이고 niche는 내부 최적화다. 현재 Compiler는 사용자 Copy Enum과 statement match까지 지원한다.
- 발견된 문제: builtin family의 source spelling·이름 shadow·type argument·생성 문맥·특수화 identity·제한이 없다.
  Move payload와 cleanup은 아직 Stage C 구현에 의존한다. Array는 원본 NOVA-029/117의 길이·용량·초기화·Drop 계약을 따르며 별도 단계다.
- 제안 변경: Copy payload만 갖는 builtin Option<T>/Result<T,E>, T? 정규화, qualified 생성과 none,
  P14의 단순 statement match·const·private tagged ABI를 아래 subset으로 확장한다.
- 변경 이유: P14의 검증된 유한 sum/Copy 기반에 실패·부재 값을 명시적으로 표현하는 타입을 연결한다.
- 영향 범위: END normalizer/type token adapter, AST/Parser/HIR/Resolver/Types/TypeChecker/const,
  MIR/validator/LLVM 및 compile/runtime tests. Scanner의 raw token과 Runtime scalar 정책은 유지한다.
- Backward Compatibility: 기존 scalar/struct/Tuple/Enum·import alias·가시성 의미는 유지한다.
  Option/Result라는 기존 사용자 type/value 이름을 새 hard keyword나 전역 예약어로 만들지 않는다.
- 대안: Option만 먼저 구현하거나 Result와 try를 한 번에 구현할 수 있다.
  이 안은 Option/Result의 Copy 값 생성·분기만 함께 동결하고 try/일반 Move cleanup은 후속 계약으로 남긴다.

## 문법·END·이름

[전용 EBNF](GRAMMAR_STAGE_B_OPTION_RESULT.ebnf)는 P14 type/primary_expr/pattern 세 production을 변경하고
`type_atom/type_arguments/type_close` 세 production을 추가한다. 총 **51개**다.

1. `Option<int8>`, `Result<(int8,bool),Failure>`, `int8?`를 type 위치에서 허용한다.
   type = type_atom 뒤 0개 이상의 ?이며 `int??`는 Option<Option<int>>다.
   `(int,bool)?`는 Tuple 전체의 Option이다. ()?는 Option<Unit>다. Type argument trailing comma를 허용한다.
   Named type의 `<...>` syntax를 파싱하되 의미 단계에서 builtin 두 family만 허용한다.
   Option 인수 1개·Result 인수 2개, 빈 인수/잘못된 구분자는 N1101, 잘못된 family arity는 N2101이다.
2. Lexer의 raw `<`, `>`, `>=`, `?` token·최장 일치·dump는 보존한다. 기존 Lexer에는 >> token이 없으므로
   `Option<Option<int>>`는 원래의 두 Greater token을 쓴다. Type close 위치의 `>=`만 Parser adapter에서
   원 byte subspan `>`와 `=`로 나눈다. `let x:Option<int>=none`도 허용한다.
   END normalizer는 TypeArguments top에서의 >=를 type close+assignment로 추적해 delimiter를 pop하고
   assignment 상태를 전환한다. normalized token 자체는 >=를 유지한다. expression 비교 >=는 바꾸지 않는다.
   기존 generic angle/newline·semicolon·Question 종료 규칙을 재사용한다. 새 전역 newline 억제는 없다.
3. Prelude에는 builtin type family Option/Result만 fallback으로 제공한다. 현재 모듈의 선언/type import가
   같은 이름을 가지면 해당 사용자 type이 우선한다. Local value는 P14처럼 type owner를 shadow하지 않는다.
   `Option<T>`와 `Option::Some`의 owner는 이 type lookup을 따른다. 사용자 Enum Option이면 기존 P14 variant다.
   shadow된 사용자 type에 type arguments를 붙이면 N1102다. `T?`는 spelling lookup 없이 intrinsic Option<T>다.
   실패한 type import는 fallback을 차단하고 원 import 진단·cascade 억제 규칙을 보존한다.
   없는 type owner/variant는 N2001, builtin이 아닌 type의 generic application은 N1102다.
   builtin family 자체를 value/type 인수로 쓰는 bare Option/Result는 N2101이며 concrete type이 아니다.
4. 생성은 `Option::Some(x)`, `Option::None`, `Result::Success(x)`, `Result::Error(e)`다.
   lowercase keyword `none`은 builtin Option::None sugar다. Some/Success/Error는 1개 positional 인수다.
   None()·Some()·Error()·잘못된 arity는 N2201이다. 생성자는 first-class function이 아니다.
   bare Some/None/Success/Error를 새 prelude value로 주입하지 않아 기존 사용자 함수 이름을 보존한다.
   expression의 `Option<int>::Some(...)` 같은 명시적 generic call 인수는 N1102다.
5. pattern은 P14의 qualified 단순 variant/binder 형식과 `none`을 허용한다.
   `Option::Some(x)`, `Option::None`/`none`, `Result::Success(x)`, `Result::Error(e)` 또는 `_`다.
   None(x)·missing binder·잘못된 arity는 N2201, 다른 family pattern은 N2101이다.
   일반 guard/nested/or/range pattern·match expression·exists·try·method API는 이번 범위가 아니다.

## 타입·생성 문맥·Copy·coverage

1. Concrete identity는 builtin family와 정규화한 모든 payload Type로 결정한다.
   int?와 Option<int32>는 같고, Result<T,E>와 Result<E,T>는 일반적으로 다르다.
   같은 field/variant 모양의 사용자 Enum과 builtin family는 서로 다른 타입이다.
   module/alias/nullable spelling 차이는 동일 concrete specialization을 재사용한다.
2. 허용 payload는 기존 숫자 10종·Bool·Char·Unit·Copy struct/Tuple/Enum·이번 Copy Option/Result다.
   String/Move payload는 N1102다. 기존 Copy struct/Tuple/Enum component에도 이 타입을 허용한다.
   binding/인수/return/const는 독립 Copy 값이며 mutable aggregate 경로 전체 값 교체는 P12/P13/P14 규칙을 따른다.
   Option/Result 직접 field/numeric projection·산술·비교·보간·cast는 N2101이다.
3. 타입이 정해진 binding/assignment/인수/return/aggregate element 문맥은 constructor payload에 기대 타입을 전달한다.
   `let x:int8?=Option::Some(7)`의 payload는 int8이다. 문맥 없이 Some(x)는 x의 기존 타입을 그대로 사용한다.
   bare scalar literal은 기존 기본 타입이며 peer literal 정책을 임의로 확대하지 않는다.
   `none`/Option::None은 기대 Option<T>가 있어야 한다. `Option::Some(none)`도 inner 문맥 없으면 N2103이다.
4. Result constructor는 완성된 기대 Result<T,E> 문맥이 필수다. 현재 한 variant의 인수만으로 반대 타입을
   추측하거나 뒤 statement/match arm에서 역추론하지 않는다. 문맥 없는 Result::Success(1)/Error(1)는 N2103이다.
   이미 타입이 정해진 Option/Result 값의 payload별 암묵 widening은 없다.
   T→Option<T> 자동 wrapping과 다른 family 자동 변환도 없다. Constructor 내부 scalar coercion은 기존 규칙이다.
5. Match는 scrutinee concrete 타입에서 family specialization을 정한다. 다른 payload specialization을 pattern에
   별도로 표기하지 않는다. Binder 타입은 해당 concrete variant payload다. 불변/duplicate/arm scope와
   scrutinee 단일 평가·Copy snapshot·모든 arm return·enclosing loop jump는 P14를 유지한다.
   누락 Some/None 또는 Success/Error는 Error N3101이고 이미 덮인 pattern은 Error N3102다.
   `none`과 Option::None은 같은 case다. 누락 note 순서는 Some/None, Success/Error다.
6. private factory가 반환한 inferred payload는 원 nominal ID와 기존 field visibility를 보존한다.
   builtin family는 직접 module item import/reexport 대상이 아니며 일반 타입 인수를 지원하는 사용자 Generic도 추가하지 않는다.

## Const·layout·자원 한도

1. const의 Some/Success/Error 생성은 1 node + 인수 expression이다. none/None은 1 node다.
   source type arguments와 constructor head/sugar는 별도 const expression node로 세지 않는다.
   Cached reference 1 node·group 규칙·initializer당 10,000-node·checked N3201·static cycle N3202를 유지한다.
   모든 runtime 함수 호출·try·match expression은 const에서 이번에도 허용하지 않는다.
2. private layout은 P14 tagged union algorithm이다. Option은 Some(T) tag 0 / None tag 1,
   Result는 Success(T) tag 0 / Error(E) tag 1로 internal metadata를 생성한다.
   source/FFI/serialization에서 tag 값·None=0·niche·public layout을 약속하지 않는다.
   Option<Unit>의 size/align은 4/4, Option<(int8,uint64)>는 24/8,
   Option<Option<(int8,uint64)>>는 32/8, Result<(),uint64>는 16/8을 독립 oracle로 검사한다.
3. per-bundle builtin concrete specialization은 두 family 합계 4,096개다. 동일 key 재사용은 늘리지 않는다.
   이 registry는 user Enum 1,024와 Tuple shape 4,096 예산과 구분한다. 모든 aggregate synthetic ID는
   서로와 source DefId에 충돌하지 않으며 type family/kind와 원 source 증거를 유지한다.
   origin은 같은 key의 최초 source 위치다. FileId/start/end 순서의 고유 origin으로 첫 초과 construct를
   결정한다. annotations 수집과 constructor inference pass 순서가 진단 위치를 바꾸지 않아야 한다.
4. 기존 Parser depth, mixed aggregate depth 128·size 1 MiB·all-variant occurrence 65,536을 유지한다.
   wrapper마다 aggregate depth 1이 추가되고 occurrence는 Some payload의 1+child, Result 양 payload의
   2+양 child occurrence다. Struct/Tuple/user Enum 직접 component·variant 제한은 P12~P14 그대로다.
   unused mixed by-value cycle도 N2101이며 builtin type argument의 closing named type Span과 cycle edge를 보존한다.
   모든 상한은 N8901에 한도 note와 첫 초과 source construct를 제공한다. 반복형/checked graph·상수·해제를 사용한다.

## MIR·Native·수용 기준

- Intrinsic family, payload type key, specialization ID와 source origin을 immutable certificate로 유지한다.
  enum과 같은 일반 MIR 생성/tag dispatch/payload read를 재사용하되 intrinsic kind/argument schema를 별도 검증한다.
  nominal user Enum으로 바꾸거나 두 specialization을 같은 ID로 바꾼 손상 MIR은 거부한다.
- P14 type/arity/완전 초기화/원 signature/layout/source provenance/coverage와 CFG active tag proof를 유지한다.
  wrong family/variant·case retarget·receiver overwrite·unproven payload read가 LLVM에 도달해서는 안 된다.
- caller Copy snapshot pointer 인수/out pointer 반환 ABI·entry temporary를 재사용한다.
  Windows Native debug/release와 Windows COFF/Linux ELF O0/O2는 별도로 검증한다. Linux host 실행은 후속 증거다.
- [두 파일 fixture](option-result-proposal-fixtures/README.md)는 import·private factory·nullable·nested none·Unit Result와
  Copy snapshot의 제안 stdout을 제공한다. 부정 source/code/UTF-8 byte Span은 [expected.json](option-result-proposal-fixtures/expected.json)에 있다.
  문서 validator의 성공은 Compiler/Native 성공이 아니다.
- 승인 후 UTF-8 truncation/복구, type argument adapter/END·기존 >= comparison token 보존,
  alias/nominal/shadow/문맥·N2103·Copy/visibility·coverage/flow, const budget/cycle, 모든 자원 경계·layout oracle,
  malformed AST/HIR/MIR specialization/schema/CFG gate·deep payload 해제와 기존 전체 회귀를 검사한다.

## 진단·승인 경계

| 상황 | 진단·primary |
|---|---|
| token 누락/잘못된 type argument 문법 | N1101, 원 token 또는 zero-width point |
| Copy 밖 payload/user generic/명시적 generic call/미포함 syntax | N1102, payload type/feature construct |
| undefined owner/variant·visibility | N2001/N2004, 이름/사용 Span |
| family arity·payload 타입·wrapping·pattern family·cycle | N2101, type/expression/path/closing named type |
| none/Result/inner payload 문맥 부족 | N2103, none 또는 constructor expression |
| constructor/pattern arity | N2201, constructor/pattern 전체 |
| binder duplicate/불변/scope | 기존 N2002/N3004/N2001 |
| coverage/도달성 | 기존 Error N3101/N3102 |
| const 실패/cycle/budget·자원 한도 | 기존 N3201/N3202/N8901 |

미포함: Array, try·exists·unwrap/메서드 API, 사용자 Generic/const generic, Move/String payload,
일반 Read/change/take·borrow/Drop, nested pattern/guard·match expression, niche/public ABI/FFI.
2026-10-07 사용자 “승인 할테니 다음 개발 작업 진행해줘” 답변으로 P15를 승인했다.
[구현·검증 기록](OPTION_RESULT_IMPLEMENTATION.md)과 [사용자 실행 명령](../../TESTING.md)을 따른다.

## 초안 준비 시점의 검증 기록 (승인 전 이력)

2026-10-07 문서 validator PASS: 원본 148개 SHA-256 보존, 51개 production 중복/참조/도달성,
Draft ledger와 부정 21사례의 UTF-8 byte Span·제안 기대값 metadata를 확인했다.
EBNF 무모호성 증명이나 P15 Compiler/Native 실행 증거는 아니다.
기존 P14 기본 workspace 276개 tests PASS (LLVM/Native 46개 ignored), fmt/Runtime rustfmt,
clippy(-D warnings)/all-features check와 git diff --check PASS를 확인했다.
`examples/enums.nova` check/debug/release는 이번 문서 준비에서도 실제 실행해 P14의 네 줄 stdout·exit 0을 확인했다.
Compiler/Runtime source와 기존 accepted ledger는 변경하지 않았다. opt-in LLVM/Native 전체 46개는 다시 실행하지 않았다.
환경은 Rust 1.99.0, LLVM 21.1.8, Windows x64다. 승인 후 P15 자체 회귀와 Native 검증이 필요하다.
