# NOVA-139 — MIR Snapshot 규격

| 항목 | 값 |
|---|---|
| 대상 | Nova 0.1 |
| 작성일 | 2026-10-03 |
| 문서 상태 | Draft — 기존 결정은 유지, 추가 상세는 승인 대기 |
| 적용 Stage | 전 Stage |
| 근거 | [원본 NOVA-139](../../13_Testing_Release/NOVA-139_MIR_Snapshot_규격.md) |

이 문서는 원본을 대체하는 확정 사양이 아니다. 기존 확정 기준은 [CANONICAL](../CANONICAL.md)을 따르며, 새 의미·문법·API·정책은 [DECISIONS](../DECISIONS.md)의 승인이 필요하다. D01~D05 Lexer 상세는 [승인 기준](../ACCEPTED_LEXER.md)을 따른다. [전체 색인](../INDEX.md).

## MIR snapshot
lowered/pre-analysis/post-drop/post-opt dump를 구분한다. fixture sidecar는 pass name/schema/Target을 고정한다. typed locals, CFG, Copy/Move operands, Call/Drop terminators, SourceInfo를 기록한다.

## 검증
validator를 snapshot 전후 실행하고 malformed fixture는 internal code를 기대한다. block numbering normalize가 edge topology를 바꾸지 않아야 한다. constructor partial init와 try cleanup을 필수 corpus로 둔다.

## 완료
snapshot 검토만으로 runtime correctness를 주장하지 않고 Drop trace/runtime fixture와 함께 확인한다.
