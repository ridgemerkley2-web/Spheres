import datetime
import hashlib
import json
import subprocess
from pathlib import Path

ROOT = Path(r'C:\Users\ridge\Documents\Codex\2026-09-05\pick-up-the-spheres-game-on\work\campaign-certification\integration')
EXTERNAL = Path(r'D:\spheres-offload\codex-next-20260928')
DEST = ROOT / 'docs/campaign-certification/development/2026-09-30-intake-followup'

def pin(path, name):
    raw = path.read_bytes()
    return {'path': name, 'bytes': len(raw), 'sha256': hashlib.sha256(raw).hexdigest()}

def write_new(path, raw):
    path.parent.mkdir(parents=True, exist_ok=True)
    with path.open('xb') as out:
        out.write(raw)

queue_path = ROOT / 'docs/planning/ai-task-queue.json'
queue = json.loads(queue_path.read_bytes())
tasks = queue['tasks']
entry = next(row for row in tasks if row['id'] == 'CLAUDE-C01-28')
entry['review_revision'] = '031bd0e3'  # Resolve the actual imported receipt below.
entry['review_revision'] = subprocess.check_output(['git', 'rev-parse', entry['review_revision']], cwd=ROOT).decode().strip()
entry['next'] = ('Independent review remains held on original-source access. The unchanged initial receipt records 56/68 exact originals and twelve unavailable originals. Addendum 031bd0e3 completes all accessible content: 113/137 claims and 20/24 holder observations checked; 24 claims and four observations remain held. No accessible claim remains deferred. Two narrow uncertainty repairs are retained only on the unmerged review branch at 35ea76f6; 39 Russia tests and refreshed index pass there. No new Russia research is imported or accepted. See docs/campaign-certification/C01/reviews/CLAUDE-C01-28/2026-09-30-available-content/README.md. No historical or parent closure.')
queue_path.write_text(json.dumps(queue, indent=2, ensure_ascii=False) + '\n', encoding='utf-8', newline='\n')

review = ROOT / 'docs/campaign-certification/C01/reviews/CLAUDE-C01-28/2026-09-30-available-content'
checked = []
for row in json.loads((review / 'manifest.json').read_bytes())['evidence']:
    assert pin(review / row['path'], row['path']) == row
    checked.append(row)
write_new(DEST / 'validation/russia-available-addendum-root-verification.json', (json.dumps({'status': 'pass', 'scope': 'Imported receipt payload byte identity only; Russia research remains unaccepted', 'receipt_revision': entry['review_revision'], 'checked': checked}, indent=2) + '\n').encode())

armed = EXTERNAL / 'full-matrix-dependent-verification-20260930-01'
manifest = json.loads((armed / 'artifact-manifest.json').read_bytes())
for row in manifest['files']:
    source = armed / row['relative']
    assert source.resolve().is_relative_to(armed.resolve())
    current = pin(source, row['relative'])
    assert current['bytes'] == row['bytes'] and current['sha256'] == row['sha256']
    write_new(DEST / 'dependent-verifier-armed' / row['relative'], source.read_bytes())
write_new(DEST / 'dependent-verifier-armed/artifact-manifest.json', (armed / 'artifact-manifest.json').read_bytes())
write_new(DEST / 'dependent-verifier-armed/README.md', (armed / 'README.md').read_bytes()) if not (DEST / 'dependent-verifier-armed/README.md').exists() else None
write_new(DEST / 'integration-history-addendum.json', (json.dumps({'parent_snapshot': 'integration-history.json', 'imported_receipt': entry['review_revision'], 'original_receipt_only_commit': '0dec078f747575f12eb851b15dc97602e2a06bfe', 'research_correction_not_imported': '35ea76f65723f2213c893bf8338d1b146508dd0a'}, indent=2) + '\n').encode())
write_new(DEST / 'recipes/finalize_intake_followup.py', Path(__file__).read_bytes())
print(json.dumps({'russia_payloads_verified': len(checked), 'immutable_arming_payloads_verified': len(manifest['files']), 'queue_updated': entry['review_revision']}))
