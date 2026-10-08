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

const namedProposal = manifest.accepted_proposals?.find(p => p.id === 'P17');
assert(namedProposal?.status === 'Accepted' && namedProposal?.implementation_verified === true
  && namedProposal?.approval_date === '2026-10-07'
  && namedProposal?.document === 'NAMED_ARGUMENTS_STAGE_B_PROPOSAL.md'
  && namedProposal?.grammar === 'GRAMMAR_STAGE_B_NAMED_ARGUMENTS.ebnf'
  && !manifest.draft_proposals?.some(p => p.id === 'P17'), 'P17 invalid Accepted ledger');
assert(fs.readFileSync(path.join(pack, 'NAMED_ARGUMENTS_STAGE_B_PROPOSAL.md'), 'utf8')
  .includes('Accepted / 사용자 승인 / 구현 완료'), 'P17 invalid proposal status');
validateGrammar('GRAMMAR_STAGE_B_NAMED_ARGUMENTS.ebnf');
const namedGrammar = fs.readFileSync(path.join(pack, 'GRAMMAR_STAGE_B_NAMED_ARGUMENTS.ebnf'), 'utf8');
assert(namedGrammar.includes('Accepted by user on 2026-10-07'), 'P17 invalid grammar status');
const namedProductions = productions(namedGrammar);
assert(namedProductions.size === 52 && tryProductions.size === 51, 'P17 must add exactly one production');
for (const [name, value] of tryProductions) {
  if (name !== 'arguments') assert(namedProductions.get(name) === value, `P17 changes existing ${name}`);
}
assert(namedProductions.get('arguments') === 'argument,{",",argument},[","]'
  && namedProductions.get('argument') === '["IDENT",":"],expression', 'P17 invalid argument grammar');
const namedRoot = path.join(pack, 'named-arguments-proposal-fixtures');
const namedFixture = JSON.parse(fs.readFileSync(path.join(namedRoot, 'expected.json'), 'utf8'));
assert(namedFixture.proposal === 'P17' && namedFixture.status === 'Accepted'
  && namedFixture.implementation_verified === true && namedFixture.approval_date === '2026-10-07',
  'P17 invalid verified fixture status');
assert(namedFixture.entry === 'main.nova' && namedFixture.source_root === '.'
  && JSON.stringify(namedFixture.reachable_modules) === '["main","helpers"]', 'P17 fixture graph mismatch');
const namedOutput = 'right\nleft\nreverse=702\npositional\nnamed\nmixed=102\nsecond\nfirst\ntext=first/second\nunit=3\nunicode=304\nminimum=-12500\ncopy=4/5\noption=7\nleaf\nlater\nafter\nsuccess=102\nleaf\nerror=-1\nbefore\nleaf\nprior-error=-1\nshort=false\n';
assert(namedFixture.validated_result?.check_exit === 0 && namedFixture.validated_result?.native_exit === 0
  && namedFixture.validated_result?.stdout === namedOutput && namedFixture.validated_result?.stderr === '',
  'P17 verified output mismatch');
assert(namedFixture.negative_cases?.length === 16
  && new Set(namedFixture.negative_cases.map(c => c.name)).size === 16
  && new Set(namedFixture.negative_cases.map(c => c.source)).size === 16, 'P17 negative set mismatch');
assert(namedFixture.positive_cases?.length === 2
  && JSON.stringify(namedFixture.positive_cases.map(c => c.source))
    === '["function_print_shadow.nova","forward_recursive_grouped.nova"]', 'P17 positive set mismatch');
for (const c of namedFixture.positive_cases) {
  assert(c.validated_result?.check_exit === 0 && c.validated_result?.native_exit === 0
    && c.validated_result?.stdout === '' && c.validated_result?.stderr === '',
    `P17 invalid positive validated result ${c.source}`);
}
for (const name of [namedFixture.entry, 'helpers.nova', ...namedFixture.positive_cases.map(c => c.source),
  ...namedFixture.negative_cases.map(c => c.source)]) {
  assert(path.basename(name) === name && name.endsWith('.nova'), `P17 invalid filename ${name}`);
  const text = fs.readFileSync(path.join(namedRoot, name), 'utf8');
  assert(text.endsWith('\n') && !text.includes('\r'), `P17 fixture encoding/newline ${name}`);
}
for (const c of namedFixture.negative_cases) {
  assert(knownCodes.has(c.expected_diagnostic), `P17 unknown diagnostic ${c.name}`);
  const bytes = fs.readFileSync(path.join(namedRoot, c.source));
  const boundaries = new Set([0]);
  let offset = 0;
  for (const scalar of bytes.toString('utf8')) { offset += Buffer.byteLength(scalar); boundaries.add(offset); }
  const { start, end } = c.primary;
  assert(start < end && start >= 0 && end <= bytes.length && boundaries.has(start) && boundaries.has(end),
    `P17 invalid byte span ${c.name}`);
  assert(bytes.subarray(start, end).toString('utf8') === c.primary_text, `P17 unexpected span text ${c.name}`);
  for (const code of c.forbidden_diagnostics ?? []) {
    assert(knownCodes.has(code) && code !== c.expected_diagnostic, `P17 invalid cascade expectation ${c.name}`);
  }
}
const namedUndefined = namedFixture.negative_cases.find(c => c.name === 'unresolved_callee');
assert(namedUndefined?.forbidden_diagnostics?.includes('N2201')
  && namedUndefined?.forbidden_diagnostics?.includes('N2101'), 'P17 missing ErrorType cascade data');
