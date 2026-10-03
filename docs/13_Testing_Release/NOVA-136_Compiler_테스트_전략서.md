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
