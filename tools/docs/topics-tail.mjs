export const topicsTail = {
105: `## 확정 foreign 예제
foreign c NativeMath { library "native_math"; @symbol("native_add") func add(first: int32, second: int32) -> int32 }의 핵심 철자는 원본을 따른다. 실제 선택 세미콜론/END는 grammar로 정규화한다.

## 선언 계약
foreign 함수는 body가 없다. ABI-safe type만 raw 선언에 사용하고 unsafe adapter → safe wrapper → application 계층으로 분리한다. owned handle에는 release 계약이 필요하다.

## 상세 초안 — D17
library와 @symbol은 contextual syntax다. 함수 Import/Export, pointer, opaque handle, callback+context를 지원 범위로 제안하며 C variadic/C++ exception/자동 bitfield mapping은 제외다.

## 검증
unknown adapter/library/symbol, foreign body 금지, ABI-unsafe string/Array 직접 전달 거부, callback context lifetime, safe wrapper ownership.`,
106: `## C ABI 경계
Nova 내부 ABI와 C ABI를 구분한다. scalar fixed-width, opaque pointer, explicit C representation aggregate만 허용하는 초안을 제안한다. Nova bool/char/string/Option/Result를 C int/string에 암묵 대응시키지 않는다.

## 계약 — D16/D17
C integer signedness/width, calling convention, symbol, nullable pointer, length units, out parameter, callback lifetime을 명시한다. C errno/status는 wrapper에서 Result로 매핑하고 runtime ownership로 raw resource를 감싼다.

## 검증
C가 부른 Nova export/Nova가 부른 C import, large aggregate return, Windows/Linux ABI differences, null/output buffer/callback roundtrip. C shim이 예외를 포착하며 unwind가 ABI를 넘지 않는다.`,
107: `## Binding generator 범위 초안 — D17
검증된 C frontend의 AST/ABI 정보에서 Nova foreign 선언을 생성한다. header text를 정규식으로 추측하지 않는다. include paths/macros/Target/compiler flags를 generation input으로 기록한다.

## 지원/거부
fixed-width scalar, pointer, opaque record, fixed signature function은 후보. variadic/bitfield/C++ template/exception-dependent function은 자동 safe binding을 생성하지 않는다. pointer ownership/length는 header만으로 추정할 수 없으므로 annotation manifest가 필요하다.

## 검증
reproducible output, typedef resolution, Target-dependent width, macro-conditioned signature, unsupported construct explicit report. generated raw binding을 safe wrapper 완료로 표시하지 않는다.`,
108: `## Opaque handle wrapper
raw pointer와 safe owned handle를 구분한다. constructor 성공 시 non-null handle를 owner에 저장, read operation은 borrow, transfer operation은 take, drop은 matched release다.

## 초안 — D17
library lifetime은 모든 handle/callback보다 길어야 한다. null은 Option/Result wrapper로 처리한다. thread-affinity/thread-safety annotation은 compiler predicate에 연결하며 unknown은 conservative다.

## 검증
double release/use after release/null handle 거부, release allocator mismatch 방지, callback 재진입, library unload-before-handle fail. foreign pointer가 가리키는 memory를 Nova allocator로 해제하지 않는다.`,
109: `## 외부 예외
외부 exception은 Nova call stack/ABI로 직접 전파하지 않는다. C/C++ shim 또는 hosted adapter가 포착하여 status+error payload를 반환하고 safe wrapper가 Result<T,E>로 바꾼다.

## 초안 — D17
변환 불가능한 예외와 계약 위반은 Abort를 제안한다. error message의 소유권과 encoding, release 함수, original error category를 명시한다. foreign success payload가 생성되기 전에 실패하면 미초기화 값 Drop을 하지 않는다.

## 검증
success/error/shim catch/unknown exception, error string release, partially constructed owned result cleanup, try propagation. Nova panic은 foreign catch로 복구 가능한 예외가 아니다.`,
110: `## FFI Ownership 표
Borrowed: caller owner 유지/호출 또는 지정 기간 동안만 사용. Owned-in: take로 전달/성공·실패 시 소비 여부 명시. Owned-out: 성공 결과에 새 owner/release 부여. Shared: retain/release pair 명시.

## 계약 초안 — D17
각 raw binding에 validity, mode, duration, nullability, length/alignment, error-path ownership, allocator, thread affinity를 기록한다. status가 실패해도 callee가 resource를 소비하는지 별도 필드다.

## 검증
success/error 모두 release count, take 뒤 재사용 fail, foreign-retained borrowed pointer 거부, mismatched free, callback context Drop ordering. Drop glue는 foreign release 호출을 한 번만 생성한다.`,
111: `## .NET Hosted Adapter 범위
Nova 0.1 확정 범위는 C FFI다. .NET host adapter는 추가 런타임 의존성을 갖는 후속 문서이며 현재 구현 완료 조건이 아니다(D27).

## 후속 계약
runtime startup/version, managed reference rooting/pinning, GC↔Nova ownership, exception translation, thread attach, string encoding, callback lifetime, assembly resolution을 설계해야 한다. managed pointer를 무기한 안전 view로 노출하지 않는다.

## 검증 조건
GC stress 중 handle validity, exception boundary, unload/reload, cross-thread callback, exactly-once release. 이러한 증거 없이 foreign dotnet 구문을 compiler에 추가하지 않는다.`,
112: `## JVM Hosted Adapter 범위
JVM adapter는 C FFI와 다른 후속 runtime feature이며 D27 승인 없이는 Nova 0.1 구현 범위에 넣지 않는다.

## 후속 계약
VM creation/attach/detach, local/global references, Java exception→Result, classpath/version, UTF conversion, callback thread, direct-buffer lifetime, host shutdown을 명시한다. local reference를 호출 종료 뒤 safe owner처럼 저장하지 않는다.

## 검증 조건
GC/moving reference stress, pending exception handling, attach leak, Unicode supplementary char, direct buffer invalidation. C ABI entry만 사용한다고 JVM 수명 문제가 해결되는 것은 아니다.`,
113: `## Python Hosted Adapter 범위
Python adapter는 후속 feature(D27)다. Nova compiler 구현 언어 Rust와도 무관하다. 0.1 C FFI만으로 Python semantics를 자동 제공하지 않는다.

## 후속 계약
interpreter version/init/finalize, object retain/release, thread/GIL 정책, exception/result, bytes/UTF-8 string 구별, extension ownership, callback reentrancy, module loading을 명시한다.

## 검증 조건
object refcount, interpreter 종료 뒤 handle 사용 금지, exception cleanup, thread callback, Python 문자열/bytes roundtrip. version별 host API는 실제 도입 시 공식 문서로 검증한다.`,
114: `## Prelude 초안 — D23
최소 Prelude는 Primitive/Unit, Option(Some/None), Result(Success/Error), Array, panic, Stage A print를 제안한다. List/Map/file/thread는 explicit module import를 요구한다.

## 이름 계약
Prelude symbol도 DefId를 가지며 user local shadow 정책은 이름 해석 D06을 따른다. print를 Parser special case가 아니라 builtin definition/runtime intrinsic으로 연결한다. private runtime symbol을 source name으로 노출하지 않는다.

## 검증
import 없는 Hello Nova, shadowed print 처리, std 없는 core compilation mode, duplicate prelude definition, qualified std 접근. module/version별 API 목록은 STDLIB_API와 일치한다.`,
115: `## Primitive API 초안 — D23
정수 checkedAdd/checkedSub/checkedMul → Option<Self>, wrappingAdd/wrappingSub/wrappingMul → Self를 제안한다. normal operator는 D07의 checked/Abort 정책과 구분한다. parse(text) → Result<T,ParseError>, toString() → string.

## float/char
float isNaN/isInfinite/abs, char isAscii와 scalar 검사 API를 제안한다. locale-dependent formatting은 core 기본에서 제외한다. narrowing은 explicit checked cast 계약을 따른다.

## 검증
MIN/MAX wrapping, parse whitespace/sign/base/overflow, NaN/-0 출력 정책, Unicode scalar char. API signature의 allocation/panic/cost/mode를 STDLIB_API에서 표시한다.`,
116: `## string 확정 의미
owned UTF-8이며 기본 index 연산은 없다. byte/scalar/grapheme API를 구분한다. UTF-8 검증 없는 bytes를 string으로 조용히 재해석하지 않는다.

## API 초안 — D23
byteLength, isEmpty, bytes(ReadOnlySpan<byte>), scalarAt(index)→Option<char>, sliceBytes(start,end)→Result<string,BoundaryError>, fromUtf8(Array<byte>)→Result<string,Utf8Error>를 제안한다. slice는 byte boundary 확인 후 owned copy이며 zero-copy view API는 별도다.

## 검증
emoji/결합문자 scalar와 grapheme 차이, boundary 중간 slice fail, invalid UTF-8, bytes view 후 owner move fail, copy allocation/Drop. grapheme segmentation은 Unicode 버전 고정 후 std 확장으로 검토한다.`,
117: `## Array<T> 메모리
ptr/len/capacity와 initialized prefix를 관리한다. 0≤len≤cap, element size×cap overflow 없음, initialized 원소만 읽고 정확히 한 번 Drop한다.

## API 초안 — D23
length/capacity, push(take value), pop()→Option<T>, reserve(additional), readAt(index)→view T, changeAt(index)→change view T, asReadOnlySpan/asSpan을 제안한다. indexing bounds 실패는 panic, checked get은 Option view다.

## 검증
growth allocation, zero-sized T, Move T reallocation, pop ownership, reserve overflow, view live 중 push 거부. List는 Array와의 관계가 승인될 때까지 별도 중복 storage를 구현하지 않는다.`,
118: `## View 의미
ReadOnlySpan<T>는 read view, Span<T>는 change view다. owner보다 오래 살 수 없으며 ptr+length ABI와 region origin을 갖는다. Compiler source Span과 별개의 언어 타입이다.

## API 초안 — D10/D23
length, checked slice(start,end), readAt/changeAt, splitAt(index)→두 비중첩 view를 제안한다. mutable split은 원 view를 소비/재빌림해 두 child view가 live 동안 parent access를 금지한다.

## 검증
0/len split, index>len error, child overlap 없음, owner Drop/Move 금지, parent 재사용 시점, dynamic-index 임의 disjoint 주장 거부. ABI pointer pair가 lifetime 검사를 대체하지 않는다.`,
119: `## Option<T>
Some(T)/None variant와 match를 사용하며 T?는 같은 타입이다. move payload의 ownership가 보존되고 niche는 internal optimization이다.

## API 초안 — D23
isSome/isNone는 Read; take unwrap→T는 None일 때 panic; take unwrapOr(take fallback)→T는 fallback도 source order로 평가한다. lazy fallback API는 Closure가 준비된 D 이후에 정의한다. implicit null/Some wrapping은 제외 제안이다.

## 검증
Some owner unwrap 이후 Option moved, None fallback Drop, Copy T copy 규칙, nested nullable, map API callback mode가 기존 payload를 중복 소비하지 않는지 검사한다.`,
120: `## Result<T,E>
Success(T)/Error(E), Unit 성공 Success(()), try Error 조기 반환을 사용한다. payload mode와 enclosing E mismatch를 검사한다.

## API 초안 — D23
isSuccess/isError는 Read; take unwrap→T는 Error panic; take unwrapError→E는 Success panic. map/mapError/andThen는 Closure 지원 이후 signatures를 확정하며 call mode와 Error cleanup을 명시한다.

## 검증
Move T/E 정확히 한 번, try chain order, success/Error branch inactive Drop 없음, unwrap panic stderr, no automatic E conversion.`,
121: `## List/Map 초안 — D23
Array가 이미 growable owner이므로 List<T>는 Array의 별칭/편의 API로 제안하고 별도 같은 buffer type를 중복 구현하지 않는다. Map<K,V>는 hash-based owning container 후보이며 Hash/Equality generic constraint는 D15와 함께 검토한다.

## 계약
iteration order는 명시 보장 없으면 unspecified; compiler 결정적 output에 Map 순서를 그대로 쓰지 않는다. insert는 old value ownership를 반환하는 Option<V>, remove도 Option<V>를 제안한다. iterator live 중 structural mutation을 금지한다.

## 검증
collision equality, replacing Drop, key/value Move, rehash with views, adversarial hash input, deterministic compiler serialization sorting.`,
122: `## I/O API 초안 — D23
print(text:string)→Unit는 Stage A UTF-8 stdout line 출력 후보다. println 별도 여부와 newline behavior는 D23 승인 대상이다. 파일/stream open/read/write/flush는 Result와 structured IoError를 반환한다.

## Resource 계약
File/Stream은 Move owner이며 drop은 close, close error를 destructor에서 반환할 수 없으므로 explicit close()→Result로 제공한다. partial read/write byte 수, EOF와 error를 구분한다. path encoding은 platform abstraction을 따른다.

## 검증
Hello exact stdout, embedded NUL/Unicode, partial write, permission denied, close double-use fail, using cleanup, broken pipe policy. print failure를 무조건 성공으로 숨기지 않는다.`,
123: `## 시간/난수/환경 초안 — D23
wallClock timestamp와 monotonic Duration를 구분한다. elapsed 계산은 monotonic만 사용한다. secureRandom(bytes)→Result와 seedable deterministic RNG를 다른 API로 제공한다.

## 환경 계약
getEnv(name)→Result<Option<string>,EnvError>, args()→Array<string>를 제안한다. 비UTF-8 OS 값은 명시 변환 오류 또는 별도 raw API이며 replacement를 숨기지 않는다. 환경의 부작용은 pure가 아니다.

## 검증
clock adjustment에도 monotonic duration, deterministic seed corpus, OS RNG failure, missing vs empty env, invalid encoding. secret env 값을 diagnostic/cache dump에 자동 넣지 않는다.`,
124: `## Thread API 초안 — D20/D23
spawn(take closure)→Result<ThreadHandle,ThreadError>, join(take handle)→Result<Unit,ThreadError>, Mutex<T>.lock()→Guard<T>, Atomic<intN> 명시 operations를 제안한다. Thread/atomic payload-return generic 확장은 signature 설계와 함께 검토한다.

## 수명/모드
closure가 borrowed local view를 캡처하여 owner보다 오래 살아서는 안 된다. Guard는 change 접근을 제공하며 Drop unlock, send/share predicate를 만족해야 한다. Mutex unlock이 panic cleanup을 보장한다는 뜻은 아니다; Abort다.

## 검증
take closure 이후 재사용 fail, thread view escape fail, guard return cleanup, atomic invalid order fail, runtime concurrent race corpus.`,
125: `## CLI 확정 표
check/build/run/test/fmt/clean/doc/version; common --manifest-path/--target/--profile/--color/--message-format/--jobs/-v; emit=tokens/ast/hir/typed-hir/mir/llvm/obj/asm를 따른다.

## exit
0 성공, 1 user source 오류, 2 CLI/Manifest, 3 Toolchain/Linker, 101 ICE. manifest 없는 .nova는 임시 package다. run은 성공 build 후 child를 실행하며 child exit 전달과 compiler error exit 구분은 D22 초안이다.

## 검증
unknown option exit2, invalid source exit1, missing clang/linker exit3, ICE101, JSON stdout/diagnostic stderr 혼합 방지, no build on check. CLI_CONTRACTS에서 side effects/output paths를 정의한다.`,
126: `## Manifest 초안 — D21
nova.toml은 schema-version, [package], [[target]], [dependencies], [profile.*]를 가진다. package name/version/edition, target kind/bin/lib/path를 validation한다. MANIFEST_SCHEMA에 최소 예제가 있다.

## 처리
duplicate key, unknown schema major, 잘못된 version/path/target를 명확히 거부한다. relative path는 manifest directory 기준이며 dependency alias 중복을 허용하지 않는다. build scripts/network hooks는 0.1 첫 승인안에서 제외한다.

## 검증
최소 bin/lib, duplicate dependencies, path escape policy, manifest 없는 source temporary package, unicode directory, source root collision. TOML library 선택은 implementation detail다.`,
127: `## Lock 계약
nova.lock은 exact version/source/revision/checksum과 graph를 기록한다. unresolved range가 남지 않으며 registry 서버는 0.1에 구현하지 않는다.

## 초안 — D21
path dependency는 source content fingerprint, git dependency는 full immutable revision을 기록한다. --locked는 변경 필요 시 오류, --offline은 cached source 없으면 오류로 제안한다. resolver는 semver constraint backtracking와 conflict chain을 출력한다.

## 검증
conflict, checksum mismatch, source alias, missing offline package, locked build graph 동일, atomic lock write. SHA-256 source archive checksum과 local tree fingerprint 포맷을 혼용하지 않는다.`,
128: `## Build profile 초안 — D26/D28
debug/release/size는 optimization/debug info/LTO/codegen units 설정을 가진다. profile마다 overflow/bounds 의미를 바꾸지 않는다. Target triple/sysroot/linker/runtime ABI는 별도 toolchain input이다.

## 기본 제안
debug O0+debug info, release O2, size Os; fast-math off, LTO는 opt-in. cross compile은 target std/runtime/linker가 모두 있어야 하며 host executable run을 자동 시도하지 않는다.

## 검증
모든 profile 동일 observable semantics, unsupported target exit3, missing sysroot, --target output directory 분리, options cache key 일치.`,
129: `## Artifact metadata 초안 — D21
schema/compiler/language/runtime ABI/Target/profile/package identity/source fingerprint/dependency fingerprints/exports/layout ABI hashes/checksum을 기록한다. object file만 있어도 export/type 정보를 추정할 수 있다는 가정을 금지한다.

## trust
untrusted metadata는 bounds/depth/schema/checksum 검증 후 로드한다. version mismatch면 source rebuild 또는 명시 error다. 경로와 symbol을 shell command로 연결하지 않는다.

## 검증
corrupt/truncated metadata, mismatched Target/ABI/compiler, forged checksum, large graph resource budget, deterministic canonical serialization.`,
130: `## Disk cache 초안 — D21
key는 normalized source+transitive relevant dependencies+compiler/runtime/Target/options/schema를 hash한다. executable cache와 frontend query cache를 별도 저장한다.

## Storage
temp write→checksum→atomic rename, per-entry concurrency lock, stale lock 복구, bounded eviction을 제안한다. damaged entries는 실패 성공으로 사용하지 않고 miss/rebuild한다. absolute workspace path normalization이 SourceInfo를 손상시키지 않아야 한다.

## 검증
unchanged hit, compiler flag change miss, std ABI change miss, interruption mid-write, parallel writer, full-cache permission error graceful handling. cache disable build와 관찰 결과가 같다.`,
131: `## Formatter
AST+Trivia 기반 document model에서 단일 canonical output을 만든다. 원본 기본 줄 길이는 100이며 주석을 보존한다. roundtrip semantic equality와 idempotence를 요구한다.

## 초안 — D24
format check는 rewrite 없이 diff/exit 상태, format write는 atomic file replacement와 원 encoding 오류 보존을 제안한다. syntax error 파일은 기본 쓰지 않는다. string/comment contents는 그대로 유지한다.

## 검증
긴 call/generic/type/match, trailing comment, doc comment attachment, END-sensitive return/else/operator, CRLF→LF offset refresh, stdin/stdout mode.`,
132: `## Lint
allow/warn/deny 수준과 scope별 override를 지원한다. LintId와 diagnostic code를 분리한다. unused local/import, shadow, naming, unreachable candidate, needless mutable binding을 초안 D25로 제안한다.

## 자동 수정
MachineApplicable만 자동 적용 후보이며 다른 file/scope 의미를 바꾸면 MaybeIncorrect다. compiler correctness error를 allow lint로 끌 수 없다. suppression syntax는 D01/D25 승인 전 임의 keyword로 추가하지 않는다.

## 검증
override precedence, duplicate suppression, unused side effect initializer 유지, name collision fix 거부, formatter-after-fix semantic equality.`,
133: `## LSP 구현 계약 초안 — D24
document URI/version/text snapshot, incremental edits, diagnostics, completion, hover, definition/references, rename, formatting을 단계적으로 제공한다. compiler byte Span과 protocol position을 변환하며 UTF-16를 기본 상호운용 제안으로 둔다.

## 동시성
analysis 결과의 document version이 최신과 다르면 publish하지 않는다. cancellation은 incomplete result를 cache 성공으로 저장하지 않는다. batch compiler와 같은 semantic core를 사용한다.

## 검증
emoji/CRLF 위치, stale diagnostics race, rename visibility/collision, incomplete syntax no crash, multi-file edits invalidation. 실제 protocol version/capability는 도입 때 공식 명세를 확인한다.`,
134: `## API docs
public definitions/signatures/modes/effects/visibility/doc comments를 export table에서 추출한다. private/internal 노출은 explicit option만 허용한다. generated example은 grammar/language version을 표시한다.

## 초안 — D24
HTML/index/search JSON output, cross-reference resolution, static example compile checks를 제안한다. doc comment HTML는 escape/sanitize하고 arbitrary embedded script를 실행하지 않는다. link target source Span을 유지한다.

## 검증
generic/receiver modes 표시, broken reference, source doc attachment, Unicode signature, invalid example report, repeated generation byte stability.`,
135: `## Registry 범위
Package Registry 서버는 Canonical 비지원이다. 0.1은 local path와 pinned git source 중심의 package graph를 D21에서 제안한다. registry client/server/publication을 당연한 release requirement로 추가하지 않는다.

## 후속 설계
package identity, namespace ownership, signatures, checksum, immutable releases, yanking, dependency attack model, authentication, rate limits, mirrors/offline cache를 함께 검토한다.

## 검증
현재 manifest에서 unsupported registry source는 명시 오류다. package cache를 registry service와 혼동하지 않는다. 미래 배포 기능은 별도 version proposal과 approval 필요다.`,
136: `## 테스트 계층
Unit → Lexer/Parser/HIR/MIR Snapshot → Compile-pass/fail → Runtime integration → E2E → Fuzz → Benchmark → Compatibility. 비용이 높은 계층은 해당 Stage에서 추가한다.

## traceability
CONFORMANCE의 T번호는 NOVA/D번호/Stage/fixture/기대 outcome를 연결한다. fail은 code+primary byte Span+필요 secondary를 확인하고 단순 nonzero exit만으로 통과시키지 않는다.

## 회귀
crash/잘못된 정상 허용/오진은 최소 재현을 추가한다. baseline을 바꾸기 전 implementation/spec/test/environment 원인을 분류한다.

## 검증
병렬 fixture 격리, Target annotation, timeout, stdout/stderr 구별, 실제 실행 command log. draft fixture는 current compiler pass 결과라는 뜻이 아니다.`,
137: `## Token/AST snapshot schema
token kind/start/end/raw spelling/trivia와 synthetic marker를 출력한다. AST는 node kind/child order/Span을 출력하고 memory pointer/random ID를 제거한다. source reconstruction 검증은 별도 assertion으로 수행한다.

## update 절차
snapshot 변경은 grammar/decision 이유와 diff 검토를 요구한다. 자동 accept를 CI 기본으로 사용하지 않는다. comment/END/call argument order를 normalize하여 숨기지 않는다.

## 검증
Unicode multiline strings, nested interpolation/comment, missing delimiter, keyword adjacency, deterministic serial/parallel output. GRAMMAR production마다 정상/오류 사례를 연결한다.`,
138: `## Compile fixture 형식 초안 — D25
각 fixture는 .nova와 sidecar .json(expected phase,code,file,start,end,secondary,Target,Stage)를 제안한다. 원본 //~^ N4101 annotation도 읽되 byte range sidecar가 authoritative다.

## 판정
pass는 check exit0/no error; fail은 정확한 진단 및 exit1. toolchain failure exit3은 fixture 성공이 아니다. negative fixture가 parser 오류로 막혀 원하는 type/move 오류를 검증하지 못하면 실패다.

## 검증
unicode byte ranges, cross-file secondary, unexpected extra error, unsupported Stage feature와 0.1 non-goal 구별, empty file/no-main phase 구별.`,
139: `## MIR snapshot
lowered/pre-analysis/post-drop/post-opt dump를 구분한다. fixture sidecar는 pass name/schema/Target을 고정한다. typed locals, CFG, Copy/Move operands, Call/Drop terminators, SourceInfo를 기록한다.

## 검증
validator를 snapshot 전후 실행하고 malformed fixture는 internal code를 기대한다. block numbering normalize가 edge topology를 바꾸지 않아야 한다. constructor partial init와 try cleanup을 필수 corpus로 둔다.

## 완료
snapshot 검토만으로 runtime correctness를 주장하지 않고 Drop trace/runtime fixture와 함께 확인한다.`,
140: `## Runtime Harness
임시 출력 디렉터리에서 build 후 child process를 실행한다. expected stdout/stderr/exit class/timeout/Drop trace를 분리한다. child process는 crash/timeout 시 종료하고 남은 process/resource를 정리한다.

## 초안 — D25
UTF-8 stdout byte comparison, OS newline normalization의 제한된 허용, abort의 platform-specific exit class mapping을 제안한다. interactive/stdin/resource-heavy test는 명시 annotation을 둔다.

## 검증
Hello exact output, recursion/overflow/bounds abort, no cleanup-on-abort, output beyond pipe capacity, spaces/unicode executable path, concurrent fixtures isolation.`,
141: `## Fuzz targets
bytes→source validation/lexer, normalized tokens→parser, structured MIR→validator/analysis. valid source mutation과 invalid source generation을 함께 사용한다.

## failure
crash/hang/ICE/non-determinism는 실패다. resource-limit diagnostic은 명시 limit을 충족하면 정상 거부다. wrong-code는 differential/observable fixture로 분리한다.

## 운용
seed/corpus/minimizer/compiler/Target/options를 저장하고 dedup한 최소 재현을 regression에 올린다. 깊은 mode/delimiter, UTF-8 boundaries, CFG cycles, move/loan permutations를 생성한다.

## 검증
시간/메모리 예산, sanitizer runtime, repeated seed 동일 outcome. 아직 C/D가 없으면 해당 fuzz target을 미리 구현하지 않는다.`,
142: `## Property testing
source reconstruction, Span validity, parse/format semantic equivalence, formatter idempotence, serial/parallel compile equality, optimization observable equivalence를 속성으로 둔다.

## Differential oracle
같은 Nova program의 O0/O2/두 Target을 비교한다. C++/Rust 프로그램을 oracle로 쓸 때는 Nova overflow/order/Drop 의미와 일치하는 subset만 사용한다. target-specific ABI 값을 무조건 같다고 비교하지 않는다.

## 검증
numeric boundary generator, equivalent parentheses/semicolon/newline transformation, alias normalization, shared race model. 실패의 seed와 minimized source를 보존한다.`,
143: `## Benchmark 설계 초안 — D26
lex/parse/typecheck/MIR/codegen/link 시간, peak RSS, incremental hit/miss, executable runtime/size를 별도로 측정한다. toolchain/Target/hardware/power/flags/corpus size를 기록한다.

## 판정
warm/cold, repeated runs median 및 분산을 보고한다. 특정 비율 regression threshold는 baseline 측정 후 승인하며 사전에 근거 없는 수치를 성능 보장으로 쓰지 않는다. correctness 실패를 속도 이득으로 상쇄하지 않는다.

## 검증
대형 파일/많은 module/generic specialization/ownership CFG, optimized executable arithmetic/IO/containers. test machine 변화 시 baseline을 별도 유지한다.`,
144: `## 메모리 안전성 증거
safe source의 uninitialized read/use-after-move/alias violation/dangling view/partial move를 compiler-fail corpus로 검증한다. runtime Array/Shared/FFI unsafe 부분은 sanitizer/model/fault injection으로 검사한다.

## 한계
테스트 통과가 soundness proof는 아니다. Loan/Place/Drop 알고리즘의 불변 조건과 counterexample review를 함께 관리한다. unsafe 사용자 계약 위반과 compiler가 잘못 허용한 safe source를 구분한다.

## 검증
Miri 적용 가능한 Rust core, address/undefined/thread sanitizer 적용 runtime harness, double-free/unaligned access/OOM/weak race, report→minimal regression 연결.`,
145: `## Unsafe/FFI 검토표
operation마다 provenance, null, align, initialized bytes, bounds, alias/exclusivity, owner duration, release function, thread/callback/reentrancy, exception boundary를 기록한다.

## compiler/tooling 입력
manifest/archive/cache path traversal, untrusted metadata length/depth, shell argument injection, imported doc script, resource exhaustion도 검토한다. package 작업에 source-controlled shell hook을 자동 실행하지 않는다.

## 검증
negative wrapper corpus, malformed artifact/cache, foreign allocator mismatch, callback-after-free, resource budget, link path spaces. safe wrapper 완료에는 계약과 runtime failure tests가 모두 필요하다.`,
146: `## 0.1 Release gate
Accepted language/grammar decisions, Stage A~E 지원표, pass/fail/runtime regression, 첫 Windows/Linux Target, toolchain locks, ABI/schema versions, clean-machine build/install/uninstall, checksum과 release note가 필요하다.

## blocking
unresolved semantic D번호, known safe-code unsoundness, wrong-code, missing entry/runtime/link behavior, reproducibility failure는 release blocker다. 제외 feature는 미구현으로 기록하며 완료 부족으로 착각하지 않는다.

## 검증
release artifact clean VM 테스트, offline/locked package, debug/release semantics, documentation examples, minimum Rust toolchain frontend checks. 현재 3-crate 기반은 release 완료가 아니다.`,
147: `## Compatibility suite
language version별 positive/negative corpus와 expected stdout/exit/Drop를 보존한다. parser acceptance만으로 호환성을 판단하지 않는다. diagnostic code/schema, std signature, package schema, ABI는 별도 matrix다.

## 초안 — D29
patch release는 이전 accepted programs와 normative rejects를 유지하는 것을 기본으로 한다. bug fix exception은 명확한 사양 근거, impact, migration을 기록한다.

## 검증
old compiler/new compiler cross-run, old lock/new resolver, old metadata/new compiler deliberate rebuild, Unicode/Target corpus. nondeterministic process IDs/addresses를 expected output에 넣지 않는다.`,
148: `## Self-hosting 제외
0.1 compiler 구현은 Rust이며 Self-hosting은 비목표다. 개발 문서 전체를 작성했다는 이유로 Nova compiler rewrite를 시작하지 않는다.

## 후속 연구
Rust Stage0 안정화 → Nova 도구 일부 → frontend → whole compiler 순서를 검토한다. Stage1/Stage2 생성 compiler 의미, runtime ABI, bootstrap trust, reproducibility를 비교해야 한다.

## 검증 조건
전체 conformance, same-language compiler self-build, Stage1/2 observable equivalence, fallback Stage0 지원. 일정과 언어 버전은 별도 proposal에서 결정한다.`,
};
