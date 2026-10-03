# NOVA-086 — Drop Elaboration 구현서

| 항목 | 값 |
|---|---|
| 대상 | Nova 0.1 |
| 작성일 | 2026-10-03 |
| 문서 상태 | Draft — 기존 결정은 유지, 추가 상세는 승인 대기 |
| 적용 Stage | C |
| 근거 | [원본 NOVA-086](../../08_MIR_Middleend/NOVA-086_Drop_Elaboration_구현서.md) |

이 문서는 원본을 대체하는 확정 사양이 아니다. 기존 확정 기준은 [CANONICAL](../CANONICAL.md)을 따르며, 새 의미·문법·API·정책은 [DECISIONS](../DECISIONS.md)의 승인이 필요하다. D01~D05 Lexer 상세는 [승인 기준](../ACCEPTED_LEXER.md)을 따른다. [전체 색인](../INDEX.md).

## Drop elaboration
Move analysis의 initialized state를 읽어 Drop sites/flags를 삽입한다. 함수 lexical scopes와 edge exit scopes를 사용해 cleanup blocks를 공유하되 순서를 바꾸지 않는다.

## 계약
StorageLive/Dead와 논리 ownership를 구분한다. moved flag는 false, successful init은 true, flag-guarded Drop 뒤 false다. Enum은 active payload만, Class는 user body/fields/free 순서를 따른다.

## 검증
conditional return, loop break/continue, try Error path, var overwrite, field init failure, optimizer 뒤 exact once. panic Abort에는 fictitious cleanup edge를 넣지 않는다.
