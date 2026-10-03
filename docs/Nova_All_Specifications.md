# Nova 0.1 전체 사양서 통합본


---

# Nova 언어 설계 원칙·목표·비목표

| 항목 | 값 |
|---|---|
| 문서 ID | NOVA-001 |
| 대상 | Nova 0.1 |
| 구현 시점 | 착수 전 |
| 상태 | 확정 |
| 갱신일 | 2026-07-27 |

## 1. 목적

이 문서는 **Nova 언어 설계 원칙·목표·비목표**에 대한 언어 의미, Compiler 책임, 구현 경계, 오류 처리 및 완료 기준을 정의한다.

이 문서는 해당 구현 단계에 진입하기 전 동결해야 하는 기준이다.

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

1. Canonical 결정표가 다른 문서보다 우선한다.
2. MVP 동결 이후 새 기능은 후속 버전으로 분리한다.
3. 변경은 결정 기록과 영향 분석을 남긴다.

## 4. 확정 규칙

1. Nova 언어 설계 원칙·목표·비목표의 입력·출력·불변 조건을 명시한다.
2. Nova 언어 설계 원칙·목표·비목표의 MVP 범위와 후속 범위를 분리한다.
3. Nova 언어 설계 원칙·목표·비목표의 결과는 컴파일 순서와 병렬 실행 순서에 독립적이어야 한다.

추가 불변 조건:

1. 평가 순서·Drop 순서·소유권 의미는 최적화로 변경하지 않는다.
2. 문자열 이름은 의미 분석 단계에서 안정적 ID로 변환한다.
3. Error Node·Error Type은 복구를 위해 허용하지만 Codegen 입력에는 남지 않는다.
4. 미지원 기능은 추측해서 실행하지 않고 안정적인 진단으로 거부한다.
5. 동일 입력과 옵션은 결정적인 결과를 생성한다.

## 5. 구현 요구사항

1. Nova 언어 설계 원칙·목표·비목표 전용 데이터 모델과 Query 경계를 구현한다.
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

- Nova 언어 설계 원칙·목표·비목표의 정상·오류·경계 사례를 테스트한다.
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


## 9. 변경 절차

범위 변경은 NOVA-006의 변경 제안·영향 분석·결정 기록 절차를 따른다.


---

# Nova 0.1 MVP 기능 동결표

## 개발 시작선

다음 기반을 만들면 나머지 문서를 기다리지 않고 구현을 시작한다.

1. Source Manager와 Span
2. Diagnostic Engine
3. Lexer
4. Parser와 AST
5. HIR Lowering
6. 최소 이름·타입 검사
7. Snapshot·Compile Test Harness

## Stage A — Hello Nova

지원:

- UTF-8 단일 파일
- `func main()`
- 문자열·정수 Literal
- 함수 선언·호출
- `let`
- `return`
- `if`
- Console 출력
- LLVM Object와 실행 파일

완료 프로그램:

```nova
func main() {
    print("Hello, Nova")
}
```

완료 명령:

```bash
nova check examples/hello.nova
nova run examples/hello.nova
```

## Stage B — 기본 언어

- `let`, `var`, `const`
- Primitive와 숫자 승격
- Struct, Enum, Tuple, Array
- 함수·메서드·생성자
- `if`, `while`, `for`, `loop`
- Pattern과 `match`
- `Option<T>`, `Result<T,E>`, `try`
- Module과 다중 파일

## Stage C — 소유권

- Copy·Move
- Read·`take`·`change`
- 초기화·Move 분석
- 사용자 부분 이동 금지
- 자동 Drop·Drop Flag
- NLL Borrow Checker
- Span·ReadOnlySpan

## Stage D — 추상화

- Class Handle
- Interface·`implements`
- Generic·Monomorphization
- Lambda·Closure

## Stage E — 제품화

- Package·Lock
- Incremental Cache
- Formatter
- C FFI
- Windows·Linux Target
- Core 표준 라이브러리

## Nova 0.1 제외

- Class 구현 상속
- Dynamic Interface Object
- Associated Type
- Async·Await
- Coroutine·Generator
- Pinning·Self-reference
- Panic Unwind
- Reflection
- Const Generic
- 명시적 Generic 호출 인수
- 사용자 Operator·Property
- Package Registry 서버
- Self-hosting

## 범위 통제

Stage A 완료 전 Stage C 이상의 기능을 구현하지 않는다. 새 기능은 Backlog 문서로만 기록한다.


---

# Nova 0.1 비지원 기능 목록

| 항목 | 값 |
|---|---|
| 문서 ID | NOVA-003 |
| 대상 | Nova 0.1 |
| 구현 시점 | 착수 전 |
| 상태 | 확정 |
| 갱신일 | 2026-07-27 |

## 1. 목적

이 문서는 **Nova 0.1 비지원 기능 목록**에 대한 언어 의미, Compiler 책임, 구현 경계, 오류 처리 및 완료 기준을 정의한다.

이 문서는 해당 구현 단계에 진입하기 전 동결해야 하는 기준이다.

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

1. Canonical 결정표가 다른 문서보다 우선한다.
2. MVP 동결 이후 새 기능은 후속 버전으로 분리한다.
3. 변경은 결정 기록과 영향 분석을 남긴다.

## 4. 확정 규칙

1. Nova 0.1 비지원 기능 목록의 입력·출력·불변 조건을 명시한다.
2. Nova 0.1 비지원 기능 목록의 MVP 범위와 후속 범위를 분리한다.
3. Nova 0.1 비지원 기능 목록의 결과는 컴파일 순서와 병렬 실행 순서에 독립적이어야 한다.

추가 불변 조건:

1. 평가 순서·Drop 순서·소유권 의미는 최적화로 변경하지 않는다.
2. 문자열 이름은 의미 분석 단계에서 안정적 ID로 변환한다.
3. Error Node·Error Type은 복구를 위해 허용하지만 Codegen 입력에는 남지 않는다.
4. 미지원 기능은 추측해서 실행하지 않고 안정적인 진단으로 거부한다.
5. 동일 입력과 옵션은 결정적인 결과를 생성한다.

## 5. 구현 요구사항

1. Nova 0.1 비지원 기능 목록 전용 데이터 모델과 Query 경계를 구현한다.
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

- Nova 0.1 비지원 기능 목록의 정상·오류·경계 사례를 테스트한다.
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


## 9. 변경 절차

범위 변경은 NOVA-006의 변경 제안·영향 분석·결정 기록 절차를 따른다.


---

# Nova 용어·키워드 Canonical 표

## 공식 키워드

```text
func let var const
struct class enum interface implements
init drop
if else while for in loop
break continue return match
true false none
change take
shared weak view
using try panic
pure
public internal private
static where
foreign unsafe
until through
as exists
```

## 제외·예약 용어

- `trait`: 소스 키워드가 아니다. 공식 용어는 `interface`.
- `external`: 소스 키워드가 아니다. 내부 설명 용어로만 사용.
- `extends`: 구현 상속 미지원.
- `async`, `await`, `yield`, `dynamic`, `any`: 후속 예약 후보.
- `operator`, `property`: Nova 0.1 미지원.

## 기본 타입

```text
bool byte char
int uint float double
int8 int16 int32 int64
uint8 uint16 uint32 uint64
string void never
```

정규화:

- `byte = uint8`
- `int = int32`
- `uint = uint32`
- `float = float32`
- `double = float64`
- `void`·생략 반환 = `Unit`
- Unit 값 = `()`
- `T? = Option<T>`

## 이름 규칙

- 타입·Interface: UpperCamelCase
- 함수·메서드·지역: lowerCamelCase
- 상수: UPPER_SNAKE_CASE
- 필드는 항상 `let` 또는 `var`

## 소유권

| 문법 | 의미 |
|---|---|
| Modifier 없음 | Read |
| `change` | 독점 수정 빌림 |
| `take` | 소유권 이전 |
| `shared` | 공유 소유 |
| `weak` | 약한 비소유 Handle |
| `view` | 원본 종속 참조 |

## 제어 흐름

- `until`: 끝 미포함
- `through`: 끝 포함
- Match Arm: `pattern => { ... }`
- 할당은 표현식이 아니라 문장
- 연속 비교는 금지


---

# Nova 문서 인덱스·의존 관계도

| 항목 | 값 |
|---|---|
| 문서 ID | NOVA-005 |
| 대상 | Nova 0.1 |
| 구현 시점 | 착수 전 |
| 상태 | 확정 |
| 갱신일 | 2026-07-27 |

## 1. 목적

이 문서는 **Nova 문서 인덱스·의존 관계도**에 대한 언어 의미, Compiler 책임, 구현 경계, 오류 처리 및 완료 기준을 정의한다.

이 문서는 해당 구현 단계에 진입하기 전 동결해야 하는 기준이다.

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

1. Canonical 결정표가 다른 문서보다 우선한다.
2. MVP 동결 이후 새 기능은 후속 버전으로 분리한다.
3. 변경은 결정 기록과 영향 분석을 남긴다.

## 4. 확정 규칙

1. Nova 문서 인덱스·의존 관계도의 입력·출력·불변 조건을 명시한다.
2. Nova 문서 인덱스·의존 관계도의 MVP 범위와 후속 범위를 분리한다.
3. Nova 문서 인덱스·의존 관계도의 결과는 컴파일 순서와 병렬 실행 순서에 독립적이어야 한다.

추가 불변 조건:

1. 평가 순서·Drop 순서·소유권 의미는 최적화로 변경하지 않는다.
2. 문자열 이름은 의미 분석 단계에서 안정적 ID로 변환한다.
3. Error Node·Error Type은 복구를 위해 허용하지만 Codegen 입력에는 남지 않는다.
4. 미지원 기능은 추측해서 실행하지 않고 안정적인 진단으로 거부한다.
5. 동일 입력과 옵션은 결정적인 결과를 생성한다.

## 5. 구현 요구사항

1. Nova 문서 인덱스·의존 관계도 전용 데이터 모델과 Query 경계를 구현한다.
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

- Nova 문서 인덱스·의존 관계도의 정상·오류·경계 사례를 테스트한다.
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


## 9. 변경 절차

범위 변경은 NOVA-006의 변경 제안·영향 분석·결정 기록 절차를 따른다.


---

# Nova 언어 변경 제안·결정 기록 절차

| 항목 | 값 |
|---|---|
| 문서 ID | NOVA-006 |
| 대상 | Nova 0.1 |
| 구현 시점 | 구현 병행 |
| 상태 | 확정 |
| 갱신일 | 2026-07-27 |

## 1. 목적

이 문서는 **Nova 언어 변경 제안·결정 기록 절차**에 대한 언어 의미, Compiler 책임, 구현 경계, 오류 처리 및 완료 기준을 정의한다.

이 문서는 해당 구현 단계에 진입하기 전 동결해야 하는 기준이다.

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

1. Canonical 결정표가 다른 문서보다 우선한다.
2. MVP 동결 이후 새 기능은 후속 버전으로 분리한다.
3. 변경은 결정 기록과 영향 분석을 남긴다.

## 4. 확정 규칙

1. Nova 언어 변경 제안·결정 기록 절차의 입력·출력·불변 조건을 명시한다.
2. Nova 언어 변경 제안·결정 기록 절차의 MVP 범위와 후속 범위를 분리한다.
3. Nova 언어 변경 제안·결정 기록 절차의 결과는 컴파일 순서와 병렬 실행 순서에 독립적이어야 한다.

추가 불변 조건:

1. 평가 순서·Drop 순서·소유권 의미는 최적화로 변경하지 않는다.
2. 문자열 이름은 의미 분석 단계에서 안정적 ID로 변환한다.
3. Error Node·Error Type은 복구를 위해 허용하지만 Codegen 입력에는 남지 않는다.
4. 미지원 기능은 추측해서 실행하지 않고 안정적인 진단으로 거부한다.
5. 동일 입력과 옵션은 결정적인 결과를 생성한다.

## 5. 구현 요구사항

1. Nova 언어 변경 제안·결정 기록 절차 전용 데이터 모델과 Query 경계를 구현한다.
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

- Nova 언어 변경 제안·결정 기록 절차의 정상·오류·경계 사례를 테스트한다.
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


## 9. 변경 절차

범위 변경은 NOVA-006의 변경 제안·영향 분석·결정 기록 절차를 따른다.


---

# Nova 버전 정책·호환성 원칙

| 항목 | 값 |
|---|---|
| 문서 ID | NOVA-007 |
| 대상 | Nova 0.1 |
| 구현 시점 | MVP 후반 |
| 상태 | 확정 |
| 갱신일 | 2026-07-27 |

## 1. 목적

이 문서는 **Nova 버전 정책·호환성 원칙**에 대한 언어 의미, Compiler 책임, 구현 경계, 오류 처리 및 완료 기준을 정의한다.

이 문서는 해당 구현 단계에 진입하기 전 동결해야 하는 기준이다.

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

1. Canonical 결정표가 다른 문서보다 우선한다.
2. MVP 동결 이후 새 기능은 후속 버전으로 분리한다.
3. 변경은 결정 기록과 영향 분석을 남긴다.

## 4. 확정 규칙

1. Nova 버전 정책·호환성 원칙의 입력·출력·불변 조건을 명시한다.
2. Nova 버전 정책·호환성 원칙의 MVP 범위와 후속 범위를 분리한다.
3. Nova 버전 정책·호환성 원칙의 결과는 컴파일 순서와 병렬 실행 순서에 독립적이어야 한다.

추가 불변 조건:

1. 평가 순서·Drop 순서·소유권 의미는 최적화로 변경하지 않는다.
2. 문자열 이름은 의미 분석 단계에서 안정적 ID로 변환한다.
3. Error Node·Error Type은 복구를 위해 허용하지만 Codegen 입력에는 남지 않는다.
4. 미지원 기능은 추측해서 실행하지 않고 안정적인 진단으로 거부한다.
5. 동일 입력과 옵션은 결정적인 결과를 생성한다.

## 5. 구현 요구사항

1. Nova 버전 정책·호환성 원칙 전용 데이터 모델과 Query 경계를 구현한다.
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

- Nova 버전 정책·호환성 원칙의 정상·오류·경계 사례를 테스트한다.
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


## 9. 변경 절차

범위 변경은 NOVA-006의 변경 제안·영향 분석·결정 기록 절차를 따른다.


---

# 소스 인코딩·Unicode·줄바꿈 사양서

| 항목 | 값 |
|---|---|
| 문서 ID | NOVA-008 |
| 대상 | Nova 0.1 |
| 구현 시점 | 착수 전 |
| 상태 | 확정 |
| 갱신일 | 2026-07-27 |

## 1. 목적

이 문서는 **소스 인코딩·Unicode·줄바꿈 사양서**에 대한 언어 의미, Compiler 책임, 구현 경계, 오류 처리 및 완료 기준을 정의한다.

이 문서는 해당 구현 단계에 진입하기 전 동결해야 하는 기준이다.

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

1. 소스는 UTF-8이다.
2. 중괄호는 필수이고 세미콜론은 선택 사항이다.
3. 유효한 줄바꿈은 END로 정규화한다.
4. Parser는 Recursive Descent와 Pratt를 조합한다.

## 4. 확정 규칙

1. 소스 인코딩·Unicode·줄바꿈의 입력·출력·불변 조건을 명시한다.
2. 소스 인코딩·Unicode·줄바꿈의 MVP 범위와 후속 범위를 분리한다.
3. 소스 인코딩·Unicode·줄바꿈의 결과는 컴파일 순서와 병렬 실행 순서에 독립적이어야 한다.

추가 불변 조건:

1. 평가 순서·Drop 순서·소유권 의미는 최적화로 변경하지 않는다.
2. 문자열 이름은 의미 분석 단계에서 안정적 ID로 변환한다.
3. Error Node·Error Type은 복구를 위해 허용하지만 Codegen 입력에는 남지 않는다.
4. 미지원 기능은 추측해서 실행하지 않고 안정적인 진단으로 거부한다.
5. 동일 입력과 옵션은 결정적인 결과를 생성한다.

## 5. 구현 요구사항

1. 소스 인코딩·Unicode·줄바꿈 전용 데이터 모델과 Query 경계를 구현한다.
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

- 소스 인코딩·Unicode·줄바꿈의 정상·오류·경계 사례를 테스트한다.
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


## 9. 변경 절차

범위 변경은 NOVA-006의 변경 제안·영향 분석·결정 기록 절차를 따른다.


---

# Token 종류·예약어·연산자 사양서

| 항목 | 값 |
|---|---|
| 문서 ID | NOVA-009 |
| 대상 | Nova 0.1 |
| 구현 시점 | 착수 전 |
| 상태 | 확정 |
| 갱신일 | 2026-07-27 |

## 1. 목적

이 문서는 **Token 종류·예약어·연산자 사양서**에 대한 언어 의미, Compiler 책임, 구현 경계, 오류 처리 및 완료 기준을 정의한다.

이 문서는 해당 구현 단계에 진입하기 전 동결해야 하는 기준이다.

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

1. 소스는 UTF-8이다.
2. 중괄호는 필수이고 세미콜론은 선택 사항이다.
3. 유효한 줄바꿈은 END로 정규화한다.
4. Parser는 Recursive Descent와 Pratt를 조합한다.

## 4. 확정 규칙

1. Token 종류·예약어·연산자의 입력·출력·불변 조건을 명시한다.
2. Token 종류·예약어·연산자의 MVP 범위와 후속 범위를 분리한다.
3. Token 종류·예약어·연산자의 결과는 컴파일 순서와 병렬 실행 순서에 독립적이어야 한다.

추가 불변 조건:

1. 평가 순서·Drop 순서·소유권 의미는 최적화로 변경하지 않는다.
2. 문자열 이름은 의미 분석 단계에서 안정적 ID로 변환한다.
3. Error Node·Error Type은 복구를 위해 허용하지만 Codegen 입력에는 남지 않는다.
4. 미지원 기능은 추측해서 실행하지 않고 안정적인 진단으로 거부한다.
5. 동일 입력과 옵션은 결정적인 결과를 생성한다.

## 5. 구현 요구사항

1. Token 종류·예약어·연산자 전용 데이터 모델과 Query 경계를 구현한다.
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

- Token 종류·예약어·연산자의 정상·오류·경계 사례를 테스트한다.
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


## 9. 변경 절차

범위 변경은 NOVA-006의 변경 제안·영향 분석·결정 기록 절차를 따른다.


---

# 숫자·문자·문자열 Literal 사양서

| 항목 | 값 |
|---|---|
| 문서 ID | NOVA-010 |
| 대상 | Nova 0.1 |
| 구현 시점 | 착수 전 |
| 상태 | 확정 |
| 갱신일 | 2026-07-27 |

## 1. 목적

이 문서는 **숫자·문자·문자열 Literal 사양서**에 대한 언어 의미, Compiler 책임, 구현 경계, 오류 처리 및 완료 기준을 정의한다.

이 문서는 해당 구현 단계에 진입하기 전 동결해야 하는 기준이다.

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

1. 소스는 UTF-8이다.
2. 중괄호는 필수이고 세미콜론은 선택 사항이다.
3. 유효한 줄바꿈은 END로 정규화한다.
4. Parser는 Recursive Descent와 Pratt를 조합한다.

## 4. 확정 규칙

1. 숫자·문자·문자열 Literal의 입력·출력·불변 조건을 명시한다.
2. 숫자·문자·문자열 Literal의 MVP 범위와 후속 범위를 분리한다.
3. 숫자·문자·문자열 Literal의 결과는 컴파일 순서와 병렬 실행 순서에 독립적이어야 한다.

추가 불변 조건:

1. 평가 순서·Drop 순서·소유권 의미는 최적화로 변경하지 않는다.
2. 문자열 이름은 의미 분석 단계에서 안정적 ID로 변환한다.
3. Error Node·Error Type은 복구를 위해 허용하지만 Codegen 입력에는 남지 않는다.
4. 미지원 기능은 추측해서 실행하지 않고 안정적인 진단으로 거부한다.
5. 동일 입력과 옵션은 결정적인 결과를 생성한다.

## 5. 구현 요구사항

1. 숫자·문자·문자열 Literal 전용 데이터 모델과 Query 경계를 구현한다.
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

- 숫자·문자·문자열 Literal의 정상·오류·경계 사례를 테스트한다.
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


## 9. 변경 절차

범위 변경은 NOVA-006의 변경 제안·영향 분석·결정 기록 절차를 따른다.


---

# 문자열 보간 Lexer Mode 사양서

| 항목 | 값 |
|---|---|
| 문서 ID | NOVA-011 |
| 대상 | Nova 0.1 |
| 구현 시점 | Lexer 전 |
| 상태 | 확정 |
| 갱신일 | 2026-07-27 |

## 1. 목적

이 문서는 **문자열 보간 Lexer Mode 사양서**에 대한 언어 의미, Compiler 책임, 구현 경계, 오류 처리 및 완료 기준을 정의한다.

이 문서는 해당 구현 단계에 진입하기 전 동결해야 하는 기준이다.

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

1. 소스는 UTF-8이다.
2. 중괄호는 필수이고 세미콜론은 선택 사항이다.
3. 유효한 줄바꿈은 END로 정규화한다.
4. Parser는 Recursive Descent와 Pratt를 조합한다.

## 4. 확정 규칙

1. 최장 일치로 Token을 생성한다.
2. Trivia와 Token을 분리하되 소스 재구성이 가능해야 한다.
3. 문자열·주석·보간은 별도 Mode로 처리한다.

추가 불변 조건:

1. 평가 순서·Drop 순서·소유권 의미는 최적화로 변경하지 않는다.
2. 문자열 이름은 의미 분석 단계에서 안정적 ID로 변환한다.
3. Error Node·Error Type은 복구를 위해 허용하지만 Codegen 입력에는 남지 않는다.
4. 미지원 기능은 추측해서 실행하지 않고 안정적인 진단으로 거부한다.
5. 동일 입력과 옵션은 결정적인 결과를 생성한다.

## 5. 구현 요구사항

1. TokenKind, cursor, mode stack, diagnostics를 구현한다.

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

- Unicode, 중첩 주석, 보간, 잘못된 문자를 테스트한다.

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


## 9. 변경 절차

범위 변경은 NOVA-006의 변경 제안·영향 분석·결정 기록 절차를 따른다.


---

# 주석·중첩 주석 사양서

| 항목 | 값 |
|---|---|
| 문서 ID | NOVA-012 |
| 대상 | Nova 0.1 |
| 구현 시점 | Lexer 전 |
| 상태 | 확정 |
| 갱신일 | 2026-07-27 |

## 1. 목적

이 문서는 **주석·중첩 주석 사양서**에 대한 언어 의미, Compiler 책임, 구현 경계, 오류 처리 및 완료 기준을 정의한다.

이 문서는 해당 구현 단계에 진입하기 전 동결해야 하는 기준이다.

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

1. 소스는 UTF-8이다.
2. 중괄호는 필수이고 세미콜론은 선택 사항이다.
3. 유효한 줄바꿈은 END로 정규화한다.
4. Parser는 Recursive Descent와 Pratt를 조합한다.

## 4. 확정 규칙

1. 주석·중첩 주석의 입력·출력·불변 조건을 명시한다.
2. 주석·중첩 주석의 MVP 범위와 후속 범위를 분리한다.
3. 주석·중첩 주석의 결과는 컴파일 순서와 병렬 실행 순서에 독립적이어야 한다.

추가 불변 조건:

1. 평가 순서·Drop 순서·소유권 의미는 최적화로 변경하지 않는다.
2. 문자열 이름은 의미 분석 단계에서 안정적 ID로 변환한다.
3. Error Node·Error Type은 복구를 위해 허용하지만 Codegen 입력에는 남지 않는다.
4. 미지원 기능은 추측해서 실행하지 않고 안정적인 진단으로 거부한다.
5. 동일 입력과 옵션은 결정적인 결과를 생성한다.

## 5. 구현 요구사항

1. 주석·중첩 주석 전용 데이터 모델과 Query 경계를 구현한다.
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

- 주석·중첩 주석의 정상·오류·경계 사례를 테스트한다.
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


## 9. 변경 절차

범위 변경은 NOVA-006의 변경 제안·영향 분석·결정 기록 절차를 따른다.


---

# 줄바꿈·문장 종료·연속 줄 규칙

| 항목 | 값 |
|---|---|
| 문서 ID | NOVA-013 |
| 대상 | Nova 0.1 |
| 구현 시점 | Parser 전 |
| 상태 | 확정 |
| 갱신일 | 2026-07-27 |

## 1. 목적

이 문서는 **줄바꿈·문장 종료·연속 줄 규칙**에 대한 언어 의미, Compiler 책임, 구현 경계, 오류 처리 및 완료 기준을 정의한다.

이 문서는 해당 구현 단계에 진입하기 전 동결해야 하는 기준이다.

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

1. 소스는 UTF-8이다.
2. 중괄호는 필수이고 세미콜론은 선택 사항이다.
3. 유효한 줄바꿈은 END로 정규화한다.
4. Parser는 Recursive Descent와 Pratt를 조합한다.

## 4. 확정 규칙

1. 줄바꿈·문장 종료·연속 줄 규칙의 입력·출력·불변 조건을 명시한다.
2. 줄바꿈·문장 종료·연속 줄 규칙의 MVP 범위와 후속 범위를 분리한다.
3. 줄바꿈·문장 종료·연속 줄 규칙의 결과는 컴파일 순서와 병렬 실행 순서에 독립적이어야 한다.

추가 불변 조건:

1. 평가 순서·Drop 순서·소유권 의미는 최적화로 변경하지 않는다.
2. 문자열 이름은 의미 분석 단계에서 안정적 ID로 변환한다.
3. Error Node·Error Type은 복구를 위해 허용하지만 Codegen 입력에는 남지 않는다.
4. 미지원 기능은 추측해서 실행하지 않고 안정적인 진단으로 거부한다.
5. 동일 입력과 옵션은 결정적인 결과를 생성한다.

## 5. 구현 요구사항

1. 줄바꿈·문장 종료·연속 줄 규칙 전용 데이터 모델과 Query 경계를 구현한다.
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

- 줄바꿈·문장 종료·연속 줄 규칙의 정상·오류·경계 사례를 테스트한다.
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


## 9. 변경 절차

범위 변경은 NOVA-006의 변경 제안·영향 분석·결정 기록 절차를 따른다.


---

# 문법 명세 및 EBNF 사양서

| 항목 | 값 |
|---|---|
| 문서 ID | NOVA-014 |
| 대상 | Nova 0.1 |
| 구현 시점 | Parser 전 |
| 상태 | 확정 |
| 갱신일 | 2026-07-27 |

## 1. 목적

이 문서는 **문법 명세 및 EBNF 사양서**에 대한 언어 의미, Compiler 책임, 구현 경계, 오류 처리 및 완료 기준을 정의한다.

이 문서는 해당 구현 단계에 진입하기 전 동결해야 하는 기준이다.

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

1. 소스는 UTF-8이다.
2. 중괄호는 필수이고 세미콜론은 선택 사항이다.
3. 유효한 줄바꿈은 END로 정규화한다.
4. Parser는 Recursive Descent와 Pratt를 조합한다.

## 4. 확정 규칙

1. 문법 명세 및 EBNF의 입력·출력·불변 조건을 명시한다.
2. 문법 명세 및 EBNF의 MVP 범위와 후속 범위를 분리한다.
3. 문법 명세 및 EBNF의 결과는 컴파일 순서와 병렬 실행 순서에 독립적이어야 한다.

추가 불변 조건:

1. 평가 순서·Drop 순서·소유권 의미는 최적화로 변경하지 않는다.
2. 문자열 이름은 의미 분석 단계에서 안정적 ID로 변환한다.
3. Error Node·Error Type은 복구를 위해 허용하지만 Codegen 입력에는 남지 않는다.
4. 미지원 기능은 추측해서 실행하지 않고 안정적인 진단으로 거부한다.
5. 동일 입력과 옵션은 결정적인 결과를 생성한다.

## 5. 구현 요구사항

1. 문법 명세 및 EBNF 전용 데이터 모델과 Query 경계를 구현한다.
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

- 문법 명세 및 EBNF의 정상·오류·경계 사례를 테스트한다.
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


## 9. 변경 절차

범위 변경은 NOVA-006의 변경 제안·영향 분석·결정 기록 절차를 따른다.


---

# 연산자 우선순위·결합 방향 사양서

| 항목 | 값 |
|---|---|
| 문서 ID | NOVA-015 |
| 대상 | Nova 0.1 |
| 구현 시점 | Parser 전 |
| 상태 | 확정 |
| 갱신일 | 2026-07-27 |

## 1. 목적

이 문서는 **연산자 우선순위·결합 방향 사양서**에 대한 언어 의미, Compiler 책임, 구현 경계, 오류 처리 및 완료 기준을 정의한다.

이 문서는 해당 구현 단계에 진입하기 전 동결해야 하는 기준이다.

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

1. 소스는 UTF-8이다.
2. 중괄호는 필수이고 세미콜론은 선택 사항이다.
3. 유효한 줄바꿈은 END로 정규화한다.
4. Parser는 Recursive Descent와 Pratt를 조합한다.

## 4. 확정 규칙

1. 연산자 우선순위·결합 방향의 입력·출력·불변 조건을 명시한다.
2. 연산자 우선순위·결합 방향의 MVP 범위와 후속 범위를 분리한다.
3. 연산자 우선순위·결합 방향의 결과는 컴파일 순서와 병렬 실행 순서에 독립적이어야 한다.

추가 불변 조건:

1. 평가 순서·Drop 순서·소유권 의미는 최적화로 변경하지 않는다.
2. 문자열 이름은 의미 분석 단계에서 안정적 ID로 변환한다.
3. Error Node·Error Type은 복구를 위해 허용하지만 Codegen 입력에는 남지 않는다.
4. 미지원 기능은 추측해서 실행하지 않고 안정적인 진단으로 거부한다.
5. 동일 입력과 옵션은 결정적인 결과를 생성한다.

## 5. 구현 요구사항

1. 연산자 우선순위·결합 방향 전용 데이터 모델과 Query 경계를 구현한다.
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

- 연산자 우선순위·결합 방향의 정상·오류·경계 사례를 테스트한다.
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


## 9. 변경 절차

범위 변경은 NOVA-006의 변경 제안·영향 분석·결정 기록 절차를 따른다.


---

# Parser·AST 사양서

| 항목 | 값 |
|---|---|
| 문서 ID | NOVA-016 |
| 대상 | Nova 0.1 |
| 구현 시점 | 착수 전 |
| 상태 | 확정 |
| 갱신일 | 2026-07-27 |

## 1. 목적

이 문서는 **Parser·AST 사양서**에 대한 언어 의미, Compiler 책임, 구현 경계, 오류 처리 및 완료 기준을 정의한다.

이 문서는 해당 구현 단계에 진입하기 전 동결해야 하는 기준이다.

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

1. 소스는 UTF-8이다.
2. 중괄호는 필수이고 세미콜론은 선택 사항이다.
3. 유효한 줄바꿈은 END로 정규화한다.
4. Parser는 Recursive Descent와 Pratt를 조합한다.

## 4. 확정 규칙

1. 선언·문장은 Recursive Descent, 표현식은 Pratt Parser를 사용한다.
2. Error Node와 Synthetic Token으로 복구한다.
3. AST는 소스 구조와 오류를 보존한다.
4. Semantic Type과 DefId는 AST에 저장하지 않는다.

추가 불변 조건:

1. 평가 순서·Drop 순서·소유권 의미는 최적화로 변경하지 않는다.
2. 문자열 이름은 의미 분석 단계에서 안정적 ID로 변환한다.
3. Error Node·Error Type은 복구를 위해 허용하지만 Codegen 입력에는 남지 않는다.
4. 미지원 기능은 추측해서 실행하지 않고 안정적인 진단으로 거부한다.
5. 동일 입력과 옵션은 결정적인 결과를 생성한다.

## 5. 구현 요구사항

1. delimiter stack과 동기화 지점을 구현한다.
2. Arena와 AstNodeId를 사용한다.

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

- 불완전 코드와 괄호 누락을 테스트한다.
- Span·Error Node·Visitor를 테스트한다.

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


## 9. 변경 절차

범위 변경은 NOVA-006의 변경 제안·영향 분석·결정 기록 절차를 따른다.


---

# Parser 오류 복구 사양서

| 항목 | 값 |
|---|---|
| 문서 ID | NOVA-017 |
| 대상 | Nova 0.1 |
| 구현 시점 | 착수 전 |
| 상태 | 확정 |
| 갱신일 | 2026-07-27 |

## 1. 목적

이 문서는 **Parser 오류 복구 사양서**에 대한 언어 의미, Compiler 책임, 구현 경계, 오류 처리 및 완료 기준을 정의한다.

이 문서는 해당 구현 단계에 진입하기 전 동결해야 하는 기준이다.

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

1. 소스는 UTF-8이다.
2. 중괄호는 필수이고 세미콜론은 선택 사항이다.
3. 유효한 줄바꿈은 END로 정규화한다.
4. Parser는 Recursive Descent와 Pratt를 조합한다.

## 4. 확정 규칙

