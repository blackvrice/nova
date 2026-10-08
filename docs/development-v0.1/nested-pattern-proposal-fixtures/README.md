# P24 중첩 Copy 패턴·Tuple match 수용 fixture — Draft

**사용자 승인 대기 / 미구현**. [최소 계약](../NESTED_PATTERN_STAGE_B_PROPOSAL.md)·[제안 EBNF](../GRAMMAR_STAGE_B_NESTED_PATTERN.ebnf)를 따른다.
두 파일 main/types·추가 정상 3·부정 18·Runtime 1과 finite-domain coverage vector 6개다.
expected.json의 **proposed_result**는 향후 구현 수용 기준이다. implementation_verified=false이며 Native 실행 증거가 아니다.
현재 Compiler는 새 nested/tuple pattern을 N1102로 거부할 수 있다. 이 제안의 check_exit=0은 아직 검증하지 않았다.

## 제안 출력

```text
scrutinee
nested=7
packet=9/false/한
snapshot=true/6
original=false/false
single=true
unit
classify=3
after try
ok=4
error=5
loop=2
```

선택 arm·단일 scrutinee 평가·import nominal identity·Unicode·독립 snapshot·singleton Tuple·Unit·전체 return·try early return·loop jump다.
partial_overlap는 앞 arm과 일부 겹쳐도 남은 값을 처리한다. union_coverage는 여러 arm의 합집합으로 완전하다.
bare_binder의 값은 const 조회가 아닌 immutable struct Copy binder다. nested_abort는 선택된 int8 payload에서 checked Abort한다.
부정 사례는 recursive type/arity/scope·중복 binder·누락·합집합 unreachable·제외 문법을 UTF-8 byte Span으로 고정한다.

## 지금 실행할 수 있는 문서 검사

저장소 루트 PowerShell:

```powershell
node tools/docs/validate-pack.mjs
node tools/tests/nested-pattern-oracle.mjs
cargo fmt --check
cargo test --workspace --offline
```

oracle는 명시적인 작은 유한 domain을 열거해 제안 coverage vector만 검사한다.
문서 validator는 source/Span/grammar/ledger 검사다. 둘 다 Nova Compiler/Native 성공을 증명하지 않는다.

## 승인 후 구현 수용에 사용할 명령

아래 명령은 **아직 성공하지 않는 향후 수용 명령**이다. 승인·구현 후 debug/release의 위 12줄·빈 stderr·exit 0을 검증한다.

```powershell
cargo run -p nova-cli --offline -- check docs/development-v0.1/nested-pattern-proposal-fixtures/main.nova --source-root docs/development-v0.1/nested-pattern-proposal-fixtures
$env:NOVA_CLANG = (Resolve-Path target/toolchains/llvm-21.1.8/bin/clang.exe).Path
cargo run -p nova-cli --offline -- run docs/development-v0.1/nested-pattern-proposal-fixtures/main.nova --source-root docs/development-v0.1/nested-pattern-proposal-fixtures --profile debug
cargo run -p nova-cli --offline -- run docs/development-v0.1/nested-pattern-proposal-fixtures/main.nova --source-root docs/development-v0.1/nested-pattern-proposal-fixtures --profile release
```

현재 구현된 P23 실행은 [TESTING.md](../../../TESTING.md)의 examples/struct_named_arguments.nova 명령을 사용한다.
