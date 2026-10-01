"""Verify retained C01-38 receipt bytes; does not repeat historical content review."""
import argparse
import hashlib
import json
from pathlib import Path


def read(root, name):
    path = (root / name).resolve(strict=True)
    assert path.is_relative_to(root), name
    return path.read_bytes()


def check(root, row, key='path'):
    data = read(root, row[key])
    assert len(data) == row['bytes'], row[key]
    assert hashlib.sha256(data).hexdigest() == row['sha256'], row[key]


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('--external-evidence', type=Path)
    args = parser.parse_args()
    root = Path(__file__).resolve().parent
    manifest = json.loads(read(root, 'manifest.json'))['files']
    seen = set()
    for row in manifest:
        assert row['path'] not in seen
        seen.add(row['path'])
        check(root, row)
    assert seen == {p.relative_to(root).as_posix() for p in root.rglob('*') if p.is_file() and p.name != 'manifest.json'}
    sources = json.loads(read(root, 'source-verification.json'))['sources']
    claims = json.loads(read(root, 'claim-review.json'))['claims']
    holders = json.loads(read(root, 'holder-review.json'))['holders']
    assert len(sources) == len({s['source_id'] for s in sources}) == 22
    assert len(claims) == len({c['claim_id'] for c in claims}) == 33
    assert {c['claim_id'] for c in claims} == {c for s in sources for c in s['claim_ids']}
    assert len(holders) == 12 and len({h['name'] for h in holders}) == 9
    assert all(h['from'] is None and h['until'] is None and h['attested_on'] for h in holders)
    attempts = json.loads(read(root, 'source-attempts.json'))
    assert len(attempts['attempts']) == attempts['attempt_count'] == 28
    assert sum(not a.get('exact', False) for a in attempts['attempts']) == attempts['failed_attempts'] == 6
    assert all(c['decision'] == 'supported_bounded_source_observation' for c in claims)
    bodies = decoded = members = 0
    if args.external_evidence:
        external = args.external_evidence.resolve(strict=True)
        for source in sources:
            assert source['exact_recorded_response'] and source['http_status'] == 200
            assert source['expected_bytes'] == source['actual_bytes']
            assert source['expected_sha256'] == source['actual_sha256']
            check(external, {'path': source['body_relative_path'], 'bytes': source['expected_bytes'], 'sha256': source['expected_sha256']})
            bodies += 1
            if 'decoded_response' in source:
                check(external, source['decoded_response'], 'relative_path')
                decoded += 1
            for member in source.get('reviewed_members', []):
                check(external, member, 'relative_path')
                members += 1
        for attempt in attempts['attempts']:
            for name in ['headers', 'error']:
                if name+'_sha256' in attempt:
                    check(external, {'path': attempt[name+'_relative_path'], 'bytes': attempt[name+'_bytes'], 'sha256': attempt[name+'_sha256']})
    print(json.dumps({'receipt_integrity_passed': True, 'payloads': len(seen), 'source_records': 22,
                      'claims': 33, 'holders': 12, 'external_originals_rehashed': bodies,
                      'decoded_html_rehashed': decoded, 'xml_records_rehashed': members,
                      'content_review_repeated': False, 'parent_qualification': False}, indent=2))


if __name__ == '__main__':
    main()
