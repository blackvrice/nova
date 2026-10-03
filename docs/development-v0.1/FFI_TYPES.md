# C FFI 타입과 안전 Wrapper 계약 초안

상태 Draft D16/D17. [원본 foreign](../10_FFI/NOVA-105_foreign_선언_문법_공통_모델_사양서.md)을 유지한다.

## Raw 타입

| Nova 설계 타입 | C 대응 | 직접 ABI 조건 |
|---|---|---|
| int8/16/32/64,uint8/16/32/64 | 대응 fixed-width integer | Target에서 width/sign 확인 |
| float/double | float/double | IEEE/ABI 확인 |
| *T / *change T | const T* / T* 후보 | pointer 유효성·null·len·align 별도 |
| opaque handle | opaque struct pointer | release/retain/ownership 계약 |
| callback+context | C function pointer+void* | calling convention/retention/thread |
| Unit | void return | argument Unit direct mapping은 별도 |

Nova bool/char/string/Array/Option/Result/Class/shared/view는 raw C와 자동 ABI 동일하지 않다.
bool은 explicit uint8/int mapping wrapper, char는 scalar uint32 또는 encoding wrapper,
string은 ptr+byte length, Array는 buffer+length wrapper다. C struct 직접 mapping은 explicit
C-compatible layout definition이 필요하다. C long/size_t의 width를 host i64로 추측하지 않는다.

## Pointer 연산 초안

*T/*change T 철자는 nullable raw pointer candidate다. 안전 view와 달리 lifetime 증명을
자동 제공하지 않는다. 최초안은 rawLoad/rawStore/rawAddress 등 명시 intrinsic을 unsafe에서
사용하는 방법을 제안하며 일반 *expression dereference/address-of operator는 grammar에
추가하지 않는다. pointer arithmetic/variadic/bitfield/자동 C++ binding은 제외한다.

## Binding sidecar 설계

```json
{
  "schema_version": 1,
  "symbol": "native_read",
  "abi": "c",
  "parameters": [
    {"name":"handle","ownership":"borrowed","duration":"call","nullable":false},
    {"name":"buffer","ownership":"change","length_parameter":"length","length_unit":"bytes"}
  ],
  "error_model": {"kind":"status","success":0},
  "thread_safety": "unknown",
  "callback_retention": "none"
}
```

Owned-in에는 consume-on-success/failure를 각각 명시한다. Owned-out에는 release symbol와
allocator/library lifetime을 명시한다. borrowed-out에는 owner origin/duration을 명시한다.
이 schema는 binding generator 입력 제안이며 language attribute 철자를 확정하지 않는다.

## Safe wrapper 절차

입력 validation → exclusive/Read loan 확보 → raw call → status 검사 → output validity/UTF-8
검사 → Result construction → error-path release. 외부 exception은 shim에서 포착한다.
원 function의 failure에서도 handle가 소비되는지 확인하기 전 owner를 원상복구하지 않는다.

## 검사

null/invalid length/alignment/UTF-8/status, callback-after-owner-drop, library unload,
allocator mismatch, thread-affinity, large return/calling convention을 first Windows/Linux
Target에서 검증한다. unsafe는 일반 ownership/type 검사를 끄지 않는다.
