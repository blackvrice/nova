# Stage B Copy struct 최소 계약 — P12

작성일: 2026-10-05. 상태: **Draft / 사용자 승인 대기**.
아래 내용은 새 의미 규칙의 제안이며 Compiler에 적용하지 않았다.
D01~D05/P01~P11과 원본 Canonical을 보존한다. 전체 Stage B 또는 D06/D10/D12/D16/D30 승인이 아니다.

## Specification Change Proposal

- 관련 기준: [Canonical](CANONICAL.md), [원본 Struct 선언](../03_Types_Declarations/NOVA-027_Struct_Class_Enum_선언_사양서.md),
  [원본 초기화](../03_Types_Declarations/NOVA-031_생성자_필드_초기화_사양서.md),
  [원본 layout](../03_Types_Declarations/NOVA-033_타입_레이아웃_정렬_Niche_사양서.md),
  [원본 namespace](../02_Names_Modules/NOVA-023_Package_간_이름_해석_사양서.md),
  [Copy 초안](specs/NOVA-051.md), [결정 초안](DECISIONS.md), [P11](MODULE_STAGE_B_PROPOSAL.md).
- 현재 확정: Struct는 값 의미이며 field는 let/var를 명시한다. 직접 재귀 값 layout은 금지한다.
  type/value/module namespace는 분리한다. Parser와 Backend는 아직 struct를 지원하지 않는다.
- 빈 부분: 생성 표기, Copy 판정, projection의 가변성, type import, const payload와 private Native ABI.
- 제안: 아래 Copy field로만 구성된 nominal struct를 추가하고 위치 인수 생성·field 읽기/대입·const·Native를 연결한다.
- 이유: Stage C의 일반 Move/borrow/Drop을 앞당기지 않고 Stage B aggregate의 값 복사와 layout을 검증한다.
- 영향: Parser/AST/HIR, Resolver/type namespace, Types/TypeChecker/const, MIR/validator, LLVM/CLI와 테스트.
- 호환성: 기존 primitive와 String, 함수/const import, checked 숫자, END, entry, CLI exit code를 유지한다.
  struct 없는 소스의 의미와 결정적 dump/snapshot은 유지한다. Lexer keyword/operator를 추가하지 않는다.
- 대안: 명시적 init와 전체 field 초기화 분석을 함께 구현하거나 struct literal을 도입할 수 있다.
  이번에는 생성된 위치 인수 생성자를 제안한다. 전체 Draft의 init/default field 규칙은 승인하지 않는다.

## 범위와 구문

전용 [37-production EBNF](GRAMMAR_STAGE_B_STRUCT.ebnf)는 P11의 program/assignment/postfix_expr만 수정하고
struct_decl/field_decl/field_path를 추가한다. 기존 newline/END·주석·Unicode 식별자 규칙을 유지한다.

```nova
public struct Pair {
    public var x: int
    public let y: int
}
func add(p: Pair) -> Pair { return Pair(p.x + 1, p.y) }
func main() {
    var p = Pair(20, 2)
    let before = p
    p.x = p.x + 1
    print("old={before.x}, new={p.x}")
}
```

1. top-level `[public|internal|private] struct Name { fields }`만 지원한다. 빈 struct도 허용한다.
   field는 `[visibility] let|var name: type`이며 초기값은 없으며 기존 END/닫는 brace 경계로 종료한다.
   field initializer, local struct, method, init/drop, self, generic/implements는 N1102로 거부한다.
   field 중복은 N2002이며 이전 선언을 secondary로 표시한다.
2. 허용 field: 8종 정수·Float32/64·Bool·Char·Unit 또는 다른 P12 struct. 기존 primitive alias를 사용할 수 있다.
   Unit 표기는 기존 `()`/void 규칙이다. String을 포함한 Move field는 이번에는 N1102다.
   미정의 type은 N2001, 이미 오류인 field type의 layout/constructor 파생 오류는 억제한다.
3. 모든 field가 위 범위인 acyclic struct는 Copy다. nominal identity는 원 StructDefId이며
   동일 field 구성이나 이름이라도 다른 선언 사이의 대입/인수/return은 N2101이다. alias는 동일 원 ID다.
   whole value 복사, 지역 binding/대입, 함수 인수/return, const 사용은 독립된 값이다.
   복사한 값을 변경해 원본을 변경할 수 없다. 부분 Move나 사용자 Copy/Drop 구현은 없다.
