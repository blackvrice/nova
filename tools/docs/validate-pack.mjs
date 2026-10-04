import fs from 'node:fs';
import path from 'node:path';
import crypto from 'node:crypto';
import { cases, codes } from './registries.mjs';
import { decisions } from './decisions.mjs';

const root = path.resolve(import.meta.dirname, '../..');
const pack = path.join(root, 'docs/development-v0.1');
const reportPath = path.join(pack, 'VALIDATION.md');
const failures = [];
const checks = [];
const assert = (condition, message) => { if (!condition) failures.push(message); };
const hash = data => crypto.createHash('sha256').update(data).digest('hex');
function walk(dir) {
  return fs.readdirSync(dir, { withFileTypes: true }).flatMap(entry => {
    const full = path.join(dir, entry.name);
    return entry.isDirectory() ? walk(full) : [full];
  });
}
const files = walk(pack);
const manifest = JSON.parse(fs.readFileSync(path.join(pack, 'MANIFEST.json'), 'utf8'));
assert(manifest.topics.length === 148, 'Topic count != 148');
assert(new Set(manifest.topics.map(t => t.id)).size === 148, 'Duplicate NOVA IDs');
for (let i = 1; i <= 148; i++) {
  assert(manifest.topics.some(t => t.id === `NOVA-${String(i).padStart(3, '0')}`), `Missing NOVA-${i}`);
}
for (const topic of manifest.topics) {
  const original = path.resolve(pack, topic.original);
  const draft = path.resolve(pack, topic.draft);
  assert(fs.existsSync(original) && fs.existsSync(draft), `Missing files for ${topic.id}`);
  if (!fs.existsSync(original) || !fs.existsSync(draft)) continue;
  assert(hash(fs.readFileSync(original)) === topic.original_sha256, `Original hash changed ${topic.id}`);
  assert(hash(fs.readFileSync(draft)) === topic.draft_sha256, `Draft hash changed ${topic.id}`);
  const text = fs.readFileSync(draft, 'utf8');
  assert(text.includes('Draft') && topic.status === 'Draft', `Invalid status ${topic.id}`);
  assert((text.match(/^## /gm) ?? []).length >= 2, `Insufficient topic sections ${topic.id}`);
}
checks.push('148개 NOVA ID 연속성/중복/원본·보완 SHA-256/상태/본문 섹션');

const knownD = new Set(decisions.map(d => d[0]));
const knownT = new Set(cases.map(c => c[0]));
const knownCodes = new Set(codes.map(c => c[0]));
let linkCount = 0;
for (const file of files.filter(f => f.endsWith('.md'))) {
  if (path.basename(file) === 'VALIDATION.md') continue;
  const text = fs.readFileSync(file, 'utf8');
  const fences = text.match(/^```/gm) ?? [];
  assert(fences.length % 2 === 0, `Unbalanced code fences ${file}`);
  for (const match of text.matchAll(/\[[^\]]*\]\(([^)]+)\)/g)) {
    let target = match[1].replace(/^<|>$/g, '').split('#')[0];
    if (!target || /^[a-z][a-z0-9+.-]*:/i.test(target)) continue;
    target = decodeURIComponent(target);
    const resolvedTarget = path.resolve(path.dirname(file), target);
    assert(resolvedTarget === reportPath || fs.existsSync(resolvedTarget), `Broken link ${file} -> ${target}`);
    linkCount++;
  }
  for (const match of text.matchAll(/\bD\d{2}\b/g)) assert(knownD.has(match[0]), `Unknown decision ${match[0]} in ${file}`);
  for (const match of text.matchAll(/\bT\d{3}\b/g)) assert(knownT.has(match[0]), `Unknown conformance ${match[0]} in ${file}`);
}
checks.push(`${linkCount}개 로컬 Markdown 링크/코드 fence/D·T 참조`);
assert(knownD.size === 30 && knownT.size === 60, 'Decision/case registry counts mismatch');
assert(knownCodes.size === codes.length && codes.every(c => /^N[1-589]\d{3}$/.test(c[0])), 'Invalid/duplicate diagnostic codes');
checks.push(`30개 결정 (Accepted 5 / Draft 25), 60개 수용 묶음, ${codes.length}개 진단 코드 유일성/영역`);

function validateGrammar(filename) {
  const grammar = fs.readFileSync(path.join(pack, filename), 'utf8');
  const stripped = grammar.replace(/\(\*[\s\S]*?\*\)/g, '').replace(/"[^"\n]*"/g, '').replace(/\?[^?]*\?/g, '');
  const definitions = [...stripped.matchAll(/^\s*([a-z][a-z_0-9]*)\s*=/gm)].map(m => m[1]);
  const defined = new Set(definitions);
  assert(defined.size === definitions.length, `${filename}: Duplicate grammar production`);
  const words = [...stripped.matchAll(/\b[a-z][a-z_0-9]*\b/g)].map(m => m[0]);
  for (const word of words) assert(defined.has(word), `${filename}: Undefined grammar nonterminal ${word}`);
  assert(defined.has('program') && defined.has('expression') && defined.has('type'), `${filename}: Missing grammar roots`);
  const references = new Map([...stripped.matchAll(/^\s*([a-z][a-z_0-9]*)\s*=([\s\S]*?);/gm)]
    .map(m => [m[1], [...m[2].matchAll(/\b[a-z][a-z_0-9]*\b/g)].map(r => r[0])]));
  const reachable = new Set();
  const grammarQueue = ['program'];
  while (grammarQueue.length) {
    const next = grammarQueue.pop();
    if (reachable.has(next)) continue;
    reachable.add(next);
    grammarQueue.push(...(references.get(next) ?? []));
  }
  for (const definition of definitions) {
    assert(reachable.has(definition), `${filename}: Unreachable grammar production ${definition}`);
  }
  checks.push(`${filename}: ${defined.size}개 EBNF production 중복/미정의·도달 불가 nonterminal 검사 (무모호성 증명 아님)`);
}
validateGrammar('GRAMMAR.ebnf');
validateGrammar('GRAMMAR_STAGE_A.ebnf');
validateGrammar('GRAMMAR_STAGE_B_CONTROL.ebnf');
validateGrammar('GRAMMAR_STAGE_B_CONST.ebnf');
validateGrammar('GRAMMAR_STAGE_B_GLOBAL_CONST.ebnf');
validateGrammar('GRAMMAR_STAGE_B_CHAR.ebnf');
validateGrammar('GRAMMAR_STAGE_B_FLOAT.ebnf');
validateGrammar('GRAMMAR_STAGE_B_CAST.ebnf');
const parserProposal = manifest.accepted_proposals?.find(p => p.id === 'P01');
assert(parserProposal?.approval_date === '2026-10-04' && parserProposal?.grammar === 'GRAMMAR_STAGE_A.ebnf', 'Missing P01 approval ledger');
assert(fs.readFileSync(path.join(pack, 'PARSER_STAGE_A_PROPOSAL.md'), 'utf8').includes('Accepted / 2026-10-04 사용자 승인'), 'Invalid P01 status');
checks.push('P01 Stage A Parser 승인 범위/날짜와 전용 EBNF 기록');
const semanticProposal = manifest.accepted_proposals?.find(p => p.id === 'P02');
assert(semanticProposal?.approval_date === '2026-10-04' && semanticProposal?.document === 'SEMANTICS_STAGE_A_PROPOSAL.md', 'Missing P02 approval ledger');
assert(fs.readFileSync(path.join(pack, 'SEMANTICS_STAGE_A_PROPOSAL.md'), 'utf8').includes('Accepted / 2026-10-04 사용자 승인'), 'Invalid P02 status');
checks.push('P02 Stage A 의미 검사 승인 범위/날짜 기록');
const nativeProposal = manifest.accepted_proposals?.find(p => p.id === 'P03');
assert(nativeProposal?.approval_date === '2026-10-04' && nativeProposal?.document === 'NATIVE_STAGE_A_PROPOSAL.md', 'Missing P03 approval ledger');
assert(fs.readFileSync(path.join(pack, 'NATIVE_STAGE_A_PROPOSAL.md'), 'utf8').includes('Accepted / 2026-10-04 사용자 승인'), 'Invalid P03 status');
checks.push('P03 Stage A Native 승인 subset/날짜 기록');
const controlProposal = manifest.accepted_proposals?.find(p => p.id === 'P04');
assert(controlProposal?.approval_date === '2026-10-04' && controlProposal?.document === 'CONTROL_STAGE_B_PROPOSAL.md'
  && controlProposal?.grammar === 'GRAMMAR_STAGE_B_CONTROL.ebnf', 'Missing P04 approval ledger');
assert(fs.readFileSync(path.join(pack, 'CONTROL_STAGE_B_PROPOSAL.md'), 'utf8').includes('Accepted / 2026-10-04 사용자 진행 요청'), 'Invalid P04 status');
assert(knownCodes.has('N3004'), 'Missing P04 immutable assignment diagnostic');
checks.push('P04 Stage B control 승인 subset/날짜/전용 EBNF와 N3004 등록');
const constProposal = manifest.accepted_proposals?.find(p => p.id === 'P05');
assert(constProposal?.approval_date === '2026-10-04' && constProposal?.document === 'CONST_STAGE_B_PROPOSAL.md'
  && constProposal?.grammar === 'GRAMMAR_STAGE_B_CONST.ebnf', 'Missing P05 approval ledger');
assert(fs.readFileSync(path.join(pack, 'CONST_STAGE_B_PROPOSAL.md'), 'utf8').includes('Accepted / 2026-10-04 사용자 승인'), 'Invalid P05 status');
checks.push('P05 local const 승인 subset/날짜/전용 EBNF 기록');
const globalConstProposal = manifest.accepted_proposals?.find(p => p.id === 'P06');
assert(globalConstProposal?.approval_date === '2026-10-04'
  && globalConstProposal?.document === 'GLOBAL_CONST_STAGE_B_PROPOSAL.md'
  && globalConstProposal?.grammar === 'GRAMMAR_STAGE_B_GLOBAL_CONST.ebnf', 'Missing P06 approval ledger');
assert(globalConstProposal?.print_name_policy === 'P02 function shadow retained; global const print is N2002', 'Missing P06 print clarification ledger');
const globalConstText = fs.readFileSync(path.join(pack, 'GLOBAL_CONST_STAGE_B_PROPOSAL.md'), 'utf8');
assert(globalConstText.includes('Accepted / 2026-10-04 사용자 승인')
  && globalConstText.includes('기존 함수 print 허용, 전역 const print만 거부'), 'Invalid P06 approval or clarification');
assert(fs.readFileSync(path.join(pack, 'GRAMMAR_STAGE_B_GLOBAL_CONST.ebnf'), 'utf8').includes('Accepted by the user on 2026-10-04'), 'Invalid P06 grammar status');
checks.push('P06 global const 승인 subset/날짜/전용 EBNF·P02 print shadow 보존 정정');
const integerProposal = manifest.accepted_proposals?.find(p => p.id === 'P07');
assert(integerProposal?.approval_date === '2026-10-04' && integerProposal?.implementation_verified === true
  && integerProposal?.document === 'INTEGER_STAGE_B_PROPOSAL.md'
  && integerProposal?.grammar === 'GRAMMAR_STAGE_B_GLOBAL_CONST.ebnf'
  && integerProposal?.grammar_change === false, 'Missing P07 approved ledger or unchanged grammar boundary');
assert(!manifest.draft_proposals?.some(p => p.id === 'P07'), 'Accepted P07 still in draft ledger');
assert(fs.readFileSync(path.join(pack, 'INTEGER_STAGE_B_PROPOSAL.md'), 'utf8').includes('Accepted / 2026-10-04 사용자 승인'), 'Invalid P07 approval status');
checks.push('P07 integer 승인 subset/날짜·P06 grammar 재사용·전체 D07 Draft 경계');

const charProposal = manifest.accepted_proposals?.find(p => p.id === 'P08');
assert(charProposal?.approval_date === '2026-10-04' && charProposal?.implementation_verified === true
  && charProposal?.document === 'CHAR_STAGE_B_PROPOSAL.md'
  && charProposal?.grammar === 'GRAMMAR_STAGE_B_CHAR.ebnf'
  && charProposal?.grammar_change === true, 'Missing P08 approved char ledger');
assert(!manifest.draft_proposals?.some(p => p.id === 'P08'), 'Accepted P08 still in draft ledger');
assert(fs.readFileSync(path.join(pack, 'CHAR_STAGE_B_PROPOSAL.md'), 'utf8').includes('Accepted / 2026-10-04 사용자 승인'), 'Invalid P08 approval status');
const charGrammar = fs.readFileSync(path.join(pack, 'GRAMMAR_STAGE_B_CHAR.ebnf'), 'utf8');
assert(charGrammar.includes('Accepted by the user on 2026-10-04'), 'Invalid P08 grammar approval status');
const compactGrammar = text => text.replace(/\(\*[\s\S]*?\*\)/g, '').replace(/\s+/g, '');
const baseGrammar = fs.readFileSync(path.join(pack, 'GRAMMAR_STAGE_B_GLOBAL_CONST.ebnf'), 'utf8');
assert(compactGrammar(charGrammar).replace('"INT"|"CHAR"|', '"INT"|') === compactGrammar(baseGrammar), 'P08 grammar changes more than the CHAR primary terminal');
checks.push('P08 char 승인 subset/날짜·전용 EBNF는 CHAR primary만 추가·전체 D07 Draft 경계');

const floatProposal = manifest.accepted_proposals?.find(p => p.id === 'P09');
assert(floatProposal?.approval_date === '2026-10-04' && floatProposal?.implementation_verified === true
  && floatProposal?.document === 'FLOAT_STAGE_B_PROPOSAL.md'
  && floatProposal?.grammar === 'GRAMMAR_STAGE_B_FLOAT.ebnf'
  && floatProposal?.grammar_change === true, 'Missing P09 approved float ledger');
assert(!manifest.draft_proposals?.some(p => p.id === 'P09'), 'Accepted P09 still in draft ledger');
assert(fs.readFileSync(path.join(pack, 'FLOAT_STAGE_B_PROPOSAL.md'), 'utf8').includes('Accepted / 2026-10-04 사용자 승인'), 'Invalid P09 approval status');
const floatGrammar = fs.readFileSync(path.join(pack, 'GRAMMAR_STAGE_B_FLOAT.ebnf'), 'utf8');
assert(floatGrammar.includes('Accepted by the user on 2026-10-04'), 'Invalid P09 grammar approval status');
assert(compactGrammar(floatGrammar).replace('"INT"|"FLOAT"|', '"INT"|') === compactGrammar(charGrammar), 'P09 grammar changes more than the FLOAT primary terminal');
checks.push('P09 float 승인 subset/날짜·전용 EBNF는 FLOAT primary만 추가·전체 D07 Draft 경계');

const castProposal = manifest.accepted_proposals?.find(p => p.id === 'P10');
assert(castProposal?.approval_date === '2026-10-05' && castProposal?.implementation_verified === true
  && castProposal?.document === 'CAST_STAGE_B_PROPOSAL.md'
  && castProposal?.grammar === 'GRAMMAR_STAGE_B_CAST.ebnf'
  && castProposal?.grammar_change === true, 'Missing P10 approved numeric cast ledger');
assert(!manifest.draft_proposals?.some(p => p.id === 'P10'), 'Accepted P10 still in draft ledger');
assert(fs.readFileSync(path.join(pack, 'CAST_STAGE_B_PROPOSAL.md'), 'utf8').includes('Accepted / 2026-10-05 사용자 승인'), 'Invalid P10 approval status');
const castGrammar = fs.readFileSync(path.join(pack, 'GRAMMAR_STAGE_B_CAST.ebnf'), 'utf8');
assert(castGrammar.includes('Accepted by the user on 2026-10-05'), 'Invalid P10 grammar approval status');
assert(compactGrammar(castGrammar).replace('|"as",type', '') === compactGrammar(floatGrammar), 'P10 grammar changes more than the as type postfix');
checks.push('P10 숫자 cast 승인 subset/날짜·전용 EBNF는 as type postfix만 추가·전체 D07 Draft 경계');

const fixtureManifest = JSON.parse(fs.readFileSync(path.join(pack, 'fixtures/CASE_MANIFEST.json'), 'utf8'));
for (const fixture of fixtureManifest.fixtures) {
  const source = fs.readFileSync(path.join(pack, 'fixtures', fixture.source));
  const expected = JSON.parse(fs.readFileSync(path.join(pack, 'fixtures', fixture.expectation), 'utf8'));
  assert(knownT.has(expected.case_id), `Unknown fixture case ${fixture.name}`);
  assert(expected.status === 'Draft' && expected.implementation_verified === false, `Incorrect fixture status ${fixture.name}`);
  if (expected.diagnostic_code) assert(knownCodes.has(expected.diagnostic_code), `Unknown fixture code ${fixture.name}`);
  if (expected.primary) {
    const { start, end } = expected.primary;
    assert(start >= 0 && start <= end && end <= source.length, `Invalid fixture span ${fixture.name}`);
    const text = source.toString('utf8');
    const byteBoundaries = new Set([0]);
    let offset = 0;
    for (const char of text) { offset += Buffer.byteLength(char); byteBoundaries.add(offset); }
    assert(byteBoundaries.has(start) && byteBoundaries.has(end), `UTF8-split fixture span ${fixture.name}`);
  }
}
checks.push(`${fixtureManifest.fixtures.length}개 예제 sidecar/UTF-8 byte Span/등록 code 검사 (컴파일 실행 아님)`);

const validationDate = new Intl.DateTimeFormat('en-CA', {
  timeZone: 'Asia/Seoul', year: 'numeric', month: '2-digit', day: '2-digit',
}).format(new Date());
const report = `# 문서 기계 검증 결과\n\n검사일: ${validationDate}. 명령: node tools/docs/validate-pack.mjs\n\n` +
  `결과: **${failures.length ? 'FAIL' : 'PASS'}**\n\n` + checks.map(c => `- ${c}`).join('\n') + '\n\n' +
  '## 검증의 범위\n\n이 검사는 문서 구조/링크/ID/hash/grammar 참조와 fixture 데이터 유효성을 확인한다. ' +
  '언어 사양 승인, EBNF 무모호성/완전성 증명, parser/semantic 구현 검증, 예제 Native 실행, ' +
  'Rust fmt/clippy/test/check는 실행하지 않았다. 초안 수용 테스트가 통과했다는 뜻이 아니다.\n' +
  (failures.length ? '\n## 실패\n\n' + failures.map(f => `- ${f}`).join('\n') + '\n' : '');
fs.writeFileSync(path.join(pack, 'VALIDATION.md'), report);
console.log(report);
if (failures.length) process.exitCode = 1;
