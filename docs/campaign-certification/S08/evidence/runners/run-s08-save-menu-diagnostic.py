"""Read-only save-list diagnosis on an existing disposable supplier directory."""
import datetime, hashlib, json, pathlib, socket, subprocess, time, urllib.request

base = pathlib.Path(__file__).resolve().parent
pin = 'd770592aeead51f1313d507edd26b02d75a69bba'
run = (base/'evidence/S08-supplier-browser-corrected-route-pool-1/supplier-v6VJe7/server').resolve()
assert run.is_relative_to((base/'evidence').resolve()) and run.name == 'server'
out = base/'evidence/S08-save-menu-list-diagnostic-1'
out.mkdir(exist_ok=False)
proof = json.loads((base/'evidence/S08-final-binary-route-pool-1.json').read_text())
binary = pathlib.Path(proof['binary'])
assert proof['passed'] and proof['revision_after'] == pin
def sha(path):
    with path.open('rb') as stream: return hashlib.file_digest(stream, 'sha256').hexdigest()
assert sha(binary) == proof['binary_sha256']
def snapshot(): return [{'file': p.name, 'bytes': p.stat().st_size, 'sha256': sha(p)} for p in sorted((run/'saves').glob('*.json'))]
before = snapshot()
with socket.socket() as probe:
    probe.bind(('127.0.0.1', 0)); port = probe.getsockname()[1]
url = f'http://127.0.0.1:{port}'
record = {'runtime': pin, 'binary_sha256': proof['binary_sha256'], 'directory': str(run), 'url': url, 'started_utc': datetime.datetime.now(datetime.timezone.utc).isoformat(), 'scope': 'Only GET /api/build and one GET /api/saves against existing disposable files; no save/load/command/advance or source edits.', 'before': before}
process = None
try:
    with (out/'server.log').open('xb') as log:
        process = subprocess.Popen([str(binary), '--port', str(port), '--no-open'], cwd=run, stdout=log, stderr=log, creationflags=subprocess.CREATE_NO_WINDOW)
        for _ in range(100):
            assert process.poll() is None
            try:
                with urllib.request.urlopen(url+'/api/build', timeout=2) as response: build = json.load(response)
                break
            except OSError: time.sleep(.1)
        else: raise RuntimeError('Server startup timed out')
        assert build['revision'] == pin[:12]
        record['build'] = build
        started = time.monotonic()
        with urllib.request.urlopen(url+'/api/saves', timeout=240) as response: payload = response.read()
        record['list_seconds'] = time.monotonic()-started
        (out/'actual-response.json').write_bytes(payload)
        record['response_sha256'] = hashlib.sha256(payload).hexdigest()
        record['response_bytes'] = len(payload)
        record['response'] = json.loads(payload)
except BaseException as error:
    record['error'] = str(error)
finally:
    if process is not None and process.poll() is None: process.terminate(); process.wait(timeout=10)
    record['after'] = snapshot()
    record['files_unchanged'] = record['after'] == before
    record['finished_utc'] = datetime.datetime.now(datetime.timezone.utc).isoformat()
    with (out/'result.json').open('x', encoding='utf-8') as stream: json.dump(record, stream, indent=2)
    print(json.dumps({k: record.get(k) for k in ['list_seconds', 'response_bytes', 'files_unchanged', 'error']}), flush=True)
