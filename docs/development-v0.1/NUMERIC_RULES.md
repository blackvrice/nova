# 숫자 타입·연산·변환 초안

상태 Draft, D07. Primitive alias 자체는 기존 Canonical이다. 아래 default/conversion/overflow는 제안이다.

Stage A Int32 Literal 기본 타입·범위·직접 unary minus 처리는 사용자 승인 [P02](SEMANTICS_STAGE_A_PROPOSAL.md)를 따른다.
그 외 widening/float/overflow/runtime 계약은 이 문서의 Draft 상태를 유지한다.
후속 사용자 승인 [P03](NATIVE_STAGE_A_PROPOSAL.md)은 Stage A Int32 checked runtime 연산 부분에 적용한다.
widening/float/cast/const 등 나머지는 Draft다.

## 범위와 기본 타입

intN: [-2^(N-1),2^(N-1)-1], uintN: [0,2^N-1]. byte=uint8, int=int32, uint=uint32.
integer literal은 기대 정수 타입을 우선하고 없으면 int32, real은 기대 float/double 우선 후
float32다. unconstrained 범위 초과를 자동으로 int64로 승격하지 않는다. unary minus를
literal magnitude와 함께 검사하여 MIN을 허용한다. unsigned 음수와 char implicit numeric은 거부한다.

## 암묵 widening

| Source → Dest | 허용 |
|---|---|
| intN → intM | M≥N |
| uintN → uintM | M≥N |
| uintN → intM | M>N |
| intN → uintM | 불허 |
| float32 → float64 | 허용 |
| integer → float32/64 | 전체 범위가 mantissa로 정확 표현 가능할 때만 |
| float → integer / narrower float | 불허 |
| bool/char ↔ numeric | 불허 |

binary32 precision p=24, binary64 p=53. signed width N은 N-1≤p, unsigned N은 N≤p일 때
integer→float lossless다. int32→float32 불허, int32→float64 허용, int64→float64 불허.
binary operation은 각 operand를 같은 허용 타입으로 변환한다. 정수끼리는 무손실 공통 정수
타입 중 최소 폭을 선택하고 동률이면 최소 전체 범위 타입을 제안한다. float operand가 있으면
해당 float 계열 내 공통 타입을 찾는다. 두 정수에 공통 정수 타입이 없다고 float로 우회하지 않는다.

## Runtime 연산

일반 정수 + - * / %는 checked, overflow/div0/MIN/-1는 Abort panic. debug/release/size에서 동일하다.
division은 0 방향 truncate, remainder는 dividend 부호를 제안한다. explicit as narrowing은
range 검사; float→int는 finite와 범위 확인 후 truncate; out-of-range/NaN은 panic 제안이다.
wrapping/checked Option API는 명시 method로 별도 제공한다.

float는 IEEE binary32/64 round-to-nearest-ties-to-even, NaN 비교 false(단 !=는 true),
±0 equality true를 제안한다. literal가 infinity로 overflow하면 compile error, subnormal/underflow
rounding은 finite model로 수용하는 제안이다. fast-math/reassociation 기본 off.
반올림/출력 decimal formatting은 runtime API로 고정하며 host locale를 따르지 않는다.

## Const와 검증

같은 실패가 const에서는 compile diagnostic이고 runtime에서는 Abort다. C++ signed UB나 Rust
profile별 overflow를 Nova 의미로 사용하지 않는다. Target fixed-width를 사용한다.
T008~T010, T018, T043 및 width별 generated boundary corpus로 검사한다.
