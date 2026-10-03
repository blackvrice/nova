# NOVA-064 — Monomorphization·특수화 사양서

| 항목 | 값 |
|---|---|
| 대상 | Nova 0.1 |
| 작성일 | 2026-10-03 |
| 문서 상태 | Draft — 기존 결정은 유지, 추가 상세는 승인 대기 |
| 적용 Stage | D |
| 근거 | [원본 NOVA-064](../../06_Interfaces_Generics/NOVA-064_Monomorphization_특수화_사양서.md) |

이 문서는 원본을 대체하는 확정 사양이 아니다. 기존 확정 기준은 [CANONICAL](../CANONICAL.md)을 따르며, 새 의미·문법·API·정책은 [DECISIONS](../DECISIONS.md)의 승인이 필요하다. D01~D05 Lexer 상세는 [승인 기준](../ACCEPTED_LEXER.md)을 따른다. [전체 색인](../INDEX.md).

## Monomorphization
Generic 구현은 구체 type substitution별 코드 생성이다. key=(DefId,canonical type arguments,Target ABI,semantic options)를 사용하고 동일 key는 한 번만 생성한다.

## 작업 queue
reachable root에서 specialization 요구를 수집하고 stable key 순서로 처리한다. 동일 key 재귀는 허용하되 T→Array<T>처럼 무한 성장하는 chain은 오류다. declaration/query origin을 함께 보존한다.

## 검증
alias 동일 key dedup, recursive 동일 specialization pass, growing specialization fail, module order 변경에도 symbol/key 같음, unused generic code 미생성을 검사한다. generic Error HIR는 codegen에 넣지 않는다.
