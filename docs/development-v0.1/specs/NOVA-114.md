# NOVA-114 — Core Prelude·기본 Symbol 사양서

| 항목 | 값 |
|---|---|
| 대상 | Nova 0.1 |
| 작성일 | 2026-10-03 |
| 문서 상태 | Draft — 기존 결정은 유지, 추가 상세는 승인 대기 |
| 적용 Stage | B~E (print: A) |
| 근거 | [원본 NOVA-114](../../11_Standard_Library/NOVA-114_Core_Prelude_기본_Symbol_사양서.md) |

이 문서는 원본을 대체하는 확정 사양이 아니다. 기존 확정 기준은 [CANONICAL](../CANONICAL.md)을 따르며, 새 의미·문법·API·정책은 [DECISIONS](../DECISIONS.md)의 승인이 필요하다. D01~D05 Lexer 상세는 [승인 기준](../ACCEPTED_LEXER.md)을 따른다. [전체 색인](../INDEX.md).

Stage A 단일 파일 의미 검사는 사용자 승인 [P02](../SEMANTICS_STAGE_A_PROPOSAL.md)가 우선한다. runtime/전체 타입/미래 Stage의 추가 상세는 Draft다.

Stage A Native 최소 arithmetic/print/entry/internal ABI/toolchain 계약은 사용자 승인 [P03](../NATIVE_STAGE_A_PROPOSAL.md)를 따른다. [구현·지원·검증 범위](../NATIVE_IMPLEMENTATION.md). 전체 D07~D28과 미래 Stage 정책은 Draft다.

8종 고정 폭 정수·기대/peer literal 문맥·lossless 승격·checked runtime/const·MIR 변환·보간은 사용자 승인 [P07](../INTEGER_STAGE_B_PROPOSAL.md)를 따른다. [구현·검증 기록](../INTEGER_IMPLEMENTATION.md). P06 grammar를 재사용하고 float/char/cast/전체 D07은 Draft다.

## Prelude 초안 — D23
최소 Prelude는 Primitive/Unit, Option(Some/None), Result(Success/Error), Array, panic, Stage A print를 제안한다. List/Map/file/thread는 explicit module import를 요구한다.

## 이름 계약
Prelude symbol도 DefId를 가지며 user local shadow 정책은 이름 해석 D06을 따른다. print를 Parser special case가 아니라 builtin definition/runtime intrinsic으로 연결한다. private runtime symbol을 source name으로 노출하지 않는다.

## 검증
import 없는 Hello Nova, shadowed print 처리, std 없는 core compilation mode, duplicate prelude definition, qualified std 접근. module/version별 API 목록은 STDLIB_API와 일치한다.
