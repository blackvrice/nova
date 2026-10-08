# P24 중첩 Copy 패턴·Tuple match 구현·검증 기록

2026-10-08 사용자 “P24 승인하고 중첩 Copy 패턴·Tuple match 구현 진행” 답변으로 [최소 계약](NESTED_PATTERN_STAGE_B_PROPOSAL.md)을 승인했다.
상태: **Accepted / 구현·검증 완료**. [60-production EBNF](GRAMMAR_STAGE_B_NESTED_PATTERN.ebnf)는 P22 pattern 두 production만 확장·두 production 추가·기존 56개 보존이다.

## 구현과 API 경계

- Parser/AST/HIR는 recursive variant·Bool·Unit·Copy Tuple·bare immutable binder를 표현한다.
  pattern nesting을 일반 expression nesting과 별도로 128에 제한한다. malformed compound/arm/match는 Error node로 복구하여 prefix HIR를 유지한다.
  Source gate는 UTF-8 XID spelling·qualified path·punctuation·child 순서·Unit·arm arrow/END를 검사한다.
- Resolver는 arm 전체 component를 source order로 순회해 같은 arm scope에 binder를 선언한다.
  bare IDENT는 const/function/type 조회 없이 새 immutable Copy binding이며 중복 N2002·변경 N3004·scope escape N2001을 유지한다.
- Checked.recursive_patterns는 HirId-indexed flat arena에 canonical Type·constructor·child IDs를 저장한다.
  nominal/import alias identity·component 기대 타입·arity·annotation secondary를 검사한다. 오류가 있는 match의 파생 N3101/N3102를 억제하고 독립 body 오류를 유지한다.
  Bool/Copy sum/Tuple/Unit root를 지원하며 기존 P14/P15 flat-only 경로와 진단은 보존한다.
- Coverage는 source-order 앞 arms 합집합의 usefulness를 constructor/default matrix로 계산한다.
  canonical cache key는 typed columns·ordered rows·candidate이며 hash collision은 실제 key equality로 확인한다.
  Bool false/true·Enum 선언 순서·Option None/Some·Result Success/Error 순서로 최대 8개 uncovered-region witness와 생략 표시를 제공한다.
  opaque leaf는 wildcard다. 구체 값 cartesian product를 미리 생성하지 않는다.
- Source pattern 10,000 nodes·cache-miss task 100,000·live matrix cells 1,000,000 한도를 분리한다.
  arm usefulness와 exhaustiveness/witness 분석은 같은 예산을 공유한다. borrowed specialization을 hash/compare한 뒤 cache miss에만 row를 할당한다.
  초과 시 match keyword N8901 또는 첫 10,001번째 source node N8901을 보고하며 파생 coverage 오류를 억제한다.
  pattern·task·witness 처리와 Drop은 flat arena/반복형이다.
- MIR은 scrutinee를 한 번 snapshot한 뒤 source-order arm tests·tag/Bool switch·tuple projection·tag-dominated EnumPayload를 낮춘다.
  선택된 body에만 component Copy binder를 쓰며 try/return/break/continue와 원본의 이후 변경을 보존한다.
  public Checked table은 원 HIR/Resolved 분석 재계산으로 검사한다. private projection/tag/source/full-body certificates는 동일 타입 path 교환·snapshot overwrite·CFG/effect 변경·body 삭제를 거부한다.
  nested coverage로 증명한 final failure sink만 private frozen body와 함께 인정하며 다른 reachable Unreachable은 거부한다.
- LLVM/Runtime production source·Cargo·aggregate ABI/layout은 변경하지 않았다. 기존 switch/project/Copy lowering과 private tagged ABI를 사용한다.

## 테스트와 실제 결과

| 검사 | 결과 |
|---|---|
| cargo fmt --check | PASS |
| cargo check --workspace --all-features --offline | PASS |
| cargo clippy --workspace --all-targets --all-features --offline -- -D warnings | PASS |
| cargo test --workspace --offline | 392 PASS / 0 FAIL / 76 ignored |
| Runtime stage_a.rs rustfmt | PASS |
| LLVM 전체 opt-in | 20 PASS / 0 FAIL, COFF/ELF O0/O2 |
| Native 전체 opt-in | 56 PASS / 0 FAIL, debug/release |
| 문서 validator / 독립 finite oracle 6개 | PASS |

총 468 PASS / 0 FAIL다. 새 테스트 23개는 기본 20·LLVM 1·Native 2다.
[수용 fixture](nested-pattern-proposal-fixtures/README.md)의 원 source 24개 bytes를 보존하며 main/추가 정상 3·부정 18·Runtime 1을 검증했다.
main의 12줄·정상 3개·nested checked Abort의 정확한 byte Span·before-only output을 debug/release에서 확인했다.
독립 [4줄 예제](../../examples/nested_patterns.nova)도 양 profile에서 동일하다.

Typecheck는 independent four-value enumeration으로 729개 Bool-pair pattern triples의 누락/도달성 결과를 대조한다.
node 10,000/10,001·task 100,000/100,001·cell 1,000,000/초과·cache hit 0·expanded row preallocation guard를 직접 검사한다.
Source의 1,024-wide Bool Tuple cell-limit N8901·128/129 nesting·UTF-8 split/child-order/Unit forgery·prefix recovery도 검사한다.
Checked plan 삭제/child/type/root-set 변조, MIR same-type tuple index/payload receiver 교환·inactive payload·tag target 반전·source/sink/body 삭제·snapshot 중복을 거부한다.
Native는 네 Bool pair의 partial overlap·inactive Empty/Int/Flag nested sums·Unicode·snapshot·try·loop jump를 실제 실행한다.

OneDrive incremental cache 경고를 피하려고 검증 프로세스에만 CARGO_INCREMENTAL=0을 설정했다.
문서 기계 검증은 Compiler/Native 실행과 구별한다. Rust 1.99.0·LLVM 21.1.8·Windows x64 MSVC에서 검증했으며 MSRV 1.80·Linux Native host 실행은 이번에 검증하지 않았다.

## 직접 사용과 후속 범위

[TESTING.md](../../TESTING.md)에 check·debug/release·집중/전체 회귀 명령을 제공한다.
guard/or/range·일반 literal·struct destructuring·irrefutable let destructuring·match expression·Read loan/take pattern·Array·일반 Move/borrow/Drop·전체 D08/D10/D12/D16/D25/D30은 후속 계약이다.
기존 P01~P23·D01~D30 결정 본문·Canonical·원본 148개는 보존한다.
