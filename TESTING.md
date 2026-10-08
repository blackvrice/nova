# 직접 실행하는 Nova 테스트

명령은 저장소 root의 PowerShell에서 실행한다. Rust/MSVC와 Native용 LLVM 21.1.8이 필요하다.
현재 개발 완료 기능은 P24 중첩 Copy 패턴·Tuple match까지다.

## P24 중첩 Copy 패턴·Tuple match

```powershell
cargo run -p nova-cli --offline -- check examples/nested_patterns.nova
$env:NOVA_CLANG = (Resolve-Path target/toolchains/llvm-21.1.8/bin/clang.exe).Path
cargo run -p nova-cli --offline -- run examples/nested_patterns.nova --profile debug
cargo run -p nova-cli --offline -- run examples/nested_patterns.nova --profile release
```

check는 exit 0, debug/release는 다음 4줄/LF·빈 stderr·exit 0이다.

```text
nested=7
classify=-1
singleton=true
unit
```

[두 파일 수용 예제](docs/development-v0.1/nested-pattern-proposal-fixtures/README.md)는 추가로 12줄 출력과 snapshot·try·loop를 검증한다.
[계약](docs/development-v0.1/NESTED_PATTERN_STAGE_B_PROPOSAL.md)·[구현 기록](docs/development-v0.1/NESTED_PATTERN_IMPLEMENTATION.md)을 제공한다.

```powershell
cargo test --workspace --offline p24_
cargo test -p nova-codegen-llvm --test emission --offline p24_ -- --ignored --test-threads=1
cargo test -p nova-cli --test native --offline p24_ -- --ignored --test-threads=1
cargo run -p nova-cli --offline -- check docs/development-v0.1/nested-pattern-proposal-fixtures/union_unreachable.nova
node tools/docs/validate-pack.mjs
node tools/tests/nested-pattern-oracle.mjs
```

새 기본 20개·실제 LLVM 1개·Native 2개다. 부정 예제 check는 N3102·exit 1이다.
729개 Bool-pair 패턴 조합을 독립 유한 값 열거와 대조하며 node/task/cell 예산과 손상된 Source/Checked/MIR 입력을 검사한다.
guard·or/range·일반 literal·struct destructuring·Array·Move/Drop은 후속 계약이다.

## 현재 기능 실행 — Copy struct Read 메서드

```powershell
cargo run -p nova-cli --offline -- check examples/methods.nova
$env:NOVA_CLANG = (Resolve-Path target/toolchains/llvm-21.1.8/bin/clang.exe).Path
cargo run -p nova-cli --offline -- run examples/methods.nova --profile debug
cargo run -p nova-cli --offline -- run examples/methods.nova --profile release
```

check는 출력 없이 exit 0, debug/release는 다음 15줄/LF·빈 stderr·exit 0이다.

```text
sum=8
alias=7
copy=3/5
text=3/4
private=3
member-main=4
tuple=7
receiver
arg
order=9
fetch
after
ok=11
fetch
error=-1
```

`func sum(self,bias:Small=SHIFT)`의 self는 immutable Read Copy receiver다. 기본 인수는 선언 module에서 해석한다.
receiver를 한 번 먼저 복사하고 일반 인수는 source order로 평가한다. var copy=self로 지역 복사본을 갱신할 수 있다.
[계약](docs/development-v0.1/METHOD_STAGE_B_PROPOSAL.md)·[구현 기록](docs/development-v0.1/METHOD_IMPLEMENTATION.md)·[두 파일 fixture](docs/development-v0.1/method-proposal-fixtures/README.md)를 제공한다.

```powershell
cargo test --workspace --offline p22_
cargo test -p nova-codegen-llvm --test emission --offline p22_ -- --ignored --test-threads=1
cargo test -p nova-cli --test native --offline p22_ -- --ignored --test-threads=1
cargo run -p nova-cli --offline -- check docs/development-v0.1/method-proposal-fixtures/mutate_self.nova
```

