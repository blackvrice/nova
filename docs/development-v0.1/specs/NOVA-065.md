# NOVA-065 — Generic Cache·코드 팽창 제어 사양서

| 항목 | 값 |
|---|---|
| 대상 | Nova 0.1 |
| 작성일 | 2026-10-03 |
| 문서 상태 | Draft — 기존 결정은 유지, 추가 상세는 승인 대기 |
| 적용 Stage | D |
| 근거 | [원본 NOVA-065](../../06_Interfaces_Generics/NOVA-065_Generic_Cache_코드_팽창_제어_사양서.md) |

이 문서는 원본을 대체하는 확정 사양이 아니다. 기존 확정 기준은 [CANONICAL](../CANONICAL.md)을 따르며, 새 의미·문법·API·정책은 [DECISIONS](../DECISIONS.md)의 승인이 필요하다. D01~D05 Lexer 상세는 [승인 기준](../ACCEPTED_LEXER.md)을 따른다. [전체 색인](../INDEX.md).

## Cache 경계
in-memory specialization registry와 disk artifact cache를 구분한다. key는 canonical types와 body/dependency fingerprint, compiler/runtime/Target/options를 포함한다.

## 예산 초안 — D30
specialization depth/count/총 IR budget을 설정하고 초과는 가장 긴 확장 chain과 함께 진단한다. dedup와 dead code 제거를 우선하고 ABI를 바꾸는 type erasure는 0.1에 추가하지 않는다.

## 검증
body 변경 invalidation, unchanged alias reuse, Target 차이 miss, 손상 cache safe rebuild, 동일 input/options에서 count/order 같음을 확인한다. 성능 경고와 의미 오류를 구분한다.