1. 선언·문장은 Recursive Descent, 표현식은 Pratt Parser를 사용한다.
2. Error Node와 Synthetic Token으로 복구한다.

추가 불변 조건:

1. 평가 순서·Drop 순서·소유권 의미는 최적화로 변경하지 않는다.
2. 문자열 이름은 의미 분석 단계에서 안정적 ID로 변환한다.
3. Error Node·Error Type은 복구를 위해 허용하지만 Codegen 입력에는 남지 않는다.
4. 미지원 기능은 추측해서 실행하지 않고 안정적인 진단으로 거부한다.
5. 동일 입력과 옵션은 결정적인 결과를 생성한다.

## 5. 구현 요구사항

1. delimiter stack과 동기화 지점을 구현한다.

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

- 불완전 코드와 괄호 누락을 테스트한다.

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


## 9. 변경 절차

범위 변경은 NOVA-006의 변경 제안·영향 분석·결정 기록 절차를 따른다.


---

# 소스 포매팅 기준서

| 항목 | 값 |
|---|---|
| 문서 ID | NOVA-018 |
| 대상 | Nova 0.1 |
| 구현 시점 | Formatter 전 |
| 상태 | 확정 |
| 갱신일 | 2026-07-27 |

## 1. 목적

이 문서는 **소스 포매팅 기준서**에 대한 언어 의미, Compiler 책임, 구현 경계, 오류 처리 및 완료 기준을 정의한다.

이 문서는 해당 구현 단계에 진입하기 전 동결해야 하는 기준이다.

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

1. 소스는 UTF-8이다.
2. 중괄호는 필수이고 세미콜론은 선택 사항이다.
3. 유효한 줄바꿈은 END로 정규화한다.
4. Parser는 Recursive Descent와 Pratt를 조합한다.

## 4. 확정 규칙

1. 소스 포매팅의 입력·출력·불변 조건을 명시한다.
2. 소스 포매팅의 MVP 범위와 후속 범위를 분리한다.
3. 소스 포매팅의 결과는 컴파일 순서와 병렬 실행 순서에 독립적이어야 한다.

추가 불변 조건:

1. 평가 순서·Drop 순서·소유권 의미는 최적화로 변경하지 않는다.
2. 문자열 이름은 의미 분석 단계에서 안정적 ID로 변환한다.
3. Error Node·Error Type은 복구를 위해 허용하지만 Codegen 입력에는 남지 않는다.
4. 미지원 기능은 추측해서 실행하지 않고 안정적인 진단으로 거부한다.
5. 동일 입력과 옵션은 결정적인 결과를 생성한다.

## 5. 구현 요구사항

1. 소스 포매팅 전용 데이터 모델과 Query 경계를 구현한다.
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

- 소스 포매팅의 정상·오류·경계 사례를 테스트한다.
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


## 9. 변경 절차

범위 변경은 NOVA-006의 변경 제안·영향 분석·결정 기록 절차를 따른다.


---

# 이름 해석·Scope 사양서

| 항목 | 값 |
|---|---|
| 문서 ID | NOVA-019 |
| 대상 | Nova 0.1 |
| 구현 시점 | 의미 분석 전 |
| 상태 | 확정 |
| 갱신일 | 2026-07-27 |

## 1. 목적

이 문서는 **이름 해석·Scope 사양서**에 대한 언어 의미, Compiler 책임, 구현 경계, 오류 처리 및 완료 기준을 정의한다.

이 문서는 해당 구현 단계에 진입하기 전 동결해야 하는 기준이다.

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

1. 선언 수집 후 참조를 해석한다.
2. 이름 참조는 안정적 ID로 변환한다.
3. Import 순서는 결과에 영향을 주지 않는다.

## 4. 확정 규칙

1. 타입·값·모듈 이름 공간을 분리한다.
2. 가장 가까운 Scope를 우선한다.
3. 모호한 이름은 임의 선택하지 않는다.

추가 불변 조건:

1. 평가 순서·Drop 순서·소유권 의미는 최적화로 변경하지 않는다.
2. 문자열 이름은 의미 분석 단계에서 안정적 ID로 변환한다.
3. Error Node·Error Type은 복구를 위해 허용하지만 Codegen 입력에는 남지 않는다.
4. 미지원 기능은 추측해서 실행하지 않고 안정적인 진단으로 거부한다.
5. 동일 입력과 옵션은 결정적인 결과를 생성한다.

## 5. 구현 요구사항

1. ScopeTree, DefinitionRegistry, ResolutionMap을 구현한다.

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

- Shadowing·중복·접근 제한을 테스트한다.

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


## 9. 변경 절차

범위 변경은 NOVA-006의 변경 제안·영향 분석·결정 기록 절차를 따른다.


---

# Module·Import 사양서

| 항목 | 값 |
|---|---|
| 문서 ID | NOVA-020 |
| 대상 | Nova 0.1 |
| 구현 시점 | 의미 분석 전 |
| 상태 | 확정 |
| 갱신일 | 2026-07-27 |

## 1. 목적

이 문서는 **Module·Import 사양서**에 대한 언어 의미, Compiler 책임, 구현 경계, 오류 처리 및 완료 기준을 정의한다.

이 문서는 해당 구현 단계에 진입하기 전 동결해야 하는 기준이다.

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

1. 선언 수집 후 참조를 해석한다.
2. 이름 참조는 안정적 ID로 변환한다.
3. Import 순서는 결과에 영향을 주지 않는다.

## 4. 확정 규칙

1. 파일 경로에서 Module 경로를 결정한다.
2. 순환 참조는 허용하되 순환 초기화는 금지한다.

추가 불변 조건:

1. 평가 순서·Drop 순서·소유권 의미는 최적화로 변경하지 않는다.
2. 문자열 이름은 의미 분석 단계에서 안정적 ID로 변환한다.
3. Error Node·Error Type은 복구를 위해 허용하지만 Codegen 입력에는 남지 않는다.
4. 미지원 기능은 추측해서 실행하지 않고 안정적인 진단으로 거부한다.
5. 동일 입력과 옵션은 결정적인 결과를 생성한다.

## 5. 구현 요구사항

1. ModuleGraph와 ImportEdge를 구현한다.

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

- Alias·재Export·순환을 테스트한다.

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


## 9. 변경 절차

범위 변경은 NOVA-006의 변경 제안·영향 분석·결정 기록 절차를 따른다.


---

# 접근 제한·가시성 사양서

| 항목 | 값 |
|---|---|
| 문서 ID | NOVA-021 |
| 대상 | Nova 0.1 |
| 구현 시점 | 의미 분석 전 |
| 상태 | 확정 |
| 갱신일 | 2026-07-27 |

## 1. 목적

이 문서는 **접근 제한·가시성 사양서**에 대한 언어 의미, Compiler 책임, 구현 경계, 오류 처리 및 완료 기준을 정의한다.

이 문서는 해당 구현 단계에 진입하기 전 동결해야 하는 기준이다.

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

1. 선언 수집 후 참조를 해석한다.
2. 이름 참조는 안정적 ID로 변환한다.
3. Import 순서는 결과에 영향을 주지 않는다.

## 4. 확정 규칙

1. 접근 제한·가시성의 입력·출력·불변 조건을 명시한다.
2. 접근 제한·가시성의 MVP 범위와 후속 범위를 분리한다.
3. 접근 제한·가시성의 결과는 컴파일 순서와 병렬 실행 순서에 독립적이어야 한다.

추가 불변 조건:

1. 평가 순서·Drop 순서·소유권 의미는 최적화로 변경하지 않는다.
2. 문자열 이름은 의미 분석 단계에서 안정적 ID로 변환한다.
3. Error Node·Error Type은 복구를 위해 허용하지만 Codegen 입력에는 남지 않는다.
4. 미지원 기능은 추측해서 실행하지 않고 안정적인 진단으로 거부한다.
5. 동일 입력과 옵션은 결정적인 결과를 생성한다.

## 5. 구현 요구사항

1. 접근 제한·가시성 전용 데이터 모델과 Query 경계를 구현한다.
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

- 접근 제한·가시성의 정상·오류·경계 사례를 테스트한다.
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


## 9. 변경 절차

범위 변경은 NOVA-006의 변경 제안·영향 분석·결정 기록 절차를 따른다.


---

# Symbol·Definition ID 사양서

| 항목 | 값 |
|---|---|
| 문서 ID | NOVA-022 |
| 대상 | Nova 0.1 |
| 구현 시점 | HIR 전 |
| 상태 | 확정 |
| 갱신일 | 2026-07-27 |

## 1. 목적

이 문서는 **Symbol·Definition ID 사양서**에 대한 언어 의미, Compiler 책임, 구현 경계, 오류 처리 및 완료 기준을 정의한다.

이 문서는 해당 구현 단계에 진입하기 전 동결해야 하는 기준이다.

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

1. 선언 수집 후 참조를 해석한다.
2. 이름 참조는 안정적 ID로 변환한다.
3. Import 순서는 결과에 영향을 주지 않는다.

## 4. 확정 규칙

1. Symbol·Definition ID의 입력·출력·불변 조건을 명시한다.
2. Symbol·Definition ID의 MVP 범위와 후속 범위를 분리한다.
3. Symbol·Definition ID의 결과는 컴파일 순서와 병렬 실행 순서에 독립적이어야 한다.

추가 불변 조건:

1. 평가 순서·Drop 순서·소유권 의미는 최적화로 변경하지 않는다.
2. 문자열 이름은 의미 분석 단계에서 안정적 ID로 변환한다.
3. Error Node·Error Type은 복구를 위해 허용하지만 Codegen 입력에는 남지 않는다.
4. 미지원 기능은 추측해서 실행하지 않고 안정적인 진단으로 거부한다.
5. 동일 입력과 옵션은 결정적인 결과를 생성한다.

## 5. 구현 요구사항

1. Symbol·Definition ID 전용 데이터 모델과 Query 경계를 구현한다.
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

- Symbol·Definition ID의 정상·오류·경계 사례를 테스트한다.
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


## 9. 변경 절차

범위 변경은 NOVA-006의 변경 제안·영향 분석·결정 기록 절차를 따른다.


---

# Package 간 이름 해석 사양서

| 항목 | 값 |
|---|---|
| 문서 ID | NOVA-023 |
| 대상 | Nova 0.1 |
| 구현 시점 | 패키지 기능 전 |
| 상태 | 확정 |
| 갱신일 | 2026-07-27 |

## 1. 목적

이 문서는 **Package 간 이름 해석 사양서**에 대한 언어 의미, Compiler 책임, 구현 경계, 오류 처리 및 완료 기준을 정의한다.

이 문서는 해당 구현 단계에 진입하기 전 동결해야 하는 기준이다.

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

1. 선언 수집 후 참조를 해석한다.
2. 이름 참조는 안정적 ID로 변환한다.
3. Import 순서는 결과에 영향을 주지 않는다.

## 4. 확정 규칙

1. 타입·값·모듈 이름 공간을 분리한다.
2. 가장 가까운 Scope를 우선한다.
3. 모호한 이름은 임의 선택하지 않는다.

추가 불변 조건:

1. 평가 순서·Drop 순서·소유권 의미는 최적화로 변경하지 않는다.
2. 문자열 이름은 의미 분석 단계에서 안정적 ID로 변환한다.
3. Error Node·Error Type은 복구를 위해 허용하지만 Codegen 입력에는 남지 않는다.
4. 미지원 기능은 추측해서 실행하지 않고 안정적인 진단으로 거부한다.
5. 동일 입력과 옵션은 결정적인 결과를 생성한다.

## 5. 구현 요구사항

1. ScopeTree, DefinitionRegistry, ResolutionMap을 구현한다.

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

- Shadowing·중복·접근 제한을 테스트한다.

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


## 9. 변경 절차

범위 변경은 NOVA-006의 변경 제안·영향 분석·결정 기록 절차를 따른다.


---

# 이름 충돌·Shadowing 진단 기준서

| 항목 | 값 |
|---|---|
| 문서 ID | NOVA-024 |
| 대상 | Nova 0.1 |
| 구현 시점 | 의미 분석 전 |
| 상태 | 확정 |
| 갱신일 | 2026-07-27 |

## 1. 목적

이 문서는 **이름 충돌·Shadowing 진단 기준서**에 대한 언어 의미, Compiler 책임, 구현 경계, 오류 처리 및 완료 기준을 정의한다.

이 문서는 해당 구현 단계에 진입하기 전 동결해야 하는 기준이다.

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

1. 선언 수집 후 참조를 해석한다.
2. 이름 참조는 안정적 ID로 변환한다.
3. Import 순서는 결과에 영향을 주지 않는다.

## 4. 확정 규칙

1. 타입·값·모듈 이름 공간을 분리한다.
2. 가장 가까운 Scope를 우선한다.
3. 모호한 이름은 임의 선택하지 않는다.
4. 모든 오류는 안정적 Error Code를 가진다.
5. Primary·Secondary Label과 Help를 구분한다.
6. 연쇄 오류를 억제한다.

추가 불변 조건:

1. 평가 순서·Drop 순서·소유권 의미는 최적화로 변경하지 않는다.
2. 문자열 이름은 의미 분석 단계에서 안정적 ID로 변환한다.
3. Error Node·Error Type은 복구를 위해 허용하지만 Codegen 입력에는 남지 않는다.
4. 미지원 기능은 추측해서 실행하지 않고 안정적인 진단으로 거부한다.
5. 동일 입력과 옵션은 결정적인 결과를 생성한다.

## 5. 구현 요구사항

1. ScopeTree, DefinitionRegistry, ResolutionMap을 구현한다.
2. Terminal·JSON renderer를 구현한다.

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

- Shadowing·중복·접근 제한을 테스트한다.
- Unicode 열·다중 파일·Snapshot을 테스트한다.

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


## 9. 변경 절차

범위 변경은 NOVA-006의 변경 제안·영향 분석·결정 기록 절차를 따른다.


---

# 타입 시스템·타입 추론·형변환 사양서

| 항목 | 값 |
|---|---|
| 문서 ID | NOVA-025 |
| 대상 | Nova 0.1 |
| 구현 시점 | 타입 검사 전 |
| 상태 | 확정 |
| 갱신일 | 2026-07-27 |

## 1. 목적

이 문서는 **타입 시스템·타입 추론·형변환 사양서**에 대한 언어 의미, Compiler 책임, 구현 경계, 오류 처리 및 완료 기준을 정의한다.

이 문서는 해당 구현 단계에 진입하기 전 동결해야 하는 기준이다.

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

1. int=int32, float=float32, double=float64다.
2. T?는 Option<T>, void는 Unit으로 정규화한다.
3. Class는 소유 Handle이며 구현 상속은 없다.

## 4. 확정 규칙

1. 양방향 타입 검사를 사용한다.
2. ErrorType으로 연쇄 오류를 억제한다.
3. 손실 없는 제한적 숫자 승격만 암묵 허용한다.

추가 불변 조건:

1. 평가 순서·Drop 순서·소유권 의미는 최적화로 변경하지 않는다.
2. 문자열 이름은 의미 분석 단계에서 안정적 ID로 변환한다.
3. Error Node·Error Type은 복구를 위해 허용하지만 Codegen 입력에는 남지 않는다.
4. 미지원 기능은 추측해서 실행하지 않고 안정적인 진단으로 거부한다.
5. 동일 입력과 옵션은 결정적인 결과를 생성한다.

## 5. 구현 요구사항

1. TypeInterner, Unification, ConversionCost를 구현한다.

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

- Literal·Nullable·함수·Generic 타입을 테스트한다.

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


## 9. 변경 절차

범위 변경은 NOVA-006의 변경 제안·영향 분석·결정 기록 절차를 따른다.


---

# Primitive·Literal 기본 타입 결정 사양서

| 항목 | 값 |
|---|---|
| 문서 ID | NOVA-026 |
| 대상 | Nova 0.1 |
| 구현 시점 | 타입 검사 전 |
| 상태 | 확정 |
| 갱신일 | 2026-07-27 |

## 1. 목적

이 문서는 **Primitive·Literal 기본 타입 결정 사양서**에 대한 언어 의미, Compiler 책임, 구현 경계, 오류 처리 및 완료 기준을 정의한다.

이 문서는 해당 구현 단계에 진입하기 전 동결해야 하는 기준이다.

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

1. int=int32, float=float32, double=float64다.
2. T?는 Option<T>, void는 Unit으로 정규화한다.
3. Class는 소유 Handle이며 구현 상속은 없다.

## 4. 확정 규칙

1. 양방향 타입 검사를 사용한다.
2. ErrorType으로 연쇄 오류를 억제한다.
3. 손실 없는 제한적 숫자 승격만 암묵 허용한다.

추가 불변 조건:

1. 평가 순서·Drop 순서·소유권 의미는 최적화로 변경하지 않는다.
2. 문자열 이름은 의미 분석 단계에서 안정적 ID로 변환한다.
3. Error Node·Error Type은 복구를 위해 허용하지만 Codegen 입력에는 남지 않는다.
4. 미지원 기능은 추측해서 실행하지 않고 안정적인 진단으로 거부한다.
5. 동일 입력과 옵션은 결정적인 결과를 생성한다.

## 5. 구현 요구사항

1. TypeInterner, Unification, ConversionCost를 구현한다.

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

- Literal·Nullable·함수·Generic 타입을 테스트한다.

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


## 9. 변경 절차

범위 변경은 NOVA-006의 변경 제안·영향 분석·결정 기록 절차를 따른다.


---

# Struct·Class·Enum 선언 사양서

| 항목 | 값 |
|---|---|
| 문서 ID | NOVA-027 |
| 대상 | Nova 0.1 |
| 구현 시점 | 타입 검사 전 |
| 상태 | 확정 |
| 갱신일 | 2026-07-27 |

## 1. 목적

이 문서는 **Struct·Class·Enum 선언 사양서**에 대한 언어 의미, Compiler 책임, 구현 경계, 오류 처리 및 완료 기준을 정의한다.

이 문서는 해당 구현 단계에 진입하기 전 동결해야 하는 기준이다.

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

1. int=int32, float=float32, double=float64다.
2. T?는 Option<T>, void는 Unit으로 정규화한다.
3. Class는 소유 Handle이며 구현 상속은 없다.

## 4. 확정 규칙

1. Class는 객체 정체성을 가진 소유 Handle이다.
2. Move는 Handle 소유권 이전이다.
3. 필드 Drop 후 저장 공간을 해제한다.
4. Struct는 값 의미를 가진다.
5. 필드는 let 또는 var를 명시한다.
6. 직접 재귀 값 포함은 금지한다.
7. 활성 Variant Payload만 읽고 Drop한다.
8. Option과 Result는 일반 Enum 규칙을 따른다.

추가 불변 조건:

1. 평가 순서·Drop 순서·소유권 의미는 최적화로 변경하지 않는다.
2. 문자열 이름은 의미 분석 단계에서 안정적 ID로 변환한다.
3. Error Node·Error Type은 복구를 위해 허용하지만 Codegen 입력에는 남지 않는다.
4. 미지원 기능은 추측해서 실행하지 않고 안정적인 진단으로 거부한다.
5. 동일 입력과 옵션은 결정적인 결과를 생성한다.

## 5. 구현 요구사항

1. ClassLayout과 Handle ABI를 분리한다.
2. 필드 초기화와 Copy 판정을 구현한다.
3. VariantId, Discriminant, Payload Layout을 구현한다.

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

- 정체성·Move·Shared·Drop을 테스트한다.
- 생성자·레이아웃·Drop을 테스트한다.
- Match·Generic·Niche·Drop을 테스트한다.

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


## 9. 변경 절차

범위 변경은 NOVA-006의 변경 제안·영향 분석·결정 기록 절차를 따른다.


---

# Nullable·Option·Result 타입 규칙

| 항목 | 값 |
|---|---|
| 문서 ID | NOVA-028 |
| 대상 | Nova 0.1 |
| 구현 시점 | 타입 검사 전 |
| 상태 | 확정 |
| 갱신일 | 2026-07-27 |

## 1. 목적

이 문서는 **Nullable·Option·Result 타입 규칙**에 대한 언어 의미, Compiler 책임, 구현 경계, 오류 처리 및 완료 기준을 정의한다.

이 문서는 해당 구현 단계에 진입하기 전 동결해야 하는 기준이다.

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

1. int=int32, float=float32, double=float64다.
2. T?는 Option<T>, void는 Unit으로 정규화한다.
3. Class는 소유 Handle이며 구현 상속은 없다.

## 4. 확정 규칙

1. 양방향 타입 검사를 사용한다.
2. ErrorType으로 연쇄 오류를 억제한다.
3. 손실 없는 제한적 숫자 승격만 암묵 허용한다.
4. Option은 Some·None이다.
5. T?는 Option<T> Sugar다.
6. Niche는 내부 최적화다.
7. Result는 Success·Error다.
8. try는 Error를 조기 반환한다.
9. Success(())로 Unit 성공을 표현한다.

추가 불변 조건:

1. 평가 순서·Drop 순서·소유권 의미는 최적화로 변경하지 않는다.
2. 문자열 이름은 의미 분석 단계에서 안정적 ID로 변환한다.
3. Error Node·Error Type은 복구를 위해 허용하지만 Codegen 입력에는 남지 않는다.
4. 미지원 기능은 추측해서 실행하지 않고 안정적인 진단으로 거부한다.
5. 동일 입력과 옵션은 결정적인 결과를 생성한다.

## 5. 구현 요구사항

1. TypeInterner, Unification, ConversionCost를 구현한다.
2. Option API와 match integration을 구현한다.
3. Result API와 try lowering을 구현한다.

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

- Literal·Nullable·함수·Generic 타입을 테스트한다.
- Copy·Move·Drop payload를 테스트한다.
- 중첩 try·cleanup·payload move를 테스트한다.

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


## 9. 변경 절차

범위 변경은 NOVA-006의 변경 제안·영향 분석·결정 기록 절차를 따른다.


---

# Tuple·Array·Function 타입 사양서

| 항목 | 값 |
|---|---|
| 문서 ID | NOVA-029 |
| 대상 | Nova 0.1 |
| 구현 시점 | 타입 검사 전 |
| 상태 | 확정 |
| 갱신일 | 2026-07-27 |

## 1. 목적

이 문서는 **Tuple·Array·Function 타입 사양서**에 대한 언어 의미, Compiler 책임, 구현 경계, 오류 처리 및 완료 기준을 정의한다.

이 문서는 해당 구현 단계에 진입하기 전 동결해야 하는 기준이다.

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

1. int=int32, float=float32, double=float64다.
2. T?는 Option<T>, void는 Unit으로 정규화한다.
3. Class는 소유 Handle이며 구현 상속은 없다.

## 4. 확정 규칙

1. 양방향 타입 검사를 사용한다.
2. ErrorType으로 연쇄 오류를 억제한다.
3. 손실 없는 제한적 숫자 승격만 암묵 허용한다.
4. Array는 길이·용량·초기화 구간을 관리한다.
5. 원소는 정확히 한 번 Drop한다.

추가 불변 조건:

1. 평가 순서·Drop 순서·소유권 의미는 최적화로 변경하지 않는다.
2. 문자열 이름은 의미 분석 단계에서 안정적 ID로 변환한다.
3. Error Node·Error Type은 복구를 위해 허용하지만 Codegen 입력에는 남지 않는다.
4. 미지원 기능은 추측해서 실행하지 않고 안정적인 진단으로 거부한다.
5. 동일 입력과 옵션은 결정적인 결과를 생성한다.

## 5. 구현 요구사항

1. TypeInterner, Unification, ConversionCost를 구현한다.
2. Buffer layout과 reserve helper를 구현한다.

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

- Literal·Nullable·함수·Generic 타입을 테스트한다.
- 성장·Move 원소·부분 초기화를 테스트한다.

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


## 9. 변경 절차

범위 변경은 NOVA-006의 변경 제안·영향 분석·결정 기록 절차를 따른다.


---

# Type Alias·타입 정규화 사양서

| 항목 | 값 |
|---|---|
| 문서 ID | NOVA-030 |
| 대상 | Nova 0.1 |
| 구현 시점 | 타입 검사 전 |
| 상태 | 확정 |
| 갱신일 | 2026-07-27 |

## 1. 목적

이 문서는 **Type Alias·타입 정규화 사양서**에 대한 언어 의미, Compiler 책임, 구현 경계, 오류 처리 및 완료 기준을 정의한다.

이 문서는 해당 구현 단계에 진입하기 전 동결해야 하는 기준이다.

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

1. int=int32, float=float32, double=float64다.
2. T?는 Option<T>, void는 Unit으로 정규화한다.
3. Class는 소유 Handle이며 구현 상속은 없다.

## 4. 확정 규칙

1. 양방향 타입 검사를 사용한다.
2. ErrorType으로 연쇄 오류를 억제한다.
3. 손실 없는 제한적 숫자 승격만 암묵 허용한다.

추가 불변 조건:

1. 평가 순서·Drop 순서·소유권 의미는 최적화로 변경하지 않는다.
2. 문자열 이름은 의미 분석 단계에서 안정적 ID로 변환한다.
3. Error Node·Error Type은 복구를 위해 허용하지만 Codegen 입력에는 남지 않는다.
4. 미지원 기능은 추측해서 실행하지 않고 안정적인 진단으로 거부한다.
5. 동일 입력과 옵션은 결정적인 결과를 생성한다.

## 5. 구현 요구사항

1. TypeInterner, Unification, ConversionCost를 구현한다.

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

- Literal·Nullable·함수·Generic 타입을 테스트한다.

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


## 9. 변경 절차

범위 변경은 NOVA-006의 변경 제안·영향 분석·결정 기록 절차를 따른다.


---

# 생성자·필드 초기화 사양서

| 항목 | 값 |
|---|---|
| 문서 ID | NOVA-031 |
| 대상 | Nova 0.1 |
| 구현 시점 | 생성자 구현 전 |
| 상태 | 확정 |
| 갱신일 | 2026-07-27 |

## 1. 목적

이 문서는 **생성자·필드 초기화 사양서**에 대한 언어 의미, Compiler 책임, 구현 경계, 오류 처리 및 완료 기준을 정의한다.

이 문서는 해당 구현 단계에 진입하기 전 동결해야 하는 기준이다.

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

1. int=int32, float=float32, double=float64다.
2. T?는 Option<T>, void는 Unit으로 정규화한다.
3. Class는 소유 Handle이며 구현 상속은 없다.

## 4. 확정 규칙

1. 생성자·필드 초기화의 입력·출력·불변 조건을 명시한다.
2. 생성자·필드 초기화의 MVP 범위와 후속 범위를 분리한다.
3. 생성자·필드 초기화의 결과는 컴파일 순서와 병렬 실행 순서에 독립적이어야 한다.

추가 불변 조건:

1. 평가 순서·Drop 순서·소유권 의미는 최적화로 변경하지 않는다.
2. 문자열 이름은 의미 분석 단계에서 안정적 ID로 변환한다.
3. Error Node·Error Type은 복구를 위해 허용하지만 Codegen 입력에는 남지 않는다.
4. 미지원 기능은 추측해서 실행하지 않고 안정적인 진단으로 거부한다.
5. 동일 입력과 옵션은 결정적인 결과를 생성한다.

## 5. 구현 요구사항

1. 생성자·필드 초기화 전용 데이터 모델과 Query 경계를 구현한다.
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

- 생성자·필드 초기화의 정상·오류·경계 사례를 테스트한다.
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


## 9. 변경 절차

범위 변경은 NOVA-006의 변경 제안·영향 분석·결정 기록 절차를 따른다.


---

# 상수 표현식·const 평가 사양서

| 항목 | 값 |
|---|---|
| 문서 ID | NOVA-032 |
| 대상 | Nova 0.1 |
| 구현 시점 | Const 구현 전 |
| 상태 | 확정 |
| 갱신일 | 2026-07-27 |

## 1. 목적

이 문서는 **상수 표현식·const 평가 사양서**에 대한 언어 의미, Compiler 책임, 구현 경계, 오류 처리 및 완료 기준을 정의한다.

이 문서는 해당 구현 단계에 진입하기 전 동결해야 하는 기준이다.

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

1. int=int32, float=float32, double=float64다.
2. T?는 Option<T>, void는 Unit으로 정규화한다.
3. Class는 소유 Handle이며 구현 상속은 없다.

## 4. 확정 규칙

1. 상수 표현식·const 평가의 입력·출력·불변 조건을 명시한다.
2. 상수 표현식·const 평가의 MVP 범위와 후속 범위를 분리한다.
3. 상수 표현식·const 평가의 결과는 컴파일 순서와 병렬 실행 순서에 독립적이어야 한다.

추가 불변 조건:

1. 평가 순서·Drop 순서·소유권 의미는 최적화로 변경하지 않는다.
2. 문자열 이름은 의미 분석 단계에서 안정적 ID로 변환한다.
3. Error Node·Error Type은 복구를 위해 허용하지만 Codegen 입력에는 남지 않는다.
4. 미지원 기능은 추측해서 실행하지 않고 안정적인 진단으로 거부한다.
5. 동일 입력과 옵션은 결정적인 결과를 생성한다.

## 5. 구현 요구사항

1. 상수 표현식·const 평가 전용 데이터 모델과 Query 경계를 구현한다.
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

- 상수 표현식·const 평가의 정상·오류·경계 사례를 테스트한다.
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


## 9. 변경 절차

범위 변경은 NOVA-006의 변경 제안·영향 분석·결정 기록 절차를 따른다.


---

# 타입 레이아웃·정렬·Niche 사양서

| 항목 | 값 |
|---|---|
| 문서 ID | NOVA-033 |
| 대상 | Nova 0.1 |
| 구현 시점 | Backend 전 |
| 상태 | 확정 |
| 갱신일 | 2026-07-27 |

## 1. 목적

이 문서는 **타입 레이아웃·정렬·Niche 사양서**에 대한 언어 의미, Compiler 책임, 구현 경계, 오류 처리 및 완료 기준을 정의한다.

이 문서는 해당 구현 단계에 진입하기 전 동결해야 하는 기준이다.

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

1. int=int32, float=float32, double=float64다.
2. T?는 Option<T>, void는 Unit으로 정규화한다.
3. Class는 소유 Handle이며 구현 상속은 없다.

## 4. 확정 규칙

1. 양방향 타입 검사를 사용한다.
2. ErrorType으로 연쇄 오류를 억제한다.
3. 손실 없는 제한적 숫자 승격만 암묵 허용한다.

추가 불변 조건:

1. 평가 순서·Drop 순서·소유권 의미는 최적화로 변경하지 않는다.
2. 문자열 이름은 의미 분석 단계에서 안정적 ID로 변환한다.
3. Error Node·Error Type은 복구를 위해 허용하지만 Codegen 입력에는 남지 않는다.
4. 미지원 기능은 추측해서 실행하지 않고 안정적인 진단으로 거부한다.
5. 동일 입력과 옵션은 결정적인 결과를 생성한다.

## 5. 구현 요구사항

1. TypeInterner, Unification, ConversionCost를 구현한다.

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

- Literal·Nullable·함수·Generic 타입을 테스트한다.

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


## 9. 변경 절차

범위 변경은 NOVA-006의 변경 제안·영향 분석·결정 기록 절차를 따른다.


---

# Class 객체 모델·정체성·배치 사양서

| 항목 | 값 |
|---|---|
| 문서 ID | NOVA-034 |
| 대상 | Nova 0.1 |
| 구현 시점 | Class 구현 전 |
| 상태 | 확정 |
| 갱신일 | 2026-07-27 |

## 1. 목적

이 문서는 **Class 객체 모델·정체성·배치 사양서**에 대한 언어 의미, Compiler 책임, 구현 경계, 오류 처리 및 완료 기준을 정의한다.

이 문서는 해당 구현 단계에 진입하기 전 동결해야 하는 기준이다.

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

1. int=int32, float=float32, double=float64다.
2. T?는 Option<T>, void는 Unit으로 정규화한다.
3. Class는 소유 Handle이며 구현 상속은 없다.

## 4. 확정 규칙

1. Class는 객체 정체성을 가진 소유 Handle이다.
2. Move는 Handle 소유권 이전이다.
3. 필드 Drop 후 저장 공간을 해제한다.

추가 불변 조건:

1. 평가 순서·Drop 순서·소유권 의미는 최적화로 변경하지 않는다.
2. 문자열 이름은 의미 분석 단계에서 안정적 ID로 변환한다.
3. Error Node·Error Type은 복구를 위해 허용하지만 Codegen 입력에는 남지 않는다.
4. 미지원 기능은 추측해서 실행하지 않고 안정적인 진단으로 거부한다.
5. 동일 입력과 옵션은 결정적인 결과를 생성한다.

## 5. 구현 요구사항

1. ClassLayout과 Handle ABI를 분리한다.

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

- 정체성·Move·Shared·Drop을 테스트한다.

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


## 9. 변경 절차

범위 변경은 NOVA-006의 변경 제안·영향 분석·결정 기록 절차를 따른다.


---

# 함수·메서드·호출 규약·Receiver 사양서

