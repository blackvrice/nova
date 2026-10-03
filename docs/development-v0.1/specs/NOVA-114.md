# NOVA-114 — Core Prelude·기본 Symbol 사양서

| 항목 | 값 |
|---|---|
| 대상 | Nova 0.1 |
| 작성일 | 2026-10-03 |
| 문서 상태 | Draft — 기존 결정은 유지, 추가 상세는 승인 대기 |
| 적용 Stage | B~E (print: A) |
| 근거 | [원본 NOVA-114](../../11_Standard_Library/NOVA-114_Core_Prelude_기본_Symbol_사양서.md) |

이 문서는 원본을 대체하는 확정 사양이 아니다. 기존 확정 기준은 [CANONICAL](../CANONICAL.md)을 따르며, 새 의미·문법·API·정책은 [DECISIONS](../DECISIONS.md)의 승인이 필요하다. [전체 색인](../INDEX.md).

## Prelude 초안 — D23
최소 Prelude는 Primitive/Unit, Option(Some/None), Result(Success/Error), Array, panic, Stage A print를 제안한다. List/Map/file/thread는 explicit module import를 요구한다.

## 이름 계약
Prelude symbol도 DefId를 가지며 user local shadow 정책은 이름 해석 D06을 따른다. print를 Parser special case가 아니라 builtin definition/runtime intrinsic으로 연결한다. private runtime symbol을 source name으로 노출하지 않는다.

## 검증
import 없는 Hello Nova, shadowed print 처리, std 없는 core compilation mode, duplicate prelude definition, qualified std 접근. module/version별 API 목록은 STDLIB_API와 일치한다.