4. String field, Class/Enum/Tuple/Array/Option/Result, reference/borrow/read/take/change,
   named argument·spread·struct literal, 상속/interface, FFI·public ABI는 후속이다.
   struct 전체의 산술·비교·보간·숫자 cast는 N2101이다. scalar field를 꺼낸 뒤에는 기존 규칙을 적용한다.

## Type namespace·import·visibility

1. bundle의 모든 struct 선언을 먼저 수집한다. field/함수 signature/const annotation은 선언 순서와 무관하게
   type namespace에서 해석한다. type명과 값명은 같은 spelling을 가질 수 있다.
   동일 type namespace의 선언/import alias 중복은 N2002다. primitive와 기존 type alias의 재정의도 N2002다.
2. P11의 `use geometry::Pair as P`에 직접 struct item import를 추가한다. 원 StructDefId와 정의 파일/Span을 보존한다.
   module alias·qualified value·reexport·Package는 추가하지 않는다. type import만으로도 파일이 reachable하다.
3. 같은 module item spelling에 type와 value 선언이 모두 존재하면 하나의 use는 두 namespace에 같은 alias로
   각각의 원 ID를 import한다. 접근 권한과 중복을 두 namespace에서 검증하며, 하나라도 실패하면 해당 use의
   binding을 둘 다 설치하지 않는다. type만 import할 수 있는 별도 문법은 후속이다.
   이는 namespace를 합치거나 임의 후보를 선택하는 동작이 아니다. 기존 value-only import는 그대로다.
4. `Name(args)`는 먼저 기존 lexical value를 검색한다. 값 binding이 있으면 기존 함수 호출 규칙을 따른다.
   non-callable 값이면 N2101이며 type 생성자로 재시도하지 않는다. 값 binding이 없을 때만 type namespace의
   struct를 생성한다. 생성자는 bare IDENT 호출로만 선택하며 first-class 함수 값이 아니다.
   annotation의 Name은 항상 type namespace다. builtin print shadow 정책은 P02/P06/P11을 유지한다.
5. struct와 field의 기본 visibility는 internal이다. internal은 현재 compilation bundle, private은 정의 module,
   public은 현재 bundle에서 접근 가능하며 미래 Package export 의미는 미정이다.
   명시적 type name/import와 각 field 접근은 각각 visibility를 검사한다. 위반은 N2004다.
   외부 module의 위치 인수 생성은 type와 모든 field에 접근 가능해야 한다. private field가 있으면 정의 module의
   factory 함수가 값을 생성하여 반환할 수 있다.
6. 함수 signature에 private type이 포함되는 것을 별도로 금지하는 API leak 규칙은 추가하지 않는다.
   caller의 추론된 값은 원 nominal ID를 유지하며 Copy하거나 같은 타입의 함수에 전달할 수 있다.
   이는 private type name을 import할 권한을 주지 않는다. field 접근에는 해당 field의 visibility를 적용한다.
   향후 Package API 검사 규칙은 별도 승인 대상이다.

## 생성·field 읽기·대입

1. `Pair(a, b)`는 선언 field 순서대로 정확히 한 값을 요구한다. 빈 struct는 `Empty()`다.
   부족/초과 인수는 N2201, 각 인수 타입 불일치는 N2101이다. 미초기화·자동 zero/default 초기화는 없다.
   인수는 source order로 한 번씩 평가하며 각 field의 기대 타입으로 P07/P09 literal과 lossless 승격을 적용한다.
   숫자 narrowing은 P10의 명시적 as를 요구한다. 중첩 struct 인수는 nominal identity가 같아야 한다.
2. `expr.field`는 call/as와 같은 postfix 층에서 왼쪽 결합한다. receiver를 한 번 평가한 뒤 Copy field를 읽는다.
   `make().x`, `box.pair.x`를 허용한다. 미정의 field는 N2001, non-struct receiver는 N2101이다.
   private field는 N2004이며 primary는 접근 field 이름, secondary는 정의 field다.
   `a as T.x`는 `(a as T).x`다. 타입 T가 숫자이면 field 접근 오류이며 qualified type 문법이 아니다.
3. 대입 target은 bare 지역 이름 뒤에 0개 이상의 `.field`가 붙은 경로만 허용한다.
   root는 local var여야 하고 경로의 모든 field가 var여야 한다. local let/const·함수 parameter 또는
   하나라도 let field를 통과하면 N3004다. var root라도 let field의 내부를 수정할 수 없다.
   `make().x = ...`, `(p).x = ...`, indexing과 compound assignment는 N1102다.
