import fs from 'node:fs';
import path from 'node:path';
import crypto from 'node:crypto';
import { topics } from './topics.mjs';
import { topicsMiddle } from './topics-middle.mjs';
import { topicsTail } from './topics-tail.mjs';
import { decisionMarkdown } from './decisions.mjs';
import { conformanceMarkdown, codes } from './registries.mjs';
import { writeFixtures } from './fixtures.mjs';

const root = path.resolve(import.meta.dirname, '../..');
const out = path.join(root, 'docs/development-v0.1');
const bodies = { ...topics, ...topicsMiddle, ...topicsTail };
function csvFields(line) {
  const fields = [];
  let field = '';
  let quoted = false;
  for (let i = 0; i < line.length; i++) {
    const character = line[i];
    if (character === '"') {
      if (quoted && line[i + 1] === '"') { field += '"'; i++; }
      else quoted = !quoted;
    } else if (character === ',' && !quoted) {
      fields.push(field);
      field = '';
    } else field += character;
  }
  if (quoted) throw new Error('Unclosed CSV quote');
  fields.push(field);
  return fields;
}
const rows = fs.readFileSync(path.join(root, 'docs/docs_manifest.csv'), 'utf8')
  .trim().split(/\r?\n/).slice(1).map(line => {
    const fields = csvFields(line);
    if (fields.length !== 6) throw new Error('Unexpected source manifest CSV shape');
    return { number: Number(fields[0]), category: fields[1], title: fields[2], original: fields[5] };
  });
if (rows.length !== 148 || Object.keys(bodies).length !== 148) throw new Error('Expected 148 topics');

const excluded = new Set([59, 66, 67, 111, 112, 113, 135, 148]);
function stage(n) {
  if (excluded.has(n)) return '제외/후속';
  if (n <= 7) return '전 Stage';
  if (n <= 18) return n === 18 ? 'E' : 'A~B';
  if (n <= 34) return [25, 26].includes(n) ? 'A~B' : n === 34 ? 'D' : 'B~E';
  if (n <= 50) return [38, 39].includes(n) ? 'D' : 'A~B';
  if (n <= 60) return n >= 57 ? 'C~E' : 'C';
  if (n <= 65) return 'D';
  if (n <= 83) return 'A~E';
  if (n <= 86) return 'C';
  if (n <= 92) return n === 89 ? 'D' : 'B~E';
  if (n <= 110) return 'A~E';
  if (n <= 124) return 'B~E (print: A)';
  if (n <= 134) return n === 125 ? 'A~E' : 'E';
  return '전 Stage';
}
const hash = bytes => crypto.createHash('sha256').update(bytes).digest('hex');
fs.mkdirSync(path.join(out, 'specs'), { recursive: true });
fs.writeFileSync(path.join(out, 'DECISIONS.md'), decisionMarkdown(), 'utf8');
fs.writeFileSync(path.join(out, 'CONFORMANCE.md'), conformanceMarkdown(), 'utf8');
fs.writeFileSync(path.join(out, 'DIAGNOSTICS.csv'), 'code,category,trigger,primary,secondary,stage\n' +
  codes.map(row => row.join(',')).join('\n') + '\n', 'utf8');
