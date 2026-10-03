// Authored topic contracts. The generated Markdown is the reviewable deliverable.
export const topics = {
1: `## 목적과 판단 기준
Nova는 정적 타입의 Native 범용 언어다. 읽기 쉬운 문법, 메모리 안전성, 예측 가능한 실행 의미를 함께 목표로 한다. 정확성 > 컴파일러 정확성 > 유지보수 > 성능 > 구현 편의 순서로 판단한다.

## 설계 검토 계약
새 기능은 문제, 최소 예제, 반례, 문법 비용, 타입/소유권 영향, 진단, 도구 영향, 대안을 설명해야 한다. 기존 언어와 닮았다는 이유만으로 채택하지 않는다. 편의 구문도 하나의 HIR 의미로 설명할 수 있어야 한다.

## 검증과 완료
예제의 결과를 독자가 LLVM 지식 없이 설명할 수 있어야 한다. 숨은 복사, 예외 전파, 비결정적인 이름 해석을 점검한다. 목표의 수치 성능 기준은 NOVA-143에서 관리하고 의미를 바꿔 달성하지 않는다.`,
2: `## Stage별 납품 계약
| Stage | 구현 범위 | 통과 조건 |
|---|---|---|
| A | 단일 UTF-8 파일, 함수/호출, 문자열/정수, let/return/if, 출력, LLVM Object/Link | check 성공과 Hello Nova Native 실행 |
| B | var/const, Primitive/승격, Struct/Enum/Tuple/Array, Method/Constructor, 반복/Pattern/Match, Option/Result/try, Module | 기능별 pass/fail 및 분기/컨테이너 실행 |
| C | Copy/Move, Read/change/take, 초기화/NLL/Drop/View | 금지된 alias 거부와 정확히 한 번 Drop |
| D | Class, Interface, Generic, Closure | 구체화/정적 호출 및 capture 검증 |
| E | Package/Lock/Cache, Formatter, C FFI, Windows/Linux, Core std | 재현 빌드·설치·C 연동 |

Stage B는 언어 의미를 생략한 채 Move 프로그램을 허용하는 단계가 아니다. 소유권 검사가 필요한 값은 C가 준비될 때까지 구현 부분집합에서 제한한다. 핵심 순서는 유지하되 의존성 때문에 C/D가 필요한 라이브러리는 함께 완성한다.

## 완료 증거
각 기능은 사양 절, Fixture ID, 실제 명령 결과, 지원 Target을 연결한다. 아직 구현되지 않은 기능은 current-stage 진단으로 거부하며 release 지원표와 구현 현황을 분리한다.`,
3: `## 명시적 제외
Class 구현 상속, Dynamic Interface Object, Associated Type, async/await, Coroutine/Generator, Pinning/Self-reference, Panic Unwind, Reflection, Const Generic, 명시적 Generic 호출 인수, 사용자 Operator/Property, Registry 서버, Self-hosting은 0.1 제외다.

## 거부 계약
Lexer가 인식 가능한 표기는 소스 Span을 보존하고 Parser 또는 의미 단계에서 기능명을 포함한 진단을 준다. 제외 기능을 Rust/C++ 의미로 추정해 실행하지 않는다. 후속 예약 후보를 실제 예약어로 지정하는 것은 D01 결정 사항이다.

## 검증
상속 선언, interface 값 저장, f<int>(x), await 호출 각각을 fail fixture로 둔다. 특히 f<int>(x)를 비교식으로 조용히 해석하여 성공시키지 않는다. 미래 설계 문서는 구현 완료 목록에 포함하지 않는다.`,
4: `## 확정 어휘
공식 키워드와 Primitive 이름은 원본 NOVA-004 및 canonical_decisions.json을 그대로 따른다. func, foreign, interface, change, take 표기를 일관되게 쓴다. Read는 modifier 부재의 의미이며 별도 소스 키워드가 아니다.

## 정규화
byte=uint8, int=int32, uint=uint32, float=float32, double=float64, void/생략 반환=Unit, ()=Unit 값, T?=Option<T>. 내부 명칭은 Unit/Read/Change/Take를 사용하고 소스 출력에서는 원본 철자를 유지한다.

## 누락과 추가 제안
use는 NOVA-072에 있으나 공식 목록에 없으므로 D01로 추적한다. type 별칭 구문, noPanic, library 및 @symbol의 분류도 D01/D11/D17 검토 전 확정 키워드로 추가하지 않는다. 이름 스타일은 lint 대상이며 문법적 거부 여부는 별도 결정한다.

## 완료
lexer keyword 표, formatter 출력, 사용자 문서의 어휘가 같은 표에서 생성되는지 검사한다.`,
5: `## 문서 의존 방향
Governance → Source/Syntax → Names/Types → Functions/Control → Ownership/Abstraction → Compiler IR → Backend/Runtime → FFI/Library → Tooling → Release 순서로 읽는다. 구현 순서는 별도 ROADMAP을 따른다.

## 문서 유형
언어 문서는 관찰 가능한 행동과 거부 조건을 정의한다. 구현 문서는 자료구조와 단계 경계를 정의한다. API 문서는 모드/오류/비용을 정의한다. 테스트 문서는 각 요구사항의 관측 방법을 정의한다.

## 추적 계약
INDEX의 각 NOVA ID는 원본, 보완 문서, Stage와 연결된다. DECISIONS의 D번호는 여러 문서가 공유하는 미확정 의미의 단일 출처다. CONFORMANCE의 T번호는 통과 기준이며 아직 실행된 테스트라는 뜻이 아니다.

## 검증
중복 ID, 누락 문서, 끊어진 링크, EBNF 미정의 nonterminal, 존재하지 않는 D/T 참조는 문서 검증 실패다.`,
6: `## 변경 절차
의미 변경은 제안 → 검토 → 승인 → 문서 동결 → 구현 → 회귀 테스트 순서다. 상태는 Draft, Accepted, Rejected, Superseded이며 작성 날짜와 승인 증거를 기록한다.

## 제안 필드
관련 NOVA ID, 현재 사양, 문제/반례, 제안 의미, 문법 예제, 영향 계층, 하위 호환성, 대안, 수용 테스트, 승인 기록을 필수로 한다. DECISIONS는 이 형식의 묶음이며 승인 기록이 없는 항목은 Draft다.

## 충돌 처리
Canonical > Governance/Freeze > Language > Architecture > Implementation > Code > Tests. 같은 우선순위의 충돌은 임의로 선택하지 않고 blocking issue로 기록한다. 구현 편의의 로컬 예외를 사양으로 승격하지 않는다.

## 완료
승인된 규칙의 문서/grammar/fixture가 함께 변경되어야 한다. 이번 보완팩은 원본 변경 없이 검토 가능한 초안을 제공한다.`,
7: `## 버전 경계
언어 버전, compiler 버전, Runtime ABI 버전, Package schema, Artifact schema, Cache schema를 독립 필드로 관리한다. 언어 버전 0.1이라는 이유로 서로 다른 compiler artifact를 재사용하지 않는다.

## 호환성 초안 — D29
0.1.x에서 기존 정상 프로그램의 타입/실행 의미를 바꾸면 breaking으로 취급한다. 명백한 버그 수정은 반례와 migration note를 남긴다. 진단 코드 삭제/재사용, 공용 std signature 변경, lock schema 변경도 호환성 검토 대상이다.

## 검증
기준 버전 corpus를 새 compiler로 check/run하고 AST 문자열보다 정상/거부 판정과 stdout/exit/Drop trace를 비교한다. ABI 호환성이 보장되지 않으면 runtime과 패키지를 다시 컴파일한다.`,
8: `## 소스 계약
UTF-8 파일과 반열린 바이트 위치를 사용한다. 잘못된 UTF-8은 교체 문자로 조용히 바꾸지 않고 입력 오류로 보고한다. 디스크 원문과 진단 byte offset은 동일해야 한다.

## 상세 초안 — D02
LF/CRLF/CR을 논리 NewLine으로 인식하되 원본 길이는 보존한다. 첫 UTF-8 BOM은 trivia로 허용하고 중간 BOM은 유효한 공백으로 취급하지 않는다. Identifier는 고정 Unicode 버전의 XID_Start/XID_Continue와 밑줄을 제안한다. NFC 변환은 하지 않고 철자 그대로 비교한다.

## 검증
한글/emoji/결합문자, CRLF 직후 Span, 빈 파일/마지막 newline/EOF, 잘못된 byte sequence를 다룬다. Compiler의 scalar 열과 LSP UTF-16 열을 별도 변환한다. Unicode 버전은 toolchain lock에 고정하며 D02 승인 전 lexer 식별자 정책을 확정하지 않는다.`,
9: `## Token 모델
Token은 kind, Span, 원문 slice를 가진다. Identifier/Keyword, Integer/Float/String/Char, 보간 경계, 구분자, Operator, NewLine, Trivia, Error, EOF를 분리한다. Trivia까지 포함한 원문 연결이 원본과 같아야 한다.

## Operator 초안 — D03
기본 집합은 + - * / % ! && || == != < <= > >= = 및 -> => ? . , : ; ( ) [ ] { }다. until/through/as/exists는 단어 연산자다. Compound assignment, bitwise, shift, pointer 철자는 D03에서 승인 전 제외/보류한다.

## 규칙
최장 일치 후 정확한 키워드를 판정한다. 비교 연산자보다 긴 =>/->를 우선하고 숫자 부호는 prefix operator로 둔다. Generic 닫는 >와 비교 >는 같은 raw token이며 Parser가 문맥을 판정한다.

## 검증
identifier에 keyword prefix가 들어간 경우, =/==/=>, 잘못된 문자, EOF 1개, Span 겹침 없는 원문 재구성을 확인한다.`,
10: `## Literal 초안 — D04
정수는 10진과 0x/0b/0o prefix, 자릿수 사이 _를 제안한다. 부호는 literal의 일부가 아니다. 실수는 decimal fractional 또는 exponent를 사용하고 suffix는 첫 승인안에서 제공하지 않는다. 문자 literal은 Unicode scalar 한 개, string은 UTF-8 sequence다.

## Escape 제안
\\n, \\r, \\t, \\0, \\\\, \\" 및 \\'와 \\u{hex}를 허용한다. surrogate와 0x10FFFF 초과를 거부한다. raw/multiline string은 이번 초안에서 제외한다. 실수 overflow/underflow와 rounding은 D07에서 정의한다.

## 단계 분리
Lexer는 철자와 escape 유효성을 검사하고 큰 정수 원문을 보존한다. 타입 단계가 기대 타입, 범위, prefix 음수와 최소 signed 값을 판정한다. token을 host i32로 먼저 파싱해 잘라내지 않는다.

## 검증
0x, 1__2, 1e+, 두 scalar char는 fail; int32 최솟값과 큰 uint64는 정확히 처리한다. LEXICAL과 NUMBER 모델을 함께 따른다.`,
11: `## Mode stack
Normal → StringText → InterpolationExpr → 중첩 StringText 전이를 stack으로 보존한다. 표현식의 { } 깊이는 문자열의 닫힘과 구분한다. 정상 string의 {{와 }}는 문자 중괄호이며 단일 {는 보간 시작이다.

## 상세 초안 — D04
보간 내부는 일반 표현식 문법을 사용한다. 닫는 }는 깊이가 0일 때만 문자열로 복귀한다. StringText에 단독 }가 나오면 오류다. format specifier와 사용자 정의 포맷 protocol은 0.1 첫 승인안에서 제외한다.

## 오류 복구
escape 실패는 해당 escape Span, 닫히지 않은 string은 opening quote와 EOF를 표시한다. 복구 결과는 Error token으로 남기고 codegen을 차단한다. mode depth resource limit은 내부 stack overflow 대신 진단으로 처리한다.

## 검증
중첩 호출/string/brace, 주석 속 brace, {{x}}, 빈 보간, EOF에서 각 mode의 상태와 결정적 token dump를 확인한다.`,
12: `## Comment 초안 — D04
//는 다음 논리 newline 전까지, /* ... */는 중첩 깊이 0까지 comment다. 문자열 안의 comment 표기는 text다. Block comment 안의 quote는 string mode를 열지 않는다.

## Source와 END
comment는 Span과 원문을 보존하고 내부 newline event를 END normalizer에 제공한다. a /* newline */ b를 a b와 같게 만들지 않는다. 최종 END 여부는 expression/delimiter 문맥에 따른다.

## Doc comment 제안 — D24
///와 /** ... */를 선언 문서 trivia로 분류한다. 일반 comment와 동일한 lexical 안전 규칙을 사용한다. 주석의 위치 이동이 doc 대상 선언을 바꾸면 formatter 오류로 본다.

## 검증
/* /* */ */ 정상, /* EOF 실패, // 마지막 EOF 정상. comment 제거 후 token Spans와 원문 복원이 동일한지, nesting limit 초과가 진단인지 확인한다.`,
13: `## END 입력/출력
Raw token+trivia를 정규화하며 세미콜론과 문장을 끝내는 newline을 END로 표현한다. ()/[] 내부 newline은 억제하고 {} 내부에서는 문장 종료를 허용한다. else, dot, comma, 연산자 앞뒤 연속 줄은 억제한다.

## 상세 초안 — D05
끝낼 수 있는 token은 Identifier/Literal/true/false/none/닫는 delimiter/exists 및 bare return/break/continue다. trivia를 건너뛰어 앞뒤 유효 token을 보고 prefix/binary 역할을 구분한다. return 뒤 newline은 bare return 종료를 우선한다. block 닫힘/EOF 직전은 Parser의 terminal boundary로 허용한다.

## 중요 예외
func signature 다음 {, condition 다음 {, else 다음 if/{는 END를 만들지 않는다. 단순 prev/next 표만으로 결정할 수 없는 header는 delimiter/header state로 처리한다. 규칙 표와 예제는 END_RULES를 따른다.

## 검증
세미콜론/줄바꿈의 AST 의미 동등성, operator 양쪽 newline, multi-line call, else 연결, return newline, block comment 내부 newline을 검사한다.`,
14: `## 문법 산출물
GRAMMAR.ebnf가 구체 Production을 담고 GRAMMAR_NOTES가 Stage와 결정 번호를 설명한다. lexer 원문과 END 정규화 후 Parser 문법을 분리한다. 현재 원본 NOVA-014에는 Production이 없으므로 새 grammar 전체는 승인 대기 초안이다.

## 문법 원칙
block은 {} 필수, 세미콜론 선택, 할당은 statement, comparison chaining 금지, generic call의 명시 타입 인수 금지다. func main()과 foreign 예제의 확정 철자는 보존한다.

## 모호성 제어
generic parameter는 선언, generic argument는 type 문맥에서만 파싱한다. type name expression으로 generic construction이 필요한 경우 D12를 먼저 결정한다. if/match/loop가 값인지 statement인지는 D08 초안에 명시한다.

## 검증
모든 nonterminal 정의/사용을 기계 검사하고 production별 최소 pass/fail을 CONFORMANCE에 연결한다. parser 구현이 grammar를 대신하지 않는다.`,
15: `## 우선순위
낮은 순서: until/through → || → && → == != < <= > >= → + - → * / % → prefix ! - + try → postfix call/member/index/exists. cast as의 정확한 위치는 D03에서 postfix 층을 제안한다.

## 결합성 초안 — D03
산술/논리는 왼쪽 결합, prefix는 오른쪽 결합, range와 comparison은 비결합이다. a < b < c와 a until b through c는 괄호 없이 거부한다. =는 Pratt 표에 넣지 않는다.

## Short-circuit
&&/||의 오른쪽은 조건에 따라 평가하지 않는다. precedence가 evaluation order를 변경하지 않는다. 나머지 operand는 소스 순서로 평가하는 D08 규칙을 따른다.

## 검증
1+2*3=7, (1+2)*3=9, !a&&b, f().x[i] exists의 AST를 검사한다. false&&panic()이 panic을 실행하지 않는 runtime fixture를 둔다.`,
16: `## Parser/AST 계약
정규화 token에서 소스 구조 AST를 만든다. declaration/statement/type은 Recursive Descent, expression은 Pratt다. Arena AstNodeId, Node Span, delimiter token 위치, trivia anchor를 보존한다. 타입과 DefId는 AST 필드가 아니다.

## 결과
ParseResult는 arena, root, diagnostics, recovered flag를 반환한다. 유효 프로그램과 복구 프로그램 모두 dump 가능하지만 오류 AST를 의미상 성공으로 표시하지 않는다. EOF에서 cursor가 진행하지 않는 반복을 금지한다.

## 검증
func/call/let/if/return 최소 노드와 production별 Span, missing delimiter synthetic Span, ErrorNode 방문을 검사한다. grammar version과 parser snapshot version을 연결한다.`,
17: `## 복구 전략
예상 token 누락이면 zero-width synthetic token과 진단을 추가한다. 예상하지 않은 token은 소비하거나 동기화한다. END/}/top-level 선언/주요 statement keyword가 동기화 후보다.

## 진행 불변 조건
각 반복은 cursor 증가, enclosing parser로 반환, 또는 EOF 종료 중 하나를 수행한다. 내부 }를 outer block 닫힘으로 무조건 삼키지 않는다. delimiter stack에는 opening Span을 저장한다.

## 진단 제한 초안 — D30
한 원인에서 다수의 expected-token 오류가 나오면 최초 오류와 구조 복구 note로 제한한다. per-file budget 도달 시 추가 오류 요약을 출력하고 종료한다. IDE mode와 batch mode의 성공 판정은 같아야 한다.

## 검증
func f(, let x=, if {, 닫힘 없는 nested block, 임의 token stream을 timeout 아래 처리하고 후속 정상 선언이 보존되는지 확인한다.`,
18: `## Source style
NOVA-131의 줄 길이 100과 주석 보존을 따른다. 4 spaces indentation, UTF-8/LF 출력, 최종 newline, 선택 세미콜론 생략을 초안 D24로 제안한다. 문장 경계를 바꾸는 줄 나눔은 허용하지 않는다.

## 배치 규칙
짧은 call은 한 줄, 긴 인수 목록은 각 한 줄과 trailing comma다. {는 header와 같은 줄, } else {를 연결한다. 연산자 continuation은 operator가 보이는 위치에 배치한다. string/comment 원문 내부는 재작성하지 않는다.

## 검증
parse(format(source))가 동일한 semantic AST/HIR를 만들고 format(format(source))가 byte 동일해야 한다. return newline 사례는 값을 연결하거나 끊지 않는다. 오류 파일을 부분 수정하는 정책은 D24에서 기본 거부를 제안한다.`,
19: `## 이름 해석
먼저 module/type/function 선언을 수집한 뒤 body 참조를 해석한다. 타입·값·모듈 namespace를 분리하고 nearest lexical scope를 검색한다. 결과는 문자열 대신 DefId/LocalId resolution side table이다.

## 상세 초안 — D06
동일 scope duplicate binding은 오류, nested scope shadowing은 허용하되 lint로 보고한다. let initializer는 새 binding 도입 전에 해석하므로 바깥 동일 이름을 참조할 수 있다. forward function/type 참조는 허용, forward local 참조는 금지다.

## 모호성
같은 이름의 후보를 import 순서로 고르지 않는다. overload set은 타입 검사로 넘기되 서로 다른 module에서 온 명확하지 않은 정의는 후보별 secondary label로 보고한다.

## 검증
호출 전 함수 선언, local 선언 전 참조, shadow initializer, namespace collision, import 순서 permutation을 다룬다.`,
20: `## Module 구조
원본 기준은 파일 경로에서 module 경로 결정, 순환 참조 허용, 순환 초기화 금지다. package source root, segment normalization, import grammar는 D01/D06 초안이다.

## 제안 계약
src/a/b.nova → package::a::b, src/main.nova는 binary root. use path [as alias]를 제안하고 wildcard는 첫 승인안에서 제외한다. 동일 경로 대소문자 충돌은 Windows/Linux 공통에서 오류로 제안한다. public use의 재export는 visibility보다 넓어질 수 없다.

## Graph 처리
모든 파일의 선언을 먼저 수집하고 SCC 단위로 이름을 확정한다. top-level runtime initializer는 첫 승인안에서 금지하며 const dependency cycle은 NOVA-032 오류다.

## 검증
mutual function call pass, 순환 const fail, 경로 충돌 fail, alias와 reexport access 검사. Stage A는 단일 파일이며 module feature를 흉내 내지 않는다.`,
21: `## Visibility 초안 — D06
public는 외부 package까지, internal은 동일 package, private는 선언 module 내부로 제안한다. top-level 기본은 internal, member 기본은 private를 제안하며 결정 승인 전 사용하지 않는다.

## 유효 접근성
public item의 signature가 private type을 노출하면 오류다. reexport는 원 정의의 접근성을 확장하지 못한다. nested type, interface method, constructor도 동일한 접근 검사에 포함한다.

## 구현
Definition에는 declared/effective visibility, owner module/package를 저장한다. overload 후보의 접근 불가를 무조건 undefined-name으로 숨기지 않고 접근 위치와 원 선언을 보여준다.

## 검증
같은 module/package/다른 package의 3개 caller에서 public/internal/private를 교차 검사한다. private field 접근과 public function의 private return type을 fail로 둔다.`,
22: `## ID 계층
FileId는 source database identity, SymbolId는 session 문자열 intern, ModuleId/DefId는 선언 identity, LocalId는 body 내 binding identity다. 서로의 숫자가 같아도 혼용하지 않도록 Rust newtype을 사용한다.

## 안정성 계약
session integer ID와 영구 cache key를 구분한다. serialized key는 package identity+module path+item path+disambiguator+signature fingerprint 초안을 사용한다. OS 파일 검색 순서나 memory address를 key에 넣지 않는다.

## SourceInfo
모든 선언은 Span, owner와 source origin을 가진다. synthetic definition에는 생성 원인 Span을 유지한다. ID 재배정은 dump 출력의 deterministic ordering을 해치지 않아야 한다.

## 검증
파일 나열 순서 변경, 동일 이름 nested scope, overloaded function, 재빌드 key 비교. duplicate DefId 발생은 내부 검증 오류다.`,
23: `## Package 경계
package identity에는 이름만이 아니라 source와 정확한 version/revision을 포함한다. 동일 이름 다른 source package를 하나로 합치지 않는다. 외부 package 참조는 NOVA-126/127의 resolved graph를 입력으로 받는다.

## Import 계약 초안 — D06/D21
dependency alias를 root segment로 사용한다. export metadata만 접근 가능하고 private/internal을 외부에서 해석하지 않는다. 동일 alias의 복수 dependency는 manifest 단계에서 오류다.

## 구현
compiler resolver는 network를 직접 조회하지 않는다. package 계층이 검증한 source/export artifact를 넘긴다. metadata compiler/ABI/schema mismatch는 source rebuild 또는 명시 오류다.

## 검증
version 다중 공존, alias collision, private symbol 거부, lock된 revision 교체 탐지, import 순서 독립성을 확인한다.`,
24: `## 충돌/Shadow 구분
duplicate는 같은 scope/namespace의 충돌, shadow는 다른 nested scope의 동일 이름이다. overload는 signature 규칙을 만족하는 함수만 묶으며 반환 타입만 다른 함수는 duplicate다.

## 진단 계약
duplicate의 primary는 뒤 선언, secondary는 앞 선언이다. ambiguity의 primary는 참조, secondary는 모든 경쟁 후보이며 정렬 순서는 module path/signature다. shadow lint는 기존 binding을 표시한다.

## 초안 코드
N2001 undefined name, N2002 duplicate definition, N2003 ambiguous name, N2004 inaccessible item을 DIAGNOSTICS.csv에서 제안한다. 아직 승인된 고정 코드라는 뜻은 아니다.

## 검증
순서를 바꿔도 후보 목록이 결정적이고, Error resolution에서 파생된 타입 오류를 중복 출력하지 않는지 확인한다.`,
25: `## 타입 검사
양방향 검사: 기대 타입이 있으면 check, 없으면 synthesize. TypeInterner는 Unit/Never/Primitive/Named/Tuple/Array/Function/GenericParam/Error를 구분한다. alias와 nullable 정규화는 unification 전에 수행한다.

## 변환 초안 — D07
같은 타입/alias identity, 제한적 lossless numeric widening만 암묵 허용한다. signed/unsigned는 전체 값 범위가 포함될 때만 허용한다. integer→float는 전 범위 정확 표현 가능할 때만 허용한다. int32→float32 및 int64→float64는 자동 허용하지 않는다.

## 오류
type mismatch는 기대 타입의 선언과 actual expression Span을 표시한다. ErrorType는 분석 지속 전용이며 codegen으로 통과하지 않는다. unconstrained literal은 기본 int/float를 제안하고 범위 초과는 추측 widening 대신 오류다.

## 검증
NUMERIC_RULES의 변환표와 overload 선택 결과가 일치해야 한다. bool↔int implicit 변환은 금지 제안이다.`,
26: `## Primitive 계약
int/uint는 32-bit, float/double은 binary32/binary64 alias다. fixed-width signed/unsigned, bool, Unicode char, owned UTF-8 string, Unit, Never를 구분한다.

## 기본값 초안 — D07
정수 literal은 문맥 우선 후 int32, 실수는 문맥 우선 후 float32. unsigned 문맥의 음수, char의 surrogate, 범위 초과 literal은 오류다. char를 byte/int로 자동 변환하지 않는다.

## 연산 초안
모든 profile의 integer overflow, divide-by-zero, signed MIN/-1, bounds 실패는 Abort panic을 제안한다. 실수는 IEEE binary32/64 semantics, NaN 비교를 그대로 따르고 fast-math 기본 off를 제안한다. explicit cast 범위 밖도 panic, wrapping은 별도 명시 API다.

## 검증
각 width 최소/최대 및 ±1, 음수 최소값 parsing, float32 precision 경계, NaN/-0/infinity, debug/release 결과 일치를 검사한다.`,
27: `## 값/Handle 구분
Struct는 값 의미, Class는 정체성을 가진 owned handle, Enum은 하나의 활성 variant와 payload다. 모든 필드는 let/var를 명시하고 직접 재귀 값 포함은 금지한다. Option/Result는 특별 null sentinel가 아닌 일반 Enum 의미다.

## 선언 초안 — D12
struct/class/enum 이름 뒤 generic parameter와 implements 목록, {} member body를 제안한다. Enum variant는 이름과 위치 payload tuple로 시작하며 custom discriminant/field syntax는 보류한다. 직접 순환 layout graph는 size query에서 오류다.

## Copy/Drop
Struct Copy 여부는 모든 field의 Copy와 사용자 drop 부재에 따라 결정하는 D10 제안이다. Class handle 복사는 금지하고 take로 이전한다. inactive Enum payload를 읽거나 Drop하지 않는다.

## 검증
recursive-by-value fail, handle indirection pass, immutable field reassignment fail, enum payload Drop count 및 zero-sized aggregate를 다룬다.`,
28: `## 확정 의미
T?는 Option<T>; variant는 Some/None다. Result<T,E>는 Success/Error이며 try는 Error를 조기 반환한다. Success(())는 Unit 성공이다. niche는 내부 최적화이므로 source 의미와 ABI 약속을 만들지 않는다.

## 상세 초안 — D08/D23
none은 기대 Option 타입이 필요하다. T→Option<T> 자동 wrapping은 제공하지 않고 Some(value)를 명시한다. x exists는 payload를 소비하지 않는 bool 테스트다. try는 enclosing Result의 Error 타입과 동일 E를 요구하며 자동 오류 변환은 첫 승인안에서 제외한다.

## 검증
Some/None match exhaustiveness, nested Option<Option<T>>, none inference fail, try Error cleanup, Move payload double use 거부. representation의 None가 항상 0이라는 가정은 금지한다.`,
29: `## Tuple/Array/Function 초안 — D12
()는 Unit, (x,)는 1-tuple, (x,y)는 tuple이다. tuple projection은 .0/.1을 제안한다. [a,b]는 owning Array<T>이고 원본 Array의 길이/용량/초기화 구간 의미를 유지한다. 고정 길이 const generic array는 제외다.

## Function type
func(mode T, ...) -> R 형태를 제안한다. function item과 closure는 내부에서 구분하고 capture 없는 lambda만 plain function pointer로 변환한다. mode가 다른 function type은 같지 않다.

## 검증
tuple arity/type mismatch, empty Array의 expected type 요구, heterogeneous Array 거부, read/change/take 함수 타입 대입, Array 성장 중 element Drop count를 검사한다. Array와 List의 public API 중복은 D23에서 해결한다.`,
30: `## Type alias 초안 — D01/D12
type Name = Type 표기를 제안하지만 type은 원본 공식 keyword 표에 없으므로 승인 전 추가하지 않는다. alias는 nominal newtype가 아니라 동일 타입의 별칭이다.

## 정규화 계약
alias를 canonical type으로 확장하되 진단에 사용자 철자를 보존한다. alias dependency graph의 cycle은 chain과 각 선언 Span을 보여준다. Option Sugar와 Primitive alias를 같은 normalizer에서 처리한다.

## 검증
alias alias chain pass, A=B/B=A fail, alias를 통한 duplicate overload fail, private target type를 public alias로 노출하는 경우 visibility fail. alias 때문에 specialization을 중복 생성하지 않는다.`,
31: `## Constructor 초안 — D12
init(parameter list) { ... }를 제안하고 각 let field는 정확히 한 번 초기화해야 한다. method receiver 철자/self binding은 D11과 함께 결정한다. 생성 완료 전에 self를 외부로 노출하거나 일반 method를 호출하지 않는다.

## 초기화 계약
필드마다 Uninitialized/Initialized/MaybeInitialized 상태를 계산한다. 모든 정상 return path에서 모든 필드가 initialized여야 한다. 실패/조기 반환에서는 이미 초기화된 field만 선언 역순 Drop한다. zero initialization은 언어 기본 의미가 아니다.

## 검증
누락 field, 분기 한쪽에서만 초기화, let field 2회 대입, self escape, partial initialization cleanup. Array 내부 부분 초기화 허용과 사용자 partial move 금지를 구분한다.`,
32: `## const 초안 — D09
const는 compile-time에 평가 가능한 불변 binding이다. literal, approved primitive operation, tuple/enum/struct construction을 허용하고 I/O, heap allocation, foreign call, mutable global access는 첫 승인안에서 금지한다.

## 평가 계약
Target width/rounding을 사용하고 host usize/float 동작에 의존하지 않는다. overflow/bounds/division 실패는 runtime panic 대신 해당 const expression의 compile error다. dependency cycle은 모든 참조 경로를 보여준다.

## resource limit
step/depth budget은 결정적인 옵션으로 기록하고 budget 초과를 user diagnostic으로 처리한다. arbitrary compile-time user function은 별도 const-function 설계가 승인되기 전 제외한다.

## 검증
const 1+2 정상, const 1/0 실패, cross-module cycle, 32-bit/64-bit Target에서 같은 fixed-width 값, budget 재현성을 검사한다.`,
33: `## Target layout 계약
Layout(TypeId,TargetSpec) → size, align, field offsets, variant layout, valid-bit-patterns/niche를 반환한다. pointer width/endian/aggregate ABI는 Target 입력이며 host에서 추정하지 않는다.

## 초안 — D16
Primitive size는 fixed-width, bool/char memory representation과 aggregate field packing은 ABI 문서에 명시한다. 기본 aggregate는 선언 field 순서, alignment padding을 제안한다. C compatible representation은 explicit foreign wrapper에서만 약속한다.

## Niche
Option의 invalid-bit-pattern 활용은 내부 layout 최적화다. public serialization/FFI는 niche를 그대로 노출하지 않는다. zero-sized 값의 storage 및 주소 identity는 D16 대상이다.

## 검증
alignment, padding, nested aggregate, recursive layout fail, Option handle와 payload enum, Target별 golden layout을 검사한다.`,
34: `## Class 의미
Class는 owned handle이고 구현 상속을 지원하지 않는다. 이동은 객체를 복사하는 것이 아니라 handle 소유권을 이전한다. 마지막 owner가 파괴될 때 field Drop 뒤 storage를 해제한다.

## 상세 초안 — D12/D23
일반 ==가 pointer identity인지 값 equality인지 원본에 없으므로 D12에서 별도 identity API를 제안한다. read receiver는 내부 수정 금지, change receiver는 독점 수정, take receiver는 소비다. shared conversion은 ownership를 명시적으로 이전한다.

## 구현
object layout과 handle ABI를 분리하고 allocation failure는 D18 정책을 따른다. owner/refcount/view를 한 handle 종류로 혼용하지 않는다.

## 검증
이동 후 동일 object identity, clone 없이 중복 owner 생성 거부, field 역순 Drop와 최종 free, weak upgrade 실패 후 dangling 접근 금지.`,
35: `## Function signature
FunctionId, ReceiverMode, ParameterMode, ReturnType/ownership, Effects를 포함한다. modifier 부재는 Read, change는 exclusive loan, take는 이전이다. 인수는 소스 순서로 평가하고 return type만으로 overload하지 않는다.

## Receiver 초안 — D11
member function의 receiver를 func name(read/change/take self, ...)처럼 별도 read 키워드로 추가하지 않는다. 이 초안은 func name(self, ...), func name(change self, ...), func name(take self, ...)를 제안하며 self는 contextual binding이다.

## 반환
Unit 반환은 생략/void 정규화다. owned 값 반환은 owner를 caller로 이전한다. view 반환은 parameter-origin lifetime 계약이 필요하며 local owner를 가리키는 반환은 거부한다.

## 검증
mode mismatch, named argument 순서, early return cleanup, method mutation with read receiver fail, overload return-only conflict를 검사한다.`,
36: `## Argument mapping 초안 — D11
위치 인수 뒤에 이름 인수(label: expression)를 허용하고 이름 인수 뒤 위치 인수는 거부한다. 같은 parameter를 두 번 채우거나 알 수 없는 label은 오류다. label은 overload 선택 signature의 일부다.

## 평가
제공된 인수는 source order로 임시값에 저장한 후 parameter order로 전달한다. 기본 인수는 원본 규칙대로 caller에서 평가한다. 빠진 default 인수는 제공 인수 뒤 declaration order로 평가하며 이전 parameter 참조는 첫 승인안에서 금지 제안이다.

## 검증
named 순서 반전에도 출력은 source order, 중복/누락/unknown label fail, default side effect 한 번, default 타입 오류는 선언과 호출을 함께 표시한다.`,
37: `## Overload 초안 — D11
후보는 이름/visibility/arity/label/mode로 필터하고 generic inference 후 변환 비용을 계산한다. identity=0, lossless numeric widening=1, 나머지 implicit conversion은 불허를 제안한다.

## 선택 알고리즘
parameter별 비용 vector의 Pareto dominance로 후보를 비교한다. 하나가 모든 위치에서 같거나 작고 한 곳에서 작으면 우수하다. 유일한 우수 후보가 없으면 ambiguous다. declaration order, 기본 인수 개수, 반환 타입에 임의 tie-break를 넣지 않는다.

## 검증
exact vs widening, (0,1) vs (1,0) ambiguity, generic/non-generic 동률, inaccessible candidate, ErrorType로 오염된 후보의 중복 진단 억제를 다룬다.`,
38: `## Lambda 초안 — D13
lambda (parameters) => expression 또는 lambda (parameters) => { statements } 철자는 제안이며 원본 keyword 목록에 없으므로 승인 전 contextual syntax로도 구현하지 않는다. expected function signature에서 parameter type을 추론한다.

## 의미
capture 없는 lambda는 function pointer로 변환 가능하다. capture 있는 closure는 environment+call function의 구체 타입이고 동적 interface object가 아니다. 캡처 모드는 Read/change/take다.

## 검증
capture-free conversion, expected type 부재 parameter 오류, returned borrowed closure의 owner 수명 오류, one-shot take capture 호출 2회 오류, environment Drop 정확성을 검사한다.`,
39: `## Capture 분석
closure body의 free Place 사용을 모은 뒤 Read/change/take 필요도를 결정한다. 필요 모드가 충돌하면 가장 강한 의미로 조용히 바꾸지 않고 closure 선언에서 소유권 요구를 보여준다.

## 초안 — D13
Read capture는 loan, change는 exclusive loan, take는 owned environment field다. capture list의 명시 철자와 기본 capture 정책을 D13에서 제안한다. take capture가 소비되는 closure는 take call receiver로 한 번만 호출 가능하다.

## 검증
같은 owner의 두 change closure 동시 live fail, owner보다 오래 사는 borrowed closure fail, move closure 반환 pass, field partial capture가 사용자 partial move를 우회하지 못하는지 확인한다.`,
40: `## Stage A 확정
단일 소스의 func main()은 Hello Nova entry다. 실행 파일은 Runtime startup wrapper에서 이를 호출한다. main의 이름을 user LLVM symbol main과 직접 동일시하지 않는다.

## 확장 초안 — D19
0.1 최초 release는 parameter 없는 Unit main만 허용하고 exit=0을 제안한다. int/Result return main, argv parameter는 후속 승인 없이 추정하지 않는다. 환경/인수는 표준 API로 접근하는 제안이다.

## 검증
main 부재, 중복, generic main, non-Unit return, parameter 있는 main은 entry 진단. library target은 main을 요구하지 않는다. Wrapper의 startup 실패와 Nova main panic의 exit 의미를 구분한다.`,
41: `## Effect 계약
pure와 noPanic은 이름만으로 LLVM attribute에 대응시키지 않는다. 순수성이 반환값, 외부 mutation/I/O/allocation/abort 중 무엇을 제한하는지 D14에서 명확히 결정한다. noPanic은 공식 keyword 표에 없으므로 표기부터 검토한다.

## 초안 — D14
pure는 외부 I/O, foreign effect, global mutation, change parameter mutation을 금지하는 보수적 검사를 제안한다. noPanic은 모든 reachable operation/callee가 bounds/overflow/explicit panic을 일으키지 않는 증명이 필요하다.

## 구현과 검증
EffectTable과 call graph fixed-point로 전파한다. unknown foreign effect는 effectful로 간주한다. pure 함수의 print fail, noPanic 함수의 unchecked arithmetic fail, recursive pure SCC pass를 다룬다. MVP에서 지원 여부는 별도 결정이다.`,
42: `## Native ABI
Nova 내부 호출과 C foreign ABI를 분리한다. semantic mode를 physical pass mode와 혼동하지 않는다. Read scalar가 register 값으로 전달되어도 owner 이전을 뜻하지 않는다.

## Target 계약 초안 — D16
Scalar는 target register convention, aggregate는 direct/coerce/indirect, 큰 return은 hidden destination, change는 exclusive pointer로 내린다. return destination과 input alias 보장은 증명된 만큼만 backend attribute로 전달한다.

## 검증
caller/callee가 같은 AbiSignature를 사용해야 한다. scalar/aggregate/Unit/handle/view와 register threshold 경계를 target별 C harness 또는 Nova cross-crate harness로 검사한다. ABI 버전을 artifact에 기록한다.`,
43: `## 제어 흐름 초안 — D08
if/while 조건은 bool만 허용한다. if/while/for/loop는 statement이며 value expression으로 사용하지 않는다. for binding in expression { ... }는 iterable/range를 한 번 평가한다.

## 평가/Scope
if는 선택 arm만 실행한다. while는 매 반복 조건을 다시 평가한다. loop는 break까지 반복한다. loop body scope의 local은 각 iteration 끝에 Drop한다. continue도 그 iteration cleanup을 거친다.

## Range
until은 끝 제외, through는 포함이다. iterable protocol은 Associated Type 없이 D15의 명시 generic interface 계약으로 제안한다.

## 검증
integer condition fail, else-if chain, zero iteration, continue cleanup, inclusive 최댓값 종료에서 overflow 없이 끝나는 range를 검사한다.`,
44: `## Jump 의미
return은 enclosing function, break/continue는 가장 가까운 loop에 대응한다. lambda 내부 jump가 바깥 function/loop를 벗어나지 않는다. labeled jump 문법은 이번 초안에서 제외한다.

## 초안 — D08
Unit 함수의 bare return 허용, 값 return은 declared type에 check한다. non-Unit 함수의 reachable fallthrough는 오류다. break value를 지원하지 않는다. return expression은 이동/평가 후 local cleanup을 실행한다.

## 검증
loop 밖 break/continue fail, function 밖 return fail, 한 branch return 누락 fail, never callee 뒤 unreachable, return된 owner가 local Drop로 파괴되지 않는지 확인한다. END의 return newline 의미는 D05와 일치해야 한다.`,
45: `## Range 계약
until는 upper exclusive, through는 inclusive다. 시작/끝은 한 번씩 source order로 평가한다. 첫 승인안 D08은 정수의 오름차순 step=1만 제공하고 custom step/descending은 제외한다.

## 경계 의미 초안
start≥end인 until는 빈 범위, start>end인 through는 빈 범위다. inclusive 최댓값은 마지막 값을 처리한 뒤 종료하며 end+1 계산으로 overflow하지 않는다. endpoints는 공통 lossless 정수 타입을 요구한다.

## 검증
0 until 3→0,1,2; 0 through 3→0,1,2,3; 3 until 3→empty; MAX through MAX→한 번. signed/unsigned 무손실 공통 타입 없는 조합은 fail이다.`,
46: `## Pattern 초안 — D08
wildcard _, binding, literal, tuple, qualified enum variant(payload), Some/None/Success/Error를 제안한다. repeated binding names는 오류이며 type checks는 scrutinee type을 따른다.

## Ownership
Read match는 payload loan을 만들고 take match는 전체 scrutinee owner를 소비한다. 사용자가 aggregate의 한 Move field만 빼오는 pattern은 partial move 금지로 거부한다. copy field projection은 이동이 아니다.

## 검증
tuple arity, variant payload type, duplicate binding, shadow scope, irrefutable let pattern, move field extraction 금지. guard의 binding 수명과 consumption은 D08/D10 승인 전에 명시적으로 제한한다.`,
47: `## Exhaustiveness
bool/Enum/Option/Result/tuple 조합은 constructor coverage로 검사한다. integer/string의 임의 값 공간에는 wildcard가 필요하다. guard가 있는 arm은 exhaustive coverage 증명으로 사용하지 않는 초안을 제안한다.

## 도달성
앞 arm이 완전히 덮는 뒤 arm은 unreachable 진단이며 severity는 D25에서 결정한다. declaration/source arm 순서를 보존한다. 누락 경우는 가능한 최소 pattern 예시로 보여준다.

## 검증
bool 한 arm 누락 fail, Option Some만 fail, wildcard 후 arm unreachable, guarded wildcard만 존재할 때 incomplete, nested enum/tuple coverage를 검사한다. algorithm recursion에는 결정적인 resource budget을 둔다.`,
48: `## Decision tree lowering
scrutinee는 한 번 평가하여 local/place에 저장한다. variant tag/literal test를 공유하되 arm 순서, guard side effect, loan lifetime은 보존한다. payload는 variant 검사가 성공한 뒤만 접근한다.

## CFG 계약
각 arm entry/binding/guard/body/join block을 구분한다. guard false는 다음 arm으로 이동하고 guard 임시값 cleanup을 수행한다. early return/try는 enclosing cleanup으로 연결한다.

## 검증
scrutinee call count=1, guard 순서 trace, inactive payload 접근 없음, source arm order가 최적화 뒤에도 동일한지 검사한다. exhaustive analysis 결과가 없으면 codegen으로 넘기지 않는다.`,
49: `## 확정 Result 의미
try expression은 Result의 Success payload를 산출하고 Error payload를 enclosing function의 Error return으로 보낸다. Success(())를 Unit 성공으로 쓴다. throw/unwind를 도입하지 않는다.

## 초안 — D08
오류 타입 E가 enclosing return Result의 E와 정확히 같아야 한다. 변환이 필요하면 사용자가 explicit wrapper를 작성한다. try는 가장 가까운 lambda/function 경계를 사용하며 일반 Unit 함수에서는 거부한다.

## MIR
Result temporary → discriminant switch → Success extraction 또는 Error construction/return → local cleanup. 전체 result를 한 번만 consume하고 inactive payload를 Drop하지 않는다.

## 검증
nested try, success side effect, Error early return의 local 역순 Drop, E mismatch fail, try outside Result fail.`,
50: `## using 초안 — D08/D10
using let resource = expression을 block scope의 owned binding sugar로 제안한다. initializer는 한 번 평가하며 정상/return/break/continue/try exit에서 resource Drop을 보장한다.

## 경계
abort panic은 unwind하지 않으므로 using cleanup을 보장하지 않는다. Dispose 같은 새 interface를 자동 호출하지 않고 일반 drop glue에 연결한다. scope 밖으로 take하여 resource lifetime을 연장하는 정책은 첫 승인안에서 거부 제안이다.

## 검증
파일 open 성공/오류, nested using 역순 cleanup, early return, initializer 실패 때 미생성 resource Drop 없음, panic 시 종료를 각각 확인한다.`,
};
