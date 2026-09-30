"""Read-only C01-37 receipt integrity; not source reapproval or parent qualification."""
import argparse
import hashlib
import json
from pathlib import Path


def digest(raw):
    return hashlib.sha256(raw).hexdigest()


def raw(root, relative):
    target = (root / relative).resolve(strict=True)
    assert target.is_relative_to(root), relative
    return target.read_bytes()


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('--external-evidence', type=Path)
    args = parser.parse_args()
    root = Path(__file__).resolve().parent
    manifest = json.loads(raw(root, 'manifest.json'))
    seen = set()
    for row in manifest['files']:
        assert row['path'] not in seen
        seen.add(row['path'])
        data = raw(root, row['path'])
        assert len(data) == row['bytes'] and digest(data) == row['sha256'], row['path']
    actual = {p.relative_to(root).as_posix() for p in root.rglob('*') if p.is_file() and p != root / 'manifest.json'}
    assert seen == actual
    sources = json.loads(raw(root, 'source-verification.json'))['sources']
    assert len(sources) == len({r['source_id'] for r in sources}) == 31
    claims = json.loads(raw(root, 'claim-review.json'))['claims']
    assert len(claims) == len({c['claim_id'] for c in claims}) == 48
    assert {c['claim_id'] for c in claims} == {c for s in sources for c in s['claim_ids']}
    boundaries = json.loads(raw(root, 'boundary-review.json'))['boundaries']
    assert len(boundaries) == 26 and all(b['reviewed_value'] is None for b in boundaries)
    assert sum(b['field'] == 'from' for b in boundaries) == 12
    scope = json.loads(raw(root, 'scope-audit.json'))
    assert len(scope['received_holders']) == len(scope['reviewed_holders']) == 16
    assert all(h['from'] is None and h['until'] is None and h['attested_on'] for h in scope['reviewed_holders'])
    count = decoded_count = 0
    if args.external_evidence:
        external = args.external_evidence.resolve(strict=True)
        for row in sources:
            data = raw(external, row['body_relative_path'])
            assert row['exact_recorded_response'] is True and row['http_status'] == 200
            assert len(data) == row['expected_bytes'] == row['actual_bytes']
            assert digest(data) == row['expected_sha256'] == row['actual_sha256']
            count += 1
            if 'decoded_response' in row:
                d = row['decoded_response']; data = raw(external, d['relative_path'])
                assert len(data) == d['bytes'] and digest(data) == d['sha256']
                decoded_count += 1
    print(json.dumps({'receipt_integrity_passed': True, 'payloads': len(seen), 'source_records': 31,
                      'material_claims': 48, 'unsupported_boundaries_removed': 26,
                      'external_original_bodies_rehashed': count, 'external_decoded_files_rehashed': decoded_count,
                      'source_content_review_repeated': False, 'effective_term_research_complete': False,
                      'parent_qualification': False}, indent=2))


if __name__ == '__main__':
    main()
