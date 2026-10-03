# Package/Lock/Artifact/Cache schema 초안

상태 Draft D21/D28/D29. TOML parser/JSON serializer 라이브러리 선택은 구현 사항이다.

## nova.toml v1

```toml
schema-version = 1

[package]
name = "hello"
version = "0.1.0"
edition = "0.1"

[[target]]
name = "hello"
kind = "bin"
path = "src/main.nova"

[dependencies]
math = { path = "../math" }
# Git source alternative: { git = "...", rev = "full immutable revision" }

[profile.debug]
opt-level = "0"
debug-info = true

[profile.release]
opt-level = "2"
debug-info = false
```

package name는 ASCII [a-z][a-z0-9_-]* 후보, version는 semver, edition=0.1이다. kind=bin/lib,
target name 중복 금지, target path는 package 내부 .nova다. path dependency의 ..는 명시 참조
가능하지만 package source/archive extraction/output paths가 의도된 경계를 벗어나면 오류다.
manifest-relative resolution, case-collision 검사, duplicate key/unknown schema 거부.
registry/build scripts/post-install hooks는 최초안에서 제외한다.

## nova.lock v1

```toml
schema-version = 1

[[package]]
name = "math"
version = "0.1.0"
source = "path:../math"
checksum-algorithm = "sha256-tree-v1"
checksum = "64-hex-content-fingerprint"
dependencies = []
```

예제 checksum은 placeholder이며 실제 유효 lock 값이 아니다. git source는 full revision+
content digest. registry exact version/source/checksum 필드는 후속 source 지원 시 정의한다.
--locked는 stale lock 변경 거부, --offline은 cached content 없으면 거부. path source 수정도
fingerprint mismatch로 감지하며 --locked 사용 시 묵시 업데이트하지 않는다.

## Tree fingerprint 제안

manifest, .nova, 명시 runtime/std inputs의 상대 경로 UTF-8 byte 정렬 → path length+path+
content length+content bytes를 length-prefix encoding → SHA-256. .git/target/cache와 원본
metadata mtime는 제외한다. symlink는 기본 거부 또는 resolved target를 명시 pin하는 선택안을
승인해야 한다. 원 파일 검색 순서, OS separator, timestamp를 digest에 넣지 않는다.

## Artifact metadata v1

```json
{
  "schema_version": 1,
  "compiler_version": "0.1.x",
  "language_version": "0.1",
  "runtime_abi": 1,
  "target": "x86_64-pc-windows-msvc",
  "package_identity": {"name":"hello","version":"0.1.0","source_fingerprint":"..."},
  "profile": "debug",
  "semantic_options_hash": "...",
  "dependencies": [],
  "exports": [],
  "object_checksum": "..."
}
```

exports는 DefPath/Signature/Mode/Visibility/Layout ABI hash이며 untrusted metadata bounds,
resource limits/schema/compiler/Target/runtime/checksum을 검사한다. incompatible artifact는
source rebuild 또는 명확 오류이며 native loader에 바로 넘기지 않는다.

## Cache record

key={compiler,schema,Target,runtimeABI,profile,semanticOptions,source,dependency fingerprints};
value={result kind,artifact checksum,dependency keys,diagnostics schema}. write temp→verify→atomic
rename, concurrent lock, corrupt miss/rebuild. unsigned/untrusted cache artifact를 실행할 때
trust policy를 별도 정해야 한다. nova clean은 명시 output/cache만 제거한다.
