"""Check receipt integrity and retained evidence, not historical interpretation."""
import argparse
import hashlib
import json
from pathlib import Path
import subprocess


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('--repo', type=Path)
    parser.add_argument('--external-root', type=Path)
    args = parser.parse_args()
    root = Path(__file__).resolve().parent

    def read(name):
        return json.loads((root / name).read_bytes())

    def check(raw, pin):
        assert len(raw) == pin['bytes']
        assert hashlib.sha256(raw).hexdigest() == pin['sha256']

    manifest = read('manifest.json')
    paths = {p.relative_to(root).as_posix() for p in root.rglob('*') if p.is_file() and p.name != 'manifest.json' and '__pycache__' not in p.parts}
    assert paths == {x['path'] for x in manifest['files']}
    for pin in manifest['files']:
        check((root / pin['path']).read_bytes(), pin)
    decision = read('decision.json')
    assert decision['decision'] == 'accepted_bounded_research'
    assert decision['counts'] == {'originals_reviewed': 68, 'claims_materially_reviewed': 137, 'holder_observations_reviewed': 24, 'remaining_original_holds': 0, 'remaining_claim_holds': 0}
    assert not decision['research_imported'] and not decision['runtime_or_art_changed'] and not decision['canonical_qualification']
    claims = read('claim-review.json')['claims']
    assert len(claims) == 4 and all(not c['holder_use'] for c in claims)
    attempts = read('retrieval.json')['records']
    assert len(attempts) == 3 and all(r['exact'] and not r['stop_all_requests'] for r in attempts)
    validation = read('validation.json')
    assert validation['exit_code'] == 0
    check((root / validation['log']).read_bytes(), {'bytes': validation['log_bytes'], 'sha256': validation['log_sha256']})
    assert len(read('retained-body-recheck.json')['bodies']) == 65
    if args.repo:
        for pin in read('preservation-pins.json'):
            raw = subprocess.check_output(['git', 'show', pin['revision'] + ':' + pin['path']], cwd=args.repo)
            check(raw, pin)
        prefix = 'docs/campaign-certification/C01/reviews/CLAUDE-C01-28/'
        def gitjson(ref, path):
            return json.loads(subprocess.check_output(['git', 'show', ref + ':' + path], cwd=args.repo))
        ref = decision['receipt_base_revision']
        before = gitjson(ref, prefix + '2026-09-30-available-content/claim-review.json')
        middle = gitjson(ref, prefix + '2026-10-01-missing-only/claim-review.json')
        ids = {c['claim_id'] for c in before['claims'] if c['status'] == 'material_content_checked'}
        assert len(ids) == 113
        ids.update(c['claim_id'] for c in middle['claims'])
        assert len(ids) == 133
        ids.update(c['claim_id'] for c in claims)
        assert len(ids) == 137
        source_ids = set(gitjson(ref, prefix + 'baseline-isolation.json')['removed_new_sources'])
        packet = gitjson(decision['reviewed_packet_revision'], 'docs/campaign-certification/C01/research/russia.json')
        assert ids == {c['id'] for s in packet['sources'] if s['id'] in source_ids for c in s['claims']}
    if args.external_root:
        for pin in read('retained-body-recheck.json')['bodies']:
            check(Path(pin['body_path']).read_bytes(), pin)
        for record in attempts:
            check((args.external_root / record['body_path']).read_bytes(), record)
            check((args.external_root / record['headers_path']).read_bytes(), {'bytes': record['headers_bytes'], 'sha256': record['headers_sha256']})
        render = read('visual-review.json')['external_render']
        check(Path(render['path']).read_bytes(), render)
        patch = read('import-provenance.json')['reviewed_patch']
        check((args.external_root / patch['path']).read_bytes(), patch)
    print(json.dumps({'result': 'pass', 'originals': 68, 'claims': 137, 'holder_observations': 24, 'new_claim_reviews': 4, 'historical_reading_recreated': False}))


if __name__ == '__main__':
    main()
