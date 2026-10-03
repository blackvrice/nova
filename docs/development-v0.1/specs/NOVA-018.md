# NOVA-018 — 소스 포매팅 기준서

| 항목 | 값 |
|---|---|
| 대상 | Nova 0.1 |
| 작성일 | 2026-10-03 |
| 문서 상태 | Draft — 기존 결정은 유지, 추가 상세는 승인 대기 |
| 적용 Stage | E |
| 근거 | [원본 NOVA-018](../../01_Source_Syntax/NOVA-018_소스_포매팅_기준서.md) |

이 문서는 원본을 대체하는 확정 사양이 아니다. 기존 확정 기준은 [CANONICAL](../CANONICAL.md)을 따르며, 새 의미·문법·API·정책은 [DECISIONS](../DECISIONS.md)의 승인이 필요하다. D01~D05 Lexer 상세는 [승인 기준](../ACCEPTED_LEXER.md)을 따른다. [전체 색인](../INDEX.md).

## Source style
NOVA-131의 줄 길이 100과 주석 보존을 따른다. 4 spaces indentation, UTF-8/LF 출력, 최종 newline, 선택 세미콜론 생략을 초안 D24로 제안한다. 문장 경계를 바꾸는 줄 나눔은 허용하지 않는다.

## 배치 규칙
짧은 call은 한 줄, 긴 인수 목록은 각 한 줄과 trailing comma다. {는 header와 같은 줄, } else {를 연결한다. 연산자 continuation은 operator가 보이는 위치에 배치한다. string/comment 원문 내부는 재작성하지 않는다.

## 검증
parse(format(source))가 동일한 semantic AST/HIR를 만들고 format(format(source))가 byte 동일해야 한다. return newline 사례는 값을 연결하거나 끊지 않는다. 오류 파일을 부분 수정하는 정책은 D24에서 기본 거부를 제안한다.