const namedMain = fs.readFileSync(path.join(namedRoot, 'main.nova'), 'utf8');
assert(namedMain.includes('use helpers::combine as joined')
  && namedMain.includes('right:try fetch(ok:flag),left:traced(value:1,tag:"later")')
  && namedMain.includes('left:traced(value:1,tag:"before"),right:try fetch(ok:false)')
  && namedMain.includes('korean(뒤:4,앞:3)') && namedMain.includes('right:300,left:-128')
  && namedMain.includes('optional(value:Option::Some(7))') && namedMain.includes('false &&'),
  'P17 missing mapping/effect/try/Unicode/context proposal data');
const namedPrivate = namedFixture.negative_cases.find(c => c.name === 'private_import');
assert(namedPrivate?.primary?.start === 0 && namedPrivate?.primary?.end === 20
  && namedPrivate?.primary_text === 'use helpers::hidden\n', 'P17 must preserve P11 whole-import N2004 Span');
checks.push('P17 Accepted/승인·구현 ledger·P16 arguments 확장/argument 추가·52-production EBNF·두 파일/정상 2·부정 16사례 UTF-8 Span·cascade·검증 24줄 출력 metadata (Compiler/Native 실행 아님)');

const defaultProposal = manifest.accepted_proposals?.find(p => p.id === 'P18');
assert(defaultProposal?.status === 'Accepted' && defaultProposal?.implementation_verified === true
  && defaultProposal?.document === 'DEFAULT_ARGUMENTS_STAGE_B_PROPOSAL.md'
  && defaultProposal?.grammar === 'GRAMMAR_STAGE_B_DEFAULT_ARGUMENTS.ebnf'
  && defaultProposal?.approval_date === '2026-10-07' && !manifest.draft_proposals?.some(p => p.id === 'P18'),
  'P18 invalid Accepted ledger');
const defaultProposalText = fs.readFileSync(path.join(pack, 'DEFAULT_ARGUMENTS_STAGE_B_PROPOSAL.md'), 'utf8');
assert(defaultProposalText.includes('Accepted / 사용자 승인 / 구현 완료')
  && defaultProposalText.includes('10,000-node') && defaultProposalText.includes('module의 top-level scope')
  && defaultProposalText.includes('나머지 **51개 production'), 'P18 invalid status/scope/budget text');
validateGrammar('GRAMMAR_STAGE_B_DEFAULT_ARGUMENTS.ebnf');
const defaultGrammar = fs.readFileSync(path.join(pack, 'GRAMMAR_STAGE_B_DEFAULT_ARGUMENTS.ebnf'), 'utf8');
assert(defaultGrammar.includes('Accepted by user on 2026-10-07'), 'P18 invalid grammar status');
const defaultProductions = productions(defaultGrammar);
assert(defaultProductions.size === 52 && namedProductions.size === 52, 'P18 must preserve production count');
for (const [name, value] of namedProductions) {
  if (name !== 'parameter') assert(defaultProductions.get(name) === value, `P18 changes existing ${name}`);
}
assert(defaultProductions.get('parameter') === '"IDENT",":",type,["=",expression]', 'P18 invalid parameter grammar');
const defaultRoot = path.join(pack, 'default-arguments-proposal-fixtures');
const defaultFixture = JSON.parse(fs.readFileSync(path.join(defaultRoot, 'expected.json'), 'utf8'));
assert(defaultFixture.proposal === 'P18' && defaultFixture.status === 'Accepted'
  && defaultFixture.implementation_verified === true && defaultFixture.approval_date === '2026-10-07',
  'P18 invalid verified fixture status');
assert(defaultFixture.entry === 'main.nova' && defaultFixture.source_root === '.'
  && JSON.stringify(defaultFixture.reachable_modules) === '["main","helpers"]', 'P18 fixture graph mismatch');
const defaultOutput = 'defaults=620\nright\nnamed=602\nleft\npositional=120\nsecond\nfirst\nreverse=702\nholes=456\ntext=default/provided\nunit=3\ncopy=4/🙂\noption=none\nresult=7\nleaf\nafter\nsuccess=602\nleaf\nerror=-1\nshort=false\n';
assert(defaultFixture.validated_result?.check_exit === 0 && defaultFixture.validated_result?.native_exit === 0
  && defaultFixture.validated_result?.stdout === defaultOutput && defaultFixture.validated_result?.stderr === '',
  'P18 verified output mismatch');
