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
validateGrammar('GRAMMAR_STAGE_B_MODULE.ebnf');
validateGrammar('GRAMMAR_STAGE_B_STRUCT.ebnf');
validateGrammar('GRAMMAR_STAGE_B_TUPLE.ebnf');
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

const moduleProposal = manifest.accepted_proposals?.find(p => p.id === 'P11');
assert(moduleProposal?.approval_date === '2026-10-05' && moduleProposal?.implementation_verified === true
  && moduleProposal?.document === 'MODULE_STAGE_B_PROPOSAL.md'
  && moduleProposal?.grammar === 'GRAMMAR_STAGE_B_MODULE.ebnf'
  && moduleProposal?.grammar_change === true, 'Missing P11 approved module ledger');
assert(!manifest.draft_proposals?.some(p => p.id === 'P11'), 'Accepted P11 still in draft ledger');
assert(fs.readFileSync(path.join(pack, 'MODULE_STAGE_B_PROPOSAL.md'), 'utf8').includes('Accepted / 2026-10-05 사용자 승인'), 'Invalid P11 approval status');
const moduleGrammar = fs.readFileSync(path.join(pack, 'GRAMMAR_STAGE_B_MODULE.ebnf'), 'utf8');
assert(moduleGrammar.includes('Accepted by the user on 2026-10-05'), 'Invalid P11 grammar approval status');
const productions = text => new Map([...text.replace(/\(\*[\s\S]*?\*\)/g, '').matchAll(/^\s*([a-z][a-z_0-9]*)\s*=([\s\S]*?);/gm)]
  .map(m => [m[1], m[2].replace(/\s+/g, '')]));
const castProductions = productions(castGrammar), moduleProductions = productions(moduleGrammar);
for (const [name, value] of castProductions) {
  if (name !== 'program') assert(moduleProductions.get(name) === value, `P11 changes existing ${name}`);
}
assert(moduleProductions.size === castProductions.size + 3, 'P11 grammar adds more than three productions');
assert(moduleProductions.get('program') === '{"END"|import_decl|[visibility],(function_decl|global_const_decl)},"EOF"', 'P11 invalid top-level grammar');
assert(moduleProductions.get('import_decl') === '"use",qualified_name,["as","IDENT"],end', 'P11 invalid import grammar');
assert(moduleProductions.get('qualified_name') === '"IDENT",{"::","IDENT"}', 'P11 invalid import path grammar');
assert(moduleProductions.get('visibility') === '"public"|"internal"|"private"', 'P11 invalid visibility grammar');
const moduleFixtureRoot = path.join(pack, 'module-proposal-fixtures');
const moduleFixture = JSON.parse(fs.readFileSync(path.join(moduleFixtureRoot, 'expected.json'), 'utf8'));
assert(moduleFixture.proposal === 'P11' && moduleFixture.status === 'Accepted' && moduleFixture.implementation_verified === true, 'P11 fixture acceptance status missing');
assert(moduleFixture.entry === 'main.nova' && moduleFixture.source_root === '.'
  && JSON.stringify(moduleFixture.reachable_modules) === '["main","math"]'
  && moduleFixture.import_cycle_permitted === true, 'P11 fixture graph mismatch');
assert(moduleFixture.validated_result?.check_exit === 0 && moduleFixture.validated_result?.native_exit === 0
  && moduleFixture.validated_result?.stdout === 'value=42\n', 'P11 fixture expected result mismatch');
for (const name of ['main.nova', 'math.nova']) {
  const bytes = fs.readFileSync(path.join(moduleFixtureRoot, name));
  const text = new TextDecoder('utf-8', { fatal: true }).decode(bytes);
  assert(text.endsWith('\n') && !text.includes('\r'), `P11 fixture encoding/newline ${name}`);
}
assert(fs.readFileSync(path.join(moduleFixtureRoot, 'main.nova'), 'utf8').includes('use math::add as plus')
  && fs.readFileSync(path.join(moduleFixtureRoot, 'math.nova'), 'utf8').includes('use main::twice'), 'P11 missing fixture cyclic import/alias');
