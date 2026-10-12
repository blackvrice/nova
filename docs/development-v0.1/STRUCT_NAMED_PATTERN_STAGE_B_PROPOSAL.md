# P34 Copy struct 이름 필드 패턴 계약

2026-10-12. 상태: **Accepted / 구현·검증 완료**.
사용자 “다음 작업 승인할테니 개발 작업 진행해줘”의 다음 단계 승인으로 이 제한 계약을 구현 전에 기록한다.
P01~P33·D01~D05·Canonical·원본 148개를 보존한다. 전체 D06/D08/D09/D10/D12/D16/D25/D30 승인이 아니다.

구현·검증 완료: 2026-10-13. 승인일 2026-10-12.

## Specification Change Proposal

관련 NOVA-026/029/035/038/044/046/047/048/073/074/076/078/081/083/087/091/094/095/136 및 P23 이름 생성자·P24 recursive pattern·P25 guard·P26~P29 binding·P33 struct 위치 패턴 계약.
문제: struct 패턴이 선언 순서에 의존하여 많은 field를 분해하거나 순서를 바꾼 Source를 읽기 어렵다.
변경: struct pattern argument에 IDENT: pattern label을 추가하고 type field 이름에 대응한다.
이유: P23의 field label 대응 정책과 P33의 Copy pattern/visibility/coverage/projection을 유지한다.
대안: 이름 전용 brace 문법·shorthand·rest·부분 필드 패턴을 함께 지원한다. 이번에는 기존 괄호 안의 전체 field 명시 subset만 적용한다.
Backward Compatibility: 기존 위치 패턴과 P01~P33 의미/진단을 보존한다. P33의 당시 이름 필드 거부 Parser 테스트는 현재 성공으로 갱신하며 과거 고정 fixture bytes는 변경하지 않는다.

## 승인 범위

1. struct_pattern의 인수를 struct_pattern_arguments로 바꾸고 struct_pattern_arguments/struct_pattern_argument 두 production을 추가한다.
   총 **66개 production / 기존 63개 보존**이다. struct_pattern_argument = [ IDENT, colon ], pattern이다.
   Point(y:flag,x:n), Point(n,y:flag)처럼 이름 순서 변경·위치 prefix 혼합을 지원하며 Empty()·trailing comma·trivia는 P33 그대로다.
   AST/HIR PatternField는 name/colon token Span과 정확히 한 pattern child를 보존한다. struct pattern의 직접 argument에만 허용한다.
   label은 value/binder/type lookup을 하지 않으며 실제 binder는 label 뒤 pattern에만 있다. brace 기반 이름 field 문법·rest/shorthand·.. 생략은 제외한다.
2. field 대응은 P23처럼 선언 이름으로 찾는다. unknown label·같은 field 중복·위치/이름 중복·이름 뒤 위치·누락·초과는 **N2201**이다.
   unknown/duplicate는 label Span, 이름 뒤 위치는 그 pattern Span, missing은 struct pattern 전체 Span과 원 field secondary를 제공한다.
   duplicate는 먼저 대응한 argument secondary를 제공한다. mapping 실패 시 파생 component 타입/coverage/const 오류를 억제한다.
   같은 binder 이름 중복은 label 중복과 구별하여 기존 Resolver N2002다.
3. 모든 field를 정확히 한 번 명시해야 한다. method는 field 이름/수에 포함하지 않는다. nominal/import head·alias annotation만 허용·P33 전체 field 가시성을 그대로 적용한다.
   다른 module의 private field는 이름/wildcard 여부와 무관하게 head N2004다. 별도 부분 접근·private field 생략을 허용하지 않는다.
4. AST/HIR children과 binder 선언/진단 방문은 Source 순서다. Checked struct plan의 children은 선언 field 순서로 정규화한다.
   각 label wrapper와 child는 대응 field의 기대 타입/원 Field Source를 갖는다. wrapper는 값 pattern에 대해 transparent이며 별도 값 constructor가 아니다.
   Bool/Char/정수 literal·nested struct/Tuple/Enum/Option/Result·guard/try·let/var/local/global const를 P33 규칙으로 검사한다.
   위치/이름 순서가 다른 동일한 pattern은 같은 coverage다. missing witness는 기존 P33 위치 형태다.
5. let/var/const는 기존 irrefutable 검사 N3103를 유지한다. 첫 refutable component secondary는 Source 순서다.
   initializer/scrutinee는 한 번 snapshot하고 대응한 field를 projection한다. pattern 검사/projection은 선언 field 순서로 진행한다.
   field 검사에 effect가 없으므로 이름 순서가 guard/body/try 효과를 바꾸지 않는다. guard는 패턴 성공 후 한 번, false 시 다음 arm이다.
   ConstValue Struct fields와 typed plan을 선언 순서로 대응하고 wrapper를 통과하여 기존 binder cache·전역 forward/순환·default 의미를 유지한다.
6. Source name/colon·trivia/delimiter/child 순서·wrapper placement/shape·Checked mapping/type/cache·MIR FieldId/Source/projection/branch 변조를 원 HIR 재계산으로 거부한다.
   MIR은 기존 Project/Place와 pattern CFG를 사용한다. LLVM/Runtime production·ABI/layout·Lexer/END·Cargo dependency·diagnostic registry는 변경하지 않는다.
7. arm 1,025·pattern nodes 10,000·matrix tasks 100,000·live cells 1,000,000·nesting/layout 예산을 유지한다.
   새 label wrapper도 source pattern node 한 개로 센다. matrix는 wrapper를 지워 추가 constructor/domain/task를 만들지 않는다. initializer const node 예산은 그대로다.
   Enum/sum/Tuple 이름 payload·field default·rest/shorthand/or/range·Array·String/Move field·parameter/for/assignment pattern·const 함수·일반 Move/borrow/Drop은 제외한다.

## 수용 계획

이름 순서 변경/위치 혼합·duplicate/unknown/missing·private/method/nominal·mapped field 타입·중첩 sum/product·완전성/도달성·binder scope/mutability·refutable const 억제를 검증한다.
고정 UTF-8 fixture/실제 code별 byte Span, AST/Checked/MIR 변조·wrapper 포함 10,000/10,001 경계·독립 Bool product oracle·LLVM COFF/ELF O0/O2·Windows Native debug/release·예제/사용자 명령을 제공한다.
