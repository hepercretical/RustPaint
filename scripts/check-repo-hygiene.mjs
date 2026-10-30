// Verify the product/test dependency boundary with Git, without a root git init.
import assert from 'node:assert/strict';
import fs from 'node:fs';
import path from 'node:path';
import {spawnSync} from 'node:child_process';
import {fileURLToPath} from 'node:url';

const root = fileURLToPath(new URL('../', import.meta.url));
const scratch = path.join(root, '.tools', 'repo-ignore-check');
fs.mkdirSync(scratch, {recursive: true});
for (const file of ['.gitignore', '.gitattributes']) {
  fs.copyFileSync(path.join(root, file), path.join(scratch, file));
}
function git(directory, args, input) {
  const result = spawnSync('git', ['-c', 'core.excludesFile=NUL', '-C', directory, ...args], {
    input, encoding: 'utf8', maxBuffer: 16 * 1024 * 1024,
  });
  assert.ok(result.status === 0 || (args[0] === 'check-ignore' && result.status === 1),
    result.error?.message || result.stderr || `Git exited ${result.status}`);
  return result.stdout;
}
git(scratch, ['init', '--quiet']);
function ignored(files) {
  if (!files.length) return new Set();
  return new Set(git(scratch, ['check-ignore', '--no-index', '--stdin', '-z'],
    files.join('\0') + '\0').split('\0').filter(Boolean));
}
function walk(relative) {
  const directory = path.join(root, relative);
  if (!fs.existsSync(directory)) return [];
  return fs.readdirSync(directory, {withFileTypes: true}).flatMap(entry => {
    const file = `${relative}/${entry.name}`;
    return entry.isDirectory() ? walk(file) : [file];
  });
}
const keep = new Set([
  'AGENTS.md', 'README.md', '.gitignore', '.gitattributes', 'Cargo.toml',
  'Cargo.lock', 'build.rs', 'scripts/package-desktop.ps1',
  'scripts/audit-desktop-binary.mjs', 'scripts/check-repo-hygiene.mjs',
  '.env.example', '.env.template',
]);
const rustFiles = [...walk('src'), ...walk('tests')].filter(file => file.endsWith('.rs'));
const inputs = new Set();
for (const file of rustFiles) {
  keep.add(file);
  const contents = fs.readFileSync(path.join(root, file), 'utf8');
  for (const match of contents.matchAll(/include_(?:str|bytes)!\(\s*"([^"]+)"\s*\)/g)) {
    const absolute = path.resolve(root, path.dirname(file), match[1]);
    const relative = path.relative(root, absolute).split(path.sep).join('/');
    assert.ok(!relative.startsWith('../'), `External embedded input: ${relative}`);
    assert.ok(fs.existsSync(absolute), `Missing embedded test/product input: ${relative}`);
    keep.add(relative);
    inputs.add(relative);
  }
}
const excludedExamples = [
  'mspaint.c', 'mspaint.exe', 'decompiled/function.c', 'disassembly/function.asm',
  'translation-archive/report.json', 'binary-recovery/export.c',
  'translations/binary-recovery/verification.json', 'translations/remaining.json',
  'translations/declarations-030.json', 'translations/batch-040.json',
  'translations/batch-999.json', 'tests/fixtures/batch-999.csv',
  'tests/fixtures/unused-capture.csv',
  'scripts/generate-declarations30.mjs', 'scripts/RecoverGuard40.java',
  'scripts/audit-progress.mjs', 'scripts/old-harness.c.inc',
  'tests/fixtures/declarations30.csv', 'tests/fixtures/declarations31.csv',
  '.tools/cargo/bin/cargo.exe', 'recovery-projects/PaintRecovery.gpr',
  'recovery-projects/PaintRecovery.rep/data', 'target/debug/editor.exe',
  'nested/target/cache', 'dist/package.zip', '.cache/state', 'coverage/index.html',
  'tmp/reference.c', 'temp/output.txt', 'scratch/capture.png', 'logs/session.txt',
  'node_modules/dependency/index.js', '.venv/pyvenv.cfg', '__pycache__/helper.pyc',
  '.vscode/settings.json', '.idea/workspace.xml', 'Thumbs.db', 'Desktop.ini',
  'native/editor.pdb', 'native/editor.obj', 'crash.dmp', 'editor.stackdump',
  'build.log', 'drawing.tmp', 'file.bak', '.rust-paint-123-0.png',
  '.env', '.env.local', '.aws/credentials', '.ssh/id_ed25519',
];
// Check every actual archived script/export too, so newly added unused files
// cannot silently become GitHub payload just because they miss an example.
const archives = [...walk('scripts'), ...walk('translations'), ...walk('tests/fixtures')]
  .filter(file => !keep.has(file));
const exclude = [...new Set([...excludedExamples, ...archives])];
const actual = ignored([...keep, ...exclude]);
for (const file of keep) assert.ok(!actual.has(file), `Required file ignored: ${file}`);
for (const file of exclude) assert.ok(actual.has(file), `Unused/local file trackable: ${file}`);

let trackedExcluded = [];
if (fs.existsSync(path.join(root, '.git'))) {
  const tracked = git(root, ['ls-files', '-z']).split('\0').filter(Boolean);
  trackedExcluded = [...ignored(tracked)];
  assert.equal(trackedExcluded.length, 0,
    `Ignored files are still tracked; .gitignore does not untrack them: ${trackedExcluded.join(', ')}`);
}
// Count actual project files separately from hypothetical regression examples.
// Toolchains, build/release outputs, caches and Ghidra project trees are already
// ignored wholesale and are outside this source/documentation/archive inventory.
const projectFiles = [
  ...fs.readdirSync(root, {withFileTypes: true})
    .filter(entry => entry.isFile() && entry.name !== '.git').map(entry => entry.name),
  ...['src', 'tests', 'scripts', 'translations', '.vscode'].flatMap(walk),
];
const actualIgnored = ignored(projectFiles);
console.log(JSON.stringify({
  required_files_checked: keep.size,
  embedded_inputs_checked: inputs.size,
  excluded_files_checked: exclude.length,
  tracked_excluded_files: trackedExcluded.length,
  actual_project_files: projectFiles.length,
  ignored_project_files: actualIgnored.size,
  trackable_project_files: projectFiles.length - actualIgnored.size,
  root_repository_created: false,
  passed: true,
}, null, 2));
