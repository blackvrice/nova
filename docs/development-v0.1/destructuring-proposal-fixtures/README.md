# P26 irrefutable Copy let 구조 분해 수용 fixture — Draft

**사용자 승인 대기 / 미구현**. [최소 계약](../DESTRUCTURING_STAGE_B_PROPOSAL.md)·[62-production 제안 EBNF](../GRAMMAR_STAGE_B_DESTRUCTURING.ebnf)을 따른다.
두 파일 main/types·추가 정상 3·부정 22·Runtime 1, source 28개는 UTF-8/LF다.
expected.json의 proposed_result는 향후 수용 기준이며 implementation_verified=false다. N3103은 P26에만 제안한 미등록 code다.
현재 Compiler/Native 성공 기록이 아니다.

## 제안된 main 출력

```text
initializer
nested=7/true/한
pair
pair=3/false
copy=5/true/6
original=9
single=true
unit-call
shadow=3
legacy=9
point=7/9
loop=2
fetch
after=true
ok=4
fetch
error=5
```

단일 initializer·nested Copy Tuple/single Enum·Unit·annotation/import alias·snapshot·ancestor shadow·기존 단일 IDENT `_`·opaque struct Copy·loop·try를 관측한다.
discard_effect는 모든 wildcard여도 호출 effect가 남는지 확인한다. opaque_copy는 struct field pattern 없이 Copy binder를 사용한다.
annotation_context는 전체 initializer에 uint8/char 기대 타입을 적용한다. initializer_abort는 binder/뒤 statement 이전 checked Abort다.
refutable 사례는 initializer의 알려진 값과 관계없이 타입 전체 domain으로 거부한다.

## 현재 실행할 수 있는 문서 검사

```powershell
node tools/docs/validate-pack.mjs
node tools/tests/destructuring-oracle.mjs
cargo fmt --check
cargo test --workspace --offline
```

독립 oracle는 작은 finite type domain의 irrefutability를 검사한다. 문서 validator는 source/UTF-8 Span/grammar/Draft ledger 검사다.
둘 다 P26 Compiler/Native 성공을 증명하지 않는다.

## 승인 후 구현에 사용할 수용 명령

아래 명령은 **아직 성공하지 않는 향후 수용 명령**이다. 구현 후 debug/release의 위 17줄·빈 stderr·exit 0을 검증한다.

```powershell
cargo run -p nova-cli --offline -- check docs/development-v0.1/destructuring-proposal-fixtures/main.nova --source-root docs/development-v0.1/destructuring-proposal-fixtures
$env:NOVA_CLANG = (Resolve-Path target/toolchains/llvm-21.1.8/bin/clang.exe).Path
cargo run -p nova-cli --offline -- run docs/development-v0.1/destructuring-proposal-fixtures/main.nova --source-root docs/development-v0.1/destructuring-proposal-fixtures --profile debug
cargo run -p nova-cli --offline -- run docs/development-v0.1/destructuring-proposal-fixtures/main.nova --source-root docs/development-v0.1/destructuring-proposal-fixtures --profile release
```

현재 구현된 P25 실행은 [TESTING.md](../../../TESTING.md)의 examples/match_guards.nova 명령을 사용한다.
