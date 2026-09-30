import datetime
import hashlib
import json
import subprocess
from pathlib import Path

ROOT = Path(r'C:\Users\ridge\Documents\Codex\2026-09-05\pick-up-the-spheres-game-on\work\campaign-certification\integration')
EXTERNAL = Path(r'D:\spheres-offload\codex-next-20260928')
DEST = ROOT / 'docs/campaign-certification/development/2026-09-30-intake-followup'

def new(path, raw):
    with path.open('xb') as out:
        out.write(raw)

for name in ('check_final_intake_metadata.py', 'seal_intake_followup.py'):
    new(DEST / 'recipes' / name, (EXTERNAL / name).read_bytes())
check = subprocess.run(['git', '-c', 'core.safecrlf=false', 'diff', '--check', '3947d1f1'], cwd=ROOT, capture_output=True)
new(DEST / 'validation/post-review-diff-check.log', check.stdout + check.stderr)
assert check.returncode == 0
payloads = []
for path in sorted(DEST.rglob('*')):
    if not path.is_file():
        continue
    assert path.name != 'manifest.json' or path.parent != DEST
    raw = path.read_bytes()
    payloads.append({'path': path.relative_to(DEST).as_posix(), 'bytes': len(raw), 'sha256': hashlib.sha256(raw).hexdigest()})
manifest = {'format': 'spheres-intake-followup/v1', 'sealed_utc': datetime.datetime.now(datetime.timezone.utc).isoformat(),
            'scope': 'Five bounded intake decisions, held Russia review addendum, local validation, CI repair, and pending full-matrix verifier arming',
            'original_tasks_complete': 6, 'original_tasks_total': 8, 'full_matrix_pass_claimed': False,
            'a1_pass_claimed': False, 'qualification_claimed': False, 'files': payloads}
new(DEST / 'manifest.json', (json.dumps(manifest, indent=2) + '\n').encode())
print(json.dumps({'payloads': len(payloads), 'bytes': sum(row['bytes'] for row in payloads), 'manifest_sha256': hashlib.sha256((DEST / 'manifest.json').read_bytes()).hexdigest()}))
