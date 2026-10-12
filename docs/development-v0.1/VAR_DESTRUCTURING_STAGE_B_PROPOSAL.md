# Stage B 가변 Copy var 구조 분해 계약 — P27

작성일: 2026-10-12. 상태: **Accepted / 구현·검증 완료**.
사용자는 “다음 개발 진행해줘”에 이어 “또한 다음개발 작업 승인할께”라고 다음 단계 진행과 승인을 명시했다.
이 승인으로 P26에 이어 아래 제한된 P27을 진행한다. 이전 P01~P26·D01~D05·Canonical·원본 148개는 보존한다.

## Specification Change Proposal

- 관련 문서: NOVA-014/029/035/037/038/044/046/047/048/074/076/078/081/083/087/091/094/095/136,
  [P04 대입](CONTROL_STAGE_B_PROPOSAL.md), [P12 Copy struct](STRUCT_STAGE_B_PROPOSAL.md), [P13 Tuple](TUPLE_STAGE_B_PROPOSAL.md),
  [P26 let 구조 분해](DESTRUCTURING_STAGE_B_PROPOSAL.md).
- 현재 사양: P26은 함수 block의 immutable let Copy 구조 분해만 구현했다. 일반 var IDENT와 가변 경로 대입은 이미 구현되어 있다.
- 발견된 문제: Copy product/single-variant sum을 여러 가변 local로 분해하는 선언은 P26에서 N1102로 제외되어 있었다.
- 제안 변경: `var destructuring_pattern [:type] = expression`을 local block에 허용한다. 모든 실제 binder를 mutable Copy local로 선언한다.
- 변경 이유: 기존 Copy destructuring과 var 대입 규칙을 연결한다. 새 ownership나 실패하는 runtime pattern을 추가하지 않는다.
- 영향 범위: Parser/AST/HIR Source·Resolver mutable flags·Checked source plan·기존 MIR/LLVM 회귀·문서/fixture.
- Backward Compatibility: 기존 let binder·match binder는 계속 immutable이다. P26의 var_pattern.nova source/metadata는 역사로 보존하되 현재 동작은 성공으로 명시한다.
- 대안: binder마다 mode/가변 modifier를 추가하거나 const 구조 분해까지 확대한다. 이번에는 declaration 전체 var 하나만 확장한다.

## 승인 범위

1. P26의 62개 production 중 binding 하나만 변경하고 나머지 **61개 보존**한다.
   compound branch의 let keyword를 `(let | var)`로 확장한다. root Tuple/Unit/qualified variant와 nested pattern은 그대로다.
   일반 `let _`/`var _`는 기존 IDENT local이며 discard로 재해석하지 않는다.
2. initializer를 기존 scope에서 먼저 해석·검사한다. 성공 후 전체 binder를 현재 block에 source depth-first order로 선언한다.
   var compound 안의 모든 binder는 mutable이며 `_`는 DefId/local을 만들지 않는다. ancestor shadow 허용·중복 N2002·자기 참조/escape N2001은 P26과 같다.
3. 전체 initializer annotation·Copy element·canonical nominal identity·irrefutability·ErrorType/cascade·자원 예산은 P26을 변경 없이 사용한다.
   알려진 initializer 값이어도 refutable Bool/Option/Result/multi-variant Enum은 N3103이다.
   새 진단 code·Literal context·type promotion·constructor·const/default 의미를 추가하지 않는다.
4. 직접 binder 대입과 기존 tuple/struct 경로 대입은 P04/P12/P13을 따른다.
   mutable root라도 `let` struct field에는 N3004이고 private field에는 N2004다. Read method receiver의 immutable self 규칙도 유지한다.
   binder 값은 initializer Copy snapshot이며 원 aggregate를 참조하거나 그 변경을 반영하지 않는다.
5. initializer 단일 평가·coercion·Copy snapshot·source-order projection/assignment를 유지한다.
   Unit/전체 wildcard는 initializer effect를 보존한다. try Error는 snapshot/binder/뒤 문장 전에 enclosing Result로 반환한다.
   loop에서는 도달한 iteration마다 평가한다. checked Abort는 기존 원 operation Source Span으로 종료한다.
6. AST/HIR/Checked plan은 mutable flag와 실제 let/var keyword Span을 저장한다.
   Source gate는 flag와 keyword spelling을 대조한다. public Resolved/Checked 재계산과 private MIR full-body/projection/tag proof로 mutable flag/DefId/type/path/initializer/source/effect 변조를 거부한다.
   LLVM/Runtime production·ABI/layout·Lexer/END는 변경하지 않는다.
7. pattern nodes **10,000**·matrix tasks **100,000**·live cells **1,000,000**·pattern nesting 128·기존 layout 한도를 유지한다.
   source node 초과는 첫 10,001번째 pattern Span, matrix 초과는 원 var keyword N8901이며 파생 N3103을 억제한다.
8. const/global/parameter/for/assignment 구조 분해·binder별 mutable marker·refutable let/var-else/guard·struct/or/range/general literal pattern·Array·일반 Move/borrow/Drop은 제외한다.
   전체 D08/D10/D12/D16/D25/D30 승인이 아니다.

## 수용 계획

[전용 EBNF](GRAMMAR_STAGE_B_VAR_DESTRUCTURING.ebnf)와 고정 source/UTF-8 Span fixture를 제공한다.
Parser/Source spelling·스코프/mutable identity·경로 대입/Copy snapshot·N3103/cascade·자원 한도·Checked/MIR 변조와 LLVM COFF/ELF O0/O2·Windows Native debug/release를 검증한다.
[구현 기록](VAR_DESTRUCTURING_IMPLEMENTATION.md)에 실제 결과를 기록했고 사용자 [테스트 명령](../../TESTING.md)을 제공한다.