assert(defaultFixture.negative_cases?.length === 20
  && new Set(defaultFixture.negative_cases.map(c => c.name)).size === 20
  && new Set(defaultFixture.negative_cases.map(c => c.source)).size === 20, 'P18 negative set mismatch');
assert(defaultFixture.positive_cases?.length === 2
  && JSON.stringify(defaultFixture.positive_cases.map(c => c.source))
    === '["unicode_print_shadow.nova","forward_recursive_grouped.nova"]', 'P18 positive set mismatch');
for (const c of defaultFixture.positive_cases) {
  assert(c.validated_result?.check_exit === 0 && c.validated_result?.native_exit === 0
    && c.validated_result?.stdout === '' && c.validated_result?.stderr === '',
    `P18 invalid positive validated result ${c.source}`);
}
for (const name of [defaultFixture.entry, 'helpers.nova', ...defaultFixture.positive_cases.map(c => c.source),
  ...defaultFixture.negative_cases.map(c => c.source)]) {
  assert(path.basename(name) === name && name.endsWith('.nova'), `P18 invalid filename ${name}`);
  const text = fs.readFileSync(path.join(defaultRoot, name), 'utf8');
  assert(text.endsWith('\n') && !text.includes('\r'), `P18 fixture encoding/newline ${name}`);
}
for (const c of defaultFixture.negative_cases) {
  assert(knownCodes.has(c.expected_diagnostic), `P18 unknown diagnostic ${c.name}`);
  const bytes = fs.readFileSync(path.join(defaultRoot, c.source));
  const boundaries = new Set([0]);
  let offset = 0;
  for (const scalar of bytes.toString('utf8')) { offset += Buffer.byteLength(scalar); boundaries.add(offset); }
  const { start, end } = c.primary;
  assert(start < end && start >= 0 && end <= bytes.length && boundaries.has(start) && boundaries.has(end),
    `P18 invalid byte span ${c.name}`);
  assert(bytes.subarray(start, end).toString('utf8') === c.primary_text, `P18 unexpected span text ${c.name}`);
  for (const code of c.forbidden_diagnostics ?? []) {
    assert(knownCodes.has(code) && code !== c.expected_diagnostic, `P18 invalid cascade expectation ${c.name}`);
  }
}
const defaultTry = defaultFixture.negative_cases.find(c => c.name === 'default_try');
assert(defaultTry?.expected_diagnostic === 'N3201' && defaultTry?.primary_text === 'try'
  && defaultTry?.forbidden_diagnostics?.includes('N3002'), 'P18 missing const-try precedence');
for (const name of ['default_type', 'default_range', 'parameter_reference', 'self_reference',
  'caller_local_reference', 'undefined_default']) {
  const c = defaultFixture.negative_cases.find(c => c.name === name);
  assert(c?.forbidden_diagnostics?.includes('N3201'), `P18 missing upstream cascade suppression ${name}`);
}
for (const name of ['default_function_call', 'skipped_function_call']) {
  const c = defaultFixture.negative_cases.find(c => c.name === name);
  const bytes = fs.readFileSync(path.join(defaultRoot, c.source));
  assert(c.primary.start === bytes.lastIndexOf(Buffer.from(c.primary_text)), `P18 Span points at declaration instead of call ${name}`);
}
const defaultPrivate = defaultFixture.negative_cases.find(c => c.name === 'private_import');
assert(defaultPrivate?.primary?.start === 0 && defaultPrivate?.primary?.end === 20
  && defaultPrivate?.primary_text === 'use helpers::SECRET\n', 'P18 must preserve P11 whole-import N2004 Span');
const defaultMain = fs.readFileSync(path.join(defaultRoot, 'main.nova'), 'utf8');
const defaultHelpers = fs.readFileSync(path.join(defaultRoot, 'helpers.nova'), 'utf8');
assert(defaultMain.includes('use helpers::compose as joined') && defaultMain.includes('const SECRET:int8=99')
  && defaultMain.includes('joined(right:try fetch(ok:flag))') && defaultMain.includes('holes(b:5)')
  && defaultMain.includes('value:Result<int8,int8>=Result::Success(7)')
  && defaultMain.includes('value:int8?=none') && defaultMain.includes('false &&')
  && defaultHelpers.includes('private const SECRET:int8=6')
  && defaultHelpers.includes('left:int8=SECRET,right:int16=20')
  && defaultHelpers.includes("value:(int8,char)=(BASE,'🙂')"), 'P18 missing scope/context/default/effect proposal data');
checks.push('P18 Accepted/승인·구현 ledger·P17 parameter 한 production 확장·52-production EBNF/기존 51개 보존·두 파일/정상 2/부정 20사례 UTF-8 Span·cascade·검증 20줄 출력 metadata (Compiler/Native 실행 아님)');

