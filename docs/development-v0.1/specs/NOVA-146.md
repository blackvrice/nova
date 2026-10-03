# NOVA-146 — 0.1 Release Checklist

| 항목 | 값 |
|---|---|
| 대상 | Nova 0.1 |
| 작성일 | 2026-10-03 |
| 문서 상태 | Draft — 기존 결정은 유지, 추가 상세는 승인 대기 |
| 적용 Stage | 전 Stage |
| 근거 | [원본 NOVA-146](../../13_Testing_Release/NOVA-146_0.1_Release_Checklist.md) |

이 문서는 원본을 대체하는 확정 사양이 아니다. 기존 확정 기준은 [CANONICAL](../CANONICAL.md)을 따르며, 새 의미·문법·API·정책은 [DECISIONS](../DECISIONS.md)의 승인이 필요하다. D01~D05 Lexer 상세는 [승인 기준](../ACCEPTED_LEXER.md)을 따른다. [전체 색인](../INDEX.md).

## 0.1 Release gate
Accepted language/grammar decisions, Stage A~E 지원표, pass/fail/runtime regression, 첫 Windows/Linux Target, toolchain locks, ABI/schema versions, clean-machine build/install/uninstall, checksum과 release note가 필요하다.

## blocking
unresolved semantic D번호, known safe-code unsoundness, wrong-code, missing entry/runtime/link behavior, reproducibility failure는 release blocker다. 제외 feature는 미구현으로 기록하며 완료 부족으로 착각하지 않는다.

## 검증
release artifact clean VM 테스트, offline/locked package, debug/release semantics, documentation examples, minimum Rust toolchain frontend checks. 현재 3-crate 기반은 release 완료가 아니다.
