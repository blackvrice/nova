# NOVA-026 — Primitive·Literal 기본 타입 결정 사양서

| 항목 | 값 |
|---|---|
| 대상 | Nova 0.1 |
| 작성일 | 2026-10-03 |
| 문서 상태 | Draft — 기존 결정은 유지, 추가 상세는 승인 대기 |
| 적용 Stage | A~B |
| 근거 | [원본 NOVA-026](../../03_Types_Declarations/NOVA-026_Primitive_Literal_기본_타입_결정_사양서.md) |

이 문서는 원본을 대체하는 확정 사양이 아니다. 기존 확정 기준은 [CANONICAL](../CANONICAL.md)을 따르며, 새 의미·문법·API·정책은 [DECISIONS](../DECISIONS.md)의 승인이 필요하다. D01~D05 Lexer 상세는 [승인 기준](../ACCEPTED_LEXER.md)을 따른다. [전체 색인](../INDEX.md).

Stage A 단일 파일 의미 검사는 사용자 승인 [P02](../SEMANTICS_STAGE_A_PROPOSAL.md)가 우선한다. runtime/전체 타입/미래 Stage의 추가 상세는 Draft다.

Stage A Native 최소 arithmetic/print/entry/internal ABI/toolchain 계약은 사용자 승인 [P03](../NATIVE_STAGE_A_PROPOSAL.md)를 따른다. [구현·지원·검증 범위](../NATIVE_IMPLEMENTATION.md). 전체 D07~D28과 미래 Stage 정책은 Draft다.

함수 내부 const와 제한된 상수 평가는 사용자 승인 [P05](../CONST_STAGE_B_PROPOSAL.md)와 [전용 EBNF](../GRAMMAR_STAGE_B_CONST.ebnf)가 우선한다. [구현·검증 기록](../CONST_IMPLEMENTATION.md). 전역 상수/const function/전체 D09는 Draft다.

## Primitive 계약
int/uint는 32-bit, float/double은 binary32/binary64 alias다. fixed-width signed/unsigned, bool, Unicode char, owned UTF-8 string, Unit, Never를 구분한다.

## 기본값 초안 — D07
정수 literal은 문맥 우선 후 int32, 실수는 문맥 우선 후 float32. unsigned 문맥의 음수, char의 surrogate, 범위 초과 literal은 오류다. char를 byte/int로 자동 변환하지 않는다.

## 연산 초안
모든 profile의 integer overflow, divide-by-zero, signed MIN/-1, bounds 실패는 Abort panic을 제안한다. 실수는 IEEE binary32/64 semantics, NaN 비교를 그대로 따르고 fast-math 기본 off를 제안한다. explicit cast 범위 밖도 panic, wrapping은 별도 명시 API다.

## 검증
각 width 최소/최대 및 ±1, 음수 최소값 parsing, float32 precision 경계, NaN/-0/infinity, debug/release 결과 일치를 검사한다.