| 항목 | 값 |
|---|---|
| 문서 ID | NOVA-035 |
| 대상 | Nova 0.1 |
| 구현 시점 | 함수 구현 전 |
| 상태 | 확정 |
| 갱신일 | 2026-07-27 |

## 1. 목적

이 문서는 **함수·메서드·호출 규약·Receiver 사양서**에 대한 언어 의미, Compiler 책임, 구현 경계, 오류 처리 및 완료 기준을 정의한다.

이 문서는 해당 구현 단계에 진입하기 전 동결해야 하는 기준이다.

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

1. 기본 인수 모드는 Read다.
2. change는 독점 수정 빌림, take는 소유권 이전이다.
3. 인수는 소스 순서로 평가한다.
4. 할당은 문장이다.

## 4. 확정 규칙

1. Signature에는 Receiver·Parameter Mode·Return Ownership·Effect가 포함된다.
2. 반환 타입만으로 Overload하지 않는다.
3. 기본 인수는 호출자에서 평가한다.

추가 불변 조건:

1. 평가 순서·Drop 순서·소유권 의미는 최적화로 변경하지 않는다.
2. 문자열 이름은 의미 분석 단계에서 안정적 ID로 변환한다.
3. Error Node·Error Type은 복구를 위해 허용하지만 Codegen 입력에는 남지 않는다.
4. 미지원 기능은 추측해서 실행하지 않고 안정적인 진단으로 거부한다.
5. 동일 입력과 옵션은 결정적인 결과를 생성한다.

## 5. 구현 요구사항

1. FunctionId, Signature, OverloadSet, Call lowering을 구현한다.

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

- Read·change·take와 이름 인수를 테스트한다.

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


## 9. 변경 절차

범위 변경은 NOVA-006의 변경 제안·영향 분석·결정 기록 절차를 따른다.


---

# 위치·이름·기본 인수 사양서

| 항목 | 값 |
|---|---|
| 문서 ID | NOVA-036 |
| 대상 | Nova 0.1 |
| 구현 시점 | 함수 구현 전 |
| 상태 | 확정 |
| 갱신일 | 2026-07-27 |

## 1. 목적

이 문서는 **위치·이름·기본 인수 사양서**에 대한 언어 의미, Compiler 책임, 구현 경계, 오류 처리 및 완료 기준을 정의한다.

이 문서는 해당 구현 단계에 진입하기 전 동결해야 하는 기준이다.

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

1. 기본 인수 모드는 Read다.
2. change는 독점 수정 빌림, take는 소유권 이전이다.
3. 인수는 소스 순서로 평가한다.
4. 할당은 문장이다.

## 4. 확정 규칙

1. 타입·값·모듈 이름 공간을 분리한다.
2. 가장 가까운 Scope를 우선한다.
3. 모호한 이름은 임의 선택하지 않는다.

추가 불변 조건:

1. 평가 순서·Drop 순서·소유권 의미는 최적화로 변경하지 않는다.
2. 문자열 이름은 의미 분석 단계에서 안정적 ID로 변환한다.
3. Error Node·Error Type은 복구를 위해 허용하지만 Codegen 입력에는 남지 않는다.
4. 미지원 기능은 추측해서 실행하지 않고 안정적인 진단으로 거부한다.
5. 동일 입력과 옵션은 결정적인 결과를 생성한다.

## 5. 구현 요구사항

1. ScopeTree, DefinitionRegistry, ResolutionMap을 구현한다.

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

- Shadowing·중복·접근 제한을 테스트한다.

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


## 9. 변경 절차

범위 변경은 NOVA-006의 변경 제안·영향 분석·결정 기록 절차를 따른다.


---

# Overload 해석·변환 비용 사양서

| 항목 | 값 |
|---|---|
| 문서 ID | NOVA-037 |
| 대상 | Nova 0.1 |
| 구현 시점 | 호출 해석 전 |
| 상태 | 확정 |
| 갱신일 | 2026-07-27 |

## 1. 목적

이 문서는 **Overload 해석·변환 비용 사양서**에 대한 언어 의미, Compiler 책임, 구현 경계, 오류 처리 및 완료 기준을 정의한다.

이 문서는 해당 구현 단계에 진입하기 전 동결해야 하는 기준이다.

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

1. 기본 인수 모드는 Read다.
2. change는 독점 수정 빌림, take는 소유권 이전이다.
3. 인수는 소스 순서로 평가한다.
4. 할당은 문장이다.

## 4. 확정 규칙

1. Overload 해석·변환 비용의 입력·출력·불변 조건을 명시한다.
2. Overload 해석·변환 비용의 MVP 범위와 후속 범위를 분리한다.
3. Overload 해석·변환 비용의 결과는 컴파일 순서와 병렬 실행 순서에 독립적이어야 한다.

추가 불변 조건:

1. 평가 순서·Drop 순서·소유권 의미는 최적화로 변경하지 않는다.
2. 문자열 이름은 의미 분석 단계에서 안정적 ID로 변환한다.
3. Error Node·Error Type은 복구를 위해 허용하지만 Codegen 입력에는 남지 않는다.
4. 미지원 기능은 추측해서 실행하지 않고 안정적인 진단으로 거부한다.
5. 동일 입력과 옵션은 결정적인 결과를 생성한다.

## 5. 구현 요구사항

1. Overload 해석·변환 비용 전용 데이터 모델과 Query 경계를 구현한다.
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

- Overload 해석·변환 비용의 정상·오류·경계 사례를 테스트한다.
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


## 9. 변경 절차

범위 변경은 NOVA-006의 변경 제안·영향 분석·결정 기록 절차를 따른다.


---

# 함수 타입·Lambda·Closure 사양서

| 항목 | 값 |
|---|---|
| 문서 ID | NOVA-038 |
| 대상 | Nova 0.1 |
| 구현 시점 | Lambda 구현 전 |
| 상태 | 확정 |
| 갱신일 | 2026-07-27 |

## 1. 목적

이 문서는 **함수 타입·Lambda·Closure 사양서**에 대한 언어 의미, Compiler 책임, 구현 경계, 오류 처리 및 완료 기준을 정의한다.

이 문서는 해당 구현 단계에 진입하기 전 동결해야 하는 기준이다.

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

1. 기본 인수 모드는 Read다.
2. change는 독점 수정 빌림, take는 소유권 이전이다.
3. 인수는 소스 순서로 평가한다.
4. 할당은 문장이다.

## 4. 확정 규칙

1. 양방향 타입 검사를 사용한다.
2. ErrorType으로 연쇄 오류를 억제한다.
3. 손실 없는 제한적 숫자 승격만 암묵 허용한다.
4. Signature에는 Receiver·Parameter Mode·Return Ownership·Effect가 포함된다.
5. 반환 타입만으로 Overload하지 않는다.
6. 기본 인수는 호출자에서 평가한다.
7. Capture는 Read·change·take로 분류한다.
8. Capture 없는 Lambda는 함수 포인터로 변환할 수 있다.

추가 불변 조건:

1. 평가 순서·Drop 순서·소유권 의미는 최적화로 변경하지 않는다.
2. 문자열 이름은 의미 분석 단계에서 안정적 ID로 변환한다.
3. Error Node·Error Type은 복구를 위해 허용하지만 Codegen 입력에는 남지 않는다.
4. 미지원 기능은 추측해서 실행하지 않고 안정적인 진단으로 거부한다.
5. 동일 입력과 옵션은 결정적인 결과를 생성한다.

## 5. 구현 요구사항

1. TypeInterner, Unification, ConversionCost를 구현한다.
2. FunctionId, Signature, OverloadSet, Call lowering을 구현한다.
3. Capture discovery와 Closure Drop Glue를 구현한다.

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

- Literal·Nullable·함수·Generic 타입을 테스트한다.
- Read·change·take와 이름 인수를 테스트한다.
- 반환 Closure·Move Capture·중첩 Lambda를 테스트한다.

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


## 9. 변경 절차

범위 변경은 NOVA-006의 변경 제안·영향 분석·결정 기록 절차를 따른다.


---

# Closure Capture·호출 Receiver 사양서

| 항목 | 값 |
|---|---|
| 문서 ID | NOVA-039 |
| 대상 | Nova 0.1 |
| 구현 시점 | Lambda 구현 전 |
| 상태 | 확정 |
| 갱신일 | 2026-07-27 |

## 1. 목적

이 문서는 **Closure Capture·호출 Receiver 사양서**에 대한 언어 의미, Compiler 책임, 구현 경계, 오류 처리 및 완료 기준을 정의한다.

이 문서는 해당 구현 단계에 진입하기 전 동결해야 하는 기준이다.

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

1. 기본 인수 모드는 Read다.
2. change는 독점 수정 빌림, take는 소유권 이전이다.
3. 인수는 소스 순서로 평가한다.
4. 할당은 문장이다.

## 4. 확정 규칙

1. Capture는 Read·change·take로 분류한다.
2. Capture 없는 Lambda는 함수 포인터로 변환할 수 있다.

추가 불변 조건:

1. 평가 순서·Drop 순서·소유권 의미는 최적화로 변경하지 않는다.
2. 문자열 이름은 의미 분석 단계에서 안정적 ID로 변환한다.
3. Error Node·Error Type은 복구를 위해 허용하지만 Codegen 입력에는 남지 않는다.
4. 미지원 기능은 추측해서 실행하지 않고 안정적인 진단으로 거부한다.
5. 동일 입력과 옵션은 결정적인 결과를 생성한다.

## 5. 구현 요구사항

1. Capture discovery와 Closure Drop Glue를 구현한다.

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

- 반환 Closure·Move Capture·중첩 Lambda를 테스트한다.

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


## 9. 변경 절차

범위 변경은 NOVA-006의 변경 제안·영향 분석·결정 기록 절차를 따른다.


---

# main·프로그램 진입점 사양서

| 항목 | 값 |
|---|---|
| 문서 ID | NOVA-040 |
| 대상 | Nova 0.1 |
| 구현 시점 | 실행 파일 전 |
| 상태 | 확정 |
| 갱신일 | 2026-07-27 |

## 1. 목적

이 문서는 **main·프로그램 진입점 사양서**에 대한 언어 의미, Compiler 책임, 구현 경계, 오류 처리 및 완료 기준을 정의한다.

이 문서는 해당 구현 단계에 진입하기 전 동결해야 하는 기준이다.

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

1. 기본 인수 모드는 Read다.
2. change는 독점 수정 빌림, take는 소유권 이전이다.
3. 인수는 소스 순서로 평가한다.
4. 할당은 문장이다.

## 4. 확정 규칙

1. main·프로그램 진입점의 입력·출력·불변 조건을 명시한다.
2. main·프로그램 진입점의 MVP 범위와 후속 범위를 분리한다.
3. main·프로그램 진입점의 결과는 컴파일 순서와 병렬 실행 순서에 독립적이어야 한다.

추가 불변 조건:

1. 평가 순서·Drop 순서·소유권 의미는 최적화로 변경하지 않는다.
2. 문자열 이름은 의미 분석 단계에서 안정적 ID로 변환한다.
3. Error Node·Error Type은 복구를 위해 허용하지만 Codegen 입력에는 남지 않는다.
4. 미지원 기능은 추측해서 실행하지 않고 안정적인 진단으로 거부한다.
5. 동일 입력과 옵션은 결정적인 결과를 생성한다.

## 5. 구현 요구사항

1. main·프로그램 진입점 전용 데이터 모델과 Query 경계를 구현한다.
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

- main·프로그램 진입점의 정상·오류·경계 사례를 테스트한다.
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


## 9. 변경 절차

범위 변경은 NOVA-006의 변경 제안·영향 분석·결정 기록 절차를 따른다.


---

# Effect·pure·noPanic 사양서

| 항목 | 값 |
|---|---|
| 문서 ID | NOVA-041 |
| 대상 | Nova 0.1 |
| 구현 시점 | Effect 구현 전 |
| 상태 | 확정 |
| 갱신일 | 2026-07-27 |

## 1. 목적

이 문서는 **Effect·pure·noPanic 사양서**에 대한 언어 의미, Compiler 책임, 구현 경계, 오류 처리 및 완료 기준을 정의한다.

이 문서는 해당 구현 단계에 진입하기 전 동결해야 하는 기준이다.

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

1. 기본 인수 모드는 Read다.
2. change는 독점 수정 빌림, take는 소유권 이전이다.
3. 인수는 소스 순서로 평가한다.
4. 할당은 문장이다.

## 4. 확정 규칙

1. MVP Panic은 Abort다.
2. FFI 경계를 넘지 않는다.
3. 메시지와 Source 위치를 기록한다.

추가 불변 조건:

1. 평가 순서·Drop 순서·소유권 의미는 최적화로 변경하지 않는다.
2. 문자열 이름은 의미 분석 단계에서 안정적 ID로 변환한다.
3. Error Node·Error Type은 복구를 위해 허용하지만 Codegen 입력에는 남지 않는다.
4. 미지원 기능은 추측해서 실행하지 않고 안정적인 진단으로 거부한다.
5. 동일 입력과 옵션은 결정적인 결과를 생성한다.

## 5. 구현 요구사항

1. panic runtime과 stack trace hook를 구현한다.

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

- bounds·overflow·명시적 panic을 테스트한다.

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


## 9. 변경 절차

범위 변경은 NOVA-006의 변경 제안·영향 분석·결정 기록 절차를 따른다.


---

# Native Calling Convention 사양서

| 항목 | 값 |
|---|---|
| 문서 ID | NOVA-042 |
| 대상 | Nova 0.1 |
| 구현 시점 | Backend 전 |
| 상태 | 확정 |
| 갱신일 | 2026-07-27 |

## 1. 목적

이 문서는 **Native Calling Convention 사양서**에 대한 언어 의미, Compiler 책임, 구현 경계, 오류 처리 및 완료 기준을 정의한다.

이 문서는 해당 구현 단계에 진입하기 전 동결해야 하는 기준이다.

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

1. 기본 인수 모드는 Read다.
2. change는 독점 수정 빌림, take는 소유권 이전이다.
3. 인수는 소스 순서로 평가한다.
4. 할당은 문장이다.

## 4. 확정 규칙

1. Native Calling Convention의 입력·출력·불변 조건을 명시한다.
2. Native Calling Convention의 MVP 범위와 후속 범위를 분리한다.
3. Native Calling Convention의 결과는 컴파일 순서와 병렬 실행 순서에 독립적이어야 한다.

추가 불변 조건:

1. 평가 순서·Drop 순서·소유권 의미는 최적화로 변경하지 않는다.
2. 문자열 이름은 의미 분석 단계에서 안정적 ID로 변환한다.
3. Error Node·Error Type은 복구를 위해 허용하지만 Codegen 입력에는 남지 않는다.
4. 미지원 기능은 추측해서 실행하지 않고 안정적인 진단으로 거부한다.
5. 동일 입력과 옵션은 결정적인 결과를 생성한다.

## 5. 구현 요구사항

1. Native Calling Convention 전용 데이터 모델과 Query 경계를 구현한다.
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

- Native Calling Convention의 정상·오류·경계 사례를 테스트한다.
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


## 9. 변경 절차

범위 변경은 NOVA-006의 변경 제안·영향 분석·결정 기록 절차를 따른다.


---

# if·while·for·loop 사양서

| 항목 | 값 |
|---|---|
| 문서 ID | NOVA-043 |
| 대상 | Nova 0.1 |
| 구현 시점 | Parser·MIR 전 |
| 상태 | 확정 |
| 갱신일 | 2026-07-27 |

## 1. 목적

이 문서는 **if·while·for·loop 사양서**에 대한 언어 의미, Compiler 책임, 구현 경계, 오류 처리 및 완료 기준을 정의한다.

이 문서는 해당 구현 단계에 진입하기 전 동결해야 하는 기준이다.

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

1. 기본 인수 모드는 Read다.
2. change는 독점 수정 빌림, take는 소유권 이전이다.
3. 인수는 소스 순서로 평가한다.
4. 할당은 문장이다.

## 4. 확정 규칙

1. if·while·for·loop의 입력·출력·불변 조건을 명시한다.
2. if·while·for·loop의 MVP 범위와 후속 범위를 분리한다.
3. if·while·for·loop의 결과는 컴파일 순서와 병렬 실행 순서에 독립적이어야 한다.

추가 불변 조건:

1. 평가 순서·Drop 순서·소유권 의미는 최적화로 변경하지 않는다.
2. 문자열 이름은 의미 분석 단계에서 안정적 ID로 변환한다.
3. Error Node·Error Type은 복구를 위해 허용하지만 Codegen 입력에는 남지 않는다.
4. 미지원 기능은 추측해서 실행하지 않고 안정적인 진단으로 거부한다.
5. 동일 입력과 옵션은 결정적인 결과를 생성한다.

## 5. 구현 요구사항

1. if·while·for·loop 전용 데이터 모델과 Query 경계를 구현한다.
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

- if·while·for·loop의 정상·오류·경계 사례를 테스트한다.
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


## 9. 변경 절차

범위 변경은 NOVA-006의 변경 제안·영향 분석·결정 기록 절차를 따른다.


---

# break·continue·return 사양서

| 항목 | 값 |
|---|---|
| 문서 ID | NOVA-044 |
| 대상 | Nova 0.1 |
| 구현 시점 | MIR 전 |
| 상태 | 확정 |
| 갱신일 | 2026-07-27 |

## 1. 목적

이 문서는 **break·continue·return 사양서**에 대한 언어 의미, Compiler 책임, 구현 경계, 오류 처리 및 완료 기준을 정의한다.

이 문서는 해당 구현 단계에 진입하기 전 동결해야 하는 기준이다.

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

1. 기본 인수 모드는 Read다.
2. change는 독점 수정 빌림, take는 소유권 이전이다.
3. 인수는 소스 순서로 평가한다.
4. 할당은 문장이다.

## 4. 확정 규칙

1. break·continue·return의 입력·출력·불변 조건을 명시한다.
2. break·continue·return의 MVP 범위와 후속 범위를 분리한다.
3. break·continue·return의 결과는 컴파일 순서와 병렬 실행 순서에 독립적이어야 한다.

추가 불변 조건:

1. 평가 순서·Drop 순서·소유권 의미는 최적화로 변경하지 않는다.
2. 문자열 이름은 의미 분석 단계에서 안정적 ID로 변환한다.
3. Error Node·Error Type은 복구를 위해 허용하지만 Codegen 입력에는 남지 않는다.
4. 미지원 기능은 추측해서 실행하지 않고 안정적인 진단으로 거부한다.
5. 동일 입력과 옵션은 결정적인 결과를 생성한다.

## 5. 구현 요구사항

1. break·continue·return 전용 데이터 모델과 Query 경계를 구현한다.
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

- break·continue·return의 정상·오류·경계 사례를 테스트한다.
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


## 9. 변경 절차

범위 변경은 NOVA-006의 변경 제안·영향 분석·결정 기록 절차를 따른다.


---

# Range until·through 사양서

| 항목 | 값 |
|---|---|
| 문서 ID | NOVA-045 |
| 대상 | Nova 0.1 |
| 구현 시점 | Parser 전 |
| 상태 | 확정 |
| 갱신일 | 2026-07-27 |

## 1. 목적

이 문서는 **Range until·through 사양서**에 대한 언어 의미, Compiler 책임, 구현 경계, 오류 처리 및 완료 기준을 정의한다.

이 문서는 해당 구현 단계에 진입하기 전 동결해야 하는 기준이다.

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

1. 기본 인수 모드는 Read다.
2. change는 독점 수정 빌림, take는 소유권 이전이다.
3. 인수는 소스 순서로 평가한다.
4. 할당은 문장이다.

## 4. 확정 규칙

1. Range until·through의 입력·출력·불변 조건을 명시한다.
2. Range until·through의 MVP 범위와 후속 범위를 분리한다.
3. Range until·through의 결과는 컴파일 순서와 병렬 실행 순서에 독립적이어야 한다.

추가 불변 조건:

1. 평가 순서·Drop 순서·소유권 의미는 최적화로 변경하지 않는다.
2. 문자열 이름은 의미 분석 단계에서 안정적 ID로 변환한다.
3. Error Node·Error Type은 복구를 위해 허용하지만 Codegen 입력에는 남지 않는다.
4. 미지원 기능은 추측해서 실행하지 않고 안정적인 진단으로 거부한다.
5. 동일 입력과 옵션은 결정적인 결과를 생성한다.

## 5. 구현 요구사항

1. Range until·through 전용 데이터 모델과 Query 경계를 구현한다.
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

- Range until·through의 정상·오류·경계 사례를 테스트한다.
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


## 9. 변경 절차

범위 변경은 NOVA-006의 변경 제안·영향 분석·결정 기록 절차를 따른다.


---

# Pattern 문법·Binding 사양서

| 항목 | 값 |
|---|---|
| 문서 ID | NOVA-046 |
| 대상 | Nova 0.1 |
| 구현 시점 | Match 구현 전 |
| 상태 | 확정 |
| 갱신일 | 2026-07-27 |

## 1. 목적

이 문서는 **Pattern 문법·Binding 사양서**에 대한 언어 의미, Compiler 책임, 구현 경계, 오류 처리 및 완료 기준을 정의한다.

이 문서는 해당 구현 단계에 진입하기 전 동결해야 하는 기준이다.

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

1. 기본 인수 모드는 Read다.
2. change는 독점 수정 빌림, take는 소유권 이전이다.
3. 인수는 소스 순서로 평가한다.
4. 할당은 문장이다.

## 4. 확정 규칙

1. Pattern 문법·Binding의 입력·출력·불변 조건을 명시한다.
2. Pattern 문법·Binding의 MVP 범위와 후속 범위를 분리한다.
3. Pattern 문법·Binding의 결과는 컴파일 순서와 병렬 실행 순서에 독립적이어야 한다.

추가 불변 조건:

1. 평가 순서·Drop 순서·소유권 의미는 최적화로 변경하지 않는다.
2. 문자열 이름은 의미 분석 단계에서 안정적 ID로 변환한다.
3. Error Node·Error Type은 복구를 위해 허용하지만 Codegen 입력에는 남지 않는다.
4. 미지원 기능은 추측해서 실행하지 않고 안정적인 진단으로 거부한다.
5. 동일 입력과 옵션은 결정적인 결과를 생성한다.

## 5. 구현 요구사항

1. Pattern 문법·Binding 전용 데이터 모델과 Query 경계를 구현한다.
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

- Pattern 문법·Binding의 정상·오류·경계 사례를 테스트한다.
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


## 9. 변경 절차

범위 변경은 NOVA-006의 변경 제안·영향 분석·결정 기록 절차를 따른다.


---

# Match 완전성·도달 불가 Arm 분석서

| 항목 | 값 |
|---|---|
| 문서 ID | NOVA-047 |
| 대상 | Nova 0.1 |
| 구현 시점 | Match 구현 전 |
| 상태 | 확정 |
| 갱신일 | 2026-07-27 |

## 1. 목적

이 문서는 **Match 완전성·도달 불가 Arm 분석서**에 대한 언어 의미, Compiler 책임, 구현 경계, 오류 처리 및 완료 기준을 정의한다.

이 문서는 해당 구현 단계에 진입하기 전 동결해야 하는 기준이다.

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

1. 기본 인수 모드는 Read다.
2. change는 독점 수정 빌림, take는 소유권 이전이다.
3. 인수는 소스 순서로 평가한다.
4. 할당은 문장이다.

## 4. 확정 규칙

1. Match는 완전성을 검사한다.
2. 완전히 가려진 Arm은 도달 불가다.
3. Arm 순서를 보존한다.

추가 불변 조건:

1. 평가 순서·Drop 순서·소유권 의미는 최적화로 변경하지 않는다.
2. 문자열 이름은 의미 분석 단계에서 안정적 ID로 변환한다.
3. Error Node·Error Type은 복구를 위해 허용하지만 Codegen 입력에는 남지 않는다.
4. 미지원 기능은 추측해서 실행하지 않고 안정적인 진단으로 거부한다.
5. 동일 입력과 옵션은 결정적인 결과를 생성한다.

## 5. 구현 요구사항

1. Pattern matrix 또는 Decision Tree를 MIR로 내린다.

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

- Enum·Bool·Guard·중첩 Pattern을 테스트한다.

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


## 9. 변경 절차

범위 변경은 NOVA-006의 변경 제안·영향 분석·결정 기록 절차를 따른다.


---

# Match Lowering·Decision Tree 사양서

| 항목 | 값 |
|---|---|
| 문서 ID | NOVA-048 |
| 대상 | Nova 0.1 |
| 구현 시점 | MIR 전 |
| 상태 | 확정 |
| 갱신일 | 2026-07-27 |

## 1. 목적

이 문서는 **Match Lowering·Decision Tree 사양서**에 대한 언어 의미, Compiler 책임, 구현 경계, 오류 처리 및 완료 기준을 정의한다.

이 문서는 해당 구현 단계에 진입하기 전 동결해야 하는 기준이다.

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

1. 기본 인수 모드는 Read다.
2. change는 독점 수정 빌림, take는 소유권 이전이다.
3. 인수는 소스 순서로 평가한다.
4. 할당은 문장이다.

## 4. 확정 규칙

1. Match는 완전성을 검사한다.
2. 완전히 가려진 Arm은 도달 불가다.
3. Arm 순서를 보존한다.

추가 불변 조건:

1. 평가 순서·Drop 순서·소유권 의미는 최적화로 변경하지 않는다.
2. 문자열 이름은 의미 분석 단계에서 안정적 ID로 변환한다.
3. Error Node·Error Type은 복구를 위해 허용하지만 Codegen 입력에는 남지 않는다.
4. 미지원 기능은 추측해서 실행하지 않고 안정적인 진단으로 거부한다.
5. 동일 입력과 옵션은 결정적인 결과를 생성한다.

## 5. 구현 요구사항

1. Pattern matrix 또는 Decision Tree를 MIR로 내린다.

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

- Enum·Bool·Guard·중첩 Pattern을 테스트한다.

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


## 9. 변경 절차

범위 변경은 NOVA-006의 변경 제안·영향 분석·결정 기록 절차를 따른다.


---

# try·Result 전파 Lowering 사양서

| 항목 | 값 |
|---|---|
| 문서 ID | NOVA-049 |
| 대상 | Nova 0.1 |
| 구현 시점 | MIR 전 |
| 상태 | 확정 |
| 갱신일 | 2026-07-27 |

## 1. 목적

이 문서는 **try·Result 전파 Lowering 사양서**에 대한 언어 의미, Compiler 책임, 구현 경계, 오류 처리 및 완료 기준을 정의한다.

이 문서는 해당 구현 단계에 진입하기 전 동결해야 하는 기준이다.

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

1. 기본 인수 모드는 Read다.
2. change는 독점 수정 빌림, take는 소유권 이전이다.
3. 인수는 소스 순서로 평가한다.
4. 할당은 문장이다.

## 4. 확정 규칙

1. Result는 Success·Error다.
2. try는 Error를 조기 반환한다.
3. Success(())로 Unit 성공을 표현한다.

추가 불변 조건:

1. 평가 순서·Drop 순서·소유권 의미는 최적화로 변경하지 않는다.
2. 문자열 이름은 의미 분석 단계에서 안정적 ID로 변환한다.
3. Error Node·Error Type은 복구를 위해 허용하지만 Codegen 입력에는 남지 않는다.
4. 미지원 기능은 추측해서 실행하지 않고 안정적인 진단으로 거부한다.
5. 동일 입력과 옵션은 결정적인 결과를 생성한다.

## 5. 구현 요구사항

1. Result API와 try lowering을 구현한다.

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

- 중첩 try·cleanup·payload move를 테스트한다.

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


## 9. 변경 절차

범위 변경은 NOVA-006의 변경 제안·영향 분석·결정 기록 절차를 따른다.


---

# using Lowering 사양서

| 항목 | 값 |
|---|---|
| 문서 ID | NOVA-050 |
| 대상 | Nova 0.1 |
| 구현 시점 | MIR 전 |
| 상태 | 확정 |
| 갱신일 | 2026-07-27 |

## 1. 목적

이 문서는 **using Lowering 사양서**에 대한 언어 의미, Compiler 책임, 구현 경계, 오류 처리 및 완료 기준을 정의한다.

이 문서는 해당 구현 단계에 진입하기 전 동결해야 하는 기준이다.

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

1. 기본 인수 모드는 Read다.
2. change는 독점 수정 빌림, take는 소유권 이전이다.
3. 인수는 소스 순서로 평가한다.
4. 할당은 문장이다.

## 4. 확정 규칙

1. using Lowering의 입력·출력·불변 조건을 명시한다.
2. using Lowering의 MVP 범위와 후속 범위를 분리한다.
3. using Lowering의 결과는 컴파일 순서와 병렬 실행 순서에 독립적이어야 한다.

추가 불변 조건:

1. 평가 순서·Drop 순서·소유권 의미는 최적화로 변경하지 않는다.
2. 문자열 이름은 의미 분석 단계에서 안정적 ID로 변환한다.
3. Error Node·Error Type은 복구를 위해 허용하지만 Codegen 입력에는 남지 않는다.
4. 미지원 기능은 추측해서 실행하지 않고 안정적인 진단으로 거부한다.
5. 동일 입력과 옵션은 결정적인 결과를 생성한다.

## 5. 구현 요구사항

1. using Lowering 전용 데이터 모델과 Query 경계를 구현한다.
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

- using Lowering의 정상·오류·경계 사례를 테스트한다.
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


## 9. 변경 절차

범위 변경은 NOVA-006의 변경 제안·영향 분석·결정 기록 절차를 따른다.


---

# 메모리·소유권 모델 사양서

| 항목 | 값 |
|---|---|
| 문서 ID | NOVA-051 |
| 대상 | Nova 0.1 |
| 구현 시점 | 타입 검사 전 |
| 상태 | 확정 |
| 갱신일 | 2026-07-27 |

## 1. 목적

이 문서는 **메모리·소유권 모델 사양서**에 대한 언어 의미, Compiler 책임, 구현 경계, 오류 처리 및 완료 기준을 정의한다.

이 문서는 해당 구현 단계에 진입하기 전 동결해야 하는 기준이다.

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

1. Move 값은 명시적 take로 이동한다.
2. 사용자 부분 이동은 금지한다.
3. Drop은 초기화된 경로에서 정확히 한 번 실행한다.
4. Panic MVP는 abort다.

## 4. 확정 규칙

1. Move 타입은 하나의 Owner를 가진다.
2. Read는 소유권을 소비하지 않는다.
3. take 이후 원본은 재초기화 전까지 사용할 수 없다.

추가 불변 조건:

1. 평가 순서·Drop 순서·소유권 의미는 최적화로 변경하지 않는다.
2. 문자열 이름은 의미 분석 단계에서 안정적 ID로 변환한다.
3. Error Node·Error Type은 복구를 위해 허용하지만 Codegen 입력에는 남지 않는다.
4. 미지원 기능은 추측해서 실행하지 않고 안정적인 진단으로 거부한다.
5. 동일 입력과 옵션은 결정적인 결과를 생성한다.

## 5. 구현 요구사항

1. OwnershipAction과 Move Origin을 기록한다.

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

- 인수·반환·분기·Closure 이동을 테스트한다.

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


## 9. 변경 절차

범위 변경은 NOVA-006의 변경 제안·영향 분석·결정 기록 절차를 따른다.


---

# 초기화·이동 상태 분석 사양서

| 항목 | 값 |
|---|---|
| 문서 ID | NOVA-052 |
| 대상 | Nova 0.1 |
| 구현 시점 | MIR 분석 전 |
| 상태 | 확정 |
| 갱신일 | 2026-07-27 |

## 1. 목적

이 문서는 **초기화·이동 상태 분석 사양서**에 대한 언어 의미, Compiler 책임, 구현 경계, 오류 처리 및 완료 기준을 정의한다.

이 문서는 해당 구현 단계에 진입하기 전 동결해야 하는 기준이다.

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

1. Move 값은 명시적 take로 이동한다.
2. 사용자 부분 이동은 금지한다.
3. Drop은 초기화된 경로에서 정확히 한 번 실행한다.
4. Panic MVP는 abort다.

## 4. 확정 규칙

1. 초기화·이동 상태 분석의 입력·출력·불변 조건을 명시한다.
2. 초기화·이동 상태 분석의 MVP 범위와 후속 범위를 분리한다.
3. 초기화·이동 상태 분석의 결과는 컴파일 순서와 병렬 실행 순서에 독립적이어야 한다.

추가 불변 조건:

1. 평가 순서·Drop 순서·소유권 의미는 최적화로 변경하지 않는다.
2. 문자열 이름은 의미 분석 단계에서 안정적 ID로 변환한다.
3. Error Node·Error Type은 복구를 위해 허용하지만 Codegen 입력에는 남지 않는다.
4. 미지원 기능은 추측해서 실행하지 않고 안정적인 진단으로 거부한다.
5. 동일 입력과 옵션은 결정적인 결과를 생성한다.

## 5. 구현 요구사항

1. 초기화·이동 상태 분석 전용 데이터 모델과 Query 경계를 구현한다.
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

- 초기화·이동 상태 분석의 정상·오류·경계 사례를 테스트한다.
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


## 9. 변경 절차

범위 변경은 NOVA-006의 변경 제안·영향 분석·결정 기록 절차를 따른다.