P22 기본 11개·실제 LLVM 1개·Native 2개를 검증한다. 마지막 명령은 self 위치의 N3004·exit 1이다.
change/take·bound method value·Enum method·init·Array·일반 Move/Drop은 후속이다.

## 기존 기능 실행 — 비제네릭 Type Alias

```powershell
cargo run -p nova-cli --offline -- check examples/type_aliases.nova
$env:NOVA_CLANG = (Resolve-Path target/toolchains/llvm-21.1.8/bin/clang.exe).Path
cargo run -p nova-cli --offline -- run examples/type_aliases.nova --profile debug
cargo run -p nova-cli --offline -- run examples/type_aliases.nova --profile release
```

check는 출력 없이 exit 0, debug/release는 다음 8줄/LF·빈 stderr·exit 0이다.

```text
small=7
pair=7/2
exists=true
flag=on
cast=7
text=한글
try=true
error=-1
```

`type Small=int8`은 같은 canonical 타입에 대한 transparent 별칭이다. 선언 module scope에서 forward/import를 해석한다.
alias는 새 value binding을 만들지 않는다. struct/Enum 생성은 원 이름으로 쓰며 alias head 생성은 N1102다.
[계약](docs/development-v0.1/ALIAS_STAGE_B_PROPOSAL.md)·[구현 기록](docs/development-v0.1/ALIAS_IMPLEMENTATION.md)·[두 파일 예제](docs/development-v0.1/alias-proposal-fixtures/README.md)를 제공한다.

```powershell
cargo test --workspace --offline p21_
cargo test -p nova-codegen-llvm --test emission --offline p21_ -- --ignored --test-threads=1
cargo test -p nova-cli --test native --offline p21_ -- --ignored --test-threads=1
cargo run -p nova-cli --offline -- check docs/development-v0.1/alias-proposal-fixtures/self_cycle.nova
```

P21 기본 12개·실제 LLVM 1개·Native 2개를 검증한다. 마지막 명령은 N2103·exit 1이다.
1,024개의 flat alias chain을 허용하고 1,025번째 이름은 N8901이다. generic alias·newtype·Array는 후속이다.

## 기존 기능 실행 — Copy Option exists

```powershell
cargo run -p nova-cli --offline -- check examples/exists.nova
$env:NOVA_CLANG = (Resolve-Path target/toolchains/llvm-21.1.8/bin/clang.exe).Path
cargo run -p nova-cli --offline -- run examples/exists.nova --profile debug
cargo run -p nova-cli --offline -- run examples/exists.nova --profile release
```

check는 출력 없이 exit 0이며 debug/release는 다음 18줄/LF·빈 stderr·exit 0이다.

```text
basic=true/false
payload=true/true
nested=true/true
const=true/true
default=true
once
once=true
short=false/true
snapshot=true/false
tuple=true
negate=true
second
first
order=false
try
try-ok=true
try
try-error=-1
```

Some(false)·Some(0)·Some(())·Some(None)은 true, None은 false다. payload 값은 검사하지 않는다.
operand는 한 번 평가하며 기존 &&/|| short-circuit·try·named/default 순서를 유지한다.
괄호 없이 try f() exists는 try (f() exists)이며, Result<Option<T>,E>의 성공 Option을 검사하려면 (try f()) exists를 쓴다.
[P20 계약](docs/development-v0.1/EXISTS_STAGE_B_PROPOSAL.md)·[구현 기록](docs/development-v0.1/EXISTS_IMPLEMENTATION.md)·
[두 파일 예제](docs/development-v0.1/exists-proposal-fixtures/README.md)를 제공한다.

```powershell
cargo test --workspace --offline p20_
cargo test -p nova-codegen-llvm --test emission --offline p20_ -- --ignored --test-threads=1
cargo test -p nova-cli --test native --offline p20_ -- --ignored --test-threads=1
cargo run -p nova-cli --offline -- check docs/development-v0.1/exists-proposal-fixtures/undefined_operand.nova
```

