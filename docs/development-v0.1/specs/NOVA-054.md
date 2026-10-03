# NOVA-054 — 빌림·수명·Place 충돌 사양서

| 항목 | 값 |
|---|---|
| 대상 | Nova 0.1 |
| 작성일 | 2026-10-03 |
| 문서 상태 | Draft — 기존 결정은 유지, 추가 상세는 승인 대기 |
| 적용 Stage | C |
| 근거 | [원본 NOVA-054](../../05_Ownership_Safety/NOVA-054_빌림_수명_Place_충돌_사양서.md) |

이 문서는 원본을 대체하는 확정 사양이 아니다. 기존 확정 기준은 [CANONICAL](../CANONICAL.md)을 따르며, 새 의미·문법·API·정책은 [DECISIONS](../DECISIONS.md)의 승인이 필요하다. D01~D05 Lexer 상세는 [승인 기준](../ACCEPTED_LEXER.md)을 따른다. [전체 색인](../INDEX.md).

## Loan 의미
Read loan은 동시 공유 가능, change loan은 독점이다. loan은 마지막 실제 사용까지 유지하며 owner의 Drop/Move/변경과 충돌을 검사한다. disjoint를 증명하지 못하면 충돌로 본다.

## Place 관계
root가 다르면 disjoint; 같은 root의 서로 다른 struct field는 disjoint 증명 가능; 같은/dynamic array index는 보수적으로 overlap이다. dereference/shared backing storage는 alias 정보를 고려한다. projection prefix 관계는 overlap이다.

## Region 계약 초안 — D10
view의 사용 지점 집합과 owner storage-live 집합을 비교한다. read/change 재빌림은 원 loan을 suspend/제한하고 child 종료 뒤 복귀한다. view return은 source parameter에 연결되는 origin 계약이 필요하다.

## 검증
last-use 뒤 mutation pass, 동시에 live read/change fail, local view 반환 fail, 다른 field change pass, dynamic index split은 NOVA-118의 증명된 API만 허용한다.
