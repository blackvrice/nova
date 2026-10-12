# P29 전역 Copy const 구조 분해 계약

상태: **Accepted / 구현·검증 완료**. 작성일: 2026-10-12.
사용자 “다음 개발 승읺 ㅏㄹ테니 개발 진행해줘”의 다음 단계 승인을 받아 이 제한된 계약을 구현 전에 기록한다.
전체 D09/D10/D12/D16/D25/D30 승인이 아니다. P01~P28 및 원본 148개는 보존한다.

## Specification Change Proposal

P28 local Copy const 구조 분해를 단일/다중 파일의 전역 선언으로 확장한다.
관련 계약: P06 전역 상수, P11 module/import, P26 Copy irrefutable pattern, P28 단일 const initializer 평가.
전역 aggregate의 component 이름을 import하고 다른 전역 상수/default에서 참조하려면 선언 소유권·의존성·가시성을 정의해야 한다.
대안인 component별 initializer 재평가는 공통 initializer의 평가량과 순환 의미를 바꾸므로 사용하지 않는다.

## 승인 범위

1. root 및 visibility 선언에서 `const destructuring_pattern [:type] = expression`을 허용한다.
   P28 EBNF global_const_decl 하나만 확장하며 나머지 **61개 보존**한다. 전역 let/var는 허용하지 않는다.
   Tuple/Unit/단일 variant Enum 및 중첩 Copy irrefutable pattern은 P26/P28 그대로다. N3103·타입 annotation·END도 유지한다.
2. 모든 component 이름을 module scope에 먼저 수집한다. 전역/다중 파일 forward reference를 허용한다.
   각 binder는 immutable GlobalConst 정의이고 별도의 declaration owner를 공유한다. discard에는 정의가 없다.
   선언 visibility가 모든 component에 적용된다. 직접 item import/alias만 허용하며 재export를 확장하지 않는다.
   duplicate와 reserved global const print는 N2002다. 기존 사용자 함수 print는 허용한다.
3. 의존성 graph는 initializer 선언당 vertex 하나다. 각 component reference는 공유 owner로 연결된다.
   생략되는 Bool RHS도 정적 edge에 포함한다. 자기 component 참조 및 cross-file SCC는 N3202다.
   기존 source-order cycle 진단 규칙을 유지한다. 순환 group의 모든 component와 whole cache는 Failed/nodes 0이다.
4. 의존성 순서대로 전체 initializer를 한 번 평가하고 cached component 값을 저장한다.
   오류가 있는 group 전체를 실패/Invalid로 처리하고 의존 group의 파생 오류를 억제한다.
   binder 없는 Unit/discard도 검사·평가한다. P05/P28의 initializer당 **10,000** expression node,
   pattern 10,000·matrix task **100,000**·live cell **1,000,000** 및 nesting/layout 한도를 유지한다.
   새로운 전역 전체 예산은 추가하지 않는다. 함수 호출/try 등 기존 const 금지는 N3201이다.
5. local const·함수 default·import·Native에서 cached component를 사용한다. MIR은 전역 초기화를 실행하지 않는다.
   Source/Resolved owner·Checked cache·MIR proof는 원 source에서 재계산하여 변조를 거부한다. LLVM ABI/Runtime production은 변경하지 않는다.
6. P28 global_pattern.nova와 visible_global_pattern.nova의 source/기존 N1102 metadata를 보존한다.
   P29 현재 성공 결과는 별도 역사 override로 기록한다. 다른 기존 의미와 진단은 유지한다.
7. String/Move component·Array·parameter/for/assignment 구조 분해·const 함수·일반 Move/borrow/Drop은 제외한다.

## 수용 계획

forward·component import/alias·private 접근·정적 순환·duplicate·discard 실패·기대 타입·default·cached MIR·Source 변조를 검증한다.
전용 고정 UTF-8 fixture와 실행 예제를 제공하고 fmt/check/clippy·전체 기본 tests·LLVM O0/O2·Windows Native debug/release 결과를 기록한다.
