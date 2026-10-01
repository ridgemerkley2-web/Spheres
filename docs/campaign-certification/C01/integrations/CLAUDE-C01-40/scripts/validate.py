"""Record focused offline checks; green checks do not grant historical approval."""
import concurrent.futures
import datetime
import hashlib
import json
import pathlib
import subprocess
import sys
import time

ROOT = pathlib.Path(__file__).resolve().parents[6]
OUT = pathlib.Path(__file__).resolve().parents[1] / 'validation'
OUT.mkdir(exist_ok=True)
PY = [sys.executable, '-B', '-X', 'utf8']
checks = [
    ('india', PY + ['-m', 'unittest', 'discover', '-s', 'tools/avatars', '-p', 'test_india*.py']),
    ('research', PY + ['-m', 'unittest', 'discover', '-s', 'tools/avatars', '-p', 'test_*research*.py']),
    ('campaign', PY + ['-m', 'unittest', 'discover', '-s', 'tools/avatars', '-p', 'test_campaign*.py']),
    ('research-index', PY + ['tools/avatars/campaign_research.py', '--check']),
    ('census', PY + ['tools/avatars/campaign_census.py', '--check']),
    ('atlas', ['node', '--test', 'tools/ui/check_leadership_research_review.cjs']),
    ('workboard', PY + ['tools/planning/workboard.py', '--check']),
    ('whitespace', ['git', 'diff', '--check']),
]


def run(item):
    name, command = item
    started = datetime.datetime.now(datetime.timezone.utc).isoformat()
    clock = time.monotonic()
    result = subprocess.run(command, cwd=ROOT, stdout=subprocess.PIPE, stderr=subprocess.STDOUT)
    path = OUT / (name + '.log')
    if path.exists():
        raise RuntimeError('Refusing to overwrite retained check: ' + str(path))
    path.write_bytes(result.stdout)
    row = {'name': name, 'command': command, 'started_utc': started,
           'seconds': round(time.monotonic() - clock, 3), 'exit_code': result.returncode,
           'log': path.name, 'bytes': len(result.stdout),
           'sha256': hashlib.sha256(result.stdout).hexdigest()}
    print(json.dumps(row), flush=True)
    return row


with concurrent.futures.ThreadPoolExecutor(max_workers=3) as pool:
    results = list(pool.map(run, checks))
(OUT / 'results.json').write_text(json.dumps({
    'head': subprocess.check_output(['git', 'rev-parse', 'HEAD'], cwd=ROOT, text=True).strip(),
    'working_tree_review_repairs_present': True,
    'checks': results,
    'caution': 'Discovery patterns overlap; execution counts are not unique-test totals. Source-content reading remains independent.'
}, indent=2) + '\n', encoding='utf-8')
raise SystemExit(any(r['exit_code'] for r in results))
