# NOVA-023 — Package 간 이름 해석 사양서

| 항목 | 값 |
|---|---|
| 대상 | Nova 0.1 |
| 작성일 | 2026-10-03 |
| 문서 상태 | Draft — 기존 결정은 유지, 추가 상세는 승인 대기 |
| 적용 Stage | B~E |
| 근거 | [원본 NOVA-023](../../02_Names_Modules/NOVA-023_Package_간_이름_해석_사양서.md) |

이 문서는 원본을 대체하는 확정 사양이 아니다. 기존 확정 기준은 [CANONICAL](../CANONICAL.md)을 따르며, 새 의미·문법·API·정책은 [DECISIONS](../DECISIONS.md)의 승인이 필요하다. D01~D05 Lexer 상세는 [승인 기준](../ACCEPTED_LEXER.md)을 따른다. [전체 색인](../INDEX.md).

## Package 경계
package identity에는 이름만이 아니라 source와 정확한 version/revision을 포함한다. 동일 이름 다른 source package를 하나로 합치지 않는다. 외부 package 참조는 NOVA-126/127의 resolved graph를 입력으로 받는다.

## Import 계약 초안 — D06/D21
dependency alias를 root segment로 사용한다. export metadata만 접근 가능하고 private/internal을 외부에서 해석하지 않는다. 동일 alias의 복수 dependency는 manifest 단계에서 오류다.

## 구현
compiler resolver는 network를 직접 조회하지 않는다. package 계층이 검증한 source/export artifact를 넘긴다. metadata compiler/ABI/schema mismatch는 source rebuild 또는 명시 오류다.

## 검증
version 다중 공존, alias collision, private symbol 거부, lock된 revision 교체 탐지, import 순서 독립성을 확인한다.
