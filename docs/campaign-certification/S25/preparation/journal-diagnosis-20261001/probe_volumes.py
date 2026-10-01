"""Probe only fresh task-owned directories; never reopen retained campaign files."""
import datetime
import hashlib
import json
import os
from pathlib import Path
import sys
import traceback


stamp = datetime.datetime.now(datetime.timezone.utc).strftime('%Y%m%dT%H%M%S%fZ')
destination = Path(__file__).with_name('volume-probe.json')
if destination.exists():
    raise SystemExit('Evidence already exists; copy this recipe to a fresh output directory before rerunning.')
parents = [
    Path('D:/spheres-offload/codex-next-20260928'),
    Path('C:/Users/ridge/Documents/Codex/2026-09-05/pick-up-the-spheres-game-on/work/campaign-certification'),
]
rows = []
for parent in parents:
    root = parent / ('s25-journal-probe-' + stamp)
    root.mkdir(exist_ok=False)
    journal = root / 'journal.jsonl'
    stage = 'open'
    expected = b''
    row = {'directory': str(root), 'started_utc': datetime.datetime.now(datetime.timezone.utc).isoformat()}
    try:
        with journal.open('xb') as stream:
            for event in ('opened', 'second_append'):
                body = (json.dumps({'event': event, 'probe_only': True}) + '\n').encode()
                stage = 'write'
                stream.write(body)
                expected += body
                stage = 'flush'
                stream.flush()
                stage = 'fsync'
                os.fsync(stream.fileno())
                stage = 'read_while_writer_open'
                assert journal.read_bytes() == expected
            stage = 'close'
        stage = 'read_after_close'
        observed = journal.read_bytes()
        assert observed == expected
        row.update(passed=True, bytes=len(observed), sha256=hashlib.sha256(observed).hexdigest())
    except Exception as error:
        row.update(passed=False, operation=stage, error=type(error).__name__, message=str(error),
                   errno=getattr(error, 'errno', None), winerror=getattr(error, 'winerror', None),
                   traceback=traceback.format_exc())
    row['finished_utc'] = datetime.datetime.now(datetime.timezone.utc).isoformat()
    rows.append(row)
result = {'format': 'spheres-s25-disposable-volume-probe/v1', 'python': sys.executable,
          'python_version': sys.version, 'passed': all(r['passed'] for r in rows), 'probes': rows,
          'scope': 'Immediate fresh-handle write/flush/fsync/read/close only; does not reproduce a14-hour handle lifetime or prove the old failure cause. No retained campaign file touched.'}
with destination.open('x', encoding='utf8') as stream:
    json.dump(result, stream, indent=2)
    stream.write('\n')
print(json.dumps(result, indent=2))
raise SystemExit(0 if result['passed'] else 1)
