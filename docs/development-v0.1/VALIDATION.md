# 문서 기계 검증 결과

검사일: 2026-10-07. 명령: node tools/docs/validate-pack.mjs

결과: **PASS**

- 148개 NOVA ID 연속성/중복/원본·보완 SHA-256/상태/본문 섹션
- 2362개 로컬 Markdown 링크/코드 fence/D·T 참조
- 30개 결정 (Accepted 5 / Draft 25), 60개 수용 묶음, 42개 진단 코드 유일성/영역
- GRAMMAR.ebnf: 94개 EBNF production 중복/미정의·도달 불가 nonterminal 검사 (무모호성 증명 아님)
- GRAMMAR_STAGE_A.ebnf: 26개 EBNF production 중복/미정의·도달 불가 nonterminal 검사 (무모호성 증명 아님)
- GRAMMAR_STAGE_B_CONTROL.ebnf: 30개 EBNF production 중복/미정의·도달 불가 nonterminal 검사 (무모호성 증명 아님)
- GRAMMAR_STAGE_B_CONST.ebnf: 30개 EBNF production 중복/미정의·도달 불가 nonterminal 검사 (무모호성 증명 아님)
- GRAMMAR_STAGE_B_GLOBAL_CONST.ebnf: 31개 EBNF production 중복/미정의·도달 불가 nonterminal 검사 (무모호성 증명 아님)
- GRAMMAR_STAGE_B_CHAR.ebnf: 31개 EBNF production 중복/미정의·도달 불가 nonterminal 검사 (무모호성 증명 아님)
- GRAMMAR_STAGE_B_FLOAT.ebnf: 31개 EBNF production 중복/미정의·도달 불가 nonterminal 검사 (무모호성 증명 아님)
- GRAMMAR_STAGE_B_CAST.ebnf: 31개 EBNF production 중복/미정의·도달 불가 nonterminal 검사 (무모호성 증명 아님)
- GRAMMAR_STAGE_B_MODULE.ebnf: 34개 EBNF production 중복/미정의·도달 불가 nonterminal 검사 (무모호성 증명 아님)
- GRAMMAR_STAGE_B_STRUCT.ebnf: 37개 EBNF production 중복/미정의·도달 불가 nonterminal 검사 (무모호성 증명 아님)
- GRAMMAR_STAGE_B_TUPLE.ebnf: 40개 EBNF production 중복/미정의·도달 불가 nonterminal 검사 (무모호성 증명 아님)
- P01 Stage A Parser 승인 범위/날짜와 전용 EBNF 기록
- P02 Stage A 의미 검사 승인 범위/날짜 기록
- P03 Stage A Native 승인 subset/날짜 기록
- P04 Stage B control 승인 subset/날짜/전용 EBNF와 N3004 등록
- P05 local const 승인 subset/날짜/전용 EBNF 기록
- P06 global const 승인 subset/날짜/전용 EBNF·P02 print shadow 보존 정정
- P07 integer 승인 subset/날짜·P06 grammar 재사용·전체 D07 Draft 경계
- P08 char 승인 subset/날짜·전용 EBNF는 CHAR primary만 추가·전체 D07 Draft 경계
- P09 float 승인 subset/날짜·전용 EBNF는 FLOAT primary만 추가·전체 D07 Draft 경계
- P10 숫자 cast 승인 subset/날짜·전용 EBNF는 as type postfix만 추가·전체 D07 Draft 경계
- P11 Module 승인 subset/날짜·accepted ledger·기존 P10 production 보존·34-production EBNF와 수용 2-file fixture 데이터 (컴파일 실행 아님)
- P12 Accepted/승인·구현 ledger·P11의 세 production 변경/세 production 추가·37-production EBNF·두 파일/부정 10사례 UTF-8 Span·검증 결과 데이터 (컴파일 실행 아님)
- P13 Accepted/승인·구현 ledger·P12 네 production 확장/세 production 추가·40-production EBNF·두 파일/부정 10사례 UTF-8 Span·검증 결과 데이터 (컴파일 실행 아님)
- 20개 예제 sidecar/UTF-8 byte Span/등록 code 검사 (컴파일 실행 아님)

## 검증의 범위

이 검사는 문서 구조/링크/ID/hash/grammar 참조와 fixture 데이터 유효성을 확인한다. 언어 사양 승인, EBNF 무모호성/완전성 증명, parser/semantic 구현 검증, 예제 Native 실행, Rust fmt/clippy/test/check는 실행하지 않았다. 초안 수용 테스트가 통과했다는 뜻이 아니다.
