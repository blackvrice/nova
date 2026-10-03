# NOVA-107 — C Header Parser·Binding Generator 사양서

| 항목 | 값 |
|---|---|
| 대상 | Nova 0.1 |
| 작성일 | 2026-10-03 |
| 문서 상태 | Draft — 기존 결정은 유지, 추가 상세는 승인 대기 |
| 적용 Stage | A~E |
| 근거 | [원본 NOVA-107](../../10_FFI/NOVA-107_C_Header_Parser_Binding_Generator_사양서.md) |

이 문서는 원본을 대체하는 확정 사양이 아니다. 기존 확정 기준은 [CANONICAL](../CANONICAL.md)을 따르며, 새 의미·문법·API·정책은 [DECISIONS](../DECISIONS.md)의 승인이 필요하다. [전체 색인](../INDEX.md).

## Binding generator 범위 초안 — D17
검증된 C frontend의 AST/ABI 정보에서 Nova foreign 선언을 생성한다. header text를 정규식으로 추측하지 않는다. include paths/macros/Target/compiler flags를 generation input으로 기록한다.

## 지원/거부
fixed-width scalar, pointer, opaque record, fixed signature function은 후보. variadic/bitfield/C++ template/exception-dependent function은 자동 safe binding을 생성하지 않는다. pointer ownership/length는 header만으로 추정할 수 없으므로 annotation manifest가 필요하다.

## 검증
reproducible output, typedef resolution, Target-dependent width, macro-conditioned signature, unsupported construct explicit report. generated raw binding을 safe wrapper 완료로 표시하지 않는다.
