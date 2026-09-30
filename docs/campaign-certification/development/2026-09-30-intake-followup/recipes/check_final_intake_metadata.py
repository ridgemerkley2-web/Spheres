import concurrent.futures
import datetime
import json
import subprocess
import sys
from pathlib import Path

ROOT = Path(r'C:\Users\ridge\Documents\Codex\2026-09-05\pick-up-the-spheres-game-on\work\campaign-certification\integration')
DEST = ROOT / 'docs/campaign-certification/development/2026-09-30-intake-followup'
OUT = DEST / 'validation'
PYTHON = r'D:\spheres-offload\codex-next-20260928\avatar-ci-clean-venv\Scripts\python.exe'

def run(name, tail):
    args = [PYTHON, '-B', '-X', 'utf8', *tail]
    with (OUT / (name + '.log')).open('xb') as stream:
        result = subprocess.run(args, cwd=ROOT, stdout=stream, stderr=subprocess.STDOUT)
    row = {'name': name, 'command': args, 'exit_code': result.returncode}
    print(json.dumps(row), flush=True)
    return row

results = []
for name, tail in [('post-review-ledger-generate', ['tools/avatars/certified_gap_ledger.py']),
                   ('post-review-boundary-generate', ['tools/avatars/certified_boundary_matrix.py'])]:
    result = run(name, tail)
    results.append(result)
    if result['exit_code']:
        raise SystemExit(result['exit_code'])
checks = [('post-review-research-check', ['tools/avatars/campaign_research.py', '--check']),
          ('post-review-ledger-check', ['tools/avatars/certified_gap_ledger.py', '--check']),
          ('post-review-boundary-check', ['tools/avatars/certified_boundary_matrix.py', '--check']),
          ('post-review-census-check', ['tools/avatars/campaign_census.py', '--check']),
          ('post-review-workboard-check', ['tools/planning/workboard.py', '--check']),
          ('post-review-planning-tests', ['-m', 'unittest', 'discover', '-s', 'tools/planning', '-p', 'test_*.py']),
          ('post-review-certified-tests', ['-m', 'unittest', 'discover', '-s', 'tools/avatars', '-p', 'test_certified_*.py'])]
with concurrent.futures.ThreadPoolExecutor(max_workers=3) as executor:
    results += list(executor.map(lambda x: run(*x), checks))
with (OUT / 'post-review-metadata-results.json').open('x', encoding='utf-8', newline='\n') as stream:
    json.dump({'recorded_utc': datetime.datetime.now(datetime.timezone.utc).isoformat(), 'results': results}, stream, indent=2)
    stream.write('\n')
raise SystemExit(any(row['exit_code'] for row in results))