writeFixtures(out);
const index = [];
const manifest = [];
const audit = [];
for (const row of rows) {
  const id = `NOVA-${String(row.number).padStart(3, '0')}`;
  const body = bodies[row.number];
  if (!body || body.length < 150) throw new Error(`Missing substantive body for ${id}`);
  const content = `# ${id} — ${row.title}\n\n` +
    `| 항목 | 값 |\n|---|---|\n| 대상 | Nova 0.1 |\n| 작성일 | 2026-10-03 |\n` +
    `| 문서 상태 | Draft — 기존 결정은 유지, 추가 상세는 승인 대기 |\n| 적용 Stage | ${stage(row.number)} |\n` +
    `| 근거 | [원본 ${id}](../../${row.original}) |\n\n` +
    `이 문서는 원본을 대체하는 확정 사양이 아니다. 기존 확정 기준은 [CANONICAL](../CANONICAL.md)을 따르며, ` +
    `새 의미·문법·API·정책은 [DECISIONS](../DECISIONS.md)의 승인이 필요하다. ` +
    `D01~D05 Lexer 상세는 [승인 기준](../ACCEPTED_LEXER.md)을 따른다. [전체 색인](../INDEX.md).\n\n` +
    ([14, 15, 16, 35, 43, 44, 72, 73, 136].includes(row.number)
      ? `Stage A Parser 구문/복구는 사용자 승인 [P01](../PARSER_STAGE_A_PROPOSAL.md)과 [전용 EBNF](../GRAMMAR_STAGE_A.ebnf)가 우선한다. 나머지 추가 상세는 Draft다.\n\n` : '') +
    ([19, 24, 25, 26, 35, 43, 44, 74, 75, 76, 77, 114, 116, 136].includes(row.number)
      ? `Stage A 단일 파일 의미 검사는 사용자 승인 [P02](../SEMANTICS_STAGE_A_PROPOSAL.md)가 우선한다. runtime/전체 타입/미래 Stage의 추가 상세는 Draft다.\n\n` : '') +
    ([81, 82, 83, 91, 92, 136].includes(row.number)
      ? `Stage A MIR은 원본 CFG/Place 기준과 P02의 평가 순서에 따라 구현했다. [MIR 구현 기록](../MIR_IMPLEMENTATION.md)은 현재 API/검증 경계이며 runtime/미래 Stage 정책의 승인이 아니다.\n\n` : '') +
    ([25, 26, 93, 94, 95, 96, 97, 98, 99, 100, 101, 114, 116, 125, 128, 136, 139, 140].includes(row.number)
      ? `Stage A Native 최소 arithmetic/print/entry/internal ABI/toolchain 계약은 사용자 승인 [P03](../NATIVE_STAGE_A_PROPOSAL.md)를 따른다. [구현·지원·검증 범위](../NATIVE_IMPLEMENTATION.md). 전체 D07~D28과 미래 Stage 정책은 Draft다.\n\n` : '') +
    ([14, 15, 16, 19, 24, 43, 44, 73, 74, 75, 76, 77, 81, 82, 83, 91, 136].includes(row.number)
      ? `Stage B 가변 지역 변수·반복문 최소 부분은 사용자 진행 요청으로 승인한 [P04](../CONTROL_STAGE_B_PROPOSAL.md)와 [전용 EBNF](../GRAMMAR_STAGE_B_CONTROL.ebnf)가 우선한다. [구현·검증 기록](../CONTROL_IMPLEMENTATION.md). 전체 Stage B와 ownership/Drop 정책은 Draft다.\n\n` : '') +
    ([14, 15, 16, 19, 24, 25, 26, 32, 73, 74, 75, 76, 77, 81, 87, 91, 136].includes(row.number)
      ? `함수 내부 const와 제한된 상수 평가는 사용자 승인 [P05](../CONST_STAGE_B_PROPOSAL.md)와 [전용 EBNF](../GRAMMAR_STAGE_B_CONST.ebnf)가 우선한다. [구현·검증 기록](../CONST_IMPLEMENTATION.md). 전역 상수는 P05 범위가 아니며 const function/전체 D09 상세는 Draft다.\n\n` : '') +
    ([14, 15, 16, 19, 24, 25, 26, 32, 73, 74, 75, 76, 77, 78, 81, 87, 91, 136].includes(row.number)
      ? `단일 파일 전역 const·forward dependency/cycle 최소 부분은 사용자 승인 [P06](../GLOBAL_CONST_STAGE_B_PROPOSAL.md)와 [전용 EBNF](../GRAMMAR_STAGE_B_GLOBAL_CONST.ebnf)를 따른다. [구현·검증 기록](../GLOBAL_CONST_IMPLEMENTATION.md). 함수 print shadow는 기존 P02대로 허용하고 전역 const print만 N2002다. module/const function/전체 D06·D09는 Draft다.\n\n` : '') +
    ([25, 26, 74, 75, 76, 77, 81, 87, 91, 93, 94, 95, 96, 97, 114, 116, 125, 136].includes(row.number)
      ? `8종 고정 폭 정수·기대/peer literal 문맥·lossless 승격·checked runtime/const·MIR 변환·보간은 사용자 승인 [P07](../INTEGER_STAGE_B_PROPOSAL.md)를 따른다. [구현·검증 기록](../INTEGER_IMPLEMENTATION.md). P06 grammar를 재사용한다. 숫자 cast는 P10, 전체 D07은 Draft다.\n\n` : '') +
    ([10, 11, 14, 15, 16, 25, 26, 73, 74, 75, 76, 77, 81, 87, 91, 93, 94, 95, 96, 97, 114, 116, 136].includes(row.number)
      ? `char의 Unicode scalar 값·동일 타입 비교·선언/대입/함수/const·UTF-8 보간·private scalar ABI는 사용자 승인 [P08](../CHAR_STAGE_B_PROPOSAL.md)과 [CHAR primary EBNF](../GRAMMAR_STAGE_B_CHAR.ebnf)를 따른다. [구현·검증 기록](../CHAR_IMPLEMENTATION.md). D04 Lexer/escape/END는 유지하며 cast/char 산술·전체 D07은 Draft다.\n\n` : '') +
    ([10, 14, 15, 16, 25, 26, 32, 73, 74, 75, 76, 77, 81, 87, 91, 93, 94, 95, 96, 97, 114, 116, 125, 136].includes(row.number)
      ? `binary32/64 literal·손실 없는 숫자 승격·IEEE 산술/비교·canonical NaN·const·최단 fixed decimal 보간·private ABI는 사용자 승인 [P09](../FLOAT_STAGE_B_PROPOSAL.md)와 [FLOAT primary EBNF](../GRAMMAR_STAGE_B_FLOAT.ebnf)를 따른다. [구현·검증 기록](../FLOAT_IMPLEMENTATION.md). float IEEE 결과는 const 실패가 아니며 INT checked 정책은 유지한다. 숫자 cast는 P10, float remainder/math API·전체 D07은 Draft다.\n\n` : '') +
    ([14, 15, 16, 25, 26, 73, 74, 75, 76, 77, 78, 81, 87, 91, 93, 94, 95, 96, 97, 114, 116, 136].includes(row.number)
      ? `숫자 10종의 postfix as·operand literal 문맥 격리·checked 범위/직접 RN 반올림·float truncation·const N3201/Runtime Abort는 사용자 승인 [P10](../CAST_STAGE_B_PROPOSAL.md)과 [전용 EBNF](../GRAMMAR_STAGE_B_CAST.ebnf)를 따른다. [구현·검증 기록](../CAST_IMPLEMENTATION.md). Bool/Char/unsafe cast와 전체 D07은 Draft다.\n\n` : '') +
    ([14, 16, 19, 20, 21, 22, 23, 24, 32, 70, 73, 74, 75, 76, 77, 78, 81, 87, 91, 93, 94, 95, 96, 97, 125, 136].includes(row.number)
      ? `root-relative 함수/전역 const item import·alias·internal/private/public·reachable graph·cross-file const·entry/source identity는 사용자 승인 [P11](../MODULE_STAGE_B_PROPOSAL.md)과 [전용 EBNF](../GRAMMAR_STAGE_B_MODULE.ebnf)를 따른다. [구현·검증 기록](../MODULE_IMPLEMENTATION.md). module alias/qualified value/reexport/Package와 전체 D06/D30은 후속이다.\n\n` : '') +
    ([14, 16, 19, 20, 23, 24, 25, 27, 31, 33, 51, 73, 74, 75, 76, 77, 78, 81, 83, 87, 91, 93, 94, 95, 96, 97, 136].includes(row.number)
      ? `nominal Copy struct·위치 생성·type import·가변 field 경로·const·private layout/ABI와 자원 제한은 사용자 승인 [P12](../STRUCT_STAGE_B_PROPOSAL.md)와 [37-production EBNF](../GRAMMAR_STAGE_B_STRUCT.ebnf)를 따른다. [구현·검증 기록](../STRUCT_IMPLEMENTATION.md). String field·init/Drop·일반 Move/borrow와 전체 D06/D10/D12/D16/D30은 후속이다.\n\n` : '') +
    ([14, 16, 25, 27, 29, 32, 33, 51, 73, 74, 75, 76, 77, 78, 81, 83, 87, 91, 93, 94, 95, 96, 97, 136].includes(row.number)
      ? `structural Copy Tuple·numeric selector subspan·혼합 가변 경로·const·private aggregate ABI와 자원 제한은 사용자 승인 [P13](../TUPLE_STAGE_B_PROPOSAL.md)와 [40-production EBNF](../GRAMMAR_STAGE_B_TUPLE.ebnf)를 따른다. [구현·검증 기록](../TUPLE_IMPLEMENTATION.md). Array/일반 Move element와 전체 D09/D10/D12/D16은 후속이다.\n\n` : '') +
    ([14, 16, 25, 27, 29, 32, 33, 47, 48, 51, 73, 74, 75, 76, 77, 78, 81, 83, 87, 91, 93, 94, 95, 96, 97, 136].includes(row.number)
      ? `사용자 승인한 Copy Enum·qualified variant·Enum/Bool statement match·coverage/binder·const·private tagged ABI는 [P14](../ENUM_STAGE_B_PROPOSAL.md), [48-production EBNF](../GRAMMAR_STAGE_B_ENUM.ebnf), [수용 fixture](../enum-proposal-fixtures/README.md), [구현 기록](../ENUM_IMPLEMENTATION.md)을 따른다. 전체 D06/D08/D09/D10/D12/D16/D25/D30은 계속 Draft다.\n\n` : '') +
    ([14, 16, 25, 27, 28, 29, 32, 33, 47, 48, 51, 73, 74, 75, 76, 77, 78, 81, 83, 87, 91, 93, 94, 95, 96, 97, 119, 120, 136].includes(row.number)
      ? `사용자 승인한 Copy Option<T>/Result<T,E>·T?·qualified 생성/none·문맥·match·const·private tagged ABI는 [P15 Accepted](../OPTION_RESULT_STAGE_B_PROPOSAL.md), [51-production EBNF](../GRAMMAR_STAGE_B_OPTION_RESULT.ebnf), [수용 fixture](../option-result-proposal-fixtures/README.md), [구현 기록](../OPTION_RESULT_IMPLEMENTATION.md)을 따른다. try·Move/Drop·사용자 Generic과 전체 D06/D08/D09/D10/D12/D15/D16/D23/D25/D30은 후속이다.\n\n` : '') + body.trim() + '\n';
  fs.writeFileSync(path.join(out, 'specs', `${id}.md`), content, 'utf8');
  const original = fs.readFileSync(path.join(root, 'docs', row.original));
  const originalText = original.toString('utf8');
  const usesGenericTemplate = originalText.includes('입력·출력·불변 조건을 명시한다');
  index.push(`| ${id} | ${row.category} | [${row.title}](specs/${id}.md) | ${stage(row.number)} | Draft |`);
  audit.push(`| [${id}](../${row.original}) | ${usesGenericTemplate ? '공통 양식 포함; 주제별 정의 보완' : '전용 요약; 상세 계약/검증 보완'} | [작성 문서](specs/${id}.md) |`);
  manifest.push({ id, title: row.title, category: row.category, stage: stage(row.number),
    status: 'Draft', original: `../${row.original}`, draft: `specs/${id}.md`,
    original_sha256: hash(original), draft_sha256: hash(content) });
}
fs.writeFileSync(path.join(out, 'INDEX.md'), `# Nova 0.1 개발 문서 전체 색인\n\n` +
  `148개 원본 주제를 빠짐없이 보완했다. 전체 상세는 Draft이며 D01~D05 Lexer, [P01 Parser](PARSER_STAGE_A_PROPOSAL.md), [P02 의미 검사](SEMANTICS_STAGE_A_PROPOSAL.md), [P03 Native](NATIVE_STAGE_A_PROPOSAL.md), [P04 가변 변수·반복문](CONTROL_STAGE_B_PROPOSAL.md), [P05 const](CONST_STAGE_B_PROPOSAL.md) subset은 Accepted다. 승인/구현/테스트 통과 상태를 구분한다.\n\n` +
  `[P06 단일 파일 전역 const](GLOBAL_CONST_STAGE_B_PROPOSAL.md)와 [전용 EBNF](GRAMMAR_STAGE_B_GLOBAL_CONST.ebnf) subset도 Accepted이며 [구현·검증 기록](GLOBAL_CONST_IMPLEMENTATION.md)을 따른다.\n\n` +
  `[P07 고정 폭 정수·승격](INTEGER_STAGE_B_PROPOSAL.md) subset도 Accepted이며 [구현·검증 기록](INTEGER_IMPLEMENTATION.md)을 따른다. 기존 P06 source grammar를 재사용한다.\n\n` +
  `[P08 char·scalar 비교·UTF-8 보간](CHAR_STAGE_B_PROPOSAL.md)과 [CHAR primary 전용 EBNF](GRAMMAR_STAGE_B_CHAR.ebnf)는 Accepted이며 [구현·검증 기록](CHAR_IMPLEMENTATION.md)을 따른다.\n\n` +
  `[P09 float·IEEE 결과·숫자 승격·보간](FLOAT_STAGE_B_PROPOSAL.md)과 [전용 EBNF](GRAMMAR_STAGE_B_FLOAT.ebnf)는 Accepted이며 [구현·검증 기록](FLOAT_IMPLEMENTATION.md)을 따른다.\n\n` +
  `[P10 명시적 숫자 cast](CAST_STAGE_B_PROPOSAL.md)와 [전용 EBNF](GRAMMAR_STAGE_B_CAST.ebnf)는 Accepted이며 [구현·검증 기록](CAST_IMPLEMENTATION.md)을 따른다.\n\n` +
  `[P11 Module·다중 파일 최소 계약](MODULE_STAGE_B_PROPOSAL.md)과 [전용 EBNF](GRAMMAR_STAGE_B_MODULE.ebnf)는 Accepted이며 [구현·검증 기록](MODULE_IMPLEMENTATION.md)을 따른다.\n\n` +
  `[P12 Copy struct 최소 계약](STRUCT_STAGE_B_PROPOSAL.md)과 [37-production EBNF](GRAMMAR_STAGE_B_STRUCT.ebnf), [수용 fixture](struct-proposal-fixtures/README.md)는 Accepted이며 [구현·검증 기록](STRUCT_IMPLEMENTATION.md)을 따른다. 전체 D06/D10/D12/D16/D30 승인이 아니다.\n\n` +
  `[P13 Copy Tuple 최소 계약](TUPLE_STAGE_B_PROPOSAL.md), [40-production EBNF](GRAMMAR_STAGE_B_TUPLE.ebnf), [수용 fixture](tuple-proposal-fixtures/README.md)는 Accepted이며 [구현 기록](TUPLE_IMPLEMENTATION.md)을 따른다. 전체 D09/D10/D12/D16 승인이 아니다.\n\n` +
  `[P14 Copy Enum·statement match 최소 계약](ENUM_STAGE_B_PROPOSAL.md), [48-production EBNF](GRAMMAR_STAGE_B_ENUM.ebnf), [수용 fixture](enum-proposal-fixtures/README.md)는 Accepted / 구현 완료이며 [구현 기록](ENUM_IMPLEMENTATION.md)을 따른다. 전체 D06/D08/D09/D10/D12/D16/D25/D30 승인이 아니다.\n\n` +
  `[P15 Copy Option·Result·nullable 최소 계약](OPTION_RESULT_STAGE_B_PROPOSAL.md), [51-production EBNF](GRAMMAR_STAGE_B_OPTION_RESULT.ebnf), [수용 fixture](option-result-proposal-fixtures/README.md)는 Accepted / 구현 완료다. [구현·검증 기록](OPTION_RESULT_IMPLEMENTATION.md)을 따른다.\n\n` +
  `[시작 문서](README.md) · [결정](DECISIONS.md) · [문법](GRAMMAR.ebnf) · [검증 사례](CONFORMANCE.md)\n\n` +
  `| ID | 분야 | 작성 문서 | Stage | 상태 |\n|---|---|---|---|---|\n${index.join('\n')}\n`, 'utf8');
