# P22 Copy struct Read 메서드 구현·검증 기록

승인 반영/구현일: 2026-10-08. 사용자 “승인하고 다음개발 진행해줘” 답변으로
앞서 준비한 [P22 계약](METHOD_STAGE_B_PROPOSAL.md)·[58-production EBNF](GRAMMAR_STAGE_B_METHOD.ebnf)를 적용했다.
D01~D05/P01~P21·Canonical·원본 148개를 보존한다.

## 구현

- struct member에 [visibility] func name(self, typed/default parameters) 문법을 지원한다.
  receiver는 정확한 self spelling의 별도 AST/HIR child이며 annotation/default/mode를 갖지 않는다.
  AST Method의 func/name/whole·visibility/paren/comma·ordinary/default/body Span을 유지한다.
  HIR은 canonical Function+Receiver로 정규화하고 private MethodSource 표에 원 token Span과 Struct owner를 보존한다.
  bundle은 owner/HirId를 rebasing하고 원 FileId/SourceOrigin을 유지한다.
- 외부 AST는 receiver/method shape·직접 struct owner·source order·parent/UTF-8 포함·self/keyword/name 양쪽 XID 경계·
  nested-comment trivia와 paren/comma/arrow gap을 검증한다. 기존 오류 복구와 typed ordinary function은 보존한다.
- Resolver는 canonical Struct DefId당 field/method member namespace를 만들고 source order의 이후 duplicate를 N2002로 보고한다.
  methods/signatures는 body 전에 전부 수집한다. forward/mutual/recursive 호출을 지원하고 top-level value/import namespace에는 넣지 않는다.
  1,024개 method를 허용하며 첫 1,025번째 name은 N8901·note다. constructor와 field/layout 한도는 field만 센다.
- receiver는 owner의 Copy struct 타입인 immutable parameter다. self/root field write는 N3004 원 self Span을 사용한다.
  var copy=self는 mutable 지역 Copy이며 caller는 유지된다. self shadow는 기존 P02 lexical scope 규칙을 따른다.
- expr.member(args)는 canonical nominal owner로 정적으로 조회한다. private는 선언 module에 한정하고 N2004 member-name/
  declaration-name Span을 사용한다. opaque factory·type alias/import·Tuple 안의 struct receiver도 같은 owner를 재사용한다.
  method value/grouped callee는 N1102이며 grouped receiver는 허용한다. method main은 root entry가 아니다.
- TypeChecker는 receiver를 먼저 검사한 뒤 일반 인수의 기대 타입을 적용한다. source ordinary index 0은 ABI slot 1이다.
  self는 이름 인수/default 후보에서 제외한다. 일반 source-order mapping은 Checked NamedCall에 유지한다.
  default는 P18 선언 module scope에서 검사·상수 평가하며 implicit receiver/ordinary parameter scope는 참조하지 않는다.
  method calls는 const/default에서 N3201 whole call이며 skipped RHS도 정적 permission 검사 대상이다.
- MIR은 receiver를 먼저 한 번 평가해 Copy snapshot을 만들고, provided ordinary arguments를 source order로 평가·변환·복사한다.
  omitted defaults는 declaration order로 caller에서 materialize한다. receiver slot 0과 일반 parameter slots로 static callee를 호출한다.
  try Error/Abort는 남은 인수/default/body를 생략한다. 기존 struct snapshot/private ABI와 String arena lifetime을 재사용한다.
- public Resolved/Checked method owner·binding·visibility·signature·offset·call plan은 기존 전체 재계산 gate로 검증한다.
  MIR은 receiver를 포함한 NamedCall certificate와 caller CFG/body proof를 보존한다. method body도 별도 private full-body proof로
  owner/parameter/source/return/effect 위조를 거부한다. 새 Lexer/END·Runtime API·LLVM production/public ABI는 없다.

## 추가 테스트

기본 11개·실제 LLVM 1개·Windows Native 2개, 총 14개를 추가했다.