checks.push('P11 Module 승인 subset/날짜·accepted ledger·기존 P10 production 보존·34-production EBNF와 수용 2-file fixture 데이터 (컴파일 실행 아님)');

const structProposal = manifest.accepted_proposals?.find(p => p.id === 'P12');
assert(structProposal?.status === 'Accepted' && structProposal?.implementation_verified === true
  && structProposal?.grammar_change === true && structProposal?.document === 'STRUCT_STAGE_B_PROPOSAL.md'
  && structProposal?.grammar === 'GRAMMAR_STAGE_B_STRUCT.ebnf' && structProposal?.approval_date === '2026-10-05',
  'P12 missing approved/implemented ledger');
assert(!manifest.draft_proposals?.some(p => p.id === 'P12'), 'P12 must not remain Draft after user approval');
assert(fs.readFileSync(path.join(pack, structProposal?.document ?? 'STRUCT_STAGE_B_PROPOSAL.md'), 'utf8')
  .includes('Accepted / 구현 완료'), 'Invalid P12 proposal status');
const structGrammar = fs.readFileSync(path.join(pack, 'GRAMMAR_STAGE_B_STRUCT.ebnf'), 'utf8');
assert(structGrammar.includes('Accepted by user 2026-10-05'), 'Invalid P12 grammar status');
const structProductions = productions(structGrammar);
for (const [name, value] of moduleProductions) {
  if (!['program', 'assignment', 'postfix_expr'].includes(name)) {
    assert(structProductions.get(name) === value, `P12 changes existing ${name}`);
  }
}
assert(structProductions.size === moduleProductions.size + 3, 'P12 must add exactly three productions');
assert(structProductions.get('program') === '{"END"|import_decl|[visibility],(function_decl|global_const_decl|struct_decl)},"EOF"', 'P12 invalid top-level grammar');
assert(structProductions.get('assignment') === 'field_path,"=",expression', 'P12 invalid assignment grammar');
assert(structProductions.get('postfix_expr') === 'primary_expr,{"(",[arguments],")"|"as",type|".","IDENT"}', 'P12 invalid projection grammar');
assert(structProductions.get('struct_decl') === '"struct","IDENT","{",{"END"|field_decl},"}"', 'P12 invalid struct grammar');
assert(structProductions.get('field_decl') === '[visibility],("let"|"var"),"IDENT",":",type,end', 'P12 invalid field grammar');
assert(structProductions.get('field_path') === '"IDENT",{".","IDENT"}', 'P12 invalid field target grammar');
const structFixtureRoot = path.join(pack, 'struct-proposal-fixtures');
const structFixture = JSON.parse(fs.readFileSync(path.join(structFixtureRoot, 'expected.json'), 'utf8'));
assert(structFixture.proposal === 'P12' && structFixture.status === 'Accepted'
  && structFixture.implementation_verified === true && !structFixture.proposed_result, 'P12 fixture acceptance status missing');
assert(structFixture.entry === 'main.nova' && structFixture.source_root === '.'
  && JSON.stringify(structFixture.reachable_modules) === '["main","geometry"]', 'P12 fixture graph mismatch');
assert(structFixture.validated_result?.check_exit === 0 && structFixture.validated_result?.native_exit === 0
  && structFixture.validated_result?.stdout === 'original=21, snapshot=20, shifted=21, tag=🙂\n', 'P12 validated result mismatch');
assert(structFixture.negative_cases?.length === 10
  && new Set(structFixture.negative_cases.map(c => c.name)).size === 10, 'P12 negative fixture set mismatch');
