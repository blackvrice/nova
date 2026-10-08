# P21 비제네릭 Type Alias 구현·검증 기록

승인 반영/구현일: 2026-10-08. 사용자 “P21 승인하고 Type Alias 구현 진행” 답변으로
[P21 계약](ALIAS_STAGE_B_PROPOSAL.md)·[55-production EBNF](GRAMMAR_STAGE_B_ALIAS.ebnf)를 적용했다.
D01~D05/P01~P20·Canonical·원본 148개를 보존한다.

## 구현

- top-level [visibility] type IDENT = type end를 지원한다. AST/HIR은 type/name/equals 및 whole byte Span과
  target type child 하나를 보존한다. HIR은 source spelling·XID 토큰 양쪽 경계·순서·parent/UTF-8 포함·trivia gap을 검증한다.
- alias RHS의 END type context를 추적해 multiline generic/Tuple을 처리한다. target 뒤 {는 body continuation이 아니다.
  END/semicolon/EOF·다음 item 복구에서 alias delimiter/context를 정리하며 Scanner/raw token은 변경하지 않았다.
- Resolver type namespace에 alias를 수집하고 forward/import·원 DefId·visibility·type/value 원자 import를 유지한다.
  alias RHS는 선언 module scope에서 해석한다. 같은 이름의 함수/지역 값과 type alias를 구분한다.
- TypeChecker는 반복형 graph DFS/SCC와 dependency postorder로 alias를 정규화한다. cyclic alias마다 RHS 전체 N2103과
  SCC 선언 순서의 secondary name labels를 제공하고 ErrorType을 캐시해 파생 진단을 억제한다.
  1,024개 alias chain을 작은 host stack에서 처리하고 첫 1,025번째 이름은 N8901·note를 제공한다.
- Checked aliases는 원 alias DefId→canonical TypeId 표다. Primitive/String/Unit/Tuple·concrete Option/Result·nominal struct/Enum을
  기존 interner/identity로 재사용한다. nominal by-value cycle·Copy 제한·mixed layout/expansion/const budget은 기존 검사를 따른다.
- alias는 value binding·constructor/variant head를 만들지 않는다. 실제 동명 함수는 호출할 수 있다.
  별칭만 존재하는 A(...)와 A::V는 원 call/path Span N1102다. private nominal 대상의 opaque factory signature를 허용한다.
- public Resolved/Checked metadata는 기존 전체 재계산 gate로 검증하며 새 alias kind/map/definition types도 비교한다.
  MIR은 alias 선언을 건너뛰고 canonical 타입·상수·cast·nominal IDs만 사용한다. 새 Runtime 타입·effect·LLVM ABI는 없다.

## 추가 테스트

기본 12개·실제 LLVM 1개·Windows Native 2개, 총 15개를 추가했다.

- Lexer 1: alias generic/Tuple END·다음 비교·semicolon/복구·{ 경계·lossless raw source.
- Parser 2: exact type/name/equals/whole spans·visibility·nested comment·모든 UTF-8 prefix·다음 함수 복구,
  256 KiB stack의 1,024개 선언과 기존 generic depth 128 한도.
- HIR 1: 외부 AST의 child 수·source spelling·identifier/keyword 경계·trivia gap·UTF-8 source shape 검증.
- TypeChecker 4: 부정 16개 exact UTF-8 Span·cycle cascade/secondary order·primitive canonical identity·sum cache·const/default,
  declaration scope·private/opaque factory·dual import 실패 원자성·forward/import/cross-file SCC·nominal cycle,
  1,024/1,025 chain·exponential Tuple expansion·String payload Copy 제한.
- MIR 2: alias runtime body/local/effect 부재·기존 canonical CFG 검증·Resolved/Checked alias metadata 위조 거부.
- CLI 1: 두 정상 fixture check·부정 16개의 check/build/run·도구 호출 전 차단·기존 output 보존.
- LLVM 2: canonical private ABI deterministic IR·실제 COFF/ELF O0/O2 객체.
- Native 2: 두 파일 수용 8줄·multiline·opaque factory/선언 scope·mixed scalar/Tuple/default/const ABI,
  cross-file checked overflow의 원 UTF-8 file/span과 Abort 전 출력.

## 검증 결과

- 기본 전체 회귀: **348 PASS / 0 FAIL / 67 ignored**.
- 실제 LLVM 전체: **17 PASS / 0 FAIL**, 13.79초. COFF/ELF 객체·O0/O2를 검증했다.
- Windows Native 전체: **50 PASS / 0 FAIL**, 514.92초. debug/release 실제 실행을 검증했다.
- 세 suite 합계 **415 PASS / 0 FAIL**이며 opt-in 67개도 LLVM 17개·Native 50개로 별도 실행했다.
- P21 Native 집중: **2 PASS / 0 FAIL**, 12.65초. debug/release 실제 실행을 검증했다.
- fmt·별도 Runtime rustfmt·clippy -D warnings·all-features PASS.
- 문서 build/validator PASS. P01~P20 계약/EBNF/ledger·D01~D05·Canonical·원본 148개 SHA-256·fixture source byte 보존 PASS.
- 독립 examples/type_aliases.nova check와 debug/release의 8줄 UTF-8 LF·빈 stderr·exit 0 PASS.
  self_cycle check는 N2103·exit 1이며 파생 N2101을 억제했다. Runtime/Cargo/LLVM production을 보존했다.

검증 중 manual AST oracle의 nested-comment 위치와 테스트 bundle의 module 정렬 순서를 정정했다.
CLI/Native oracle은 실제 fixture 파일명을 사용하도록 정정했다. 기존 테스트를 삭제하거나 완화하지 않았다.
원 fixture source byte/Span을 보존했으며 Draft 주석은 초안 작성 시점의 역사 기록이다.

환경: Rust 1.99.0, LLVM 21.1.8, Windows x64 MSVC. MSRV 1.80은 별도 실행하지 않았다.
Linux Native host는 미검증이며 ELF 객체 생성과 구분한다.

## 직접 실행과 후속

[독립 예제](../../examples/type_aliases.nova)·[TESTING.md](../../TESTING.md)를 따른다.
check는 출력 없이 exit 0, debug/release는 [8줄](alias-proposal-fixtures/README.md)·LF·빈 stderr·exit 0이다.
generic alias·newtype·alias constructor/variant head·API leak/export 정책·Array·method·일반 Move/Drop와
전체 D06/D10/D11/D12/D16/D30은 후속이다.
