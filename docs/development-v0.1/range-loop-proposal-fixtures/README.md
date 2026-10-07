# P19 loop·정수 range for 제안 수용 fixture

상태: **Draft / 사용자 승인 대기 / 미구현**. expected.json의 proposed_result는 제안 기대값이다.
Compiler/Native 검증 완료 기록이 아니다. [계약](../RANGE_LOOP_STAGE_B_PROPOSAL.md)·[문법](../GRAMMAR_STAGE_B_RANGE_LOOP.ebnf).

- main/helpers 두 파일: import alias·P18 default/named bound calls·until/through·빈 범위·uint8 MAX continue·mixed promotion·shadow·nested while·loop·try.
- 정상 2개: Unicode outer scope·bound snapshot·inner shadow·int64 MIN/uint64 MAX·equal/reversed·loop/for/match jump.
- 부정 18개: 정수 제약·peer 범위·ErrorType cascade·binder scope/mutability/duplicate·range/iterable/pattern/return 미지원 경계.
- Runtime 2개: UTF-8 bound/body overflow의 N5201 원 expression Span과 Abort 전 effect.
  Span은 반열린 UTF-8 byte offset이다. syntax/type 실패에서 Backend/도구를 호출하지 않는 gate도 승인 후 검사한다.

## 승인·구현 후 사용할 명령

현재 Compiler는 이 구문을 지원하지 않는다. 다음 명령은 구현 후 사용할 테스트 절차다.

```powershell
cargo run -p nova-cli --offline -- check docs/development-v0.1/range-loop-proposal-fixtures/main.nova
$env:NOVA_CLANG = (Resolve-Path target/toolchains/llvm-21.1.8/bin/clang.exe).Path
cargo run -p nova-cli --offline -- run docs/development-v0.1/range-loop-proposal-fixtures/main.nova --profile debug
cargo run -p nova-cli --offline -- run docs/development-v0.1/range-loop-proposal-fixtures/main.nova --profile release
```

제안 stdout 18줄/LF·빈 stderr·exit 0:

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

문서 metadata 검사는 `node tools/docs/validate-pack.mjs`다. 실제 compile/Native 검증과 구분한다.
