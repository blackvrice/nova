# NOVA-041 — Effect·pure·noPanic 사양서

| 항목 | 값 |
|---|---|
| 대상 | Nova 0.1 |
| 작성일 | 2026-10-03 |
| 문서 상태 | Draft — 기존 결정은 유지, 추가 상세는 승인 대기 |
| 적용 Stage | A~B |
| 근거 | [원본 NOVA-041](../../04_Functions_Control/NOVA-041_Effect_pure_noPanic_사양서.md) |

이 문서는 원본을 대체하는 확정 사양이 아니다. 기존 확정 기준은 [CANONICAL](../CANONICAL.md)을 따르며, 새 의미·문법·API·정책은 [DECISIONS](../DECISIONS.md)의 승인이 필요하다. D01~D05 Lexer 상세는 [승인 기준](../ACCEPTED_LEXER.md)을 따른다. [전체 색인](../INDEX.md).

## Effect 계약
pure와 noPanic은 이름만으로 LLVM attribute에 대응시키지 않는다. 순수성이 반환값, 외부 mutation/I/O/allocation/abort 중 무엇을 제한하는지 D14에서 명확히 결정한다. noPanic은 공식 keyword 표에 없으므로 표기부터 검토한다.

## 초안 — D14
pure는 외부 I/O, foreign effect, global mutation, change parameter mutation을 금지하는 보수적 검사를 제안한다. noPanic은 모든 reachable operation/callee가 bounds/overflow/explicit panic을 일으키지 않는 증명이 필요하다.

## 구현과 검증
EffectTable과 call graph fixed-point로 전파한다. unknown foreign effect는 effectful로 간주한다. pure 함수의 print fail, noPanic 함수의 unchecked arithmetic fail, recursive pure SCC pass를 다룬다. MVP에서 지원 여부는 별도 결정이다.
