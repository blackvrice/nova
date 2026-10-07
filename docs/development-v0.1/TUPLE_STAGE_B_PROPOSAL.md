# Stage B Copy Tuple 최소 계약 — P13

작성일: 2026-10-07. 상태: **Draft / 사용자 승인 대기**. Compiler에는 아직 적용하지 않았다.
기존 D01~D05/P01~P12·Canonical·원본 148개 문서를 보존한다. 전체 D09/D10/D12/D16 승인이 아니다.

## Specification Change Proposal

- 관련 문서: [Canonical](CANONICAL.md), [MVP Freeze](../00_Governance/NOVA-002_Nova_0.1_MVP_기능_동결표.md),
  [원본 Tuple/Array/Function](../03_Types_Declarations/NOVA-029_Tuple_Array_Function_타입_사양서.md),
  [원본 layout](../03_Types_Declarations/NOVA-033_타입_레이아웃_정렬_Niche_사양서.md),
  [Tuple 초안](specs/NOVA-029.md), [결정](DECISIONS.md), [P12](STRUCT_STAGE_B_PROPOSAL.md).
- 현재 사양: Unit은 (), 양방향 타입 검사·손실 없는 숫자 승격·source-order 평가를 유지한다.
  Stage B에 Tuple이 있지만 원본은 구체 생성/타입/projection/가변성/ABI를 제공하지 않는다.
  보완 문서의 `(x,)`, `(x,y)`, `.0/.1`은 D12 Draft이며 현재 Compiler는 Unit/group만 지원한다.
- 발견된 문제: struct 다음의 익명 product type을 위한 structural identity와 Copy 경로 규칙이 필요하다.
  기존 longest-match Lexer는 `t.0.1`을 IDENT/Dot/FLOAT(0.1)로 읽으므로 Parser 계약 없이 임의 해석할 수 없다.
- 제안 변경: 아래 Copy Tuple subset과 전용 EBNF·numeric selector 해석·자원 제한을 동결한다.
- 변경 이유: 일반 Move/borrow/Drop·Array allocation·Enum/match를 앞당기지 않고 product 값의 의미를 검증한다.
- 영향 범위: Parser/AST/HIR type 표현, Types/Resolver/TypeChecker, const, MIR/validator, LLVM Adapter와 테스트.
  Lexer token 종류·원본 token/Span·END, Runtime scalar API와 CLI는 유지한다.
- Backward Compatibility: 기존 scalar/struct 소스, P07/P09 기대 literal, P10 cast, P11 import·가시성은 유지한다.
  P12 struct field에 Copy Tuple을 허용하고 mixed graph layout 검증으로 확장한다. 전체 D12를 승인하지 않는다.
- 대안: tuple을 뒤 Stage로 미루거나 `.0.1`에 `(t.0).1` 표기를 요구할 수 있다.
  이번 제안은 숫자 selector 문맥에서만 원 token의 byte subspan을 사용해 연속 selector를 해석한다.

## 구문·Lexer 경계

[전용 EBNF](GRAMMAR_STAGE_B_TUPLE.ebnf)는 P12의 type/primary_expr/postfix_expr/field_path를 확장하고
세 production(tuple_type/tuple_expr/tuple_index)을 추가한다. 합계 40개다.

1. `()`는 계속 Unit, `(x)`는 grouping, `(x,)`는 1-tuple, `(x,y)`/`(x,y,)`는 2-tuple이다.
   element는 왼쪽부터 한 번씩 평가한다. tuple 타입은 `(T,)`, `(T,U)`/`(T,U,)`이며 `(T)` 타입 grouping은 이번 범위 밖이다.
   trailing comma는 허용하며 newline/주석/END는 기존 괄호 규칙을 따른다. 중첩 expression/type parsing은 기존 nesting 상한 128을 따른다.
2. `expr.0`, `expr.1`은 call/as/struct projection과 같은 postfix 층에서 왼쪽 결합한다.
   index는 `0` 또는 `[1-9][0-9]*`의 ASCII decimal 철자다. leading zero·underscore·radix·exponent·음수는 N1102다.
   tuple index는 Integer 타입을 가진 expression이 아니다. 매우 큰 decimal도 host integer overflow 없이 out-of-range N2001로 처리한다.
