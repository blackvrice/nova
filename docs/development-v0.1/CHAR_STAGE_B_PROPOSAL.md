# Stage B char·Unicode scalar·보간 착수안 — P08

작성일: 2026-10-04. 상태: **Accepted / 2026-10-04 사용자 승인**.
사용자 답변 “P08 승인하고 char 구현 진행”에 따라 아래 subset을 승인했다.
현재 Compiler 적용과 검증 증거는 [P08 구현 기록](CHAR_IMPLEMENTATION.md)을 따른다. P01~P07의 승인·구현 상태를 보존한다.
Primitive 확장의 작은 단계이며 float/cast·전체 D07/Stage B의 동결안은 아니다.

## Specification Change Proposal

- 관련 문서: [Canonical NOVA-004](../00_Governance/NOVA-004_Nova_용어_키워드_Canonical_표.md),
  [Literal NOVA-010](../01_Source_Syntax/NOVA-010_숫자_문자_문자열_Literal_사양서.md),
  [타입 NOVA-025](../03_Types_Declarations/NOVA-025_타입_시스템_타입_추론_형변환_사양서.md),
  [Primitive NOVA-026](../03_Types_Declarations/NOVA-026_Primitive_Literal_기본_타입_결정_사양서.md),
  [승인 D04 Lexer](ACCEPTED_LEXER.md), [D07](DECISIONS.md), [P07 구현](INTEGER_IMPLEMENTATION.md),
  [테스트 NOVA-136](../13_Testing_Release/NOVA-136_Compiler_테스트_전략서.md).
- 승인 전 사양: char는 Canonical Primitive 이름이며 D04는 한 Unicode scalar의 character token과
  escape를 승인했다. 승인 전 Compiler는 character token 및 char type를 N1102로 거부했다.
  char의 실행 비교·const·보간·내부 ABI 상세는 동결하지 않았다.
- 발견된 문제: UTF-8 token을 수용하는 것만으로 타입/값·함수·const·MIR/Native가 지원되지는 않는다.
  정수 i32 표현과 char 의미를 혼동하거나 string의 brace decode를 재사용하면 잘못된 문자를 수용·거부할 수 있다.
- 제안 변경: 아래 char 타입/리터럴, scalar 비교, 기존 binding/call/return/대입·const와
  UTF-8 보간을 연결한다. private Native scalar ABI와 Runtime formatter를 명시한다.
- 변경 이유: aggregate/module 전 Primitive의 문자 값을 단일 파일 전체 pipeline에서 처리한다.
  이미 승인된 문자 token/escape와 기존 const/제어 흐름/Runtime을 사용한다.
- 영향 범위: 전용 EBNF의 primary, AST·Parser·HIR leaf, Types·TypeChecker·const engine,
  MIR Constant/validator, LLVM Adapter와 private Runtime, pass/fail/Native tests·문서.
  Lexer 철자·escape·END 정책은 변경하지 않는다.
- Backward Compatibility: 기존 정수/Bool/String/Unit의 표현·연산·출력·panic·const budget을 유지한다.
  미지원인 char를 추가하며 int/uint/byte/string으로 자동 변환하지 않는다.
  Rust Core API에는 char variant를 추가할 수 있지만 public Nova ABI/FFI 안정성을 약속하지 않는다.
- 대안: 먼저 char를 literal/타입 검사까지만 수용하면 유효한 char 프로그램이 Native에서 다시 막힌다.
  float와 함께 동결하면 IEEE rounding/NaN/출력·숫자 문맥까지 확대되므로 float는 별도 후속으로 남긴다.

## 타입·값·문자 철자