---

# 부분 이동 정책 사양서

| 항목 | 값 |
|---|---|
| 문서 ID | NOVA-053 |
| 대상 | Nova 0.1 |
| 구현 시점 | MIR 분석 전 |
| 상태 | 확정 |
| 갱신일 | 2026-07-27 |

## 1. 목적

이 문서는 **부분 이동 정책 사양서**에 대한 언어 의미, Compiler 책임, 구현 경계, 오류 처리 및 완료 기준을 정의한다.

이 문서는 해당 구현 단계에 진입하기 전 동결해야 하는 기준이다.

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

1. Move 값은 명시적 take로 이동한다.
2. 사용자 부분 이동은 금지한다.
3. Drop은 초기화된 경로에서 정확히 한 번 실행한다.
4. Panic MVP는 abort다.

## 4. 확정 규칙

1. 부분 이동 정책의 입력·출력·불변 조건을 명시한다.
2. 부분 이동 정책의 MVP 범위와 후속 범위를 분리한다.
3. 부분 이동 정책의 결과는 컴파일 순서와 병렬 실행 순서에 독립적이어야 한다.

추가 불변 조건:

1. 평가 순서·Drop 순서·소유권 의미는 최적화로 변경하지 않는다.
2. 문자열 이름은 의미 분석 단계에서 안정적 ID로 변환한다.
3. Error Node·Error Type은 복구를 위해 허용하지만 Codegen 입력에는 남지 않는다.
4. 미지원 기능은 추측해서 실행하지 않고 안정적인 진단으로 거부한다.
5. 동일 입력과 옵션은 결정적인 결과를 생성한다.

## 5. 구현 요구사항

1. 부분 이동 정책 전용 데이터 모델과 Query 경계를 구현한다.
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

- 부분 이동 정책의 정상·오류·경계 사례를 테스트한다.
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


## 9. 변경 절차

범위 변경은 NOVA-006의 변경 제안·영향 분석·결정 기록 절차를 따른다.


---

# 빌림·수명·Place 충돌 사양서

| 항목 | 값 |
|---|---|
| 문서 ID | NOVA-054 |
| 대상 | Nova 0.1 |
| 구현 시점 | Borrow Checker 전 |
| 상태 | 확정 |
| 갱신일 | 2026-07-27 |

## 1. 목적

이 문서는 **빌림·수명·Place 충돌 사양서**에 대한 언어 의미, Compiler 책임, 구현 경계, 오류 처리 및 완료 기준을 정의한다.

이 문서는 해당 구현 단계에 진입하기 전 동결해야 하는 기준이다.

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

1. Move 값은 명시적 take로 이동한다.
2. 사용자 부분 이동은 금지한다.
3. Drop은 초기화된 경로에서 정확히 한 번 실행한다.
4. Panic MVP는 abort다.

## 4. 확정 규칙

1. Read Loan은 공유 가능하고 Change Loan은 독점이다.
2. Loan은 마지막 실제 사용까지 유지한다.
3. Disjoint를 증명하지 못하면 충돌로 본다.

추가 불변 조건:

1. 평가 순서·Drop 순서·소유권 의미는 최적화로 변경하지 않는다.
2. 문자열 이름은 의미 분석 단계에서 안정적 ID로 변환한다.
3. Error Node·Error Type은 복구를 위해 허용하지만 Codegen 입력에는 남지 않는다.
4. 미지원 기능은 추측해서 실행하지 않고 안정적인 진단으로 거부한다.
5. 동일 입력과 옵션은 결정적인 결과를 생성한다.

## 5. 구현 요구사항

1. LoanSet, RegionInference, PlaceRelation을 구현한다.

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

- NLL·재빌림·필드 분할·View 반환을 테스트한다.

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


## 9. 변경 절차

범위 변경은 NOVA-006의 변경 제안·영향 분석·결정 기록 절차를 따른다.


---

# Drop 정교화·Drop Flag 사양서

| 항목 | 값 |
|---|---|
| 문서 ID | NOVA-055 |
| 대상 | Nova 0.1 |
| 구현 시점 | Drop 구현 전 |
| 상태 | 확정 |
| 갱신일 | 2026-07-27 |

## 1. 목적

이 문서는 **Drop 정교화·Drop Flag 사양서**에 대한 언어 의미, Compiler 책임, 구현 경계, 오류 처리 및 완료 기준을 정의한다.

이 문서는 해당 구현 단계에 진입하기 전 동결해야 하는 기준이다.

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

1. Move 값은 명시적 take로 이동한다.
2. 사용자 부분 이동은 금지한다.
3. Drop은 초기화된 경로에서 정확히 한 번 실행한다.
4. Panic MVP는 abort다.

## 4. 확정 규칙

1. Local과 필드는 선언 역순으로 Drop한다.
2. Moved 값은 원본에서 Drop하지 않는다.
3. 조건부 상태에는 Drop Flag를 사용한다.

추가 불변 조건:

1. 평가 순서·Drop 순서·소유권 의미는 최적화로 변경하지 않는다.
2. 문자열 이름은 의미 분석 단계에서 안정적 ID로 변환한다.
3. Error Node·Error Type은 복구를 위해 허용하지만 Codegen 입력에는 남지 않는다.
4. 미지원 기능은 추측해서 실행하지 않고 안정적인 진단으로 거부한다.
5. 동일 입력과 옵션은 결정적인 결과를 생성한다.

## 5. 구현 요구사항

1. needsDrop, Drop Glue, Cleanup CFG를 구현한다.

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

- return·break·try·재대입·부분 초기화를 테스트한다.

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


## 9. 변경 절차

범위 변경은 NOVA-006의 변경 제안·영향 분석·결정 기록 절차를 따른다.


---

# 사용자 drop·Drop Glue 사양서

| 항목 | 값 |
|---|---|
| 문서 ID | NOVA-056 |
| 대상 | Nova 0.1 |
| 구현 시점 | Drop 구현 전 |
| 상태 | 확정 |
| 갱신일 | 2026-07-27 |

## 1. 목적

이 문서는 **사용자 drop·Drop Glue 사양서**에 대한 언어 의미, Compiler 책임, 구현 경계, 오류 처리 및 완료 기준을 정의한다.

이 문서는 해당 구현 단계에 진입하기 전 동결해야 하는 기준이다.

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

1. Move 값은 명시적 take로 이동한다.
2. 사용자 부분 이동은 금지한다.
3. Drop은 초기화된 경로에서 정확히 한 번 실행한다.
4. Panic MVP는 abort다.

## 4. 확정 규칙

1. Local과 필드는 선언 역순으로 Drop한다.
2. Moved 값은 원본에서 Drop하지 않는다.
3. 조건부 상태에는 Drop Flag를 사용한다.

추가 불변 조건:

1. 평가 순서·Drop 순서·소유권 의미는 최적화로 변경하지 않는다.
2. 문자열 이름은 의미 분석 단계에서 안정적 ID로 변환한다.
3. Error Node·Error Type은 복구를 위해 허용하지만 Codegen 입력에는 남지 않는다.
4. 미지원 기능은 추측해서 실행하지 않고 안정적인 진단으로 거부한다.
5. 동일 입력과 옵션은 결정적인 결과를 생성한다.

## 5. 구현 요구사항

1. needsDrop, Drop Glue, Cleanup CFG를 구현한다.

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

- return·break·try·재대입·부분 초기화를 테스트한다.

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


## 9. 변경 절차

범위 변경은 NOVA-006의 변경 제안·영향 분석·결정 기록 절차를 따른다.


---

# Shared·Weak·참조 횟수 모델 사양서

| 항목 | 값 |
|---|---|
| 문서 ID | NOVA-057 |
| 대상 | Nova 0.1 |
| 구현 시점 | Runtime 전 |
| 상태 | 확정 |
| 갱신일 | 2026-07-27 |

## 1. 목적

이 문서는 **Shared·Weak·참조 횟수 모델 사양서**에 대한 언어 의미, Compiler 책임, 구현 경계, 오류 처리 및 완료 기준을 정의한다.

이 문서는 해당 구현 단계에 진입하기 전 동결해야 하는 기준이다.

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

1. Move 값은 명시적 take로 이동한다.
2. 사용자 부분 이동은 금지한다.
3. Drop은 초기화된 경로에서 정확히 한 번 실행한다.
4. Panic MVP는 abort다.

## 4. 확정 규칙

1. 마지막 Strong Drop에서 T를 Drop한다.
2. Weak는 Control Block 수명만 연장한다.
3. 순환은 Weak로 끊는다.

추가 불변 조건:

1. 평가 순서·Drop 순서·소유권 의미는 최적화로 변경하지 않는다.
2. 문자열 이름은 의미 분석 단계에서 안정적 ID로 변환한다.
3. Error Node·Error Type은 복구를 위해 허용하지만 Codegen 입력에는 남지 않는다.
4. 미지원 기능은 추측해서 실행하지 않고 안정적인 진단으로 거부한다.
5. 동일 입력과 옵션은 결정적인 결과를 생성한다.

## 5. 구현 요구사항

1. Atomic reference count와 ControlBlock을 구현한다.

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

- Weak upgrade와 경쟁 상황을 테스트한다.

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


## 9. 변경 절차

범위 변경은 NOVA-006의 변경 제안·영향 분석·결정 기록 절차를 따른다.


---

# Raw Pointer·Unsafe Capability 사양서

| 항목 | 값 |
|---|---|
| 문서 ID | NOVA-058 |
| 대상 | Nova 0.1 |
| 구현 시점 | FFI 전 |
| 상태 | 확정 |
| 갱신일 | 2026-07-27 |

## 1. 목적

이 문서는 **Raw Pointer·Unsafe Capability 사양서**에 대한 언어 의미, Compiler 책임, 구현 경계, 오류 처리 및 완료 기준을 정의한다.

이 문서는 해당 구현 단계에 진입하기 전 동결해야 하는 기준이다.

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

1. Move 값은 명시적 take로 이동한다.
2. 사용자 부분 이동은 금지한다.
3. Drop은 초기화된 경로에서 정확히 한 번 실행한다.
4. Panic MVP는 abort다.

## 4. 확정 규칙

1. unsafe는 Capability Block으로 제한한다.
2. 기존 소유권·빌림 검사를 자동 해제하지 않는다.
3. Raw Pointer는 안전 View보다 약한 보장만 가진다.
4. Nova ABI와 C ABI를 구분한다.
5. 큰 반환은 hidden return place를 사용할 수 있다.
6. change는 독점 Pointer로 내린다.

추가 불변 조건:

1. 평가 순서·Drop 순서·소유권 의미는 최적화로 변경하지 않는다.
2. 문자열 이름은 의미 분석 단계에서 안정적 ID로 변환한다.
3. Error Node·Error Type은 복구를 위해 허용하지만 Codegen 입력에는 남지 않는다.
4. 미지원 기능은 추측해서 실행하지 않고 안정적인 진단으로 거부한다.
5. 동일 입력과 옵션은 결정적인 결과를 생성한다.

## 5. 구현 요구사항

1. Capability stack과 위험 연산 검증을 구현한다.
2. AbiClass와 PassMode를 계산한다.

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

- Pointer escape와 FFI wrapper를 테스트한다.
- Scalar·Aggregate·View·Handle 호출을 테스트한다.

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


## 9. 변경 절차

범위 변경은 NOVA-006의 변경 제안·영향 분석·결정 기록 절차를 따른다.


---

# Pinning·자기 참조 타입 사양서

| 항목 | 값 |
|---|---|
| 문서 ID | NOVA-059 |
| 대상 | Nova 0.1 |
| 구현 시점 | MVP 이후 |
| 상태 | 후속 |
| 갱신일 | 2026-07-27 |

## 1. 목적

이 문서는 **Pinning·자기 참조 타입 사양서**에 대한 언어 의미, Compiler 책임, 구현 경계, 오류 처리 및 완료 기준을 정의한다.

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

1. Move 값은 명시적 take로 이동한다.
2. 사용자 부분 이동은 금지한다.
3. Drop은 초기화된 경로에서 정확히 한 번 실행한다.
4. Panic MVP는 abort다.

## 4. 확정 규칙

1. 양방향 타입 검사를 사용한다.
2. ErrorType으로 연쇄 오류를 억제한다.
3. 손실 없는 제한적 숫자 승격만 암묵 허용한다.

추가 불변 조건:

1. 평가 순서·Drop 순서·소유권 의미는 최적화로 변경하지 않는다.
2. 문자열 이름은 의미 분석 단계에서 안정적 ID로 변환한다.
3. Error Node·Error Type은 복구를 위해 허용하지만 Codegen 입력에는 남지 않는다.
4. 미지원 기능은 추측해서 실행하지 않고 안정적인 진단으로 거부한다.
5. 동일 입력과 옵션은 결정적인 결과를 생성한다.

## 5. 구현 요구사항

1. TypeInterner, Unification, ConversionCost를 구현한다.

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

- Literal·Nullable·함수·Generic 타입을 테스트한다.

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


---

# Thread 이동·공유 안전성 사양서

| 항목 | 값 |
|---|---|
| 문서 ID | NOVA-060 |
| 대상 | Nova 0.1 |
| 구현 시점 | Thread 구현 전 |
| 상태 | 확정 |
| 갱신일 | 2026-07-27 |

## 1. 목적

이 문서는 **Thread 이동·공유 안전성 사양서**에 대한 언어 의미, Compiler 책임, 구현 경계, 오류 처리 및 완료 기준을 정의한다.

이 문서는 해당 구현 단계에 진입하기 전 동결해야 하는 기준이다.

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

1. Move 값은 명시적 take로 이동한다.
2. 사용자 부분 이동은 금지한다.
3. Drop은 초기화된 경로에서 정확히 한 번 실행한다.
4. Panic MVP는 abort다.

## 4. 확정 규칙

1. Thread 이동·공유 안전성의 입력·출력·불변 조건을 명시한다.
2. Thread 이동·공유 안전성의 MVP 범위와 후속 범위를 분리한다.
3. Thread 이동·공유 안전성의 결과는 컴파일 순서와 병렬 실행 순서에 독립적이어야 한다.

추가 불변 조건:

1. 평가 순서·Drop 순서·소유권 의미는 최적화로 변경하지 않는다.
2. 문자열 이름은 의미 분석 단계에서 안정적 ID로 변환한다.
3. Error Node·Error Type은 복구를 위해 허용하지만 Codegen 입력에는 남지 않는다.
4. 미지원 기능은 추측해서 실행하지 않고 안정적인 진단으로 거부한다.
5. 동일 입력과 옵션은 결정적인 결과를 생성한다.

## 5. 구현 요구사항

1. Thread 이동·공유 안전성 전용 데이터 모델과 Query 경계를 구현한다.
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

- Thread 이동·공유 안전성의 정상·오류·경계 사례를 테스트한다.
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


## 9. 변경 절차

범위 변경은 NOVA-006의 변경 제안·영향 분석·결정 기록 절차를 따른다.


---

# Interface·구현·정적 디스패치 사양서

| 항목 | 값 |
|---|---|
| 문서 ID | NOVA-061 |
| 대상 | Nova 0.1 |
| 구현 시점 | Interface 전 |
| 상태 | 확정 |
| 갱신일 | 2026-07-27 |

## 1. 목적

이 문서는 **Interface·구현·정적 디스패치 사양서**에 대한 언어 의미, Compiler 책임, 구현 경계, 오류 처리 및 완료 기준을 정의한다.

이 문서는 해당 구현 단계에 진입하기 전 동결해야 하는 기준이다.

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

1. Interface는 implements로 명시한다.
2. Generic은 Monomorphization한다.
3. Interface 호출은 정적 디스패치한다.
4. 동적 Interface Object는 후속이다.

## 4. 확정 규칙

1. implements는 명시적이다.
2. 동일 타입·Interface 구현은 하나다.
3. MVP는 정적 디스패치다.

추가 불변 조건:

1. 평가 순서·Drop 순서·소유권 의미는 최적화로 변경하지 않는다.
2. 문자열 이름은 의미 분석 단계에서 안정적 ID로 변환한다.
3. Error Node·Error Type은 복구를 위해 허용하지만 Codegen 입력에는 남지 않는다.
4. 미지원 기능은 추측해서 실행하지 않고 안정적인 진단으로 거부한다.
5. 동일 입력과 옵션은 결정적인 결과를 생성한다.

## 5. 구현 요구사항

1. ImplRegistry와 MethodMapping을 구현한다.

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

- 누락 구현·상속·Receiver 불일치를 테스트한다.

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


## 9. 변경 절차

범위 변경은 NOVA-006의 변경 제안·영향 분석·결정 기록 절차를 따른다.


---

# Generic 제약 증명 사양서

| 항목 | 값 |
|---|---|
| 문서 ID | NOVA-062 |
| 대상 | Nova 0.1 |
| 구현 시점 | Generic 전 |
| 상태 | 확정 |
| 갱신일 | 2026-07-27 |

## 1. 목적

이 문서는 **Generic 제약 증명 사양서**에 대한 언어 의미, Compiler 책임, 구현 경계, 오류 처리 및 완료 기준을 정의한다.

이 문서는 해당 구현 단계에 진입하기 전 동결해야 하는 기준이다.

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

1. Interface는 implements로 명시한다.
2. Generic은 Monomorphization한다.
3. Interface 호출은 정적 디스패치한다.
4. 동적 Interface Object는 후속이다.

## 4. 확정 규칙

1. 타입 인수는 인수·Receiver·기대 타입에서 추론한다.
2. 동일 SpecializationKey는 한 번만 생성한다.
3. 무한 성장 특수화는 오류다.

추가 불변 조건:

1. 평가 순서·Drop 순서·소유권 의미는 최적화로 변경하지 않는다.
2. 문자열 이름은 의미 분석 단계에서 안정적 ID로 변환한다.
3. Error Node·Error Type은 복구를 위해 허용하지만 Codegen 입력에는 남지 않는다.
4. 미지원 기능은 추측해서 실행하지 않고 안정적인 진단으로 거부한다.
5. 동일 입력과 옵션은 결정적인 결과를 생성한다.

## 5. 구현 요구사항

1. Substitution, Registry, Monomorphization queue를 구현한다.

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

- 추론·조건부 구현·Cache·코드 팽창을 테스트한다.

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


## 9. 변경 절차

범위 변경은 NOVA-006의 변경 제안·영향 분석·결정 기록 절차를 따른다.


---

# Generic 타입 추론 사양서

| 항목 | 값 |
|---|---|
| 문서 ID | NOVA-063 |
| 대상 | Nova 0.1 |
| 구현 시점 | Generic 전 |
| 상태 | 확정 |
| 갱신일 | 2026-07-27 |

## 1. 목적

이 문서는 **Generic 타입 추론 사양서**에 대한 언어 의미, Compiler 책임, 구현 경계, 오류 처리 및 완료 기준을 정의한다.

이 문서는 해당 구현 단계에 진입하기 전 동결해야 하는 기준이다.

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

1. Interface는 implements로 명시한다.
2. Generic은 Monomorphization한다.
3. Interface 호출은 정적 디스패치한다.
4. 동적 Interface Object는 후속이다.

## 4. 확정 규칙

1. 양방향 타입 검사를 사용한다.
2. ErrorType으로 연쇄 오류를 억제한다.
3. 손실 없는 제한적 숫자 승격만 암묵 허용한다.
4. 타입 인수는 인수·Receiver·기대 타입에서 추론한다.
5. 동일 SpecializationKey는 한 번만 생성한다.
6. 무한 성장 특수화는 오류다.

추가 불변 조건:

1. 평가 순서·Drop 순서·소유권 의미는 최적화로 변경하지 않는다.
2. 문자열 이름은 의미 분석 단계에서 안정적 ID로 변환한다.
3. Error Node·Error Type은 복구를 위해 허용하지만 Codegen 입력에는 남지 않는다.
4. 미지원 기능은 추측해서 실행하지 않고 안정적인 진단으로 거부한다.
5. 동일 입력과 옵션은 결정적인 결과를 생성한다.

## 5. 구현 요구사항

1. TypeInterner, Unification, ConversionCost를 구현한다.
2. Substitution, Registry, Monomorphization queue를 구현한다.

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

- Literal·Nullable·함수·Generic 타입을 테스트한다.
- 추론·조건부 구현·Cache·코드 팽창을 테스트한다.

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


## 9. 변경 절차

범위 변경은 NOVA-006의 변경 제안·영향 분석·결정 기록 절차를 따른다.


---

# Monomorphization·특수화 사양서

| 항목 | 값 |
|---|---|
| 문서 ID | NOVA-064 |
| 대상 | Nova 0.1 |
| 구현 시점 | Generic 전 |
| 상태 | 확정 |
| 갱신일 | 2026-07-27 |

## 1. 목적

이 문서는 **Monomorphization·특수화 사양서**에 대한 언어 의미, Compiler 책임, 구현 경계, 오류 처리 및 완료 기준을 정의한다.

이 문서는 해당 구현 단계에 진입하기 전 동결해야 하는 기준이다.

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

1. Interface는 implements로 명시한다.
2. Generic은 Monomorphization한다.
3. Interface 호출은 정적 디스패치한다.
4. 동적 Interface Object는 후속이다.

## 4. 확정 규칙

1. Monomorphization·특수화의 입력·출력·불변 조건을 명시한다.
2. Monomorphization·특수화의 MVP 범위와 후속 범위를 분리한다.
3. Monomorphization·특수화의 결과는 컴파일 순서와 병렬 실행 순서에 독립적이어야 한다.

추가 불변 조건:

1. 평가 순서·Drop 순서·소유권 의미는 최적화로 변경하지 않는다.
2. 문자열 이름은 의미 분석 단계에서 안정적 ID로 변환한다.
3. Error Node·Error Type은 복구를 위해 허용하지만 Codegen 입력에는 남지 않는다.
4. 미지원 기능은 추측해서 실행하지 않고 안정적인 진단으로 거부한다.
5. 동일 입력과 옵션은 결정적인 결과를 생성한다.

## 5. 구현 요구사항

1. Monomorphization·특수화 전용 데이터 모델과 Query 경계를 구현한다.
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

- Monomorphization·특수화의 정상·오류·경계 사례를 테스트한다.
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


## 9. 변경 절차

범위 변경은 NOVA-006의 변경 제안·영향 분석·결정 기록 절차를 따른다.


---

# Generic Cache·코드 팽창 제어 사양서

| 항목 | 값 |
|---|---|
| 문서 ID | NOVA-065 |
| 대상 | Nova 0.1 |
| 구현 시점 | 최적화 전 |
| 상태 | 확정 |
| 갱신일 | 2026-07-27 |

## 1. 목적

이 문서는 **Generic Cache·코드 팽창 제어 사양서**에 대한 언어 의미, Compiler 책임, 구현 경계, 오류 처리 및 완료 기준을 정의한다.

이 문서는 해당 구현 단계에 진입하기 전 동결해야 하는 기준이다.

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

1. Interface는 implements로 명시한다.
2. Generic은 Monomorphization한다.
3. Interface 호출은 정적 디스패치한다.
4. 동적 Interface Object는 후속이다.

## 4. 확정 규칙

1. 타입 인수는 인수·Receiver·기대 타입에서 추론한다.
2. 동일 SpecializationKey는 한 번만 생성한다.
3. 무한 성장 특수화는 오류다.

추가 불변 조건:

1. 평가 순서·Drop 순서·소유권 의미는 최적화로 변경하지 않는다.
2. 문자열 이름은 의미 분석 단계에서 안정적 ID로 변환한다.
3. Error Node·Error Type은 복구를 위해 허용하지만 Codegen 입력에는 남지 않는다.
4. 미지원 기능은 추측해서 실행하지 않고 안정적인 진단으로 거부한다.
5. 동일 입력과 옵션은 결정적인 결과를 생성한다.

## 5. 구현 요구사항

1. Substitution, Registry, Monomorphization queue를 구현한다.

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

- 추론·조건부 구현·Cache·코드 팽창을 테스트한다.

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


## 9. 변경 절차

범위 변경은 NOVA-006의 변경 제안·영향 분석·결정 기록 절차를 따른다.


---

# Associated Type·Dynamic Interface Object 사양서

| 항목 | 값 |
|---|---|
| 문서 ID | NOVA-066 |
| 대상 | Nova 0.1 |
| 구현 시점 | MVP 이후 |
| 상태 | 후속 |
| 갱신일 | 2026-07-27 |

## 1. 목적

이 문서는 **Associated Type·Dynamic Interface Object 사양서**에 대한 언어 의미, Compiler 책임, 구현 경계, 오류 처리 및 완료 기준을 정의한다.

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

1. Interface는 implements로 명시한다.
2. Generic은 Monomorphization한다.
3. Interface 호출은 정적 디스패치한다.
4. 동적 Interface Object는 후속이다.

## 4. 확정 규칙

1. implements는 명시적이다.
2. 동일 타입·Interface 구현은 하나다.
3. MVP는 정적 디스패치다.

추가 불변 조건:

1. 평가 순서·Drop 순서·소유권 의미는 최적화로 변경하지 않는다.
2. 문자열 이름은 의미 분석 단계에서 안정적 ID로 변환한다.
3. Error Node·Error Type은 복구를 위해 허용하지만 Codegen 입력에는 남지 않는다.
4. 미지원 기능은 추측해서 실행하지 않고 안정적인 진단으로 거부한다.
5. 동일 입력과 옵션은 결정적인 결과를 생성한다.

## 5. 구현 요구사항

1. ImplRegistry와 MethodMapping을 구현한다.

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

- 누락 구현·상속·Receiver 불일치를 테스트한다.

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


---

# VTable·Object Safety 사양서

| 항목 | 값 |
|---|---|
| 문서 ID | NOVA-067 |
| 대상 | Nova 0.1 |
| 구현 시점 | MVP 이후 |
| 상태 | 후속 |
| 갱신일 | 2026-07-27 |

## 1. 목적

이 문서는 **VTable·Object Safety 사양서**에 대한 언어 의미, Compiler 책임, 구현 경계, 오류 처리 및 완료 기준을 정의한다.

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

1. Interface는 implements로 명시한다.
2. Generic은 Monomorphization한다.
3. Interface 호출은 정적 디스패치한다.
4. 동적 Interface Object는 후속이다.

## 4. 확정 규칙

1. VTable·Object Safety의 입력·출력·불변 조건을 명시한다.
2. VTable·Object Safety의 MVP 범위와 후속 범위를 분리한다.
3. VTable·Object Safety의 결과는 컴파일 순서와 병렬 실행 순서에 독립적이어야 한다.

추가 불변 조건:

1. 평가 순서·Drop 순서·소유권 의미는 최적화로 변경하지 않는다.
2. 문자열 이름은 의미 분석 단계에서 안정적 ID로 변환한다.
3. Error Node·Error Type은 복구를 위해 허용하지만 Codegen 입력에는 남지 않는다.
4. 미지원 기능은 추측해서 실행하지 않고 안정적인 진단으로 거부한다.
5. 동일 입력과 옵션은 결정적인 결과를 생성한다.

## 5. 구현 요구사항

1. VTable·Object Safety 전용 데이터 모델과 Query 경계를 구현한다.
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

- VTable·Object Safety의 정상·오류·경계 사례를 테스트한다.
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


---

# Compiler 전체 아키텍처 사양서

## 파이프라인

```text
Source
→ Lexer
→ END Normalizer
→ Parser
→ AST
→ HIR
→ Name Resolution
→ Type·Ownership Check
→ MIR
→ Move·Borrow·Drop Analysis
→ MIR Optimization
→ LLVM IR
→ Object
→ Link
```

## 구현 언어와 Backend

- Compiler: Rust
- Native Backend: LLVM Adapter
- Runtime: Rust와 최소 C ABI
- 표준 라이브러리: 초기에는 Nova+Runtime Intrinsic 혼합

## Workspace

```text
nova/
├─ crates/
│  ├─ nova-core-ids
│  ├─ nova-source
│  ├─ nova-diagnostics
│  ├─ nova-syntax
│  ├─ nova-lexer
│  ├─ nova-parser
│  ├─ nova-ast
│  ├─ nova-hir
│  ├─ nova-resolve
│  ├─ nova-types
│  ├─ nova-typecheck
│  ├─ nova-mir
│  ├─ nova-analysis
│  ├─ nova-codegen
│  ├─ nova-codegen-llvm
│  ├─ nova-package
│  └─ nova-cli
├─ runtime/
├─ std/
├─ tests/
├─ examples/
└─ docs/
```

## Query

```text
source_file(FileId)
lex(FileId)
parse(FileId)
lower_hir(ModuleId)
resolve_names(ModuleId)
type_check(FunctionId)
build_mir(FunctionId)
analyze_moves(FunctionId)
analyze_borrows(FunctionId)
elaborate_drops(FunctionId)
codegen_unit(CodegenUnitId)
```

## 필수 Dump

```bash
nova check file.nova --emit=tokens
nova check file.nova --emit=ast
nova check file.nova --emit=hir
nova check file.nova --emit=typed-hir
nova check file.nova --emit=mir
nova build file.nova --emit=llvm
```

## 첫 구현 순서

1. ID·Source·Span
2. Diagnostic
3. Lexer
4. Parser·AST
5. HIR
6. 최소 Resolve·Type
7. 최소 MIR
8. LLVM `main`
9. Console Runtime
10. Hello Nova E2E


---

# 저장소·Crate·모듈 구조 사양서

## Crate 책임

| Crate | 책임 |
|---|---|
| nova-core-ids | 안정적 ID |
| nova-source | File·Span·LineIndex |
| nova-diagnostics | 진단과 Renderer |
| nova-syntax | TokenKind·Trivia |
| nova-lexer | Source→Token |
| nova-parser | Token→AST |
| nova-ast | AST Node·Visitor |
| nova-hir | HIR Node·Lowering |
| nova-resolve | Scope·DefId·Import |
| nova-types | Type Interner |
| nova-typecheck | 타입·호출·소유권 |
| nova-mir | MIR 구조·Lowering |
| nova-analysis | Move·Borrow·Drop |
| nova-codegen | Backend Trait |
| nova-codegen-llvm | LLVM 구현 |
| nova-package | Manifest·Artifact |
| nova-cli | 명령행 |

## 금지 의존

- Lexer→Parser 참조 금지
- HIR→Typecheck 참조 금지
- Types→LLVM 참조 금지
- Analysis→CLI 참조 금지
- LLVM 타입의 Core 노출 금지

## 기본 CI

```bash
cargo fmt --check
cargo clippy --workspace --all-targets -- -D warnings
cargo test --workspace
cargo check --workspace --all-features
```


---

# Source Manager·File ID·Span 사양서

| 항목 | 값 |
|---|---|
| 문서 ID | NOVA-070 |
| 대상 | Nova 0.1 |
| 구현 시점 | 착수 전 |
| 상태 | 확정 |
| 갱신일 | 2026-07-27 |

## 1. 목적

이 문서는 **Source Manager·File ID·Span 사양서**에 대한 언어 의미, Compiler 책임, 구현 경계, 오류 처리 및 완료 기준을 정의한다.

이 문서는 해당 구현 단계에 진입하기 전 동결해야 하는 기준이다.

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

1. Source→Token→AST→HIR→Typed HIR→MIR→Codegen 단계로 분리한다.
2. 각 단계는 SourceInfo와 안정적 ID를 보존한다.
3. Rust Crate 경계는 Compiler 단계 경계와 일치시킨다.

## 4. 확정 규칙

1. FileId는 안정적 정수 ID다.
2. Span은 반열린 바이트 범위다.
3. Line map은 지연 계산한다.
4. ReadOnlySpan은 Read View, Span은 Change View다.
5. 원본보다 오래 살 수 없다.
6. splitAt은 비중첩 계약을 제공한다.

추가 불변 조건:

1. 평가 순서·Drop 순서·소유권 의미는 최적화로 변경하지 않는다.
2. 문자열 이름은 의미 분석 단계에서 안정적 ID로 변환한다.
3. Error Node·Error Type은 복구를 위해 허용하지만 Codegen 입력에는 남지 않는다.
4. 미지원 기능은 추측해서 실행하지 않고 안정적인 진단으로 거부한다.
5. 동일 입력과 옵션은 결정적인 결과를 생성한다.

## 5. 구현 요구사항

1. SourceDatabase와 LineIndex를 구현한다.
2. pointer+length ABI와 region 계약을 구현한다.

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

- Unicode·CRLF·대형 파일을 테스트한다.
- slice·split·borrow conflict를 테스트한다.

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


## 9. 변경 절차

범위 변경은 NOVA-006의 변경 제안·영향 분석·결정 기록 절차를 따른다.


---

# Lexer 구현 사양서

## 입력·출력

UTF-8 Source를 Trivia 포함 Token Stream으로 변환한다.

```rust
struct Token {
    kind: TokenKind,
    span: Span,
}
```

## Token

- Identifier와 Keyword
- Integer·Float·String·Char
- Interpolation Start·Text·Expression Boundary
- `(){}[],:.;`
- Operator
- NewLine
- Whitespace·Comment Trivia
- Error
- EOF

## 규칙

