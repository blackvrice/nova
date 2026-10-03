# NOVA-022 — Symbol·Definition ID 사양서

| 항목 | 값 |
|---|---|
| 대상 | Nova 0.1 |
| 작성일 | 2026-10-03 |
| 문서 상태 | Draft — 기존 결정은 유지, 추가 상세는 승인 대기 |
| 적용 Stage | B~E |
| 근거 | [원본 NOVA-022](../../02_Names_Modules/NOVA-022_Symbol_Definition_ID_사양서.md) |

이 문서는 원본을 대체하는 확정 사양이 아니다. 기존 확정 기준은 [CANONICAL](../CANONICAL.md)을 따르며, 새 의미·문법·API·정책은 [DECISIONS](../DECISIONS.md)의 승인이 필요하다. D01~D05 Lexer 상세는 [승인 기준](../ACCEPTED_LEXER.md)을 따른다. [전체 색인](../INDEX.md).

## ID 계층
FileId는 source database identity, SymbolId는 session 문자열 intern, ModuleId/DefId는 선언 identity, LocalId는 body 내 binding identity다. 서로의 숫자가 같아도 혼용하지 않도록 Rust newtype을 사용한다.

## 안정성 계약
session integer ID와 영구 cache key를 구분한다. serialized key는 package identity+module path+item path+disambiguator+signature fingerprint 초안을 사용한다. OS 파일 검색 순서나 memory address를 key에 넣지 않는다.

## SourceInfo
모든 선언은 Span, owner와 source origin을 가진다. synthetic definition에는 생성 원인 Span을 유지한다. ID 재배정은 dump 출력의 deterministic ordering을 해치지 않아야 한다.

## 검증
파일 나열 순서 변경, 동일 이름 nested scope, overloaded function, 재빌드 key 비교. duplicate DefId 발생은 내부 검증 오류다.
