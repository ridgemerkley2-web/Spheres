"""Verify held C01-31 receipt integrity without fetching or accepting research."""
import argparse
import hashlib
import json
import pathlib
import subprocess

parser = argparse.ArgumentParser(description=__doc__)
parser.add_argument('--external-originals', action='store_true')
parser.add_argument('--repo', type=pathlib.Path)
args = parser.parse_args()
root = pathlib.Path(__file__).resolve().parent
read = lambda name: json.loads((root / name).read_text(encoding='utf8'))
sha = lambda b: hashlib.sha256(b).hexdigest()
manifest = read('manifest.json')
actual = {p.relative_to(root).as_posix() for p in root.rglob('*') if p.is_file() and p.name != 'manifest.json' and '__pycache__' not in p.parts}
assert actual == {r['path'] for r in manifest['files']}
for row in manifest['files']:
    data = (root / row['path']).read_bytes()
    assert len(data) == row['bytes'] and sha(data) == row['sha256'], row['path']
sources, claims, holders, held = read('source-verification.json'), read('claim-review.json'), read('holder-review.json'), read('held-dependencies.json')
assert (len(sources), len(claims), len(holders)) == (31, 64, 15)
assert len({s['source_id'] for s in sources}) == 31 and len({c['claim_id'] for c in claims}) == 64
assert held['decision'] == 'held_not_accepted'
assert set(held['sources']) == {s['source_id'] for s in sources if s['retrieval_decision'] == 'held_original_unavailable'}
assert set(held['claims']) == {c['claim_id'] for c in claims if c['decision'] == 'held_original_unavailable'}
assert (len(held['sources']), len(held['claims']), sum(bool(h['missing_source_ids']) for h in holders)) == (7, 14, 5)
assert sum(s['retrieval_decision'] == 'exact_original_reproduced' for s in sources) == 24
for holder in holders:
    assert set(holder['missing_source_ids']) == set(holder['reviewed']['sources']) & set(held['sources'])
takeya, = [h for h in holders if h['name'] == '竹谷とし子']
assert takeya['submitted']['from'] == '2026-03-14'
assert takeya['reviewed']['from'] is None and takeya['reviewed']['attested_on'] == '2026-03-14'
if args.external_originals:
    for source in sources:
        if source['retrieval_decision'] != 'exact_original_reproduced':
            continue
        data = pathlib.Path(source['body_external']).read_bytes()
        assert len(data) == source['expected_bytes'] and sha(data) == source['expected_sha256'], source['source_id']
        assert sha(pathlib.Path(source['readable_external']).read_bytes()) == source['readable_sha256']
    for header in read('header-provenance.json'):
        data = pathlib.Path(header['original_external']).read_bytes()
        assert len(data) == header['original_bytes'] and sha(data) == header['original_sha256']
if args.repo:
    scope = read('scope-audit.json')
    for row in scope['authored_paths']:
        for prefix, revision in [('authored', scope['authored_import']), ('reviewed', scope['correction'])]:
            data = subprocess.check_output(['git', 'show', revision + ':' + row['path']], cwd=args.repo)
            assert len(data) == row[prefix + '_bytes'] and sha(data) == row[prefix + '_sha256'], row['path']
print('Receipt integrity verified. C01-31 remains held: 7 originals, 14 claims, 5 holder dependencies. No historical acceptance inferred.')
