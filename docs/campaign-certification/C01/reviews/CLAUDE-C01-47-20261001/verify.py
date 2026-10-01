#!/usr/bin/env python3
"""Verify bounded review receipt integrity; this is not a historical-source reviewer."""
import argparse
import hashlib
import json
from pathlib import Path
import subprocess

ROOT = Path(__file__).resolve().parent

def sha(data):
    return hashlib.sha256(data).hexdigest()

def read(name):
    return json.loads((ROOT / name).read_text(encoding='utf-8'))

def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('--repo', type=Path)
    parser.add_argument('--external-root', type=Path)
    parser.add_argument('--require-validated', action='store_true')
    args = parser.parse_args()
    manifest = read('manifest.json')
    expected = {r['path'] for r in manifest['files']}
    actual = {p.relative_to(ROOT).as_posix() for p in ROOT.rglob('*') if p.is_file() and p.name != 'manifest.json'}
    assert actual == expected, 'Receipt file inventory differs'
    for row in manifest['files']:
        body = (ROOT / row['path']).read_bytes()
        assert (len(body), sha(body)) == (row['bytes'], row['sha256']), row['path']
    assert (ROOT / '.gitattributes').read_bytes() == b'* -text whitespace=cr-at-eol\n*.log -whitespace\n*.patch -whitespace\n'
    sources, claims, holders = read('source-review.json'), read('claim-review.json'), read('holder-review.json')
    assert len(sources) == len({s['source_id'] for s in sources}) == 24
    assert len(claims) == len({c['claim_id'] for c in claims}) == 47
    assert len(holders) == 13 and len({h['name'] for h in holders}) == 10
    assert all(s['material_read'] and s['independent_http_status'] == 200 for s in sources)
    assert all(c['material_read'] and c['no_effective_boundary_inferred'] for c in claims)
    assert all(h['from'] is None and h['until'] is None for h in holders)
    assert sum(len(s['visual_pages_reviewed']) for s in sources) == 20
    corrected = {c['claim_id'] for c in claims if c['decision'] == 'accepted_with_precision_correction'}
    assert corrected == set(read('scope.json')['corrected_claims'])
    claim_by_id = {c['claim_id']: c for c in claims}
    for cid in ('fr_ps_cambadelis_cedes_place_2017', 'fr_ps_ratification_81e_congres_20250605'):
        assert claim_by_id[cid]['accepted_attested_on'] is None
    attempts = read('source-attempts.json')
    assert len(attempts) == 24 and all(a['exact_original_reproduced'] and a['status'] == 200 for a in attempts)
    if args.require_validated:
        for name in ('focused-france', 'research-census-fixed', 'importer-check', 'local-index-check'):
            assert read('validation/' + name + '.json')['exit_code'] == 0, name
        assert read('validation/guard-red-02.json')['exit_code'] == 1
        assert b'FAILED (failures=3)' in (ROOT / 'validation/guard-red-02.log').read_bytes()
        assert b'Ran 48 tests' in (ROOT / 'validation/focused-france.log').read_bytes()
        assert b'Ran 16 tests' in (ROOT / 'validation/research-census-fixed.log').read_bytes()
    if args.repo:
        for row in read('repo-pins.json'):
            data = subprocess.check_output(['git', 'show', row['commit'] + ':' + row['path']], cwd=args.repo)
            assert (len(data), sha(data)) == (row['bytes'], row['sha256']), row['path']
        scope = read('scope.json')
        path = 'docs/campaign-certification/C01/research/france.json'
        versions = [json.loads(subprocess.check_output(['git', 'show', ref + ':' + path], cwd=args.repo))
                    for ref in (scope['source_import_commit'], scope['correction_commit'])]
        def holder_list(packet):
            return next(o for o in packet['organizations'] if o['id'] == 'fr_cnccfp_76')['roles'][0]['holder_claims']
        assert holder_list(versions[0]) == holder_list(versions[1])
        assert sha(json.dumps(holder_list(versions[1]), ensure_ascii=False, indent=2).encode()) == scope['holder_payload_sha256']
    if args.external_root:
        for row in attempts:
            data = (args.external_root / row['external_body_path']).read_bytes()
            assert (len(data), sha(data)) == (row['bytes'], row['sha256']), row['source_id']
    print(json.dumps({'status': 'pass', 'receipt_files': len(expected), 'sources': 24, 'claims': 47, 'holders': 13,
                      'pinned_repo_checked': bool(args.repo), 'external_originals_checked': bool(args.external_root),
                      'historical_interpretation_reperformed': False}))

if __name__ == '__main__':
    main()