const rangeProposal = manifest.accepted_proposals?.find(p => p.id === 'P19');
assert(rangeProposal?.status === 'Accepted' && rangeProposal?.implementation_verified === true
  && rangeProposal?.approval_date === '2026-10-08'
  && rangeProposal?.document === 'RANGE_LOOP_STAGE_B_PROPOSAL.md'
  && rangeProposal?.grammar === 'GRAMMAR_STAGE_B_RANGE_LOOP.ebnf'
  && !manifest.draft_proposals?.some(p => p.id === 'P19'), 'P19 invalid Accepted ledger');
const rangeText = fs.readFileSync(path.join(pack, rangeProposal?.document ?? 'RANGE_LOOP_STAGE_B_PROPOSAL.md'), 'utf8');
assert(rangeText.includes('Accepted / 사용자 승인 / 구현 완료')
  && rangeText.includes('증가시키지 않고 종료') && rangeText.includes('보수적으로 분석')
  && rangeText.includes('기존 나머지 51개'), 'P19 missing scope/termination/status boundaries');
validateGrammar('GRAMMAR_STAGE_B_RANGE_LOOP.ebnf');
const rangeGrammar = fs.readFileSync(path.join(pack, 'GRAMMAR_STAGE_B_RANGE_LOOP.ebnf'), 'utf8');
assert(rangeGrammar.includes('Accepted by user; recorded on 2026-10-08'), 'P19 invalid grammar status');
const rangeProductions = productions(rangeGrammar);
assert(rangeProductions.size === 54 && defaultProductions.size === 52, 'P19 production count mismatch');
for (const [name, value] of defaultProductions) {
  if (name !== 'statement') assert(rangeProductions.get(name) === value, `P19 changes existing ${name}`);
}
assert(rangeProductions.get('statement') === 'if_stmt|while_stmt|for_stmt|loop_stmt|match_stmt|simple_statement'
  && rangeProductions.get('for_stmt') === '"for","IDENT","in",expression,("until"|"through"),expression,block'
  && rangeProductions.get('loop_stmt') === '"loop",block', 'P19 invalid minimal statement grammar');
const rangeRoot = path.join(pack, 'range-loop-proposal-fixtures');
const rangeFixture = JSON.parse(fs.readFileSync(path.join(rangeRoot, 'expected.json'), 'utf8'));
assert(rangeFixture.proposal === 'P19' && rangeFixture.status === 'Accepted'
  && rangeFixture.implementation_verified === true && rangeFixture.approval_date === '2026-10-08' && !rangeFixture.proposed_result
  && rangeFixture.entry === 'main.nova' && rangeFixture.source_root === '.'
  && JSON.stringify(rangeFixture.reachable_modules) === '["main","helpers"]', 'P19 invalid verified fixture graph/status');
const rangeOutput = 'range=8\nstart\nend\nbounds=23\nempty-start\nempty-end\nempty=0\nmaximum=2/255\nmixed=0\nshadow=99/3\nnested=4\nloop=3\ntry-start\ntry-end\ntry-ok=3\ntry-start\ntry-end\ntry-error=-1\n';
assert(rangeFixture.validated_result?.check_exit === 0 && rangeFixture.validated_result?.native_exit === 0
  && rangeFixture.validated_result?.stdout === rangeOutput && rangeFixture.validated_result?.stderr === '', 'P19 verified output mismatch');
assert(rangeFixture.negative_cases?.length === 18 && rangeFixture.runtime_cases?.length === 2
  && rangeFixture.positive_cases?.length === 2, 'P19 fixture counts mismatch');
const rangeCases = [...rangeFixture.negative_cases, ...rangeFixture.runtime_cases];
assert(new Set(rangeCases.map(c => c.name)).size === 20 && new Set(rangeCases.map(c => c.source)).size === 20,
  'P19 duplicate diagnostic fixtures');
for (const c of rangeCases) {
  assert(knownCodes.has(c.expected_diagnostic), `P19 unknown diagnostic ${c.name}`);
  const bytes = fs.readFileSync(path.join(rangeRoot, c.source));
  const boundaries = new Set([0]);
  let offset = 0;
  for (const scalar of bytes.toString('utf8')) { offset += Buffer.byteLength(scalar); boundaries.add(offset); }
  const { start, end } = c.primary;
  assert(start < end && start >= 0 && end <= bytes.length && boundaries.has(start) && boundaries.has(end)
    && bytes.subarray(start, end).toString('utf8') === c.primary_text, `P19 invalid UTF-8 byte Span ${c.name}`);
  for (const code of c.forbidden_diagnostics ?? []) {
    assert(knownCodes.has(code) && code !== c.expected_diagnostic, `P19 invalid cascade ${c.name}`);
  }
}
for (const c of rangeFixture.positive_cases) {
  assert(!c.proposed_result && c.validated_result?.check_exit === 0 && c.validated_result?.native_exit === 0
    && c.validated_result?.stdout === '' && c.validated_result?.stderr === '', `P19 invalid verified positive ${c.source}`);
}
for (const c of rangeFixture.runtime_cases) {
  assert(c.expected_diagnostic === 'N5201' && c.validated_result?.check_exit === 0
    && c.validated_result?.native_exit_nonzero === true, `P19 invalid verified Runtime failure ${c.name}`);
}
assert(rangeFixture.runtime_cases.find(c => c.name === 'body_overflow')?.validated_result?.stdout === 'before\n'
  && rangeFixture.runtime_cases.find(c => c.name === 'bound_overflow')?.validated_result?.stdout === '', 'P19 Abort effect data mismatch');
