"""Verify immutable DA acceptance evidence and its scoped patch; no checkout writes."""
import argparse
import hashlib
import json
import os
from pathlib import Path
import subprocess
import tempfile

RECEIPT = 'docs/campaign-certification/C01/reviews/CLAUDE-C01-39/2026-10-01-accepted-03'
EXTERNAL_PREFIX = 'D:/spheres-offload/codex-next-20260928/'
p = argparse.ArgumentParser(description=__doc__)
p.add_argument('--root', type=Path)
p.add_argument('--receipt-revision', default='HEAD', help='Use : for staged receipt.')
p.add_argument('--verify-external', action='store_true')
p.add_argument('--external-root', type=Path)
a = p.parse_args()
root = a.root or Path(subprocess.check_output(['git', 'rev-parse', '--show-toplevel'], text=True).strip())

def git(*args, env=None):
    return subprocess.check_output(['git', *args], cwd=root, env=env)

def blob(rev, path):
    return git('show', ':' + path if rev == ':' else rev + ':' + path)

def data(name):
    return json.loads(blob(a.receipt_revision, RECEIPT + '/' + name))

def check(raw, row):
    assert len(raw) == row['bytes'] and hashlib.sha256(raw).hexdigest() == row['sha256'], row['path']

manifest = data('manifest.json')
for row in manifest['files']:
    check(blob(a.receipt_revision, RECEIPT + '/' + row['path']), row)
pins = data('reviewed-git-blobs.json')
for row in pins:
    check(blob(row['revision'], row['path']), row)
    assert git('rev-parse', row['revision'] + ':' + row['path']).decode().strip() == row['git_blob']
review = data('review.json')
assert review['packet_accepted'] is True and review['canonical_import_performed'] is False
sources, claims, holders = [data(n + '-review.json') for n in ('source', 'claim', 'holder')]
assert (len(sources), len(claims), len(holders)) == (24, 29, 12)
assert all(s['exact_original_reproduced'] and s['actual_body_sha256'] == s['expected_sha256'] and s['actual_body_bytes'] == s['expected_bytes'] for s in sources)
assert all(c['status'] == 'materially_supported' for c in claims)
assert all(h['status'] == 'materially_supported' and h['accepted_for_integration'] for h in holders)
recovered = data('retrieval.json')
assert recovered['http_status'] == 200 and recovered['redirect_count'] == recovered['retries'] == 0
assert recovered['requested_url'] == recovered['effective_url']
assert '/20210311042629id_/' in recovered['requested_url']
assert recovered['actual_bytes'] == 134450 and recovered['actual_sha256'] == '730846b64891bac5bcd0fe512300a18b2413d9e8274fce780ff7027d04d5e32c'
files = data('reviewed-source-files.json')
assert len(files) == 33
assert all(x['path'] not in data('scope-review.json')['excluded_from_import'] for x in files)
with tempfile.TemporaryDirectory(prefix='c01-da-receipt-') as temp:
    env = os.environ.copy()
    env['GIT_INDEX_FILE'] = str(Path(temp) / 'index')
    git('read-tree', review['base'], env=env)
    patch = Path(temp) / 'reviewed.patch'
    patch.write_bytes(blob(a.receipt_revision, RECEIPT + '/reviewed-scoped-import.patch'))
    git('apply', '--cached', '--check', '--whitespace=error', str(patch), env=env)
    git('apply', '--cached', str(patch), env=env)
    changed = git('diff', '--cached', '--name-only', review['base'], env=env).decode().splitlines()
    assert set(changed) == {x['path'] for x in files}
    for row in files:
        check(git('show', ':' + row['path'], env=env), row)
    old = json.loads(blob(review['base'], 'docs/campaign-certification/C01/research/south-africa.json'))
    new = json.loads(git('show', ':docs/campaign-certification/C01/research/south-africa.json', env=env))
    assert old['sources'] == new['sources'][:245] and old['institutions'] == new['institutions']
    assert [o for o in old['organizations'] if o['id'] != 'za_iec_n2024_027'] == [o for o in new['organizations'] if o['id'] != 'za_iec_n2024_027']
    def role(packet):
        org = next(o for o in packet['organizations'] if o['id'] == 'za_iec_n2024_027')
        return next(r for r in org['roles'] if r['id'] == 'za_da_federal_leader')
    assert len(role(old)['holder_claims']) == 2 and len(role(new)['holder_claims']) == 14
    assert all(h in role(new)['holder_claims'] for h in role(old)['holder_claims'])
    authored = json.loads(blob(review['submission'], 'docs/campaign-certification/C01/research/south-africa.json'))
    assert new['organizations'] == authored['organizations'] and new['institutions'] == authored['institutions']
checked = 0
if a.verify_external or a.external_root:
    for row in data('external-files.json'):
        original = row['path'].replace('\\', '/')
        assert original.startswith(EXTERNAL_PREFIX), original
        path = a.external_root / original[len(EXTERNAL_PREFIX):] if a.external_root else Path(original)
        check(path.read_bytes(), row)
        checked += 1
print(json.dumps({'receipt_payloads': len(manifest['files']), 'reviewed_git_blobs': len(pins),
                  'reviewed_patch_files': len(files), 'external_files_checked': checked,
                  'exact_originals': 24, 'supported_claims': 29, 'supported_new_holders': 12,
                  'accepted_for_scoped_integration': True, 'canonical_import_performed': False}))