4. target ID/path/type/visibility를 정적으로 검증하고 RHS를 한 번 평가한 다음 저장한다.
   RHS는 저장 전 root를 읽을 수 있다(`p.x = p.x + 1`). path 중간에 runtime 호출/인수 평가가 없다.
   whole assignment는 기존 P04 규칙과 독립 Copy를 따른다. root/field 오류 뒤의 파생 타입 진단은 억제한다.

## Const·layout·자원 제한

1. 지역/전역 const에 순수 struct 생성과 field projection을 추가한다. 사용자 함수 호출은 계속 금지한다.
   ConstValue에는 원 struct ID와 선언 순서의 완전 초기화된 field 값을 보관한다.
   선택하지 않은 field도 생성 시 모두 평가한다. scalar checked arithmetic/cast 실패는 기존 N3201이다.
2. initializer당 10,000-node budget을 유지한다. 생성 expression은 1 node + 실제 평가한 인수 expression,
   projection은 1 node + receiver expression이다. 생성자 head 이름과 field identifier는 별도 expression node가 아니다.
   cached const reference는 payload 크기와 무관하게 기존대로 1 node다. logical RHS short-circuit은 유지한다.
   static 전역 const dependency는 모든 생성 인수와 생략 logical RHS를 검사하며 순환은 기존 N3202다.
3. 직접/간접 by-value field cycle은 사용 여부와 무관하게 N2101이다. primary는 cycle을 닫는 field의 type,
   secondary는 해당 cycle의 다른 field type을 표시한다. Unit/빈 struct는 indirection이 아니다.
   cycle 탐색과 layout 계산은 반복형 graph 처리로 선언 순서에 의존하지 않는다.
4. layout은 정의 field 순서, 자연 alignment와 padding을 사용하며 전체 size는 최대 field alignment의 배수다.
   target은 기존 Windows x64 Native와 Linux x64 Object다. 정수 size/alignment는 width/8,
   Float32=4/4, Float64=8/8, Bool=1/1, Char=4/4, Unit와 빈 struct=0/1이다.
   중첩 field는 해당 struct size/alignment를 사용한다. Unit/빈 field도 논리적 field ID/초기화 검사를 유지한다.
   pointer identity/sizeof/serialization/packed layout·C ABI를 언어에 노출하지 않는다.
5. bundle당 struct 최대 1,024개, struct당 직접 field 최대 1,024개, by-value 의존 깊이 최대 128
   (빈/primitive-only struct의 깊이 1), target layout size 최대 1,048,576 byte를 제안한다.
   zero-size field의 지수적 payload 확장을 막기 위해 타입별 transitive field occurrence 총수도 65,536 이하로 제한한다.
   `count(S) = sum(1 + count(field struct type))`이고 primitive/Unit의 child count는 0이다.
   checked 합/곱/align 계산을 쓰며 limit/overflow는 N8901이다. 크기/count/depth 오류는 struct 이름이 primary이고
   한도 초과를 만든 field를 secondary로 표시한다. cycle 오류를 먼저 처리하고 같은 타입의 후속 layout 오류는 억제한다.
   기존 1,024-module, parser 중첩 128과 const budget은 별도이며 바뀌지 않는다.
6. 상한은 compiler 작업량 계약이며 임의 runtime stack 크기를 보장하는 계약이 아니다.
   nested const 복사도 위 확장 상한을 적용하며 malformed payload와 host stack overflow를 방치하지 않는다.

## MIR·Native 계약

- Core에 StructDefId/FieldId와 nominal type 정보를 추가하되 LLVM 자료형을 포함하지 않는다.
  AST/HIR source origin은 원 파일을 보존하며 합성 constructor도 type 정의와 call site를 구분한다.
- MIR은 aggregate 생성·field projection·local root와 field path의 Place를 표현한다. 모든 field 초기화,
  원 ID, 인수 수/타입, field path/가변성, conversion, function signature, layout metadata를 독립 검증한다.
  오류 타입·미해결 field·const 오류가 있는 bundle은 CodegenUnit을 만들 수 없다.
  HIR/Resolve/Checked와 MIR의 불일치는 user error가 아닌 기존 ICE 경계다.
- 내부 struct ABI는 caller-owned 독립 snapshot을 가리키는 private 간접 인수와 caller-owned out pointer 반환이다.
  callee는 struct parameter를 읽기 전용으로 취급한다. call 이전의 Copy와 out 저장으로 외부 값에 write-back하지 않는다.
  scalar parameter/return ABI는 유지한다. user borrow/pointer ABI나 foreign interface를 공개하지 않는다.
