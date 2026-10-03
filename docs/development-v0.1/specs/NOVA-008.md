# NOVA-008 — 소스 인코딩·Unicode·줄바꿈 사양서

| 항목 | 값 |
|---|---|
| 대상 | Nova 0.1 |
| 작성일 | 2026-10-03 |
| 문서 상태 | Draft — 기존 결정은 유지, 추가 상세는 승인 대기 |
| 적용 Stage | A~B |
| 근거 | [원본 NOVA-008](../../01_Source_Syntax/NOVA-008_소스_인코딩_Unicode_줄바꿈_사양서.md) |

이 문서는 원본을 대체하는 확정 사양이 아니다. 기존 확정 기준은 [CANONICAL](../CANONICAL.md)을 따르며, 새 의미·문법·API·정책은 [DECISIONS](../DECISIONS.md)의 승인이 필요하다. [전체 색인](../INDEX.md).

## 소스 계약
UTF-8 파일과 반열린 바이트 위치를 사용한다. 잘못된 UTF-8은 교체 문자로 조용히 바꾸지 않고 입력 오류로 보고한다. 디스크 원문과 진단 byte offset은 동일해야 한다.

## 상세 초안 — D02
LF/CRLF/CR을 논리 NewLine으로 인식하되 원본 길이는 보존한다. 첫 UTF-8 BOM은 trivia로 허용하고 중간 BOM은 유효한 공백으로 취급하지 않는다. Identifier는 고정 Unicode 버전의 XID_Start/XID_Continue와 밑줄을 제안한다. NFC 변환은 하지 않고 철자 그대로 비교한다.

## 검증
한글/emoji/결합문자, CRLF 직후 Span, 빈 파일/마지막 newline/EOF, 잘못된 byte sequence를 다룬다. Compiler의 scalar 열과 LSP UTF-16 열을 별도 변환한다. Unicode 버전은 toolchain lock에 고정하며 D02 승인 전 lexer 식별자 정책을 확정하지 않는다.
