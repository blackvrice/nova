# Stage B Module·다중 파일 최소 계약 — P11

작성일: 2026-10-05. 상태: **Draft / 사용자 승인 대기**.
검토용 제안이며 Compiler에 적용하지 않았다. D01~D05/P01~P10과 원본 Canonical을 보존한다.
이번 범위는 하나의 source root에서 함수·전역 const를 가져오는 다중 파일 컴파일이다.
전체 D06/Stage B·Package·재export·aggregate 승인이 아니다.

## Specification Change Proposal

- 관련 문서: [원본 NOVA-020 Module·Import](../02_Names_Modules/NOVA-020_Module_Import_사양서.md),
  [NOVA-019 Scope](../02_Names_Modules/NOVA-019_이름_해석_Scope_사양서.md),
  [NOVA-021 가시성](../02_Names_Modules/NOVA-021_접근_제한_가시성_사양서.md),
  [NOVA-022 ID](../02_Names_Modules/NOVA-022_Symbol_Definition_ID_사양서.md),
  [NOVA-023 namespace](../02_Names_Modules/NOVA-023_Package_간_이름_해석_사양서.md),
  [D06](DECISIONS.md), [P02](SEMANTICS_STAGE_A_PROPOSAL.md), [P06](GLOBAL_CONST_STAGE_B_PROPOSAL.md),
  [CLI](CLI_CONTRACTS.md), [전체 Draft EBNF](GRAMMAR.ebnf).
- 현재 사양: 원본 NOVA-020은 파일 경로로 module 경로 결정, 순환 참조 허용, 순환 초기화 금지를 확정했다.
  NOVA-023은 타입·값·모듈 namespace 분리, nearest scope, 모호성의 임의 선택 금지를 요구한다.
  Lexer에는 use/as/public/internal/private/::가 있고 전체 Draft 문법은 use path as alias를 제안한다.
  현재 Compiler는 단일 파일만 검사하며 use·visibility·qualified name을 N1102로 거부한다.
- 발견된 문제: source root와 discovery, import 대상/alias scope, 기본 visibility, cross-file const/entry,
  서로 다른 파일의 ID/Span 및 private Native 이름을 연결하는 최소 계약이 없다.
- 제안 변경: 아래 root-relative item import·가시성·graph·const·진단·CLI/Native 계약과
  [전용 EBNF](GRAMMAR_STAGE_B_MODULE.ebnf)를 적용한다.
- 변경 이유: 기존 scalar 함수·const 의미로 실제 코드 분리를 검증하고 aggregate/ownership 구현 전에
  다중 파일 이름·source identity 기반을 마련한다.
- 영향 범위: Source/Driver, Parser/AST/HIR, Resolve/TypeCheck/const, MIR/validator, Codegen entry/LLVM,
  CLI와 diagnostics/harness. Lexer keyword/operator와 END 의미·scalar Runtime ABI는 유지한다.
- Backward Compatibility: 기존 단일 파일의 함수 print shadow, 전역 const print 금지,
  P07~P10 숫자/char 의미와 const budget·evaluation order를 유지한다. 기존 명령과 Hello snapshots를 유지한다.
  현재 지원하지 않는 use/visibility에만 새 의미를 추가한다.
- 대안: aggregate를 먼저 구현하거나, manifest 전체를 먼저 동결하거나, 모든 파일을 자동 스캔하거나,
  module alias/qualified value expression/reexport까지 한 번에 지원하는 방식.
  이번 안은 reachable 파일의 직접 선언만 item import하며 package/aggregate에 의존하지 않는다.

## Source root·파일 발견

1. 기존 `nova check|build|run <entry.nova>`를 유지하고 세 명령에 `--source-root <directory>`를 선택 옵션으로 제안한다.
   생략하면 entry 파일의 부모 디렉터리가 root다. 옵션 중복/누락은 기존 usage exit 2다.
   nova.toml/lock을 찾거나 생성하지 않는다. 현재 bundle은 하나의 임시 package로 취급한다.
2. root/entry는 canonical filesystem 경로로 검증한다. entry는 root 안의 .nova 파일이어야 한다.
   root-relative `a/b.nova`의 module path는 `a::b`, `main.nova`는 `main`이다.
   경로 segment는 D02 IDENT 철자이며 namespace lookup은 case-sensitive·NFC 변환 없음이다.
   import는 OS 문자열 경로/`.`/`..`가 아니라 `IDENT :: IDENT ...`로 작성한다.
