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