for (const name of ['main.nova', 'geometry.nova', ...structFixture.negative_cases.map(c => c.source)]) {
  assert(path.basename(name) === name && name.endsWith('.nova'), `P12 fixture invalid filename ${name}`);
  const bytes = fs.readFileSync(path.join(structFixtureRoot, name));
  const text = new TextDecoder('utf-8', { fatal: true }).decode(bytes);
  assert(text.endsWith('\n') && !text.includes('\r'), `P12 fixture encoding/newline ${name}`);
}
for (const c of structFixture.negative_cases) {
  assert(knownCodes.has(c.expected_diagnostic), `P12 unknown diagnostic ${c.name}`);
  const bytes = fs.readFileSync(path.join(structFixtureRoot, c.source));
  const text = bytes.toString('utf8');
  const boundaries = new Set([0]);
  let offset = 0;
  for (const char of text) { offset += Buffer.byteLength(char); boundaries.add(offset); }
  const { start, end } = c.primary;
  assert(start >= 0 && start < end && end <= bytes.length && boundaries.has(start) && boundaries.has(end), `P12 invalid byte span ${c.name}`);
  assert(bytes.subarray(start, end).toString('utf8') === c.primary_text, `P12 unexpected span text ${c.name}`);
}
assert(fs.readFileSync(path.join(structFixtureRoot, 'main.nova'), 'utf8').includes('use geometry::Pair as P')
  && fs.readFileSync(path.join(structFixtureRoot, 'main.nova'), 'utf8').includes('original.pair.x = original.pair.x + 1')
  && fs.readFileSync(path.join(structFixtureRoot, 'geometry.nova'), 'utf8').includes('public struct Pair'), 'P12 missing Copy/import fixture data');
checks.push('P12 Accepted/승인·구현 ledger·P11의 세 production 변경/세 production 추가·37-production EBNF·두 파일/부정 10사례 UTF-8 Span·검증 결과 데이터 (컴파일 실행 아님)');

const tupleProposal = manifest.accepted_proposals?.find(p => p.id === 'P13');
assert(tupleProposal?.status === 'Accepted' && tupleProposal?.implementation_verified === true
  && tupleProposal?.grammar_change === true && tupleProposal?.approval_date === '2026-10-07'
  && tupleProposal?.document === 'TUPLE_STAGE_B_PROPOSAL.md'
  && tupleProposal?.grammar === 'GRAMMAR_STAGE_B_TUPLE.ebnf', 'P13 missing approved/implemented ledger');
assert(!manifest.draft_proposals?.some(p => p.id === 'P13'), 'P13 must not remain Draft after user approval');
assert(fs.readFileSync(path.join(pack, 'TUPLE_STAGE_B_PROPOSAL.md'), 'utf8')
  .includes('Accepted / 구현 완료'), 'P13 invalid proposal status');
const tupleGrammar = fs.readFileSync(path.join(pack, 'GRAMMAR_STAGE_B_TUPLE.ebnf'), 'utf8');
assert(tupleGrammar.includes('Accepted by user 2026-10-07'), 'P13 invalid grammar status');
const tupleProductions = productions(tupleGrammar);
for (const [name, value] of structProductions) {
  if (!['type', 'primary_expr', 'postfix_expr', 'field_path'].includes(name)) {
    assert(tupleProductions.get(name) === value, `P13 changes existing ${name}`);
  }
}
assert(tupleProductions.size === structProductions.size + 3, 'P13 must add exactly three productions');
assert(tupleProductions.get('type') === '"IDENT"|"(",")"|tuple_type', 'P13 invalid type grammar');
assert(tupleProductions.get('primary_expr') === '"INT"|"FLOAT"|"CHAR"|"STRING"|"true"|"false"|"IDENT"|"(",")"|"(",expression,")"|tuple_expr|interpolated_string', 'P13 invalid primary grammar');
assert(tupleProductions.get('postfix_expr') === 'primary_expr,{"(",[arguments],")"|"as",type|".",("IDENT"|tuple_index)}', 'P13 invalid projection grammar');
assert(tupleProductions.get('field_path') === '"IDENT",{".",("IDENT"|tuple_index)}', 'P13 invalid target grammar');
assert(tupleProductions.get('tuple_type') === '"(",type,",",[type,{",",type},[","]],")"', 'P13 invalid tuple type grammar');
assert(tupleProductions.get('tuple_expr') === '"(",expression,",",[expression,{",",expression},[","]],")"', 'P13 invalid tuple expression grammar');
assert(tupleProductions.get('tuple_index')?.includes('canonicalASCIIdecimal0or[1-9][0-9]*')
  && tupleProductions.get('tuple_index')?.includes('originalbytesubspans'), 'P13 missing selector lexical contract');
