# NOVA-122 — 파일·Stream·Console I/O 사양서

| 항목 | 값 |
|---|---|
| 대상 | Nova 0.1 |
| 작성일 | 2026-10-03 |
| 문서 상태 | Draft — 기존 결정은 유지, 추가 상세는 승인 대기 |
| 적용 Stage | B~E (print: A) |
| 근거 | [원본 NOVA-122](../../11_Standard_Library/NOVA-122_파일_Stream_Console_I_O_사양서.md) |

이 문서는 원본을 대체하는 확정 사양이 아니다. 기존 확정 기준은 [CANONICAL](../CANONICAL.md)을 따르며, 새 의미·문법·API·정책은 [DECISIONS](../DECISIONS.md)의 승인이 필요하다. D01~D05 Lexer 상세는 [승인 기준](../ACCEPTED_LEXER.md)을 따른다. [전체 색인](../INDEX.md).

## I/O API 초안 — D23
print(text:string)→Unit는 Stage A UTF-8 stdout line 출력 후보다. println 별도 여부와 newline behavior는 D23 승인 대상이다. 파일/stream open/read/write/flush는 Result와 structured IoError를 반환한다.

## Resource 계약
File/Stream은 Move owner이며 drop은 close, close error를 destructor에서 반환할 수 없으므로 explicit close()→Result로 제공한다. partial read/write byte 수, EOF와 error를 구분한다. path encoding은 platform abstraction을 따른다.

## 검증
Hello exact stdout, embedded NUL/Unicode, partial write, permission denied, close double-use fail, using cleanup, broken pipe policy. print failure를 무조건 성공으로 숨기지 않는다.
