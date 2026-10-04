# 문서 동결과 Nova 구현 로드맵

상태 Draft. Stage A~E 순서는 기존 Freeze를 유지한다. 문서 작성은 완료했어도 아래 승인/구현
gate가 자동 충족되지 않는다. 미래 문서를 전부 승인해야 첫 Lexer 작업을 할 수 있다는 뜻은 아니다.

| Gate | 먼저 동결할 결정/자료 | 구현 산출물 | 완료 증거 |
|---|---|---|---|
| 0 기반 | 기존 IDs/Source/Diagnostic, D02/D25 필요한 부분 | 현재 3 crates의 검증/오류 API 정리 | Cargo 4 checks, Unicode/Span/renderer tests |
| A Frontend | D01~D08 최소 부분, D11 call subset, EBNF/END | syntax/lexer/AST/parser/HIR/resolve/types/typecheck | production snapshots와 pass/fail |
| A Native | D16/D18/D19/D22/D23 print/D28 host pin | 최소 MIR/validation/backend/runtime/link CLI | Hello check/run, output/exit, invalid source codegen 차단 |
| B 기본 언어 | D06~D12 type/control/aggregate, D09 const | modules, primitives, aggregates, loops/match/try | T008~T019/T047~T048과 multi-file corpus |
| C 안전성 | D10/D20 predicate core/View API | initialization/Move/NLL/Drop/View | T020~T022/T041와 sanitizer corpus |
| D 추상화 | D11~D15/D23 generic APIs | Class/interface/generic/closure | T023~T025, specialization cache/inference |
| E 제품화 | D16~D26/D28~D30 | package/lock/cache/formatter/C FFI/std/Targets | locked clean builds, C ABI, tooling, release checklist |

## 상호 의존성 처리

Source Array/Option/Result 문법은 B에서 준비하되 Move element-safe execution은 C, 사용자
Generic/Closure 기반 std API 완성은 D에 의존한다. 특정 라이브러리 기능의 납품을 뒤 Stage로
미루고 안전성 검사를 생략하지 않는다. C/D를 Stage A 전에 선행 구현하는 방식은 금지한다.

## 첫 구현 재개 조건

1. Rust frontend toolchain 및 기존 Cargo fmt/clippy/test/check 실행 완료.
2. D01~D05 승인과 Token/Lexer/Stage A END 구현 완료. D07/D08의 Stage A numeric/condition 의미는 승인 필요.
3. Stage A parser fixtures의 source+expected diagnostic Span 구체화.
4. SourceInfo/Diagnostic API에 승인된 변화만 적용, Lexer부터 순서대로 구현.

현재 Source/Diagnostic과 Token/Lexer/END, P01 Stage A AST/Parser를 구현했다.
2026-10-04 사용자 승인 [P01](PARSER_STAGE_A_PROPOSAL.md)은 구문·복구 subset에 적용한다.
문서 검증 스크립트 결과와 compiler tests 통과를 구분한다. 다음은 HIR lowering과
최소 이름/타입 검사다. D06/D07/D08/D11의 필요한 의미 계약은 구현 전에 별도 확인한다.

## Backlog 경계

Pin/self-reference, dynamic objects/vtable/associated types, async/generator, registry server,
hosted .NET/JVM/Python, self-hosting은 별도 버전/범위 검토. Stage A의 Advanced Optimization,
borrow checker/generalized factory 등은 앞당겨 구현하지 않는다.
