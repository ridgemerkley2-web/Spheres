"""Read-only failure reconciliation; never launches or repairs a campaign.

Run with --out NEW_DIRECTORY. Source paths are the exact frozen local attempt.
The original completed-cell audit is reused, not rerun as a new simulation.
"""
import argparse
from datetime import datetime, timezone
import gzip
import hashlib
import json
from pathlib import Path
import subprocess

ROOT = Path('D:/spheres-offload/codex-next-20260928/full-matrix-local-20260930-01')
LAUNCH = ROOT.parent / 'lazy-digest-validation-20260930'
DEPENDENT = ROOT.parent / 'full-matrix-dependent-verification-20260930-01'
REV = '68ba0622ec709b78617aadd1f9198d18f532bb32'
SNAPSHOT = Path(__file__).resolve().parent.parent / 'snapshot.json'

def read(path):
    return json.loads(path.read_bytes())

def pin(path):
    with path.open('rb') as stream:
        digest = hashlib.file_digest(stream, 'sha256').hexdigest()
    return {'path': str(path), 'bytes': path.stat().st_size, 'sha256': digest}

def verify(path, expected):
    actual = pin(path)
    if any(actual[k] != expected[k] for k in ('bytes', 'sha256')):
        raise ValueError(f'Changed evidence: {path}')
    return actual

