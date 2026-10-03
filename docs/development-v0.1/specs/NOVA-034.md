# NOVA-034 — Class 객체 모델·정체성·배치 사양서

| 항목 | 값 |
|---|---|
| 대상 | Nova 0.1 |
| 작성일 | 2026-10-03 |
| 문서 상태 | Draft — 기존 결정은 유지, 추가 상세는 승인 대기 |
| 적용 Stage | D |
| 근거 | [원본 NOVA-034](../../03_Types_Declarations/NOVA-034_Class_객체_모델_정체성_배치_사양서.md) |

이 문서는 원본을 대체하는 확정 사양이 아니다. 기존 확정 기준은 [CANONICAL](../CANONICAL.md)을 따르며, 새 의미·문법·API·정책은 [DECISIONS](../DECISIONS.md)의 승인이 필요하다. D01~D05 Lexer 상세는 [승인 기준](../ACCEPTED_LEXER.md)을 따른다. [전체 색인](../INDEX.md).

## Class 의미
Class는 owned handle이고 구현 상속을 지원하지 않는다. 이동은 객체를 복사하는 것이 아니라 handle 소유권을 이전한다. 마지막 owner가 파괴될 때 field Drop 뒤 storage를 해제한다.

## 상세 초안 — D12/D23
일반 ==가 pointer identity인지 값 equality인지 원본에 없으므로 D12에서 별도 identity API를 제안한다. read receiver는 내부 수정 금지, change receiver는 독점 수정, take receiver는 소비다. shared conversion은 ownership를 명시적으로 이전한다.

## 구현
object layout과 handle ABI를 분리하고 allocation failure는 D18 정책을 따른다. owner/refcount/view를 한 handle 종류로 혼용하지 않는다.

## 검증
이동 후 동일 object identity, clone 없이 중복 owner 생성 거부, field 역순 Drop와 최종 free, weak upgrade 실패 후 dangling 접근 금지.