집중 검사는 기본 10개·LLVM 1개·Native 2개다. 마지막 명령은 N2001·exit 1과 파생 N2101 억제가 예상된다.
Result exists·flow narrowing·unwrap·String/Move payload·Array는 후속이다.

## 기존 기능 실행 — loop·정수 범위 for

```powershell
cargo run -p nova-cli --offline -- check examples/range_loops.nova
$env:NOVA_CLANG = (Resolve-Path target/toolchains/llvm-21.1.8/bin/clang.exe).Path
cargo run -p nova-cli --offline -- run examples/range_loops.nova --profile debug
cargo run -p nova-cli --offline -- run examples/range_loops.nova --profile release
```

check는 출력 없이 exit 0이며 debug/release는 다음 18줄/LF·빈 stderr·exit 0이다.

```text
range=8
start
end
bounds=23
empty-start
empty-end
empty=0
maximum=2/255
mixed=0
shadow=99/3
nested=4
loop=3
try-start
try-end
try-ok=3
try-start
try-end
try-error=-1
```

[P19 계약](docs/development-v0.1/RANGE_LOOP_STAGE_B_PROPOSAL.md)·[구현 기록](docs/development-v0.1/RANGE_LOOP_IMPLEMENTATION.md)·[두 파일 수용 예제](docs/development-v0.1/range-loop-proposal-fixtures/README.md)를 제공한다.
until은 끝 제외, through는 끝 포함이고 step +1이다. 양 끝 값은 시작할 때 소스 순서로 한 번씩 저장한다.
for binder는 불변이고 continue는 advance로 이동한다. through 마지막 값은 증가하지 않으므로 최댓값도 정상 종료한다.

```powershell
cargo test --workspace --offline p19_
cargo test -p nova-codegen-llvm --test emission --offline p19_ -- --ignored --test-threads=1
cargo test -p nova-cli --test native --offline p19_ -- --ignored --test-threads=1
cargo run -p nova-cli --offline -- check docs/development-v0.1/range-loop-proposal-fixtures/immutable_binder.nova
```

집중 검사는 기본 10개·LLVM 1개·Native 2개다. 마지막 명령은 binder 대입 N3004·exit 1이 예상된다.
Array/일반 iterable·Range 값·step/descending·일반 Move/Drop은 후속이다.

## 기존 기능 실행 — 함수 기본 인수

```powershell
cargo run -p nova-cli --offline -- check examples/default_arguments.nova
$env:NOVA_CLANG = (Resolve-Path target/toolchains/llvm-21.1.8/bin/clang.exe).Path
cargo run -p nova-cli --offline -- run examples/default_arguments.nova --profile debug
cargo run -p nova-cli --offline -- run examples/default_arguments.nova --profile release
```

check는 출력 없이 exit 0이며 두 Native 실행은 다음 20줄과 각 LF, stderr 없음·exit 0이다.

```text
defaults=620
right
named=602
left
positional=120
second
first
reverse=702
holes=456
text=default/provided
unit=3
copy=4/🙂
option=none
result=7
leaf
after
success=602
leaf
error=-1
short=false
```

[P18 계약](docs/development-v0.1/DEFAULT_ARGUMENTS_STAGE_B_PROPOSAL.md)·[구현 기록](docs/development-v0.1/DEFAULT_ARGUMENTS_IMPLEMENTATION.md)·[두 파일 수용 예제](docs/development-v0.1/default-arguments-proposal-fixtures/README.md)를 제공한다.
기본값은 선언 module의 상수 표현식이다. caller local로 다시 해석하지 않으며 parameter/runtime 값 의존 default는 지원하지 않는다.
제공 인수를 source order로 평가한 뒤 생략 default를 declaration order로 caller에서 채운다.

