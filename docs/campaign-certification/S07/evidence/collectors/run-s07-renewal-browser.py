"""Run an ignored startup-and-renewal candidate against the exact clean S07 binary.

Usage: python run-s07-renewal-browser.py PIN {tonga,browser} UNIQUE_STEM
No build, source edit or candidate generation happens here.
"""
import datetime, hashlib, json, os, pathlib, re, shutil, subprocess, sys, time

base = pathlib.Path(__file__).resolve().parent
repo = base / 'integration'
revision, lane, label = sys.argv[1:]
assert lane == 'tonga', 'This second candidate qualifies Tonga renewal only'
assert re.fullmatch(r'[A-Za-z0-9_-]+', label), 'Use a simple unique evidence stem'
evidence = base / 'evidence' / 'S07-renewal-harness'
stem = evidence / label
assert not stem.with_suffix('.log').exists() and not stem.with_suffix('.json').exists()
def git(*args):
    return subprocess.check_output(['git', *args], cwd=repo, text=True).strip()
def sha(path):
    return hashlib.sha256(path.read_bytes()).hexdigest()
assert git('rev-parse', 'HEAD') == revision and not git('status', '--porcelain')
manifest = json.loads((evidence / 'candidates.json').read_text(encoding='utf-8'))
assert manifest['revision'] == revision
name = 'ci-construction.cjs' if lane == 'tonga' else 'ci-browser.cjs'
entry = next(row for row in manifest['sources'] if row['source'] == 'tools/ui/' + name)
candidate = pathlib.Path(entry['candidate'])
wrapper = candidate.parent / 'run.cjs'
assert sha(repo / entry['source']) == entry['checkout_sha256']
assert sha(candidate) == entry['candidate_sha256']
binary = base / 'integration-target/release/spheres-web.exe'
assert sha(binary) == manifest['binary_sha256'], 'Pinned binary changed'
env = os.environ.copy()
env.update(SPHERES_BINARY=str(binary), SPHERES_EXPECTED_REVISION=revision, SPHERES_BROWSER_CHANNEL='chrome')
if lane == 'tonga':
    output = repo / 'artifacts/s07-renewal-harness' / (label + '-construction')
    assert not output.exists(), 'Refusing to replace construction artifacts'
    env['SPHERES_CONSTRUCTION_NATION'] = 'Tonga'
    env['SPHERES_CONSTRUCTION_OUTPUT'] = str(output)
else:
    # ci-browser's existing output path is unchanged. Preserve its old top-level
    # results/screenshots before it writes, and archive the new files afterward.
    output = repo / 'artifacts/browser-ci'
    prior = evidence / (label + '-prior-browser-artifacts')
    assert not prior.exists()
    prior.mkdir()
    if output.exists():
        for item in output.iterdir():
            if item.is_file(): shutil.copy2(item, prior / item.name)
command = ['node', str(wrapper), lane]
record = {'revision_before': revision, 'clean_before': True, 'lane': lane,
    'command': command, 'candidate': str(candidate), 'candidate_sha256': sha(candidate),
    'source_sha256': sha(repo / entry['source']), 'wrapper_sha256': sha(wrapper),
    'runner_sha256': sha(pathlib.Path(__file__)), 'manifest_sha256': sha(evidence / 'candidates.json'),
    'binary': str(binary), 'binary_sha256_before': sha(binary),
    'browser_channel': 'chrome', 'artifact_root': str(output),
    'started_utc': datetime.datetime.now(datetime.timezone.utc).isoformat()}
started = time.monotonic()
with stem.with_suffix('.log').open('xb') as log:
    result = subprocess.run(command, cwd=repo, env=env, stdout=log, stderr=subprocess.STDOUT,
        creationflags=subprocess.CREATE_NO_WINDOW if os.name == 'nt' else 0)
record.update(exit_code=result.returncode, wall_seconds=time.monotonic()-started,
    finished_utc=datetime.datetime.now(datetime.timezone.utc).isoformat(),
    revision_after=git('rev-parse', 'HEAD'), clean_after=not git('status', '--porcelain'),
    binary_sha256_after=sha(binary), candidate_sha256_after=sha(candidate),
    source_sha256_after=sha(repo / entry['source']), log_sha256=sha(stem.with_suffix('.log')))
if lane == 'browser':
    archived = evidence / (label + '-browser-artifacts')
    archived.mkdir()
    for item in output.iterdir():
        if item.is_file(): shutil.copy2(item, archived / item.name)
    record['archived_top_level_artifacts'] = str(archived)
record['preservation_passed'] = (record['revision_after'] == revision and record['clean_after']
    and record['binary_sha256_after'] == record['binary_sha256_before']
    and record['candidate_sha256_after'] == record['candidate_sha256']
    and record['source_sha256_after'] == record['source_sha256'])
stem.with_suffix('.json').write_text(json.dumps(record, indent=2) + '\n', encoding='utf-8')
print(json.dumps(record, indent=2), flush=True)
sys.exit(result.returncode or (0 if record['preservation_passed'] else 1))
