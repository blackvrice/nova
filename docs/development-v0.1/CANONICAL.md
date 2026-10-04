# 기존 확정 기준과 우선순위

근거: [원본 결정표](../canonical_decisions.json), [공식 어휘](../00_Governance/NOVA-004_Nova_용어_키워드_Canonical_표.md),
[MVP Freeze](../00_Governance/NOVA-002_Nova_0.1_MVP_기능_동결표.md), 사용자가 제공한 개발 지침.

| 항목 | 유지하는 결정 |
|---|---|
| 언어/버전 | Nova 0.1 |
| Compiler | Rust |
| Backend | LLVM Adapter, Native AOT |
| 파일/인코딩 | .nova, UTF-8 |
| Package | nova.toml, nova.lock |
| FFI | foreign |
| 종료 | 선택 세미콜론 + 정규화 END |
| Ownership | 기본 Read, change exclusive borrow, take transfer |
| nullable | T? = Option<T> |
| Unit | void/생략 반환 = Unit, 값 () |
| Generic | Monomorphization |
| Interface | Static dispatch |
| Panic | Abort |
| 비지원 | 구현 상속, 사용자 partial move, dynamic interface object, explicit generic call type args |

## 확정 의미의 추가 근거

NOVA-004: primitive alias, 필드 let/var, until exclusive/through inclusive, 할당은 statement,
comparison chain 금지. NOVA-028/119/120: Some/None, Success/Error, try Error 반환.
NOVA-035: source-order argument evaluation, caller-side defaults, return-only overload 금지.
NOVA-054: NLL last use와 conservative overlap. NOVA-055: local/field 역순 Drop.
NOVA-071/072: 최장 일치, nested block comment, brace interpolation, Recursive Descent+Pratt.
NOVA-078: Nxxxx 영역, byte Span/scalar column. NOVA-091: non-SSA MIR.
NOVA-125: CLI commands/options/exit categories. NOVA-131: 100-column canonical formatter.

## 적용 순서

Canonical → Governance/Freeze → Language → Architecture → Implementation → Code → Tests.
동일 수준의 실제 충돌은 DECISIONS로 해결한다. 원본 공통 양식의 '불변 조건을 명시한다'는
문장은 구체 타입/grammar/ABI 정의를 제공하지 않으므로 없는 의미를 확정이라고 추정하지 않는다.

구현 언어 Rust는 Nova 사용자가 작성하는 언어가 아니다. LLVM이 Nova의 숫자/소유권 의미를
결정하지 않는다. C++ 전환은 본 문서 작업의 범위가 아니며 기존 결정을 변경하지 않는다.

## 승인 경계

이 문서의 기존 결정은 보존이다. 새 상세는 DECISIONS의 Draft다. 승인된 경우에만 관련
원본과 grammar, 테스트 기준을 함께 갱신한다. 원본 파일의 '확정' 라벨은 자동으로 새
보완 문서에 승계되지 않는다.

2026-10-03 사용자가 D01~D05를 승인했다. Lexer 추가 기준은
[ACCEPTED_LEXER](ACCEPTED_LEXER.md)을 따른다. 원본 Canonical 표기의 의미는 유지하고
추가 keyword/Unicode/literal/operator/END 상세만 승인 범위에 적용한다.

2026-10-04 사용자 승인 [P01](PARSER_STAGE_A_PROPOSAL.md)은 Stage A Parser 구문·AST·복구
subset에만 적용한다. Canonical 변경이나 D06~D30 전체 의미 정책 승인은 아니다.

2026-10-04 사용자 승인 [P02](SEMANTICS_STAGE_A_PROPOSAL.md)는 Stage A 단일 파일
HIR/이름·타입 검사 최소 계약이다. runtime/ABI와 미래 Stage 전체 정책 승인은 아니다.
