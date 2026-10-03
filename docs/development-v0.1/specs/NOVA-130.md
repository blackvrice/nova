# NOVA-130 — 증분 컴파일 Cache 형식 사양서

| 항목 | 값 |
|---|---|
| 대상 | Nova 0.1 |
| 작성일 | 2026-10-03 |
| 문서 상태 | Draft — 기존 결정은 유지, 추가 상세는 승인 대기 |
| 적용 Stage | E |
| 근거 | [원본 NOVA-130](../../12_Tooling_Packaging/NOVA-130_증분_컴파일_Cache_형식_사양서.md) |

이 문서는 원본을 대체하는 확정 사양이 아니다. 기존 확정 기준은 [CANONICAL](../CANONICAL.md)을 따르며, 새 의미·문법·API·정책은 [DECISIONS](../DECISIONS.md)의 승인이 필요하다. D01~D05 Lexer 상세는 [승인 기준](../ACCEPTED_LEXER.md)을 따른다. [전체 색인](../INDEX.md).

## Disk cache 초안 — D21
key는 normalized source+transitive relevant dependencies+compiler/runtime/Target/options/schema를 hash한다. executable cache와 frontend query cache를 별도 저장한다.

## Storage
temp write→checksum→atomic rename, per-entry concurrency lock, stale lock 복구, bounded eviction을 제안한다. damaged entries는 실패 성공으로 사용하지 않고 miss/rebuild한다. absolute workspace path normalization이 SourceInfo를 손상시키지 않아야 한다.

## 검증
unchanged hit, compiler flag change miss, std ABI change miss, interruption mid-write, parallel writer, full-cache permission error graceful handling. cache disable build와 관찰 결과가 같다.