3. Lexer의 최장 일치를 유지한다. Dot 뒤 selector 문맥의 numeric token 전체가 두 canonical decimal index와 점 하나
   (`0.1`, `12.0` 등)로만 구성되면 Parser가 이를 index/Dot/index의 원 byte subspan으로 해석한다.
   `t.0.1.2`는 세 projection, `(t.0).1`도 같은 뜻이다. numeric selector를 기다리지 않는 위치의 `0.1`은 기존 float literal이다.
   `t.0.1e2`/`t.0_1` 등은 numeric token 전체에 N1102이며 숫자 접두사만 수용하지 않는다.
   trivia/newline이 있어도 기존 tokens와 END를 바꾸지 않는다. Token dump/source reconstruction은 바뀌지 않는다.
4. assignment target은 bare local 이름 뒤 `.IDENT`/`.index`의 혼합 경로다. 임시 값·grouped root·indexing target은 P12와 같이 N1102다.
   tuple literal/tuple type은 오류 후 다음 element/statement/top-level 선언으로 복구하며 사용자 입력으로 panic하지 않는다.

## 타입·Copy·가변 경로

1. element는 8종 정수·Float32/64·Bool·Char·Unit·P12 struct·다른 P13 tuple만 허용한다.
   String/Function/Array/Enum/Option/Result/borrow/Move element는 N1102다. undefined named type은 기존 N2001이다.
   P12 struct field도 위 Copy tuple 타입을 허용한다. type import는 기존 struct만 대상으로 하며 tuple은 익명 타입이다.
2. tuple identity는 arity와 순서 있는 정규화 element type의 structural identity다. primitive alias는 같고 struct element는 원 nominal ID로 구분한다.
   서로 다른 위치에서 같은 타입 목록을 가진 tuple은 동일하다. `(int,)`는 int와 다르고 `()`는 0-tuple로 별도 생성하지 않는다.
3. 기대 tuple 타입이 있고 arity가 같으면 tuple expression의 각 element로 기대 타입을 전파한다.
   P07/P09 literal 범위·손실 없는 scalar 승격과 중첩 tuple expression 문맥을 적용한다.
   기대 arity 불일치 N2101은 tuple expression 전체, element mismatch는 해당 expression이 primary다.
   문맥 없는 element는 기존 기본 타입을 따른다. 이미 만들어진 tuple/struct 값에는 element-wise 암묵 변환을 제공하지 않는다.
   예: `(1,): (int8,)` 문맥은 허용하지만 `(int32,)` binding을 `(int64,)` 변수에 바로 대입하는 것은 N2101이다.
4. tuple binding/대입/함수 인수·반환/const는 독립 Copy 값이다. tuple 전체 산술·비교·보간·cast는 N2101이며 scalar element를 꺼내면 기존 규칙을 따른다.
   non-tuple receiver의 numeric selector와 tuple receiver의 named selector는 N2101이다.
   범위 밖 numeric selector는 N2001에 selector digits만 primary로 표시한다. `.0.9`에서 9만 표시하며 원 FileId/byte Span을 유지한다.
5. root는 local var여야 한다. tuple element는 별도의 let/var 표시 없이 var root 아래에서 변경할 수 있다.
   경로의 모든 struct field는 var여야 한다. let/const root·parameter 또는 let struct field를 통과하면 첫 불변 이름에 N3004다.
   `t.0.x`, `s.items.1`도 같은 규칙이다. RHS는 한 번 평가한 뒤 저장하며 저장 전 root 값을 읽는다.
6. tuple element로 private struct 타입을 사용할 때 명시적 이름에는 기존 type visibility를 적용한다.
   factory 반환 등 추론된 private struct 값의 tuple 보관은 허용하고 named field에는 P12 field visibility를 유지한다.
   destructuring·pattern binding·spread·named tuple·type alias·first-class function type은 이번 범위 밖이다.

## Const·layout·자원 제한

1. const에 순수 tuple 생성·projection을 추가한다. tuple expression은 1 node + 실제 평가한 element expression,
   projection은 1 node + receiver다. grouping·selector digits는 새 예산 node가 아니다.
   cached const reference 1 node, initializer별 10,000-node 제한과 checked N3201, static dependency/skipped RHS cycle N3202를 보존한다.
   tuple element는 선택되지 않더라도 생성 시 모두 평가한다. 사용자 함수는 const에서 계속 금지한다.
2. tuple layout은 element 순서와 P12 x64 natural alignment/padding이다. Unit/빈 struct element도 논리 ID/완전 초기화를 유지한다.
   structural tuple metadata는 Compiler Core에 두고 LLVM 전용 자료형을 넣지 않는다.