```powershell
cargo test --workspace --offline p18_
cargo test -p nova-codegen-llvm --test emission --offline p18_ -- --ignored --test-threads=1
cargo test -p nova-cli --test native --offline p18_ -- --ignored --test-threads=1
cargo run -p nova-cli --offline -- check docs/development-v0.1/default-arguments-proposal-fixtures/unused_overflow.nova
```

집중 검사는 기본 10개·LLVM 1개·Native 2개다. 마지막 명령은 사용하지 않는 default의 `127+1`에서도 N3201·exit 1이 예상된다.
문서 metadata 검사는 `node tools/docs/validate-pack.mjs`로 실행하며 기대 결과는 PASS다. 실제 Compiler/Native 실행과 구분한다.

## 기존 기능 실행 — 함수 이름 인수

```powershell
cargo run -p nova-cli --offline -- check examples/named_arguments.nova
$env:NOVA_CLANG = (Resolve-Path target/toolchains/llvm-21.1.8/bin/clang.exe).Path
cargo run -p nova-cli --offline -- run examples/named_arguments.nova --profile debug
cargo run -p nova-cli --offline -- run examples/named_arguments.nova --profile release
```

check는 출력 없이 exit 0이며 두 Native 실행은 다음 24줄과 각 LF, stderr 없음·exit 0이다.

```text
right
left
reverse=702
positional
named
mixed=102
second
first
text=first/second
unit=3
unicode=304
minimum=-12500
copy=4/5
option=7
leaf
later
after
success=102
leaf
error=-1
before
leaf
prior-error=-1
short=false
```

[P17 계약](docs/development-v0.1/NAMED_ARGUMENTS_STAGE_B_PROPOSAL.md)·[구현 기록](docs/development-v0.1/NAMED_ARGUMENTS_IMPLEMENTATION.md)·[두 파일 수용 예제](docs/development-v0.1/named-arguments-proposal-fixtures/README.md)를 제공한다.
이름은 원 함수 parameter와 대응하며 label과 value 이름 공간은 별개다. source-order 평가 후 parameter-order로 전달한다.
상수 표현식 기본 인수는 P18에서 추가했다. named constructor·overload는 지원하지 않는다.

```powershell
cargo test --workspace --offline p17_
cargo test -p nova-codegen-llvm --test emission --offline p17_ -- --ignored --test-threads=1
cargo test -p nova-cli --test native --offline p17_ -- --ignored --test-threads=1
cargo run -p nova-cli --offline -- check docs/development-v0.1/named-arguments-proposal-fixtures/unknown_label.nova
```

집중 검사는 기본 9개·LLVM 1개·Native 2개다. 마지막 명령은 unknown label `other`에 N2201·exit 1이 예상된다.
문서 metadata 검사는 `node tools/docs/validate-pack.mjs`로 실행하며 기대 결과는 PASS다.
Compiler/Native 실행 검증과 구분한다.

## 기존 기능 실행 — Copy try·Result 오류 전파

```powershell
$env:NOVA_CLANG = (Resolve-Path target/toolchains/llvm-21.1.8/bin/clang.exe).Path
cargo run -p nova-cli --offline -- check examples/try_result.nova
cargo run -p nova-cli --offline -- run examples/try_result.nova --profile debug
cargo run -p nova-cli --offline -- run examples/try_result.nova --profile release
```

check는 출력 없이 exit 0이다. debug/release는 다음 19줄과 각 LF, stderr 없음·exit 0이다.

```text
start
leaf
second
after
ok=8
start
leaf
error=-1
leaf
nested=7
leaf
nested-error=-1
ping
unit=success
snapshot=9
short=false
leaf
leaf
loop=7
```

[계약](docs/development-v0.1/TRY_STAGE_B_PROPOSAL.md)·[구현 기록](docs/development-v0.1/TRY_IMPLEMENTATION.md)·
[두 파일 import/alias fixture](docs/development-v0.1/try-proposal-fixtures/README.md)를 제공한다.
P16 집중 검사는 기본 10개·실제 LLVM 1개·Native 2개이며 opt-in 검사는 O0/O2를 포함한다.

