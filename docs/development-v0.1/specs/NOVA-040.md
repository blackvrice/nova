# NOVA-040 — main·프로그램 진입점 사양서

| 항목 | 값 |
|---|---|
| 대상 | Nova 0.1 |
| 작성일 | 2026-10-03 |
| 문서 상태 | Draft — 기존 결정은 유지, 추가 상세는 승인 대기 |
| 적용 Stage | A~B |
| 근거 | [원본 NOVA-040](../../04_Functions_Control/NOVA-040_main_프로그램_진입점_사양서.md) |

이 문서는 원본을 대체하는 확정 사양이 아니다. 기존 확정 기준은 [CANONICAL](../CANONICAL.md)을 따르며, 새 의미·문법·API·정책은 [DECISIONS](../DECISIONS.md)의 승인이 필요하다. D01~D05 Lexer 상세는 [승인 기준](../ACCEPTED_LEXER.md)을 따른다. [전체 색인](../INDEX.md).

사용자 승인한 Copy prefix try·Result Error 조기 반환·operand 문맥 격리·정확한 E·const 금지·Source/CFG 검증은 [P16 Accepted](../TRY_STAGE_B_PROPOSAL.md), [51-production EBNF](../GRAMMAR_STAGE_B_TRY.ebnf), [수용 fixture](../try-proposal-fixtures/README.md), [구현 기록](../TRY_IMPLEMENTATION.md)을 따른다. Option try·error conversion·일반 Move/Drop와 전체 D08/D09/D10/D12/D16/D23/D25는 후속이다.

Copy struct Read instance method·contextual self·member scope/visibility·receiver-first snapshot·named/default·Source/MIR 검증은 [P22 Draft](../METHOD_STAGE_B_PROPOSAL.md), [58-production EBNF](../GRAMMAR_STAGE_B_METHOD.ebnf), [제안 fixture](../method-proposal-fixtures/README.md)로 준비했다. 미승인·미구현이다. change/take·Move/borrow/Drop·init·overload·bound method·Enum method·Array와 전체 D06/D09/D10/D11/D12/D16/D25/D30은 승인하지 않았다.

## Stage A 확정
단일 소스의 func main()은 Hello Nova entry다. 실행 파일은 Runtime startup wrapper에서 이를 호출한다. main의 이름을 user LLVM symbol main과 직접 동일시하지 않는다.

## 확장 초안 — D19
0.1 최초 release는 parameter 없는 Unit main만 허용하고 exit=0을 제안한다. int/Result return main, argv parameter는 후속 승인 없이 추정하지 않는다. 환경/인수는 표준 API로 접근하는 제안이다.

## 검증
main 부재, 중복, generic main, non-Unit return, parameter 있는 main은 entry 진단. library target은 main을 요구하지 않는다. Wrapper의 startup 실패와 Nova main panic의 exit 의미를 구분한다.