- 최장 일치
- 잘못된 문자는 Error Token으로 보존
- 숫자 범위 검사는 타입 단계
- 중첩 Block Comment 허용
- String 보간: `"value: {value}"`
- `{{`, `}}`는 문자 중괄호

## END Normalizer

문장을 끝낼 수 있는 Token 다음의 NewLine을 `END`로 바꾼다. `()`, `[]` 내부에서는 억제하고 `{}` 내부에서는 허용한다. `else`, `.`, `,`, 연산자 앞뒤 연속 줄은 END를 억제한다.

## 완료

- 선형 시간
- Unicode·CRLF
- 보간 중첩
- Fuzz Crash 없음
- Token Snapshot 결정성


---

# Parser 구현 사양서

## 전략

- 선언·문장·타입: Recursive Descent
- 표현식: Pratt
- 오류 복구: Error Node·Synthetic Token
- 입력: END가 정규화된 Token Stream

## 상태

```rust
struct Parser<'a> {
    tokens: &'a [Token],
    cursor: usize,
    diagnostics: Vec<Diagnostic>,
    delimiters: Vec<TokenKind>,
}
```

## Top-level

- use
- const
- func
- struct·class·enum·interface
- foreign

Top-level 실행 문장은 금지한다.

## 표현식 우선순위

1. `until`, `through`
2. OR
3. AND
4. 비교
5. 덧셈
6. 곱셈
7. Prefix
8. Postfix Call·Member·Index·`exists`

할당은 Statement Parser가 처리한다.

## 복구

동기화:

```text
END }
func struct class enum interface const foreign
if while for loop match return break continue
```

## 완료

- 불완전 IDE 입력에서 Panic 없음
- 모든 문법 Production Snapshot
- 같은 Token은 같은 AST·진단


---

# AST Node 구조서

| 항목 | 값 |
|---|---|
| 문서 ID | NOVA-073 |
| 대상 | Nova 0.1 |
| 구현 시점 | 착수 전 |
| 상태 | 확정 |
| 갱신일 | 2026-07-27 |

## 1. 목적

이 문서는 **AST Node 구조서**에 대한 언어 의미, Compiler 책임, 구현 경계, 오류 처리 및 완료 기준을 정의한다.

이 문서는 해당 구현 단계에 진입하기 전 동결해야 하는 기준이다.

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

1. Source→Token→AST→HIR→Typed HIR→MIR→Codegen 단계로 분리한다.
2. 각 단계는 SourceInfo와 안정적 ID를 보존한다.
3. Rust Crate 경계는 Compiler 단계 경계와 일치시킨다.

## 4. 확정 규칙

1. AST는 소스 구조와 오류를 보존한다.
2. Semantic Type과 DefId는 AST에 저장하지 않는다.

추가 불변 조건:

1. 평가 순서·Drop 순서·소유권 의미는 최적화로 변경하지 않는다.
2. 문자열 이름은 의미 분석 단계에서 안정적 ID로 변환한다.
3. Error Node·Error Type은 복구를 위해 허용하지만 Codegen 입력에는 남지 않는다.
4. 미지원 기능은 추측해서 실행하지 않고 안정적인 진단으로 거부한다.
5. 동일 입력과 옵션은 결정적인 결과를 생성한다.

## 5. 구현 요구사항

1. Arena와 AstNodeId를 사용한다.

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

- Span·Error Node·Visitor를 테스트한다.

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


## 9. 변경 절차

범위 변경은 NOVA-006의 변경 제안·영향 분석·결정 기록 절차를 따른다.


---

# HIR Node 구조서

## 역할

HIR은 AST를 의미 분석용 Canonical 구조로 바꾼다.

```text
AST → HIR → Resolution/Type Tables → Typed HIR → MIR
```

## ID

```text
HirId HirItemId HirExprId HirStmtId
HirPatternId ScopeId DefId LocalId
```

## 정규화

- `T?` → `Option<T>`
- `void`·생략 반환 → Unit
- Unit 값 → `()`
- `try`, `exists`, `using`, `for`는 전용 HIR로 보존
- 이름 인수는 SourceOrder와 Label을 보존
- 할당은 HirStatement

## Side Table

```text
ResolutionMap<HirId, Resolution>
TypeTable<HirId, TypeId>
CoercionTable<HirId, Coercion>
OwnershipTable<HirId, OwnershipAction>
EffectTable<DefId, Effects>
```

## 원칙

- HIR은 Trivia에 의존하지 않음
- Synthetic Node는 SourceOrigin 보존
- Error HIR로 후속 분석 지속


---

# AST→HIR Lowering 사양서

| 항목 | 값 |
|---|---|
| 문서 ID | NOVA-075 |
| 대상 | Nova 0.1 |
| 구현 시점 | HIR 전 |
| 상태 | 확정 |
| 갱신일 | 2026-07-27 |

## 1. 목적

이 문서는 **AST→HIR Lowering 사양서**에 대한 언어 의미, Compiler 책임, 구현 경계, 오류 처리 및 완료 기준을 정의한다.

이 문서는 해당 구현 단계에 진입하기 전 동결해야 하는 기준이다.

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

1. Source→Token→AST→HIR→Typed HIR→MIR→Codegen 단계로 분리한다.
2. 각 단계는 SourceInfo와 안정적 ID를 보존한다.
3. Rust Crate 경계는 Compiler 단계 경계와 일치시킨다.

## 4. 확정 규칙

1. AST는 소스 구조와 오류를 보존한다.
2. Semantic Type과 DefId는 AST에 저장하지 않는다.
3. HIR은 문법 Sugar를 Canonical 의미 구조로 정규화한다.
4. Resolution·Type 결과는 Side Table로 둔다.

추가 불변 조건:

1. 평가 순서·Drop 순서·소유권 의미는 최적화로 변경하지 않는다.
2. 문자열 이름은 의미 분석 단계에서 안정적 ID로 변환한다.
3. Error Node·Error Type은 복구를 위해 허용하지만 Codegen 입력에는 남지 않는다.
4. 미지원 기능은 추측해서 실행하지 않고 안정적인 진단으로 거부한다.
5. 동일 입력과 옵션은 결정적인 결과를 생성한다.

## 5. 구현 요구사항

1. Arena와 AstNodeId를 사용한다.
2. HirArena, ScopeId, Lowerer를 구현한다.

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

- Span·Error Node·Visitor를 테스트한다.
- AST→HIR Snapshot과 Error HIR을 테스트한다.

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


## 9. 변경 절차

범위 변경은 NOVA-006의 변경 제안·영향 분석·결정 기록 절차를 따른다.


---

# Symbol Table·Definition Registry 사양서

| 항목 | 값 |
|---|---|
| 문서 ID | NOVA-076 |
| 대상 | Nova 0.1 |
| 구현 시점 | 이름 해석 전 |
| 상태 | 확정 |
| 갱신일 | 2026-07-27 |

## 1. 목적

이 문서는 **Symbol Table·Definition Registry 사양서**에 대한 언어 의미, Compiler 책임, 구현 경계, 오류 처리 및 완료 기준을 정의한다.

이 문서는 해당 구현 단계에 진입하기 전 동결해야 하는 기준이다.

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

1. Source→Token→AST→HIR→Typed HIR→MIR→Codegen 단계로 분리한다.
2. 각 단계는 SourceInfo와 안정적 ID를 보존한다.
3. Rust Crate 경계는 Compiler 단계 경계와 일치시킨다.

## 4. 확정 규칙

1. Symbol Table·Definition Registry의 입력·출력·불변 조건을 명시한다.
2. Symbol Table·Definition Registry의 MVP 범위와 후속 범위를 분리한다.
3. Symbol Table·Definition Registry의 결과는 컴파일 순서와 병렬 실행 순서에 독립적이어야 한다.

추가 불변 조건:

1. 평가 순서·Drop 순서·소유권 의미는 최적화로 변경하지 않는다.
2. 문자열 이름은 의미 분석 단계에서 안정적 ID로 변환한다.
3. Error Node·Error Type은 복구를 위해 허용하지만 Codegen 입력에는 남지 않는다.
4. 미지원 기능은 추측해서 실행하지 않고 안정적인 진단으로 거부한다.
5. 동일 입력과 옵션은 결정적인 결과를 생성한다.

## 5. 구현 요구사항

1. Symbol Table·Definition Registry 전용 데이터 모델과 Query 경계를 구현한다.
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

- Symbol Table·Definition Registry의 정상·오류·경계 사례를 테스트한다.
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


## 9. 변경 절차

범위 변경은 NOVA-006의 변경 제안·영향 분석·결정 기록 절차를 따른다.


---

# 타입 검사기·소유권 검사기 아키텍처 사양서

| 항목 | 값 |
|---|---|
| 문서 ID | NOVA-077 |
| 대상 | Nova 0.1 |
| 구현 시점 | 타입 검사 전 |
| 상태 | 확정 |
| 갱신일 | 2026-07-27 |

## 1. 목적

이 문서는 **타입 검사기·소유권 검사기 아키텍처 사양서**에 대한 언어 의미, Compiler 책임, 구현 경계, 오류 처리 및 완료 기준을 정의한다.

이 문서는 해당 구현 단계에 진입하기 전 동결해야 하는 기준이다.

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

1. Source→Token→AST→HIR→Typed HIR→MIR→Codegen 단계로 분리한다.
2. 각 단계는 SourceInfo와 안정적 ID를 보존한다.
3. Rust Crate 경계는 Compiler 단계 경계와 일치시킨다.

## 4. 확정 규칙

1. 양방향 타입 검사를 사용한다.
2. ErrorType으로 연쇄 오류를 억제한다.
3. 손실 없는 제한적 숫자 승격만 암묵 허용한다.
4. Move 타입은 하나의 Owner를 가진다.
5. Read는 소유권을 소비하지 않는다.
6. take 이후 원본은 재초기화 전까지 사용할 수 없다.

추가 불변 조건:

1. 평가 순서·Drop 순서·소유권 의미는 최적화로 변경하지 않는다.
2. 문자열 이름은 의미 분석 단계에서 안정적 ID로 변환한다.
3. Error Node·Error Type은 복구를 위해 허용하지만 Codegen 입력에는 남지 않는다.
4. 미지원 기능은 추측해서 실행하지 않고 안정적인 진단으로 거부한다.
5. 동일 입력과 옵션은 결정적인 결과를 생성한다.

## 5. 구현 요구사항

1. TypeInterner, Unification, ConversionCost를 구현한다.
2. OwnershipAction과 Move Origin을 기록한다.

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

- Literal·Nullable·함수·Generic 타입을 테스트한다.
- 인수·반환·분기·Closure 이동을 테스트한다.

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


## 9. 변경 절차

범위 변경은 NOVA-006의 변경 제안·영향 분석·결정 기록 절차를 따른다.


---

# 진단 시스템·Error Code 관리 사양서

## 구조

```rust
struct Diagnostic {
    code: DiagnosticCode,
    severity: Severity,
    message: String,
    primary: Label,
    secondary: Vec<Label>,
    notes: Vec<String>,
    suggestions: Vec<Suggestion>,
}
```

## Code 영역

```text
N1xxx Lexer·Parser
N2xxx 이름·타입·함수·Interface·Generic
N3xxx 제어 흐름·Match·Const
N4xxx Move·Borrow·Drop·수명
N5xxx ABI·FFI·Runtime
N8xxx Tooling·Package
N9xxx ICE·내부 검증
```

## Renderer

- ANSI Terminal
- Plain Text
- JSON
- Snapshot

Span은 바이트 기준, 표시 열은 Unicode Scalar 기준이다.

## Suggestion

- MachineApplicable
- MaybeIncorrect
- HasPlaceholders
- Unspecified

## ICE

Compiler 버전, Target, Query Stack, Source 위치, 재현 Dump 정보를 출력한다.


---

# Compiler Query·의존성·Cache 사양서

| 항목 | 값 |
|---|---|
| 문서 ID | NOVA-079 |
| 대상 | Nova 0.1 |
| 구현 시점 | 증분 기능 전 |
| 상태 | 확정 |
| 갱신일 | 2026-07-27 |

## 1. 목적

이 문서는 **Compiler Query·의존성·Cache 사양서**에 대한 언어 의미, Compiler 책임, 구현 경계, 오류 처리 및 완료 기준을 정의한다.

이 문서는 해당 구현 단계에 진입하기 전 동결해야 하는 기준이다.

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

1. Source→Token→AST→HIR→Typed HIR→MIR→Codegen 단계로 분리한다.
2. 각 단계는 SourceInfo와 안정적 ID를 보존한다.
3. Rust Crate 경계는 Compiler 단계 경계와 일치시킨다.

## 4. 확정 규칙

1. Compiler 단계는 Query 경계로 분리한다.
2. Frontend와 Backend는 MIR로 분리한다.
3. 단계별 Dump를 제공한다.

추가 불변 조건:

1. 평가 순서·Drop 순서·소유권 의미는 최적화로 변경하지 않는다.
2. 문자열 이름은 의미 분석 단계에서 안정적 ID로 변환한다.
3. Error Node·Error Type은 복구를 위해 허용하지만 Codegen 입력에는 남지 않는다.
4. 미지원 기능은 추측해서 실행하지 않고 안정적인 진단으로 거부한다.
5. 동일 입력과 옵션은 결정적인 결과를 생성한다.

## 5. 구현 요구사항

1. Rust workspace와 공통 ID crate를 구성한다.

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

- Hello Nova E2E를 테스트한다.

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


## 9. 변경 절차

범위 변경은 NOVA-006의 변경 제안·영향 분석·결정 기록 절차를 따른다.


---

# Compiler 내부 오류·ICE 처리 사양서

| 항목 | 값 |
|---|---|
| 문서 ID | NOVA-080 |
| 대상 | Nova 0.1 |
| 구현 시점 | 구현 병행 |
| 상태 | 확정 |
| 갱신일 | 2026-07-27 |

## 1. 목적

이 문서는 **Compiler 내부 오류·ICE 처리 사양서**에 대한 언어 의미, Compiler 책임, 구현 경계, 오류 처리 및 완료 기준을 정의한다.

이 문서는 해당 구현 단계에 진입하기 전 동결해야 하는 기준이다.

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

1. Source→Token→AST→HIR→Typed HIR→MIR→Codegen 단계로 분리한다.
2. 각 단계는 SourceInfo와 안정적 ID를 보존한다.
3. Rust Crate 경계는 Compiler 단계 경계와 일치시킨다.

## 4. 확정 규칙

1. Compiler 단계는 Query 경계로 분리한다.
2. Frontend와 Backend는 MIR로 분리한다.
3. 단계별 Dump를 제공한다.

추가 불변 조건:

1. 평가 순서·Drop 순서·소유권 의미는 최적화로 변경하지 않는다.
2. 문자열 이름은 의미 분석 단계에서 안정적 ID로 변환한다.
3. Error Node·Error Type은 복구를 위해 허용하지만 Codegen 입력에는 남지 않는다.
4. 미지원 기능은 추측해서 실행하지 않고 안정적인 진단으로 거부한다.
5. 동일 입력과 옵션은 결정적인 결과를 생성한다.

## 5. 구현 요구사항

1. Rust workspace와 공통 ID crate를 구성한다.

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

- Hello Nova E2E를 테스트한다.

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


## 9. 변경 절차

범위 변경은 NOVA-006의 변경 제안·영향 분석·결정 기록 절차를 따른다.


---

# MIR 구조·CFG·평가 순서 사양서

| 항목 | 값 |
|---|---|
| 문서 ID | NOVA-081 |
| 대상 | Nova 0.1 |
| 구현 시점 | MIR 전 |
| 상태 | 확정 |
| 갱신일 | 2026-07-27 |

## 1. 목적

이 문서는 **MIR 구조·CFG·평가 순서 사양서**에 대한 언어 의미, Compiler 책임, 구현 경계, 오류 처리 및 완료 기준을 정의한다.

이 문서는 해당 구현 단계에 진입하기 전 동결해야 하는 기준이다.

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

1. MIR은 Basic Block 기반 CFG다.
2. Move·Borrow·Drop 분석은 MIR에서 수행한다.
3. 최적화 전후 검증기를 실행한다.

## 4. 확정 규칙

1. MIR은 Place·Operand·Rvalue·Statement·Terminator로 구성한다.
2. Call과 Drop은 Terminator다.
3. 평가 순서를 명시한다.

추가 불변 조건:

1. 평가 순서·Drop 순서·소유권 의미는 최적화로 변경하지 않는다.
2. 문자열 이름은 의미 분석 단계에서 안정적 ID로 변환한다.
3. Error Node·Error Type은 복구를 위해 허용하지만 Codegen 입력에는 남지 않는다.
4. 미지원 기능은 추측해서 실행하지 않고 안정적인 진단으로 거부한다.
5. 동일 입력과 옵션은 결정적인 결과를 생성한다.

## 5. 구현 요구사항

1. BasicBlockData와 MIR validator를 구현한다.

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

- CFG·Cleanup·Dump를 테스트한다.

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


## 9. 변경 절차

범위 변경은 NOVA-006의 변경 제안·영향 분석·결정 기록 절차를 따른다.


---

# HIR→MIR Lowering 사양서

| 항목 | 값 |
|---|---|
| 문서 ID | NOVA-082 |
| 대상 | Nova 0.1 |
| 구현 시점 | MIR 전 |
| 상태 | 확정 |
| 갱신일 | 2026-07-27 |

## 1. 목적

이 문서는 **HIR→MIR Lowering 사양서**에 대한 언어 의미, Compiler 책임, 구현 경계, 오류 처리 및 완료 기준을 정의한다.

이 문서는 해당 구현 단계에 진입하기 전 동결해야 하는 기준이다.

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

1. MIR은 Basic Block 기반 CFG다.
2. Move·Borrow·Drop 분석은 MIR에서 수행한다.
3. 최적화 전후 검증기를 실행한다.

## 4. 확정 규칙

1. HIR은 문법 Sugar를 Canonical 의미 구조로 정규화한다.
2. Resolution·Type 결과는 Side Table로 둔다.
3. MIR은 Place·Operand·Rvalue·Statement·Terminator로 구성한다.
4. Call과 Drop은 Terminator다.
5. 평가 순서를 명시한다.

추가 불변 조건:

1. 평가 순서·Drop 순서·소유권 의미는 최적화로 변경하지 않는다.
2. 문자열 이름은 의미 분석 단계에서 안정적 ID로 변환한다.
3. Error Node·Error Type은 복구를 위해 허용하지만 Codegen 입력에는 남지 않는다.
4. 미지원 기능은 추측해서 실행하지 않고 안정적인 진단으로 거부한다.
5. 동일 입력과 옵션은 결정적인 결과를 생성한다.

## 5. 구현 요구사항

1. HirArena, ScopeId, Lowerer를 구현한다.
2. BasicBlockData와 MIR validator를 구현한다.

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

- AST→HIR Snapshot과 Error HIR을 테스트한다.
- CFG·Cleanup·Dump를 테스트한다.

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


## 9. 변경 절차

범위 변경은 NOVA-006의 변경 제안·영향 분석·결정 기록 절차를 따른다.


---

# MIR 검증기 사양서

| 항목 | 값 |
|---|---|
| 문서 ID | NOVA-083 |
| 대상 | Nova 0.1 |
| 구현 시점 | MIR 전 |
| 상태 | 확정 |
| 갱신일 | 2026-07-27 |

## 1. 목적

이 문서는 **MIR 검증기 사양서**에 대한 언어 의미, Compiler 책임, 구현 경계, 오류 처리 및 완료 기준을 정의한다.

이 문서는 해당 구현 단계에 진입하기 전 동결해야 하는 기준이다.

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

1. MIR은 Basic Block 기반 CFG다.
2. Move·Borrow·Drop 분석은 MIR에서 수행한다.
3. 최적화 전후 검증기를 실행한다.

## 4. 확정 규칙

1. MIR은 Place·Operand·Rvalue·Statement·Terminator로 구성한다.
2. Call과 Drop은 Terminator다.
3. 평가 순서를 명시한다.

추가 불변 조건:

1. 평가 순서·Drop 순서·소유권 의미는 최적화로 변경하지 않는다.
2. 문자열 이름은 의미 분석 단계에서 안정적 ID로 변환한다.
3. Error Node·Error Type은 복구를 위해 허용하지만 Codegen 입력에는 남지 않는다.
4. 미지원 기능은 추측해서 실행하지 않고 안정적인 진단으로 거부한다.
5. 동일 입력과 옵션은 결정적인 결과를 생성한다.

## 5. 구현 요구사항

1. BasicBlockData와 MIR validator를 구현한다.

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

- CFG·Cleanup·Dump를 테스트한다.

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


## 9. 변경 절차

범위 변경은 NOVA-006의 변경 제안·영향 분석·결정 기록 절차를 따른다.


---

# 초기화·Move 분석 구현서

| 항목 | 값 |
|---|---|
| 문서 ID | NOVA-084 |
| 대상 | Nova 0.1 |
| 구현 시점 | MIR 분석 전 |
| 상태 | 확정 |
| 갱신일 | 2026-07-27 |

## 1. 목적

이 문서는 **초기화·Move 분석 구현서**에 대한 언어 의미, Compiler 책임, 구현 경계, 오류 처리 및 완료 기준을 정의한다.

이 문서는 해당 구현 단계에 진입하기 전 동결해야 하는 기준이다.

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

1. MIR은 Basic Block 기반 CFG다.
2. Move·Borrow·Drop 분석은 MIR에서 수행한다.
3. 최적화 전후 검증기를 실행한다.

## 4. 확정 규칙

1. 초기화·Move 분석의 입력·출력·불변 조건을 명시한다.
2. 초기화·Move 분석의 MVP 범위와 후속 범위를 분리한다.
3. 초기화·Move 분석의 결과는 컴파일 순서와 병렬 실행 순서에 독립적이어야 한다.

추가 불변 조건:

1. 평가 순서·Drop 순서·소유권 의미는 최적화로 변경하지 않는다.
2. 문자열 이름은 의미 분석 단계에서 안정적 ID로 변환한다.
3. Error Node·Error Type은 복구를 위해 허용하지만 Codegen 입력에는 남지 않는다.
4. 미지원 기능은 추측해서 실행하지 않고 안정적인 진단으로 거부한다.
5. 동일 입력과 옵션은 결정적인 결과를 생성한다.

## 5. 구현 요구사항

1. 초기화·Move 분석 전용 데이터 모델과 Query 경계를 구현한다.
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

- 초기화·Move 분석의 정상·오류·경계 사례를 테스트한다.
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


## 9. 변경 절차

범위 변경은 NOVA-006의 변경 제안·영향 분석·결정 기록 절차를 따른다.


---

# Borrow Checker 구현서

| 항목 | 값 |
|---|---|
| 문서 ID | NOVA-085 |
| 대상 | Nova 0.1 |
| 구현 시점 | Borrow 전 |
| 상태 | 확정 |
| 갱신일 | 2026-07-27 |

## 1. 목적

이 문서는 **Borrow Checker 구현서**에 대한 언어 의미, Compiler 책임, 구현 경계, 오류 처리 및 완료 기준을 정의한다.

이 문서는 해당 구현 단계에 진입하기 전 동결해야 하는 기준이다.

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

1. MIR은 Basic Block 기반 CFG다.
2. Move·Borrow·Drop 분석은 MIR에서 수행한다.
3. 최적화 전후 검증기를 실행한다.

## 4. 확정 규칙

1. Borrow Checker의 입력·출력·불변 조건을 명시한다.
2. Borrow Checker의 MVP 범위와 후속 범위를 분리한다.
3. Borrow Checker의 결과는 컴파일 순서와 병렬 실행 순서에 독립적이어야 한다.

추가 불변 조건:

1. 평가 순서·Drop 순서·소유권 의미는 최적화로 변경하지 않는다.
2. 문자열 이름은 의미 분석 단계에서 안정적 ID로 변환한다.
3. Error Node·Error Type은 복구를 위해 허용하지만 Codegen 입력에는 남지 않는다.
4. 미지원 기능은 추측해서 실행하지 않고 안정적인 진단으로 거부한다.
5. 동일 입력과 옵션은 결정적인 결과를 생성한다.

## 5. 구현 요구사항

1. Borrow Checker 전용 데이터 모델과 Query 경계를 구현한다.
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

- Borrow Checker의 정상·오류·경계 사례를 테스트한다.
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


## 9. 변경 절차

범위 변경은 NOVA-006의 변경 제안·영향 분석·결정 기록 절차를 따른다.


---

# Drop Elaboration 구현서

| 항목 | 값 |
|---|---|
| 문서 ID | NOVA-086 |
| 대상 | Nova 0.1 |
| 구현 시점 | Drop 전 |
| 상태 | 확정 |
| 갱신일 | 2026-07-27 |

## 1. 목적

이 문서는 **Drop Elaboration 구현서**에 대한 언어 의미, Compiler 책임, 구현 경계, 오류 처리 및 완료 기준을 정의한다.

이 문서는 해당 구현 단계에 진입하기 전 동결해야 하는 기준이다.

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

1. MIR은 Basic Block 기반 CFG다.
2. Move·Borrow·Drop 분석은 MIR에서 수행한다.
3. 최적화 전후 검증기를 실행한다.

## 4. 확정 규칙

1. Local과 필드는 선언 역순으로 Drop한다.
2. Moved 값은 원본에서 Drop하지 않는다.
3. 조건부 상태에는 Drop Flag를 사용한다.

추가 불변 조건:

1. 평가 순서·Drop 순서·소유권 의미는 최적화로 변경하지 않는다.
2. 문자열 이름은 의미 분석 단계에서 안정적 ID로 변환한다.
3. Error Node·Error Type은 복구를 위해 허용하지만 Codegen 입력에는 남지 않는다.
4. 미지원 기능은 추측해서 실행하지 않고 안정적인 진단으로 거부한다.
5. 동일 입력과 옵션은 결정적인 결과를 생성한다.

## 5. 구현 요구사항

1. needsDrop, Drop Glue, Cleanup CFG를 구현한다.

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

- return·break·try·재대입·부분 초기화를 테스트한다.

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


## 9. 변경 절차

범위 변경은 NOVA-006의 변경 제안·영향 분석·결정 기록 절차를 따른다.


---

# Constant Evaluation Engine 사양서

| 항목 | 값 |
|---|---|
| 문서 ID | NOVA-087 |
| 대상 | Nova 0.1 |
| 구현 시점 | Const 전 |
| 상태 | 확정 |
| 갱신일 | 2026-07-27 |

## 1. 목적

이 문서는 **Constant Evaluation Engine 사양서**에 대한 언어 의미, Compiler 책임, 구현 경계, 오류 처리 및 완료 기준을 정의한다.

이 문서는 해당 구현 단계에 진입하기 전 동결해야 하는 기준이다.

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

1. MIR은 Basic Block 기반 CFG다.
2. Move·Borrow·Drop 분석은 MIR에서 수행한다.
3. 최적화 전후 검증기를 실행한다.

## 4. 확정 규칙

1. Constant Evaluation Engine의 입력·출력·불변 조건을 명시한다.
2. Constant Evaluation Engine의 MVP 범위와 후속 범위를 분리한다.
3. Constant Evaluation Engine의 결과는 컴파일 순서와 병렬 실행 순서에 독립적이어야 한다.

추가 불변 조건:

1. 평가 순서·Drop 순서·소유권 의미는 최적화로 변경하지 않는다.
2. 문자열 이름은 의미 분석 단계에서 안정적 ID로 변환한다.
3. Error Node·Error Type은 복구를 위해 허용하지만 Codegen 입력에는 남지 않는다.
4. 미지원 기능은 추측해서 실행하지 않고 안정적인 진단으로 거부한다.
5. 동일 입력과 옵션은 결정적인 결과를 생성한다.

## 5. 구현 요구사항

1. Constant Evaluation Engine 전용 데이터 모델과 Query 경계를 구현한다.
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

- Constant Evaluation Engine의 정상·오류·경계 사례를 테스트한다.
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


## 9. 변경 절차

범위 변경은 NOVA-006의 변경 제안·영향 분석·결정 기록 절차를 따른다.


---

# Match Lowering 구현서

| 항목 | 값 |
|---|---|
| 문서 ID | NOVA-088 |
| 대상 | Nova 0.1 |
| 구현 시점 | Match 전 |
| 상태 | 확정 |
| 갱신일 | 2026-07-27 |

## 1. 목적

이 문서는 **Match Lowering 구현서**에 대한 언어 의미, Compiler 책임, 구현 경계, 오류 처리 및 완료 기준을 정의한다.

이 문서는 해당 구현 단계에 진입하기 전 동결해야 하는 기준이다.

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

1. MIR은 Basic Block 기반 CFG다.
2. Move·Borrow·Drop 분석은 MIR에서 수행한다.
3. 최적화 전후 검증기를 실행한다.

## 4. 확정 규칙

1. Match는 완전성을 검사한다.
2. 완전히 가려진 Arm은 도달 불가다.
3. Arm 순서를 보존한다.

추가 불변 조건:

1. 평가 순서·Drop 순서·소유권 의미는 최적화로 변경하지 않는다.
2. 문자열 이름은 의미 분석 단계에서 안정적 ID로 변환한다.
3. Error Node·Error Type은 복구를 위해 허용하지만 Codegen 입력에는 남지 않는다.
4. 미지원 기능은 추측해서 실행하지 않고 안정적인 진단으로 거부한다.
5. 동일 입력과 옵션은 결정적인 결과를 생성한다.

## 5. 구현 요구사항

1. Pattern matrix 또는 Decision Tree를 MIR로 내린다.

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

- Enum·Bool·Guard·중첩 Pattern을 테스트한다.

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


## 9. 변경 절차

범위 변경은 NOVA-006의 변경 제안·영향 분석·결정 기록 절차를 따른다.


---

# Generic 특수화 Pass 구현서

| 항목 | 값 |
|---|---|
| 문서 ID | NOVA-089 |
| 대상 | Nova 0.1 |
| 구현 시점 | Generic 전 |
| 상태 | 확정 |
| 갱신일 | 2026-07-27 |

## 1. 목적

이 문서는 **Generic 특수화 Pass 구현서**에 대한 언어 의미, Compiler 책임, 구현 경계, 오류 처리 및 완료 기준을 정의한다.

이 문서는 해당 구현 단계에 진입하기 전 동결해야 하는 기준이다.

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

1. MIR은 Basic Block 기반 CFG다.
2. Move·Borrow·Drop 분석은 MIR에서 수행한다.
3. 최적화 전후 검증기를 실행한다.

## 4. 확정 규칙

1. 타입 인수는 인수·Receiver·기대 타입에서 추론한다.
2. 동일 SpecializationKey는 한 번만 생성한다.
3. 무한 성장 특수화는 오류다.

추가 불변 조건:

1. 평가 순서·Drop 순서·소유권 의미는 최적화로 변경하지 않는다.
2. 문자열 이름은 의미 분석 단계에서 안정적 ID로 변환한다.
3. Error Node·Error Type은 복구를 위해 허용하지만 Codegen 입력에는 남지 않는다.
4. 미지원 기능은 추측해서 실행하지 않고 안정적인 진단으로 거부한다.
5. 동일 입력과 옵션은 결정적인 결과를 생성한다.

## 5. 구현 요구사항

1. Substitution, Registry, Monomorphization queue를 구현한다.

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

- 추론·조건부 구현·Cache·코드 팽창을 테스트한다.

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


## 9. 변경 절차

범위 변경은 NOVA-006의 변경 제안·영향 분석·결정 기록 절차를 따른다.


---

# MIR 최적화 Pass 목록·순서 사양서

| 항목 | 값 |
|---|---|
| 문서 ID | NOVA-090 |
| 대상 | Nova 0.1 |
| 구현 시점 | 최적화 전 |
| 상태 | 확정 |
| 갱신일 | 2026-07-27 |

## 1. 목적

이 문서는 **MIR 최적화 Pass 목록·순서 사양서**에 대한 언어 의미, Compiler 책임, 구현 경계, 오류 처리 및 완료 기준을 정의한다.

이 문서는 해당 구현 단계에 진입하기 전 동결해야 하는 기준이다.

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

1. MIR은 Basic Block 기반 CFG다.
2. Move·Borrow·Drop 분석은 MIR에서 수행한다.
3. 최적화 전후 검증기를 실행한다.

## 4. 확정 규칙

1. MIR은 Place·Operand·Rvalue·Statement·Terminator로 구성한다.
2. Call과 Drop은 Terminator다.
3. 평가 순서를 명시한다.

추가 불변 조건:

1. 평가 순서·Drop 순서·소유권 의미는 최적화로 변경하지 않는다.
2. 문자열 이름은 의미 분석 단계에서 안정적 ID로 변환한다.
3. Error Node·Error Type은 복구를 위해 허용하지만 Codegen 입력에는 남지 않는다.
4. 미지원 기능은 추측해서 실행하지 않고 안정적인 진단으로 거부한다.
5. 동일 입력과 옵션은 결정적인 결과를 생성한다.

## 5. 구현 요구사항

1. BasicBlockData와 MIR validator를 구현한다.

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

- CFG·Cleanup·Dump를 테스트한다.

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


## 9. 변경 절차

