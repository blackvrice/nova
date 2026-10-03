# NOVA-148 — Self-hosting 계획서

| 항목 | 값 |
|---|---|
| 대상 | Nova 0.1 |
| 작성일 | 2026-10-03 |
| 문서 상태 | Draft — 기존 결정은 유지, 추가 상세는 승인 대기 |
| 적용 Stage | 제외/후속 |
| 근거 | [원본 NOVA-148](../../13_Testing_Release/NOVA-148_Self-hosting_계획서.md) |

이 문서는 원본을 대체하는 확정 사양이 아니다. 기존 확정 기준은 [CANONICAL](../CANONICAL.md)을 따르며, 새 의미·문법·API·정책은 [DECISIONS](../DECISIONS.md)의 승인이 필요하다. D01~D05 Lexer 상세는 [승인 기준](../ACCEPTED_LEXER.md)을 따른다. [전체 색인](../INDEX.md).

## Self-hosting 제외
0.1 compiler 구현은 Rust이며 Self-hosting은 비목표다. 개발 문서 전체를 작성했다는 이유로 Nova compiler rewrite를 시작하지 않는다.

## 후속 연구
Rust Stage0 안정화 → Nova 도구 일부 → frontend → whole compiler 순서를 검토한다. Stage1/Stage2 생성 compiler 의미, runtime ABI, bootstrap trust, reproducibility를 비교해야 한다.

## 검증 조건
전체 conformance, same-language compiler self-build, Stage1/2 observable equivalence, fallback Stage0 지원. 일정과 언어 버전은 별도 proposal에서 결정한다.
