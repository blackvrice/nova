# NOVA-089 — Generic 특수화 Pass 구현서

| 항목 | 값 |
|---|---|
| 대상 | Nova 0.1 |
| 작성일 | 2026-10-03 |
| 문서 상태 | Draft — 기존 결정은 유지, 추가 상세는 승인 대기 |
| 적용 Stage | D |
| 근거 | [원본 NOVA-089](../../08_MIR_Middleend/NOVA-089_Generic_특수화_Pass_구현서.md) |

이 문서는 원본을 대체하는 확정 사양이 아니다. 기존 확정 기준은 [CANONICAL](../CANONICAL.md)을 따르며, 새 의미·문법·API·정책은 [DECISIONS](../DECISIONS.md)의 승인이 필요하다. D01~D05 Lexer 상세는 [승인 기준](../ACCEPTED_LEXER.md)을 따른다. [전체 색인](../INDEX.md).

## Specialization pass
type substitution를 typed HIR/MIR에 적용하고 concrete type/obligation를 재검증한다. call graph에서 필요한 specialization을 queue에 추가한다. generic body마다 새 syntax parser를 실행하지 않는다.

## key/symbol
canonical type와 owning package identity를 사용하고 mangling NOVA-097과 일치시킨다. 같은 specialization의 concurrent 요청은 한 결과를 공유한다.

## 검증
cross-module dedup, inference failure source label, recursive growth detection, foreign ABI generic exclusion, cache invalidation/serial-parallel equivalence.
