# NOVA-029 — Tuple·Array·Function 타입 사양서

| 항목 | 값 |
|---|---|
| 대상 | Nova 0.1 |
| 작성일 | 2026-10-03 |
| 문서 상태 | Draft — 기존 결정은 유지, 추가 상세는 승인 대기 |
| 적용 Stage | B~E |
| 근거 | [원본 NOVA-029](../../03_Types_Declarations/NOVA-029_Tuple_Array_Function_타입_사양서.md) |

이 문서는 원본을 대체하는 확정 사양이 아니다. 기존 확정 기준은 [CANONICAL](../CANONICAL.md)을 따르며, 새 의미·문법·API·정책은 [DECISIONS](../DECISIONS.md)의 승인이 필요하다. D01~D05 Lexer 상세는 [승인 기준](../ACCEPTED_LEXER.md)을 따른다. [전체 색인](../INDEX.md).

## Tuple/Array/Function 초안 — D12
()는 Unit, (x,)는 1-tuple, (x,y)는 tuple이다. tuple projection은 .0/.1을 제안한다. [a,b]는 owning Array<T>이고 원본 Array의 길이/용량/초기화 구간 의미를 유지한다. 고정 길이 const generic array는 제외다.

## Function type
func(mode T, ...) -> R 형태를 제안한다. function item과 closure는 내부에서 구분하고 capture 없는 lambda만 plain function pointer로 변환한다. mode가 다른 function type은 같지 않다.

## 검증
tuple arity/type mismatch, empty Array의 expected type 요구, heterogeneous Array 거부, read/change/take 함수 타입 대입, Array 성장 중 element Drop count를 검사한다. Array와 List의 public API 중복은 D23에서 해결한다.
