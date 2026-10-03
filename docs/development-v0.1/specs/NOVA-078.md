# NOVA-078 — 진단 시스템·Error Code 관리 사양서

| 항목 | 값 |
|---|---|
| 대상 | Nova 0.1 |
| 작성일 | 2026-10-03 |
| 문서 상태 | Draft — 기존 결정은 유지, 추가 상세는 승인 대기 |
| 적용 Stage | A~E |
| 근거 | [원본 NOVA-078](../../07_Compiler_Frontend/NOVA-078_진단_시스템_Error_Code_관리_사양서.md) |

이 문서는 원본을 대체하는 확정 사양이 아니다. 기존 확정 기준은 [CANONICAL](../CANONICAL.md)을 따르며, 새 의미·문법·API·정책은 [DECISIONS](../DECISIONS.md)의 승인이 필요하다. D01~D05 Lexer 상세는 [승인 기준](../ACCEPTED_LEXER.md)을 따른다. [전체 색인](../INDEX.md).

## Diagnostic 구조
code/severity/message/primary/secondary/notes/suggestions를 보존한다. N1xxx syntax, N2xxx name/type, N3xxx control/const, N4xxx ownership, N5xxx ABI/runtime, N8xxx tooling/package, N9xxx ICE다.

## Renderer
Plain/ANSI/JSON/Snapshot을 제공한다. byte Span과 Unicode scalar 표시 열은 다르다. 다중 파일/다중 줄/EOF label, note/help, suggestion applicability를 잃지 않는다. JSON schema는 DIAGNOSTIC_SCHEMA에 제안한다.

## 코드 운영
DIAGNOSTICS.csv의 각 코드에는 category, trigger, primary/secondary 의미, Stage를 기록한다. proposal을 승인 전 안정 API라고 부르지 않는다. code는 폐기 후 재사용하지 않는 D25 제안이다.

## 검증
renderer metadata 보존, control character escaping, invalid internal Span은 ICE 또는 명시 error, 연쇄 오류 suppression, sorting 결정성.
