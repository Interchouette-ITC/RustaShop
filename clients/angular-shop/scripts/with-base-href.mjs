#!/usr/bin/env node
/**
 * Run `ng serve` / `ng build` with a normalized base href.
 *
 * Env (first wins): RUSTASHOP_BASE_HREF, BASE_HREF. Default: `/`.
 * Trailing slash is enforced except for `/` itself.
 *
 * Usage:
 *   node scripts/with-base-href.mjs serve --port 4242
 *   node scripts/with-base-href.mjs build --configuration production
 */
import { spawn } from 'node:child_process';
import { fileURLToPath } from 'node:url';
import path from 'node:path';

function normalizeBaseHref(raw) {
  const value = (raw ?? '/').trim() || '/';
  if (value === '/') {
    return '/';
  }
  return value.endsWith('/') ? value : `${value}/`;
}

const baseHref = normalizeBaseHref(
  process.env.RUSTASHOP_BASE_HREF ?? process.env.BASE_HREF,
);

const [command, ...passthrough] = process.argv.slice(2);
if (!command) {
  console.error('usage: with-base-href.mjs <serve|build|…> [ng args…]');
  process.exit(1);
}

const ngBin = path.join(
  path.dirname(fileURLToPath(import.meta.url)),
  '..',
  'node_modules',
  '@angular',
  'cli',
  'bin',
  'ng.js',
);

const args = [ngBin, command, `--base-href=${baseHref}`, ...passthrough];
process.stderr.write(
  `shop base href → ${baseHref} (set RUSTASHOP_BASE_HREF or BASE_HREF to override)\n`,
);

const child = spawn(process.execPath, args, { stdio: 'inherit' });
child.on('exit', (code, signal) => {
  if (signal) {
    process.kill(process.pid, signal);
    return;
  }
  process.exit(code ?? 1);
});
