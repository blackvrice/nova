# P23 Copy struct 생성자 이름 인수 구현·검증 기록

2026-10-08 사용자 “P23 승인하고 생성자 이름 인수 구현 진행” 답변으로 [최소 계약](STRUCT_NAMED_ARGUMENTS_STAGE_B_PROPOSAL.md)을 승인했다.
상태: **Accepted / 구현·검증 완료**. 문법은 P22의 58개 production을 raw bytes 변경 없이 재사용한다.

## 구현과 API 경계

- lexical value 우선과 nominal constructor 선택을 유지한다. 원 FieldId만 대응 대상으로 삼고 method·receiver/default slot을 제외한다.
  위치 prefix 이후 이름 인수는 field를 정확히 한 번 채운다. 원 import alias·field self·Unicode XID spelling을 지원한다.
- 기존 함수/메서드 대응 절차를 storage field 목록에도 적용했다. Checked.named_constructors와 NamedConstructor는
  원 StructId·source-order HirId 목록·source-index→FieldId 대응을 보존한다. named_calls/defaults와 별도 table이다.
  기대 타입은 대응 field, source secondary는 그 type annotation이다. 순수 위치 생성의 기존 진단/Span은 유지한다.
- 전체 constructor field 접근 권한을 이름 대응 전에 확인한다. private field가 있는 외부 생성은 N2004이며
  N2201/known-label note를 추가하지 않는다. factory의 같은 module 이름 생성과 opaque 반환은 가능하다.
  잘못된 label에는 임의 기대 타입을 주지 않으며 원 name/type 실패의 파생 오류를 억제한다.
- const evaluator는 NamedArgument wrapper를 0-node로 통과한다. constructor 1+기존 값 tree 비용이므로 위치/이름 생성 비용이 같다.
  값은 source order로 평가/변환하고 ConstValue::Struct는 declaration field order로 저장한다.
  skipped RHS permission/dependency·cached name·checked N3201·10,000-node/N3202·P18 선언 module default를 유지한다.
- MIR은 제공 값을 source order로 평가·변환·snapshot한 다음 field order로 Aggregate operand를 배치한다.
  Checked/Resolved 재계산 gate, private NamedConstructorCertificate와 caller full-body proof를 결합했다.
  원 constructor/field/argument identity·registry/layout·snapshot source와 단일 write·slot 완전성·aggregate statement를 독립 검증한다.
  동일 타입 operand 교환이나 effect 재배열도 거부한다. try Error/Abort 후 뒤 인수/aggregate는 실행되지 않는다.
- Lexer/END·Parser/AST/HIR production·Runtime source/API·LLVM adapter production·Cargo·공용 ABI를 변경하지 않았다.
  label lookup은 argument+field 수에 선형이며 validator는 write counts를 한 번 수집한다. 기존 자원 한도를 유지한다.

## 테스트와 실제 결과

| 검사 | 결과 |
|---|---|
| cargo fmt --check | PASS |
| cargo check --workspace --all-features --offline | PASS |
| cargo clippy --workspace --all-targets --all-features --offline -- -D warnings | PASS |
| cargo test --workspace --offline | 372 PASS / 0 FAIL / 73 ignored |
| Runtime stage_a.rs rustfmt | PASS |
| LLVM 전체 opt-in | 19 PASS / 0 FAIL, COFF/ELF O0/O2 |
| Native 전체 opt-in | 54 PASS / 0 FAIL, debug/release |
| 문서 validator | PASS |

총 445 PASS / 0 FAIL다. 새 테스트 16개는 기본 13·LLVM 1·Native 2다.
[두 파일 fixture](struct-named-arguments-proposal-fixtures/README.md)의 원 source bytes를 보존하며
정상 main/추가 2·부정 24·Runtime 1의 실제 코드/UTF-8 Span/cascade/20줄을 검증했다.
Node 비용 10,000/10,001을 global const와 parameter default에 적용하고 label 이름이 dependency가 아님을 검증했다.
1,024/1,025 field 경계·역순 1,024개 이름 인수의 complete MIR snapshot proof를 검증했다.
Source의 UTF-8 split/colon 변조·prefix recovery, public mapping 삭제/field/owner/argument 변조,
same-type operand 교환·snapshot duplicate/source 변경·effect CFG 변경·body 삭제·try 진입 우회를 거부했다.
Char/Float/Unit/Tuple/Option/Result·private factory·기본값·nominal import alias·checked overflow를 Native에서도 검증했다.

기존 P17 struct_label 부정 기록은 그 승인 시점 기록이다. P23가 해당 생성자를 확장하므로
현재 source는 성공한다. 기존 raw fixture/기대 기록을 보존하고 P17 회귀에 P23 성공 override를 명시했다.
문서 승인/기계 검증/Compiler/Native 실행은 구별한다. MSRV 1.80·Linux Native host 실행은 이번에 검증하지 않았다.

## 직접 사용과 후속 범위

[독립 예제](../../examples/struct_named_arguments.nova)와 [실행 명령](../../TESTING.md)을 제공한다.
explicit init·field initializer/default·Enum/Option/Result named payload·generic constructor·Array·일반 Move/borrow/Drop·
change/take·전체 D09/D11/D12/D16/D30은 후속 계약이다. 기존 P01~P22·Canonical·원본 148개는 보존한다.
