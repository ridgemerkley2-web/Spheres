"""Verify the held C01-39 checkpoint without needing the unaccepted country import."""
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
sources = json.loads((ROOT / 'source-review.json').read_text(encoding='utf-8'))
claims = json.loads((ROOT / 'claim-review.json').read_text(encoding='utf-8'))
holders = json.loads((ROOT / 'holder-review.json').read_text(encoding='utf-8'))
assert (sources['exact'], sources['rate_limited'], sources['not_attempted']) == (1, 1, 22)
assert len(sources['sources']) == 24 and len(claims['claims']) == 29
assert (claims['independently_read'], claims['held'], claims['accessible_claims_deferred']) == (1, 28, 0)
assert not claims['packet_accepted'] and holders['new_supported'] == 0 and holders['new_held'] == 11
assert {c for s in sources['sources'] for c in s['claim_ids']} == {c['claim_id'] for c in claims['claims']}
checked = 0
if args.external_evidence:
    journal = json.loads((ROOT / 'source-attempts.json').read_text(encoding='utf-8'))
    text = json.loads((ROOT / 'source-texts.json').read_text(encoding='utf-8'))
    assert len(journal['attempts']) == 2 and len(journal['not_attempted']) == 22
    for row in journal['attempts']:
        for kind in ('body', 'headers', 'stderr'):
            path = args.external_evidence / ('attempt-%02d' % row['attempt']) / (row['source_id'] + '.' + kind)
            digest = row.get('sha256' if kind == 'body' else kind + '_sha256')
            raw = path.read_bytes()
            assert hashlib.sha256(raw).hexdigest() == digest, str(path)
            assert len(raw) == row['bytes' if kind == 'body' else kind + '_bytes'], str(path)
            checked += 1
    for row in text['sources']:
        original = pathlib.Path(row['text_path'])
        path = args.external_evidence / original.parent.name / original.name
        assert hashlib.sha256(path.read_bytes()).hexdigest() == row['text_sha256'], str(path)
        checked += 1
    assert checked == 7
print(json.dumps({'receipt_files_checked': len(manifest['files']), 'external_files_checked': checked,
                  'source_outcomes': {'exact': 1, 'rate_limited': 1, 'not_attempted': 22},
                  'claim_outcomes': {'read': 1, 'held': 28}, 'new_holders_held': 11,
                  'accepted': False}))
