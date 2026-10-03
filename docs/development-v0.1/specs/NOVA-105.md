# NOVA-105 — foreign 선언 문법·공통 모델 사양서

| 항목 | 값 |
|---|---|
| 대상 | Nova 0.1 |
| 작성일 | 2026-10-03 |
| 문서 상태 | Draft — 기존 결정은 유지, 추가 상세는 승인 대기 |
| 적용 Stage | A~E |
| 근거 | [원본 NOVA-105](../../10_FFI/NOVA-105_foreign_선언_문법_공통_모델_사양서.md) |

이 문서는 원본을 대체하는 확정 사양이 아니다. 기존 확정 기준은 [CANONICAL](../CANONICAL.md)을 따르며, 새 의미·문법·API·정책은 [DECISIONS](../DECISIONS.md)의 승인이 필요하다. [전체 색인](../INDEX.md).

## 확정 foreign 예제
foreign c NativeMath { library "native_math"; @symbol("native_add") func add(first: int32, second: int32) -> int32 }의 핵심 철자는 원본을 따른다. 실제 선택 세미콜론/END는 grammar로 정규화한다.

## 선언 계약
foreign 함수는 body가 없다. ABI-safe type만 raw 선언에 사용하고 unsafe adapter → safe wrapper → application 계층으로 분리한다. owned handle에는 release 계약이 필요하다.

## 상세 초안 — D17
library와 @symbol은 contextual syntax다. 함수 Import/Export, pointer, opaque handle, callback+context를 지원 범위로 제안하며 C variadic/C++ exception/자동 bitfield mapping은 제외다.

## 검증
unknown adapter/library/symbol, foreign body 금지, ABI-unsafe string/Array 직접 전달 거부, callback context lifetime, safe wrapper ownership.
