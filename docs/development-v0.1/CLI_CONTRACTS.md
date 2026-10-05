# CLI 동작 계약 초안

기존 command/option/exit 영역은 NOVA-125. 새 stream/child/output/clean 상세는 D22다.
Stage A 단일 파일 check/build/run 최소 부분은 사용자 승인 [P03](NATIVE_STAGE_A_PROPOSAL.md)과
[현재 구현](NATIVE_IMPLEMENTATION.md)을 따른다. 아래 전체 options/package/JSON 등은 Draft다.

float는 사용자 승인 [P09](FLOAT_STAGE_B_PROPOSAL.md)의 범위와 [구현·검증](FLOAT_IMPLEMENTATION.md)을 따른다.
float source 오류는 exit 1로 도구 실행 전에 거부하며 기존 output을 보존한다.
미지원 float 평가 host는 exit 3이며 현재 Core 환경 제어/Native host는 x86_64/Windows x64다.

| 명령 | 입력 | 산출물/행동 | Stage |
|---|---|---|---|
| check | file 또는 manifest target | frontend/semantic 진단; object/link 없음 | A→E |
| build | file 또는 manifest target | 검증 MIR→object→executable/library | A→E |
| run | binary target + -- 뒤 argv | build 성공 뒤 child 실행 | A→E |
| test | fixtures/test targets | 격리 실행 + 결과 | B→E |
| fmt | source files | --check 비교 또는 write | E |
| clean | manifest output | 의도된 build/cache 경계 삭제 | E |
| doc | public export/docs | static API documentation | E |
| version | 없음 | compiler/language/runtime/LLVM support info | A |

manifest 없는 .nova는 임시 package이며 원본 옆에 nova.toml를 자동 만들지 않는다.
output은 명시 -o 또는 workspace target/<triple>/<profile> 후보다. 이름/Target/emit 옵션을
cache key와 artifact metadata에 반영한다. 중간 생성 실패 시 partial output를 성공 산출물로
등록하지 않는다. supported emit는 해당 단계가 구현된 경우만 허용한다.

## P11 승인된 다중 파일 옵션

check/build/run에 `--source-root <directory>`를 선택적으로 사용한다. 기본은 entry의 부모다.
entry FileId 0과 도달한 import 파일만 전체 frontend/MIR에서 검사한다.
가시성·const 순환·읽기·UTF-8·root/1,024-module 오류는 exit 1이며 도구/출력 생성 전에 거부한다.
옵션 중복/값 누락은 exit 2다. [P11](MODULE_STAGE_B_PROPOSAL.md)과 [검증 기록](MODULE_IMPLEMENTATION.md).
Package manifest/dependency와 qualified value/reexport는 제외한다.

## stream과 exit

human diagnostics stderr, requested dump stdout; run child stdout/stderr는 그대로 연결.
JSON compiler diagnostic/events는 stdout JSON Lines, child I/O와 혼합 방지는 run에 별도
compiler-message 파일 또는 stderr JSON event 사용을 제안한다. 어느 mode인지 CLI usage에서
명시한다. 결과를 silently parse 가능한 JSON이라고 부르지 않는다.

compiler exit: success0,user1,CLI/manifest2,toolchain/link3,ICE101. run에서는 build 실패면
compiler code, build 성공이면 child exit/signal 정보를 전달한다. child의 1/2/3은 compiler
오류와 numeric 충돌 가능하므로 structured result의 kind=child를 함께 제공하는 제안이다.
cross target executable은 자동 실행하지 않고 target mismatch를 설명한다.

## Clean과 입력 보안

clean은 resolved output 경계 검사 후 삭제한다. source/docs/nova.lock 또는 manifest가
지정한 임의 parent directory를 recursive delete하지 않는다. linker는 argument array로
실행한다. 사용자 이름/경로를 shell script 문자열로 만들지 않는다.

## 검증

unknown option/unsupported emit/Target, 한글·공백 경로, emit JSON 분리, invalid source
codegen 금지, missing linker, run argv 정확 전달, child timeout/signal, clean 경계 테스트.
