# 진단 모델과 출력 schema 초안

기존 Nxxxx 영역과 byte Span/scalar column 유지. 구체 코드/JSON 필드는 D25 제안이다.

## Canonical model

Diagnostic={code,severity,message,primary Label,secondary Labels,notes,suggestions}.
Label={file_id,start,end,message}. Suggestion={span,replacement,message,applicability}.
MachineApplicable/MaybeIncorrect/HasPlaceholders/Unspecified를 구분한다. 다중 편집 fix는
향후 별도 batch contract가 필요하며 현재 단일 replacement를 겹치는 자동 fix로 적용하지 않는다.

## JSON envelope v1 제안

```json
{
  "schema_version": 1,
  "kind": "diagnostic",
  "code": "N2001",
  "severity": "error",
  "message": "이름을 찾을 수 없습니다",
  "primary": {"file":"main.nova","start":18,"end":25,"line":2,"column":5,"message":"unknown"},
  "secondary": [],
  "notes": [],
  "suggestions": []
}
```

CLI JSON은 event별 JSON Lines를 제안한다. human output은 source excerpt와 label을 보여주고
Snapshot은 ANSI 없는 deterministic text다. line/column은 1-based Unicode scalar, byte
range는 0-based 반열림이다. EOF는 start=end=length. path separators는 snapshot에서 /
normalize하지만 user input text/byte Span은 바꾸지 않는다.

## 현재 코드와 차이

현재 nova-diagnostics render JSON에는 schema_version/kind envelope가 없다. 이 제안은
문서 작업에서 코드를 변경하지 않았으며 D25 승인 후 Driver/schema version 추가를 검토한다.
invalid UTF-8/file load 오류는 유효한 source Span가 없을 수 있으므로 file-input event
{path,byte_offset?,cause}를 별도 제안한다. 가짜 UTF-8 SourceFile을 만들어 Span를 부여하지 않는다.

## 운영 규칙

DIAGNOSTICS.csv는 제안 registry이며 기존 실제 코드로 확인되지 않은 번호를 stable이라고
주장하지 않는다. 코드의 trigger/primary/secondary를 version control로 관리하고 폐기 code
재사용을 금지한다. syntax error에서 유래한 ErrorType/type/move 연쇄를 suppression한다.
정렬은 file canonical path,start,severity,code,secondary stable order를 제안하며 parallel
분석 완료 순서가 출력 순서가 되어서는 안 된다.

## ICE

N9xxx, compiler/Target/options/query stack/repro path를 포함하고 exit101. Source dump/환경
값은 사용자에게 로컬 경로로 제공하며 자동 외부 업로드하지 않는다. Toolchain/Linker 실패는
ICE와 구분한다. ANSI/control chars/JSON escaping/멀티라인/다중파일/invalid internal span을 검사한다.
