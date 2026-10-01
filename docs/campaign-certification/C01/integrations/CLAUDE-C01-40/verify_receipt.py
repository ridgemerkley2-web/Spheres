"""Check receipt integrity, optionally original files. Does not repeat content review."""
import argparse
import hashlib
import json
import pathlib

ROOT = pathlib.Path(__file__).resolve().parent
parser = argparse.ArgumentParser(description=__doc__)
parser.add_argument('--external-evidence', type=pathlib.Path)
args = parser.parse_args()
manifest = json.loads((ROOT / 'manifest.json').read_text(encoding='utf-8'))
for row in manifest['files']:
    raw = (ROOT / row['path']).read_bytes()
    assert len(raw) == row['bytes'], row['path']
    assert hashlib.sha256(raw).hexdigest() == row['sha256'], row['path']
source_review = json.loads((ROOT / 'source-review.json').read_text(encoding='utf-8'))
claim_review = json.loads((ROOT / 'claim-review.json').read_text(encoding='utf-8'))
assert len(source_review['sources']) == 18
assert len({c['claim_id'] for c in claim_review['claims']}) == 24
assert {c for s in source_review['sources'] for c in s['claim_ids']} == {c['claim_id'] for c in claim_review['claims']}
assert not source_review['held_originals']
checked = 0
if args.external_evidence:
    journal = json.loads((ROOT / 'source-attempts.json').read_text(encoding='utf-8'))
    text = json.loads((ROOT / 'source-texts.json').read_text(encoding='utf-8'))
    successful = set()
    for row in journal['attempts']:
        for kind in ('body', 'headers', 'stderr'):
            path = args.external_evidence / ('attempt-%02d' % row['attempt']) / (row['source_id'] + '.' + kind)
            digest = row.get('sha256' if kind == 'body' else kind + '_sha256')
            if digest:
                raw = path.read_bytes()
                assert hashlib.sha256(raw).hexdigest() == digest, str(path)
                if kind == 'body':
                    assert len(raw) == row['bytes'], str(path)
                checked += 1
            else:
                assert not path.exists(), 'Unexpected retained file without receipt pin: ' + str(path)
        if row['exact']:
            successful.add(row['source_id'])
    for row in text['sources']:
        path = pathlib.Path(row['text_path'])
        path = args.external_evidence / path.parent.name / path.name
        assert hashlib.sha256(path.read_bytes()).hexdigest() == row['text_sha256'], str(path)
        checked += 1
    assert len(successful) == 18
print(json.dumps({'receipt_files_checked': len(manifest['files']), 'external_files_checked': checked,
                  'source_identities': 18, 'content_decisions': 24,
                  'historical_review_repeated': False}))
