"""Run the merged offline preparation checks; does not award qualification."""
import argparse
from datetime import datetime, timezone
import hashlib
import importlib.metadata
import json
import os
from pathlib import Path
import subprocess
import sys
import time


def pin(path, root):
    raw = path.read_bytes()
    return {'path': path.relative_to(root).as_posix(), 'bytes': len(raw),
            'sha256': hashlib.sha256(raw).hexdigest(), 'hash_scope': 'raw'}


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('--root', type=Path, default=Path(__file__).resolve().parents[4])
    parser.add_argument('--output', type=Path, required=True)
    args = parser.parse_args()
    root, out = args.root.resolve(), args.output.resolve()
    if out.exists():
        parser.error('Use a new directory; keep previous evidence unchanged.')
    out.mkdir(parents=True)
    env = {**os.environ, 'PYTHONDONTWRITEBYTECODE': '1', 'PYTHONIOENCODING': 'utf-8'}
    scripts = [
        ('census', ['tools/avatars/campaign_census.py', '--check']),
        ('research-index', ['tools/avatars/campaign_research.py', '--check']),
        ('gap-ledger', ['tools/avatars/certified_gap_ledger.py', '--check']),
        ('cartoon-export', ['tools/avatars/cartoon_review.py', '--check']),
        ('boundary-matrix', ['tools/avatars/certified_boundary_matrix.py', '--check']),
        ('successor-proposals', ['tools/avatars/check_successor_proposals.py']),
        ('workboard', ['tools/planning/workboard.py', '--check']),
        ('avatar-suite', ['-m', 'unittest', 'discover', '-s', 'tools/avatars', '-p', 'test_*.py']),
        ('company-pilot', ['tools/planning/check_company_pilot.py']),
        ('company-tests', ['-m', 'unittest', 'discover', '-s', 'tools/planning', '-p', 'test_company_pilot.py']),
        ('playtest-tests', ['-m', 'unittest', 'discover', '-s', 'tools/playtests', '-p', 'test_*.py']),
    ]
    paths = subprocess.check_output(['git', 'ls-files', 'tools/avatars', 'tools/ui', 'tools/planning',
        'tools/playtests', 'docs/research/company-pilot', 'docs/planning',
        'docs/campaign-certification/C01/research', 'docs/campaign-certification/C04/preparation',
        'spheres-web/src/main.rs', 'spheres-web/ui/index.html'], cwd=root, text=True).splitlines()
    record = {'scope': 'Merged offline tooling and research checks, not S24/S26/E05/CP1 qualification.',
              'started_utc': datetime.now(timezone.utc).isoformat(), 'python': sys.version,
              'pypdf_version': importlib.metadata.version('pypdf'),
              'head_before_checks': subprocess.check_output(['git', 'rev-parse', 'HEAD'], cwd=root, text=True).strip(),
              'working_inputs': [pin(root / p, root) for p in paths if (root / p).is_file()], 'checks': []}
    for name, command in scripts:
        argv = [sys.executable, '-X', 'utf8', *command]
        tick = time.monotonic()
        result = subprocess.run(argv, cwd=root, env=env, stdout=subprocess.PIPE, stderr=subprocess.STDOUT)
        log = out / (name + '.log')
        log.write_bytes(result.stdout)
        record['checks'].append({'name': name, 'command': argv, 'returncode': result.returncode,
                                 'seconds': round(time.monotonic() - tick, 3), 'log': pin(log, out)})
        print(name + ': ' + ('PASS' if result.returncode == 0 else 'FAIL'), flush=True)
        (out / 'validation.json').write_text(json.dumps(record, indent=2) + '\n', encoding='utf-8', newline='\n')
    record['inputs_unchanged'] = all(pin(root / row['path'], root) == row for row in record['working_inputs'])
    record['passed'] = record['inputs_unchanged'] and all(c['returncode'] == 0 for c in record['checks'])
    record['finished_utc'] = datetime.now(timezone.utc).isoformat()
    (out / 'validation.json').write_text(json.dumps(record, indent=2) + '\n', encoding='utf-8', newline='\n')
    return 0 if record['passed'] else 1


if __name__ == '__main__':
    raise SystemExit(main())
