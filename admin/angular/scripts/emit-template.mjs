#!/usr/bin/env node
/**
 * Admin Angular host: emit generated/*.ng.ts from templates/admin/default.
 */
import { createRequire } from 'node:module';
import {
  readdirSync,
  readFileSync,
  writeFileSync,
  mkdirSync,
  existsSync,
  unlinkSync,
} from 'node:fs';
import { dirname, join } from 'node:path';
import { fileURLToPath } from 'node:url';
import { assertTemplateKind } from '../../../templates/scripts/assert-template-kind.mjs';

const adminRoot = join(dirname(fileURLToPath(import.meta.url)), '..');
const templateRoot = join(adminRoot, '../../templates/admin/default');
const adminComponents = join(adminRoot, 'src/components');
const generatedRoot = join(adminRoot, 'generated');
const require = createRequire(join(adminRoot, 'package.json'));
const sass = require('sass');

assertTemplateKind(templateRoot, 'admin');

mkdirSync(generatedRoot, { recursive: true });
writeFileSync(join(generatedRoot, '.gitkeep'), '');

for (const name of readdirSync(templateRoot)) {
  const htmlPath = join(templateRoot, name, `${name}.html`);
  const scssPath = join(templateRoot, name, `${name}.scss`);
  if (!existsSync(htmlPath)) continue;

  const html = readFileSync(htmlPath, 'utf8');
  let css = '';
  if (existsSync(scssPath)) {
    css = sass.compile(scssPath, {
      loadPaths: [templateRoot, join(adminRoot, 'node_modules')],
      style: 'expanded',
    }).css;
  }

  writeFileSync(
    join(generatedRoot, `${name}.ng.ts`),
    `/** Generated from templates/admin/default/${name}/ - do not edit. */\n` +
      `export const template = ${JSON.stringify(html)};\n` +
      `export const styles = [${JSON.stringify(css)}];\n`,
  );

  const staleNg = join(adminComponents, name, `${name}.ng.ts`);
  const staleScss = join(adminComponents, name, `${name}.scss`);
  if (existsSync(staleNg)) unlinkSync(staleNg);
  if (existsSync(staleScss)) unlinkSync(staleScss);

  console.log('emit', name);
}
