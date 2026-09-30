import datetime
import hashlib
import json
import shutil
import subprocess
from pathlib import Path

ROOT = Path(r'C:\Users\ridge\Documents\Codex\2026-09-05\pick-up-the-spheres-game-on\work\campaign-certification\integration')
EXTERNAL = Path(r'D:\spheres-offload\codex-next-20260928')
DEST = ROOT / 'docs/campaign-certification/development/2026-09-30-intake-followup'

def pin(path, name):
    raw = path.read_bytes()
    return {'path': name, 'bytes': len(raw), 'sha256': hashlib.sha256(raw).hexdigest()}

def copy_exact(source, destination):
    destination.parent.mkdir(parents=True, exist_ok=True)
    with destination.open('xb') as output:
        output.write(source.read_bytes())
    assert source.read_bytes() == destination.read_bytes()

assert not DEST.exists(), 'Create-only evidence packet already exists'
DEST.mkdir(parents=True)
(DEST / '.gitattributes').write_bytes(b'* -text -whitespace\n')
for source in sorted((EXTERNAL / 'combined-intakes-34-37-validation').iterdir()):
    if source.is_file():
        copy_exact(source, DEST / 'validation' / source.name)
audit = EXTERNAL / 'integration-intakes-34-37-audit-01'
for row in json.loads((audit / 'manifest.json').read_bytes())['files']:
    assert pin(audit / row['path'], row['path']) == row
for source in sorted(audit.rglob('*')):
    if source.is_file():
        copy_exact(source, DEST / 'independent-34-37-audit' / source.relative_to(audit))
recipes = ['accept_reviewed_intakes_34_35.py', 'verify_new_intake_receipts.py',
           'update_intake_board_34_37.py', 'update_japan_acceptance.py',
           'import_japan_retained_correction.py', 'package_intake_followup.py']
for name in recipes:
    copy_exact(EXTERNAL / name, DEST / 'recipes' / name)
history = subprocess.check_output(['git', 'log', '--reverse', '--format=%H%x09%P%x09%s', '3947d1f1..HEAD'], cwd=ROOT).decode('utf-8')
commits = [dict(zip(('revision', 'parents', 'subject'), line.split('\t', 2))) for line in history.splitlines()]
(DEST / 'integration-history.json').write_text(json.dumps({'base': '3947d1f1d34fba93dc81c476a97a6e52edf0c8b8',
    'through': subprocess.check_output(['git', 'rev-parse', 'HEAD'], cwd=ROOT).decode().strip(), 'commits': commits}, indent=2, ensure_ascii=False) + '\n', encoding='utf-8', newline='\n')
scratch = ROOT.parent / 'full-matrix-local-scratch-20260930-01/cells'
progress = []
for cell in sorted(scratch.iterdir()):
    path = cell / 'native/progress.jsonl'
    if path.exists():
        rows = path.read_bytes().splitlines()
        if rows:
            progress.append({'cell': cell.name, 'last_recorded_progress': json.loads(rows[-1])})
(DEST / 'matrix-progress-snapshot.json').write_text(json.dumps({'observed_utc': datetime.datetime.now(datetime.timezone.utc).isoformat(),
    'candidate': '68ba0622ec709b78617aadd1f9198d18f532bb32', 'full_matrix_pass_claimed': False,
    'qualification': False, 'progress': progress}, indent=2) + '\n', encoding='utf-8', newline='\n')
# Final README and any dependent-job launch receipt are added before finalizing the manifest.
print(json.dumps({'prepared': str(DEST), 'copied_files': sum(p.is_file() for p in DEST.rglob('*'))}))
