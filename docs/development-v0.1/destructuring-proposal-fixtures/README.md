# P26 irrefutable Copy let 구조 분해 수용 fixture — Accepted

2026-10-12 사용자 승인. **Accepted / 구현·검증 완료**. [최소 계약](../DESTRUCTURING_STAGE_B_PROPOSAL.md)·[62-production EBNF](../GRAMMAR_STAGE_B_DESTRUCTURING.ebnf)을 따른다.
두 파일 main/types·추가 정상 3·부정 22·Runtime 1, source 28개는 UTF-8/LF다.
expected.json의 validated_result는 실제 Compiler/Native 검증 결과이며 implementation_verified=true다. N3103은 승인 후 등록했다.
28개 Nova source 원 UTF-8/LF bytes와 Draft 주석은 역사로 보존하고 source_sha256으로 검사한다. [구현 기록](../DESTRUCTURING_IMPLEMENTATION.md)을 따른다.

## 검증된 main 출력

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

독립 oracle는 작은 finite type domain의 irrefutability를 검사한다. 문서 validator는 source/UTF-8 Span/grammar/Accepted ledger 검사다.
둘 다 P26 Compiler/Native 성공을 증명하지 않는다.

## 직접 실행하는 수용 명령

아래 명령은 check exit 0·debug/release의 위 17줄/LF·빈 stderr·exit 0을 확인한다.

```powershell
cargo run -p nova-cli --offline -- check docs/development-v0.1/destructuring-proposal-fixtures/main.nova --source-root docs/development-v0.1/destructuring-proposal-fixtures
$env:NOVA_CLANG = (Resolve-Path target/toolchains/llvm-21.1.8/bin/clang.exe).Path
cargo run -p nova-cli --offline -- run docs/development-v0.1/destructuring-proposal-fixtures/main.nova --source-root docs/development-v0.1/destructuring-proposal-fixtures --profile debug
cargo run -p nova-cli --offline -- run docs/development-v0.1/destructuring-proposal-fixtures/main.nova --source-root docs/development-v0.1/destructuring-proposal-fixtures --profile release
```

P26 [독립 예제](../../../examples/let_destructuring.nova)·[직접 명령](../../../TESTING.md)을 제공한다.
집중 회귀는 `cargo test --workspace --offline p26_`다. NOVA_CLANG 설정 후 실제 LLVM/Native는
`cargo test -p nova-codegen-llvm --test emission --offline p26_ -- --ignored --test-threads=1`과
`cargo test -p nova-cli --test native --offline p26_ -- --ignored --test-threads=1`이다.