범위 변경은 NOVA-006의 변경 제안·영향 분석·결정 기록 절차를 따른다.


---

# SSA 변환 여부 결정서

| 항목 | 값 |
|---|---|
| 문서 ID | NOVA-091 |
| 대상 | Nova 0.1 |
| 구현 시점 | 최적화 전 |
| 상태 | 확정 |
| 갱신일 | 2026-07-27 |

## 1. 목적

이 문서는 **SSA 변환 여부 결정서**에 대한 언어 의미, Compiler 책임, 구현 경계, 오류 처리 및 완료 기준을 정의한다.

이 문서는 해당 구현 단계에 진입하기 전 동결해야 하는 기준이다.

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

1. MIR은 Basic Block 기반 CFG다.
2. Move·Borrow·Drop 분석은 MIR에서 수행한다.
3. 최적화 전후 검증기를 실행한다.

## 4. 확정 규칙

1. MVP MIR은 비SSA Place 기반이다.
2. 소유권 분석은 SSA 이전에 수행한다.
3. SSA는 후속 최적화 IR로 제한한다.

추가 불변 조건:

1. 평가 순서·Drop 순서·소유권 의미는 최적화로 변경하지 않는다.
2. 문자열 이름은 의미 분석 단계에서 안정적 ID로 변환한다.
3. Error Node·Error Type은 복구를 위해 허용하지만 Codegen 입력에는 남지 않는다.
4. 미지원 기능은 추측해서 실행하지 않고 안정적인 진단으로 거부한다.
5. 동일 입력과 옵션은 결정적인 결과를 생성한다.

## 5. 구현 요구사항

1. MIR→SSA 경계를 예약한다.

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

- Loop와 Drop Flag 변환을 시험한다.

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


## 9. 변경 절차

범위 변경은 NOVA-006의 변경 제안·영향 분석·결정 기록 절차를 따른다.


---

# Debug MIR 출력 형식 사양서

| 항목 | 값 |
|---|---|
| 문서 ID | NOVA-092 |
| 대상 | Nova 0.1 |
| 구현 시점 | 구현 병행 |
| 상태 | 확정 |
| 갱신일 | 2026-07-27 |

## 1. 목적

이 문서는 **Debug MIR 출력 형식 사양서**에 대한 언어 의미, Compiler 책임, 구현 경계, 오류 처리 및 완료 기준을 정의한다.

이 문서는 해당 구현 단계에 진입하기 전 동결해야 하는 기준이다.

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

1. MIR은 Basic Block 기반 CFG다.
2. Move·Borrow·Drop 분석은 MIR에서 수행한다.
3. 최적화 전후 검증기를 실행한다.

## 4. 확정 규칙

1. MIR은 Place·Operand·Rvalue·Statement·Terminator로 구성한다.
2. Call과 Drop은 Terminator다.
3. 평가 순서를 명시한다.

추가 불변 조건:

1. 평가 순서·Drop 순서·소유권 의미는 최적화로 변경하지 않는다.
2. 문자열 이름은 의미 분석 단계에서 안정적 ID로 변환한다.
3. Error Node·Error Type은 복구를 위해 허용하지만 Codegen 입력에는 남지 않는다.
4. 미지원 기능은 추측해서 실행하지 않고 안정적인 진단으로 거부한다.
5. 동일 입력과 옵션은 결정적인 결과를 생성한다.

## 5. 구현 요구사항

1. BasicBlockData와 MIR validator를 구현한다.

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

- CFG·Cleanup·Dump를 테스트한다.

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


## 9. 변경 절차

범위 변경은 NOVA-006의 변경 제안·영향 분석·결정 기록 절차를 따른다.


---

# Backend 선택·LLVM 연동 결정서

## 결정

Nova 0.1 Native AOT Backend는 LLVM을 사용한다. 의존성은 `nova-codegen-llvm`에 격리한다.

## 이유

- 다중 Target
- 성숙한 최적화
- Object·Debug Info
- C ABI 생태계
- LTO·SIMD 확장

## 인터페이스

```rust
trait CodegenBackend {
    fn codegen_unit(
        &self,
        unit: &CodegenUnit,
        target: &TargetSpec,
        options: &CodegenOptions,
    ) -> Result<ObjectArtifact, CodegenError>;
}
```

## 우선 Target

1. Windows x86_64 MSVC
2. Linux x86_64 GNU
3. Linux AArch64
4. Windows AArch64

## Pipeline

```text
Verified MIR
→ Codegen Unit
→ LLVM Module
→ Verify
→ Optimize
→ Object
→ Link
```

JIT·Wasm·GPU·직접 Machine Code는 0.1에서 제외한다.


---

# Nova 타입→Backend 타입 Mapping 사양서

| 항목 | 값 |
|---|---|
| 문서 ID | NOVA-094 |
| 대상 | Nova 0.1 |
| 구현 시점 | Backend 전 |
| 상태 | 확정 |
| 갱신일 | 2026-07-27 |

## 1. 목적

이 문서는 **Nova 타입→Backend 타입 Mapping 사양서**에 대한 언어 의미, Compiler 책임, 구현 경계, 오류 처리 및 완료 기준을 정의한다.

이 문서는 해당 구현 단계에 진입하기 전 동결해야 하는 기준이다.

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

1. Backend는 LLVM Adapter다.
2. ABI와 Layout은 Target Query다.
3. Runtime은 Startup·Allocation·Panic·Shared·Thread로 모듈화한다.

## 4. 확정 규칙

1. 양방향 타입 검사를 사용한다.
2. ErrorType으로 연쇄 오류를 억제한다.
3. 손실 없는 제한적 숫자 승격만 암묵 허용한다.
4. Backend 입력은 검증된 구체 MIR다.
5. LLVM 의존성은 Adapter crate에 격리한다.
6. IR verify 실패는 ICE다.

추가 불변 조건:

1. 평가 순서·Drop 순서·소유권 의미는 최적화로 변경하지 않는다.
2. 문자열 이름은 의미 분석 단계에서 안정적 ID로 변환한다.
3. Error Node·Error Type은 복구를 위해 허용하지만 Codegen 입력에는 남지 않는다.
4. 미지원 기능은 추측해서 실행하지 않고 안정적인 진단으로 거부한다.
5. 동일 입력과 옵션은 결정적인 결과를 생성한다.

## 5. 구현 요구사항

1. TypeInterner, Unification, ConversionCost를 구현한다.
2. TargetMachine과 Function lowerer를 구현한다.

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

- Literal·Nullable·함수·Generic 타입을 테스트한다.
- Scalar·Struct·Enum·Drop E2E를 테스트한다.

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


## 9. 변경 절차

범위 변경은 NOVA-006의 변경 제안·영향 분석·결정 기록 절차를 따른다.


---

# Backend 타입 레이아웃·정렬·Niche 구현서

| 항목 | 값 |
|---|---|
| 문서 ID | NOVA-095 |
| 대상 | Nova 0.1 |
| 구현 시점 | Backend 전 |
| 상태 | 확정 |
| 갱신일 | 2026-07-27 |

## 1. 목적

이 문서는 **Backend 타입 레이아웃·정렬·Niche 구현서**에 대한 언어 의미, Compiler 책임, 구현 경계, 오류 처리 및 완료 기준을 정의한다.

이 문서는 해당 구현 단계에 진입하기 전 동결해야 하는 기준이다.

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

1. Backend는 LLVM Adapter다.
2. ABI와 Layout은 Target Query다.
3. Runtime은 Startup·Allocation·Panic·Shared·Thread로 모듈화한다.

## 4. 확정 규칙

1. 양방향 타입 검사를 사용한다.
2. ErrorType으로 연쇄 오류를 억제한다.
3. 손실 없는 제한적 숫자 승격만 암묵 허용한다.
4. Backend 입력은 검증된 구체 MIR다.
5. LLVM 의존성은 Adapter crate에 격리한다.
6. IR verify 실패는 ICE다.

추가 불변 조건:

1. 평가 순서·Drop 순서·소유권 의미는 최적화로 변경하지 않는다.
2. 문자열 이름은 의미 분석 단계에서 안정적 ID로 변환한다.
3. Error Node·Error Type은 복구를 위해 허용하지만 Codegen 입력에는 남지 않는다.
4. 미지원 기능은 추측해서 실행하지 않고 안정적인 진단으로 거부한다.
5. 동일 입력과 옵션은 결정적인 결과를 생성한다.

## 5. 구현 요구사항

1. TypeInterner, Unification, ConversionCost를 구현한다.
2. TargetMachine과 Function lowerer를 구현한다.

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

- Literal·Nullable·함수·Generic 타입을 테스트한다.
- Scalar·Struct·Enum·Drop E2E를 테스트한다.

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


## 9. 변경 절차

범위 변경은 NOVA-006의 변경 제안·영향 분석·결정 기록 절차를 따른다.


---

# 함수 ABI·인수·반환 Lowering 사양서

| 항목 | 값 |
|---|---|
| 문서 ID | NOVA-096 |
| 대상 | Nova 0.1 |
| 구현 시점 | Backend 전 |
| 상태 | 확정 |
| 갱신일 | 2026-07-27 |

## 1. 목적

이 문서는 **함수 ABI·인수·반환 Lowering 사양서**에 대한 언어 의미, Compiler 책임, 구현 경계, 오류 처리 및 완료 기준을 정의한다.

이 문서는 해당 구현 단계에 진입하기 전 동결해야 하는 기준이다.

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

1. Backend는 LLVM Adapter다.
2. ABI와 Layout은 Target Query다.
3. Runtime은 Startup·Allocation·Panic·Shared·Thread로 모듈화한다.

## 4. 확정 규칙

1. Signature에는 Receiver·Parameter Mode·Return Ownership·Effect가 포함된다.
2. 반환 타입만으로 Overload하지 않는다.
3. 기본 인수는 호출자에서 평가한다.
4. Nova ABI와 C ABI를 구분한다.
5. 큰 반환은 hidden return place를 사용할 수 있다.
6. change는 독점 Pointer로 내린다.

추가 불변 조건:

1. 평가 순서·Drop 순서·소유권 의미는 최적화로 변경하지 않는다.
2. 문자열 이름은 의미 분석 단계에서 안정적 ID로 변환한다.
3. Error Node·Error Type은 복구를 위해 허용하지만 Codegen 입력에는 남지 않는다.
4. 미지원 기능은 추측해서 실행하지 않고 안정적인 진단으로 거부한다.
5. 동일 입력과 옵션은 결정적인 결과를 생성한다.

## 5. 구현 요구사항

1. FunctionId, Signature, OverloadSet, Call lowering을 구현한다.
2. AbiClass와 PassMode를 계산한다.

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

- Read·change·take와 이름 인수를 테스트한다.
- Scalar·Aggregate·View·Handle 호출을 테스트한다.

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


## 9. 변경 절차

범위 변경은 NOVA-006의 변경 제안·영향 분석·결정 기록 절차를 따른다.


---

# Name Mangling·Symbol 사양서

| 항목 | 값 |
|---|---|
| 문서 ID | NOVA-097 |
| 대상 | Nova 0.1 |
| 구현 시점 | Link 전 |
| 상태 | 확정 |
| 갱신일 | 2026-07-27 |

## 1. 목적

이 문서는 **Name Mangling·Symbol 사양서**에 대한 언어 의미, Compiler 책임, 구현 경계, 오류 처리 및 완료 기준을 정의한다.

이 문서는 해당 구현 단계에 진입하기 전 동결해야 하는 기준이다.

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

1. Backend는 LLVM Adapter다.
2. ABI와 Layout은 Target Query다.
3. Runtime은 Startup·Allocation·Panic·Shared·Thread로 모듈화한다.

## 4. 확정 규칙

1. Name Mangling·Symbol의 입력·출력·불변 조건을 명시한다.
2. Name Mangling·Symbol의 MVP 범위와 후속 범위를 분리한다.
3. Name Mangling·Symbol의 결과는 컴파일 순서와 병렬 실행 순서에 독립적이어야 한다.

추가 불변 조건:

1. 평가 순서·Drop 순서·소유권 의미는 최적화로 변경하지 않는다.
2. 문자열 이름은 의미 분석 단계에서 안정적 ID로 변환한다.
3. Error Node·Error Type은 복구를 위해 허용하지만 Codegen 입력에는 남지 않는다.
4. 미지원 기능은 추측해서 실행하지 않고 안정적인 진단으로 거부한다.
5. 동일 입력과 옵션은 결정적인 결과를 생성한다.

## 5. 구현 요구사항

1. Name Mangling·Symbol 전용 데이터 모델과 Query 경계를 구현한다.
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

- Name Mangling·Symbol의 정상·오류·경계 사례를 테스트한다.
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


## 9. 변경 절차

범위 변경은 NOVA-006의 변경 제안·영향 분석·결정 기록 절차를 따른다.


---

# Object File·Linker 연동 사양서

| 항목 | 값 |
|---|---|
| 문서 ID | NOVA-098 |
| 대상 | Nova 0.1 |
| 구현 시점 | Link 전 |
| 상태 | 확정 |
| 갱신일 | 2026-07-27 |

## 1. 목적

이 문서는 **Object File·Linker 연동 사양서**에 대한 언어 의미, Compiler 책임, 구현 경계, 오류 처리 및 완료 기준을 정의한다.

이 문서는 해당 구현 단계에 진입하기 전 동결해야 하는 기준이다.

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

1. Backend는 LLVM Adapter다.
2. ABI와 Layout은 Target Query다.
3. Runtime은 Startup·Allocation·Panic·Shared·Thread로 모듈화한다.

## 4. 확정 규칙

1. Object File·Linker 연동의 입력·출력·불변 조건을 명시한다.
2. Object File·Linker 연동의 MVP 범위와 후속 범위를 분리한다.
3. Object File·Linker 연동의 결과는 컴파일 순서와 병렬 실행 순서에 독립적이어야 한다.

추가 불변 조건:

1. 평가 순서·Drop 순서·소유권 의미는 최적화로 변경하지 않는다.
2. 문자열 이름은 의미 분석 단계에서 안정적 ID로 변환한다.
3. Error Node·Error Type은 복구를 위해 허용하지만 Codegen 입력에는 남지 않는다.
4. 미지원 기능은 추측해서 실행하지 않고 안정적인 진단으로 거부한다.
5. 동일 입력과 옵션은 결정적인 결과를 생성한다.

## 5. 구현 요구사항

1. Object File·Linker 연동 전용 데이터 모델과 Query 경계를 구현한다.
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

- Object File·Linker 연동의 정상·오류·경계 사례를 테스트한다.
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


## 9. 변경 절차

범위 변경은 NOVA-006의 변경 제안·영향 분석·결정 기록 절차를 따른다.


---

# Runtime Startup·main 호출 사양서

| 항목 | 값 |
|---|---|
| 문서 ID | NOVA-099 |
| 대상 | Nova 0.1 |
| 구현 시점 | 실행 전 |
| 상태 | 확정 |
| 갱신일 | 2026-07-27 |

## 1. 목적

이 문서는 **Runtime Startup·main 호출 사양서**에 대한 언어 의미, Compiler 책임, 구현 경계, 오류 처리 및 완료 기준을 정의한다.

이 문서는 해당 구현 단계에 진입하기 전 동결해야 하는 기준이다.

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

1. Backend는 LLVM Adapter다.
2. ABI와 Layout은 Target Query다.
3. Runtime은 Startup·Allocation·Panic·Shared·Thread로 모듈화한다.

## 4. 확정 규칙

1. Runtime Startup·main 호출의 입력·출력·불변 조건을 명시한다.
2. Runtime Startup·main 호출의 MVP 범위와 후속 범위를 분리한다.
3. Runtime Startup·main 호출의 결과는 컴파일 순서와 병렬 실행 순서에 독립적이어야 한다.

추가 불변 조건:

1. 평가 순서·Drop 순서·소유권 의미는 최적화로 변경하지 않는다.
2. 문자열 이름은 의미 분석 단계에서 안정적 ID로 변환한다.
3. Error Node·Error Type은 복구를 위해 허용하지만 Codegen 입력에는 남지 않는다.
4. 미지원 기능은 추측해서 실행하지 않고 안정적인 진단으로 거부한다.
5. 동일 입력과 옵션은 결정적인 결과를 생성한다.

## 5. 구현 요구사항

1. Runtime Startup·main 호출 전용 데이터 모델과 Query 경계를 구현한다.
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

- Runtime Startup·main 호출의 정상·오류·경계 사례를 테스트한다.
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


## 9. 변경 절차

범위 변경은 NOVA-006의 변경 제안·영향 분석·결정 기록 절차를 따른다.


---

# 메모리 Allocator·OOM 정책 사양서

| 항목 | 값 |
|---|---|
| 문서 ID | NOVA-100 |
| 대상 | Nova 0.1 |
| 구현 시점 | Heap 사용 전 |
| 상태 | 확정 |
| 갱신일 | 2026-07-27 |

## 1. 목적

이 문서는 **메모리 Allocator·OOM 정책 사양서**에 대한 언어 의미, Compiler 책임, 구현 경계, 오류 처리 및 완료 기준을 정의한다.

이 문서는 해당 구현 단계에 진입하기 전 동결해야 하는 기준이다.

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

1. Backend는 LLVM Adapter다.
2. ABI와 Layout은 Target Query다.
3. Runtime은 Startup·Allocation·Panic·Shared·Thread로 모듈화한다.

## 4. 확정 규칙

1. 메모리 Allocator·OOM 정책의 입력·출력·불변 조건을 명시한다.
2. 메모리 Allocator·OOM 정책의 MVP 범위와 후속 범위를 분리한다.
3. 메모리 Allocator·OOM 정책의 결과는 컴파일 순서와 병렬 실행 순서에 독립적이어야 한다.

추가 불변 조건:

1. 평가 순서·Drop 순서·소유권 의미는 최적화로 변경하지 않는다.
2. 문자열 이름은 의미 분석 단계에서 안정적 ID로 변환한다.
3. Error Node·Error Type은 복구를 위해 허용하지만 Codegen 입력에는 남지 않는다.
4. 미지원 기능은 추측해서 실행하지 않고 안정적인 진단으로 거부한다.
5. 동일 입력과 옵션은 결정적인 결과를 생성한다.

## 5. 구현 요구사항

1. 메모리 Allocator·OOM 정책 전용 데이터 모델과 Query 경계를 구현한다.
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

- 메모리 Allocator·OOM 정책의 정상·오류·경계 사례를 테스트한다.
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


## 9. 변경 절차

범위 변경은 NOVA-006의 변경 제안·영향 분석·결정 기록 절차를 따른다.


---

# Panic Runtime·Stack Trace 사양서

| 항목 | 값 |
|---|---|
| 문서 ID | NOVA-101 |
| 대상 | Nova 0.1 |
| 구현 시점 | Panic 구현 전 |
| 상태 | 확정 |
| 갱신일 | 2026-07-27 |

## 1. 목적

이 문서는 **Panic Runtime·Stack Trace 사양서**에 대한 언어 의미, Compiler 책임, 구현 경계, 오류 처리 및 완료 기준을 정의한다.

이 문서는 해당 구현 단계에 진입하기 전 동결해야 하는 기준이다.

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

1. Backend는 LLVM Adapter다.
2. ABI와 Layout은 Target Query다.
3. Runtime은 Startup·Allocation·Panic·Shared·Thread로 모듈화한다.

## 4. 확정 규칙

1. MVP Panic은 Abort다.
2. FFI 경계를 넘지 않는다.
3. 메시지와 Source 위치를 기록한다.

추가 불변 조건:

1. 평가 순서·Drop 순서·소유권 의미는 최적화로 변경하지 않는다.
2. 문자열 이름은 의미 분석 단계에서 안정적 ID로 변환한다.
3. Error Node·Error Type은 복구를 위해 허용하지만 Codegen 입력에는 남지 않는다.
4. 미지원 기능은 추측해서 실행하지 않고 안정적인 진단으로 거부한다.
5. 동일 입력과 옵션은 결정적인 결과를 생성한다.

## 5. 구현 요구사항

1. panic runtime과 stack trace hook를 구현한다.

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

- bounds·overflow·명시적 panic을 테스트한다.

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


## 9. 변경 절차

범위 변경은 NOVA-006의 변경 제안·영향 분석·결정 기록 절차를 따른다.


---

# Shared·Weak Runtime 사양서

| 항목 | 값 |
|---|---|
| 문서 ID | NOVA-102 |
| 대상 | Nova 0.1 |
| 구현 시점 | Shared 구현 전 |
| 상태 | 확정 |
| 갱신일 | 2026-07-27 |

## 1. 목적

이 문서는 **Shared·Weak Runtime 사양서**에 대한 언어 의미, Compiler 책임, 구현 경계, 오류 처리 및 완료 기준을 정의한다.

이 문서는 해당 구현 단계에 진입하기 전 동결해야 하는 기준이다.

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

1. Backend는 LLVM Adapter다.
2. ABI와 Layout은 Target Query다.
3. Runtime은 Startup·Allocation·Panic·Shared·Thread로 모듈화한다.

## 4. 확정 규칙

1. 마지막 Strong Drop에서 T를 Drop한다.
2. Weak는 Control Block 수명만 연장한다.
3. 순환은 Weak로 끊는다.

추가 불변 조건:

1. 평가 순서·Drop 순서·소유권 의미는 최적화로 변경하지 않는다.
2. 문자열 이름은 의미 분석 단계에서 안정적 ID로 변환한다.
3. Error Node·Error Type은 복구를 위해 허용하지만 Codegen 입력에는 남지 않는다.
4. 미지원 기능은 추측해서 실행하지 않고 안정적인 진단으로 거부한다.
5. 동일 입력과 옵션은 결정적인 결과를 생성한다.

## 5. 구현 요구사항

1. Atomic reference count와 ControlBlock을 구현한다.

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

- Weak upgrade와 경쟁 상황을 테스트한다.

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


## 9. 변경 절차

범위 변경은 NOVA-006의 변경 제안·영향 분석·결정 기록 절차를 따른다.


---

# Thread·Atomic·Lock Runtime 사양서

| 항목 | 값 |
|---|---|
| 문서 ID | NOVA-103 |
| 대상 | Nova 0.1 |
| 구현 시점 | MVP 후반 |
| 상태 | 확정 |
| 갱신일 | 2026-07-27 |

## 1. 목적

이 문서는 **Thread·Atomic·Lock Runtime 사양서**에 대한 언어 의미, Compiler 책임, 구현 경계, 오류 처리 및 완료 기준을 정의한다.

이 문서는 해당 구현 단계에 진입하기 전 동결해야 하는 기준이다.

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

1. Backend는 LLVM Adapter다.
2. ABI와 Layout은 Target Query다.
3. Runtime은 Startup·Allocation·Panic·Shared·Thread로 모듈화한다.

## 4. 확정 규칙

1. Thread·Atomic·Lock Runtime의 입력·출력·불변 조건을 명시한다.
2. Thread·Atomic·Lock Runtime의 MVP 범위와 후속 범위를 분리한다.
3. Thread·Atomic·Lock Runtime의 결과는 컴파일 순서와 병렬 실행 순서에 독립적이어야 한다.

추가 불변 조건:

1. 평가 순서·Drop 순서·소유권 의미는 최적화로 변경하지 않는다.
2. 문자열 이름은 의미 분석 단계에서 안정적 ID로 변환한다.
3. Error Node·Error Type은 복구를 위해 허용하지만 Codegen 입력에는 남지 않는다.
4. 미지원 기능은 추측해서 실행하지 않고 안정적인 진단으로 거부한다.
5. 동일 입력과 옵션은 결정적인 결과를 생성한다.

## 5. 구현 요구사항

1. Thread·Atomic·Lock Runtime 전용 데이터 모델과 Query 경계를 구현한다.
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

- Thread·Atomic·Lock Runtime의 정상·오류·경계 사례를 테스트한다.
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


## 9. 변경 절차

범위 변경은 NOVA-006의 변경 제안·영향 분석·결정 기록 절차를 따른다.


---

# Platform Abstraction Layer 사양서

| 항목 | 값 |
|---|---|
| 문서 ID | NOVA-104 |
| 대상 | Nova 0.1 |
| 구현 시점 | 다중 OS 전 |
| 상태 | 확정 |
| 갱신일 | 2026-07-27 |

## 1. 목적

이 문서는 **Platform Abstraction Layer 사양서**에 대한 언어 의미, Compiler 책임, 구현 경계, 오류 처리 및 완료 기준을 정의한다.

이 문서는 해당 구현 단계에 진입하기 전 동결해야 하는 기준이다.

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

1. Backend는 LLVM Adapter다.
2. ABI와 Layout은 Target Query다.
3. Runtime은 Startup·Allocation·Panic·Shared·Thread로 모듈화한다.

## 4. 확정 규칙

1. Platform Abstraction Layer의 입력·출력·불변 조건을 명시한다.
2. Platform Abstraction Layer의 MVP 범위와 후속 범위를 분리한다.
3. Platform Abstraction Layer의 결과는 컴파일 순서와 병렬 실행 순서에 독립적이어야 한다.

추가 불변 조건:

1. 평가 순서·Drop 순서·소유권 의미는 최적화로 변경하지 않는다.
2. 문자열 이름은 의미 분석 단계에서 안정적 ID로 변환한다.
3. Error Node·Error Type은 복구를 위해 허용하지만 Codegen 입력에는 남지 않는다.
4. 미지원 기능은 추측해서 실행하지 않고 안정적인 진단으로 거부한다.
5. 동일 입력과 옵션은 결정적인 결과를 생성한다.

## 5. 구현 요구사항

1. Platform Abstraction Layer 전용 데이터 모델과 Query 경계를 구현한다.
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

- Platform Abstraction Layer의 정상·오류·경계 사례를 테스트한다.
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


## 9. 변경 절차

범위 변경은 NOVA-006의 변경 제안·영향 분석·결정 기록 절차를 따른다.


---

# foreign 선언 문법·공통 모델 사양서

## Canonical 문법

```nova
foreign c NativeMath {
    library "native_math"

    @symbol("native_add")
    func add(
        first: int32,
        second: int32,
    ) -> int32
}
```

공식 키워드는 `foreign`이다. `external`은 내부 설명 용어로만 사용한다.

## 계층

```text
Raw foreign declaration
→ unsafe adapter
→ safe Nova wrapper
→ application
```

## 규칙

- foreign 함수는 Body가 없다.
- ABI 안전 타입만 직접 사용한다.
- Read·change·take와 Handle Wrapper로 소유권을 표시한다.
- 외부 예외는 직접 통과하지 않는다.
- Owned Handle은 release 함수가 필요하다.

## MVP C Adapter

- Scalar
- Opaque Handle
- Pointer
- 함수 Import·Export
- Callback+Context
- Library·Symbol

C Variadic·C++ 예외·Bitfield 자동 Mapping은 제외한다.


---

# C ABI Import·Export 사양서

| 항목 | 값 |
|---|---|
| 문서 ID | NOVA-106 |
| 대상 | Nova 0.1 |
| 구현 시점 | FFI 전 |
| 상태 | 확정 |
| 갱신일 | 2026-07-27 |

## 1. 목적

이 문서는 **C ABI Import·Export 사양서**에 대한 언어 의미, Compiler 책임, 구현 경계, 오류 처리 및 완료 기준을 정의한다.

이 문서는 해당 구현 단계에 진입하기 전 동결해야 하는 기준이다.

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

1. 공식 키워드는 foreign이다.
2. FFI 경계에서 소유권·ABI·예외 계약을 명시한다.
3. 외부 예외는 Result 또는 Abort로 변환한다.

## 4. 확정 규칙

1. Nova ABI와 C ABI를 구분한다.
2. 큰 반환은 hidden return place를 사용할 수 있다.
3. change는 독점 Pointer로 내린다.

추가 불변 조건:

1. 평가 순서·Drop 순서·소유권 의미는 최적화로 변경하지 않는다.
2. 문자열 이름은 의미 분석 단계에서 안정적 ID로 변환한다.
3. Error Node·Error Type은 복구를 위해 허용하지만 Codegen 입력에는 남지 않는다.
4. 미지원 기능은 추측해서 실행하지 않고 안정적인 진단으로 거부한다.
5. 동일 입력과 옵션은 결정적인 결과를 생성한다.

## 5. 구현 요구사항

1. AbiClass와 PassMode를 계산한다.

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

- Scalar·Aggregate·View·Handle 호출을 테스트한다.

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


## 9. 변경 절차

범위 변경은 NOVA-006의 변경 제안·영향 분석·결정 기록 절차를 따른다.


---

# C Header Parser·Binding Generator 사양서

| 항목 | 값 |
|---|---|
| 문서 ID | NOVA-107 |
| 대상 | Nova 0.1 |
| 구현 시점 | FFI 후반 |
| 상태 | 확정 |
| 갱신일 | 2026-07-27 |

## 1. 목적

이 문서는 **C Header Parser·Binding Generator 사양서**에 대한 언어 의미, Compiler 책임, 구현 경계, 오류 처리 및 완료 기준을 정의한다.

이 문서는 해당 구현 단계에 진입하기 전 동결해야 하는 기준이다.

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

1. 공식 키워드는 foreign이다.
2. FFI 경계에서 소유권·ABI·예외 계약을 명시한다.
3. 외부 예외는 Result 또는 Abort로 변환한다.

## 4. 확정 규칙

1. 선언·문장은 Recursive Descent, 표현식은 Pratt Parser를 사용한다.
2. Error Node와 Synthetic Token으로 복구한다.

추가 불변 조건:

1. 평가 순서·Drop 순서·소유권 의미는 최적화로 변경하지 않는다.
2. 문자열 이름은 의미 분석 단계에서 안정적 ID로 변환한다.
3. Error Node·Error Type은 복구를 위해 허용하지만 Codegen 입력에는 남지 않는다.
4. 미지원 기능은 추측해서 실행하지 않고 안정적인 진단으로 거부한다.
5. 동일 입력과 옵션은 결정적인 결과를 생성한다.

## 5. 구현 요구사항

1. delimiter stack과 동기화 지점을 구현한다.

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

- 불완전 코드와 괄호 누락을 테스트한다.

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


## 9. 변경 절차

범위 변경은 NOVA-006의 변경 제안·영향 분석·결정 기록 절차를 따른다.


---

# C++ Bridge·Opaque Handle 사양서

| 항목 | 값 |
|---|---|
| 문서 ID | NOVA-108 |
| 대상 | Nova 0.1 |
| 구현 시점 | MVP 이후 가능 |
| 상태 | 후속 |
| 갱신일 | 2026-07-27 |

## 1. 목적

이 문서는 **C++ Bridge·Opaque Handle 사양서**에 대한 언어 의미, Compiler 책임, 구현 경계, 오류 처리 및 완료 기준을 정의한다.

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

1. 공식 키워드는 foreign이다.
2. FFI 경계에서 소유권·ABI·예외 계약을 명시한다.
3. 외부 예외는 Result 또는 Abort로 변환한다.

## 4. 확정 규칙

1. C++ Bridge·Opaque Handle의 입력·출력·불변 조건을 명시한다.
2. C++ Bridge·Opaque Handle의 MVP 범위와 후속 범위를 분리한다.
3. C++ Bridge·Opaque Handle의 결과는 컴파일 순서와 병렬 실행 순서에 독립적이어야 한다.

추가 불변 조건:

1. 평가 순서·Drop 순서·소유권 의미는 최적화로 변경하지 않는다.
2. 문자열 이름은 의미 분석 단계에서 안정적 ID로 변환한다.
3. Error Node·Error Type은 복구를 위해 허용하지만 Codegen 입력에는 남지 않는다.
4. 미지원 기능은 추측해서 실행하지 않고 안정적인 진단으로 거부한다.
5. 동일 입력과 옵션은 결정적인 결과를 생성한다.

## 5. 구현 요구사항

1. C++ Bridge·Opaque Handle 전용 데이터 모델과 Query 경계를 구현한다.
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

- C++ Bridge·Opaque Handle의 정상·오류·경계 사례를 테스트한다.
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


---

# 외부 예외→Result 변환 사양서

| 항목 | 값 |
|---|---|
| 문서 ID | NOVA-109 |
| 대상 | Nova 0.1 |
| 구현 시점 | FFI 전 |
| 상태 | 확정 |
| 갱신일 | 2026-07-27 |

## 1. 목적

이 문서는 **외부 예외→Result 변환 사양서**에 대한 언어 의미, Compiler 책임, 구현 경계, 오류 처리 및 완료 기준을 정의한다.

이 문서는 해당 구현 단계에 진입하기 전 동결해야 하는 기준이다.

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

1. 공식 키워드는 foreign이다.
2. FFI 경계에서 소유권·ABI·예외 계약을 명시한다.
3. 외부 예외는 Result 또는 Abort로 변환한다.

## 4. 확정 규칙

1. Result는 Success·Error다.
2. try는 Error를 조기 반환한다.
3. Success(())로 Unit 성공을 표현한다.

추가 불변 조건:

