# NOVA-125 — Compiler CLI 명령·옵션 사양서

| 항목 | 값 |
|---|---|
| 대상 | Nova 0.1 |
| 작성일 | 2026-10-03 |
| 문서 상태 | Draft — 기존 결정은 유지, 추가 상세는 승인 대기 |
| 적용 Stage | A~E |
| 근거 | [원본 NOVA-125](../../12_Tooling_Packaging/NOVA-125_Compiler_CLI_명령_옵션_사양서.md) |

이 문서는 원본을 대체하는 확정 사양이 아니다. 기존 확정 기준은 [CANONICAL](../CANONICAL.md)을 따르며, 새 의미·문법·API·정책은 [DECISIONS](../DECISIONS.md)의 승인이 필요하다. D01~D05 Lexer 상세는 [승인 기준](../ACCEPTED_LEXER.md)을 따른다. [전체 색인](../INDEX.md).

Stage A Native 최소 arithmetic/print/entry/internal ABI/toolchain 계약은 사용자 승인 [P03](../NATIVE_STAGE_A_PROPOSAL.md)를 따른다. [구현·지원·검증 범위](../NATIVE_IMPLEMENTATION.md). 전체 D07~D28과 미래 Stage 정책은 Draft다.

8종 고정 폭 정수·기대/peer literal 문맥·lossless 승격·checked runtime/const·MIR 변환·보간은 사용자 승인 [P07](../INTEGER_STAGE_B_PROPOSAL.md)를 따른다. [구현·검증 기록](../INTEGER_IMPLEMENTATION.md). P06 grammar를 재사용하고 float/char/cast/전체 D07은 Draft다.

## CLI 확정 표
check/build/run/test/fmt/clean/doc/version; common --manifest-path/--target/--profile/--color/--message-format/--jobs/-v; emit=tokens/ast/hir/typed-hir/mir/llvm/obj/asm를 따른다.

## exit
0 성공, 1 user source 오류, 2 CLI/Manifest, 3 Toolchain/Linker, 101 ICE. manifest 없는 .nova는 임시 package다. run은 성공 build 후 child를 실행하며 child exit 전달과 compiler error exit 구분은 D22 초안이다.

## 검증
unknown option exit2, invalid source exit1, missing clang/linker exit3, ICE101, JSON stdout/diagnostic stderr 혼합 방지, no build on check. CLI_CONTRACTS에서 side effects/output paths를 정의한다.