3. `use helpers::math::add`는 root/helpers/math.nova의 직접 선언 add를 가리킨다.
   root/helpers/math/mod.nova, 디렉터리 index, parent-relative 탐색, 검색 경로 fallback은 없다.
   마지막 segment는 item이고 앞의 모든 segment가 module path다.
   entry 자체 파일은 기존처럼 .nova면 검사할 수 있지만, 식별자 아닌 module path는 import 대상으로 쓸 수 없다.
4. entry와 import로 도달한 파일만 읽는다. 사용하지 않는 .nova 파일의 오류는 이 bundle 검사에 포함하지 않는다.
   각 경로 segment는 실제 디렉터리 entry 철자와 정확히 일치해야 한다. 대소문자 추정 fallback은 하지 않는다.
   도달한 논리 경로의 ASCII case-only 충돌 또는 한 physical file의 서로 다른 논리 module 경로는 N2002다.
   참조된 파일의 canonical path가 root 밖으로 나가면 N8001이다. junction/symlink를 통해서도 이 경계를 유지한다.
5. 모듈 graph는 iteratively 발견하며 동일 module은 한 번만 등록한다. self/circular import를 무한 탐색하지 않는다.
   entry 포함 최대 1,024개 module을 제안한다. 초과 시 추가 module을 요구한 use path에서 N8901이다.
   이 제한은 per-const 10,000-node 예산과 별개이며 전체 D30 정책을 동결하지 않는다.
6. discovery 완료 후 entry FileId는 0, 나머지는 normalized root-relative path의 UTF-8 byte 순서로 할당한다.
   module/definition 수집 결과와 함수·const reference는 import 선언 순서/OS 열거 순서에 의존하지 않는다.
   FileId/ModuleId/DefId는 bundle 내부 identity이며 영구 cache ID를 제안하는 것은 아니다.

## 구문·import scope·visibility

1. top-level `use module_path::item [as alias]`를 END로 끝낸다. alias가 없으면 마지막 item 이름으로 도입한다.
   module_path는 한 개 이상의 segment다. use/as/:: 전후 newline은 기존 D05 규칙을 그대로 따른다.
   선언보다 앞/뒤 어디에 있어도 module scope에서 유효하다. block-local use는 N1102다.
2. import target은 그 파일에 직접 선언된 함수 또는 전역 const만 허용한다.
   imported alias를 다시 import하는 reexport와 `public use`, wildcard/group imports,
   module-only `use math [as m]`는 N1102로 제외한다. qualified name의 1 segment는 이 의미 검사에서 제외한다.
   `math::add(...)` 같은 qualified value expression, module value, 타입 import도 이번 범위에 없다.
   해당 구문은 기존 N1102 경계를 유지하며 silent fallback을 하지 않는다.
3. 함수/전역 const에 public/internal/private 하나를 선택적으로 붙인다. 기본은 internal이다.
   private는 선언 module만, internal은 현재 bundle, public도 현재 bundle에서 접근 가능하다.
   public의 향후 외부 package 허용 방향은 기존 D06 초안을 유지하되 외부 package loader는 구현하지 않는다.
   함수 내부 binding/parameter에 visibility를 붙이지 않는다. 중복 modifier·잘못된 위치는 syntax 오류다.
4. 이름 수집은 파일별이다. 서로 다른 module에 같은 함수/const 이름이 있어도 중복이 아니다.
   own top-level 선언과 import binding, import binding 사이 같은 value 이름은 N2002다.
   같은 target을 두 번 같은 이름으로 import해도 중복이며 import 순서로 우승자를 고르지 않는다.
   다른 alias 이름으로 같은 DefId를 참조하는 것은 허용한다.
5. local/parameter shadow 및 initializer-before-binding 규칙은 P02/P04를 따른다.
   import는 원 DefId를 가리키며 새 함수/const 사본을 만들지 않는다. builtin print는 각 module의 fallback이다.
   own 함수 print와 import된 함수 print는 builtin을 shadow할 수 있다.
   own 전역 const print와 const를 print라는 alias로 import하는 것은 P06 경계에 맞춰 N2002다.
6. private target을 타 module에서 가져오면 N2004이며 use path가 primary, 원 선언이 secondary다.
   private 자기 module 선언의 다른 alias는 접근 가능하다. 타입 namespace의 기존 primitive/alias 의미는 유지한다.
   module graph namespace와 value scope를 합치거나 import 경로를 local 이름 검색으로 해석하지 않는다.

