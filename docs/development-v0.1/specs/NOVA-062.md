# NOVA-062 — Generic 제약 증명 사양서

| 항목 | 값 |
|---|---|
| 대상 | Nova 0.1 |
| 작성일 | 2026-10-03 |
| 문서 상태 | Draft — 기존 결정은 유지, 추가 상세는 승인 대기 |
| 적용 Stage | D |
| 근거 | [원본 NOVA-062](../../06_Interfaces_Generics/NOVA-062_Generic_제약_증명_사양서.md) |

이 문서는 원본을 대체하는 확정 사양이 아니다. 기존 확정 기준은 [CANONICAL](../CANONICAL.md)을 따르며, 새 의미·문법·API·정책은 [DECISIONS](../DECISIONS.md)의 승인이 필요하다. [전체 색인](../INDEX.md).

## Constraint 증명
Generic declaration은 type parameter와 where/interface constraint를 가진다. 구체 call에서 inferred type가 각 constraint를 만족하는지 registry로 증명한다. constraint를 unchecked LLVM cast로 대신하지 않는다.

## 초안 — D15
coherence는 같은 type/interface pair에 유일 implementation, orphan 규칙은 type 또는 interface 중 하나가 current package 소유여야 함을 제안한다. blanket/conditional implementation overlap은 보수적으로 오류다.

## 검증
충족/누락 impl, generic body에서 constraint 없는 method 사용 fail, 조건부 impl cycle, package 간 overlap을 다룬다. 진단은 generic 선언 constraint와 call-site type를 함께 표시한다.