fs.writeFileSync(path.join(out, 'SPEC_AUDIT.md'), `# 원본 사양 감사와 보완 경계\n\n` +
  `원본 148개 NOVA 문서를 기준으로 새 계약을 작성했다. '공통 양식 포함'은 자동 탐지한 구조 분류이며 ` +
  `그 문서 전체가 무효라는 판정은 아니다. 원본 파일은 수정하지 않았다.\n\n` +
  `## 구현 전 핵심 문제\n\n` +
  `- NOVA-014: 실제 EBNF Production 부재 → 전체 GRAMMAR는 Draft. D01~D05와 Stage A Parser P01/GRAMMAR_STAGE_A.ebnf는 승인 완료. 나머지 의미/미래 구문 결정은 승인 필요.\n` +
  `- NOVA-004 vs 072: use keyword 누락 → D01; alias/lambda/noPanic contextual 표기도 검토.\n` +
  `- NOVA-070: compiler source Span과 runtime Span<T> 항목 혼재 → 담당 문서 070/118 구분(D10).\n` +
  `- NOVA-026/037: numeric widening/default/overload 비용 불완전 → D07/D11.\n` +
  `- Stage A HIR/name/type 최소 부분은 P02 승인 완료. 전체 widening/overload/ABI/runtime 정책은 Draft.\n` +
  `- NOVA-029/117/121: growable Array와 List 역할 중복 → D23.\n` +
  `- NOVA-059/066/067/135/148: 비지원 기능의 문서 존재는 구현 허가가 아님.\n` +
  `- NOVA-111~113: hosted adapters는 C FFI 동결표에 없음 → D27 후속 제안.\n` +
  `- 사양 날짜는 원본 값 그대로; 새 작성일은 2026-10-03.\n\n` +
  `| 원본 | 조사 분류 | 보완 문서 |\n|---|---|---|\n${audit.join('\n')}\n`, 'utf8');
