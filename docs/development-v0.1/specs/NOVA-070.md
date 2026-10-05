# NOVA-070 — Source Manager·File ID·Span 사양서

| 항목 | 값 |
|---|---|
| 대상 | Nova 0.1 |
| 작성일 | 2026-10-03 |
| 문서 상태 | Draft — 기존 결정은 유지, 추가 상세는 승인 대기 |
| 적용 Stage | A~E |
| 근거 | [원본 NOVA-070](../../07_Compiler_Frontend/NOVA-070_Source_Manager_File_ID_Span_사양서.md) |

이 문서는 원본을 대체하는 확정 사양이 아니다. 기존 확정 기준은 [CANONICAL](../CANONICAL.md)을 따르며, 새 의미·문법·API·정책은 [DECISIONS](../DECISIONS.md)의 승인이 필요하다. D01~D05 Lexer 상세는 [승인 기준](../ACCEPTED_LEXER.md)을 따른다. [전체 색인](../INDEX.md).

root-relative 함수/전역 const item import·alias·internal/private/public·reachable graph·cross-file const·entry/source identity는 사용자 승인 [P11](../MODULE_STAGE_B_PROPOSAL.md)과 [전용 EBNF](../GRAMMAR_STAGE_B_MODULE.ebnf)를 따른다. [구현·검증 기록](../MODULE_IMPLEMENTATION.md). module alias/qualified value/reexport/Package와 전체 D06/D30은 후속이다.

## Compiler Source 모델
FileId는 database 내 stable integer, Span은 [start,end) byte range, LineIndex는 lazy다. immutable SourceFile과 append-only SourceDatabase를 첫 계약으로 제안한다. runtime Span<T>는 NOVA-118로 분리한다.

## API
add(path,text)/add_bytes(path,bytes) → FileId 또는 input error; file(id) → source; slice(span) → checked &str; location(offset) → 1-based line/scalar column. UTF-8 boundary, start≤end, end≤length, file 존재를 확인한다.

## 현재 코드와 사양
현재 세 crate는 이 기반을 구현했지만 Rust 실행 검증은 미완료다. file ID 안정성과 persistent cache fingerprint는 별개다. source revision을 지원할 때 기존 Span을 새 text에 자동 적용하지 않는다.

## 검증
Unicode/CRLF/CR/BOM/EOF/빈 파일/큰 파일/unknown FileId, lazy index 1회 계산, invalid span에서 panic 없음. mixed 문서 항목의 region/ABI는 Stage C로 추적한다.
