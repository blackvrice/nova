# P27 가변 Copy var 구조 분해 수용 fixture

2026-10-12 사용자 다음 개발 진행·승인에 따른 **Accepted / 구현·검증 완료**.
[최소 계약](../VAR_DESTRUCTURING_STAGE_B_PROPOSAL.md)·[62-production EBNF](../GRAMMAR_STAGE_B_VAR_DESTRUCTURING.ebnf)·[구현 기록](../VAR_DESTRUCTURING_IMPLEMENTATION.md)을 따른다.
두 파일 main/types·추가 정상 2·부정 18·Runtime 2, 24개 UTF-8/LF source bytes는 source_sha256으로 고정한다.
expected.json의 validated_result는 실제 Compiler/Native 결과다. 문서 validator 자체는 실행 검증이 아니다.

## 검증된 main 출력

```text
initializer
mutable=9/8/false/🙂
copy=5/true/original=9
discard-effect
unit-call
shadow=7
legacy=10
loop=3
fetch
after=false
ok=5
fetch
error=5
```

중첩 mutable binder·field/tuple path·annotation/type import·단일 initializer·Copy snapshot·shadow·기존 var `_`·discard/Unit effect·nullary Enum·loop·try를 관측한다.
scalars는 float32/double/char 직접 변경, mixed_paths는 nested Copy struct/tuple 대입과 원 aggregate 보존을 관측한다.
initializer_abort와 binder_abort는 원 checked operation Span·before-only 출력·exit 1을 debug/release에서 검증한다.

## 직접 실행

```powershell
cargo run -p nova-cli --offline -- check examples/var_destructuring.nova
$env:NOVA_CLANG=(Resolve-Path target/toolchains/llvm-21.1.8/bin/clang.exe).Path
cargo run -p nova-cli --offline -- run examples/var_destructuring.nova --profile debug
cargo run -p nova-cli --offline -- run examples/var_destructuring.nova --profile release
cargo run -p nova-cli --offline -- run docs/development-v0.1/var-destructuring-proposal-fixtures/main.nova --source-root docs/development-v0.1/var-destructuring-proposal-fixtures --profile debug
cargo run -p nova-cli --offline -- run docs/development-v0.1/var-destructuring-proposal-fixtures/main.nova --source-root docs/development-v0.1/var-destructuring-proposal-fixtures --profile release
cargo test --workspace --offline p27_
cargo test -p nova-codegen-llvm --test emission --offline p27_ -- --ignored --test-threads=1
cargo test -p nova-cli --test native --offline p27_ -- --ignored --test-threads=1
```

기본 13·실제 LLVM 1·Native 2개를 추가했다. [전체 실행 명령](../../../TESTING.md)을 제공한다.
P26 var_pattern.nova와 원 expected.json은 당시 N1102 기록으로 보존한다.
P27에서 해당 source의 check/native가 성공·빈 출력으로 바뀌었음을 historical_overrides와 전용 회귀에 명시한다.
let/match binder는 계속 immutable이며 refutable pattern·const 구조 분해·private/let field 변경은 성공으로 재해석하지 않는다.
