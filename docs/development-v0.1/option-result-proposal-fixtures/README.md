# P15 Copy Option·Result·nullable 수용 fixture

상태 **Accepted / 사용자 승인 완료 / 구현 완료**. [최소 계약](../OPTION_RESULT_STAGE_B_PROPOSAL.md)과
[51-production EBNF](../GRAMMAR_STAGE_B_OPTION_RESULT.ebnf)의 수용 source·검증 기대값이다.
문서 validator는 metadata/UTF-8 byte Span만 검사하며 Compiler/Native를 실행하지 않는다.

## 정상 프로그램과 검증 출력

두 파일은 nullable↔Option identity, Copy snapshot, Result Tuple·nominal Enum payload/import alias,
private factory·nested none과 Success(Unit)을 사용한다. 다음 stdout과 각 LF·stderr 없음·exit 0을 확인했다.

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

아래 명령으로 두 파일을 check/debug/release에서 실행할 수 있다.

```powershell
cargo run -p nova-cli --offline -- check docs/development-v0.1/option-result-proposal-fixtures/main.nova --source-root docs/development-v0.1/option-result-proposal-fixtures
$env:NOVA_CLANG = (Resolve-Path target/toolchains/llvm-21.1.8/bin/clang.exe).Path
cargo run -p nova-cli --offline -- run docs/development-v0.1/option-result-proposal-fixtures/main.nova --source-root docs/development-v0.1/option-result-proposal-fixtures --profile debug
cargo run -p nova-cli --offline -- run docs/development-v0.1/option-result-proposal-fixtures/main.nova --source-root docs/development-v0.1/option-result-proposal-fixtures --profile release
```

## 부정 사례와 수용 기준

[expected.json](expected.json)의 21개 부정 source는 Compiler의 지정 code와 정확한 primary byte Span을 검증한다.
문맥 부족·wrapping/widening·family/payload/arity·Copy·coverage·binder·cycle·user generic·try 거부를 포함한다.
파생 진단은 추가될 수 있으나 지정 code/Span은 실제 결과에 있어야 한다.

END/>= token adapter, multi-file shadow/private factory, 자원 직전/초과·const budget,
독립 layout oracle·malformed specialization/CFG tag proof와 Native O0/O2는 [구현 기록](../OPTION_RESULT_IMPLEMENTATION.md)의 별도 실행 증거를 따른다.
직접 실행할 standalone 예제와 검사 명령은 [TESTING.md](../../../TESTING.md)를 따른다.
