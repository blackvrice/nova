# NOVA-079 — Compiler Query·의존성·Cache 사양서

| 항목 | 값 |
|---|---|
| 대상 | Nova 0.1 |
| 작성일 | 2026-10-03 |
| 문서 상태 | Draft — 기존 결정은 유지, 추가 상세는 승인 대기 |
| 적용 Stage | A~E |
| 근거 | [원본 NOVA-079](../../07_Compiler_Frontend/NOVA-079_Compiler_Query_의존성_Cache_사양서.md) |

이 문서는 원본을 대체하는 확정 사양이 아니다. 기존 확정 기준은 [CANONICAL](../CANONICAL.md)을 따르며, 새 의미·문법·API·정책은 [DECISIONS](../DECISIONS.md)의 승인이 필요하다. D01~D05 Lexer 상세는 [승인 기준](../ACCEPTED_LEXER.md)을 따른다. [전체 색인](../INDEX.md).

## Query 계약
key/value/dependency fingerprint/diagnostics를 분리한다. query identity는 compiler/schema/options/Target과 관련 input을 포함한다. LLVM backend 결과만 Target key가 필요하다고 가정하지 않는다; layout/const도 Target 의존이다.

## 무효화
source text, imported export signature, generic body, runtime ABI, options가 바뀌면 해당 dependency path 결과를 무효화한다. query cycle은 user semantic cycle과 compiler dependency bug를 구분한다.

## 단계적 도입
Stage A는 pure function 호출과 session memoization으로 충분하다. disk cache는 E에서 checksum, atomic write, trust boundary를 검증하고 도입한다.

## 검증
unchanged reuse, body-only change 영향, public signature change, corrupted disk entry miss/rebuild, serial/parallel 동일 diagnostics.
