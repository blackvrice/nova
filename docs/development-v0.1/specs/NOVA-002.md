# NOVA-002 — Nova 0.1 MVP 기능 동결표

| 항목 | 값 |
|---|---|
| 대상 | Nova 0.1 |
| 작성일 | 2026-10-03 |
| 문서 상태 | Draft — 기존 결정은 유지, 추가 상세는 승인 대기 |
| 적용 Stage | 전 Stage |
| 근거 | [원본 NOVA-002](../../00_Governance/NOVA-002_Nova_0.1_MVP_기능_동결표.md) |

이 문서는 원본을 대체하는 확정 사양이 아니다. 기존 확정 기준은 [CANONICAL](../CANONICAL.md)을 따르며, 새 의미·문법·API·정책은 [DECISIONS](../DECISIONS.md)의 승인이 필요하다. [전체 색인](../INDEX.md).

## Stage별 납품 계약
| Stage | 구현 범위 | 통과 조건 |
|---|---|---|
| A | 단일 UTF-8 파일, 함수/호출, 문자열/정수, let/return/if, 출력, LLVM Object/Link | check 성공과 Hello Nova Native 실행 |
| B | var/const, Primitive/승격, Struct/Enum/Tuple/Array, Method/Constructor, 반복/Pattern/Match, Option/Result/try, Module | 기능별 pass/fail 및 분기/컨테이너 실행 |
| C | Copy/Move, Read/change/take, 초기화/NLL/Drop/View | 금지된 alias 거부와 정확히 한 번 Drop |
| D | Class, Interface, Generic, Closure | 구체화/정적 호출 및 capture 검증 |
| E | Package/Lock/Cache, Formatter, C FFI, Windows/Linux, Core std | 재현 빌드·설치·C 연동 |

Stage B는 언어 의미를 생략한 채 Move 프로그램을 허용하는 단계가 아니다. 소유권 검사가 필요한 값은 C가 준비될 때까지 구현 부분집합에서 제한한다. 핵심 순서는 유지하되 의존성 때문에 C/D가 필요한 라이브러리는 함께 완성한다.

## 완료 증거
각 기능은 사양 절, Fixture ID, 실제 명령 결과, 지원 Target을 연결한다. 아직 구현되지 않은 기능은 current-stage 진단으로 거부하며 release 지원표와 구현 현황을 분리한다.