for (const name of [rangeFixture.entry, 'helpers.nova', ...rangeFixture.positive_cases.map(c => c.source), ...rangeCases.map(c => c.source)]) {
  assert(path.basename(name) === name && name.endsWith('.nova'), `P19 invalid filename ${name}`);
  const text = fs.readFileSync(path.join(rangeRoot, name), 'utf8');
  assert(text.endsWith('\n') && !text.includes('\r'), `P19 newline mismatch ${name}`);
}
for (const name of ['undefined_bound', 'binder_not_in_bound_scope', 'peer_literal_range']) {
  assert(rangeFixture.negative_cases.find(c => c.name === name)?.forbidden_diagnostics?.includes('N2101'),
    `P19 missing upstream cascade suppression ${name}`);
}
const rangeMain = fs.readFileSync(path.join(rangeRoot, 'main.nova'), 'utf8');
assert(rangeMain.includes('use helpers::edge as bound') && rangeMain.includes('through MAX')
  && rangeMain.includes('tag:"empty-start"') && rangeMain.includes('let lo:int8=-1')
  && rangeMain.includes('until try fetch') && rangeMain.includes('loop {') && rangeMain.includes('continue'),
  'P19 missing bound/effect/max/jump/context proposal cases');
checks.push('P19 Accepted/승인·구현 ledger·P18 statement 확장/두 production 추가·54-production EBNF/기존 51개 보존·두 파일/정상 2/부정 18/Runtime 2 UTF-8 Span·cascade·검증 18줄 metadata (문서 validator는 Compiler/Native 실행 아님)');

const rangeUnit = rangeFixture.negative_cases.find(c => c.name === 'unit_bound');
assert(rangeUnit?.primary?.start === 22 && rangeUnit?.primary?.end === 24, 'P19 Unit bound Span must not point to function parameter parentheses');

const existsProposal = manifest.accepted_proposals?.find(p => p.id === 'P20');
assert(existsProposal?.status === 'Accepted' && existsProposal?.implementation_verified === true && existsProposal?.approval_date === '2026-10-08'
  && existsProposal?.document === 'EXISTS_STAGE_B_PROPOSAL.md' && existsProposal?.grammar === 'GRAMMAR_STAGE_B_EXISTS.ebnf'
  && !manifest.draft_proposals?.some(p => p.id === 'P20'), 'P20 invalid Accepted ledger');
const existsText = fs.readFileSync(path.join(pack, 'EXISTS_STAGE_B_PROPOSAL.md'), 'utf8');
assert(existsText.includes('Accepted / 사용자 승인 / 구현 완료') && existsText.includes('53개 보존')
  && existsText.includes('10,000-node') && existsText.includes('None=0/nonnull pointer/payload truthiness')
  && existsText.includes('flow narrowing'), 'P20 missing status/scope/const/layout boundaries');
validateGrammar('GRAMMAR_STAGE_B_EXISTS.ebnf');
const existsGrammar = fs.readFileSync(path.join(pack, 'GRAMMAR_STAGE_B_EXISTS.ebnf'), 'utf8');
const existsProductions = productions(existsGrammar);
assert(existsGrammar.includes('Accepted by user; recorded on 2026-10-08') && existsProductions.size === 54, 'P20 invalid grammar');
for (const [name, value] of rangeProductions) {
  if (name !== 'postfix_expr') assert(existsProductions.get(name) === value, 'P20 changed approved production ' + name);
}
assert(existsProductions.get('postfix_expr') === rangeProductions.get('postfix_expr').replace('}', '|"exists"}'), 'P20 invalid postfix extension');
const existsRoot = path.join(pack, 'exists-proposal-fixtures');
const existsFixture = JSON.parse(fs.readFileSync(path.join(existsRoot, 'expected.json'), 'utf8'));
const existsOutput = 'basic=true/false\npayload=true/true\nnested=true/true\nconst=true/true\ndefault=true\nonce\nonce=true\nshort=false/true\nsnapshot=true/false\ntuple=true\nnegate=true\nsecond\nfirst\norder=false\ntry\ntry-ok=true\ntry\ntry-error=-1\n';
assert(existsFixture.proposal === 'P20' && existsFixture.status === 'Accepted' && existsFixture.implementation_verified === true
  && !existsFixture.proposed_result && existsFixture.approval_date === '2026-10-08' && existsFixture.entry === 'main.nova'
  && existsFixture.source_root === '.' && JSON.stringify(existsFixture.reachable_modules) === '["main","helpers"]', 'P20 invalid verified fixture status');
