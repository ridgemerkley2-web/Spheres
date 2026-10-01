"""Verify retained review evidence; this does not reproduce historical reading."""
from pathlib import Path
import argparse
import hashlib
import json
import subprocess


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('--repo', type=Path)
    parser.add_argument('--external-root', type=Path)
    parser.add_argument('--require-validated', action='store_true')
    args = parser.parse_args()
    root = Path(__file__).resolve().parent

    def read(name):
        return json.loads((root / name).read_bytes())

    def check_bytes(data, record):
        assert len(data) == record['bytes'], record.get('path', record)
        assert hashlib.sha256(data).hexdigest() == record['sha256'], record.get('path', record)

    manifest = read('manifest.json')
    assert manifest['format'] == 'spheres-c01-independent-review-receipt/v1'
    expected_paths = {row['path'] for row in manifest['files']}
    actual_paths = {p.relative_to(root).as_posix() for p in root.rglob('*') if p.is_file() and p.name != 'manifest.json' and '__pycache__' not in p.parts}
    assert expected_paths == actual_paths, 'Unexpected or missing receipt file'
    for row in manifest['files']:
        check_bytes((root / row['path']).read_bytes(), row)
    scope = read('scope.json')
    sources = read('source-review.json')
    claims = read('claim-review.json')
    holders = read('holder-review.json')
    assert len(sources) == 16 and len(claims) == 20 and len(holders) == 9
    assert len({s['source_id'] for s in sources}) == 16
    assert len({c['claim_id'] for c in claims}) == 20
    assert all(s['material_read'] and s['document_text_read_in_full'] and s['portal_card_read_in_full'] for s in sources)
    assert all(c['material_read'] and c['decision'] == 'accepted_bounded_claim_unchanged' for c in claims)
    assert all(h['accepted_holder']['from'] is None for h in holders)
    ends = [h for h in holders if h['accepted_holder']['until'] is not None]
    assert len(ends) == 1 and ends[0]['accepted_holder']['until'] == '1993-10-03'
    assert scope['packet_exact_submitted_blob_unchanged']
    assert scope['old_source_and_claim_objects_unchanged'] and scope['old_holder_objects_unchanged']
    assert not scope['generated_index_imported'] and not scope['runtime_or_art_changed'] and not scope['parent_qualification_changed']
    validation = read('validation-summary.json')
    assert validation['decision'] == 'passed_bounded_checks'
    assert validation['commands'] and all(c['exit_code'] == 0 for c in validation['commands'])
    for command in validation['commands']:
        check_bytes((root / command['log']).read_bytes(), {'bytes': command['log_bytes'], 'sha256': command['log_sha256']})
    assert len(read('guard-controls.json')) == 7
    assert all(c['expected'] == c['actual'] for c in read('guard-controls.json'))
    attempts = read('source-attempts.json')
    assert len(attempts) == 35 and sum(a['matches'] for a in attempts) == 32
    assert {(a['source_id'], a['kind']) for a in attempts if a['matches']} == {(s['source_id'], kind) for s in sources for kind in ('original', 'card')}
    repo_checks = 0
    if args.repo:
        for row in read('repo-pins.json'):
            data = subprocess.check_output(['git', 'show', row['commit'] + ':' + row['path']], cwd=args.repo)
            check_bytes(data, row)
            repo_checks += 1
        packet_path = 'docs/campaign-certification/C01/research/russia.json'
        submitted = subprocess.check_output(['git', 'show', scope['exact_source_revision'] + ':' + packet_path], cwd=args.repo)
        reviewed = subprocess.check_output(['git', 'show', scope['correction_commit'] + ':' + packet_path], cwd=args.repo)
        assert submitted == reviewed
    external_checks = 0
    if args.external_root:
        for attempt in attempts:
            path = args.external_root / attempt['external_body_path']
            if attempt['body_file_retained']:
                check_bytes(path.read_bytes(), attempt)
                external_checks += 1
            else:
                assert not path.exists() and not attempt['matches'] and attempt['bytes'] == 0 and attempt['http'] == '000'
            if attempt['headers']:
                header = attempt['headers']
                check_bytes((args.external_root / header['path']).read_bytes(), header)
    print(json.dumps({'result': 'pass', 'receipt_files': len(expected_paths), 'sources': 16, 'claims': 20, 'holders': 9, 'repo_blob_checks': repo_checks, 'external_body_checks': external_checks, 'historical_reading_recreated': False}))


if __name__ == '__main__':
    main()
