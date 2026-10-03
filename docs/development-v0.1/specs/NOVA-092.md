# NOVA-092 — Debug MIR 출력 형식 사양서

| 항목 | 값 |
|---|---|
| 대상 | Nova 0.1 |
| 작성일 | 2026-10-03 |
| 문서 상태 | Draft — 기존 결정은 유지, 추가 상세는 승인 대기 |
| 적용 Stage | B~E |
| 근거 | [원본 NOVA-092](../../08_MIR_Middleend/NOVA-092_Debug_MIR_출력_형식_사양서.md) |

이 문서는 원본을 대체하는 확정 사양이 아니다. 기존 확정 기준은 [CANONICAL](../CANONICAL.md)을 따르며, 새 의미·문법·API·정책은 [DECISIONS](../DECISIONS.md)의 승인이 필요하다. [전체 색인](../INDEX.md).

## MIR dump schema 초안
header: schema/compiler/Target/pass/body ID. locals: _N:type,mode,scope,source. blocks: bbN { statements; terminator }. projections는 field/index/deref와 byte Span을 별도 필드로 출력한다.

## 결정성
memory address/hash-map iteration/time를 출력하지 않는다. local/block 번호는 canonical traversal order로 normalize하되 semantics-affecting operand order는 유지한다. before/after pass 이름을 기록한다.

## 검증
같은 입력 두 번 byte 동일, parallel jobs 동일, phi 없는 loop CFG, Call/Drop successor와 SourceInfo 표시. snapshot schema 변경은 version과 승인된 baseline update를 요구한다.