1. 평가 순서·Drop 순서·소유권 의미는 최적화로 변경하지 않는다.
2. 문자열 이름은 의미 분석 단계에서 안정적 ID로 변환한다.
3. Error Node·Error Type은 복구를 위해 허용하지만 Codegen 입력에는 남지 않는다.
4. 미지원 기능은 추측해서 실행하지 않고 안정적인 진단으로 거부한다.
5. 동일 입력과 옵션은 결정적인 결과를 생성한다.

## 5. 구현 요구사항

1. Result API와 try lowering을 구현한다.

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

- 중첩 try·cleanup·payload move를 테스트한다.

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


## 9. 변경 절차

범위 변경은 NOVA-006의 변경 제안·영향 분석·결정 기록 절차를 따른다.


---

# Foreign Ownership·Drop 계약 사양서

| 항목 | 값 |
|---|---|
| 문서 ID | NOVA-110 |
| 대상 | Nova 0.1 |
| 구현 시점 | FFI 전 |
| 상태 | 확정 |
| 갱신일 | 2026-07-27 |

## 1. 목적

이 문서는 **Foreign Ownership·Drop 계약 사양서**에 대한 언어 의미, Compiler 책임, 구현 경계, 오류 처리 및 완료 기준을 정의한다.

이 문서는 해당 구현 단계에 진입하기 전 동결해야 하는 기준이다.

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

1. 공식 키워드는 foreign이다.
2. FFI 경계에서 소유권·ABI·예외 계약을 명시한다.
3. 외부 예외는 Result 또는 Abort로 변환한다.

## 4. 확정 규칙

1. Local과 필드는 선언 역순으로 Drop한다.
2. Moved 값은 원본에서 Drop하지 않는다.
3. 조건부 상태에는 Drop Flag를 사용한다.
4. foreign 블록은 Adapter와 Module 이름을 가진다.
5. Raw 선언과 안전 Wrapper를 분리한다.
6. Owned Handle은 release 계약이 필요하다.

추가 불변 조건:

1. 평가 순서·Drop 순서·소유권 의미는 최적화로 변경하지 않는다.
2. 문자열 이름은 의미 분석 단계에서 안정적 ID로 변환한다.
3. Error Node·Error Type은 복구를 위해 허용하지만 Codegen 입력에는 남지 않는다.
4. 미지원 기능은 추측해서 실행하지 않고 안정적인 진단으로 거부한다.
5. 동일 입력과 옵션은 결정적인 결과를 생성한다.

## 5. 구현 요구사항

1. needsDrop, Drop Glue, Cleanup CFG를 구현한다.
2. C Adapter와 Symbol resolver를 구현한다.

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

- return·break·try·재대입·부분 초기화를 테스트한다.
- ABI·소유권·예외 변환을 테스트한다.

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


## 9. 변경 절차

범위 변경은 NOVA-006의 변경 제안·영향 분석·결정 기록 절차를 따른다.


---

# .NET Hosted Adapter 사양서

| 항목 | 값 |
|---|---|
| 문서 ID | NOVA-111 |
| 대상 | Nova 0.1 |
| 구현 시점 | MVP 이후 |
| 상태 | 후속 |
| 갱신일 | 2026-07-27 |

## 1. 목적

이 문서는 **.NET Hosted Adapter 사양서**에 대한 언어 의미, Compiler 책임, 구현 경계, 오류 처리 및 완료 기준을 정의한다.

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

1. 공식 키워드는 foreign이다.
2. FFI 경계에서 소유권·ABI·예외 계약을 명시한다.
3. 외부 예외는 Result 또는 Abort로 변환한다.

## 4. 확정 규칙

1. .NET Hosted Adapter의 입력·출력·불변 조건을 명시한다.
2. .NET Hosted Adapter의 MVP 범위와 후속 범위를 분리한다.
3. .NET Hosted Adapter의 결과는 컴파일 순서와 병렬 실행 순서에 독립적이어야 한다.

추가 불변 조건:

1. 평가 순서·Drop 순서·소유권 의미는 최적화로 변경하지 않는다.
2. 문자열 이름은 의미 분석 단계에서 안정적 ID로 변환한다.
3. Error Node·Error Type은 복구를 위해 허용하지만 Codegen 입력에는 남지 않는다.
4. 미지원 기능은 추측해서 실행하지 않고 안정적인 진단으로 거부한다.
5. 동일 입력과 옵션은 결정적인 결과를 생성한다.

## 5. 구현 요구사항

1. .NET Hosted Adapter 전용 데이터 모델과 Query 경계를 구현한다.
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

- .NET Hosted Adapter의 정상·오류·경계 사례를 테스트한다.
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


---

# JVM Hosted Adapter 사양서

| 항목 | 값 |
|---|---|
| 문서 ID | NOVA-112 |
| 대상 | Nova 0.1 |
| 구현 시점 | MVP 이후 |
| 상태 | 후속 |
| 갱신일 | 2026-07-27 |

## 1. 목적

이 문서는 **JVM Hosted Adapter 사양서**에 대한 언어 의미, Compiler 책임, 구현 경계, 오류 처리 및 완료 기준을 정의한다.

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

1. 공식 키워드는 foreign이다.
2. FFI 경계에서 소유권·ABI·예외 계약을 명시한다.
3. 외부 예외는 Result 또는 Abort로 변환한다.

## 4. 확정 규칙

1. JVM Hosted Adapter의 입력·출력·불변 조건을 명시한다.
2. JVM Hosted Adapter의 MVP 범위와 후속 범위를 분리한다.
3. JVM Hosted Adapter의 결과는 컴파일 순서와 병렬 실행 순서에 독립적이어야 한다.

추가 불변 조건:

1. 평가 순서·Drop 순서·소유권 의미는 최적화로 변경하지 않는다.
2. 문자열 이름은 의미 분석 단계에서 안정적 ID로 변환한다.
3. Error Node·Error Type은 복구를 위해 허용하지만 Codegen 입력에는 남지 않는다.
4. 미지원 기능은 추측해서 실행하지 않고 안정적인 진단으로 거부한다.
5. 동일 입력과 옵션은 결정적인 결과를 생성한다.

## 5. 구현 요구사항

1. JVM Hosted Adapter 전용 데이터 모델과 Query 경계를 구현한다.
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

- JVM Hosted Adapter의 정상·오류·경계 사례를 테스트한다.
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


---

# Python Hosted Adapter 사양서

| 항목 | 값 |
|---|---|
| 문서 ID | NOVA-113 |
| 대상 | Nova 0.1 |
| 구현 시점 | MVP 이후 |
| 상태 | 후속 |
| 갱신일 | 2026-07-27 |

## 1. 목적

이 문서는 **Python Hosted Adapter 사양서**에 대한 언어 의미, Compiler 책임, 구현 경계, 오류 처리 및 완료 기준을 정의한다.

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

1. 공식 키워드는 foreign이다.
2. FFI 경계에서 소유권·ABI·예외 계약을 명시한다.
3. 외부 예외는 Result 또는 Abort로 변환한다.

## 4. 확정 규칙

1. Python Hosted Adapter의 입력·출력·불변 조건을 명시한다.
2. Python Hosted Adapter의 MVP 범위와 후속 범위를 분리한다.
3. Python Hosted Adapter의 결과는 컴파일 순서와 병렬 실행 순서에 독립적이어야 한다.

추가 불변 조건:

1. 평가 순서·Drop 순서·소유권 의미는 최적화로 변경하지 않는다.
2. 문자열 이름은 의미 분석 단계에서 안정적 ID로 변환한다.
3. Error Node·Error Type은 복구를 위해 허용하지만 Codegen 입력에는 남지 않는다.
4. 미지원 기능은 추측해서 실행하지 않고 안정적인 진단으로 거부한다.
5. 동일 입력과 옵션은 결정적인 결과를 생성한다.

## 5. 구현 요구사항

1. Python Hosted Adapter 전용 데이터 모델과 Query 경계를 구현한다.
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

- Python Hosted Adapter의 정상·오류·경계 사례를 테스트한다.
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


---

# Core Prelude·기본 Symbol 사양서

| 항목 | 값 |
|---|---|
| 문서 ID | NOVA-114 |
| 대상 | Nova 0.1 |
| 구현 시점 | 첫 실행 전 |
| 상태 | 확정 |
| 갱신일 | 2026-07-27 |

## 1. 목적

이 문서는 **Core Prelude·기본 Symbol 사양서**에 대한 언어 의미, Compiler 책임, 구현 경계, 오류 처리 및 완료 기준을 정의한다.

이 문서는 해당 구현 단계에 진입하기 전 동결해야 하는 기준이다.

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

1. Core Prelude는 최소 Symbol만 제공한다.
2. Collection은 초기화·Drop 불변 조건을 유지한다.
3. 문자열은 UTF-8이다.

## 4. 확정 규칙

1. Core Prelude·기본 Symbol의 입력·출력·불변 조건을 명시한다.
2. Core Prelude·기본 Symbol의 MVP 범위와 후속 범위를 분리한다.
3. Core Prelude·기본 Symbol의 결과는 컴파일 순서와 병렬 실행 순서에 독립적이어야 한다.

추가 불변 조건:

1. 평가 순서·Drop 순서·소유권 의미는 최적화로 변경하지 않는다.
2. 문자열 이름은 의미 분석 단계에서 안정적 ID로 변환한다.
3. Error Node·Error Type은 복구를 위해 허용하지만 Codegen 입력에는 남지 않는다.
4. 미지원 기능은 추측해서 실행하지 않고 안정적인 진단으로 거부한다.
5. 동일 입력과 옵션은 결정적인 결과를 생성한다.

## 5. 구현 요구사항

1. Core Prelude·기본 Symbol 전용 데이터 모델과 Query 경계를 구현한다.
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

- Core Prelude·기본 Symbol의 정상·오류·경계 사례를 테스트한다.
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


## 9. 변경 절차

범위 변경은 NOVA-006의 변경 제안·영향 분석·결정 기록 절차를 따른다.


---

# Primitive 메서드 API 사양서

| 항목 | 값 |
|---|---|
| 문서 ID | NOVA-115 |
| 대상 | Nova 0.1 |
| 구현 시점 | Core 구현 전 |
| 상태 | 확정 |
| 갱신일 | 2026-07-27 |

## 1. 목적

이 문서는 **Primitive 메서드 API 사양서**에 대한 언어 의미, Compiler 책임, 구현 경계, 오류 처리 및 완료 기준을 정의한다.

이 문서는 해당 구현 단계에 진입하기 전 동결해야 하는 기준이다.

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

1. Core Prelude는 최소 Symbol만 제공한다.
2. Collection은 초기화·Drop 불변 조건을 유지한다.
3. 문자열은 UTF-8이다.

## 4. 확정 규칙

1. Primitive 메서드 API의 입력·출력·불변 조건을 명시한다.
2. Primitive 메서드 API의 MVP 범위와 후속 범위를 분리한다.
3. Primitive 메서드 API의 결과는 컴파일 순서와 병렬 실행 순서에 독립적이어야 한다.

추가 불변 조건:

1. 평가 순서·Drop 순서·소유권 의미는 최적화로 변경하지 않는다.
2. 문자열 이름은 의미 분석 단계에서 안정적 ID로 변환한다.
3. Error Node·Error Type은 복구를 위해 허용하지만 Codegen 입력에는 남지 않는다.
4. 미지원 기능은 추측해서 실행하지 않고 안정적인 진단으로 거부한다.
5. 동일 입력과 옵션은 결정적인 결과를 생성한다.

## 5. 구현 요구사항

1. Primitive 메서드 API 전용 데이터 모델과 Query 경계를 구현한다.
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

- Primitive 메서드 API의 정상·오류·경계 사례를 테스트한다.
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


## 9. 변경 절차

범위 변경은 NOVA-006의 변경 제안·영향 분석·결정 기록 절차를 따른다.


---

# string·UTF-8 API 사양서

| 항목 | 값 |
|---|---|
| 문서 ID | NOVA-116 |
| 대상 | Nova 0.1 |
| 구현 시점 | 문자열 구현 전 |
| 상태 | 확정 |
| 갱신일 | 2026-07-27 |

## 1. 목적

이 문서는 **string·UTF-8 API 사양서**에 대한 언어 의미, Compiler 책임, 구현 경계, 오류 처리 및 완료 기준을 정의한다.

이 문서는 해당 구현 단계에 진입하기 전 동결해야 하는 기준이다.

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

1. Core Prelude는 최소 Symbol만 제공한다.
2. Collection은 초기화·Drop 불변 조건을 유지한다.
3. 문자열은 UTF-8이다.

## 4. 확정 규칙

1. string은 소유 UTF-8이다.
2. 기본 Index 연산은 제공하지 않는다.
3. 바이트·Scalar·Grapheme API를 구분한다.

추가 불변 조건:

1. 평가 순서·Drop 순서·소유권 의미는 최적화로 변경하지 않는다.
2. 문자열 이름은 의미 분석 단계에서 안정적 ID로 변환한다.
3. Error Node·Error Type은 복구를 위해 허용하지만 Codegen 입력에는 남지 않는다.
4. 미지원 기능은 추측해서 실행하지 않고 안정적인 진단으로 거부한다.
5. 동일 입력과 옵션은 결정적인 결과를 생성한다.

## 5. 구현 요구사항

1. UTF-8 validation과 storage를 구현한다.

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

- 다국어·잘못된 UTF-8·Drop을 테스트한다.

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


## 9. 변경 절차

범위 변경은 NOVA-006의 변경 제안·영향 분석·결정 기록 절차를 따른다.


---

# Array<T> API·메모리 모델 사양서

| 항목 | 값 |
|---|---|
| 문서 ID | NOVA-117 |
| 대상 | Nova 0.1 |
| 구현 시점 | Array 구현 전 |
| 상태 | 확정 |
| 갱신일 | 2026-07-27 |

## 1. 목적

이 문서는 **Array<T> API·메모리 모델 사양서**에 대한 언어 의미, Compiler 책임, 구현 경계, 오류 처리 및 완료 기준을 정의한다.

이 문서는 해당 구현 단계에 진입하기 전 동결해야 하는 기준이다.

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

1. Core Prelude는 최소 Symbol만 제공한다.
2. Collection은 초기화·Drop 불변 조건을 유지한다.
3. 문자열은 UTF-8이다.

## 4. 확정 규칙

1. Array는 길이·용량·초기화 구간을 관리한다.
2. 원소는 정확히 한 번 Drop한다.

추가 불변 조건:

1. 평가 순서·Drop 순서·소유권 의미는 최적화로 변경하지 않는다.
2. 문자열 이름은 의미 분석 단계에서 안정적 ID로 변환한다.
3. Error Node·Error Type은 복구를 위해 허용하지만 Codegen 입력에는 남지 않는다.
4. 미지원 기능은 추측해서 실행하지 않고 안정적인 진단으로 거부한다.
5. 동일 입력과 옵션은 결정적인 결과를 생성한다.

## 5. 구현 요구사항

1. Buffer layout과 reserve helper를 구현한다.

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

- 성장·Move 원소·부분 초기화를 테스트한다.

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


## 9. 변경 절차

범위 변경은 NOVA-006의 변경 제안·영향 분석·결정 기록 절차를 따른다.


---

# Span<T>·ReadOnlySpan<T> 사양서

| 항목 | 값 |
|---|---|
| 문서 ID | NOVA-118 |
| 대상 | Nova 0.1 |
| 구현 시점 | View 구현 전 |
| 상태 | 확정 |
| 갱신일 | 2026-07-27 |

## 1. 목적

이 문서는 **Span<T>·ReadOnlySpan<T> 사양서**에 대한 언어 의미, Compiler 책임, 구현 경계, 오류 처리 및 완료 기준을 정의한다.

이 문서는 해당 구현 단계에 진입하기 전 동결해야 하는 기준이다.

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

1. Core Prelude는 최소 Symbol만 제공한다.
2. Collection은 초기화·Drop 불변 조건을 유지한다.
3. 문자열은 UTF-8이다.

## 4. 확정 규칙

1. ReadOnlySpan은 Read View, Span은 Change View다.
2. 원본보다 오래 살 수 없다.
3. splitAt은 비중첩 계약을 제공한다.

추가 불변 조건:

1. 평가 순서·Drop 순서·소유권 의미는 최적화로 변경하지 않는다.
2. 문자열 이름은 의미 분석 단계에서 안정적 ID로 변환한다.
3. Error Node·Error Type은 복구를 위해 허용하지만 Codegen 입력에는 남지 않는다.
4. 미지원 기능은 추측해서 실행하지 않고 안정적인 진단으로 거부한다.
5. 동일 입력과 옵션은 결정적인 결과를 생성한다.

## 5. 구현 요구사항

1. pointer+length ABI와 region 계약을 구현한다.

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

- slice·split·borrow conflict를 테스트한다.

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


## 9. 변경 절차

범위 변경은 NOVA-006의 변경 제안·영향 분석·결정 기록 절차를 따른다.


---

# Option<T> API 사양서

| 항목 | 값 |
|---|---|
| 문서 ID | NOVA-119 |
| 대상 | Nova 0.1 |
| 구현 시점 | Match 구현 전 |
| 상태 | 확정 |
| 갱신일 | 2026-07-27 |

## 1. 목적

이 문서는 **Option<T> API 사양서**에 대한 언어 의미, Compiler 책임, 구현 경계, 오류 처리 및 완료 기준을 정의한다.

이 문서는 해당 구현 단계에 진입하기 전 동결해야 하는 기준이다.

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

1. Core Prelude는 최소 Symbol만 제공한다.
2. Collection은 초기화·Drop 불변 조건을 유지한다.
3. 문자열은 UTF-8이다.

## 4. 확정 규칙

1. Option은 Some·None이다.
2. T?는 Option<T> Sugar다.
3. Niche는 내부 최적화다.

추가 불변 조건:

1. 평가 순서·Drop 순서·소유권 의미는 최적화로 변경하지 않는다.
2. 문자열 이름은 의미 분석 단계에서 안정적 ID로 변환한다.
3. Error Node·Error Type은 복구를 위해 허용하지만 Codegen 입력에는 남지 않는다.
4. 미지원 기능은 추측해서 실행하지 않고 안정적인 진단으로 거부한다.
5. 동일 입력과 옵션은 결정적인 결과를 생성한다.

## 5. 구현 요구사항

1. Option API와 match integration을 구현한다.

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

- Copy·Move·Drop payload를 테스트한다.

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


## 9. 변경 절차

범위 변경은 NOVA-006의 변경 제안·영향 분석·결정 기록 절차를 따른다.


---

# Result<T,E>·try API 사양서

| 항목 | 값 |
|---|---|
| 문서 ID | NOVA-120 |
| 대상 | Nova 0.1 |
| 구현 시점 | Error 처리 전 |
| 상태 | 확정 |
| 갱신일 | 2026-07-27 |

## 1. 목적

이 문서는 **Result<T,E>·try API 사양서**에 대한 언어 의미, Compiler 책임, 구현 경계, 오류 처리 및 완료 기준을 정의한다.

이 문서는 해당 구현 단계에 진입하기 전 동결해야 하는 기준이다.

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

1. Core Prelude는 최소 Symbol만 제공한다.
2. Collection은 초기화·Drop 불변 조건을 유지한다.
3. 문자열은 UTF-8이다.

## 4. 확정 규칙

1. Result는 Success·Error다.
2. try는 Error를 조기 반환한다.
3. Success(())로 Unit 성공을 표현한다.

추가 불변 조건:

1. 평가 순서·Drop 순서·소유권 의미는 최적화로 변경하지 않는다.
2. 문자열 이름은 의미 분석 단계에서 안정적 ID로 변환한다.
3. Error Node·Error Type은 복구를 위해 허용하지만 Codegen 입력에는 남지 않는다.
4. 미지원 기능은 추측해서 실행하지 않고 안정적인 진단으로 거부한다.
5. 동일 입력과 옵션은 결정적인 결과를 생성한다.

## 5. 구현 요구사항

1. Result API와 try lowering을 구현한다.

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

- 중첩 try·cleanup·payload move를 테스트한다.

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


## 9. 변경 절차

범위 변경은 NOVA-006의 변경 제안·영향 분석·결정 기록 절차를 따른다.


---

# List<T>·Map<K,V> 사양서

| 항목 | 값 |
|---|---|
| 문서 ID | NOVA-121 |
| 대상 | Nova 0.1 |
| 구현 시점 | 기본 실행 후 |
| 상태 | 확정 |
| 갱신일 | 2026-07-27 |

## 1. 목적

이 문서는 **List<T>·Map<K,V> 사양서**에 대한 언어 의미, Compiler 책임, 구현 경계, 오류 처리 및 완료 기준을 정의한다.

이 문서는 해당 구현 단계에 진입하기 전 동결해야 하는 기준이다.

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

1. Core Prelude는 최소 Symbol만 제공한다.
2. Collection은 초기화·Drop 불변 조건을 유지한다.
3. 문자열은 UTF-8이다.

## 4. 확정 규칙

1. List<T>·Map<K,V>의 입력·출력·불변 조건을 명시한다.
2. List<T>·Map<K,V>의 MVP 범위와 후속 범위를 분리한다.
3. List<T>·Map<K,V>의 결과는 컴파일 순서와 병렬 실행 순서에 독립적이어야 한다.

추가 불변 조건:

1. 평가 순서·Drop 순서·소유권 의미는 최적화로 변경하지 않는다.
2. 문자열 이름은 의미 분석 단계에서 안정적 ID로 변환한다.
3. Error Node·Error Type은 복구를 위해 허용하지만 Codegen 입력에는 남지 않는다.
4. 미지원 기능은 추측해서 실행하지 않고 안정적인 진단으로 거부한다.
5. 동일 입력과 옵션은 결정적인 결과를 생성한다.

## 5. 구현 요구사항

1. List<T>·Map<K,V> 전용 데이터 모델과 Query 경계를 구현한다.
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

- List<T>·Map<K,V>의 정상·오류·경계 사례를 테스트한다.
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


## 9. 변경 절차

범위 변경은 NOVA-006의 변경 제안·영향 분석·결정 기록 절차를 따른다.


---

# 파일·Stream·Console I/O 사양서

| 항목 | 값 |
|---|---|
| 문서 ID | NOVA-122 |
| 대상 | Nova 0.1 |
| 구현 시점 | 실행 프로그램 전 |
| 상태 | 확정 |
| 갱신일 | 2026-07-27 |

## 1. 목적

이 문서는 **파일·Stream·Console I/O 사양서**에 대한 언어 의미, Compiler 책임, 구현 경계, 오류 처리 및 완료 기준을 정의한다.

이 문서는 해당 구현 단계에 진입하기 전 동결해야 하는 기준이다.

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

1. Core Prelude는 최소 Symbol만 제공한다.
2. Collection은 초기화·Drop 불변 조건을 유지한다.
3. 문자열은 UTF-8이다.

## 4. 확정 규칙

1. 파일·Stream·Console I/O의 입력·출력·불변 조건을 명시한다.
2. 파일·Stream·Console I/O의 MVP 범위와 후속 범위를 분리한다.
3. 파일·Stream·Console I/O의 결과는 컴파일 순서와 병렬 실행 순서에 독립적이어야 한다.

추가 불변 조건:

1. 평가 순서·Drop 순서·소유권 의미는 최적화로 변경하지 않는다.
2. 문자열 이름은 의미 분석 단계에서 안정적 ID로 변환한다.
3. Error Node·Error Type은 복구를 위해 허용하지만 Codegen 입력에는 남지 않는다.
4. 미지원 기능은 추측해서 실행하지 않고 안정적인 진단으로 거부한다.
5. 동일 입력과 옵션은 결정적인 결과를 생성한다.

## 5. 구현 요구사항

1. 파일·Stream·Console I/O 전용 데이터 모델과 Query 경계를 구현한다.
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

- 파일·Stream·Console I/O의 정상·오류·경계 사례를 테스트한다.
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


## 9. 변경 절차

범위 변경은 NOVA-006의 변경 제안·영향 분석·결정 기록 절차를 따른다.


---

# 시간·난수·환경 변수 사양서

| 항목 | 값 |
|---|---|
| 문서 ID | NOVA-123 |
| 대상 | Nova 0.1 |
| 구현 시점 | MVP 후반 |
| 상태 | 확정 |
| 갱신일 | 2026-07-27 |

## 1. 목적

이 문서는 **시간·난수·환경 변수 사양서**에 대한 언어 의미, Compiler 책임, 구현 경계, 오류 처리 및 완료 기준을 정의한다.

이 문서는 해당 구현 단계에 진입하기 전 동결해야 하는 기준이다.

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

1. Core Prelude는 최소 Symbol만 제공한다.
2. Collection은 초기화·Drop 불변 조건을 유지한다.
3. 문자열은 UTF-8이다.

## 4. 확정 규칙

1. 시간·난수·환경 변수의 입력·출력·불변 조건을 명시한다.
2. 시간·난수·환경 변수의 MVP 범위와 후속 범위를 분리한다.
3. 시간·난수·환경 변수의 결과는 컴파일 순서와 병렬 실행 순서에 독립적이어야 한다.

추가 불변 조건:

1. 평가 순서·Drop 순서·소유권 의미는 최적화로 변경하지 않는다.
2. 문자열 이름은 의미 분석 단계에서 안정적 ID로 변환한다.
3. Error Node·Error Type은 복구를 위해 허용하지만 Codegen 입력에는 남지 않는다.
4. 미지원 기능은 추측해서 실행하지 않고 안정적인 진단으로 거부한다.
5. 동일 입력과 옵션은 결정적인 결과를 생성한다.

## 5. 구현 요구사항

1. 시간·난수·환경 변수 전용 데이터 모델과 Query 경계를 구현한다.
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

- 시간·난수·환경 변수의 정상·오류·경계 사례를 테스트한다.
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


## 9. 변경 절차

범위 변경은 NOVA-006의 변경 제안·영향 분석·결정 기록 절차를 따른다.


---

# Thread·Lock·Atomic API 사양서

| 항목 | 값 |
|---|---|
| 문서 ID | NOVA-124 |
| 대상 | Nova 0.1 |
| 구현 시점 | MVP 이후 가능 |
| 상태 | 후속 |
| 갱신일 | 2026-07-27 |

## 1. 목적

이 문서는 **Thread·Lock·Atomic API 사양서**에 대한 언어 의미, Compiler 책임, 구현 경계, 오류 처리 및 완료 기준을 정의한다.

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

1. Core Prelude는 최소 Symbol만 제공한다.
2. Collection은 초기화·Drop 불변 조건을 유지한다.
3. 문자열은 UTF-8이다.

## 4. 확정 규칙

1. Thread·Lock·Atomic API의 입력·출력·불변 조건을 명시한다.
2. Thread·Lock·Atomic API의 MVP 범위와 후속 범위를 분리한다.
3. Thread·Lock·Atomic API의 결과는 컴파일 순서와 병렬 실행 순서에 독립적이어야 한다.

추가 불변 조건:

1. 평가 순서·Drop 순서·소유권 의미는 최적화로 변경하지 않는다.
2. 문자열 이름은 의미 분석 단계에서 안정적 ID로 변환한다.
3. Error Node·Error Type은 복구를 위해 허용하지만 Codegen 입력에는 남지 않는다.
4. 미지원 기능은 추측해서 실행하지 않고 안정적인 진단으로 거부한다.
5. 동일 입력과 옵션은 결정적인 결과를 생성한다.

## 5. 구현 요구사항

1. Thread·Lock·Atomic API 전용 데이터 모델과 Query 경계를 구현한다.
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

- Thread·Lock·Atomic API의 정상·오류·경계 사례를 테스트한다.
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


---

# Compiler CLI 명령·옵션 사양서

## 명령

```text
nova check
nova build
nova run
nova test
nova fmt
nova clean
nova doc
nova version
```

## 공통 옵션

```text
--manifest-path
--target
--profile debug|release|size
--color auto|always|never
--message-format human|json
--jobs
-v -vv
```

## Dump

```text
--emit=tokens|ast|hir|typed-hir|mir|llvm|obj|asm
```

## Exit Code

```text
0 성공
1 사용자 코드 오류
2 CLI·Manifest 오류
3 Toolchain·Linker 오류
101 ICE
```

Manifest 없는 `.nova` 파일은 임시 Package로 실행한다.


---

# Package Manifest 사양서

| 항목 | 값 |
|---|---|
| 문서 ID | NOVA-126 |
| 대상 | Nova 0.1 |
| 구현 시점 | 다중 패키지 전 |
| 상태 | 확정 |
| 갱신일 | 2026-07-27 |

## 1. 목적

이 문서는 **Package Manifest 사양서**에 대한 언어 의미, Compiler 책임, 구현 경계, 오류 처리 및 완료 기준을 정의한다.

이 문서는 해당 구현 단계에 진입하기 전 동결해야 하는 기준이다.

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

1. Manifest는 nova.toml이다.
2. Package·Target·Dependency·Profile을 정의한다.
3. Schema version을 가진다.

추가 불변 조건:

1. 평가 순서·Drop 순서·소유권 의미는 최적화로 변경하지 않는다.
2. 문자열 이름은 의미 분석 단계에서 안정적 ID로 변환한다.
3. Error Node·Error Type은 복구를 위해 허용하지만 Codegen 입력에는 남지 않는다.
4. 미지원 기능은 추측해서 실행하지 않고 안정적인 진단으로 거부한다.
5. 동일 입력과 옵션은 결정적인 결과를 생성한다.

## 5. 구현 요구사항

1. TOML parser와 validator를 구현한다.

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

- 최소·오류·중복 dependency를 테스트한다.

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


## 9. 변경 절차

범위 변경은 NOVA-006의 변경 제안·영향 분석·결정 기록 절차를 따른다.


---

# 의존성 해석·Lock File 사양서

| 항목 | 값 |
|---|---|
| 문서 ID | NOVA-127 |
| 대상 | Nova 0.1 |
| 구현 시점 | 패키지 관리자 전 |
| 상태 | 확정 |
| 갱신일 | 2026-07-27 |

## 1. 목적

이 문서는 **의존성 해석·Lock File 사양서**에 대한 언어 의미, Compiler 책임, 구현 경계, 오류 처리 및 완료 기준을 정의한다.

이 문서는 해당 구현 단계에 진입하기 전 동결해야 하는 기준이다.

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

1. Lock file은 nova.lock이다.
2. 정확한 버전·source·checksum을 기록한다.
3. 재현 가능한 의존성 그래프를 보장한다.

추가 불변 조건:

1. 평가 순서·Drop 순서·소유권 의미는 최적화로 변경하지 않는다.
2. 문자열 이름은 의미 분석 단계에서 안정적 ID로 변환한다.
3. Error Node·Error Type은 복구를 위해 허용하지만 Codegen 입력에는 남지 않는다.
4. 미지원 기능은 추측해서 실행하지 않고 안정적인 진단으로 거부한다.
5. 동일 입력과 옵션은 결정적인 결과를 생성한다.

## 5. 구현 요구사항

1. Resolver와 serializer를 구현한다.

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

- 충돌·offline·checksum을 테스트한다.

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


## 9. 변경 절차

범위 변경은 NOVA-006의 변경 제안·영향 분석·결정 기록 절차를 따른다.


---

# Build Profile·Target 사양서

| 항목 | 값 |
|---|---|
| 문서 ID | NOVA-128 |
| 대상 | Nova 0.1 |
| 구현 시점 | Backend 전 |
| 상태 | 확정 |
| 갱신일 | 2026-07-27 |

## 1. 목적

이 문서는 **Build Profile·Target 사양서**에 대한 언어 의미, Compiler 책임, 구현 경계, 오류 처리 및 완료 기준을 정의한다.

이 문서는 해당 구현 단계에 진입하기 전 동결해야 하는 기준이다.

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

1. Build Profile·Target의 입력·출력·불변 조건을 명시한다.
2. Build Profile·Target의 MVP 범위와 후속 범위를 분리한다.
3. Build Profile·Target의 결과는 컴파일 순서와 병렬 실행 순서에 독립적이어야 한다.

추가 불변 조건:

1. 평가 순서·Drop 순서·소유권 의미는 최적화로 변경하지 않는다.
2. 문자열 이름은 의미 분석 단계에서 안정적 ID로 변환한다.
3. Error Node·Error Type은 복구를 위해 허용하지만 Codegen 입력에는 남지 않는다.
4. 미지원 기능은 추측해서 실행하지 않고 안정적인 진단으로 거부한다.
5. 동일 입력과 옵션은 결정적인 결과를 생성한다.

## 5. 구현 요구사항

1. Build Profile·Target 전용 데이터 모델과 Query 경계를 구현한다.
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

- Build Profile·Target의 정상·오류·경계 사례를 테스트한다.
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


## 9. 변경 절차

범위 변경은 NOVA-006의 변경 제안·영향 분석·결정 기록 절차를 따른다.


---

# Package Artifact·Metadata 형식 사양서

| 항목 | 값 |
|---|---|
| 문서 ID | NOVA-129 |
| 대상 | Nova 0.1 |
| 구현 시점 | 외부 패키지 전 |
| 상태 | 확정 |
| 갱신일 | 2026-07-27 |

## 1. 목적

이 문서는 **Package Artifact·Metadata 형식 사양서**에 대한 언어 의미, Compiler 책임, 구현 경계, 오류 처리 및 완료 기준을 정의한다.