```powershell
cargo test --workspace --offline p16_
cargo test -p nova-codegen-llvm --test emission --offline p16_ -- --ignored --test-threads=1
cargo test -p nova-cli --test native --offline p16_ -- --ignored --test-threads=1
cargo run -p nova-cli --offline -- check docs/development-v0.1/try-proposal-fixtures/error_width.nova
```

마지막 check는 int8/Int16 Error 타입이 정확히 같지 않아 try keyword의 **N2101·exit 1**이다.

## 현재 기능 실행 — Copy Option·Result·nullable

```powershell
$env:NOVA_CLANG = (Resolve-Path target/toolchains/llvm-21.1.8/bin/clang.exe).Path
cargo run -p nova-cli --offline -- check examples/option_result.nova
cargo run -p nova-cli --offline -- run examples/option_result.nova --profile debug
cargo run -p nova-cli --offline -- run examples/option_result.nova --profile release
```

check는 성공 시 출력 없이 exit 0, 두 실행은 다음 8줄과 각 LF, stderr 없음·exit 0이다.

```text
maybe
some=7
original=none
success=8, flag=true
error=-1
nested=none
private=9
unit=success
```

[계약](docs/development-v0.1/OPTION_RESULT_STAGE_B_PROPOSAL.md),
[구현 기록](docs/development-v0.1/OPTION_RESULT_IMPLEMENTATION.md),
[두 파일 import·private factory fixture 실행](docs/development-v0.1/option-result-proposal-fixtures/README.md)을 제공한다.
P15의 빠른 검증은 기본 회귀 11개, 실제 LLVM 1개, Native 2개다. O0/O2는 opt-in 테스트 안에서 검사한다.

```powershell
cargo test --workspace --offline p15_
cargo test -p nova-codegen-llvm --test emission --offline p15_ -- --ignored --test-threads=1
cargo test -p nova-cli --test native --offline p15_ -- --ignored --test-threads=1
cargo run -p nova-cli --offline -- check docs/development-v0.1/option-result-proposal-fixtures/untyped_none.nova
```

마지막 명령은 `none`의 타입 문맥 부족으로 N2103·exit 1이 예상된다.
문맥 없이 `Option::Some(7)`은 Option<int32>지만 `none` 및 Result 생성자는 완성된 기대 타입이 필요하다.
`let x:int8?=Option::Some(7)`처럼 타입을 지정할 수 있다. String/Move payload·사용자 Generic은 후속이다. try는 P16을 따른다.

## 기존 기능 실행 — Copy Enum·match

LLVM이 PATH에 없다면 저장소 로컬 toolchain의 clang 경로를 먼저 설정한다.

```powershell
$env:NOVA_CLANG = (Resolve-Path target/toolchains/llvm-21.1.8/bin/clang.exe).Path
cargo run -p nova-cli --offline -- check examples/enums.nova
cargo run -p nova-cli --offline -- run examples/enums.nova --profile debug
cargo run -p nova-cli --offline -- run examples/enums.nova --profile release
```

두 실행의 예상 출력은 다음 네 줄과 각 LF이며 stderr 없음, exit 0이다.

```text
make
sum=30, code=7, flag=true
original=empty
ok
```

[Enum·match 계약](docs/development-v0.1/ENUM_STAGE_B_PROPOSAL.md),
[구현·검증 기록](docs/development-v0.1/ENUM_IMPLEMENTATION.md),
[import alias·private factory의 두 파일 실행 명령](docs/development-v0.1/enum-proposal-fixtures/README.md)을 제공한다.
기존 Copy Tuple/struct 예제는 `examples/tuples.nova`, `examples/structs.nova`로 실행할 수 있다.
Tuple 예상 출력은 `original=21, snapshot=20, shifted=21, one=7, tag=🙂` 한 줄과 LF다.

