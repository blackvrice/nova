# 검토용 Nova fixture

20개 예제/sidecar는 사양 제안의 검토용이다. 현재 compiler로 실행 검증하지 않았다. 예상 code/Span 역시 D25 승인 후 diagnostic primary 정책에 맞춰 동결한다. 현재 Rust 기반 unit tests와 분리된다.

| 예제 | 수용 묶음 | 기대 파일 |
|---|---|---|
| [hello](hello.nova) | T013 | [sidecar](hello.json) |
| [function-call](function-call.nova) | T014 | [sidecar](function-call.json) |
| [semicolon-newline](semicolon-newline.nova) | T006 | [sidecar](semicolon-newline.json) |
| [operator-continuation](operator-continuation.nova) | T006 | [sidecar](operator-continuation.json) |
| [nested-comment](nested-comment.nova) | T005 | [sidecar](nested-comment.json) |
| [non-bool-if](non-bool-if.nova) | T012 | [sidecar](non-bool-if.json) |
| [undefined-name](undefined-name.nova) | T011 | [sidecar](undefined-name.json) |
| [type-mismatch](type-mismatch.nova) | T008 | [sidecar](type-mismatch.json) |
| [chained-compare](chained-compare.nova) | T007 | [sidecar](chained-compare.json) |
| [bad-literal](bad-literal.nova) | T003 | [sidecar](bad-literal.json) |
| [missing-return](missing-return.nova) | T012 | [sidecar](missing-return.json) |
| [literal-range](literal-range.nova) | T008 | [sidecar](literal-range.json) |
| [narrow-float](narrow-float.nova) | T009 | [sidecar](narrow-float.json) |
| [const-divzero](const-divzero.nova) | T018 | [sidecar](const-divzero.json) |
| [explicit-generic-call](explicit-generic-call.nova) | T023 | [sidecar](explicit-generic-call.json) |
| [use-after-move](use-after-move.nova) | T020 | [sidecar](use-after-move.json) |
| [range-max](range-max.nova) | T012 | [sidecar](range-max.json) |
| [short-circuit](short-circuit.nova) | T012 | [sidecar](short-circuit.json) |
| [unicode-column](unicode-column.nova) | T035 | [sidecar](unicode-column.json) |
| [overflow-abort](overflow-abort.nova) | T010 | [sidecar](overflow-abort.json) |
