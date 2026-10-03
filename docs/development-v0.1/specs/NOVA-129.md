# NOVA-129 — Package Artifact·Metadata 형식 사양서

| 항목 | 값 |
|---|---|
| 대상 | Nova 0.1 |
| 작성일 | 2026-10-03 |
| 문서 상태 | Draft — 기존 결정은 유지, 추가 상세는 승인 대기 |
| 적용 Stage | E |
| 근거 | [원본 NOVA-129](../../12_Tooling_Packaging/NOVA-129_Package_Artifact_Metadata_형식_사양서.md) |

이 문서는 원본을 대체하는 확정 사양이 아니다. 기존 확정 기준은 [CANONICAL](../CANONICAL.md)을 따르며, 새 의미·문법·API·정책은 [DECISIONS](../DECISIONS.md)의 승인이 필요하다. [전체 색인](../INDEX.md).

## Artifact metadata 초안 — D21
schema/compiler/language/runtime ABI/Target/profile/package identity/source fingerprint/dependency fingerprints/exports/layout ABI hashes/checksum을 기록한다. object file만 있어도 export/type 정보를 추정할 수 있다는 가정을 금지한다.

## trust
untrusted metadata는 bounds/depth/schema/checksum 검증 후 로드한다. version mismatch면 source rebuild 또는 명시 error다. 경로와 symbol을 shell command로 연결하지 않는다.

## 검증
corrupt/truncated metadata, mismatched Target/ABI/compiler, forged checksum, large graph resource budget, deterministic canonical serialization.
