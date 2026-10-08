// Independent, small finite-domain reference for P24 draft coverage vectors.
// Nova's proposed implementation uses bounded symbolic coverage, not enumeration.
import fs from 'node:fs';
import assert from 'node:assert/strict';

function matches(pattern, value) {
  if (pattern === '_') return true;
  if (Array.isArray(pattern)) {
    return Array.isArray(value) && pattern.length === value.length
      && pattern.every((p, i) => matches(p, value[i]));
  }
  if (pattern !== null && typeof pattern === 'object') {
    return value !== null && typeof value === 'object' && !Array.isArray(value)
      && pattern.ctor === value.ctor && pattern.args.length === value.args.length
      && pattern.args.every((p, i) => matches(p, value.args[i]));
  }
  return pattern === value;
}

const ledger = JSON.parse(fs.readFileSync(new URL(
  '../../docs/development-v0.1/nested-pattern-proposal-fixtures/coverage-vectors.json', import.meta.url), 'utf8'));
assert.equal(ledger.proposal, 'P24');
assert.equal(ledger.finite_domain_reference_only, true);
assert.equal(ledger.cases.length, 6);
const names = new Set();
for (const c of ledger.cases) {
  assert(!names.has(c.name), 'duplicate reference name');
  names.add(c.name);
  const covered = new Set();
  const unreachable = [];
  c.patterns.forEach((pattern, arm) => {
    const selected = c.domain.flatMap((value, index) => matches(pattern, value) ? [index] : []);
    if (!selected.some(index => !covered.has(index))) unreachable.push(arm);
    selected.forEach(index => covered.add(index));
  });
  const missing = c.domain.flatMap((_, index) => covered.has(index) ? [] : [index]);
  assert.deepEqual(missing, c.missing_indexes, `${c.name}: missing cases`);
  assert.deepEqual(unreachable, c.unreachable_arms, `${c.name}: unreachable arms`);
}
console.log('PASS: 6 P24 finite-domain draft coverage vectors (not Compiler/Native validation).');