assert(existsFixture.validated_result?.check_exit === 0 && existsFixture.validated_result?.native_exit === 0
  && existsFixture.validated_result?.stdout === existsOutput && existsFixture.validated_result?.stderr === '', 'P20 verified output mismatch');
assert(existsFixture.negative_cases?.length === 20 && existsFixture.positive_cases?.length === 2
  && existsFixture.runtime_cases?.length === 1, 'P20 invalid fixture counts');
for (const c of [...existsFixture.negative_cases, ...existsFixture.runtime_cases]) {
  assert(path.basename(c.source) === c.source && c.source.endsWith('.nova'), 'P20 invalid filename ' + c.source);
  const bytes = fs.readFileSync(path.join(existsRoot, c.source));
  assert(knownCodes.has(c.expected_diagnostic) && c.primary?.start >= 0 && c.primary?.end > c.primary.start
    && c.primary.end <= bytes.length && bytes.subarray(c.primary.start, c.primary.end).toString('utf8') === c.primary_text, 'P20 invalid code/UTF-8 Span ' + c.name);
  assert((c.forbidden_diagnostics ?? []).every(code => knownCodes.has(code) && code !== c.expected_diagnostic), 'P20 invalid cascade ' + c.name);
}
for (const c of existsFixture.positive_cases) {
  assert(path.basename(c.source) === c.source && c.source.endsWith('.nova') && !c.proposed_result
    && c.validated_result?.check_exit === 0 && c.validated_result?.native_exit === 0
    && c.validated_result?.stdout === '' && c.validated_result?.stderr === '', 'P20 invalid verified positive ' + c.source);
  assert(fs.existsSync(path.join(existsRoot, c.source)), 'P20 missing positive source');
}
const existsRuntime = existsFixture.runtime_cases[0];
assert(!existsRuntime.proposed_result && existsRuntime.expected_diagnostic === 'N5201'
  && existsRuntime.validated_result?.check_exit === 0 && existsRuntime.validated_result?.native_exit_nonzero === true
  && existsRuntime.validated_result?.stdout === 'before\n', 'P20 invalid verified Abort effect');
const existsMain = fs.readFileSync(path.join(existsRoot, existsFixture.entry), 'utf8');
const existsHelpers = fs.readFileSync(path.join(existsRoot, 'helpers.nova'), 'utf8');
assert(existsMain.includes('ABSENT as 없음') && existsMain.includes('LATER exists') && existsMain.includes('data.value exists')
  && existsMain.includes('(try fetch_result(fail)) exists') && existsMain.includes('second:fetch(')
  && existsHelpers.includes('private enum Secret') && existsHelpers.includes('Option::Some(none)')
  && existsHelpers.includes('second:bool=PRESENT'), 'P20 missing import/nested/const/default/snapshot/effect/try data');
checks.push('P20 Accepted/승인·구현 ledger·P19 postfix 한 production 확장·54-production EBNF/기존 53개 보존·두 파일/정상 2/부정 20/Runtime 1 UTF-8 Span·cascade·검증 18줄 metadata (Compiler/Native 실행 아님)');

for (const name of ['const_runtime_call','skipped_const_runtime_call','default_runtime_call']) {
  const c = existsFixture.negative_cases.find(c => c.name === name);
  const bytes = fs.readFileSync(path.join(existsRoot,c.source));
  assert(c.primary.start === bytes.lastIndexOf(Buffer.from('fetch()')), 'P20 const/default diagnostic must target call, not declaration');
}
const existsPrecedence = existsFixture.negative_cases.find(c => c.name === 'try_precedence');
assert(existsPrecedence.primary_text === 'fetch()' && existsPrecedence.primary.start === 153
  && existsPrecedence.primary.end === 160, 'P20 Result exists fails before derived try error');

const aliasProposal = manifest.accepted_proposals?.find(p => p.id === 'P21');
assert(aliasProposal?.status === 'Accepted' && aliasProposal?.implementation_verified === true
  && aliasProposal?.document === 'ALIAS_STAGE_B_PROPOSAL.md' && aliasProposal?.grammar === 'GRAMMAR_STAGE_B_ALIAS.ebnf'
  && !manifest.draft_proposals?.some(p => p.id === 'P21'), 'P21 invalid Accepted ledger');
const aliasText = fs.readFileSync(path.join(pack,'ALIAS_STAGE_B_PROPOSAL.md'),'utf8');
assert(aliasText.includes('Accepted / 사용자 승인 / 구현 완료') && aliasText.includes('1,024개')
  && aliasText.includes('53개 보존') && aliasText.includes('N2103') && aliasText.includes('API leak'), 'P21 missing boundaries');
