# NOVA-126 — Package Manifest 사양서

| 항목 | 값 |
|---|---|
| 대상 | Nova 0.1 |
| 작성일 | 2026-10-03 |
| 문서 상태 | Draft — 기존 결정은 유지, 추가 상세는 승인 대기 |
| 적용 Stage | E |
| 근거 | [원본 NOVA-126](../../12_Tooling_Packaging/NOVA-126_Package_Manifest_사양서.md) |

이 문서는 원본을 대체하는 확정 사양이 아니다. 기존 확정 기준은 [CANONICAL](../CANONICAL.md)을 따르며, 새 의미·문법·API·정책은 [DECISIONS](../DECISIONS.md)의 승인이 필요하다. D01~D05 Lexer 상세는 [승인 기준](../ACCEPTED_LEXER.md)을 따른다. [전체 색인](../INDEX.md).

## Manifest 초안 — D21
nova.toml은 schema-version, [package], [[target]], [dependencies], [profile.*]를 가진다. package name/version/edition, target kind/bin/lib/path를 validation한다. MANIFEST_SCHEMA에 최소 예제가 있다.

## 처리
duplicate key, unknown schema major, 잘못된 version/path/target를 명확히 거부한다. relative path는 manifest directory 기준이며 dependency alias 중복을 허용하지 않는다. build scripts/network hooks는 0.1 첫 승인안에서 제외한다.

## 검증
최소 bin/lib, duplicate dependencies, path escape policy, manifest 없는 source temporary package, unicode directory, source root collision. TOML library 선택은 implementation detail다.
