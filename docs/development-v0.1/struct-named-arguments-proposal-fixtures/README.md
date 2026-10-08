# P23 Copy struct 생성자 이름 인수 수용 계획 — Draft

**미승인 / 미구현**. [최소 계약](../STRUCT_NAMED_ARGUMENTS_STAGE_B_PROPOSAL.md)과
[재사용하는 승인 P22 58-production EBNF](../GRAMMAR_STAGE_B_METHOD.ebnf)를 검토한다.
main/types 두 파일·추가 정상 2·부정 24·Runtime 1사례이며 expected.json의 proposed_result는 제안값이다.
Compiler/Native 수용 테스트 통과나 사양 승인을 뜻하지 않는다. 새 fixture source는 UTF-8/LF다.

## 승인 후 제안 출력

```text
y
x
order=1/2
prefix
named
mixed=3/4
mapped=7/300
default=5/6
const=3/20
nested=3/true/한/true
self=7
private=8
empty=0
copy=1/6
fetch
later
constructed
ok=1
fetch
error=-1
```

20줄·각 LF·빈 stderr·exit 0을 제안한다. 원 nominal import alias·mixed field/method·source order와 field order 차이·
mapped 숫자/Char/Tuple/Option/Result·field self·const/default module scope·private factory·Copy·try early return을 포함한다.
multiline은 colon/trivia/개행/trailing comma, value_namespace는 같은 이름 함수 우선/default를 검토한다.
부정 24사례는 primary의 UTF-8 byte Span과 cascade 금지를 고정한다. Runtime은 before LF 이후 n+1에서 Abort하며 later/after는 출력하지 않는다.

## 지금 확인할 수 있는 명령

```powershell
node tools/docs/validate-pack.mjs
cargo test --workspace --offline
```

문서 validator는 proposed_result/Span/링크를 검사하며 Compiler/Native를 실행하지 않는다.
현재 Compiler는 다음 P23 entry의 이름 생성에 N2201을 보고하고 exit 1로 종료한다. 아직 성공 실행 예제가 아니다.

```powershell
cargo run -p nova-cli --offline -- check docs/development-v0.1/struct-named-arguments-proposal-fixtures/main.nova --source-root docs/development-v0.1/struct-named-arguments-proposal-fixtures
```

## 승인 후 구현에서 검증할 명령

다음 명령의 성공/제안 출력은 이번 문서 준비 단계에서 검증하지 않았다.

```powershell
$env:NOVA_CLANG = (Resolve-Path target/toolchains/llvm-21.1.8/bin/clang.exe).Path
cargo run -p nova-cli --offline -- run docs/development-v0.1/struct-named-arguments-proposal-fixtures/main.nova --source-root docs/development-v0.1/struct-named-arguments-proposal-fixtures --profile debug
cargo run -p nova-cli --offline -- run docs/development-v0.1/struct-named-arguments-proposal-fixtures/main.nova --source-root docs/development-v0.1/struct-named-arguments-proposal-fixtures --profile release
```

const dependency/cycle·skipped RHS·10,000/10,001 투명 wrapper 비용·자원 경계·Source/Checked/MIR 위조 회귀는
승인 후 자동 테스트를 추가한다. 기존 P17 struct_label 부정 사례는 P17 당시 위치 전용 계약 기록으로 보존하며,
P23 구현 테스트에서 해당 현재 소스가 새 성공 기준으로 바뀌는 점을 별도 기록한다.

## 이번 문서 준비 단계의 실제 검증 — 2026-10-08

- 문서 validator와 cargo fmt/check/clippy gate: PASS. runtime stage_a.rs rustfmt: PASS.
- 기존 cargo test --workspace --offline: **359 PASS / 0 FAIL / 70 ignored**.
  Windows hard-link/junction 권한 검사는 sandbox 밖에서 동일 명령으로 재검증했다.
  ignored LLVM/Native 전체 수용 테스트는 이번 문서 변경에서 실행하지 않았다.
- P01~P22 Accepted ledger·승인 계약/grammar/fixture raw bytes·D01~D30 결정 표·Canonical·원본 148개 SHA-256 보존.
  Compiler/Runtime/Cargo/vendor도 기준 9da0d80과 동일하다. source는 Git newline 정규화를 적용해 비교했다.
- 현재 P23 main의 check는 **exit 1 / N2201**이다. 미지원 인수 문맥으로 sum의 N2103도 발생한다.
  구문은 기존 NamedArgument로 파싱되지만 P23 의미는 구현되지 않았다.
- 예상값의 타당성만 확인하려고 ignored target 안에 위치 생성과 명시적 source-order 임시 변수를 사용하는 별도 예제를 만들었다.
  그 기존 기능 예제는 check와 debug Native에서 제안 20줄 UTF-8/LF·빈 stderr·exit 0이었다.
  원 P23 fixture의 Compiler/Native 성공이나 이름 생성자 구현 검증으로 계산하지 않는다. expected.json은 계속 Draft/false다.