const tupleFixtureRoot = path.join(pack, 'tuple-proposal-fixtures');
const tupleFixture = JSON.parse(fs.readFileSync(path.join(tupleFixtureRoot, 'expected.json'), 'utf8'));
assert(tupleFixture.proposal === 'P13' && tupleFixture.status === 'Accepted'
  && tupleFixture.implementation_verified === true && !tupleFixture.proposed_result, 'P13 fixture acceptance status missing');
assert(tupleFixture.entry === 'main.nova' && tupleFixture.source_root === '.'
  && JSON.stringify(tupleFixture.reachable_modules) === '["main","tuples"]', 'P13 fixture graph mismatch');
assert(tupleFixture.validated_result?.check_exit === 0 && tupleFixture.validated_result?.native_exit === 0
  && tupleFixture.validated_result?.stdout === 'original=21, snapshot=20, shifted=21, one=7, tag=🙂\n', 'P13 validated result mismatch');
assert(tupleFixture.negative_cases?.length === 10
  && new Set(tupleFixture.negative_cases.map(c => c.name)).size === 10, 'P13 negative fixture set mismatch');
for (const name of ['main.nova', 'tuples.nova', ...tupleFixture.negative_cases.map(c => c.source)]) {
  assert(path.basename(name) === name && name.endsWith('.nova'), `P13 invalid filename ${name}`);
  const bytes = fs.readFileSync(path.join(tupleFixtureRoot, name));
  const text = new TextDecoder('utf-8', { fatal: true }).decode(bytes);
  assert(text.endsWith('\n') && !text.includes('\r'), `P13 fixture encoding/newline ${name}`);
}
for (const c of tupleFixture.negative_cases) {
  assert(knownCodes.has(c.expected_diagnostic), `P13 unknown diagnostic ${c.name}`);
  const bytes = fs.readFileSync(path.join(tupleFixtureRoot, c.source));
  const boundaries = new Set([0]);
  let offset = 0;
  for (const char of bytes.toString('utf8')) { offset += Buffer.byteLength(char); boundaries.add(offset); }
  const { start, end } = c.primary;
  assert(start >= 0 && start < end && end <= bytes.length && boundaries.has(start) && boundaries.has(end), `P13 invalid byte span ${c.name}`);
  assert(bytes.subarray(start, end).toString('utf8') === c.primary_text, `P13 unexpected span text ${c.name}`);
}
assert(fs.readFileSync(path.join(tupleFixtureRoot, 'main.nova'), 'utf8').includes('original.items.0.1 = original.items.0.1 + 1')
  && fs.readFileSync(path.join(tupleFixtureRoot, 'main.nova'), 'utf8').includes('let one: (int8,) = (7,)')
  && fs.readFileSync(path.join(tupleFixtureRoot, 'tuples.nova'), 'utf8').includes('public const START: ((int, int), bool)'), 'P13 missing nested/one-tuple/const fixture');
checks.push('P13 Accepted/승인·구현 ledger·P12 네 production 확장/세 production 추가·40-production EBNF·두 파일/부정 10사례 UTF-8 Span·검증 결과 데이터 (컴파일 실행 아님)');

const enumProposal = manifest.accepted_proposals?.find(p => p.id === 'P14');
assert(enumProposal?.status === 'Accepted' && enumProposal?.implementation_verified === true
  && enumProposal?.approval_date === '2026-10-07' && enumProposal?.grammar_change === true
  && enumProposal?.document === 'ENUM_STAGE_B_PROPOSAL.md'
  && enumProposal?.grammar === 'GRAMMAR_STAGE_B_ENUM.ebnf', 'P14 missing approved/implemented ledger');
assert(!manifest.draft_proposals?.some(p => p.id === 'P14'), 'Accepted P14 still in Draft ledger');
assert(fs.readFileSync(path.join(pack, 'ENUM_STAGE_B_PROPOSAL.md'), 'utf8')
  .includes('Accepted / 구현 완료'), 'P14 invalid proposal status');