fs.writeFileSync(path.join(out, 'MANIFEST.json'), JSON.stringify({
  schema_version: 1, language: 'Nova', language_version: '0.1', document_status: 'PartiallyAccepted',
  accepted_decisions: ['D01', 'D02', 'D03', 'D04', 'D05'],
  accepted_proposals: [
    { id: 'P01', scope: 'Stage A Parser syntax and recovery', approval_date: '2026-10-04', document: 'PARSER_STAGE_A_PROPOSAL.md', grammar: 'GRAMMAR_STAGE_A.ebnf' },
    { id: 'P02', scope: 'Stage A single-file HIR name and type checking', approval_date: '2026-10-04', document: 'SEMANTICS_STAGE_A_PROPOSAL.md' },
    { id: 'P03', scope: 'Stage A Native checked arithmetic print entry internal ABI and LLVM host subset', approval_date: '2026-10-04', document: 'NATIVE_STAGE_A_PROPOSAL.md' },
    { id: 'P04', scope: 'Stage B initialized mutable locals direct assignment while break continue and N3004', approval_date: '2026-10-04', document: 'CONTROL_STAGE_B_PROPOSAL.md', grammar: 'GRAMMAR_STAGE_B_CONTROL.ebnf' },
    { id: 'P05', scope: 'Stage B local const restricted expressions checked evaluation and 10000 node budget', approval_date: '2026-10-04', document: 'CONST_STAGE_B_PROPOSAL.md', grammar: 'GRAMMAR_STAGE_B_CONST.ebnf' },
    { id: 'P06', scope: 'Stage B single-file global const forward references static dependency cycles and P05 evaluation', approval_date: '2026-10-04', document: 'GLOBAL_CONST_STAGE_B_PROPOSAL.md', grammar: 'GRAMMAR_STAGE_B_GLOBAL_CONST.ebnf', print_name_policy: 'P02 function shadow retained; global const print is N2002' },
    { id: 'P07', approval_date: '2026-10-04', scope: 'Stage B fixed-width signed and unsigned integers contextual literals lossless widening checked arithmetic const MIR conversion and interpolation', document: 'INTEGER_STAGE_B_PROPOSAL.md', grammar: 'GRAMMAR_STAGE_B_GLOBAL_CONST.ebnf', grammar_change: false, implementation_verified: true },
    { id: 'P08', approval_date: '2026-10-04', scope: 'Stage B Unicode scalar char literals type comparison const UTF-8 interpolation and private scalar ABI', document: 'CHAR_STAGE_B_PROPOSAL.md', grammar: 'GRAMMAR_STAGE_B_CHAR.ebnf', grammar_change: true, implementation_verified: true },
    { id: 'P09', approval_date: '2026-10-04', scope: 'Stage B binary32 binary64 contextual real literals lossless numeric promotion IEEE arithmetic canonical NaN const interpolation and private ABI', document: 'FLOAT_STAGE_B_PROPOSAL.md', grammar: 'GRAMMAR_STAGE_B_FLOAT.ebnf', grammar_change: true, implementation_verified: true },
    { id: 'P10', approval_date: '2026-10-05', scope: 'Stage B explicit checked numeric postfix casts integer ranges float rounding truncation const and private Native failure behavior', document: 'CAST_STAGE_B_PROPOSAL.md', grammar: 'GRAMMAR_STAGE_B_CAST.ebnf', grammar_change: true, implementation_verified: true },
    { id: 'P11', approval_date: '2026-10-05', scope: 'Stage B root-relative multi-file direct function and global const item imports aliases visibility module graph and cross-file checking', document: 'MODULE_STAGE_B_PROPOSAL.md', grammar: 'GRAMMAR_STAGE_B_MODULE.ebnf', grammar_change: true, implementation_verified: true },
    { id: 'P12', status: 'Accepted', approval_date: '2026-10-05', scope: 'Stage B nominal Copy structs positional construction type imports field reads mutable paths const private aggregate ABI and resource bounds', document: 'STRUCT_STAGE_B_PROPOSAL.md', grammar: 'GRAMMAR_STAGE_B_STRUCT.ebnf', grammar_change: true, implementation_verified: true },
    { id: 'P13', status: 'Accepted', approval_date: '2026-10-07', scope: 'Stage B structural Copy tuples selector token subspans mixed mutation paths const private aggregate ABI and resource bounds', document: 'TUPLE_STAGE_B_PROPOSAL.md', grammar: 'GRAMMAR_STAGE_B_TUPLE.ebnf', grammar_change: true, implementation_verified: true },
    { id: 'P14', status: 'Accepted', approval_date: '2026-10-07', scope: 'Stage B nominal Copy enums qualified variants Enum and Bool statement match Copy binders exhaustive coverage const construction private tagged ABI and resource bounds', document: 'ENUM_STAGE_B_PROPOSAL.md', grammar: 'GRAMMAR_STAGE_B_ENUM.ebnf', grammar_change: true, implementation_verified: true },
    { id: 'P15', status: 'Accepted', approval_date: '2026-10-07', scope: 'Stage B intrinsic Copy Option Result nullable type arguments contextual construction none match const private tagged ABI and bounded specialization', document: 'OPTION_RESULT_STAGE_B_PROPOSAL.md', grammar: 'GRAMMAR_STAGE_B_OPTION_RESULT.ebnf', grammar_change: true, implementation_verified: true },
  ],
  draft_proposals: [
  ],
  authored_date: '2026-10-03', topics: manifest,
}, null, 2) + '\n', 'utf8');
console.log(`Generated ${manifest.length} authored topic documents, INDEX, audit and manifest.`);
