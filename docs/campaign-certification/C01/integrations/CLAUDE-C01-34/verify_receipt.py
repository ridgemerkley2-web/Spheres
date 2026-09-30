"""Read-only C01-34 receipt integrity check, not source reapproval or campaign qualification."""
import argparse
import hashlib
import json
from pathlib import Path


def digest(raw):
    return hashlib.sha256(raw).hexdigest()


def read(root, relative):
    target = (root / relative).resolve(strict=True)
    assert target.is_relative_to(root), relative
    return target.read_bytes()


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('--external-evidence', type=Path)
    args = parser.parse_args()
    root = Path(__file__).resolve().parent
    manifest = json.loads(read(root, 'manifest.json'))
    listed = set()
    for row in manifest['files']:
        assert row['path'] not in listed
        listed.add(row['path'])
        raw = read(root, row['path'])
        assert len(raw) == row['bytes'] and digest(raw) == row['sha256'], row['path']
    actual = {p.relative_to(root).as_posix() for p in root.rglob('*') if p.is_file() and p != root / 'manifest.json'}
    assert listed == actual, 'Receipt has missing or unlisted payloads'
    sources = json.loads(read(root, 'source-verification.json'))['sources']
    assert len(sources) == len({r['source_id'] for r in sources}) == 76
    assert sum(r['exact_recorded_response'] for r in sources) == 75
    replacement = [r for r in sources if not r['exact_recorded_response']]
    assert len(replacement) == 1 and replacement[0]['source_id'] == 'br_mdb_tse_sgip_cen_2013_2019'
    assert replacement[0]['current_response_accepted'] is True
    assert (replacement[0]['expected_bytes'], replacement[0]['expected_sha256']) == (18952, '1d00f3018ea6d2fdc9f863cff6762edee15cd546dd791880f6c75dcb0b2b2247')
    assert (replacement[0]['actual_bytes'], replacement[0]['actual_sha256']) == (18945, 'f6a0c4cacda5f5ce2e5021c74a923880eb6f2a9d270b2fb80bac1aed4c17b17d')
    claims = json.loads(read(root, 'claim-review.json'))['claims']
    assert len(claims) == len({r['claim_id'] for r in claims}) == 127
    assert {r['claim_id'] for r in claims} == {cid for r in sources for cid in r['claim_ids']}
    external_count = decoded_count = attempts_count = 0
    if args.external_evidence is not None:
        external = args.external_evidence.resolve(strict=True)
        for row in sources:
            raw = read(external, row['body_relative_path'])
            assert len(raw) == row['actual_bytes'] and digest(raw) == row['actual_sha256'], row['source_id']
            if row['exact_recorded_response']:
                assert len(raw) == row['expected_bytes'] and digest(raw) == row['expected_sha256']
            external_count += 1
            if 'decoded_response' in row:
                decoded = row['decoded_response']
                raw = read(external, decoded['relative_path'])
                assert len(raw) == decoded['bytes'] and digest(raw) == decoded['sha256']
                decoded_count += 1
        for group in json.loads(read(root, 'source-attempts.json'))['attempts']:
            raw = read(external, group['attempt_log'])
            assert len(raw) == group['original_log_pin']['bytes'] and digest(raw) == group['original_log_pin']['sha256']
            for row in group['rows']:
                if row['http_status'] != 200:
                    continue
                raw = read(external, row['body_relative_path'])
                assert len(raw) == row['actual_bytes'] and digest(raw) == row['actual_sha256']
                attempts_count += 1
    print(json.dumps({'receipt_integrity_passed': True, 'payloads': len(listed),
                      'source_records': 76, 'submitted_original_matches': 75, 'qualified_current_response': 1,
                      'external_selected_bodies_rehashed': external_count,
                      'external_decoded_files_rehashed': decoded_count, 'external_successful_attempt_bodies_rehashed': attempts_count,
                      'source_content_review_repeated': False, 'original_changed_body_recovered': False,
                      'parent_qualification': False}, indent=2))


if __name__ == '__main__':
    main()
