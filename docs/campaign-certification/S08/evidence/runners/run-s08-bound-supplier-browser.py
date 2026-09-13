"""Bind an isolated supplier browser run to actual qualified producer phases."""
import datetime, hashlib, json, os, pathlib, re, subprocess, sys

base = pathlib.Path(__file__).resolve().parent
pin, native_name, build_name, export_name, label = sys.argv[1:]
assert re.fullmatch('[0-9a-f]{40}', pin) and re.fullmatch('[A-Za-z0-9_-]+', label)
def read(p): return json.loads(pathlib.Path(p).read_text(encoding='utf-8-sig'))
def sha(p):
    with pathlib.Path(p).open('rb') as stream: return hashlib.file_digest(stream, 'sha256').hexdigest()
native_path, build_path, export_path = [pathlib.Path(p).resolve() for p in (native_name, build_name, export_name)]
native, build, export = map(read, (native_path, build_path, export_path))
for proof in (native, build):
    assert proof['passed'] and proof['clean_before'] and proof['clean_after']
    assert proof['revision_before'] == proof['revision_after'] == pin
    assert sha(proof['binary']) == proof['binary_sha256']
assert export['passed'] and export['integrity_passed'] and export['candidate'] == pin
assert export['test_binary_sha256_before'] == export['test_binary_sha256_after'] == native['binary_sha256']
phases = []
for suffix in ('-ready-before-purchase.campaign.json', '0360-progress.campaign.json'):
    matches = [p for p in export['archive_phases'] if p['file'].endswith(suffix)]
    assert len(matches) == 1
    item = matches[0]
    path = pathlib.Path(export['export_directory']) / item['file']
    assert sha(path) == item['sha256'] and path.stat().st_size == item['bytes']
    phases.append(dict(item, path=str(path)))
deal = export['native_outcome']['contract']
assert deal['seller'] == 'France' and deal['source_revision']['spec']['platform'] == 'ground_apc'
out = base / ('evidence/S08-supplier-browser-' + label)
out.mkdir(exist_ok=False)
env = dict(os.environ)
removed = sorted(k for k in env if k.startswith('SPHERES_SUPPLIER_') or k == 'SPHERES_AUDIT_PYTHON')
for key in removed: env.pop(key)
bound = {
    'SPHERES_SUPPLIER_SAVE': phases[0]['path'],
    'SPHERES_SUPPLIER_PREPARATION_SAVE': phases[1]['path'],
    'SPHERES_SUPPLIER_PROVENANCE': str(export_path),
    'SPHERES_SUPPLIER_OUTPUT': str(out),
    'SPHERES_SUPPLIER_SLOT': 's08-earned-stock',
    'SPHERES_SUPPLIER_MAX_DAYS': '120',
    'SPHERES_AUDIT_PYTHON': sys.executable,
    'SPHERES_SUPPLIER_OFFER': f"supplier:{deal['seller']}:{deal['company']}:equipment:{deal['product']}",
}
env.update(bound)
record = {'candidate': pin, 'started_utc': datetime.datetime.now(datetime.timezone.utc).isoformat(), 'runner_sha256': sha(__file__), 'proofs': [{'path': str(p), 'sha256': sha(p)} for p in (native_path, build_path, export_path)], 'phases': phases, 'bound_environment': bound, 'removed_environment_keys': removed}
with (out/'input-binding.json').open('x', encoding='utf-8') as stream: json.dump(record, stream, indent=2)
command = [sys.executable, str(base/'run-s08-final.py'), pin, 'supplier-browser', 'supplier-browser-'+label]
result = subprocess.run(command, cwd=base, env=env)
record.update(exit_code=result.returncode, finished_utc=datetime.datetime.now(datetime.timezone.utc).isoformat(), command=command)
record['inputs_unchanged'] = all(sha(p['path']) == p['sha256'] for p in phases) and all(sha(p['path']) == p['sha256'] for p in record['proofs'])
record['passed'] = result.returncode == 0 and record['inputs_unchanged']
with (out/'binding-result.json').open('x', encoding='utf-8') as stream: json.dump(record, stream, indent=2)
print(json.dumps({'passed': record['passed'], 'binding_result': str(out/'binding-result.json')}), flush=True)
sys.exit(0 if record['passed'] else 1)
