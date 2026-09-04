#!/usr/bin/env node
/**
 * Generate OpenAPI TypeScript types into src/app/api/schema.d.ts.
 * Locates repo-root openapi/openapi.json by walking parents (no hardcoded ../..).
 */

import { spawnSync } from 'node:child_process';
import { existsSync } from 'node:fs';
import { dirname, join } from 'node:path';
import { fileURLToPath } from 'node:url';

const packageDir = join(dirname(fileURLToPath(import.meta.url)), '..');
const outFile = join(packageDir, 'src', 'app', 'api', 'schema.d.ts');

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
const result = spawnSync(bin, [spec, '-o', outFile], { stdio: 'inherit' });
process.exit(result.status === null ? 1 : result.status);
