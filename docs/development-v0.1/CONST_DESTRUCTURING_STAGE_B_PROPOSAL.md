# Stage B 함수 내부 Copy const 구조 분해 계약 — P28

작성일: 2026-10-12. 상태: **Accepted / 구현·검증 완료**.
사용자 “다음 개발 승인할테니 개발 진행해줘”에 따라 다음 단계로 이 제한된 P28 계약을 먼저 기록하고 구현한다.
P01~P27·D01~D05·Canonical·원본 148개를 보존한다. 전체 D09/D10/D12/D16/D25/D30 승인이 아니다.

## Specification Change Proposal

- 관련 문서: NOVA-014/029/032/035/037/038/044/046/047/048/074/076/078/081/083/087/091/094/095/136,
  [P05 상수 평가](CONST_STAGE_B_PROPOSAL.md), [P06 전역 const](GLOBAL_CONST_STAGE_B_PROPOSAL.md),
  [P26 let](DESTRUCTURING_STAGE_B_PROPOSAL.md), [P27 var](VAR_DESTRUCTURING_STAGE_B_PROPOSAL.md).
- 현재 사양: P26/P27은 local let/var Copy 구조 분해를 구현했고 기존 const는 단일 IDENT 선언만 허용한다.
- 발견된 문제: aggregate 상수의 여러 component를 local compile-time 상수로 선언하려면 공통 initializer 평가·binder 저장·실패 전파를 정의해야 한다.
- 제안 변경: 함수 block의 `const destructuring_pattern [:type] = expression`을 허용한다. 각 binder를 immutable compile-time Copy const로 저장한다.
- 변경 이유: 기존 Copy pattern과 bounded const engine을 연결한다. const 함수를 실행하거나 LLVM folding으로 상수 의미를 정하지 않는다.
- 영향 범위: Parser/AST/HIR Source·Resolver constant owner·Type Checker/const cached values·MIR direct constant binding·검증·문서/fixture.
- Backward Compatibility: let/var/match·단일 IDENT const·전역 const의 기존 의미/진단/예산을 유지한다.
  P26/P27 const_pattern.nova는 원 source/metadata를 보존하고 P28 성공 override를 별도 기록한다.
- 대안: global/parameter 구조 분해·const 함수·String/Move component까지 함께 구현한다. 이번에는 local Copy subset만 적용한다.

## 승인 범위

1. P27 62개 production 중 binding 하나의 compound keyword를 `(let | var | const)`로 확장하며 **나머지 61개 보존**한다.
   root Tuple/Unit/qualified variant·nested pattern·annotation·필수 initializer·END는 P26/P27 그대로다.
   global compound const와 visibility 붙은 global compound const는 N1102이며 성공 Root/Visible AST가 되지 않는다.
   기존 `const _`는 IDENT 상수이며 compound 안의 `_`만 discard다.
2. initializer는 기존 scope에서 먼저 해석·검사한다. 전체 binder는 source depth-first order로 현재 block에 선언한다.
   immutable/constant flags·duplicate N2002·ancestor shadow·self/forward local reference·escape N2001·mutation N3004는 기존 규칙을 따른다.
3. 전체 initializer 기대 타입·Copy component·nominal identity·irrefutability는 P26과 같다.
   initializer 값이 알려져도 Bool literal·Option/Result/multi-variant constructor는 N3103이다.
   pattern/type/name/arity/resource 오류가 있으면 initializer 계산과 파생 const 오류를 억제한다.
   const-context try는 기존 P16 N3201이며 성공 값을 만들지 않는다.
4. 유효한 pattern을 확인한 뒤 기존 P05~P23 const engine으로 initializer 전체를 **한 번 평가**한다.
   root/group/literal/name·checked numeric/cast·Bool short-circuit·Copy Tuple/Struct/Enum/Option/Result 생성과 기존 projection/exists만 허용한다.
   let/var/parameter 이름·사용자/Read method/print 호출·보간은 N3201이다. 생략되는 논리 RHS도 허용성·타입·node 예산에 포함한다.
   named constructor label은 기존 P23대로 node 예산에서 제외한다. 바깥/이전/global const와 import는 기존 문맥을 사용한다.
5. initializer당 expression node 예산 **10,000**은 pattern/binder 수와 독립이다. binder마다 다시 세거나 다시 평가하지 않는다.
   성공하면 statement별 whole initializer Value/nodes와 각 binder의 projected cached ConstValue/nodes를 저장한다.
   binder nodes는 공유 initializer의 평가량 기록이며 새 계산 예산이 아니다. 뒤 const가 binder를 참조하면 기존 cached name 한 node로 센다.
   모든 discard/Unit도 계산한다. return 뒤/while false/loop body의 source const도 compile-time 검사한다.
6. 첫 평가 실패는 원 operation/name/call Span에 N3201, 첫 expression 10,001번째 node는 N3202다.
   secondary는 compound const 선언 전체이며 node limit note를 포함한다. 모든 binder는 실패 상태/ErrorType으로 전파한다.
   뒤 참조의 파생 평가 오류를 억제한다. pattern node 10,000·matrix task 100,000·live cell 1,000,000·nesting/layout 예산은 별도로 유지한다.
7. MIR은 도달한 declaration 위치에서 cached component Constant를 binder local에 직접 저장한다.
   initializer 산술/호출/aggregate 생성·tag branch/payload projection을 Runtime에서 다시 수행하지 않는다. discard/Unit-only는 Runtime statement가 필요 없다.
   공개 ConstEvaluation/flags/paths/nodes/값은 원 HIR/Resolved에서 재계산하고 private full-body/source proof로 cached 값·binder 순서·Source·body 변조를 거부한다.
   LLVM/Runtime production·ABI/layout·Lexer/END·새 diagnostic code는 변경하지 않는다.
8. const는 Copy component에 한정한다. String/Move aggregate element·전역/parameter/for/assignment 구조 분해·refutable else/guard·struct/or/range/general literal pattern·const 함수·Array·일반 Move/borrow/Drop은 제외한다.

## 수용 계획

[전용 EBNF](GRAMMAR_STAGE_B_CONST_DESTRUCTURING.ebnf)·고정 UTF-8/LF source/진단 Span·독립 예제를 제공한다.
scope/flags/const dependencies·known-value irrefutability·const 실패/cascade·두 예산·unreachable const·Source/Checked/MIR forgery·LLVM COFF/ELF O0/O2·Windows Native debug/release를 검증한다.
[구현 기록](CONST_DESTRUCTURING_IMPLEMENTATION.md)에 실제 결과를 기록했고 [직접 테스트 명령](../../TESTING.md)을 제공한다.
