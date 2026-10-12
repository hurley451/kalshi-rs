#!/usr/bin/env node
import { mkdtemp, readFile, rm } from 'node:fs/promises';
import { tmpdir } from 'node:os';
import path from 'node:path';
import { fileURLToPath } from 'node:url';
import { spawnSync } from 'node:child_process';

export const repositoryRoot = fileURLToPath(new URL('../', import.meta.url));
export const generatedFiles = Object.freeze([
  'src/generated/typed-schema.json',
  'src/generated/models.rs',
  'src/generated/typed.rs',
  'src/generated/domains.rs',
  'tests/typed_signatures.rs',
  'tests/typed_wire.rs',
  'tests/typed_errors.rs',
  'tests/typed_pagination.rs',
  'tests/typed_models.rs',
  'tests/typed-fixtures.json',
  'tests/typed-model-fixtures.json',
  'tests/typed-model-full-fixtures.json',
  'tools/typed-coverage.json',
  'tools/typed-model-manifest.json',
]);

function run(script, args, cwd) {
  const result = spawnSync(process.execPath, [path.join(repositoryRoot, script), ...args], {cwd, stdio:'inherit'});
  if (result.error) throw result.error;
  if (result.status !== 0) throw Error(`${script} failed (${result.status ?? result.signal})`);
}

export async function stageGenerated(bundlePath) {
  const stage = await mkdtemp(path.join(tmpdir(), 'kalshi-typed-check-'));
  try {
    run('tools/generate_typed.mjs', [path.resolve(bundlePath), '--out-dir', stage], repositoryRoot);
    run('tools/probe-fixtures/generate.mjs', [], stage);
    return stage;
  } catch (error) {
    await rm(stage, {recursive:true, force:true});
    throw error;
  }
}

export async function compareGenerated(originalRoot, stage) {
  const problems = [];
  for (const file of generatedFiles) {
    let expected;
    let actual;
    try { expected = await readFile(path.join(originalRoot, file)); }
    catch (error) {
      if (error.code !== 'ENOENT') throw error;
      problems.push(`Missing original: ${file}`);
    }
    try { actual = await readFile(path.join(stage, file)); }
    catch (error) {
      if (error.code !== 'ENOENT') throw error;
      problems.push(`Missing staged output: ${file}`);
    }
    if (expected && actual && !expected.equals(actual)) problems.push(`Generated output differs: ${file}`);
  }
  if (problems.length) throw Error(problems.join('\n'));
}

export async function checkGenerated(bundlePath, originalRoot = repositoryRoot) {
  const stage = await stageGenerated(bundlePath);
  try {
    await compareGenerated(originalRoot, stage);
    console.log(`All ${generatedFiles.length} generated outputs match; originals were not written.`);
  } finally {
    await rm(stage, {recursive:true, force:true});
  }
}

if (process.argv[1] && path.resolve(process.argv[1]) === fileURLToPath(import.meta.url)) {
  const bundlePath = process.argv[2] ?? (process.env.KALSHICORE_ROOT && path.join(process.env.KALSHICORE_ROOT, 'contracts/current-official.json'));
  try {
    if (!bundlePath || process.argv.length > 3) throw Error('Pass a contract bundle or set KALSHICORE_ROOT.');
    await checkGenerated(bundlePath);
  } catch (error) {
    console.error(error.message);
    process.exitCode = 1;
  }
}
