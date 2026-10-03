# Nova 0.1 — 개발 문서 보완팩

작성일: 2026-10-03 · 언어: Nova 0.1 · 상태: **검토용 Draft**

기존 Documentation Pack의 148개 주제에 대해 구현 계약, 오류 조건, 검증 사례를 작성했다.
추가로 실제 EBNF, lexical/END/숫자 모델, 표준 API, 파일/schema 계약, 진단 코드,
결정 기록, conformance 계획, 개발/배포 지침을 제공한다. 범위는 Nova 0.1과 그 개발에
필요한 절차다. 비지원 기능은 제외/후속 문서로 명시하며 구현 범위를 확장하지 않는다.

## 먼저 읽기

1. [Canonical 확정 기준](CANONICAL.md): 기존 결정을 유지하는 경계.
2. [원본 사양 감사](SPEC_AUDIT.md): 빈 정의, 충돌, 주제 혼재.
3. [결정 기록 30건](DECISIONS.md): 새 규칙의 제안/대안/영향/승인 조건.
4. [전체 문서 148개 색인](INDEX.md): 주제와 Stage별 계약.
5. [문법 설명](GRAMMAR_NOTES.md)과 [EBNF](GRAMMAR.ebnf), [Lexical](LEXICAL.md), [END](END_RULES.md).
6. [숫자 규칙](NUMERIC_RULES.md), [표준 API](STDLIB_API.md), [FFI 타입](FFI_TYPES.md).
7. [진단 schema](DIAGNOSTIC_SCHEMA.md), [Manifest/Lock/Artifact](MANIFEST_SCHEMA.md), [CLI](CLI_CONTRACTS.md).
8. [수용 테스트](CONFORMANCE.md), [개발 로드맵](ROADMAP.md), [기여/운영](CONTRIBUTING.md).
9. [기계 검증 결과](VALIDATION.md), [변경 기록](CHANGELOG.md).

## 상태와 효력

**작성 완료와 사양 승인, 구현 완료, 테스트 통과는 서로 다른 상태다.** 원본 Canonical
결정은 유지한다. 새 문법·언어 의미·공용 API·ABI·Package 형식은 Draft다. D번호 승인
기록 없이 이 보완팩을 확정 사양으로 구현하지 않는다. 본문에서 '제안'이 생략된 구현
설명도 문서 상태는 Draft다. 기존 코드와 달라지는 규칙 역시 코드에 자동 적용하지 않았다.

구현에 필요한 정의를 빈 칸으로 남기는 대신 하나의 일관된 기본안을 작성하고 대안을
DECISIONS에 기록했다. 원본에 있는 '확정' 상태를 새 내용에 그대로 복사하지 않았다.
단계별 구현 시작 때 해당 Stage의 결정만 먼저 승인·동결할 수 있다.

## 관리

- 원본: 부모 docs의 155개 파일. 이번 작업에서는 보존.
- 보완: 이 디렉터리의 148개 specs와 부속 문서.
- MANIFEST.json: 원본/보완 경로, Stage, 상태, SHA-256 추적.
- topics 작성 원천: tools/docs/topics*.mjs. build-pack.mjs로 specs/index/audit를 재생성.
- 기계 검사: node tools/docs/validate-pack.mjs.

자료구조/API/schema는 설계 계약 예시이며 실행 가능한 현재 compiler API 목록이 아니다.
Nova 예제와 conformance fixture 역시 향후 수용 사례다. 현 compiler는 첫 3개 Rust crate
기반만 존재하고 Rust 실행 검증은 아직 완료되지 않았다.
