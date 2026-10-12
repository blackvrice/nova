// Independent small-domain P25 reference. This does not run the compiler.
import fs from 'node:fs';
import assert from 'node:assert/strict';

const ledger = JSON.parse(fs.readFileSync(new URL('../../docs/development-v0.1/guard-proposal-fixtures/coverage-vectors.json', import.meta.url), 'utf8'));
assert.equal(ledger.proposal, 'P25');
assert.equal(ledger.finite_domain_reference_only, true);
assert.equal(ledger.cases.length, 6);
function matches(pattern, value) {
  if (pattern === '_') return true;
  if (Array.isArray(pattern)) return Array.isArray(value) && pattern.length === value.length
    && pattern.every((child, index) => matches(child, value[index]));
  return pattern === value;
}
const names = new Set();
for (const c of ledger.cases) {
  assert(!names.has(c.name), 'duplicate reference name');
  names.add(c.name);
  const covered = new Set(), unreachable = [];
  c.arms.forEach((arm, index) => {
    const matching = c.domain.flatMap((value, at) => matches(arm.pattern, value) ? [at] : []);
    if (!matching.some(at => !covered.has(at))) unreachable.push(index);
    // Analysis deliberately does not use even literal true/false guard values.
    if (arm.guard === null) matching.forEach(at => covered.add(at));
  });
  const missing = c.domain.flatMap((_, at) => covered.has(at) ? [] : [at]);
  assert.deepEqual(missing, c.missing_indexes, `${c.name}: missing`);
  assert.deepEqual(unreachable, c.unreachable_arms, `${c.name}: unreachable`);
  const winners = [], traces = [];
  for (const value of c.domain) {
    let winner = null;
    const trace = [];
    for (const [index, arm] of c.arms.entries()) {
      if (!matches(arm.pattern, value)) continue;
      if (arm.guard !== null) trace.push(arm.trace);
      if (arm.guard === null || arm.guard) { winner = index; break; }
    }
    winners.push(winner); traces.push(trace);
  }
  assert.deepEqual(winners, c.winners, `${c.name}: selected arms`);
  assert.deepEqual(traces, c.traces, `${c.name}: guard effects`);
}
console.log('PASS: 6 P25 finite-domain coverage/usefulness and guard-trace reference vectors (not Compiler/Native validation).');
