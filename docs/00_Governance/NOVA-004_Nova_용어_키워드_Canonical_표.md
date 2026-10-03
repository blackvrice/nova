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
