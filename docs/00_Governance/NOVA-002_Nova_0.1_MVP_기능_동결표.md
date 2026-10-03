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
