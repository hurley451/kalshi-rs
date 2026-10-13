import assert from 'node:assert/strict';
import { readFile, mkdtemp, mkdir, writeFile, rm } from 'node:fs/promises';
import { spawnSync } from 'node:child_process';
import { tmpdir } from 'node:os';
import { fileURLToPath } from 'node:url';
import path from 'node:path';
import test from 'node:test';
import { qualifyPredictionsOrderRules } from './predictions_order_rules.mjs';

test('V2 source rules qualify exact descriptions and bindings without mutation', async () => {
  assert.ok(process.env.KALSHICORE_ROOT, 'Set KALSHICORE_ROOT to the pinned Core input');
  const bundle = JSON.parse(await readFile(path.join(process.env.KALSHICORE_ROOT, 'contracts/current-official.json'), 'utf8'));
  const source = bundle.sources.openapi.document;
  const before = JSON.stringify(source);
  const metadata = qualifyPredictionsOrderRules(source, bundle.sources.openapi.sha256);
  assert.equal(metadata.rules.length, 2);
  assert.deepEqual(metadata.bindings.map(binding => binding.operationId), ['CreateOrderV2', 'BatchCreateOrdersV2']);
  const mutations = [
    value => { value.components.schemas.CreateOrderV2Request.properties.reduce_only.description += 'changed'; },
    value => { value.components.schemas.CreateOrderV2Request.properties.expiration_time.description += 'changed'; },
    value => { value.components.schemas.CreateOrderV2Request.properties.expiration_time.format = 'int32'; },
    value => { value.components.schemas.CreateOrderV2Request.properties.expiration_time.nullable = true; },
    value => { value.components.schemas.CreateOrderV2Request.required = value.components.schemas.CreateOrderV2Request.required.filter(name => name !== 'time_in_force'); },
    value => { value.components.schemas.CreateOrderV2Request.properties.reduce_only.nullable = true; },
    value => { value.components.schemas.BatchCreateOrdersV2Request.required = []; },
    value => { value.components.schemas.CreateOrderV2Request.properties.time_in_force.enum.pop(); },
    value => { value.paths['/portfolio/events/orders'].post.operationId = 'Wrong'; },
    value => { value.paths['/portfolio/events/orders'].post.requestBody.required = false; },
    value => { value.paths['/portfolio/events/orders/batched'].post.requestBody.content['application/json'].schema.$ref = '#/components/schemas/CreateOrderV2Request'; },
    value => { value.components.schemas.BatchCreateOrdersV2Request.properties.orders.items.$ref = '#/components/schemas/CreateOrderRequest'; },
  ];
  for (const mutate of mutations) {
    const changed = structuredClone(source); mutate(changed);
    assert.throws(() => qualifyPredictionsOrderRules(changed, bundle.sources.openapi.sha256), /V2/);
  }
  assert.throws(() => qualifyPredictionsOrderRules(source, 'wrong'), /source hash/);
  assert.equal(JSON.stringify(source), before);
});

test('unqualified rule source fails before replacing staged outputs', async t => {
  assert.ok(process.env.KALSHICORE_ROOT, 'Set KALSHICORE_ROOT to the pinned Core input');
  const bundle = JSON.parse(await readFile(path.join(process.env.KALSHICORE_ROOT, 'contracts/current-official.json'), 'utf8'));
  const root = await mkdtemp(path.join(tmpdir(), 'V2 rule preservation with spaces '));
  t.after(() => rm(root, { recursive: true, force: true }));
  const output = path.join(root, 'output');
  await mkdir(path.join(output, 'src/generated'), { recursive: true });
  const files = ['typed-schema.json', 'predictions-request-rules.json', 'models.rs', 'typed.rs'];
  for (const file of files) await writeFile(path.join(output, 'src/generated', file), `sentinel:${file}`);
  const original = await Promise.all(files.map(file => readFile(path.join(output, 'src/generated', file))));
  const mutations = [
    value => { value.components.schemas.CreateOrderV2Request.properties.reduce_only.description += 'changed'; },
    value => { value.components.schemas.CreateOrderV2Request.required = value.components.schemas.CreateOrderV2Request.required.filter(name => name !== 'time_in_force'); },
    value => { value.components.schemas.CreateOrderV2Request.properties.reduce_only.nullable = true; },
    value => { value.components.schemas.BatchCreateOrdersV2Request.required = []; },
  ];
  const input = path.join(root, 'bundle.json');
  const script = fileURLToPath(new URL('./generate_typed.mjs', import.meta.url));
  for (const mutate of mutations) {
    const changed = structuredClone(bundle); mutate(changed.sources.openapi.document);
    await writeFile(input, JSON.stringify(changed));
    const result = spawnSync(process.execPath, [script, input, '--out-dir', output], { encoding: 'utf8' });
    assert.notEqual(result.status, 0);
    assert.match(result.stderr, /V2 order (?:rule source|model prerequisites) changed/);
    const after = await Promise.all(files.map(file => readFile(path.join(output, 'src/generated', file))));
    assert.deepEqual(after, original);
  }
});