def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('--out', type=Path, required=True)
    out = parser.parse_args().out.resolve()
    assert not out.exists() and not out.is_relative_to(ROOT.resolve())
    out.mkdir(parents=True)
    copied = []
    def capture(path, relative):
        dest = out / relative
        dest.parent.mkdir(parents=True, exist_ok=True)
        raw = path.read_bytes()
        with dest.open('xb') as stream:
            stream.write(raw)
        source = pin(path)
        verify(dest, source)
        copied.append({'relative': relative, 'source': source})
    def write(name, data):
        (out / name).write_text(json.dumps(data, indent=2) + '\n', encoding='utf-8')

    freeze = read(ROOT / 'freeze.json')
    assert freeze['candidate_revision'] == REV and freeze['scope'] == 'full'
    for key in ['binary', 'plan', 'frozen_plan', 'harness', 'frozen_harness']:
        verify(Path(freeze[key]['path']), freeze[key])
    snapshot = read(SNAPSHOT)
    assert snapshot['candidate_revision'] == REV
    for row in snapshot['copied_metadata']:
        verify(Path(row['source']), row)
    events = [json.loads(s) for s in (ROOT / 'journal.jsonl').read_bytes().splitlines()]
    starts = {r['id'] for r in events if r['event'] == 'cell_started'}
    finishes = {r['id'] for r in events if r['event'] == 'cell_finished'}
    assert len(starts) == 8 and len(finishes) == 4
    assert not any(r['event'] == 'matrix_finished' for r in events)
    assert not (ROOT / 'result.json').exists()
    assert not (DEPENDENT / 'verification-invocation.json').exists()
    assert not (DEPENDENT / 'verify.stdout.json').exists()
    for name in ['freeze.json', 'plan.json', 'resource-setup.json', 'journal.jsonl']:
        capture(ROOT / name, 'run/' + name)
    for name in ['full-launch.json', 'full-exit.json', 'full.stdout.log', 'full.stderr.log']:
        capture(LAUNCH / name, 'launcher/' + name)
    for name in ['result.json', 'events.jsonl', 'pending.json', 'config.json']:
        capture(DEPENDENT / name, 'dependent/' + name)

    failures, retained = [], []
    archive_count = 0
    for cell in read(ROOT / 'plan.json')['cells']:
        name = cell['id']
        if name not in starts or name in finishes:
            continue
        base = ROOT / 'cells' / name
        transfer = read(base / 'transfer.json')
        assert transfer['destination'] == str(base)
        listed = {row['relative'] for row in transfer['files']}
        actual = {str(p.relative_to(base)).replace('\\', '/') for p in base.rglob('*') if p.is_file()}
        assert listed | {'transfer.json'} == actual
        for row in transfer['files']:
            path = (base / row['relative']).resolve()
            assert path.is_relative_to(base.resolve())
            assert Path(row['retained']['path']).resolve() == path
            retained.append(verify(path, row['retained']))
            assert all(row['original'][k] == row['retained'][k] for k in ('bytes', 'sha256'))
        retained.append(pin(base / 'transfer.json'))
        for row in read(base / 'archive-manifest.json')['archives']:
            path = base / 'native' / row['gzip_relative']
            verify(path, row['gzip'])
            h, count = hashlib.sha256(), 0
            with gzip.open(path, 'rb') as stream:
                while block := stream.read(1024 * 1024):
                    h.update(block)
                    count += len(block)
            assert {'bytes': count, 'sha256': h.hexdigest()} == row['decoded']
            archive_count += 1
        execution = read(base / 'execution.json')
        native = read(base / 'native/result.json')
        assert execution['exit_code'] != 0 and execution['elapsed_seconds'] < 43200
        assert execution['binary_before'] == execution['binary_after'] == freeze['binary']
        assert native['revision'] == REV and native['id'] == name and native['passed'] is False
        assert native['end_native_date'] < '2036-01-01'
        assert not (base / 'verdict.json').exists()
        failures.append({'id': name, 'exit_code': execution['exit_code'],
                         'finished_utc': execution['finished_utc'],
                         'elapsed_seconds': execution['elapsed_seconds'],
                         'last_reported_date': native['end_native_date'],
                         'days_each_leg': native['days_each_leg'],
                         'partial_comparisons': len(native['comparisons']),
                         'native_report_passed': False, 'native_report_failure': native['failure']})
        for relative in ['execution.json', 'request.json', 'transfer.json', 'archive-manifest.json',
                         'native/result.json', 'native/progress.jsonl', 'stdout.log', 'stderr.log']:
            capture(base / relative, 'cells/' + name + '/' + relative)

    process_command = "@(Get-CimInstance Win32_Process | Where-Object { $_.Name -match 'python|spheres-web-test' -and $_.CommandLine -match 'full-matrix-local|dependent_verify|run_full.py|lazy-digest-validation-20260930|s25_stability_cell' } | Select-Object ProcessId,ParentProcessId,Name,CreationDate,CommandLine) | ConvertTo-Json -Depth 3 -Compress"
    process = subprocess.run(['powershell.exe', '-NoProfile', '-Command', process_command],
                             check=True, capture_output=True, text=True)
    live = json.loads(process.stdout) if process.stdout.strip() else []
    assert live == [], live
    cells = [{**c, 'status': ('prior_pass_revalidated' if c['id'] in finishes else
                             'abnormal_exit_incomplete' if c['id'] in starts else 'not_started')}
             for c in read(ROOT / 'plan.json')['cells']]
    receipt = {'format': 'spheres-matrix-interruption-reconciliation/v1',
               'observed_utc': datetime.now(timezone.utc).isoformat(),
               'candidate_revision': REV, 'source': str(ROOT),
               'counts': {'prior_pass_revalidated': 4, 'abnormal_exit_incomplete': 4, 'not_started': 16, 'running': 0},
               'cells': cells, 'failures': failures, 'launcher': read(LAUNCH/'full-exit.json'),
               'dependent_verifier': read(DEPENDENT/'result.json'),
               'matching_active_processes': live, 'prior_snapshot': pin(SNAPSHOT),
               'prior_metadata_pins_rechecked': len(snapshot['copied_metadata']),
               'new_retained_files_rehashed': retained, 'new_gzip_archives_decoded_and_verified': archive_count,
               'copied_metadata': copied, 'native_reexecuted': False, 'full_matrix_passed': False,
               'qualification': False, 's25_complete': False,
               'scope': 'Failure and byte-integrity reconciliation, not full retained campaign verification. Prior four archive audits are referenced, not needlessly repeated.'}
    write('reconciliation.json', receipt)
    print(json.dumps({k: receipt[k] for k in ['counts','failures','prior_metadata_pins_rechecked','new_gzip_archives_decoded_and_verified']}, indent=2))

if __name__ == '__main__':
    main()