validateGrammar('GRAMMAR_STAGE_B_ENUM.ebnf');
const enumGrammar = fs.readFileSync(path.join(pack, 'GRAMMAR_STAGE_B_ENUM.ebnf'), 'utf8');
assert(enumGrammar.includes('Accepted by user 2026-10-07'), 'P14 invalid grammar status');
const enumProductions = productions(enumGrammar);
for (const [name, value] of tupleProductions) {
  if (!['program', 'statement', 'primary_expr'].includes(name)) {
    assert(enumProductions.get(name) === value, `P14 changes existing ${name}`);
  }
}
assert(enumProductions.size === tupleProductions.size + 8, 'P14 must add exactly eight productions');
assert(enumProductions.get('program') === '{"END"|import_decl|[visibility],(function_decl|global_const_decl|struct_decl|enum_decl)},"EOF"', 'P14 invalid top-level grammar');
assert(enumProductions.get('statement') === 'if_stmt|while_stmt|match_stmt|simple_statement', 'P14 invalid statement grammar');
assert(enumProductions.get('primary_expr') === tupleProductions.get('primary_expr').replace('|interpolated_string', '|variant_path|interpolated_string'), 'P14 changes primary beyond variant path');
const newEnumProductions = {
  enum_decl: '"enum","IDENT","{",{"END"|variant_decl},"}"',
  variant_decl: '"IDENT",["(",payload_types,")"],end',
  payload_types: 'type,{",",type},[","]',
  variant_path: '"IDENT","::","IDENT"',
  match_stmt: '"match",expression,"{",{"END"|match_arm},"}"',
  match_arm: 'pattern,"=>",block,(end|",")',
  pattern: 'variant_path,["(",pattern_arguments,")"]|"true"|"false"|"_"',
  pattern_arguments: '"IDENT",{",","IDENT"},[","]',
};
for (const [name, value] of Object.entries(newEnumProductions)) {
  assert(enumProductions.get(name) === value, `P14 invalid ${name} grammar`);
}
const enumFixtureRoot = path.join(pack, 'enum-proposal-fixtures');
const enumFixture = JSON.parse(fs.readFileSync(path.join(enumFixtureRoot, 'expected.json'), 'utf8'));
assert(enumFixture.proposal === 'P14' && enumFixture.status === 'Accepted'
  && enumFixture.implementation_verified === true && !enumFixture.proposed_result, 'P14 fixture acceptance status missing');
assert(enumFixture.entry === 'main.nova' && enumFixture.source_root === '.'
  && JSON.stringify(enumFixture.reachable_modules) === '["main","events"]', 'P14 fixture graph mismatch');
assert(enumFixture.validated_result?.check_exit === 0 && enumFixture.validated_result?.native_exit === 0
  && enumFixture.validated_result?.stderr === ''
  && enumFixture.validated_result?.stdout === 'make\nsum=30, code=7, flag=true\noriginal=empty\nok\n', 'P14 validated result mismatch');
assert(enumFixture.negative_cases?.length === 16
  && new Set(enumFixture.negative_cases.map(c => c.name)).size === 16, 'P14 negative fixture set mismatch');
for (const name of ['main.nova', 'events.nova', ...enumFixture.negative_cases.map(c => c.source)]) {
  assert(path.basename(name) === name && name.endsWith('.nova'), `P14 invalid filename ${name}`);
  const bytes = fs.readFileSync(path.join(enumFixtureRoot, name));
  const text = new TextDecoder('utf-8', { fatal: true }).decode(bytes);
  assert(text.endsWith('\n') && !text.includes('\r'), `P14 fixture encoding/newline ${name}`);
}
for (const c of enumFixture.negative_cases) {
  assert(knownCodes.has(c.expected_diagnostic), `P14 unknown diagnostic ${c.name}`);
  const bytes = fs.readFileSync(path.join(enumFixtureRoot, c.source));
  const boundaries = new Set([0]);
  let offset = 0;
  for (const char of bytes.toString('utf8')) { offset += Buffer.byteLength(char); boundaries.add(offset); }
  const { start, end } = c.primary;
  assert(start >= 0 && start < end && end <= bytes.length && boundaries.has(start) && boundaries.has(end), `P14 invalid byte span ${c.name}`);
  assert(bytes.subarray(start, end).toString('utf8') === c.primary_text, `P14 unexpected span text ${c.name}`);
}
assert(fs.readFileSync(path.join(enumFixtureRoot, 'main.nova'), 'utf8').includes('use events::Event as E')
  && fs.readFileSync(path.join(enumFixtureRoot, 'main.nova'), 'utf8').includes('E::Data(pair, meta)')
  && fs.readFileSync(path.join(enumFixtureRoot, 'events.nova'), 'utf8').includes('public const BASE: Event'), 'P14 missing Copy/import/const fixture data');
