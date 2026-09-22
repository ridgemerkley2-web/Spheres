"""Read-only source comparison; never runs the Node test suite or a generator."""
import argparse, collections, hashlib, json, subprocess, time
from datetime import datetime, timezone
from pathlib import Path

ap = argparse.ArgumentParser()
ap.add_argument('--out', required=True)
ap.add_argument('--expected-dyads-sha', required=True)
ap.add_argument('--selection', required=True)
a = ap.parse_args()
root = Path.cwd()
out = Path(a.out).resolve()
out.mkdir(parents=True, exist_ok=False)
base = root / 'docs/campaign-certification/verification/evidence/2026-09-22-completion/checkpoint26-validation'
sha = lambda b: hashlib.sha256(b).hexdigest()
def load(p): return json.loads(p.read_text(encoding='utf-8-sig'))
def git(*args): return subprocess.check_output(['git', *args], cwd=root, text=True, encoding='utf-8').strip()
node = load(base / 'node-result.json')
manifest = load(base / 'source-manifest-9f7faed5.json')
archive = {r['path']: r for r in load(base / 'manifest.json')['files']}
receipt_names = ['node-result.json', 'source-manifest-9f7faed5.json', 'completion-checkpoint26-node-20260922.log']
for name in receipt_names:
    assert sha((base / name).read_bytes()) == archive[name]['sha256'], name
assert sha((base / receipt_names[-1]).read_bytes()) == node['log']['sha256']
assert (node['passed'], node['failed'], node['skipped']) == (1701, 0, 0)
expected = sorted(r['path'] for r in node['input_files_at_receipt'] if r['path'] != 'tools/ui/run-unit.cjs')
current = sorted(p.relative_to(root).as_posix() for p in (root / 'tools/ui').iterdir()
                 if p.name.startswith('check_') and p.name.endswith('.cjs') and '_browser' not in p.name)
assert current == expected
rows = []
for r in node['input_files_at_receipt']:
    data = (root / r['path']).read_bytes()
    rows.append({'path': r['path'], 'before_sha256': r['sha256'],
                 'current_sha256': sha(data), 'byte_identical': sha(data) == r['sha256']})
assert all(r['byte_identical'] for r in rows)
assert not any('dyads' in (root / r['path']).read_text(encoding='utf-8') for r in rows)
groups = collections.defaultdict(lambda: {'files': 0, 'bytes': 0, 'identical': 0})
changes = []
started = time.monotonic()
for r in manifest['files']:
    p = root / r['path']
    before = p.stat()
    with p.open('rb') as stream:
        h = hashlib.file_digest(stream, 'sha256').hexdigest()
    after = p.stat()
    assert (before.st_size, before.st_mtime_ns) == (after.st_size, after.st_mtime_ns), r['path']
    identical = h == r['raw_sha256']
    g = groups['/'.join(r['path'].split('/')[:2])]
    g['files'] += 1
    g['bytes'] += r['raw_bytes']
    g['identical'] += int(identical)
    if not identical:
        changes.append({'path': r['path'], 'before_sha256': r['raw_sha256'], 'current_sha256': h,
                        'before_bytes': r['raw_bytes'], 'current_bytes': after.st_size})
assert len(changes) == 1 and changes[0]['path'] == 'spheres-sim/src/dyads.rs'
assert changes[0]['current_sha256'] == a.expected_dyads_sha
code_diff = git('diff', '--name-only', node['revision'], '--', 'tools', 'spheres-web', 'spheres-sim', 'Cargo.lock', 'Cargo.toml').splitlines()
assert code_diff == ['spheres-sim/src/dyads.rs'], code_diff
helper_diff = git('diff', '--name-only', node['revision'], '--', 'tools/ui', 'tools/arsenal').splitlines()
assert not helper_diff
version = subprocess.check_output(['node', '--version'], text=True).strip()
assert version == node['node']
result = {
    'scope': 'Read-only applicability comparison, not a test rerun or certification of the Rust optimization.',
    'selection': a.selection, 'created_utc': datetime.now(timezone.utc).isoformat(),
    'baseline_revision': node['revision'], 'current_git_head': git('rev-parse', 'HEAD'),
    'working_tree_diff_paths': git('diff', '--name-only', node['revision']).splitlines(),
    'comparison_helper_sha256': sha(Path(__file__).read_bytes()),
    'baseline_result': {k: node[k] for k in ['command', 'node', 'exit_code', 'passed', 'failed', 'skipped', 'duration_ms', 'test_files']},
    'baseline_receipts': [{'path': str(base / n), 'sha256': sha((base / n).read_bytes())} for n in receipt_names],
    'direct_node_inputs': {'files': len(rows), 'all_raw_byte_identical': True, 'files_added_or_removed': False, 'rows': rows},
    'broad_source_manifest_comparison': {'files': len(manifest['files']), 'raw_bytes': sum(r['raw_bytes'] for r in manifest['files']),
       'identical_files': len(manifest['files']) - len(changes), 'changed_files': changes, 'categories': dict(groups), 'seconds': round(time.monotonic() - started, 3)},
    'additional_tooling_inputs': {'method': 'Git tracked-content comparison, not a claim of raw historical hashes for helpers absent the source manifest.', 'paths': ['tools/ui', 'tools/arsenal'], 'changed_paths': helper_diff},
    'native_dependency_review': {'test_source_dyads_mentions': 0, 'native_test_execution': False,
        'description': 'The unchanged runner selects the same107 tests. Reviewed test subprocess sites launch Node code/generator checks, not Cargo or a live native simulation.'},
    'current_node_version': version,
    'conclusion': 'Checkpoint26 1701-pass Node result applies to unchanged Node/test/generated inputs without rerun. All107 tests plus runner and1932 broad runtime/data/UI files are raw-byte identical. Sole difference is unconsumed native dyads.rs. Native, browser, performance and political validation remain separately receipted.'
}
(out / 'result.json').write_text(json.dumps(result, indent=2) + '\n', encoding='utf-8', newline='\n')
print(json.dumps({'result': str(out / 'result.json'), 'selection': a.selection, 'node_input_files': len(rows), 'raw_identical_files': len(manifest['files']) - 1, 'dyads': changes[0]['current_sha256']}))
