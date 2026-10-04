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
const parserProposal = manifest.accepted_proposals?.find(p => p.id === 'P01');
assert(parserProposal?.approval_date === '2026-10-04' && parserProposal?.grammar === 'GRAMMAR_STAGE_A.ebnf', 'Missing P01 approval ledger');
assert(fs.readFileSync(path.join(pack, 'PARSER_STAGE_A_PROPOSAL.md'), 'utf8').includes('Accepted / 2026-10-04 사용자 승인'), 'Invalid P01 status');
checks.push('P01 Stage A Parser 승인 범위/날짜와 전용 EBNF 기록');

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