- Adapter는 DefId별 고유 struct type/symbol과 field offset/layout을 사용한다. logical Unit/빈 struct의 Copy/생성도
  유효하며 실제 byte copy는 0 byte다. padding은 언어 값이 아니며 padding으로 비교/출력하지 않는다.
  O0/O2가 같은 값·평가 순서·checked 실패 위치를 보존해야 한다. 일반 escape optimization/Drop은 추가하지 않는다.
- CLI 옵션·source root·entry main·artifact gate·exit 0/1/2/3/101은 P03/P11 그대로다.

## 진단·수용 기준

| 상황 | 코드 | 위치/추가 정보 |
|---|---|---|
| 잘못된 구문 / 범위 밖 member·field type | N1101 / N1102 | 오류 token/type, 이후 선언 복구 |
| 중복 type/field/import | N2002 | 새 이름 + 이전 정의 secondary |
| 미정의 type/field | N2001 | 원 파일의 해당 이름 |
| private type/field/생성 | N2004 | 접근 token + 정의 secondary |
| nominal/인수/return/연산 타입 오류, layout cycle | N2101 | expression/field type + 기대 타입/순환 근거 |
| 생성 인수 개수 | N2201 | 전체 호출 + struct 정의 secondary |
| 변경 불가 root/path | N3004 | target의 첫 변경 불가 이름 + 정의 secondary |
| checked const 실패 / const cycle·budget | N3201 / N3202 | 기존 P05/P06/P10 Span 규칙 |
| type/field/depth/layout/count 상한 | N8901 | 초과 선언 + 한도 note/field secondary |

[두 파일 수용 예제와 부정 사례](struct-proposal-fixtures/README.md)는 Draft 기대값이다.
현재 compiler 실행 증거나 승인된 conformance가 아니다. 구현 완료에는 다음 독립 검증이 필요하다.

1. Parser의 field END/Unicode/주석·잘린 입력 복구·postfix 결합·제한 target과 기존 snapshot 회귀.
2. type/value 분리·예약 primitive·forward type·alias 원 ID·동명 type/value 원자 import·private 생성/추론 값 검사.
3. 서로 다른 nominal 타입·중첩 Copy 독립성·let 경로·함수 전달/return·left-to-right 단일 평가·literal 승격.
4. const 생성/읽기·cross-file 순환·skipped RHS·10,000-node 경계·확장 payload와 alias cache 검증.
5. 미사용 recursive type·size/alignment/padding·Unit/빈 field·깊이/count/size 경계와 무해한 N8901.
6. 손상된 MIR struct ID/field/path/인수/초기화/layout/signature 거부와 Codegen gate.
7. LLVM 실제 verify와 Windows COFF/Linux ELF Object의 O0/O2, Windows Native byte-exact 출력·복사·checked 실패.
   Linux Native 실행은 현 host에서 검증했다고 주장하지 않는다.
8. cargo fmt/clippy/workspace test/all-features check, Runtime fmt와 문서 validator.

## 이번 초안의 검증 기록

2026-10-05 현재 초안 단계에서 다음 명령이 PASS다.

```text
node tools/docs/build-pack.mjs
node tools/docs/validate-pack.mjs
cargo fmt --check
rustfmt --check --edition 2021 crates/nova-cli/runtime/stage_a.rs
cargo clippy --workspace --all-targets --offline -- -D warnings
cargo test --workspace --offline
cargo check --workspace --all-features --offline
```

[문서 검증 결과](VALIDATION.md)는 원본 148개 hash, 37-production grammar 경계와 Draft fixture 데이터를 확인한다.
Rust 검사는 기존 P01~P11 구현의 회귀 검증이다. ignored LLVM/Native 테스트는 이번 문서 변경에서 재실행하지 않았다.
P12 Parser/타입/MIR/Native 수용 테스트는 구현 전이므로 통과했다고 보고하지 않는다.

## 승인 요청과 미포함 범위

사용자의 붙여넣은 개발 지침: “사용자의 승인을 받기 전에는 해당 사양 변경을 적용하지 마십시오.”
승인 요청은 **P12의 Copy struct·위치 인수 생성·type import·field 읽기/가변 경로·const·private ABI·명시된 제한**이다.
승인 후 이 subset을 구현한다. 전체 D06/D10/D12/D16/D30이나 String field·명시적 init/Drop·일반 borrow/Move,
Enum/Tuple/Array/Class·Package·public ABI는 승인 대상에 포함하지 않는다.