checks.push('P14 Accepted/승인·구현 ledger·P13 세 production 확장/여덟 추가·48-production EBNF·두 파일/부정 16사례 UTF-8 Span·검증 기대값 데이터 (문서 validator는 컴파일 실행 아님)');

const optionResultProposal = manifest.accepted_proposals?.find(p => p.id === 'P15');
assert(optionResultProposal?.status === 'Accepted' && optionResultProposal?.implementation_verified === true
  && optionResultProposal?.approval_date === '2026-10-07' && optionResultProposal?.grammar_change === true
  && optionResultProposal?.document === 'OPTION_RESULT_STAGE_B_PROPOSAL.md'
  && optionResultProposal?.grammar === 'GRAMMAR_STAGE_B_OPTION_RESULT.ebnf', 'P15 missing accepted/implemented ledger');
assert(!manifest.draft_proposals?.some(p => p.id === 'P15'), 'P15 must not remain draft after approval');
assert(fs.readFileSync(path.join(pack, 'OPTION_RESULT_STAGE_B_PROPOSAL.md'), 'utf8')
  .includes('Accepted / 사용자 승인 완료 / 구현 완료'), 'P15 invalid proposal status');
validateGrammar('GRAMMAR_STAGE_B_OPTION_RESULT.ebnf');
const optionResultGrammar = fs.readFileSync(path.join(pack, 'GRAMMAR_STAGE_B_OPTION_RESULT.ebnf'), 'utf8');
assert(optionResultGrammar.includes('Accepted, user approved 2026-10-07'), 'P15 invalid grammar status');
const optionResultProductions = productions(optionResultGrammar);
for (const [name, value] of enumProductions) {
  if (!['type', 'primary_expr', 'pattern'].includes(name)) {
    assert(optionResultProductions.get(name) === value, `P15 changes existing ${name}`);
  }
}
assert(optionResultProductions.size === enumProductions.size + 3, 'P15 must add exactly three productions');
assert(optionResultProductions.get('type') === 'type_atom,{"?"}', 'P15 invalid nullable type grammar');
assert(optionResultProductions.get('primary_expr') === enumProductions.get('primary_expr').replace('|interpolated_string', '|"none"|interpolated_string'), 'P15 changes primary beyond none');
assert(optionResultProductions.get('pattern') === enumProductions.get('pattern').replace('|"_"', '|"none"|"_"'), 'P15 changes pattern beyond none');
assert(optionResultProductions.get('type_atom') === '"IDENT",[type_arguments]|"(",")"|tuple_type', 'P15 invalid type atom');
assert(optionResultProductions.get('type_arguments') === '"<",type,{",",type},[","],type_close', 'P15 invalid type arguments');
assert(optionResultProductions.get('type_close')?.includes('>=tokensplitsonlyintypeargumentcontextpreservingone-byte>and=subspans'), 'P15 missing type-close byte subspan contract');
const optionResultRoot = path.join(pack, 'option-result-proposal-fixtures');
const optionResultFixture = JSON.parse(fs.readFileSync(path.join(optionResultRoot, 'expected.json'), 'utf8'));
assert(optionResultFixture.proposal === 'P15' && optionResultFixture.status === 'Accepted'
  && optionResultFixture.implementation_verified === true && optionResultFixture.validated_result, 'P15 fixture must be verified after approval');
assert(optionResultFixture.entry === 'main.nova' && optionResultFixture.source_root === '.'
  && JSON.stringify(optionResultFixture.reachable_modules) === '["main","values"]', 'P15 fixture graph mismatch');
assert(optionResultFixture.validated_result?.check_exit === 0 && optionResultFixture.validated_result?.native_exit === 0
  && optionResultFixture.validated_result?.stderr === ''
  && optionResultFixture.validated_result?.stdout === 'maybe\nsome=7\noriginal=none\nsuccess=8, flag=true\nerror=-1\nnested=none\nprivate=9\nunit=success\n', 'P15 validated result mismatch');
