# NOVA-098 — Object File·Linker 연동 사양서

| 항목 | 값 |
|---|---|
| 대상 | Nova 0.1 |
| 작성일 | 2026-10-03 |
| 문서 상태 | Draft — 기존 결정은 유지, 추가 상세는 승인 대기 |
| 적용 Stage | A~E |
| 근거 | [원본 NOVA-098](../../09_Backend_Runtime/NOVA-098_Object_File_Linker_연동_사양서.md) |

이 문서는 원본을 대체하는 확정 사양이 아니다. 기존 확정 기준은 [CANONICAL](../CANONICAL.md)을 따르며, 새 의미·문법·API·정책은 [DECISIONS](../DECISIONS.md)의 승인이 필요하다. D01~D05 Lexer 상세는 [승인 기준](../ACCEPTED_LEXER.md)을 따른다. [전체 색인](../INDEX.md).

## Object/Link 계약
ObjectArtifact는 path,Target,format,compiler/runtime ABI,checksum을 가진다. COFF/ELF는 Target에 맞추며 output/temp path는 workspace output 아래로 제한한다.

## Linker invocation
argument list API를 사용하고 source/package 값을 shell command로 연결하지 않는다. required runtime/system libs, search paths, entry/export를 명시한다. toolchain discovery 실패, missing symbol, unsupported Target은 사용자 코드 오류와 분리한다.

## 검증
공백/한글 경로, object format mismatch, missing library/symbol, response-file 큰 command, temporary cleanup, partial output를 정상 executable로 오인하지 않음을 확인한다.