- Parser 1: mixed field/method source order·self/func/paren/comma exact Span·trailing comma·UTF-8 prefix recovery.
- HIR 1: source token/spelling/boundary·visibility·paren/comma·receiver/owner 외부 AST 위조 거부.
- Driver 2: 두 파일/정상/Runtime fixture의 verified MIR·원 MethodSource/file/owner와 부정 20개 exact UTF-8 code/Span,
  private nominal opaque factory와 선언 module의 private method 호출.
- TypeChecker 3: forward/mutual recursion·동명 owner/global 함수·P02 self shadow·immutable write·bound method/cast 제외,
  ABI offset/default/global self·1,024/1,025 method 한도와 field 독립성·skipped const permission.
- MIR 2: method main 비entry·receiver snapshot/argument slot·method body return·Resolved owner/Checked static callee 위조 거부.
- CLI 1: check/build/run의 semantic 실패 시 도구 호출 전 차단·기존 output 보존·method main만 있으면 N2001.
- LLVM 2: distinct private method symbols·struct receiver·root entry·deterministic Windows/Linux IR·실제 COFF/ELF O0/O2 객체.
- Native 2: debug/release 두 파일 15줄·empty/trailing receiver·opaque/private·mutable local Copy·recursion·String·Float/Char/Tuple/Option ABI,
  receiver Abort 뒤 일반 arg/output 생략·cross-file 원 UTF-8 file/span과 checked overflow.

## 검증 결과

- 기본 전체 회귀: **359 PASS / 0 FAIL / 70 ignored**.
- 실제 LLVM 전체: **18 PASS / 0 FAIL**, 13.67초. COFF/ELF 객체와 O0/O2를 검증했다.
- Windows Native 전체: **52 PASS / 0 FAIL**, 440.85초. debug/release 실제 실행을 검증했다.
- 세 suite 합계 **429 PASS / 0 FAIL**이며 opt-in 70개도 LLVM 18개·Native 52개로 별도 실행했다.
- P22 Native 집중: **2 PASS / 0 FAIL**, 13.88초. debug/release 실제 실행을 검증했다.
- fmt·별도 Runtime rustfmt·clippy -D warnings·all-features PASS.
- 추가 CLI 경계 9개 PASS: method default마다 10,000-node 허용·10,001-node/생략 RHS N3202,
  unused method default의 receiver/ordinary parameter 참조 N2001·field 뒤 typed receiver N1102,
  mapped int8/int16/uint8/double 기대 문맥·literal range N2102·static type method head N2001.

- 문서 build/validator PASS. P01~P21 계약/EBNF/ledger·D01~D05·Canonical·원본 148개 SHA-256·P22 fixture source bytes 보존 PASS.
- 추가 Native: (try fetch()).m(n:try arg())의 receiver/argument 성공·Error 출력과 뒤 인수/body 생략이 debug/release에서 PASS.
- 독립 examples/methods.nova check와 debug/release 15줄 UTF-8 LF·빈 stderr·exit 0 PASS.
기존 테스트를 삭제하거나 완화하지 않았다. parser의 기존 missing struct brace 복구를 보존했다.
receiver shadow 테스트는 기존 P02의 nested if scope를 사용한다. Draft fixture 주석은 작성 시점의 역사 기록이며
source bytes와 승인된 diagnostic/runtime Span을 유지했다.

Git 원본 감사 중 OneDrive의 pack mmap 읽기 오류가 발생했다. GitHub 원본으로 byte 비교를 완료한 뒤,
기존 이력을 삭제하지 않고 검증된 pack을 추가하고 MIDX를 백업·갱신해 원 저장소 읽기도 복구했다.

환경: Rust 1.99.0, LLVM 21.1.8, Windows x64 MSVC. MSRV 1.80은 별도 실행하지 않았다.
Linux Native host는 미검증이며 ELF 객체 생성과 구분한다.

## 직접 실행과 후속

[독립 예제](../../examples/methods.nova)·[TESTING.md](../../TESTING.md)를 따른다.
check는 출력 없이 exit 0, debug/release는 [15줄](method-proposal-fixtures/README.md)·LF·빈 stderr·exit 0이다.
change/take·일반 Move/borrow/Drop·explicit init·overload·bound method·Enum method·Array·Class/Generic/interface·
Package/API leak/export와 전체 D06/D09/D10/D11/D12/D16/D25/D30은 후속이다.
