# P28 함수 내부 Copy const 구조 분해 수용 fixture

2026-10-12 사용자 다음 단계 승인에 따른 **Accepted / 구현·검증 완료**.
[최소 계약](../CONST_DESTRUCTURING_STAGE_B_PROPOSAL.md)·[62-production EBNF](../GRAMMAR_STAGE_B_CONST_DESTRUCTURING.ebnf)·[구현 기록](../CONST_DESTRUCTURING_IMPLEMENTATION.md)을 따른다.
두 파일 main/types·정상 2·부정 20·Runtime 1, 25개 UTF-8/LF source를 source_sha256으로 고정한다.
validated_result는 실제 Compiler/Native 결과이며 문서 validator 자체는 실행 검증이 아니다.

## 검증된 main 출력

```text
cached=7/true/🙂
derived=15/false
point=7/7/9
scalars=255/1.5
sum=4/true
single=6
shadow=4
legacy=9
loop=1/2/true
loop=2/2/true
```

상수 Enum/Tuple/Unit·primitive·type import/annotation·named Copy struct·cached binder dependency·projection/exists·shadow·기존 const `_`·loop를 관측한다.
short_circuit는 skipped division-by-zero를, opaque_const는 cached Copy struct와 원 값의 보존을 확인한다.
runtime_abort는 const binder를 사용하는 일반 runtime 산술의 overflow Source Span·before-only 출력·exit 1을 두 profile에서 확인한다.

## 직접 실행

```powershell
cargo run -p nova-cli --offline -- check examples/const_destructuring.nova
$env:NOVA_CLANG=(Resolve-Path target/toolchains/llvm-21.1.8/bin/clang.exe).Path
cargo run -p nova-cli --offline -- run examples/const_destructuring.nova --profile debug
cargo run -p nova-cli --offline -- run examples/const_destructuring.nova --profile release
cargo run -p nova-cli --offline -- run docs/development-v0.1/const-destructuring-proposal-fixtures/main.nova --source-root docs/development-v0.1/const-destructuring-proposal-fixtures --profile debug
cargo run -p nova-cli --offline -- run docs/development-v0.1/const-destructuring-proposal-fixtures/main.nova --source-root docs/development-v0.1/const-destructuring-proposal-fixtures --profile release
cargo test --workspace --offline p28_
cargo test -p nova-codegen-llvm --test emission --offline p28_ -- --ignored --test-threads=1
cargo test -p nova-cli --test native --offline p28_ -- --ignored --test-threads=1
```

기본 17·실제 LLVM 1·Native 2개를 추가했다. [전체 사용자 테스트 안내](../../../TESTING.md)를 제공한다.
P26/P27 const_pattern.nova와 원 N1102 metadata를 보존하고 현재 성공/빈 출력을 historical_overrides와 전용 회귀로 명시한다.
전역/visibility global compound const는 계속 N1102다. N3103·N3201/N3202·불변/Copy/private 규칙을 성공으로 재해석하지 않는다.
