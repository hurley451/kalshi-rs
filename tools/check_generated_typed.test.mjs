import assert from 'node:assert/strict';
import { test } from 'node:test';
import { cp, mkdir, mkdtemp, readFile, rm, writeFile } from 'node:fs/promises';
import path from 'node:path';
import { tmpdir } from 'node:os';
import { spawnSync } from 'node:child_process';
import { fileURLToPath } from 'node:url';
import { generatedFiles, repositoryRoot, stageGenerated, compareGenerated, checkGenerated } from './check_generated_typed.mjs';

test('explicit contract source is required', () => {
  const env = {...process.env};
  delete env.KALSHICORE_ROOT;
  for (const script of ['generate_typed.mjs', 'check_generated_typed.mjs']) {
    const result = spawnSync(process.execPath, [fileURLToPath(new URL(script, import.meta.url))], {env, encoding:'utf8'});
    assert.notEqual(result.status, 0);
    assert.match(result.stderr, /Pass a contract bundle or set KALSHICORE_ROOT/);
  }
});

test('real generation is deterministic and every missing/tampered output fails without repairs', async () => {
  assert.ok(process.env.KALSHICORE_ROOT, 'Set KALSHICORE_ROOT for the real generation test');
  const bundle = path.join(process.env.KALSHICORE_ROOT, 'contracts/current-official.json');
  const protectedFiles = [...generatedFiles, 'src/typed.rs', 'src/client.rs', 'src/auth.rs', 'src/error.rs', 'src/generated/mod.rs', 'tests/typed_regressions.rs', 'tests/auth_keys.rs', 'README.md', 'Cargo.toml', 'Cargo.lock'];
  const before = await Promise.all(protectedFiles.map(file => readFile(path.join(repositoryRoot, file))));
  const stages = [];
  const originals = await mkdtemp(path.join(tmpdir(), 'kalshi-typed-originals-'));
  try {
    stages.push(await stageGenerated(bundle));
    stages.push(await stageGenerated(bundle));
    assert.equal(generatedFiles.length, 15);
    await compareGenerated(stages[0], stages[1]);
    await compareGenerated(repositoryRoot, stages[0]);
    for (const file of generatedFiles) {
      await mkdir(path.dirname(path.join(originals, file)), {recursive:true});
      await cp(path.join(repositoryRoot, file), path.join(originals, file));
    }
    for (const file of generatedFiles) {
      const target = path.join(originals, file);
      const bytes = await readFile(target);
      const tampered = Buffer.concat([bytes, Buffer.from('\nTAMPERED\n')]);
      await writeFile(target, tampered);
      await assert.rejects(compareGenerated(originals, stages[0]), error => error.message === `Generated output differs: ${file}`);
      assert.deepEqual(await readFile(target), tampered);
      await rm(target);
      await assert.rejects(compareGenerated(originals, stages[0]), error => error.message === `Missing original: ${file}`);
      await assert.rejects(readFile(target), {code:'ENOENT'});
      await writeFile(target, bytes);
    }
    // Exercise the full checker on failure, including generation and cleanup.
    const target = path.join(originals, generatedFiles[0]);
    await writeFile(target, 'tampered original');
    await assert.rejects(checkGenerated(bundle, originals), /Generated output differs/);
    assert.equal(await readFile(target, 'utf8'), 'tampered original');
    await rm(target);
    await assert.rejects(checkGenerated(bundle, originals), /Missing original/);
    await assert.rejects(readFile(target), {code:'ENOENT'});
  } finally {
    for (const stage of stages) await rm(stage, {recursive:true, force:true});
    await rm(originals, {recursive:true, force:true});
    for (const [index, file] of protectedFiles.entries()) {
      assert.deepEqual(await readFile(path.join(repositoryRoot, file)), before[index], `${file} must remain unchanged`);
    }
  }
});
