# NOVA-093 — Backend 선택·LLVM 연동 결정서

| 항목 | 값 |
|---|---|
| 대상 | Nova 0.1 |
| 작성일 | 2026-10-03 |
| 문서 상태 | Draft — 기존 결정은 유지, 추가 상세는 승인 대기 |
| 적용 Stage | A~E |
| 근거 | [원본 NOVA-093](../../09_Backend_Runtime/NOVA-093_Backend_선택_LLVM_연동_결정서.md) |

이 문서는 원본을 대체하는 확정 사양이 아니다. 기존 확정 기준은 [CANONICAL](../CANONICAL.md)을 따르며, 새 의미·문법·API·정책은 [DECISIONS](../DECISIONS.md)의 승인이 필요하다. D01~D05 Lexer 상세는 [승인 기준](../ACCEPTED_LEXER.md)을 따른다. [전체 색인](../INDEX.md).

Stage A Native 최소 arithmetic/print/entry/internal ABI/toolchain 계약은 사용자 승인 [P03](../NATIVE_STAGE_A_PROPOSAL.md)를 따른다. [구현·지원·검증 범위](../NATIVE_IMPLEMENTATION.md). 전체 D07~D28과 미래 Stage 정책은 Draft다.

8종 고정 폭 정수·기대/peer literal 문맥·lossless 승격·checked runtime/const·MIR 변환·보간은 사용자 승인 [P07](../INTEGER_STAGE_B_PROPOSAL.md)를 따른다. [구현·검증 기록](../INTEGER_IMPLEMENTATION.md). P06 grammar를 재사용하고 cast/전체 D07은 Draft다.

char의 Unicode scalar 값·동일 타입 비교·선언/대입/함수/const·UTF-8 보간·private scalar ABI는 사용자 승인 [P08](../CHAR_STAGE_B_PROPOSAL.md)과 [CHAR primary EBNF](../GRAMMAR_STAGE_B_CHAR.ebnf)를 따른다. [구현·검증 기록](../CHAR_IMPLEMENTATION.md). D04 Lexer/escape/END는 유지하며 cast/char 산술·전체 D07은 Draft다.

binary32/64 literal·손실 없는 숫자 승격·IEEE 산술/비교·canonical NaN·const·최단 fixed decimal 보간·private ABI는 사용자 승인 [P09](../FLOAT_STAGE_B_PROPOSAL.md)와 [FLOAT primary EBNF](../GRAMMAR_STAGE_B_FLOAT.ebnf)를 따른다. [구현·검증 기록](../FLOAT_IMPLEMENTATION.md). float IEEE 결과는 const 실패가 아니며 INT checked 정책은 유지한다. source cast/float remainder/math API·전체 D07은 Draft다.

## Backend 결정
Rust compiler와 LLVM Adapter를 유지한다. nova-codegen의 CodegenBackend는 verified CodegenUnit/TargetSpec/Options를 받아 ObjectArtifact 또는 CodegenError를 반환한다.

## Pipeline
MIR concrete verify → LLVM Module → LLVM verify → optimize → object → linker. LLVM version/binding 선택은 D28에서 toolchain 잠금으로 정한다. frontend build는 LLVM 없이 가능해야 한다.

## Target 범위
원본 우선순위는 Windows x86_64 MSVC, Linux x86_64 GNU, Linux AArch64, Windows AArch64다. release 필수 첫 두 Target과 나머지 experimental 여부는 D28에서 제안한다. JIT/Wasm/GPU/direct machine code는 제외다.

## 검증
adapter isolation, LLVM verify failure→ICE, wrong toolchain→exit3, Hello object/link/run.
