# NOVA-007 — Nova 버전 정책·호환성 원칙

| 항목 | 값 |
|---|---|
| 대상 | Nova 0.1 |
| 작성일 | 2026-10-03 |
| 문서 상태 | Draft — 기존 결정은 유지, 추가 상세는 승인 대기 |
| 적용 Stage | 전 Stage |
| 근거 | [원본 NOVA-007](../../00_Governance/NOVA-007_Nova_버전_정책_호환성_원칙.md) |

이 문서는 원본을 대체하는 확정 사양이 아니다. 기존 확정 기준은 [CANONICAL](../CANONICAL.md)을 따르며, 새 의미·문법·API·정책은 [DECISIONS](../DECISIONS.md)의 승인이 필요하다. D01~D05 Lexer 상세는 [승인 기준](../ACCEPTED_LEXER.md)을 따른다. [전체 색인](../INDEX.md).

## 버전 경계
언어 버전, compiler 버전, Runtime ABI 버전, Package schema, Artifact schema, Cache schema를 독립 필드로 관리한다. 언어 버전 0.1이라는 이유로 서로 다른 compiler artifact를 재사용하지 않는다.

## 호환성 초안 — D29
0.1.x에서 기존 정상 프로그램의 타입/실행 의미를 바꾸면 breaking으로 취급한다. 명백한 버그 수정은 반례와 migration note를 남긴다. 진단 코드 삭제/재사용, 공용 std signature 변경, lock schema 변경도 호환성 검토 대상이다.

## 검증
기준 버전 corpus를 새 compiler로 check/run하고 AST 문자열보다 정상/거부 판정과 stdout/exit/Drop trace를 비교한다. ABI 호환성이 보장되지 않으면 runtime과 패키지를 다시 컴파일한다.
