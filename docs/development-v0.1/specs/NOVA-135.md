# NOVA-135 — Package Registry 사양서

| 항목 | 값 |
|---|---|
| 대상 | Nova 0.1 |
| 작성일 | 2026-10-03 |
| 문서 상태 | Draft — 기존 결정은 유지, 추가 상세는 승인 대기 |
| 적용 Stage | 제외/후속 |
| 근거 | [원본 NOVA-135](../../12_Tooling_Packaging/NOVA-135_Package_Registry_사양서.md) |

이 문서는 원본을 대체하는 확정 사양이 아니다. 기존 확정 기준은 [CANONICAL](../CANONICAL.md)을 따르며, 새 의미·문법·API·정책은 [DECISIONS](../DECISIONS.md)의 승인이 필요하다. [전체 색인](../INDEX.md).

## Registry 범위
Package Registry 서버는 Canonical 비지원이다. 0.1은 local path와 pinned git source 중심의 package graph를 D21에서 제안한다. registry client/server/publication을 당연한 release requirement로 추가하지 않는다.

## 후속 설계
package identity, namespace ownership, signatures, checksum, immutable releases, yanking, dependency attack model, authentication, rate limits, mirrors/offline cache를 함께 검토한다.

## 검증
현재 manifest에서 unsupported registry source는 명시 오류다. package cache를 registry service와 혼동하지 않는다. 미래 배포 기능은 별도 version proposal과 approval 필요다.
