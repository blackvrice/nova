# 문서 기계 검증 결과

검사일: 2026-10-03. 명령: node tools/docs/validate-pack.mjs

결과: **PASS**

- 148개 NOVA ID 연속성/중복/원본·보완 SHA-256/상태/본문 섹션
- 1136개 로컬 Markdown 링크/코드 fence/D·T 참조
- 30개 Draft 결정, 60개 수용 묶음, 41개 진단 코드 유일성/영역
- 94개 EBNF production 중복/미정의·도달 불가 nonterminal 검사 (무모호성 증명 아님)
- 20개 예제 sidecar/UTF-8 byte Span/등록 code 검사 (컴파일 실행 아님)

## 검증의 범위

이 검사는 문서 구조/링크/ID/hash/grammar 참조와 fixture 데이터 유효성을 확인한다. 언어 사양 승인, EBNF 무모호성/완전성 증명, parser/semantic 구현 검증, 예제 Native 실행, Rust fmt/clippy/test/check는 실행하지 않았다. 초안 수용 테스트가 통과했다는 뜻이 아니다.
