"""Build only the unchanged release library test in an isolated target directory."""
from datetime import datetime, timezone
import hashlib
import json
import os
from pathlib import Path
import shutil
import subprocess
import sys
import time

OUT = Path(__file__).resolve().parent
ROOT = next(p for p in OUT.parents if (p / '.git').exists())
if len(sys.argv) > 1:
    OUT = OUT / sys.argv[1]
    OUT.mkdir(exist_ok=False)
TARGET = Path('D:/spheres-offload/codex-next-20260928/cp1-a1-followup-20261001-target')

def git(*args):
    return subprocess.check_output(['git', *args], cwd=ROOT).decode().strip()

def sha(path):
    with path.open('rb') as stream:
        return hashlib.file_digest(stream, 'sha256').hexdigest()

def write(name, obj):
    (OUT / name).write_text(json.dumps(obj, indent=2) + '\n', encoding='utf-8')

assert not git('diff', '7c6f112c', 'HEAD', '--', 'spheres-sim', 'Cargo.toml', 'Cargo.lock')
assert not git('diff', 'HEAD', '--', 'spheres-sim', 'Cargo.toml', 'Cargo.lock')
assert shutil.disk_usage('D:/').free > 5_000_000_000
assert not TARGET.exists(), 'Use a new private target directory, preserving all old builds.'
source_paths = git('ls-tree', '-r', '--name-only', 'HEAD', '--', 'spheres-sim', 'Cargo.toml', 'Cargo.lock',
                   'spheres-web/data/district_population.json', 'docs/research').splitlines()
source_pins = [{'path': p, 'git_blob': git('rev-parse', 'HEAD:' + p), 'working_sha256': sha(ROOT / p)}
               for p in source_paths if (ROOT / p).is_file()]
preflight = {
    'head': git('rev-parse', 'HEAD'), 'accepted_runtime': git('rev-parse', '7c6f112c'),
    'sim_tree': git('rev-parse', 'HEAD:spheres-sim'), 'accepted_sim_tree': git('rev-parse', '7c6f112c:spheres-sim'),
    'cargo_target_dir': str(TARGET), 'free_bytes': {d: shutil.disk_usage(d + ':/').free for d in ('C', 'D')},
    'rustc': subprocess.check_output(['rustc', '-Vv']).decode(),
    'cargo': subprocess.check_output(['cargo', '-V']).decode(),
    'recorded_utc': datetime.now(timezone.utc).isoformat(), 'source_pins': source_pins,
    'reason_for_fresh_build': 'The previous final library-test executable was not pinned in the original frozen-binary manifest. A new isolated build removes that provenance ambiguity.'
}
write('build-preflight.json', preflight)
env = dict(os.environ, CARGO_TARGET_DIR=str(TARGET))
command = ['cargo', 'test', '--locked', '--release', '-p', 'spheres-sim', '--lib', '--no-run', '-j', '2']
start = time.monotonic()
with (OUT / 'build.log').open('wb') as stream:
    result = subprocess.run(command, cwd=ROOT, env=env, stdout=stream, stderr=subprocess.STDOUT)
receipt = {'command': command, 'cwd': str(ROOT), 'target': str(TARGET), 'exit_code': result.returncode,
           'elapsed_seconds': time.monotonic() - start, 'log_sha256': sha(OUT / 'build.log'),
           'preflight_sha256': sha(OUT / 'build-preflight.json'), 'binaries': []}
for path in (TARGET / 'release/deps').glob('spheres_sim-*.exe'):
    receipt['binaries'].append({'path': str(path), 'bytes': path.stat().st_size, 'sha256': sha(path)})
receipt['source_pins_unchanged'] = all(sha(ROOT / p['path']) == p['working_sha256'] for p in source_pins)
write('build-result.json', receipt)
print(json.dumps({k: v for k, v in receipt.items() if k != 'command'}))
raise SystemExit(result.returncode)