## 선언 graph·const·실행

1. 모든 reachable 파일을 Lexer/Parser/HIR로 만든 뒤 모든 직접 top-level 선언을 수집하고 import를 연결한다.
   함수 signature를 수집한 후 body를 검사한다. 함수의 상호 recursion과 module graph cycle은 허용한다.
   import target 파일은 실제 선언을 수집해야 하며 import alias끼리의 순환 연결을 재export로 해석하지 않는다.
2. 전역 const dependency graph는 모든 파일의 원 DefId를 사용한다. alias는 identity를 바꾸지 않는다.
   skipped logical RHS의 참조도 P06대로 graph에 포함한다. cross-file static const cycle은 N3202다.
   initializer type inference/evaluation은 dependency 순서로 진행하고 정상 const 값은 한 번 계산해 재사용한다.
   top-level let/var·runtime initializer·const function은 여전히 지원하지 않는다.
3. P05/P06 const permission·checked 평가·initializer마다 10,000-node 예산,
   P10 target type syntax 제외 규칙을 그대로 유지한다. imported const 참조는 name 1 node이며 cached subtree를 다시 세지 않는다.
   const 실패 primary는 원 initializer/cast, secondary는 원 const 선언이고 파일이 다르면 각 FileId를 그대로 표시한다.
4. 모든 loaded module의 body/const를 검사한다. 사용하지 않는 함수의 오류도 검사한다.
   의도적으로 source text를 이어 붙여 같은 scope로 만들거나 이름 문자열을 치환해 기존 checker를 우회하지 않는다.
5. Native entry는 entry 파일에 직접 선언된 `main() -> Unit`만 선택한다.
   imported alias main 또는 다른 파일의 main은 entry를 대체하지 않는다. fragment check에는 main이 필수가 아니다.
   기존 entry 진단 N2001(누락)/N2002(중복)/N2201(인수)/N2101(반환 타입)과 exit 분류를 유지한다. imported const main은 entry 함수가 아니다.
6. MIR/LLVM은 bundle 전체를 하나의 검증 CodegenUnit/object로 생성한다. 각 body/callee는 원 DefId를 따른다.
   같은 이름의 서로 다른 module 함수는 private Native symbol로 충돌하지 않는다.
   top-level 순서나 import 순서에 따라 runtime initializer가 실행되는 의미를 새로 도입하지 않는다.
   print·인수 source-order·String lifetime·P03/P07/P09/P10 실패 정책은 그대로다.

## Source identity·분석 경계

- AST arena/root는 파일별이며 모든 source Span은 원 FileId/UTF-8 byte range를 보존한다.
  다른 파일의 노드를 하나의 원본 AST parent span에 억지로 포함하지 않는다.
- HIR bundle은 파일별 root와 module ownership을 기록한다. 모든 HIR/Def/Scope identity는 bundle에서 구분 가능해야 한다.
  SourceOrigin의 AST identity는 FileId와 AstNodeId를 함께 구분한다. 가상 합쳐진 소스에 대해 fake Span을 만들지 않는다.
- Core의 ModuleGraph/ImportEdge·visibility·resolution 모델은 파일 I/O/LLVM에 의존하지 않는다.
  Driver가 SourceDatabase와 discovery를 제공한다. resolver/typecheck/const/MIR는 ownership과 원 DefId로 동작한다.
- MIR recomputation gate는 module/visibility/import targets/entry/const metadata가 실제 HIR bundle과 일치함을 검증한다.
  malformed public API 입력은 panic/codegen 성공으로 통과하지 않는다.
- no-import 기존 단일 파일의 ID/dump/snapshot과 출력은 보존한다. 다중 파일 dump는 root-relative path/원 source range로 결정적이다.

## 진단·CLI 계약

새 diagnostic code는 추가하지 않고 기존 등록 code의 아래 trigger를 동결한다.

