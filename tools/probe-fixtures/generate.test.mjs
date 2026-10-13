import test from 'node:test';
import assert from 'node:assert/strict';
import { mkdtemp, mkdir, writeFile, readFile, rm } from 'node:fs/promises';
import { tmpdir } from 'node:os';
import path from 'node:path';
import { fileURLToPath } from 'node:url';
import { spawnSync } from 'node:child_process';

test('cached references preserve distinct local examples without mutating source schemas', async t => {
  const root = await mkdtemp(path.join(tmpdir(), 'kalshi-fixture-examples-'));
  t.after(() => rm(root, { recursive: true, force: true }));
  for (const directory of ['src/generated', 'tools', 'tests']) await mkdir(path.join(root, directory), { recursive: true });
  const schema = { $schema: 'http://json-schema.org/draft-07/schema#', definitions: {
    Dollars: { type: 'string', example: '0.5600' },
    Settlement: { type: 'object', required: ['cost', 'fee'], additionalProperties: false, properties: {
      cost: { $ref: '#/definitions/Dollars' },
      fee: { $ref: '#/definitions/Dollars', example: '0.3400', description: 'Distinct fee example' },
    } },
    SettlementPage: { type: 'object', required: ['settlements'], additionalProperties: false, properties: {
      settlements: { type: 'array', minItems: 1, maxItems: 1, items: { $ref: '#/definitions/Settlement' } },
    } },
  } };
  const input = JSON.stringify(schema, null, 2);
  const schemaFile = path.join(root, 'src/generated/typed-schema.json');
  await writeFile(schemaFile, input);
  await writeFile(path.join(root, 'tools/typed-coverage.json'), JSON.stringify({ operations: [
    { id: 'SettlementProbe', success: 'SettlementPage', errors: [] },
  ] }));
  const script = fileURLToPath(new URL('./generate.mjs', import.meta.url));
  const run = () => {
    const result = spawnSync(process.execPath, [script], { cwd: root, encoding: 'utf8' });
    assert.equal(result.status, 0, result.stderr);
  };
  run();
  const files = ['typed-fixtures.json', 'typed-model-fixtures.json', 'typed-model-full-fixtures.json'];
  const before = await Promise.all(files.map(file => readFile(path.join(root, 'tests', file), 'utf8')));
  for (const text of before) {
    const fixture = JSON.parse(text);
    const settlements = fixture.SettlementProbe?.response.settlements ?? [fixture.Settlement, ...fixture.SettlementPage.settlements];
    for (const settlement of settlements) {
      assert.equal(settlement.cost, '0.5600');
      assert.equal(settlement.fee, '0.3400');
    }
  }
  assert.equal(await readFile(schemaFile, 'utf8'), input);
  run();
  assert.deepEqual(await Promise.all(files.map(file => readFile(path.join(root, 'tests', file), 'utf8'))), before);
});
