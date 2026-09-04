#!/usr/bin/env node
/**
 * Angular host: emit src/generated/*.ng.ts from templates/default (HTML + CSS).
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

const shopRoot = join(dirname(fileURLToPath(import.meta.url)), '..');
const templateRoot = join(shopRoot, '../../templates/default');
const shopComponents = join(shopRoot, 'src/components');
const generatedRoot = join(shopRoot, 'src/generated');
const require = createRequire(join(shopRoot, 'package.json'));
const sass = require('sass');

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
      loadPaths: [templateRoot, join(shopRoot, 'node_modules')],
      style: 'expanded',
    }).css;
  }

  writeFileSync(
    join(generatedRoot, `${name}.ng.ts`),
    `/** Generated from templates/default/${name}/ - do not edit. */\n` +
      `export const template = ${JSON.stringify(html)};\n` +
      `export const styles = [${JSON.stringify(css)}];\n`,
  );

  const staleNg = join(shopComponents, name, `${name}.ng.ts`);
  const staleScss = join(shopComponents, name, `${name}.scss`);
  if (existsSync(staleNg)) unlinkSync(staleNg);
  if (existsSync(staleScss)) unlinkSync(staleScss);

  console.log('emit', name);
}
