# NOVA-128 — Build Profile·Target 사양서

| 항목 | 값 |
|---|---|
| 대상 | Nova 0.1 |
| 작성일 | 2026-10-03 |
| 문서 상태 | Draft — 기존 결정은 유지, 추가 상세는 승인 대기 |
| 적용 Stage | E |
| 근거 | [원본 NOVA-128](../../12_Tooling_Packaging/NOVA-128_Build_Profile_Target_사양서.md) |

이 문서는 원본을 대체하는 확정 사양이 아니다. 기존 확정 기준은 [CANONICAL](../CANONICAL.md)을 따르며, 새 의미·문법·API·정책은 [DECISIONS](../DECISIONS.md)의 승인이 필요하다. D01~D05 Lexer 상세는 [승인 기준](../ACCEPTED_LEXER.md)을 따른다. [전체 색인](../INDEX.md).

## Build profile 초안 — D26/D28
debug/release/size는 optimization/debug info/LTO/codegen units 설정을 가진다. profile마다 overflow/bounds 의미를 바꾸지 않는다. Target triple/sysroot/linker/runtime ABI는 별도 toolchain input이다.

## 기본 제안
debug O0+debug info, release O2, size Os; fast-math off, LTO는 opt-in. cross compile은 target std/runtime/linker가 모두 있어야 하며 host executable run을 자동 시도하지 않는다.

## 검증
모든 profile 동일 observable semantics, unsupported target exit3, missing sysroot, --target output directory 분리, options cache key 일치.