validateGrammar('GRAMMAR_STAGE_B_ALIAS.ebnf');
const aliasGrammar = fs.readFileSync(path.join(pack,'GRAMMAR_STAGE_B_ALIAS.ebnf'),'utf8');
const aliasProductions = productions(aliasGrammar);
assert(aliasGrammar.includes('Accepted by user; recorded on 2026-10-08') && aliasProductions.size === 55, 'P21 invalid grammar');
for (const [name,value] of existsProductions) {
  if (name !== 'program') assert(aliasProductions.get(name) === value, 'P21 changed approved production '+name);
}
assert(aliasProductions.get('program') === existsProductions.get('program').replace('|enum_decl)', '|enum_decl|alias_decl)'), 'P21 invalid top-level extension');
assert(aliasProductions.get('alias_decl') === '"type","IDENT","=",type,end', 'P21 invalid alias declaration');
const aliasRoot=path.join(pack,'alias-proposal-fixtures');
const aliasFixture=JSON.parse(fs.readFileSync(path.join(aliasRoot,'expected.json'),'utf8'));
const aliasOutput='small=7\npair=7/2\nexists=true\nflag=on\ncast=7\ntext=한글\ntry=true\nerror=-1\n';
assert(aliasFixture.proposal === 'P21' && aliasFixture.status === 'Accepted' && aliasFixture.implementation_verified === true
  && aliasFixture.approval_date === '2026-10-08' && !aliasFixture.proposed_result && aliasFixture.entry === 'main.nova'
  && aliasFixture.source_root === '.' && JSON.stringify(aliasFixture.reachable_modules) === '["main","types"]', 'P21 invalid verified fixture ledger');
assert(aliasFixture.validated_result?.check_exit === 0 && aliasFixture.validated_result?.native_exit === 0
  && aliasFixture.validated_result?.stdout === aliasOutput && aliasFixture.validated_result?.stderr === '', 'P21 verified output mismatch');
assert(aliasFixture.negative_cases?.length === 16 && aliasFixture.positive_cases?.length === 1,'P21 invalid case counts');
for (const c of aliasFixture.positive_cases ?? []) {
  assert(c.source === 'multiline.nova' && fs.existsSync(path.join(aliasRoot,c.source)) && !c.proposed_result
    && c.validated_result?.check_exit === 0 && c.validated_result?.native_exit === 0
    && c.validated_result?.stdout === '' && c.validated_result?.stderr === '', 'P21 invalid verified positive');
}
for (const c of aliasFixture.negative_cases ?? []) {
  const bytes=fs.readFileSync(path.join(aliasRoot,c.source));
  assert(path.basename(c.source) === c.source && knownCodes.has(c.expected_diagnostic) && c.primary.start >= 0
    && c.primary.end <= bytes.length && c.primary.end > c.primary.start
    && bytes.subarray(c.primary.start,c.primary.end).toString('utf8') === c.primary_text, 'P21 invalid code/UTF-8 span '+c.name);
}
const aliasMain=fs.readFileSync(path.join(aliasRoot,'main.nova'),'utf8');
const aliasTypes=fs.readFileSync(path.join(aliasRoot,'types.nova'),'utf8');
assert(aliasMain.includes('use types::Small as Tiny') && aliasMain.includes('func Tiny(value:Tiny)')
  && aliasMain.includes('let shape:Shape') && aliasMain.includes('wide as Tiny') && aliasMain.includes('try sum(fail)')
  && aliasMain.includes('value exists') && aliasTypes.includes('type Small = Later') && aliasTypes.includes('type Later = int8')
  && aliasTypes.includes('value:Small=3'), 'P21 missing forward/scope/namespace/Tuple/cast/try/default data');
checks.push('P21 Accepted ledger·P20 program 한 production 확장/alias_decl 추가·55-production EBNF/기존 53개 보존·두 파일/정상 1/부정 16 UTF-8 Span·검증된 8줄 metadata (문서 검사 자체는 Compiler/Native 실행 아님)');

const methodProposal = manifest.accepted_proposals?.find(p => p.id === 'P22');
assert(methodProposal?.status === 'Accepted' && methodProposal?.implementation_verified === true && methodProposal?.approval_date === '2026-10-08'
  && methodProposal?.document === 'METHOD_STAGE_B_PROPOSAL.md' && methodProposal?.grammar === 'GRAMMAR_STAGE_B_METHOD.ebnf'
  && !manifest.draft_proposals?.some(p => p.id === 'P22'), 'P22 invalid Accepted ledger');
const methodText = fs.readFileSync(path.join(pack, 'METHOD_STAGE_B_PROPOSAL.md'), 'utf8');
assert(methodText.includes('Accepted / 2026-10-08 사용자 승인 / 구현 완료') && methodText.includes('54개 보존')
  && methodText.includes('bundle 1,024개') && methodText.includes('immutable Read') && methodText.includes('10,000-node')
  && methodText.includes('receiver slot') && methodText.includes('API leak'), 'P22 missing boundaries');
