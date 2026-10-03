# Compiler CLI 명령·옵션 사양서

## 명령

```text
nova check
nova build
nova run
nova test
nova fmt
nova clean
nova doc
nova version
```

## 공통 옵션

```text
--manifest-path
--target
--profile debug|release|size
--color auto|always|never
--message-format human|json
--jobs
-v -vv
```

## Dump

```text
--emit=tokens|ast|hir|typed-hir|mir|llvm|obj|asm
```

## Exit Code

```text
0 성공
1 사용자 코드 오류
2 CLI·Manifest 오류
3 Toolchain·Linker 오류
101 ICE
```

Manifest 없는 `.nova` 파일은 임시 Package로 실행한다.