3. bundle의 고유 tuple shape는 최대 4,096개, tuple당 element는 최대 1,024개다.
   동일 normalized shape 재사용은 개수를 늘리지 않는다. source-order 첫 초과 type/tuple expression에 N8901을 표시한다.
   P12의 struct 개수/field 한도는 유지한다. struct와 tuple의 mixed aggregate layout 깊이 128·size 1 MiB·
   transitive element/field occurrence 65,536 제한을 함께 적용한다. 각 child occurrence 1 + child aggregate count로 계산한다.
   cycle은 unused 여부와 무관하게 N2101이고 layout보다 먼저 진단한다. `(A,)`를 field로 갖는 A도 재귀 값이므로 거부한다.
   cycle을 닫는 named type token과 다른 cycle edge를 primary/secondary로 보존한다.
4. graph·layout·payload validation·deep malformed payload 해제는 반복형/checked 처리다. Resource limit은 runtime stack 크기를 보장하지 않는다.

## MIR·Native 계약

- MIR에 structural tuple shape/element identity, 생성/읽기/혼합 경로 갱신을 표현한다.
  원 source/type/shape/path provenance·인수/반환 signature·완전 초기화·layout을 독립 검증한다.
  오류 type 또는 변경된 side table/MIR metadata는 CodegenUnit을 만들 수 없다.
- private ABI는 P12의 caller-owned snapshot 간접 인수와 out pointer 반환이다. scalar ABI는 유지한다.
  tuples와 struct 내 tuple, tuple 내 struct의 zero-size/mixed layout을 두 target object와 O0/O2로 검증한다.
  tuple shape의 결정적 ID는 동일 bundle/options에서 안정적이어야 한다. 공개 FFI/serialization/layout API는 제공하지 않는다.
- Windows Native debug/release는 독립 Copy·source-order 단일 평가·checked Abort의 원 file/byte Span과 출력이 동일해야 한다.
  Linux ELF object 검증을 Linux Native host 실행이라고 보고하지 않는다.

## 수용 기준과 진단

[두 파일 제안 fixture](tuple-proposal-fixtures/README.md)는 **미검증 Draft 기대값**이다.

| 상황 | 진단/primary |
|---|---|
| 잘못된 구문/selector 철자 | N1101/N1102, 오류 token 또는 numeric token 전체 |
| 범위 밖 Copy element | N1102, element expression/type |
| undefined type / index 범위 | N2001, 이름 또는 index digits |
| visibility | 기존 N2004, 접근 이름 + 원 정의 secondary |
| arity/element/nominal mismatch·aggregate 연산·cycle | N2101, tuple/element/type·cycle 근거 |
| 불변 root/struct field 경로 | N3004, 첫 불변 이름 + 정의 secondary |
| checked const / dependency cycle·budget | 기존 N3201/N3202와 원 source Span |
| aggregate 자원 상한 | N8901, 초과 type/tuple expression + 한도 note/원인 element |

승인 후 Parser UTF-8 truncation·one-tuple/group/Unit·trailing comma·연속 selector subspan·혼합 target 복구,
structural/nominal 타입·양방향 문맥·private factory·multi-file 원 ID, const budget/cycle,
shape/arity/depth/count/size 경계, 독립 layout oracle, 손상 MIR gate를 검증한다.
LLVM COFF/ELF O0/O2와 Windows Native 두 profile의 exact stdout/effect order/Copy/Abort, 기존 전체 회귀도 검증한다.

## 초안 단계 검증 기록

2026-10-07 문서 validator PASS: 원본 148개 hash, 링크, Draft ledger, 40-production 경계,
두 파일·부정 10사례의 UTF-8 Span/기대값 데이터 유효성을 확인했다. 무모호성 증명이나 Tuple 실행 검증은 아니다.
기존 P01~P12의 기본 workspace 252개 tests, fmt/Runtime rustfmt/clippy(-D warnings)/all-features check를 통과했다.
사용자 안내의 기존 `examples/structs.nova` check/debug/release를 실제 실행해 P12 기대 출력과 exit 0을 확인했다.
이번 변경은 문서/문서 도구뿐이므로 opt-in LLVM/Native 전체 40개를 다시 실행하지 않았다.
P13 Compiler 테스트와 Tuple Native 실행은 구현 전이므로 통과했다고 보고하지 않는다.

## 승인 요청·미포함 범위

사용자 개발 지침: “사용자의 승인을 받기 전에는 해당 사양 변경을 적용하지 마십시오.”
승인 요청은 P13 Copy Tuple 생성/타입·structural identity·numeric projection의 token subspan 처리·혼합 가변 경로·const·
P12 field 확장·private ABI·자원 제한이다. Array·Enum/match·destructuring·String element·일반 Move/borrow·init/Drop·
전체 D09/D10/D12/D16 승인은 포함하지 않는다. 승인 전에는 Compiler/accepted ledger에 적용하지 않는다.
