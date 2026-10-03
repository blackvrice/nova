# Language Server·LSP 사양서

| 항목 | 값 |
|---|---|
| 문서 ID | NOVA-133 |
| 대상 | Nova 0.1 |
| 구현 시점 | MVP 이후 |
| 상태 | 후속 |
| 갱신일 | 2026-07-27 |

## 1. 목적

이 문서는 **Language Server·LSP 사양서**에 대한 언어 의미, Compiler 책임, 구현 경계, 오류 처리 및 완료 기준을 정의한다.

이 기능은 MVP 이후 확장을 위한 사양이며 Nova 0.1 Compiler는 암묵적으로 허용하지 않는다.

## 2. Canonical 기준

- 구현 언어: Rust
- Backend: LLVM Adapter
- Source: UTF-8 `.nova`
- Manifest: `nova.toml`
- 외부 연동 키워드: `foreign`
- 소유권: Read / `change` / `take`
- `T?`는 `Option<T>`
- `void`와 생략 반환은 Unit, 값은 `()`
- Generic은 Monomorphization
- Interface는 정적 디스패치
- Panic MVP는 Abort

## 3. 공통 규칙

1. Manifest·Artifact·Cache는 버전 필드를 가진다.
2. 빌드는 재현 가능해야 한다.
3. Formatter는 의미를 변경하지 않는다.

## 4. 확정 규칙

1. Language Server·LSP의 입력·출력·불변 조건을 명시한다.
2. Language Server·LSP의 MVP 범위와 후속 범위를 분리한다.
3. Language Server·LSP의 결과는 컴파일 순서와 병렬 실행 순서에 독립적이어야 한다.

추가 불변 조건:

1. 평가 순서·Drop 순서·소유권 의미는 최적화로 변경하지 않는다.
2. 문자열 이름은 의미 분석 단계에서 안정적 ID로 변환한다.
3. Error Node·Error Type은 복구를 위해 허용하지만 Codegen 입력에는 남지 않는다.
4. 미지원 기능은 추측해서 실행하지 않고 안정적인 진단으로 거부한다.
5. 동일 입력과 옵션은 결정적인 결과를 생성한다.

## 5. 구현 요구사항

1. Language Server·LSP 전용 데이터 모델과 Query 경계를 구현한다.
2. 정상 결과와 오류 복구 결과를 구분한다.
3. Debug dump와 SourceInfo를 제공한다.

권장 처리 구조:

```text
input/query
→ validation
→ canonical model
→ analysis/lowering
→ verification
→ dump/metadata
```

각 내부 항목은 가능한 경우 StableId, SourceInfo, OwnerId, TypeId 또는 Mode를 가진다.

## 6. 오류·진단

- 사용자 오류는 안정적인 `Nxxxx` Code를 가진다.
- 실제 수정 위치를 Primary Span으로 표시한다.
- 선언·호출·제약 위치를 Secondary Note로 표시한다.
- 안전하게 적용 가능한 경우만 자동 수정 Suggestion을 제공한다.
- ErrorType에서 파생된 중복 오류를 억제한다.
- Compiler 불변 조건 위반은 ICE로 보고한다.

## 7. 테스트

- Language Server·LSP의 정상·오류·경계 사례를 테스트한다.
- 대형 입력·증분 무효화·결정성을 검증한다.

공통 항목:

- 최소 정상 입력
- 대표 실패 입력
- 빈 값과 경계값
- 깊은 중첩과 대형 입력
- Snapshot 결정성
- 증분 재빌드 무효화
- 병렬 실행 결과
- Fuzz 회귀 Corpus

## 8. 완료 기준

- 구현이 본 문서의 확정 규칙과 일치한다.
- Compile-pass와 Compile-fail 테스트가 통과한다.
- Debug Dump가 안정적이고 Source 위치를 보존한다.
- 잘못된 입력에서도 Compiler가 Panic하지 않는다.
- 관련 Query와 Cache 무효화 테스트가 통과한다.
- MVP에서는 미지원 진단이 존재하고 기존 0.1 의미를 깨지 않는 확장 지점이 준비되어 있다.

## 9. 변경 절차

범위 변경은 NOVA-006의 변경 제안·영향 분석·결정 기록 절차를 따른다.
