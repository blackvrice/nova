# Core/표준 라이브러리 API 초안

상태 Draft, D10/D15/D20/D23. 아래 signature는 설계 표기다. Self/view/result mode 표기를
그대로 현재 Parser 지원으로 해석하지 않는다. Read는 소스 modifier 부재다. take는 owner 소비,
change는 exclusive access다. view 수명은 owner/receiver에 연결되며 source syntax는 승인 대기다.

## Core

| API | 인수/반환 | 실패/소유권 | 비용/Stage |
|---|---|---|---|
| print | Read string → Unit | UTF-8 stdout+newline; I/O 실패 Abort 제안 | O(bytes), A |
| panic | Read string → Never | SourceInfo stderr 후 Abort; cleanup 없음 | A |
| Option.isSome/isNone | Read self → bool | consume 없음 | O(1), B/C |
| Option.unwrap | take self → T | None panic | O(1), B/C |
| Option.unwrapOr | take self, take T → T | 둘 모두 평가; 미사용 payload Drop | O(1)+Drop, B/C |
| Result.isSuccess/isError | Read self → bool | consume 없음 | O(1), B/C |
| Result.unwrap/unwrapError | take self → T/E | 반대 variant panic | O(1), B/C |
| integer.checkedAdd/Sub/Mul | Read self, same T → Option<T> | overflow None | O(1), B |
| integer.wrappingAdd/Sub/Mul | Read self, same T → T | 2^N modular wrap | O(1), B |
| integer.parse | Read string → Result<T,ParseError> | invalid/range 오류, complete input 요구 | O(bytes), E |
| primitive.toString | Read self → string | allocation OOM Abort | O(digits), E |
| float.isNaN/isInfinite/abs | Read self → bool/bool/Self | IEEE 의미 | O(1), B/E |

Print는 현재 확정 출력 예제의 구체화 제안이다. file/stream write는 recoverable Result,
간단 print는 실패 Abort라는 구분을 D23에서 승인해야 한다. raw bytes/NUL은 길이를 전달하며
C의 NUL-terminated string를 호출해 문자열을 잘라내지 않는다.

## 문자열

| API | 모드/반환 | 계약 | 비용 |
|---|---|---|---|
| byteLength/isEmpty | Read self → uint64/bool | length는 UTF-8 bytes | O(1) |
| bytes | Read self → ReadOnlySpan<byte> | owner region 유지 | O(1) |
| scalarAt | Read self,uint64 → Option<char> | Unicode scalar index | O(n) |
| sliceBytes | Read self,start,end → Result<string,BoundaryError> | [start,end), UTF-8 boundaries, owned copy | O(slice) |
| fromUtf8 | take Array<byte> → Result<string,Utf8Error> | 전체 validation, 실패 payload 소유권 명시 | O(n) |
| concat | Read self,Read string → string | 새 owned UTF-8 | O(n+m) |

Grapheme API는 별도 Unicode segmentation data/version가 필요하므로 최초 Core에 포함하지
않는 제안이다. scalarAt는 O(1)이라고 약속하지 않는다. 소스 문자열의 단순 index는 비지원이다.
fromUtf8의 오류 payload에는 원 bytes owner를 반환하는 설계를 제안하여 소비된 데이터 복구를 허용한다.

## Array/View/List/Map

| API | 모드/반환 | 실패/수명 | 비용 |
|---|---|---|---|
| Array.new | → Array<T> | expected T 필요 | O(1) |
| length/capacity | Read self → uint64 | initialized len≤cap | O(1) |
| push | change self,take T → Unit | live view와 충돌; OOM Abort | amortized O(1) |
| pop | change self → Option<T> | element owner 반환 | O(1) |
| reserve | change self,uint64 → Unit | cap×sizeof overflow/OOM Abort | O(n) worst |
| readAt/changeAt | Read/change self,uint64 → Read/change view T | bounds panic; owner region | O(1) |
| get/getChange | Read/change self,uint64 → Option<view T> | bounds None | O(1) |
| asReadOnlySpan/asSpan | Read/change self → respective span | region=Array storage | O(1) |
| Span.slice | Read/change span,start,end → Result<span,BoundsError> | bounds/non-overlap rules | O(1) |
| Span.splitAt | take/reborrow span,index → (span,span) | parent suspended while child live | O(1) |
| Map.insert | change self,take K,take V → Option<V> | old value returned, equal old key Drop | average O(1) |
| Map.remove | change self,Read K → Option<V> | iterator invalidation | average O(1) |
| Shared.clone | Read self → Shared<T> | checked atomic increment | O(1) |
| Weak.upgrade | Read self → Option<Shared<T>> | strong 0→None | O(1) |

List는 growable Array의 별칭/편의층 제안이다. Map constraint Hash/Equality와 iterable
interface는 Associated Type 없이 명시 generic type parameter로 설계한다(D15). 이 API를
Stage B에서 소유권 검사 없이 전부 허용하지 않는다; C/D 의존성을 로드맵에 기록한다.
공개 길이/index는 uint64를 제안하며 Target pointer-sized storage로의 변환과 size 곱셈은
checked다. uintptr/usize라는 새 Primitive를 암묵 도입하지 않는다. 32-bit Target에서는
표현 가능한 storage 크기를 넘는 요청을 명시 bounds/capacity failure로 처리한다.

## 시스템 API

| 모듈/API | 결과 | 계약 |
|---|---|---|
| File.open(path,mode) | Result<File,IoError> | Move owner, OS path validation |
| File.read(change buffer) | Result<uint64,IoError> | 실제 읽은 byte 수; 0=EOF |
| File.write(ReadOnlySpan<byte>) | Result<uint64,IoError> | partial write 가능 |
| File.flush/close(take) | Result<Unit,IoError> | explicit close에서 실패 보고 |
| Clock.monotonicNow | Instant | backward jump 없음; elapsed Duration |
| Clock.wallNow | Timestamp | wall clock 조정 가능 |
| Random.fillSecure(change span) | Result<Unit,RandomError> | OS entropy 실패 보고 |
| Env.get(Read name) | Result<Option<string>,EnvError> | missing과 empty 구분 |
| Env.args | Array<string> 또는 명시 encoding Result | main signature와 독립 |
| Thread.spawn(take closure) | Result<ThreadHandle,ThreadError> | Sendable 검사 |
| Thread.join(take handle) | Result<Unit,ThreadError> | handle 한 번 소비 |
| Mutex.lock | Guard<T> 또는 lock Result | guard region=mutex, Drop unlock |
| Atomic.load/store/CAS | T/Unit/Result-type result | 명시 memory order와 허용 조합 |

OS error payload는 category+native code+operation, secret path/value 출력은 필요한 context만
담는다. API가 error를 반환하는 경우 구조화 code와 payload 소유권을 명시한다. drop close는
오류를 반환하지 못하므로 explicit close의 역할을 문서화한다.

## 완료 기준

각 API 구현 전 정확한 source signature, receiver/parameter/return mode, effects, allocation,
panic/Result, complexity, region, Target 지원을 동결한다. 각 표 행에 정상/오류/Drop/lifetime
테스트가 필요하다. 현재 이 표는 새 공개 API 제안이며 std 구현이 존재한다는 뜻이 아니다.
