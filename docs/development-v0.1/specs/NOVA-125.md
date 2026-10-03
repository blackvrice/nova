# NOVA-125 — Compiler CLI 명령·옵션 사양서

| 항목 | 값 |
|---|---|
| 대상 | Nova 0.1 |
| 작성일 | 2026-10-03 |
| 문서 상태 | Draft — 기존 결정은 유지, 추가 상세는 승인 대기 |
| 적용 Stage | A~E |
| 근거 | [원본 NOVA-125](../../12_Tooling_Packaging/NOVA-125_Compiler_CLI_명령_옵션_사양서.md) |

이 문서는 원본을 대체하는 확정 사양이 아니다. 기존 확정 기준은 [CANONICAL](../CANONICAL.md)을 따르며, 새 의미·문법·API·정책은 [DECISIONS](../DECISIONS.md)의 승인이 필요하다. [전체 색인](../INDEX.md).

## CLI 확정 표
check/build/run/test/fmt/clean/doc/version; common --manifest-path/--target/--profile/--color/--message-format/--jobs/-v; emit=tokens/ast/hir/typed-hir/mir/llvm/obj/asm를 따른다.

## exit
0 성공, 1 user source 오류, 2 CLI/Manifest, 3 Toolchain/Linker, 101 ICE. manifest 없는 .nova는 임시 package다. run은 성공 build 후 child를 실행하며 child exit 전달과 compiler error exit 구분은 D22 초안이다.

## 검증
unknown option exit2, invalid source exit1, missing clang/linker exit3, ICE101, JSON stdout/diagnostic stderr 혼합 방지, no build on check. CLI_CONTRACTS에서 side effects/output paths를 정의한다.
