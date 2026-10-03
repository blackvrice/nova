import fs from 'node:fs';
import path from 'node:path';

const examples = [
  ['hello','T013','A','runtime','D19/D23', 'func main() {\n    print("Hello, Nova")\n}\n', null, null, 'Hello, Nova\n'],
  ['function-call','T014','A','check','D07/D11', 'func twice(value: int) -> int {\n    return value + value\n}\nfunc main() {\n    let answer = twice(21)\n}\n'],
  ['semicolon-newline','T006','A','check','D05','func main() {\n    let first = 1; let second = 2\n    print("ok")\n}\n'],
  ['operator-continuation','T006','A','check','D05','func main() {\n    let answer = 1\n        + 2 * 3\n}\n'],
  ['nested-comment','T005','A','check','D04','func main() {\n    /* outer /* inner */ outer */\n    print("ok")\n}\n'],
  ['non-bool-if','T012','A','typecheck','D08','func main() {\n    if 1 { print("bad") }\n}\n','N3001','1'],
  ['undefined-name','T011','A','resolve','D06','func main() {\n    print(missing)\n}\n','N2001','missing'],
  ['type-mismatch','T008','A','typecheck','D07','func main() {\n    let value: int = "hello"\n}\n','N2101','"hello"'],
  ['chained-compare','T007','A','parse','D03','func main() {\n    let bad = 1 < 2 < 3\n}\n','N1103','< 3'],
  ['bad-literal','T003','A','lex','D04','func main() {\n    let bad = 1__2\n}\n','N1002','1__2'],
  ['missing-return','T012','A','control','D08','func value() -> int {\n    if true { return 1 }\n}\nfunc main() {}\n','N3003','}\nfunc main'],
  ['literal-range','T008','A','typecheck','D07','func main() {\n    let bad: int8 = 128\n}\n','N2102','128'],
  ['narrow-float','T009','B','typecheck','D07','func main() {\n    let value: int32 = 1\n    let narrowed: float = value\n}\n','N2101','value'],
  ['const-divzero','T018','B','const','D09','const BAD: int = 1 / 0\nfunc main() {}\n','N3201','1 / 0'],
  ['explicit-generic-call','T023','D','parse','D15','func identity<T>(value: T) -> T { return value }\nfunc main() { let value = identity<int>(1) }\n','N1102','identity<int>'],
  ['use-after-move','T020','C','move','D10','func main() {\n    let message = "owned"\n    let moved = message\n    print(message)\n}\n','N4101','message'],
  ['range-max','T012','B','runtime','D08','func main() {\n    for value in 2147483647 through 2147483647 {\n        print("once")\n    }\n}\n',null,null,'once\n'],
  ['short-circuit','T012','A','runtime','D08','func fail() -> bool {\n    panic("must not execute")\n}\nfunc main() {\n    if false && fail() { print("bad") }\n    print("ok")\n}\n',null,null,'ok\n'],
  ['unicode-column','T035','A','resolve','D02/D25','func main() {\n    let 이름 = "🙂"\n    print(없는이름)\n}\n','N2001','없는이름'],
  ['overflow-abort','T010','A','runtime','D07','func add(value: int) -> int { return value + 1 }\nfunc main() { let result = add(2147483647) }\n',null,null,null,'abort'],
];

export function writeFixtures(out) {
  const dir = path.join(out, 'fixtures');
  fs.mkdirSync(dir, { recursive: true });
  const records = [];
  for (const [name,caseId,stage,phase,decisions,source,code,needle,stdout,exitClass] of examples) {
    const bytes = Buffer.from(source);
    const start = needle ? bytes.lastIndexOf(Buffer.from(needle)) : null;
    if (needle && start < 0) throw new Error(`Fixture span not found: ${name}`);
    // Multi-token/context needles are deliberate draft expectations; finalize
    // the exact primary policy when the corresponding diagnostic is accepted.
    const primaryLength = ['N1103', 'N3003'].includes(code) ? 1 : needle ? Buffer.byteLength(needle) : 0;
    const primary = needle ? { start, end: start + primaryLength } : null;
    const expected = { status: 'Draft', implementation_verified: false, case_id: caseId,
      stage, phase, decisions: decisions.split('/'), outcome: code ? 'compile-fail' : phase === 'runtime' ? 'runtime' : 'compile-pass',
      diagnostic_code: code ?? null, primary, stdout: stdout ?? null,
      exit_class: exitClass ?? (code ? 'compiler-user-error' : 'success') };
    fs.writeFileSync(path.join(dir, `${name}.nova`), source, 'utf8');
    fs.writeFileSync(path.join(dir, `${name}.json`), JSON.stringify(expected, null, 2) + '\n', 'utf8');
    records.push({ name, source: `${name}.nova`, expectation: `${name}.json`, case_id: caseId });
  }
  fs.writeFileSync(path.join(dir, 'CASE_MANIFEST.json'), JSON.stringify({ schema_version: 1, status: 'Draft', fixtures: records }, null, 2) + '\n');
  fs.writeFileSync(path.join(dir, 'README.md'), '# 검토용 Nova fixture\n\n' +
    '20개 예제/sidecar는 사양 제안의 검토용이다. 현재 compiler로 실행 검증하지 않았다. ' +
    '예상 code/Span 역시 D25 승인 후 diagnostic primary 정책에 맞춰 동결한다. 현재 Rust 기반 unit tests와 분리된다.\n\n' +
    '| 예제 | 수용 묶음 | 기대 파일 |\n|---|---|---|\n' +
    records.map(r => `| [${r.name}](${r.source}) | ${r.case_id} | [sidecar](${r.expectation}) |`).join('\n') + '\n');
  return records.length;
}