assert(optionResultFixture.negative_cases?.length === 21
  && new Set(optionResultFixture.negative_cases.map(c => c.name)).size === 21, 'P15 negative fixture set mismatch');
for (const name of ['main.nova', 'values.nova', ...optionResultFixture.negative_cases.map(c => c.source)]) {
  assert(path.basename(name) === name && name.endsWith('.nova'), `P15 invalid filename ${name}`);
  const bytes = fs.readFileSync(path.join(optionResultRoot, name));
  const text = new TextDecoder('utf-8', { fatal: true }).decode(bytes);
  assert(text.endsWith('\n') && !text.includes('\r'), `P15 fixture encoding/newline ${name}`);
}
for (const c of optionResultFixture.negative_cases) {
  assert(knownCodes.has(c.expected_diagnostic), `P15 unknown diagnostic ${c.name}`);
  const bytes = fs.readFileSync(path.join(optionResultRoot, c.source));
  const boundaries = new Set([0]);
  let offset = 0;
  for (const char of bytes.toString('utf8')) { offset += Buffer.byteLength(char); boundaries.add(offset); }
  const { start, end } = c.primary;
  assert(start >= 0 && start < end && end <= bytes.length && boundaries.has(start) && boundaries.has(end), `P15 invalid byte span ${c.name}`);
  assert(bytes.subarray(start, end).toString('utf8') === c.primary_text, `P15 unexpected span text ${c.name}`);
}
assert(fs.readFileSync(path.join(optionResultRoot, 'main.nova'), 'utf8').includes('use values::Failure as F')
  && fs.readFileSync(path.join(optionResultRoot, 'main.nova'), 'utf8').includes('match snapshot')
  && fs.readFileSync(path.join(optionResultRoot, 'values.nova'), 'utf8').includes('public const NESTED:Option<Option<int8>>=Option::Some(none)')
  && fs.readFileSync(path.join(optionResultRoot, 'values.nova'), 'utf8').includes('private struct Hidden'), 'P15 missing Copy/import/const/private fixture data');
checks.push('P15 Accepted/승인·구현 ledger·P14 세 production 확장/세 추가·51-production EBNF·두 파일/부정 21사례 UTF-8 Span·검증 기대값 데이터 (문서 validator는 컴파일 실행 아님)');

const tryProposal = manifest.accepted_proposals?.find(p => p.id === 'P16');
assert(tryProposal?.status === 'Accepted' && tryProposal?.implementation_verified === true
  && tryProposal?.approval_date === '2026-10-07' && tryProposal?.grammar_change === true
  && tryProposal?.document === 'TRY_STAGE_B_PROPOSAL.md'
  && tryProposal?.grammar === 'GRAMMAR_STAGE_B_TRY.ebnf', 'P16 missing accepted/implemented ledger');
assert(!manifest.draft_proposals?.some(p => p.id === 'P16'), 'Accepted P16 still in Draft ledger');
assert(fs.readFileSync(path.join(pack, 'TRY_STAGE_B_PROPOSAL.md'), 'utf8')
  .includes('Accepted / 사용자 승인 완료 / 구현 완료'), 'P16 invalid proposal status');
validateGrammar('GRAMMAR_STAGE_B_TRY.ebnf');
const tryGrammar = fs.readFileSync(path.join(pack, 'GRAMMAR_STAGE_B_TRY.ebnf'), 'utf8');
assert(tryGrammar.includes('Accepted, user approved 2026-10-07'), 'P16 invalid grammar status');
const tryProductions = productions(tryGrammar);
assert(tryProductions.size === 51 && tryProductions.size === optionResultProductions.size,
  'P16 must retain all 51 productions');
for (const [name, value] of optionResultProductions) {
  if (name !== 'prefix_expr') assert(tryProductions.get(name) === value, `P16 changes existing ${name}`);
}
assert(tryProductions.get('prefix_expr') === '("+"|"-"|"!"|"try"),prefix_expr|postfix_expr',
  'P16 invalid prefix grammar');
