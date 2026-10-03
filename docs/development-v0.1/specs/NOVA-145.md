# NOVA-145 — Unsafe·FFI 보안 검토 기준서

| 항목 | 값 |
|---|---|
| 대상 | Nova 0.1 |
| 작성일 | 2026-10-03 |
| 문서 상태 | Draft — 기존 결정은 유지, 추가 상세는 승인 대기 |
| 적용 Stage | 전 Stage |
| 근거 | [원본 NOVA-145](../../13_Testing_Release/NOVA-145_Unsafe_FFI_보안_검토_기준서.md) |

이 문서는 원본을 대체하는 확정 사양이 아니다. 기존 확정 기준은 [CANONICAL](../CANONICAL.md)을 따르며, 새 의미·문법·API·정책은 [DECISIONS](../DECISIONS.md)의 승인이 필요하다. [전체 색인](../INDEX.md).

## Unsafe/FFI 검토표
operation마다 provenance, null, align, initialized bytes, bounds, alias/exclusivity, owner duration, release function, thread/callback/reentrancy, exception boundary를 기록한다.

## compiler/tooling 입력
manifest/archive/cache path traversal, untrusted metadata length/depth, shell argument injection, imported doc script, resource exhaustion도 검토한다. package 작업에 source-controlled shell hook을 자동 실행하지 않는다.

## 검증
negative wrapper corpus, malformed artifact/cache, foreign allocator mismatch, callback-after-free, resource budget, link path spaces. safe wrapper 완료에는 계약과 runtime failure tests가 모두 필요하다.
