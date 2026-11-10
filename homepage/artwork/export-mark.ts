/** Export the approved native Blender mark with the current Ferramenta icon policy. */
import { spawnSync } from 'node:child_process';
import { createHash } from 'node:crypto';
import { mkdtempSync, readFileSync, writeFileSync, rmSync } from 'node:fs';
import { tmpdir } from 'node:os';
import { join } from 'node:path';
import { fileURLToPath } from 'node:url';

const master = process.argv[2];
const family = process.argv[3];
if (!master || !family) {
  throw new Error('Usage: node homepage/artwork/export-mark.ts MASTER.png /path/to/ferramenta');
}
const output = fileURLToPath(new URL('../public/', import.meta.url));
const recordPath = fileURLToPath(new URL('./ferrixel-mark.provenance.json', import.meta.url));
const tokenPath = join(family, 'design/ferra/tokens.json');
const sha = (path: string) => createHash('sha256').update(readFileSync(path)).digest('hex');
if (sha(tokenPath) !== '1db5c57209f4cec921d2b303f548e7f2dfa112fb81e6b14a43b7621c0733f725') {
  throw new Error('Unexpected Ferramenta palette tokens');
}
const paper = JSON.parse(readFileSync(tokenPath, 'utf8')).palette.paper;
const run = (command: string, args: string[]) => {
  const result = spawnSync(command, args, { encoding: 'utf8' });
  if (result.status !== 0) throw new Error(`${command}: ${result.stderr || result.stdout}`);
  return result.stdout.trim();
};
const source = JSON.parse(readFileSync(`${master}.json`, 'utf8'));
if (source.masterSha256 !== sha(master)) throw new Error('Native master hash mismatch');
const construction = fileURLToPath(new URL('./ferrixel-mark.py', import.meta.url));
if (source.constructionSha256 !== sha(construction)) throw new Error('Native construction hash mismatch');
const bounds = run('magick', [master, '-alpha', 'extract', '-threshold', '10%', '-trim', '-format', '%wx%h%X%Y', 'info:']);
const scratch = mkdtempSync(join(tmpdir(), 'ferrixel-mark-export-'));
const files: Record<string, string> = {};
try {
  for (const size of [32, 48, 64, 256, 512]) {
    const target = size === 512 ? join(output, 'ferrixel-mark-512.png') : join(scratch, `mark-${size}.png`);
    run('magick', [master, '-crop', bounds, '+repage', '-background', 'none', '-gravity', 'center',
      '-extent', '%[fx:ceil(max(w,h)/0.875)]x%[fx:ceil(max(w,h)/0.875)]',
      '-filter', 'Lanczos', '-resize', `${size}x${size}`, target]);
    if (size === 512) files['ferrixel-mark-512.png'] = sha(target);
  }
  const webp = join(output, 'ferrixel-mark.webp');
  run('magick', [join(scratch, 'mark-256.png'), '-define', 'webp:lossless=true', webp]);
  files['ferrixel-mark.webp'] = sha(webp);
  const ico = join(output, 'favicon.ico');
  run('magick', [32, 48, 64].map((size) => join(scratch, `mark-${size}.png`)).concat(ico));
  files['favicon.ico'] = sha(ico);
  const touch = join(output, 'apple-touch-icon.png');
  run('magick', [join(scratch, 'mark-256.png'), '-resize', '156x156', '-background', paper,
    '-gravity', 'center', '-extent', '180x180', '-alpha', 'remove', touch]);
  files['apple-touch-icon.png'] = sha(touch);
  source.export = {
    pipeline: '10% alpha bounds; 12.5% transparent padding; Lanczos resize; 32/48/64 ICO; 256 lossless WebP; 512 PNG; 180 paper touch icon',
    alphaBounds: bounds,
    familyTokensSha256: sha(tokenPath),
    paper,
    imageMagick: run('magick', ['-version']).split('\n')[0],
    exporterSha256: sha(fileURLToPath(import.meta.url)),
    files,
  };
  writeFileSync(recordPath, `${JSON.stringify(source, null, 2)}\n`);
  console.log(`Exported raster loupe mark: ${Object.keys(files).join(', ')}`);
} finally {
  rmSync(scratch, { recursive: true, force: true });
}
