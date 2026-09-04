#!/usr/bin/env node
/**
 * Generate OpenAPI TypeScript types into src/app/api/schema.d.ts.
 * Locates repo-root openapi/openapi.json by walking parents (no hardcoded ../..).
 */

import { spawnSync } from 'node:child_process';
import { existsSync } from 'node:fs';
import { dirname, join } from 'node:path';
import { fileURLToPath } from 'node:url';
import { performance } from 'node:perf_hooks';

const packageDir = join(dirname(fileURLToPath(import.meta.url)), '..');
const outRel = 'src/app/api/schema.d.ts';
const outFile = join(packageDir, outRel);

function findOpenApiSpec(startDir) {
  let dir = startDir;
  for (;;) {
    const candidate = join(dir, 'openapi', 'openapi.json');
    if (existsSync(candidate)) {
      return candidate;
    }
    const parent = dirname(dir);
    if (parent === dir) {
      return null;
    }
    dir = parent;
  }
}

const spec = findOpenApiSpec(packageDir);
if (!spec) {
  console.error(
    'openapi/openapi.json not found above this package. Run `make openapi` from the repo root first.',
  );
  process.exit(1);
}

const bin = join(packageDir, 'node_modules', '.bin', 'openapi-typescript');
const started = performance.now();
const result = spawnSync(bin, [spec, '-o', outFile], {
  encoding: 'utf8',
  stdio: ['ignore', 'pipe', 'pipe'],
});
if (result.status !== 0) {
  if (result.stderr) {
    process.stderr.write(result.stderr);
  }
  if (result.stdout) {
    process.stdout.write(result.stdout);
  }
  process.exit(result.status === null ? 1 : result.status);
}

const ms = (performance.now() - started).toFixed(1);
console.log(`openapi  openapi.json → ${outRel}  (${ms}ms)`);
process.exit(0);
