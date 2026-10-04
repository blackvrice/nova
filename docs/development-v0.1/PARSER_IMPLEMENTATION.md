# P01 Stage A Parser 구현 계약

사양: [사용자 승인 P01](PARSER_STAGE_A_PROPOSAL.md) 및 [전용 EBNF](GRAMMAR_STAGE_A.ebnf).
구현 위치: crates/nova-ast와 crates/nova-parser. Backend/의미 타입 의존은 없다.

## 사용 순서

SourceDatabase에 UTF-8 파일 추가 → nova_lexer::lex → normalize_ends → nova_parser::parse.
Parser가 Lexer를 호출하지 않는다. raw token/trivia 원문은 Lexed에 보존하며 AST는 byte Span으로
원문을 참조한다. source 파일은 append-only여서 기존 Span이 변하지 않는다.

parse(sources, file, normalized_tokens) → Result<Parsed, ParseInputError>.
Parsed는 Arena/root/Diagnostic/SyntheticToken을 가진다. raw END origin은 caller의 normalized
tokens에 남는다. AST statement의 끝에 세미콜론을 강제로 삽입하지 않는다.

올바른 API 입력의 잘못된 Nova 구문은 Result::Ok의 recovery AST/diagnostics다.
Result::Err는 존재하지 않는 파일, 유효하지 않은 Span/token stream/options 등 API 입력 오류다.
Normalized stream은 source-order, same-file, UTF-8 경계이고 마지막 token이 EOF여야 한다.
Whitespace/Comment/NewLine/Semicolon이 남아 있으면 InvalidTokenStream이다.

호출자는 Lexed.has_errors와 Parsed.has_errors를 모두 확인해야 한다.
Parser가 lexer Error token에서 새 진단을 억제해도 Error Node 때문에 Parsed.has_errors는 true다.
오류가 있는 결과는 향후 HIR/codegen 성공 경로에 보내지 않는다.

## 저장과 순회

AstNodeId는 arena 내부 insertion 순서의 usize index다. 영구 cache ID가 아니다.
NodeKind는 syntax 구분과 name/operator metadata, children은 source-order ID를 저장한다.
Function은 parameter들/optional return type/body, Binding은 optional type/initializer,
If는 condition/then/optional else, Call은 callee/argument 순서다.

자식 ID는 부모보다 먼저 삽입된다. Arena는 존재하지 않는 ID, 다른 FileId, 부모 밖 child Span을
거부한다. 순환 연결은 불가능하다. Visitor/walk/dump/drop은 반복형이며 deep AST에서 재귀하지 않는다.
SyntheticToken은 real source bytes가 아닌 zero-width Span을 가지고 실제 punctuation과 구분된다.

## 복구와 한도

Recursive Descent는 declaration/statement/type, Pratt는 expression을 담당한다.
block/if/expression의 활성 재귀 진입 수를 nesting으로 센다. 기본 128, 검증된 설정 범위 1~128.
한도 도달 시 N1102와 configured limit note를 내고 반복형 skip/recovery를 사용한다.
literal 문자열 길이, source byte 크기, flat statement/node 수 자체에는 새 한도를 두지 않는다.

누락 punctuation은 N1101 및 synthetic token으로 남긴다. delimiter opening Span은 secondary label이다.
N1102는 Stage A 미지원 구문/한도, N1103는 comparison의 두 번째 operator와 첫 operator secondary다.
함수 본문에서 func를 만나면 누락된 `}`를 복구하고 다음 함수 선언을 보존한다.

## 검증 범위

- AST 3개: stable ID/Span/dump/Visitor, invalid insertion, 32,001-node iterative walk/drop.
- Parser 16개: production coverage와 Hello AST snapshot, 연산자·call·if·interpolation,
  END 동등성, UTF-8/CRLF anchors, 실패 code/Span, synthetic/next-function recovery.
- fixture의 모든 prefix, seed 고정 1,000개 adversarial token sequence의 두 번 parsing 결과 비교.
- 10,000개 grouping/prefix nesting, 2,000개 if/interpolation nesting의 limit 진단,
  10,000개 flat call statement 수용, 더 작은 configured limit 검증.

이 증거는 구문 검사다. Semantic type/range, main signature/존재, print formatting/runtime,
bool-only 조건/short circuit, overload/visibility/ownership는 구현·검증하지 않는다.
