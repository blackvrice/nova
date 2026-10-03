# NOVA-085 — Borrow Checker 구현서

| 항목 | 값 |
|---|---|
| 대상 | Nova 0.1 |
| 작성일 | 2026-10-03 |
| 문서 상태 | Draft — 기존 결정은 유지, 추가 상세는 승인 대기 |
| 적용 Stage | C |
| 근거 | [원본 NOVA-085](../../08_MIR_Middleend/NOVA-085_Borrow_Checker_구현서.md) |

이 문서는 원본을 대체하는 확정 사양이 아니다. 기존 확정 기준은 [CANONICAL](../CANONICAL.md)을 따르며, 새 의미·문법·API·정책은 [DECISIONS](../DECISIONS.md)의 승인이 필요하다. D01~D05 Lexer 상세는 [승인 기준](../ACCEPTED_LEXER.md)을 따른다. [전체 색인](../INDEX.md).

## NLL 구현
Ref creation에서 Loan(place,mode,origin)을 만들고 use points, CFG reachability와 region constraints로 live range를 계산한다. lexical scope 전체를 무조건 lifetime으로 사용하지 않는다.

## conflict
loan live point에서 overlapping Read/change, mutation, take, Drop을 검사한다. struct field disjoint 증명, reborrow suspend, external alias uncertainty를 PlaceRelation에 분리한다. dynamic index는 보수적이다.

## 검증
last-use 직후 mutation pass, branch-specific loan, loop NLL, returning local view fail, reborrow parent access fail, SplitAt non-overlap pass. soundness 반례를 최소 regression으로 보존한다.
