# NOVA-127 — 의존성 해석·Lock File 사양서

| 항목 | 값 |
|---|---|
| 대상 | Nova 0.1 |
| 작성일 | 2026-10-03 |
| 문서 상태 | Draft — 기존 결정은 유지, 추가 상세는 승인 대기 |
| 적용 Stage | E |
| 근거 | [원본 NOVA-127](../../12_Tooling_Packaging/NOVA-127_의존성_해석_Lock_File_사양서.md) |

이 문서는 원본을 대체하는 확정 사양이 아니다. 기존 확정 기준은 [CANONICAL](../CANONICAL.md)을 따르며, 새 의미·문법·API·정책은 [DECISIONS](../DECISIONS.md)의 승인이 필요하다. [전체 색인](../INDEX.md).

## Lock 계약
nova.lock은 exact version/source/revision/checksum과 graph를 기록한다. unresolved range가 남지 않으며 registry 서버는 0.1에 구현하지 않는다.

## 초안 — D21
path dependency는 source content fingerprint, git dependency는 full immutable revision을 기록한다. --locked는 변경 필요 시 오류, --offline은 cached source 없으면 오류로 제안한다. resolver는 semver constraint backtracking와 conflict chain을 출력한다.

## 검증
conflict, checksum mismatch, source alias, missing offline package, locked build graph 동일, atomic lock write. SHA-256 source archive checksum과 local tree fingerprint 포맷을 혼용하지 않는다.
