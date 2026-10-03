# NOVA-080 — Compiler 내부 오류·ICE 처리 사양서

| 항목 | 값 |
|---|---|
| 대상 | Nova 0.1 |
| 작성일 | 2026-10-03 |
| 문서 상태 | Draft — 기존 결정은 유지, 추가 상세는 승인 대기 |
| 적용 Stage | A~E |
| 근거 | [원본 NOVA-080](../../07_Compiler_Frontend/NOVA-080_Compiler_내부_오류_ICE_처리_사양서.md) |

이 문서는 원본을 대체하는 확정 사양이 아니다. 기존 확정 기준은 [CANONICAL](../CANONICAL.md)을 따르며, 새 의미·문법·API·정책은 [DECISIONS](../DECISIONS.md)의 승인이 필요하다. D01~D05 Lexer 상세는 [승인 기준](../ACCEPTED_LEXER.md)을 따른다. [전체 색인](../INDEX.md).

## ICE 경계
잘못된 Nova source는 user diagnostic이며 panic 원인이 아니다. verified MIR invariant/LLVM verify 실패처럼 compiler 내부 불변 조건이 깨지면 ICE다. system I/O/toolchain 실패와도 분리한다.

## 보고 계약
compiler version/build ID, Target, command options, query stack, source locations, reproduction dump 경로를 포함한다. CLI exit=101을 따른다. source/환경 비밀값 업로드는 자동 수행하지 않는다.

## 검증
fault-injected invariant fail은 N9xxx와 exit101, user malformed source는 N1xxx/exit1, missing linker는 exit3. 모든 재현 가능한 ICE는 최소 corpus regression을 추가한다.
