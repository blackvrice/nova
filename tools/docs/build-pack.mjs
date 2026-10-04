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
      ? `단일 파일 전역 const·forward dependency/cycle 최소 부분은 사용자 승인 [P06](../GLOBAL_CONST_STAGE_B_PROPOSAL.md)와 [전용 EBNF](../GRAMMAR_STAGE_B_GLOBAL_CONST.ebnf)를 따른다. [구현·검증 기록](../GLOBAL_CONST_IMPLEMENTATION.md). 함수 print shadow는 기존 P02대로 허용하고 전역 const print만 N2002다. module/const function/전체 D06·D09는 Draft다.\n\n` : '') + body.trim() + '\n';
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
  ],
  authored_date: '2026-10-03', topics: manifest,
}, null, 2) + '\n', 'utf8');
console.log(`Generated ${manifest.length} authored topic documents, INDEX, audit and manifest.`);
