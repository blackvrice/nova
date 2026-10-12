// Independent small finite type domains; does not run the Nova compiler.
import fs from 'node:fs';
import assert from 'node:assert/strict';
const ledger=JSON.parse(fs.readFileSync(new URL('../../docs/development-v0.1/destructuring-proposal-fixtures/coverage-vectors.json',import.meta.url),'utf8'));
assert.equal(ledger.proposal,'P26');
assert.equal(ledger.finite_domain_reference_only,true);
assert.equal(ledger.cases.length,6);
function matches(pattern,value) {
  if (pattern==='_') return true;
  if (Array.isArray(pattern)) return Array.isArray(value) && pattern.length===value.length
    && pattern.every((p,i)=>matches(p,value[i]));
  if (pattern && typeof pattern==='object') {
    if (pattern.unit) return value?.unit===true;
    return pattern.variant===value?.variant && pattern.fields.length===value.fields.length
      && pattern.fields.every((p,i)=>matches(p,value.fields[i]));
  }
  return pattern===value;
}
const names=new Set();
for (const c of ledger.cases) {
  assert(!names.has(c.name),'duplicate reference name');names.add(c.name);
  const missing=c.domain.flatMap((value,index)=>matches(c.pattern,value)?[]:[index]);
  assert.deepEqual(missing,c.missing_indexes,`${c.name}: type-domain coverage`);
  if (c.initializer_example!==undefined) {
    assert(matches(c.pattern,c.initializer_example),`${c.name}: initializer example must match`);
    assert(missing.length>0,`${c.name}: known initializer must not prove irrefutability`);
  }
}
console.log('PASS: 6 P26 finite type-domain irrefutability reference vectors (not Compiler/Native validation).');