자신의 파일은 예제 경로를 `.nova` 파일 경로로 바꾸면 된다. 예를 들어 기존 `examples/function.nova`를
`cargo run -p nova-cli --offline -- check examples/function.nova`로 검사할 수 있다. 파일의 동작/통과 여부는 별도로 확인한다.

## 자동 테스트와 품질 검사

```powershell
cargo fmt --check
rustfmt --check --edition 2021 crates/nova-cli/runtime/stage_a.rs
cargo clippy --workspace --all-targets --offline -- -D warnings
cargo test --workspace --offline
cargo check --workspace --all-features --offline
```

P22 기준 기본 tests 359개가 성공하고 실제 LLVM/Native tests 70개는 ignored로 표시된다.
이 70개(LLVM 18개·Native 52개)를 실제 실행하려면 LLVM/Rust/MSVC 환경에서 다음을 별도로 실행한다.

```powershell
cargo test -p nova-codegen-llvm --test emission --offline -- --ignored --test-threads=1
cargo test -p nova-cli --test native --offline -- --ignored --test-threads=1
```

P14만 빠르게 확인하려면 다음 명령을 사용한다. LLVM 1개·Native 2개 테스트 안에서 O0/O2를 함께 검사한다.

```powershell
cargo test --workspace --offline p14_
cargo test -p nova-codegen-llvm --test emission --offline p14_ -- --ignored --test-threads=1
cargo test -p nova-cli --test native --offline p14_ -- --ignored --test-threads=1
```

누락 variant 검사를 직접 확인하면 `N3101`과 exit 1이 나온다.

```powershell
cargo run -p nova-cli --offline -- check docs/development-v0.1/enum-proposal-fixtures/missing_variant.nova
```

Native tests는 격리된 target 경로에 실행 파일을 만들고 실행한다. ELF object 검증은 Linux Native host 실행 검증이 아니다.
앞으로 기능 구현 완료 보고에는 새 예제의 check/debug/release 명령·예상 출력·기능별 테스트 명령을 함께 제공한다.


## P23 Copy struct 생성자 이름 인수

독립 예제는 [struct_named_arguments.nova](examples/struct_named_arguments.nova)다. 저장소 루트 PowerShell에서 실행한다.

```powershell
cargo run -p nova-cli --offline -- check examples/struct_named_arguments.nova
$env:NOVA_CLANG = (Resolve-Path target/toolchains/llvm-21.1.8/bin/clang.exe).Path
cargo run -p nova-cli --offline -- run examples/struct_named_arguments.nova --profile debug
cargo run -p nova-cli --offline -- run examples/struct_named_arguments.nova --profile release
```

check는 출력 없이 exit 0, debug/release는 다음 8줄·각 LF·빈 stderr·exit 0이다.

```text
y
x
point=1/2
mixed=3/300
const=4/20
default=5/6
copy=1/7
sum=3
```

[두 파일 수용 fixture/20줄](docs/development-v0.1/struct-named-arguments-proposal-fixtures/README.md)도 직접 실행할 수 있다.
현재 P23 구현은 생성된 Copy struct constructor의 이름 인수를 지원한다. Enum/Option/Result named payload·field default·explicit init은 범위 밖이다.

```powershell
cargo fmt --check
cargo check --workspace --all-features --offline
cargo clippy --workspace --all-targets --all-features --offline -- -D warnings
cargo test --workspace --offline
node tools/docs/validate-pack.mjs
```

LLVM/Native opt-in을 직접 검증하려면 앞의 NOVA_CLANG을 설정한 뒤 실행한다.

```powershell
cargo test -p nova-codegen-llvm --test emission --offline -- --ignored
cargo test -p nova-cli --test native --offline -- --ignored
```

2026-10-08 실제 결과: 기본 372 PASS / 0 FAIL / 73 ignored, LLVM 전체 19 PASS, Native 전체 54 PASS.
기본 test에서 제외된 73개는 별도 opt-in으로 모두 통과했다. P23 신규 회귀는 16개(기본 13+LLVM 1+Native 2)다.
