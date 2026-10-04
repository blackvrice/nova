# NOVA-035 — 함수·메서드·호출 규약·Receiver 사양서

| 항목 | 값 |
|---|---|
| 대상 | Nova 0.1 |
| 작성일 | 2026-10-03 |
| 문서 상태 | Draft — 기존 결정은 유지, 추가 상세는 승인 대기 |
| 적용 Stage | A~B |
| 근거 | [원본 NOVA-035](../../04_Functions_Control/NOVA-035_함수_메서드_호출_규약_Receiver_사양서.md) |

이 문서는 원본을 대체하는 확정 사양이 아니다. 기존 확정 기준은 [CANONICAL](../CANONICAL.md)을 따르며, 새 의미·문법·API·정책은 [DECISIONS](../DECISIONS.md)의 승인이 필요하다. D01~D05 Lexer 상세는 [승인 기준](../ACCEPTED_LEXER.md)을 따른다. [전체 색인](../INDEX.md).

Stage A Parser 구문/복구는 사용자 승인 [P01](../PARSER_STAGE_A_PROPOSAL.md)과 [전용 EBNF](../GRAMMAR_STAGE_A.ebnf)가 우선한다. 나머지 추가 상세는 Draft다.

Stage A 단일 파일 의미 검사는 사용자 승인 [P02](../SEMANTICS_STAGE_A_PROPOSAL.md)가 우선한다. runtime/전체 타입/미래 Stage의 추가 상세는 Draft다.

## Function signature
FunctionId, ReceiverMode, ParameterMode, ReturnType/ownership, Effects를 포함한다. modifier 부재는 Read, change는 exclusive loan, take는 이전이다. 인수는 소스 순서로 평가하고 return type만으로 overload하지 않는다.

## Receiver 초안 — D11
member function의 receiver를 func name(read/change/take self, ...)처럼 별도 read 키워드로 추가하지 않는다. 이 초안은 func name(self, ...), func name(change self, ...), func name(take self, ...)를 제안하며 self는 contextual binding이다.

## 반환
Unit 반환은 생략/void 정규화다. owned 값 반환은 owner를 caller로 이전한다. view 반환은 parameter-origin lifetime 계약이 필요하며 local owner를 가리키는 반환은 거부한다.

## 검증
mode mismatch, named argument 순서, early return cleanup, method mutation with read receiver fail, overload return-only conflict를 검사한다.
