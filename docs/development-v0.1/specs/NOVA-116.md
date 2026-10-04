# NOVA-116 — string·UTF-8 API 사양서

| 항목 | 값 |
|---|---|
| 대상 | Nova 0.1 |
| 작성일 | 2026-10-03 |
| 문서 상태 | Draft — 기존 결정은 유지, 추가 상세는 승인 대기 |
| 적용 Stage | B~E (print: A) |
| 근거 | [원본 NOVA-116](../../11_Standard_Library/NOVA-116_string_UTF-8_API_사양서.md) |

이 문서는 원본을 대체하는 확정 사양이 아니다. 기존 확정 기준은 [CANONICAL](../CANONICAL.md)을 따르며, 새 의미·문법·API·정책은 [DECISIONS](../DECISIONS.md)의 승인이 필요하다. D01~D05 Lexer 상세는 [승인 기준](../ACCEPTED_LEXER.md)을 따른다. [전체 색인](../INDEX.md).

Stage A 단일 파일 의미 검사는 사용자 승인 [P02](../SEMANTICS_STAGE_A_PROPOSAL.md)가 우선한다. runtime/전체 타입/미래 Stage의 추가 상세는 Draft다.

## string 확정 의미
owned UTF-8이며 기본 index 연산은 없다. byte/scalar/grapheme API를 구분한다. UTF-8 검증 없는 bytes를 string으로 조용히 재해석하지 않는다.

## API 초안 — D23
byteLength, isEmpty, bytes(ReadOnlySpan<byte>), scalarAt(index)→Option<char>, sliceBytes(start,end)→Result<string,BoundaryError>, fromUtf8(Array<byte>)→Result<string,Utf8Error>를 제안한다. slice는 byte boundary 확인 후 owned copy이며 zero-copy view API는 별도다.

## 검증
emoji/결합문자 scalar와 grapheme 차이, boundary 중간 slice fail, invalid UTF-8, bytes view 후 owner move fail, copy allocation/Drop. grapheme segmentation은 Unicode 버전 고정 후 std 확장으로 검토한다.
