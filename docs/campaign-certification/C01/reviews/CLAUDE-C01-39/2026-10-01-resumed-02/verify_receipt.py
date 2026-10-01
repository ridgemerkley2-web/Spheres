"""Verify exact Git and retained-file identities; this does not confer acceptance."""
import argparse
from datetime import datetime
import hashlib
import json
from pathlib import Path
import subprocess

RECEIPT = 'docs/campaign-certification/C01/reviews/CLAUDE-C01-39/2026-10-01-resumed-02'
EXTERNAL_PREFIX = 'D:/spheres-offload/codex-next-20260928/'
parser = argparse.ArgumentParser(description=__doc__)
parser.add_argument('--root', type=Path)
parser.add_argument('--receipt-revision', default='HEAD', help='Use : to verify the staged receipt before its commit.')
parser.add_argument('--verify-external', action='store_true')
parser.add_argument('--external-root', type=Path, help='Relocated equivalent of the common evidence parent.')
args = parser.parse_args()
root = args.root or Path(subprocess.check_output(['git', 'rev-parse', '--show-toplevel'], text=True).strip())


def git_blob(revision, path):
    spec = ':' + path if revision == ':' else revision + ':' + path
    return subprocess.check_output(['git', 'show', spec], cwd=root)


def receipt_json(name):
    return json.loads(git_blob(args.receipt_revision, RECEIPT + '/' + name))


manifest = receipt_json('manifest.json')
for row in manifest['files']:
    raw = git_blob(args.receipt_revision, RECEIPT + '/' + row['path'])
    assert len(raw) == row['bytes'] and hashlib.sha256(raw).hexdigest() == row['sha256'], row['path']
pins = receipt_json('reviewed-git-blobs.json')
for row in pins:
    raw = git_blob(row['revision'], row['path'])
    assert len(raw) == row['bytes'] and hashlib.sha256(raw).hexdigest() == row['sha256'], row['path']
    oid = subprocess.check_output(['git', 'rev-parse', row['revision'] + ':' + row['path']], cwd=root, text=True).strip()
    assert oid == row['git_blob']
review = receipt_json('review.json')
assert review['packet_accepted'] is False and review['country_import'] is False
sources, claims, holders = [receipt_json(n + '-review.json') for n in ('source', 'claim', 'holder')]
assert (len(sources), len(claims), len(holders)) == (24, 29, 12)
assert sum(s['exact_original_reproduced'] for s in sources) == 23
assert sum(c['status'] == 'materially_supported' for c in claims) == 28
assert sum(h['status'] == 'materially_supported' for h in holders) == 11
assert all(not h['accepted_for_integration'] for h in holders)
held = next(s for s in sources if not s['exact_original_reproduced'])
assert held['source_id'] == 'za_da_statement_stellenbosch_visit_20210310'
assert '/20210311042629id_/' in held['requested_url'] and '/20230315100631id_/' in held['effective_url']
assert held['actual_body_sha256'] != held['expected_sha256']
journal = receipt_json('source-attempts-resumed.json')
assert len(journal['attempts']) == 23 and journal['not_attempted'] == []
assert len({a['url'] for a in journal['attempts']}) == 23
assert journal['retained_original_not_requested'] not in {a['source_id'] for a in journal['attempts']}
for before, after in zip(journal['attempts'], journal['attempts'][1:]):
    assert (datetime.fromisoformat(after['started_utc']) - datetime.fromisoformat(before['finished_utc'])).total_seconds() >= 15
assert all(a['http_status'] == '200' for a in journal['attempts'])
repairs = receipt_json('proposed-locator-corrections.json')
assert len(repairs) == 3 and all(not r['applied'] for r in repairs)
old = json.loads(git_blob(review['base'], 'docs/campaign-certification/C01/research/south-africa.json'))
new = json.loads(git_blob(review['submission'], 'docs/campaign-certification/C01/research/south-africa.json'))
assert old['sources'] == new['sources'][:245] and old['institutions'] == new['institutions']
assert [o for o in old['organizations'] if o['id'] != 'za_iec_n2024_027'] == [o for o in new['organizations'] if o['id'] != 'za_iec_n2024_027']
old_da = next(o for o in old['organizations'] if o['id'] == 'za_iec_n2024_027')
new_da = next(o for o in new['organizations'] if o['id'] == 'za_iec_n2024_027')
old_role = next(r for r in old_da['roles'] if r['id'] == 'za_da_federal_leader')
new_role = next(r for r in new_da['roles'] if r['id'] == 'za_da_federal_leader')
assert len(old_role['holder_claims']) == 2 and all(h in new_role['holder_claims'] for h in old_role['holder_claims'])
checked = 0
if args.verify_external or args.external_root:
    for row in receipt_json('external-files.json'):
        original = row['path'].replace('\\', '/')
        assert original.startswith(EXTERNAL_PREFIX), original
        path = args.external_root / original[len(EXTERNAL_PREFIX):] if args.external_root else Path(original)
        raw = path.read_bytes()
        assert len(raw) == row['bytes'] and hashlib.sha256(raw).hexdigest() == row['sha256'], str(path)
        checked += 1
    assert checked == 98
print(json.dumps({'receipt_payloads': len(manifest['files']), 'reviewed_git_blobs': len(pins),
                  'external_files_checked': checked, 'exact_originals': 23, 'supported_claims': 28,
                  'supported_new_holders': 11, 'packet_accepted': False}))