1. `char`는 정확히 한 Unicode scalar다. 허용 값은 U+0000~U+D7FF와 U+E000~U+10FFFF다.
   surrogate U+D800~U+DFFF는 포함하지 않는다. ASCII byte/UTF-16 code unit/grapheme와 구분한다.
   scalar 범위는 [Unicode의 정의](https://www.unicode.org/glossary/#unicode_scalar_value)를 따른다.
2. unassigned/private-use/noncharacter code point도 위 scalar 범위 안이면 허용한다.
   identifier XID의 Unicode 18.0.0 분류표는 char 값의 허용 여부를 결정하지 않는다.
   NFC/NFD·locale/case folding을 수행하지 않는다.
3. 기존 character token `'...'`과 D04의 escape `\n \r \t \0 \\ \" \'`,
   `\u{1~6 hex digits}`를 그대로 사용한다. token의 원본 byte Span은 양쪽 quote를 포함한다.
4. 직접 표기/escape decode 결과가 한 scalar여야 한다. `'가'`, `'🙂'`, `'\u{1F642}'`, `'\0'`는 허용한다.
   `''`, `'ab'`, `'é'`(e와 combining accent 두 scalar), surrogate/out-of-range escape는 Lexer 오류다.
   한 scalar인 `'é'`와 두 scalar인 `'é'`를 normalization으로 서로 바꾸지 않는다.
5. character literal 안의 `'{'`, `'}'`는 그대로 한 문자다. 보간·brace doubling을 적용하지 않는다.
   `'{{'`/`'}}'`는 두 scalar이므로 실패한다. String의 `{{`/`}}` decode 규칙을 char에 적용하지 않는다.
   quote/backslash 및 brace가 보간 expression 안에 있어도 character token 경계를 유지한다.
6. raw newline을 character literal에 넣지 않는다. 줄바꿈 값은 `'\n'`/`'\r'`로 표기한다.
   malformed escape/길이는 기존 N1002, EOF의 닫히지 않은 quote는 기존 N1003이다.
   후속 declaration을 복구하는 Lexer/Parser 정책과 진단 위치는 D04/P01을 보존한다.
7. Rust Core의 유효한 `char` 또는 동일 invariant를 가진 값 타입을 사용한다.
   [Rust char의 scalar invariant](https://doc.rust-lang.org/std/primitive.char.html#validity-and-layout)는
   구현 검증에 활용하되 Rust layout이 public Nova FFI를 결정하지 않는다.
   잘못된 public AST/변조된 분석 자료는 기존 API 오류로 거부하고 unchecked char 생성은 사용하지 않는다.

## 타입 검사와 연산

- char literal의 타입은 항상 Char다. 기대 타입으로 integer/String/Bool을 재해석하지 않는다.
  `let x='A'`, `let x:char='A'`, `var x:char='가'`, local/global `const`를 허용한다.
- annotation, 직접 이름 대입, positional 인수, return에서 char→char identity만 허용한다.
  이름/함수의 원래 타입과 SourceInfo를 유지한다. P07 integer coercion 대상에 Char를 포함하지 않는다.
- char↔integer/byte/Bool/String의 implicit 변환은 없다. `'A'==65`, `let x:int='A'`,
  `let x:char=65`, `let x:string='A'`, char와 String equality는 N2101이다.
- `== != < <= > >=`는 양쪽 Char일 때만 지원한다. equality는 scalar 값의 동일성,
  순서는 scalar의 수치 순서다. 언어별 collation/locale, UTF-8 byte 수·UTF-16 surrogate 순서는 사용하지 않는다.
  비교의 결과는 Bool이며 comparison chain 금지는 기존 P01을 따른다.
- `+ - * / %`, unary `+ - !`, `&& ||`의 char operand는 N2101이다.
  문자 덧셈으로 문자열을 만들거나 code point를 증가시키는 기능은 추가하지 않는다.
- char 조건의 `if/while`은 기존 Bool 조건 N3001이다. implicit truthiness는 없다.
- Char 자체의 escape/메서드/인덱싱/정수 cast·범위 iteration API와 String equality 확장은 후속이다.

## local/global const

1. `ConstValue::Char` 또는 동등한 scalar 값 variant를 추가한다. char literal, grouping,
   const name 참조 및 위 char 비교를 P05 제한 표현식의 허용 범위에 넣는다.
2. local/global const, forward dependency와 cache, type annotation의 identity는 같은 계약이다.
   mutable var/일반 let 참조·call/print/보간 실행은 계속 const initializer에서 불허한다.
   Runtime char 보간 지원은 const String 보간 허용이 아니다.
3. char 비교는 실패하지 않으며 Bool을 반환한다. skipped logical RHS도 permission/type을 검사하고
   실제 Bool short-circuit과 정수 checked 실패·N3201을 유지한다.
4. initializer당 10,000 HIR expression nodes와 N3202, const reference를 한 node로 세는 정책,
   P06의 모든 정적 참조를 포함한 순환 N3202/오류 전파를 그대로 사용한다.
5. 전역 const print는 N2002, 사용자 함수 print/local shadow는 P02/P06대로 보존한다.
   const function·새 전역 초기화 실행은 추가하지 않는다.

## MIR·LLVM·private Runtime

- HIR은 source char 철자에서 검증·decode한 한 scalar와 원본 Span/SourceOrigin을 보존한다.
  문자열 값과 문자 값은 다른 HIR/Type/ConstValue variant다.
- MIR에는 Char Constant를 추가하고 기존 Use/Place/Call/Return와 char 비교를 사용한다.
  MIR `Widen`은 integer 전용이며 Char를 허용하지 않는다.
  독립 validator는 char assignment/call/return의 정확한 타입, char 비교와 초기화 read를 검사한다.
  integer와 같은 저장 폭이라고 Char 산술·integer mixing을 수용하지 않는다.
- 기존 Resolver/Checked 정확한 재계산 gate로 type/value/const metadata 변조를 거부한다.
  ErrorNode/Type와 유효하지 않은 자료가 LLVM 입력에 도달하지 않는다.
- Windows x64/Linux x64의 이 단계 private scalar ABI에서는 char를 code point 값의 i32로 표현한다.
  함수 인수/return/local도 i32이며 scalar 값은 unsigned 비교 `ult/ule/ugt/uge`, equality는 `eq/ne`다.
  LLVM i32가 Core Char의 범위를 정의하지 않는다. 외부 C char/wchar_t/public layout 계약은 동결하지 않는다.
- private C Runtime symbol `nova_format_char(out, u32, file, start, end)`를 추가한다.
  out은 기존 NovaString pointer, location 세 필드는 기존 u32다. 선언의 LLVM value 인수는 i32다.
  formatter는 검증된 scalar의 UTF-8 bytes 1~4개를 기존 arena에 보존한다.
- 유효한 char는 보간에서 그 UTF-8 문자 자체로 출력한다. quote/escape/code point decimal을 출력하지 않는다.
  NUL은 byte 0, newline/tab은 해당 제어 byte다. 추가 normalization·locale·replacement character는 없다.
  `print(string)`의 LF/flush, String arena lifetime과 allocation/I/O Abort 정책을 유지한다.
- frontend가 생성한 char는 유효하므로 새로운 사용자 산술 panic은 없다.
  손상된 private Runtime u32 인수가 scalar가 아니면 SourceInfo와 `invalid char scalar`로 Abort한다.
  unchecked scalar 변환·invalid UTF-8 생성은 하지 않는다. 기존 panic reason 1/2/3은 보존한다.
- `print('A')`는 여전히 N2101이다. char의 String 사용은 기존 보간 `print("{'A'}")`을 통해 가능하다.
  새로운 overloaded print/자동 stringify API를 승인한 것으로 해석하지 않는다.

## 문법·진단 경계

[검토용 전용 EBNF](GRAMMAR_STAGE_B_CHAR.ebnf)는 P06의 31-production 문법에서
`primary_expr`에 기존 Lexer terminal `CHAR`만 추가한다. type는 기존 IDENT다.
production 수·precedence·END·binding/global const grammar는 유지한다.
기존 accepted EBNF/P01~P07은 수정하지 않는다. 승인 후 Parser는 이 P08 subset을 사용한다.
전체 [GRAMMAR](GRAMMAR.ebnf)의 FLOAT/cast/미래 구문을 승인한 것이 아니다.

| 상황 | 코드 | 위치 |
|---|---|---|
| 비어 있거나 여러 scalar·invalid escape·scalar 범위 오류 | 기존 N1002 | 기존 Lexer literal/escape byte Span |
| EOF의 닫히지 않은 character quote | 기존 N1003 | opening quote / EOF label |
| char의 문법 오류 | 기존 N1101/N1103 | 기존 Parser 복구 위치 |
| 비지원 float/cast 등 | 기존 N1102 | 미지원 construct |
| char와 다른 타입의 변환/연산·인수/return 불일치 | N2101 | expression/operation / 기대 타입 선언 |
| 함수 arity | N2201 | call / 함수 선언 |
| char 조건 | N3001 | condition |
| 불변 char binding 대입 | N3004 | target / 선언 |
| const permission 실패 | N3201 | 불허 expression / const 선언 |
| const budget/cycle | N3202 | 기존 P05/P06 위치·note |

새 diagnostic code를 추가하지 않는다. Lexer가 거부한 문자에서 파생되는 type/const 오류를 억제한다.
오류 소스는 CLI check/build/run에서 LLVM 호출 및 output 생성 전에 source exit 1로 거부한다.

## 수용 예제와 검증 계획

아래 수용 예제는 [characters.nova](../../examples/characters.nova)로 구현했다. 실제 검증 범위는 구현 기록을 따른다.

```nova
const FACE: char = '\u{1F642}'
const LOW: char = '\u{D7FF}'
const HIGH: char = '\u{E000}'
const ORDERED = LOW < HIGH

func echo(value: char) -> char {
    return value
}

func main() {
    var letter: char = 'A'
    letter = '가'
    let brace = '{'
    print("letter={echo(letter)}, face={FACE}, brace={brace}, ordered={ORDERED}")
}
```

예상 stdout UTF-8 bytes는 `letter=가, face=🙂, brace={, ordered=true\n`, exit 0이다.
surrogate 간격 양끝의 순서, 인수/return·대입, const와 char brace 보간을 함께 확인한다.

1. Lexer의 기존 char/escape/END regression을 보존한다. ASCII/2·3·4-byte scalar, NUL/모든 escape,
   brace/quote/backslash, raw newline·empty/multi-scalar·invalid escape·EOF recovery의 정확한 code/Span을 검사한다.
2. Parser AST char leaf와 HIR decode/type/source를 검사한다. 유효한 token을 수용하되
   malformed public AST는 panic 없이 기존 API 오류로 거부한다. char 안의 brace는 String 보간과 구분한다.
3. U+0000/007F/0080/07FF/0800/D7FF/E000/FFFF/10000/10FFFF, noncharacter/unassigned를 검증한다.
   Core 값/UTF-8 encode는 전체 scalar 범위를 독립 boundary/range oracle과 대조한다.
   surrogate·110000 이상의 값은 construct/Runtime 경계에서 거부한다.
4. 6종 char 비교의 const/runtime 대조, 8종 정수 및 Bool/String/Unit과의 혼용 실패,
   call/return/assignment·조건·불변성·peer literal/P07 정수 regression을 검사한다.
5. local/global const·forward/cycle·10,000-node budget·skipped permission/type과
   raw type/const value/coercion 변조의 no-codegen, MIR Char/Widen/SourceInfo/initialization을 검사한다.
6. 결정적 LLVM IR과 실제 LLVM 21.1.8 O0/O2 Windows COFF/Linux ELF object를 검사한다.
   기존 Hello snapshots, private 함수 char ABI·비교·formatter와 old Runtime symbols를 검증한다.
7. Windows Native O0/O2의 예제 stdout/exit, UTF-8/NUL/제어 문자 정확한 bytes와 String lifetime,
   함수·mutable assignment·const 비교, 잘못된 source의 CLI no-tool/no-output을 검사한다.
   Runtime의 invalid scalar invariant 검사는 별도 test harness로 수행하며 사용자 cast 기능을 만들지 않는다.
8. Cargo fmt/clippy/workspace test/all-features check, standalone Runtime rustfmt와 문서 validator를 실행한다.
   grammar 검사 성공과 Compiler/Native 수용 테스트 통과는 별도로 기록한다.

## 승인 경계

승인 대상은 char leaf/type/value·scalar 비교, 기존 선언·대입·인수·return·const 연동,
UTF-8 보간·private i32 scalar ABI와 formatter, 전용 CHAR primary EBNF다.
float/never·source cast·char arithmetic·String equality/char indexing/Unicode collation API,
aggregate/module/const function·ownership/Drop·public FFI·Linux Native와 전체 D07은 후속이다.
이번 사용자 승인을 Compiler와 accepted ledger에 반영한다. 기존 accepted EBNF는 보존하고 P08 전용 EBNF를 추가한다.
