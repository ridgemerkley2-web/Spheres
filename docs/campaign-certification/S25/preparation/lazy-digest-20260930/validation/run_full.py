"""Launch one fresh, complete fixed matrix; preserve every attempt and exit."""
from pathlib import Path
import datetime, hashlib, json, subprocess, sys, time

ROOT = Path(__file__).resolve().parent
FROZEN = ROOT / 'frozen'
manifest = json.loads((FROZEN / 'manifest.json').read_text())
for name, pin in manifest['files'].items():
    raw = (FROZEN / name).read_bytes()
    assert len(raw) == pin['bytes'] and hashlib.sha256(raw).hexdigest() == pin['sha256'], name
binary = ROOT / 'spheres-web-test.exe'
assert hashlib.sha256(binary.read_bytes()).hexdigest() == manifest['files']['native-test']['sha256']
pilot = json.loads((ROOT / 'pilot-verification.json').read_text())
assert pilot['integrity_verified'] and pilot['archives_verified'] == 8
out = ROOT.parent / 'full-matrix-local-20260930-01'
scratch = Path('C:/Users/ridge/Documents/Codex/2026-09-05/pick-up-the-spheres-game-on/work/campaign-certification/full-matrix-local-scratch-20260930-01')
args = [sys.executable, '-B', '-X', 'utf8', str(FROZEN / 'stability_matrix.py'),
        '--binary', str(binary), '--revision', manifest['revision'],
        '--plan', str(FROZEN / 'plan.json'), '--out', str(out),
        '--scratch-root', str(scratch), '--compress-scratch',
        '--jobs', '4', '--min-free-bytes', str(3 * 1024 ** 3),
        '--timeout-seconds', '43200']
started = datetime.datetime.now(datetime.timezone.utc).isoformat()
with (ROOT / 'full-launch.json').open('x', encoding='utf-8') as f:
    json.dump({'started_utc': started, 'revision': manifest['revision'], 'args': args,
               'complete_new_plan': True, 'earlier_cells_reused': False,
               'qualification': False, 'resource_note': 'Four native processes, private NTFS-compressed SSD scratch and verified offload to D; 12-hour per-cell execution budget.'}, f, indent=2)
clock = time.monotonic()
with (ROOT / 'full.stdout.log').open('xb') as stdout, (ROOT / 'full.stderr.log').open('xb') as stderr:
    result = subprocess.run(args, cwd=ROOT, stdout=stdout, stderr=stderr)
with (ROOT / 'full-exit.json').open('x', encoding='utf-8') as f:
    json.dump({'exit_code': result.returncode, 'elapsed_seconds': time.monotonic() - clock,
               'finished_utc': datetime.datetime.now(datetime.timezone.utc).isoformat(),
               'result': str(out / 'result.json')}, f, indent=2)
raise SystemExit(result.returncode)