| 상황 | Code / primary | secondary 또는 event |
|---|---|---|
| 모듈 경로/직접 target 선언 없음 | N2001 / use path 또는 target segment | 기대 module path / 원 import |
| 선언·import 이름 중복, 도달한 module 경로 충돌 | N2002 / 뒤 binding 또는 use path | 앞 선언/import/path |
| private target 접근 | N2004 / use path | 원 private 선언 |
| cross-file const 순환/const 예산 | N3202 / 기존 P06 const 참조/initializer | 파일을 포함한 원 dependency chain |
| permission/checked const 실패 | N3201 / 원 실패 식 | 원 const 선언 |
| 읽기·UTF-8·root 경계 실패 | N8001 / import path | 실패 path·I/O cause 또는 invalid byte offset |
| root entry를 읽을 수 없음 | N8001 / source-input event | 실제 path/offset; 존재하지 않는 파일의 가짜 Span 금지 |
| module 1,024개 초과 | N8901 / 새 파일을 요구한 use path | module limit |
| 제외한 use/qualified value/reexport 문법 | N1102 / 제외 construct | 지원 범위 설명 |

- source/name/type/const/graph 실패는 exit 1이며 LLVM/Rustc 실행·output 생성 전에 거부한다.
  check는 LLVM 없이 전체 reachable bundle을 검사한다. usage 2/toolchain·unsupported float host 3/ICE 101을 유지한다.
- root 입력 실패 외의 진단은 유효한 loaded source Span을 사용한다.
  unresolved import의 후속 reference에서는 Error resolution을 사용해 파생 중복 진단을 억제한다.
- build/run은 모두 frontend gate를 통과한 뒤 기존 output 정책을 따른다.
  파일을 새로 만들거나 invalid UTF-8을 replacement 문자로 바꾸지 않는다.

## 수용 fixture와 완료 기준

[Draft fixture](module-proposal-fixtures/README.md)는 두 module 사이의 import cycle와 alias를 담고 있다.
구현 뒤 main.nova의 출력은 `value=42` + LF이며 exit 0이어야 한다. 현재 Compiler 통과 증거가 아니다.

- Parser: use/alias/visibility, ::·as newline/END, missing target/alias/EOF, 모든 source truncation 및 후속 선언 recovery.
- Discovery: nested root-relative path, 명시/기본 root, unused broken file 제외, 중복 module dedup,
  root 외 canonical path/읽기/invalid UTF-8/철자 불일치, 1,024/1,025 파일과 깊은 graph의 iterative 처리.
- Resolve: own/import duplicate, 동일 target alias, private/internal/public 접근, nearest local shadow,
  print 정책, 같은 이름의 다른 module 정의, import order permutation·circular function reference.
- Type/const: imported parameter/return/numeric cast·String·Unit, forward/cached const, cross-file skipped cycle,
  10,000-node 경계, private/undefined cascade suppression와 여러 파일 primary/secondary.
- MIR/Codegen: HIR ownership·module/import/visibility/entry/const 변조 거부, source identity 보존,
  module마다 동명 함수와 entry isolation, LLVM COFF/ELF O0/O2 및 Windows Native 전체 회귀.
- Native: Draft fixture exact stdout/exit, cross-file call/recursion/effect order·String lifetime,
  다른 파일의 checked arithmetic/cast Abort의 정확한 FileId/Span, 모든 실패에서 후속 효과 중단.
- Cargo fmt/clippy/workspace test/all-features, Runtime rustfmt, 문서/EBNF/ledger 검증.
  문서 validator PASS와 언어 사양 승인·Compiler/Native 수용 통과를 구분한다.

## 작성 단계 검증 — 2026-10-05

문서 validator는 148개 원본/보완 hash·로컬 링크·34-production EBNF·기존 P10 production 보존,
P11 Draft ledger/accepted 제외와 두 파일 fixture의 JSON/UTF-8/LF 데이터를 검사해 PASS했다.
기존 Compiler 기반은 cargo fmt/clippy/workspace **217개** tests/all-features와 Runtime rustfmt가 통과했다.
P11 source fixture를 Compiler/Native에서 수용한 결과는 아니며 해당 수용 검증은 승인 뒤 구현할 때 수행한다.

## 승인 경계

승인 대상은 root-relative 직접 함수/const item import·alias, 기본 internal과 private/public 가시성,
reachable ModuleGraph/circular reference·cross-file P06 const cycle, Source identity·entry/MIR 연결,
source-root CLI·1,024 module 예산과 위 진단/검증, 전용 EBNF다.
module alias/qualified value/type import·wildcard/reexport·package manifest/dependency·aggregate,
ownership·public FFI·incremental cache·Linux Native와 전체 D06/D30은 후속이다.
사용자 승인 전 Compiler와 accepted ledger/기존 accepted EBNF에는 적용하지 않는다.
