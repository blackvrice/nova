# NOVA-004 — Nova 용어·키워드 Canonical 표

| 항목 | 값 |
|---|---|
| 대상 | Nova 0.1 |
| 작성일 | 2026-10-03 |
| 문서 상태 | Draft — 기존 결정은 유지, 추가 상세는 승인 대기 |
| 적용 Stage | 전 Stage |
| 근거 | [원본 NOVA-004](../../00_Governance/NOVA-004_Nova_용어_키워드_Canonical_표.md) |

이 문서는 원본을 대체하는 확정 사양이 아니다. 기존 확정 기준은 [CANONICAL](../CANONICAL.md)을 따르며, 새 의미·문법·API·정책은 [DECISIONS](../DECISIONS.md)의 승인이 필요하다. D01~D05 Lexer 상세는 [승인 기준](../ACCEPTED_LEXER.md)을 따른다. [전체 색인](../INDEX.md).

## 확정 어휘
공식 키워드와 Primitive 이름은 원본 NOVA-004 및 canonical_decisions.json을 그대로 따른다. func, foreign, interface, change, take 표기를 일관되게 쓴다. Read는 modifier 부재의 의미이며 별도 소스 키워드가 아니다.

## 정규화
byte=uint8, int=int32, uint=uint32, float=float32, double=float64, void/생략 반환=Unit, ()=Unit 값, T?=Option<T>. 내부 명칭은 Unit/Read/Change/Take를 사용하고 소스 출력에서는 원본 철자를 유지한다.

## 누락과 추가 제안
use는 NOVA-072에 있으나 공식 목록에 없으므로 D01로 추적한다. type 별칭 구문, noPanic, library 및 @symbol의 분류도 D01/D11/D17 검토 전 확정 키워드로 추가하지 않는다. 이름 스타일은 lint 대상이며 문법적 거부 여부는 별도 결정한다.

## 완료
lexer keyword 표, formatter 출력, 사용자 문서의 어휘가 같은 표에서 생성되는지 검사한다.