const tryRoot = path.join(pack, 'try-proposal-fixtures');
const tryFixture = JSON.parse(fs.readFileSync(path.join(tryRoot, 'expected.json'), 'utf8'));
assert(tryFixture.proposal === 'P16' && tryFixture.status === 'Accepted'
  && tryFixture.implementation_verified === true && !tryFixture.proposed_result,
  'P16 fixture must record implementation');
assert(tryFixture.entry === 'main.nova' && tryFixture.source_root === '.'
  && JSON.stringify(tryFixture.reachable_modules) === '["main","effects"]', 'P16 fixture graph mismatch');
const tryExpectedStdout = [
  'start', 'leaf', 'second', 'after', 'ok=8', 'start', 'leaf', 'error=-1',
  'leaf', 'nested=7', 'leaf', 'nested-error=-1', 'ping', 'unit=success',
  'snapshot=9', 'short=false', 'leaf', 'leaf', 'loop=7',
].join('\n') + '\n';
assert(tryFixture.validated_result?.check_exit === 0 && tryFixture.validated_result?.native_exit === 0
  && tryFixture.validated_result?.stdout === tryExpectedStdout
  && tryFixture.validated_result?.stderr === '', 'P16 validated output mismatch');
assert(tryFixture.negative_cases?.length === 18
  && new Set(tryFixture.negative_cases.map(c => c.name)).size === 18
  && new Set(tryFixture.negative_cases.map(c => c.source)).size === 18, 'P16 negative fixture set mismatch');
for (const name of ['main.nova', 'effects.nova', ...tryFixture.negative_cases.map(c => c.source)]) {
  assert(path.basename(name) === name && name.endsWith('.nova'), `P16 invalid filename ${name}`);
  const bytes = fs.readFileSync(path.join(tryRoot, name));
  const text = new TextDecoder('utf-8', { fatal: true }).decode(bytes);
  assert(text.endsWith('\n') && !text.includes('\r'), `P16 fixture encoding/newline ${name}`);
}
for (const c of tryFixture.negative_cases) {
  assert(knownCodes.has(c.expected_diagnostic), `P16 unknown diagnostic ${c.name}`);
  const bytes = fs.readFileSync(path.join(tryRoot, c.source));
  const boundaries = new Set([0]);
  let offset = 0;
  for (const char of bytes.toString('utf8')) { offset += Buffer.byteLength(char); boundaries.add(offset); }
  const { start, end } = c.primary;
  assert(start >= 0 && start < end && end <= bytes.length && boundaries.has(start) && boundaries.has(end),
    `P16 invalid byte span ${c.name}`);
  assert(bytes.subarray(start, end).toString('utf8') === c.primary_text, `P16 unexpected span text ${c.name}`);
  for (const code of c.forbidden_diagnostics ?? []) {
    assert(knownCodes.has(code) && code !== c.expected_diagnostic, `P16 invalid cascade expectation ${c.name}`);
  }
}
for (const name of ['global_const', 'local_const', 'skipped_const_rhs']) {
  const c = tryFixture.negative_cases.find(c => c.name === name);
  assert(c?.expected_diagnostic === 'N3201' && c?.primary_text === 'try'
    && c?.forbidden_diagnostics?.includes('N3002'), `P16 missing const precedence ${name}`);
}
const tryMain = fs.readFileSync(path.join(tryRoot, 'main.nova'), 'utf8');
const tryEffects = fs.readFileSync(path.join(tryRoot, 'effects.nova'), 'utf8');
assert(tryMain.includes('use effects::Failure as F')
  && tryMain.includes('combine(try leaf(flag),second())')
  && tryMain.includes('try try nested(flag)') && tryMain.includes('false && try truth()')
  && tryMain.includes('current=Result::Error(F::Bad(-5))')
  && tryEffects.includes('public func leaf') && tryEffects.includes('print("second")'),
  'P16 missing effect/snapshot/nested/short-circuit proposal data');
checks.push('P16 Accepted/승인·구현 ledger·P15 prefix_expr 한 production 확장·51-production EBNF·두 파일/부정 18사례 UTF-8 Span·const/cascade·검증 19줄 출력 데이터 (문서 validator는 Compiler/Native 실행 아님)');

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
