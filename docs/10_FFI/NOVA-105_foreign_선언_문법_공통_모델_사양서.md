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
