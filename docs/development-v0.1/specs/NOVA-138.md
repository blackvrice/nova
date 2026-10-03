# NOVA-138 — Compile-pass·Compile-fail 규격

| 항목 | 값 |
|---|---|
| 대상 | Nova 0.1 |
| 작성일 | 2026-10-03 |
| 문서 상태 | Draft — 기존 결정은 유지, 추가 상세는 승인 대기 |
| 적용 Stage | 전 Stage |
| 근거 | [원본 NOVA-138](../../13_Testing_Release/NOVA-138_Compile-pass_Compile-fail_규격.md) |

이 문서는 원본을 대체하는 확정 사양이 아니다. 기존 확정 기준은 [CANONICAL](../CANONICAL.md)을 따르며, 새 의미·문법·API·정책은 [DECISIONS](../DECISIONS.md)의 승인이 필요하다. D01~D05 Lexer 상세는 [승인 기준](../ACCEPTED_LEXER.md)을 따른다. [전체 색인](../INDEX.md).

## Compile fixture 형식 초안 — D25
각 fixture는 .nova와 sidecar .json(expected phase,code,file,start,end,secondary,Target,Stage)를 제안한다. 원본 //~^ N4101 annotation도 읽되 byte range sidecar가 authoritative다.

## 판정
pass는 check exit0/no error; fail은 정확한 진단 및 exit1. toolchain failure exit3은 fixture 성공이 아니다. negative fixture가 parser 오류로 막혀 원하는 type/move 오류를 검증하지 못하면 실패다.

## 검증
unicode byte ranges, cross-file secondary, unexpected extra error, unsupported Stage feature와 0.1 non-goal 구별, empty file/no-main phase 구별.
