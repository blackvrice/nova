# NOVA-084 — 초기화·Move 분석 구현서

| 항목 | 값 |
|---|---|
| 대상 | Nova 0.1 |
| 작성일 | 2026-10-03 |
| 문서 상태 | Draft — 기존 결정은 유지, 추가 상세는 승인 대기 |
| 적용 Stage | C |
| 근거 | [원본 NOVA-084](../../08_MIR_Middleend/NOVA-084_초기화_Move_분석_구현서.md) |

이 문서는 원본을 대체하는 확정 사양이 아니다. 기존 확정 기준은 [CANONICAL](../CANONICAL.md)을 따르며, 새 의미·문법·API·정책은 [DECISIONS](../DECISIONS.md)의 승인이 필요하다. [전체 색인](../INDEX.md).

## Dataflow 구현
CFG predecessor/successor worklist로 initialization/move 상태의 least fixed point를 계산한다. finite lattice와 monotone transfer를 사용하고 loop에서도 종료해야 한다.

## 단위
Move path는 root와 field initialization 상태를 추적한다. 사용자 partial move 금지는 별도 semantic check지만 constructor/Array 내부 init tracking은 유지한다. use 지점과 move origin의 secondary label을 연결한다.

## 검증
diamond merge, loop-carried owner, unreachable block 제외, reinitialize then read pass, moved then borrowed fail, analysis iteration 순서 바꿔 동일 결과.
