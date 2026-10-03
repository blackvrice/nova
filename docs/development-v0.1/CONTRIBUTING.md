# Nova 개발·검토·운영 지침

이 문서는 작업 절차 초안이다. 기존 사용자 지침과 Canonical이 우선한다.

## 시작 순서

저장소 상태/사용자 변경 확인 → Stage와 관련 NOVA/결정 확인 → existing tests 확인 → 최소
변경 → 구현 → unit/pass/fail/regression → Cargo checks → 문서/변경 결과 보고. 사양 누락을
code 편의로 채우지 않는다. 실제 의미 변경은 NOVA-006의 proposal/승인을 따른다.

## 코드 경계와 검토

모듈의 input/output/invariants/ownership/source origin을 설명한다. Core에서 LLVM type를
노출하지 않는다. user-input reachable unwrap/panic, unsound alias attribute, clone/heap abuse,
global state, non-deterministic map order를 검토한다. Stage와 관계없는 refactor는 분리한다.

Unit는 단일 함수 불변 조건, integration은 단계 경계, compile-fail은 source code+Span,
runtime는 Native observable semantics를 검사한다. test 기대를 낮춰 실패를 숨기지 않는다.
실패는 implementation/spec/test/environment로 분류한다.

## Toolchain과 commands

현재 Workspace Rust minimum=1.80은 기존 Cargo 설정이며 release toolchain pin의 승인과
실제 minimum build 확인은 D28 필요다. LLVM/binding exact version은 backend 시작 시 공식
support를 확인하고 lock한다. network dependencies/MSVC/system packages는 설치 기록을 남긴다.

```text
cargo fmt --check
cargo clippy --workspace --all-targets -- -D warnings
cargo test --workspace
cargo check --workspace --all-features
node tools/docs/build-pack.mjs
node tools/docs/validate-pack.mjs
```

버전/commands는 계획이며 로컬 환경에 없는 도구가 있다고 주장하지 않는다. CI의 실제
실행 log, OS/Target/options, skipped reason를 함께 보고한다.

## 문서 변경

topics*.mjs는 주제별 작성 원천이다. 보완 specs를 수정했다면 원천도 갱신하고 build로
재생성한다. 부속 .md/grammar는 직접 편집한다. MANIFEST hash와 연결 validation을 갱신한다.
Accepted로 바꾸려면 승인 기록과 grammar/fixtures/원본 영향 분석이 필요하다.

## Release/incident

wrong-code/unsafe acceptance는 우선 수정하고 affected versions/재현/workaround를 기록한다.
재현 파일을 regression으로 추가하고 fix를 conformance와 함께 검토한다. 배포는 checksum,
toolchain/runtime/schema/Target, known limits, migration note와 clean-machine install 확인을
포함한다. 사용자 source/secret environment를 자동 외부 전송하지 않는다.

## 보고 형식

Implemented / Changed / Tests Added / Validation / Specification / Remaining을 사용한다.
작성·승인·구현·실행 검증은 각각 사실대로 보고한다. Draft 문서 자체의 작성 완료를 언어
0.1 전체 지원 완료로 표현하지 않는다.