validateGrammar('GRAMMAR_STAGE_B_METHOD.ebnf');
const methodGrammar = fs.readFileSync(path.join(pack, 'GRAMMAR_STAGE_B_METHOD.ebnf'), 'utf8');
const methodProductions = productions(methodGrammar);
assert(methodGrammar.includes('Accepted by the user on 2026-10-08') && methodProductions.size === 58, 'P22 invalid grammar');
for (const [name, value] of aliasProductions) {
  if (name !== 'struct_decl') assert(methodProductions.get(name) === value, 'P22 changed approved production '+name);
}
assert(methodProductions.get('struct_decl') === aliasProductions.get('struct_decl').replace('field_decl','struct_member'), 'P22 invalid struct extension');
assert(methodProductions.get('struct_member') === 'field_decl|[visibility],method_decl'
  && methodProductions.get('method_decl') === '"func","IDENT","(",receiver,[",",[parameters]],")",["->",type],block'
  && methodProductions.get('receiver')?.includes('contextualIDENT'), 'P22 invalid receiver/method declaration');
const methodRoot=path.join(pack,'method-proposal-fixtures');
const methodFixture=JSON.parse(fs.readFileSync(path.join(methodRoot,'expected.json'),'utf8'));
const methodOutput='sum=8\nalias=7\ncopy=3/5\ntext=3/4\nprivate=3\nmember-main=4\ntuple=7\nreceiver\narg\norder=9\nfetch\nafter\nok=11\nfetch\nerror=-1\n';
assert(methodFixture.proposal === 'P22' && methodFixture.status === 'Accepted' && methodFixture.implementation_verified === true
  && methodFixture.approval_date === '2026-10-08' && !methodFixture.proposed_result && methodFixture.entry === 'main.nova'
  && methodFixture.source_root === '.' && JSON.stringify(methodFixture.reachable_modules) === '["main","types"]', 'P22 invalid verified fixture ledger');
assert(methodFixture.validated_result?.check_exit === 0 && methodFixture.validated_result?.native_exit === 0
  && methodFixture.validated_result?.stdout === methodOutput && methodFixture.validated_result?.stderr === '', 'P22 validated output mismatch');
assert(methodFixture.negative_cases?.length === 20 && methodFixture.positive_cases?.length === 1
  && methodFixture.runtime_cases?.length === 1, 'P22 invalid case counts');
for (const c of [...methodFixture.negative_cases, ...methodFixture.runtime_cases]) {
  const bytes=fs.readFileSync(path.join(methodRoot,c.source));
  assert(path.basename(c.source) === c.source && c.primary.start >= 0 && c.primary.end <= bytes.length
    && c.primary.end > c.primary.start && bytes.subarray(c.primary.start,c.primary.end).toString('utf8') === c.primary_text
, 'P22 invalid UTF-8 Span '+c.name);
  if (c.expected_diagnostic) assert(knownCodes.has(c.expected_diagnostic), 'P22 unknown code '+c.name);
}
for (const c of methodFixture.positive_cases) {
  assert(c.source === 'newline_and_self.nova' && fs.existsSync(path.join(methodRoot,c.source)) && !c.proposed_result
    && c.validated_result?.check_exit === 0 && c.validated_result?.native_exit === 0
    && c.validated_result?.stdout === '' && c.validated_result?.stderr === '', 'P22 invalid verified positive');
}
const methodAbort=methodFixture.runtime_cases[0];
assert(methodAbort.validated_result?.check_exit === 0 && methodAbort.validated_result?.native_exit === 1
  && methodAbort.validated_result?.stdout === 'before\n'
  && methodAbort.validated_result?.stderr_first_line === `Nova panic: integer overflow at file#0:${methodAbort.primary.start}..${methodAbort.primary.end}`, 'P22 invalid validated checked Abort');
const methodMain=fs.readFileSync(path.join(methodRoot,'main.nova'),'utf8');
const methodTypes=fs.readFileSync(path.join(methodRoot,'types.nova'),'utf8');
assert(methodMain.includes('use types::PointType') && methodMain.includes('make().sum(bias:mark("arg",2))')
  && methodMain.includes('try p.checked(fail:fail)') && methodMain.includes('p.main()') && methodMain.includes('SHIFT:int8=20')
  && methodTypes.includes('func sum(self,bias:Small=SHIFT)') && methodTypes.includes('private func secret(self)')
  && methodTypes.includes('var copy=self') && methodTypes.includes('type Small=int8'), 'P22 missing receiver/scope/alias/named/default/try data');
checks.push('P22 Accepted ledger·P21 struct_decl 한 production 확장/세 production 추가·58-production EBNF/기존 54개 보존·두 파일/정상 1/부정 20/Runtime 1 UTF-8 Span·검증된 15줄 metadata (문서 검사 자체는 Compiler/Native 실행 아님)');

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
