# P23 Copy struct 생성자 이름 인수 수용 fixture — Accepted

**2026-10-08 사용자 승인 / 구현 완료**. [최소 계약](../STRUCT_NAMED_ARGUMENTS_STAGE_B_PROPOSAL.md)과
[재사용하는 승인 P22 58-production EBNF](../GRAMMAR_STAGE_B_METHOD.ebnf)를 따른다.
main/types 두 파일·추가 정상 2·부정 24·Runtime 1사례다. expected.json의 validated_result를 Compiler/Native에서 검증했다.
초안 작성 때의 Draft 주석은 역사 기록으로 남기고 원 source UTF-8/LF bytes·Span은 보존했다.

## 검증된 출력

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

20줄·각 LF·빈 stderr·exit 0이다. 원 nominal import alias·mixed field/method·source order와 field order 차이·
mapped 숫자/Char/Tuple/Option/Result·field self·const/default module scope·private factory·Copy·try early return을 포함한다.
multiline은 colon/trivia/개행/trailing comma, value_namespace는 같은 이름 함수 우선/default를 검증했다.
부정 24사례의 진단 코드·primary UTF-8 byte Span·cascade 금지는 driver 회귀와 실제 CLI check에서 모두 일치했다.
constructor_abort는 check exit 0, Native exit 1이며 before LF 이후 n+1에서 Abort한다. later/after는 출력하지 않는다.

## 직접 실행

저장소 루트 PowerShell에서 실행한다. check는 출력 없이 exit 0이다.

```powershell
cargo run -p nova-cli --offline -- check docs/development-v0.1/struct-named-arguments-proposal-fixtures/main.nova --source-root docs/development-v0.1/struct-named-arguments-proposal-fixtures
$env:NOVA_CLANG = (Resolve-Path target/toolchains/llvm-21.1.8/bin/clang.exe).Path
cargo run -p nova-cli --offline -- run docs/development-v0.1/struct-named-arguments-proposal-fixtures/main.nova --source-root docs/development-v0.1/struct-named-arguments-proposal-fixtures --profile debug
cargo run -p nova-cli --offline -- run docs/development-v0.1/struct-named-arguments-proposal-fixtures/main.nova --source-root docs/development-v0.1/struct-named-arguments-proposal-fixtures --profile release
```

debug/release에서 위 20줄을 정확히 검증했다. [독립 예제/명령](../../../TESTING.md)도 제공한다.

## 실제 검증 — 2026-10-08

- fmt·check·clippy -D warnings·Runtime rustfmt·문서 validator: PASS.
- 기본 전체 workspace: **372 PASS / 0 FAIL / 73 ignored**. ignored 테스트는 다음 opt-in에서 별도로 실행했다.
- LLVM 전체 **19 PASS**: LLVM 21.1.8, COFF/ELF O0/O2.
- Windows x64 Native 전체 **54 PASS**: debug/release. 총 **445 PASS / 0 FAIL**.
- 16개 새 회귀: 기본 13·LLVM opt-in 1·Native opt-in 2. source 변조/UTF-8 복구·field mapping 변조·same-type operand/CFG/effect/snapshot·try early return·
  const dependency·10,000/10,001 투명 wrapper 비용·1,024 field 완전한 MIR proof를 검사한다.
- P17 struct_label.nova는 P17 당시 위치 전용 부정 기록으로 bytes/expected metadata를 보존했다.
  P23가 이 생성만 확장하므로 현재 해당 source는 성공하며 P17 회귀에서 새 성공 기준을 명시했다.
  나머지 승인 P01~P22 계약/grammar/ledger와 원본 148개를 보존했다.

문서 validator는 데이터/링크/Span을 검사한다. Compiler/Native 실행 증거는 별도 회귀 로그와 [구현 기록](../STRUCT_NAMED_ARGUMENTS_IMPLEMENTATION.md)이다.