이 문서는 해당 구현 단계에 진입하기 전 동결해야 하는 기준이다.

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

1. Package Artifact·Metadata 형식의 입력·출력·불변 조건을 명시한다.
2. Package Artifact·Metadata 형식의 MVP 범위와 후속 범위를 분리한다.
3. Package Artifact·Metadata 형식의 결과는 컴파일 순서와 병렬 실행 순서에 독립적이어야 한다.

추가 불변 조건:

1. 평가 순서·Drop 순서·소유권 의미는 최적화로 변경하지 않는다.
2. 문자열 이름은 의미 분석 단계에서 안정적 ID로 변환한다.
3. Error Node·Error Type은 복구를 위해 허용하지만 Codegen 입력에는 남지 않는다.
4. 미지원 기능은 추측해서 실행하지 않고 안정적인 진단으로 거부한다.
5. 동일 입력과 옵션은 결정적인 결과를 생성한다.

## 5. 구현 요구사항

1. Package Artifact·Metadata 형식 전용 데이터 모델과 Query 경계를 구현한다.
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

- Package Artifact·Metadata 형식의 정상·오류·경계 사례를 테스트한다.
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


## 9. 변경 절차

범위 변경은 NOVA-006의 변경 제안·영향 분석·결정 기록 절차를 따른다.


---

# 증분 컴파일 Cache 형식 사양서

| 항목 | 값 |
|---|---|
| 문서 ID | NOVA-130 |
| 대상 | Nova 0.1 |
| 구현 시점 | 증분 기능 전 |
| 상태 | 확정 |
| 갱신일 | 2026-07-27 |

## 1. 목적

이 문서는 **증분 컴파일 Cache 형식 사양서**에 대한 언어 의미, Compiler 책임, 구현 경계, 오류 처리 및 완료 기준을 정의한다.

이 문서는 해당 구현 단계에 진입하기 전 동결해야 하는 기준이다.

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

1. 증분 컴파일 Cache 형식의 입력·출력·불변 조건을 명시한다.
2. 증분 컴파일 Cache 형식의 MVP 범위와 후속 범위를 분리한다.
3. 증분 컴파일 Cache 형식의 결과는 컴파일 순서와 병렬 실행 순서에 독립적이어야 한다.

추가 불변 조건:

1. 평가 순서·Drop 순서·소유권 의미는 최적화로 변경하지 않는다.
2. 문자열 이름은 의미 분석 단계에서 안정적 ID로 변환한다.
3. Error Node·Error Type은 복구를 위해 허용하지만 Codegen 입력에는 남지 않는다.
4. 미지원 기능은 추측해서 실행하지 않고 안정적인 진단으로 거부한다.
5. 동일 입력과 옵션은 결정적인 결과를 생성한다.

## 5. 구현 요구사항

1. 증분 컴파일 Cache 형식 전용 데이터 모델과 Query 경계를 구현한다.
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

- 증분 컴파일 Cache 형식의 정상·오류·경계 사례를 테스트한다.
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


## 9. 변경 절차

범위 변경은 NOVA-006의 변경 제안·영향 분석·결정 기록 절차를 따른다.


---

# Formatter 사양·구현서

| 항목 | 값 |
|---|---|
| 문서 ID | NOVA-131 |
| 대상 | Nova 0.1 |
| 구현 시점 | Parser 안정 후 |
| 상태 | 확정 |
| 갱신일 | 2026-07-27 |

## 1. 목적

이 문서는 **Formatter 사양·구현서**에 대한 언어 의미, Compiler 책임, 구현 경계, 오류 처리 및 완료 기준을 정의한다.

이 문서는 해당 구현 단계에 진입하기 전 동결해야 하는 기준이다.

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

1. Formatter는 단일 Canonical 출력을 만든다.
2. 기본 줄 길이는 100이다.
3. 주석을 보존한다.

추가 불변 조건:

1. 평가 순서·Drop 순서·소유권 의미는 최적화로 변경하지 않는다.
2. 문자열 이름은 의미 분석 단계에서 안정적 ID로 변환한다.
3. Error Node·Error Type은 복구를 위해 허용하지만 Codegen 입력에는 남지 않는다.
4. 미지원 기능은 추측해서 실행하지 않고 안정적인 진단으로 거부한다.
5. 동일 입력과 옵션은 결정적인 결과를 생성한다.

## 5. 구현 요구사항

1. AST+Trivia 기반 document model을 구현한다.

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

- Idempotence와 긴 구문을 테스트한다.

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


## 9. 변경 절차

범위 변경은 NOVA-006의 변경 제안·영향 분석·결정 기록 절차를 따른다.


---

# Linter 규칙 사양서

| 항목 | 값 |
|---|---|
| 문서 ID | NOVA-132 |
| 대상 | Nova 0.1 |
| 구현 시점 | MVP 후반 |
| 상태 | 확정 |
| 갱신일 | 2026-07-27 |

## 1. 목적

이 문서는 **Linter 규칙 사양서**에 대한 언어 의미, Compiler 책임, 구현 경계, 오류 처리 및 완료 기준을 정의한다.

이 문서는 해당 구현 단계에 진입하기 전 동결해야 하는 기준이다.

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

1. Lint는 allow·warn·deny 수준을 가진다.
2. 안전한 경우만 자동 수정한다.

추가 불변 조건:

1. 평가 순서·Drop 순서·소유권 의미는 최적화로 변경하지 않는다.
2. 문자열 이름은 의미 분석 단계에서 안정적 ID로 변환한다.
3. Error Node·Error Type은 복구를 위해 허용하지만 Codegen 입력에는 남지 않는다.
4. 미지원 기능은 추측해서 실행하지 않고 안정적인 진단으로 거부한다.
5. 동일 입력과 옵션은 결정적인 결과를 생성한다.

## 5. 구현 요구사항

1. LintId와 scope level을 구현한다.

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

- Suppression·fix·unused를 테스트한다.

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


## 9. 변경 절차

범위 변경은 NOVA-006의 변경 제안·영향 분석·결정 기록 절차를 따른다.


---

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


---

# API 문서 생성기 사양서

| 항목 | 값 |
|---|---|
| 문서 ID | NOVA-134 |
| 대상 | Nova 0.1 |
| 구현 시점 | MVP 이후 |
| 상태 | 후속 |
| 갱신일 | 2026-07-27 |

## 1. 목적

이 문서는 **API 문서 생성기 사양서**에 대한 언어 의미, Compiler 책임, 구현 경계, 오류 처리 및 완료 기준을 정의한다.

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

1. API 문서 생성기의 입력·출력·불변 조건을 명시한다.
2. API 문서 생성기의 MVP 범위와 후속 범위를 분리한다.
3. API 문서 생성기의 결과는 컴파일 순서와 병렬 실행 순서에 독립적이어야 한다.

추가 불변 조건:

1. 평가 순서·Drop 순서·소유권 의미는 최적화로 변경하지 않는다.
2. 문자열 이름은 의미 분석 단계에서 안정적 ID로 변환한다.
3. Error Node·Error Type은 복구를 위해 허용하지만 Codegen 입력에는 남지 않는다.
4. 미지원 기능은 추측해서 실행하지 않고 안정적인 진단으로 거부한다.
5. 동일 입력과 옵션은 결정적인 결과를 생성한다.

## 5. 구현 요구사항

1. API 문서 생성기 전용 데이터 모델과 Query 경계를 구현한다.
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

- API 문서 생성기의 정상·오류·경계 사례를 테스트한다.
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


---

# Package Registry 사양서

| 항목 | 값 |
|---|---|
| 문서 ID | NOVA-135 |
| 대상 | Nova 0.1 |
| 구현 시점 | MVP 이후 |
| 상태 | 후속 |
| 갱신일 | 2026-07-27 |

## 1. 목적

이 문서는 **Package Registry 사양서**에 대한 언어 의미, Compiler 책임, 구현 경계, 오류 처리 및 완료 기준을 정의한다.

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

1. Package Registry의 입력·출력·불변 조건을 명시한다.
2. Package Registry의 MVP 범위와 후속 범위를 분리한다.
3. Package Registry의 결과는 컴파일 순서와 병렬 실행 순서에 독립적이어야 한다.

추가 불변 조건:

1. 평가 순서·Drop 순서·소유권 의미는 최적화로 변경하지 않는다.
2. 문자열 이름은 의미 분석 단계에서 안정적 ID로 변환한다.
3. Error Node·Error Type은 복구를 위해 허용하지만 Codegen 입력에는 남지 않는다.
4. 미지원 기능은 추측해서 실행하지 않고 안정적인 진단으로 거부한다.
5. 동일 입력과 옵션은 결정적인 결과를 생성한다.

## 5. 구현 요구사항

1. Package Registry 전용 데이터 모델과 Query 경계를 구현한다.
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

- Package Registry의 정상·오류·경계 사례를 테스트한다.
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


---

# Compiler 테스트 전략서

## 계층

```text
Unit
Snapshot
Compile-pass
Compile-fail
MIR validation
Runtime integration
End-to-end
Fuzz
Benchmark
Compatibility
```

## 디렉터리

```text
tests/
├─ lexer
├─ parser
├─ hir
├─ resolve
├─ typecheck
├─ ownership
├─ mir
├─ codegen
├─ runtime
├─ ui
├─ e2e
├─ fuzz
└─ benchmarks
```

## Compile-fail

기대 Error Code와 Span을 검증한다.

```nova
consume(take image)
display(image)
//~^ N4101
```

## E2E

- Hello Nova
- 산술·분기
- Struct·Enum
- Option·Result
- Move·Drop
- Generic·Interface
- C FFI

## 회귀

모든 Crash와 잘못된 코드 허용 사례는 최소 재현 테스트를 추가한 뒤 수정한다.


---

# Lexer·Parser Snapshot 테스트 사양서

| 항목 | 값 |
|---|---|
| 문서 ID | NOVA-137 |
| 대상 | Nova 0.1 |
| 구현 시점 | Lexer와 병행 |
| 상태 | 확정 |
| 갱신일 | 2026-07-27 |

## 1. 목적

이 문서는 **Lexer·Parser Snapshot 테스트 사양서**에 대한 언어 의미, Compiler 책임, 구현 경계, 오류 처리 및 완료 기준을 정의한다.

이 문서는 해당 구현 단계에 진입하기 전 동결해야 하는 기준이다.

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

1. 성공·실패·경계·회귀 테스트를 둔다.
2. 진단 Snapshot은 Code와 Span을 검증한다.
3. Fuzzing은 Crash·Hang·비결정성을 실패로 본다.

## 4. 확정 규칙

1. 최장 일치로 Token을 생성한다.
2. Trivia와 Token을 분리하되 소스 재구성이 가능해야 한다.
3. 문자열·주석·보간은 별도 Mode로 처리한다.
4. 선언·문장은 Recursive Descent, 표현식은 Pratt Parser를 사용한다.
5. Error Node와 Synthetic Token으로 복구한다.
6. Unit·Snapshot·Compile-pass·Compile-fail·E2E를 계층화한다.
7. 모든 회귀는 최소 재현 테스트를 남긴다.

추가 불변 조건:

1. 평가 순서·Drop 순서·소유권 의미는 최적화로 변경하지 않는다.
2. 문자열 이름은 의미 분석 단계에서 안정적 ID로 변환한다.
3. Error Node·Error Type은 복구를 위해 허용하지만 Codegen 입력에는 남지 않는다.
4. 미지원 기능은 추측해서 실행하지 않고 안정적인 진단으로 거부한다.
5. 동일 입력과 옵션은 결정적인 결과를 생성한다.

## 5. 구현 요구사항

1. TokenKind, cursor, mode stack, diagnostics를 구현한다.
2. delimiter stack과 동기화 지점을 구현한다.
3. Test harness와 fixture 규칙을 구현한다.

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

- Unicode, 중첩 주석, 보간, 잘못된 문자를 테스트한다.
- 불완전 코드와 괄호 누락을 테스트한다.
- 병렬 격리와 target 차이를 테스트한다.

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


## 9. 변경 절차

범위 변경은 NOVA-006의 변경 제안·영향 분석·결정 기록 절차를 따른다.


---

# Compile-pass·Compile-fail 규격

| 항목 | 값 |
|---|---|
| 문서 ID | NOVA-138 |
| 대상 | Nova 0.1 |
| 구현 시점 | 타입 검사 전 |
| 상태 | 확정 |
| 갱신일 | 2026-07-27 |

## 1. 목적

이 문서는 **Compile-pass·Compile-fail 규격**에 대한 언어 의미, Compiler 책임, 구현 경계, 오류 처리 및 완료 기준을 정의한다.

이 문서는 해당 구현 단계에 진입하기 전 동결해야 하는 기준이다.

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

1. 성공·실패·경계·회귀 테스트를 둔다.
2. 진단 Snapshot은 Code와 Span을 검증한다.
3. Fuzzing은 Crash·Hang·비결정성을 실패로 본다.

## 4. 확정 규칙

1. Compile-pass·Compile-fail 규격의 입력·출력·불변 조건을 명시한다.
2. Compile-pass·Compile-fail 규격의 MVP 범위와 후속 범위를 분리한다.
3. Compile-pass·Compile-fail 규격의 결과는 컴파일 순서와 병렬 실행 순서에 독립적이어야 한다.

추가 불변 조건:

1. 평가 순서·Drop 순서·소유권 의미는 최적화로 변경하지 않는다.
2. 문자열 이름은 의미 분석 단계에서 안정적 ID로 변환한다.
3. Error Node·Error Type은 복구를 위해 허용하지만 Codegen 입력에는 남지 않는다.
4. 미지원 기능은 추측해서 실행하지 않고 안정적인 진단으로 거부한다.
5. 동일 입력과 옵션은 결정적인 결과를 생성한다.

## 5. 구현 요구사항

1. Compile-pass·Compile-fail 규격 전용 데이터 모델과 Query 경계를 구현한다.
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

- Compile-pass·Compile-fail 규격의 정상·오류·경계 사례를 테스트한다.
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


## 9. 변경 절차

범위 변경은 NOVA-006의 변경 제안·영향 분석·결정 기록 절차를 따른다.


---

# MIR Snapshot 규격

| 항목 | 값 |
|---|---|
| 문서 ID | NOVA-139 |
| 대상 | Nova 0.1 |
| 구현 시점 | MIR 전 |
| 상태 | 확정 |
| 갱신일 | 2026-07-27 |

## 1. 목적

이 문서는 **MIR Snapshot 규격**에 대한 언어 의미, Compiler 책임, 구현 경계, 오류 처리 및 완료 기준을 정의한다.

이 문서는 해당 구현 단계에 진입하기 전 동결해야 하는 기준이다.

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

1. 성공·실패·경계·회귀 테스트를 둔다.
2. 진단 Snapshot은 Code와 Span을 검증한다.
3. Fuzzing은 Crash·Hang·비결정성을 실패로 본다.

## 4. 확정 규칙

1. MIR은 Place·Operand·Rvalue·Statement·Terminator로 구성한다.
2. Call과 Drop은 Terminator다.
3. 평가 순서를 명시한다.

추가 불변 조건:

1. 평가 순서·Drop 순서·소유권 의미는 최적화로 변경하지 않는다.
2. 문자열 이름은 의미 분석 단계에서 안정적 ID로 변환한다.
3. Error Node·Error Type은 복구를 위해 허용하지만 Codegen 입력에는 남지 않는다.
4. 미지원 기능은 추측해서 실행하지 않고 안정적인 진단으로 거부한다.
5. 동일 입력과 옵션은 결정적인 결과를 생성한다.

## 5. 구현 요구사항

1. BasicBlockData와 MIR validator를 구현한다.

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

- CFG·Cleanup·Dump를 테스트한다.

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


## 9. 변경 절차

범위 변경은 NOVA-006의 변경 제안·영향 분석·결정 기록 절차를 따른다.


---

# Runtime Test Harness 사양서

| 항목 | 값 |
|---|---|
| 문서 ID | NOVA-140 |
| 대상 | Nova 0.1 |
| 구현 시점 | Runtime 전 |
| 상태 | 확정 |
| 갱신일 | 2026-07-27 |

## 1. 목적

이 문서는 **Runtime Test Harness 사양서**에 대한 언어 의미, Compiler 책임, 구현 경계, 오류 처리 및 완료 기준을 정의한다.

이 문서는 해당 구현 단계에 진입하기 전 동결해야 하는 기준이다.

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

1. 성공·실패·경계·회귀 테스트를 둔다.
2. 진단 Snapshot은 Code와 Span을 검증한다.
3. Fuzzing은 Crash·Hang·비결정성을 실패로 본다.

## 4. 확정 규칙

1. Runtime Test Harness의 입력·출력·불변 조건을 명시한다.
2. Runtime Test Harness의 MVP 범위와 후속 범위를 분리한다.
3. Runtime Test Harness의 결과는 컴파일 순서와 병렬 실행 순서에 독립적이어야 한다.

추가 불변 조건:

1. 평가 순서·Drop 순서·소유권 의미는 최적화로 변경하지 않는다.
2. 문자열 이름은 의미 분석 단계에서 안정적 ID로 변환한다.
3. Error Node·Error Type은 복구를 위해 허용하지만 Codegen 입력에는 남지 않는다.
4. 미지원 기능은 추측해서 실행하지 않고 안정적인 진단으로 거부한다.
5. 동일 입력과 옵션은 결정적인 결과를 생성한다.

## 5. 구현 요구사항

1. Runtime Test Harness 전용 데이터 모델과 Query 경계를 구현한다.
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

- Runtime Test Harness의 정상·오류·경계 사례를 테스트한다.
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


## 9. 변경 절차

범위 변경은 NOVA-006의 변경 제안·영향 분석·결정 기록 절차를 따른다.


---

# Lexer·Parser·MIR Fuzzing 사양서

| 항목 | 값 |
|---|---|
| 문서 ID | NOVA-141 |
| 대상 | Nova 0.1 |
| 구현 시점 | 각 단계 병행 |
| 상태 | 확정 |
| 갱신일 | 2026-07-27 |

## 1. 목적

이 문서는 **Lexer·Parser·MIR Fuzzing 사양서**에 대한 언어 의미, Compiler 책임, 구현 경계, 오류 처리 및 완료 기준을 정의한다.

이 문서는 해당 구현 단계에 진입하기 전 동결해야 하는 기준이다.

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

1. 성공·실패·경계·회귀 테스트를 둔다.
2. 진단 Snapshot은 Code와 Span을 검증한다.
3. Fuzzing은 Crash·Hang·비결정성을 실패로 본다.

## 4. 확정 규칙

1. 최장 일치로 Token을 생성한다.
2. Trivia와 Token을 분리하되 소스 재구성이 가능해야 한다.
3. 문자열·주석·보간은 별도 Mode로 처리한다.
4. 선언·문장은 Recursive Descent, 표현식은 Pratt Parser를 사용한다.
5. Error Node와 Synthetic Token으로 복구한다.
6. MIR은 Place·Operand·Rvalue·Statement·Terminator로 구성한다.
7. Call과 Drop은 Terminator다.
8. 평가 순서를 명시한다.
9. Token·AST·MIR 단계별 Fuzzer를 둔다.
10. Crash·Hang·ICE·비결정성은 실패다.

추가 불변 조건:

1. 평가 순서·Drop 순서·소유권 의미는 최적화로 변경하지 않는다.
2. 문자열 이름은 의미 분석 단계에서 안정적 ID로 변환한다.
3. Error Node·Error Type은 복구를 위해 허용하지만 Codegen 입력에는 남지 않는다.
4. 미지원 기능은 추측해서 실행하지 않고 안정적인 진단으로 거부한다.
5. 동일 입력과 옵션은 결정적인 결과를 생성한다.

## 5. 구현 요구사항

1. TokenKind, cursor, mode stack, diagnostics를 구현한다.
2. delimiter stack과 동기화 지점을 구현한다.
3. BasicBlockData와 MIR validator를 구현한다.
4. Corpus·minimizer·timeout을 구성한다.

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

- Unicode, 중첩 주석, 보간, 잘못된 문자를 테스트한다.
- 불완전 코드와 괄호 누락을 테스트한다.
- CFG·Cleanup·Dump를 테스트한다.
- Unicode·깊은 중첩·CFG 순환을 생성한다.

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


## 9. 변경 절차

범위 변경은 NOVA-006의 변경 제안·영향 분석·결정 기록 절차를 따른다.


---

# Property·Differential Testing 사양서

| 항목 | 값 |
|---|---|
| 문서 ID | NOVA-142 |
| 대상 | Nova 0.1 |
| 구현 시점 | MVP 후반 |
| 상태 | 확정 |
| 갱신일 | 2026-07-27 |

## 1. 목적

이 문서는 **Property·Differential Testing 사양서**에 대한 언어 의미, Compiler 책임, 구현 경계, 오류 처리 및 완료 기준을 정의한다.

이 문서는 해당 구현 단계에 진입하기 전 동결해야 하는 기준이다.

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

1. 성공·실패·경계·회귀 테스트를 둔다.
2. 진단 Snapshot은 Code와 Span을 검증한다.
3. Fuzzing은 Crash·Hang·비결정성을 실패로 본다.

## 4. 확정 규칙

1. Property·Differential Testing의 입력·출력·불변 조건을 명시한다.
2. Property·Differential Testing의 MVP 범위와 후속 범위를 분리한다.
3. Property·Differential Testing의 결과는 컴파일 순서와 병렬 실행 순서에 독립적이어야 한다.

추가 불변 조건:

1. 평가 순서·Drop 순서·소유권 의미는 최적화로 변경하지 않는다.
2. 문자열 이름은 의미 분석 단계에서 안정적 ID로 변환한다.
3. Error Node·Error Type은 복구를 위해 허용하지만 Codegen 입력에는 남지 않는다.
4. 미지원 기능은 추측해서 실행하지 않고 안정적인 진단으로 거부한다.
5. 동일 입력과 옵션은 결정적인 결과를 생성한다.

## 5. 구현 요구사항

1. Property·Differential Testing 전용 데이터 모델과 Query 경계를 구현한다.
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

- Property·Differential Testing의 정상·오류·경계 사례를 테스트한다.
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


## 9. 변경 절차

범위 변경은 NOVA-006의 변경 제안·영향 분석·결정 기록 절차를 따른다.


---

# 성능 Benchmark·회귀 기준서

| 항목 | 값 |
|---|---|
| 문서 ID | NOVA-143 |
| 대상 | Nova 0.1 |
| 구현 시점 | Backend 후 |
| 상태 | 확정 |
| 갱신일 | 2026-07-27 |

## 1. 목적

이 문서는 **성능 Benchmark·회귀 기준서**에 대한 언어 의미, Compiler 책임, 구현 경계, 오류 처리 및 완료 기준을 정의한다.

이 문서는 해당 구현 단계에 진입하기 전 동결해야 하는 기준이다.

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

1. 성공·실패·경계·회귀 테스트를 둔다.
2. 진단 Snapshot은 Code와 Span을 검증한다.
3. Fuzzing은 Crash·Hang·비결정성을 실패로 본다.

## 4. 확정 규칙

1. 성능 Benchmark·회귀의 입력·출력·불변 조건을 명시한다.
2. 성능 Benchmark·회귀의 MVP 범위와 후속 범위를 분리한다.
3. 성능 Benchmark·회귀의 결과는 컴파일 순서와 병렬 실행 순서에 독립적이어야 한다.

추가 불변 조건:

1. 평가 순서·Drop 순서·소유권 의미는 최적화로 변경하지 않는다.
2. 문자열 이름은 의미 분석 단계에서 안정적 ID로 변환한다.
3. Error Node·Error Type은 복구를 위해 허용하지만 Codegen 입력에는 남지 않는다.
4. 미지원 기능은 추측해서 실행하지 않고 안정적인 진단으로 거부한다.
5. 동일 입력과 옵션은 결정적인 결과를 생성한다.

## 5. 구현 요구사항

1. 성능 Benchmark·회귀 전용 데이터 모델과 Query 경계를 구현한다.
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

- 성능 Benchmark·회귀의 정상·오류·경계 사례를 테스트한다.
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


## 9. 변경 절차

범위 변경은 NOVA-006의 변경 제안·영향 분석·결정 기록 절차를 따른다.


---

# 메모리 안전성 검증 전략서

| 항목 | 값 |
|---|---|
| 문서 ID | NOVA-144 |
| 대상 | Nova 0.1 |
| 구현 시점 | Ownership 구현 후 |
| 상태 | 확정 |
| 갱신일 | 2026-07-27 |

## 1. 목적

이 문서는 **메모리 안전성 검증 전략서**에 대한 언어 의미, Compiler 책임, 구현 경계, 오류 처리 및 완료 기준을 정의한다.

이 문서는 해당 구현 단계에 진입하기 전 동결해야 하는 기준이다.

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

1. 성공·실패·경계·회귀 테스트를 둔다.
2. 진단 Snapshot은 Code와 Span을 검증한다.
3. Fuzzing은 Crash·Hang·비결정성을 실패로 본다.

## 4. 확정 규칙

1. 메모리 안전성 검증 전략서의 입력·출력·불변 조건을 명시한다.
2. 메모리 안전성 검증 전략서의 MVP 범위와 후속 범위를 분리한다.
3. 메모리 안전성 검증 전략서의 결과는 컴파일 순서와 병렬 실행 순서에 독립적이어야 한다.

추가 불변 조건:

1. 평가 순서·Drop 순서·소유권 의미는 최적화로 변경하지 않는다.
2. 문자열 이름은 의미 분석 단계에서 안정적 ID로 변환한다.
3. Error Node·Error Type은 복구를 위해 허용하지만 Codegen 입력에는 남지 않는다.
4. 미지원 기능은 추측해서 실행하지 않고 안정적인 진단으로 거부한다.
5. 동일 입력과 옵션은 결정적인 결과를 생성한다.

## 5. 구현 요구사항

1. 메모리 안전성 검증 전략서 전용 데이터 모델과 Query 경계를 구현한다.
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

- 메모리 안전성 검증 전략서의 정상·오류·경계 사례를 테스트한다.
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


## 9. 변경 절차

범위 변경은 NOVA-006의 변경 제안·영향 분석·결정 기록 절차를 따른다.


---

# Unsafe·FFI 보안 검토 기준서

| 항목 | 값 |
|---|---|
| 문서 ID | NOVA-145 |
| 대상 | Nova 0.1 |
| 구현 시점 | FFI 전 |
| 상태 | 확정 |
| 갱신일 | 2026-07-27 |

## 1. 목적

이 문서는 **Unsafe·FFI 보안 검토 기준서**에 대한 언어 의미, Compiler 책임, 구현 경계, 오류 처리 및 완료 기준을 정의한다.

이 문서는 해당 구현 단계에 진입하기 전 동결해야 하는 기준이다.

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

1. 성공·실패·경계·회귀 테스트를 둔다.
2. 진단 Snapshot은 Code와 Span을 검증한다.
3. Fuzzing은 Crash·Hang·비결정성을 실패로 본다.

## 4. 확정 규칙

1. unsafe는 Capability Block으로 제한한다.
2. 기존 소유권·빌림 검사를 자동 해제하지 않는다.
3. Raw Pointer는 안전 View보다 약한 보장만 가진다.

추가 불변 조건:

1. 평가 순서·Drop 순서·소유권 의미는 최적화로 변경하지 않는다.
2. 문자열 이름은 의미 분석 단계에서 안정적 ID로 변환한다.
3. Error Node·Error Type은 복구를 위해 허용하지만 Codegen 입력에는 남지 않는다.
4. 미지원 기능은 추측해서 실행하지 않고 안정적인 진단으로 거부한다.
5. 동일 입력과 옵션은 결정적인 결과를 생성한다.

## 5. 구현 요구사항

1. Capability stack과 위험 연산 검증을 구현한다.

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

- Pointer escape와 FFI wrapper를 테스트한다.

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


## 9. 변경 절차

범위 변경은 NOVA-006의 변경 제안·영향 분석·결정 기록 절차를 따른다.


---

# 0.1 Release Checklist

| 항목 | 값 |
|---|---|
| 문서 ID | NOVA-146 |
| 대상 | Nova 0.1 |
| 구현 시점 | 배포 전 |
| 상태 | 확정 |
| 갱신일 | 2026-07-27 |

## 1. 목적

이 문서는 **0.1 Release Checklist**에 대한 언어 의미, Compiler 책임, 구현 경계, 오류 처리 및 완료 기준을 정의한다.

이 문서는 해당 구현 단계에 진입하기 전 동결해야 하는 기준이다.

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

1. 성공·실패·경계·회귀 테스트를 둔다.
2. 진단 Snapshot은 Code와 Span을 검증한다.
3. Fuzzing은 Crash·Hang·비결정성을 실패로 본다.

## 4. 확정 규칙

1. Release는 테스트·성능·안전 Checklist를 통과한다.
2. Artifact에 checksum을 제공한다.
3. Clean machine에서 검증한다.

추가 불변 조건:

1. 평가 순서·Drop 순서·소유권 의미는 최적화로 변경하지 않는다.
2. 문자열 이름은 의미 분석 단계에서 안정적 ID로 변환한다.
3. Error Node·Error Type은 복구를 위해 허용하지만 Codegen 입력에는 남지 않는다.
4. 미지원 기능은 추측해서 실행하지 않고 안정적인 진단으로 거부한다.
5. 동일 입력과 옵션은 결정적인 결과를 생성한다.

## 5. 구현 요구사항

1. CI release pipeline을 구현한다.

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

- 재현 빌드와 설치를 테스트한다.

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


## 9. 변경 절차

범위 변경은 NOVA-006의 변경 제안·영향 분석·결정 기록 절차를 따른다.


---

# 언어 호환성 Test Suite 사양서

| 항목 | 값 |
|---|---|
| 문서 ID | NOVA-147 |
| 대상 | Nova 0.1 |
| 구현 시점 | 배포 전 |
| 상태 | 확정 |
| 갱신일 | 2026-07-27 |

## 1. 목적

이 문서는 **언어 호환성 Test Suite 사양서**에 대한 언어 의미, Compiler 책임, 구현 경계, 오류 처리 및 완료 기준을 정의한다.

이 문서는 해당 구현 단계에 진입하기 전 동결해야 하는 기준이다.

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

1. 성공·실패·경계·회귀 테스트를 둔다.
2. 진단 Snapshot은 Code와 Span을 검증한다.
3. Fuzzing은 Crash·Hang·비결정성을 실패로 본다.

## 4. 확정 규칙

1. 언어 호환성 Test Suite의 입력·출력·불변 조건을 명시한다.
2. 언어 호환성 Test Suite의 MVP 범위와 후속 범위를 분리한다.
3. 언어 호환성 Test Suite의 결과는 컴파일 순서와 병렬 실행 순서에 독립적이어야 한다.

추가 불변 조건:

1. 평가 순서·Drop 순서·소유권 의미는 최적화로 변경하지 않는다.
2. 문자열 이름은 의미 분석 단계에서 안정적 ID로 변환한다.
3. Error Node·Error Type은 복구를 위해 허용하지만 Codegen 입력에는 남지 않는다.
4. 미지원 기능은 추측해서 실행하지 않고 안정적인 진단으로 거부한다.
5. 동일 입력과 옵션은 결정적인 결과를 생성한다.

## 5. 구현 요구사항

1. 언어 호환성 Test Suite 전용 데이터 모델과 Query 경계를 구현한다.
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

- 언어 호환성 Test Suite의 정상·오류·경계 사례를 테스트한다.
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


## 9. 변경 절차

범위 변경은 NOVA-006의 변경 제안·영향 분석·결정 기록 절차를 따른다.


---

# Self-hosting 계획서

| 항목 | 값 |
|---|---|
| 문서 ID | NOVA-148 |
| 대상 | Nova 0.1 |
| 구현 시점 | MVP 이후 |
| 상태 | 후속 |
| 갱신일 | 2026-07-27 |

## 1. 목적

이 문서는 **Self-hosting 계획서**에 대한 언어 의미, Compiler 책임, 구현 경계, 오류 처리 및 완료 기준을 정의한다.

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

1. 성공·실패·경계·회귀 테스트를 둔다.
2. 진단 Snapshot은 Code와 Span을 검증한다.
3. Fuzzing은 Crash·Hang·비결정성을 실패로 본다.

## 4. 확정 규칙

1. Rust Stage0 안정화 이후 시작한다.
2. 표준 도구→Frontend→전체 Compiler 순서다.
3. Stage 결과를 비교한다.

추가 불변 조건:

1. 평가 순서·Drop 순서·소유권 의미는 최적화로 변경하지 않는다.
2. 문자열 이름은 의미 분석 단계에서 안정적 ID로 변환한다.
3. Error Node·Error Type은 복구를 위해 허용하지만 Codegen 입력에는 남지 않는다.
4. 미지원 기능은 추측해서 실행하지 않고 안정적인 진단으로 거부한다.
5. 동일 입력과 옵션은 결정적인 결과를 생성한다.

## 5. 구현 요구사항

1. Bootstrap pipeline을 설계한다.

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

- Stage1→Stage2 의미 동일성을 검증한다.

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
