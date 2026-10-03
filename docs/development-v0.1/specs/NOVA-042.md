# NOVA-042 — Native Calling Convention 사양서

| 항목 | 값 |
|---|---|
| 대상 | Nova 0.1 |
| 작성일 | 2026-10-03 |
| 문서 상태 | Draft — 기존 결정은 유지, 추가 상세는 승인 대기 |
| 적용 Stage | A~B |
| 근거 | [원본 NOVA-042](../../04_Functions_Control/NOVA-042_Native_Calling_Convention_사양서.md) |

이 문서는 원본을 대체하는 확정 사양이 아니다. 기존 확정 기준은 [CANONICAL](../CANONICAL.md)을 따르며, 새 의미·문법·API·정책은 [DECISIONS](../DECISIONS.md)의 승인이 필요하다. [전체 색인](../INDEX.md).

## Native ABI
Nova 내부 호출과 C foreign ABI를 분리한다. semantic mode를 physical pass mode와 혼동하지 않는다. Read scalar가 register 값으로 전달되어도 owner 이전을 뜻하지 않는다.

## Target 계약 초안 — D16
Scalar는 target register convention, aggregate는 direct/coerce/indirect, 큰 return은 hidden destination, change는 exclusive pointer로 내린다. return destination과 input alias 보장은 증명된 만큼만 backend attribute로 전달한다.

## 검증
caller/callee가 같은 AbiSignature를 사용해야 한다. scalar/aggregate/Unit/handle/view와 register threshold 경계를 target별 C harness 또는 Nova cross-crate harness로 검사한다. ABI 버전을 artifact에 기록한다.
