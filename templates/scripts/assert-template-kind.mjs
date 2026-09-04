#!/usr/bin/env node
/**
 * Refuse to emit when package.json rustashop.templateKind does not match the host.
 */
import { readFileSync } from 'node:fs';
import { join } from 'node:path';

/**
 * @param {string} templateRoot absolute path to templates/<kind>/<id>
 * @param {'shop' | 'admin'} expected
 */
export function assertTemplateKind(templateRoot, expected) {
  const pkgPath = join(templateRoot, 'package.json');
  let pkg;
  try {
    pkg = JSON.parse(readFileSync(pkgPath, 'utf8'));
  } catch (err) {
    throw new Error(`Cannot read ${pkgPath}: ${err instanceof Error ? err.message : String(err)}`);
  }
  const kind = pkg?.rustashop?.templateKind;
  if (kind !== expected) {
    throw new Error(
      `Template kind mismatch at ${templateRoot}: expected "${expected}", got ${JSON.stringify(kind)}. ` +
        `Use templates/shop/<id> for shops and templates/admin/<id> for admin hosts.`,
    );
  }
}
