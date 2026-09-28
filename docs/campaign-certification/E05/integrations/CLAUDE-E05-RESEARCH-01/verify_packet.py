"""Verify this frozen review packet; no network or historical-certification inference."""
import collections
import hashlib
import json
from pathlib import Path


def main():
    root = Path(__file__).resolve().parent
    manifest = json.loads((root / 'review-manifest.json').read_text(encoding='utf-8'))
    assert manifest['format'] == 'spheres-research-review/v1'
    assert manifest['task'] == 'CLAUDE-E05-RESEARCH-01'
    assert manifest['status'] == 'accepted_for_integration'
    assert manifest['scope'] == 'research_preparation_only'
    paths = set()
    for item in manifest['evidence']:
        rel = item['path']
        path = (root / rel).resolve()
        assert path.is_relative_to(root) and rel not in paths, rel
        paths.add(rel)
        raw = path.read_bytes()
        assert len(raw) == item['bytes'], rel
        assert hashlib.sha256(raw).hexdigest() == item['sha256'], rel
    actual = {p.relative_to(root).as_posix() for p in root.rglob('*') if p.is_file()
              and p.name != 'review-manifest.json' and '__pycache__' not in p.parts}
    assert paths == actual, (paths - actual, actual - paths)
    sources = json.loads((root / 'reviewed-inputs/docs/research/company-pilot/sources.json').read_text(encoding='utf-8'))['sources']
    claims = [c for p in (root / 'reviewed-inputs/docs/research/company-pilot/dossiers').glob('*.json')
              for c in json.loads(p.read_text(encoding='utf-8'))['claims']]
    observations = json.loads((root / 'retrievals.json').read_text(encoding='utf-8'))['results']
    review = json.loads((root / 'claim-review.json').read_text(encoding='utf-8'))
    corrections = json.loads((root / 'extraction-corrections.json').read_text(encoding='utf-8'))
    assert len(sources) == len(observations) == 99
    assert {r['source'] for r in observations} == {s['id'] for s in sources}
    assert len(claims) == len(review['rows']) == 195
    assert {c['id'] for c in claims} == {r['claim'] for r in review['rows']}
    counts = collections.Counter(r['content_status'] for r in review['rows'])
    assert dict(counts) == review['counts']
    assert counts == {'bounded_direct_content_checked': 20, 'partial_direct_content_check': 26,
                      'bounded_primary_web_text_checked_no_body_reproduction': 10,
                      'not_independently_verified': 139}
    assert sum(r['status'] == 200 for r in observations) == 68
    assert sum(r['matches_first_response'] for r in observations) == 54
    assert sum(r['status'] == 403 for r in observations) == 30
    assert sum(r['status'] is None for r in observations) == 1
    assert len(corrections) == 4 and all(c['found'] for c in corrections)
    assert sum(r['anchor_found'] for r in review['rows']) == 150
    assert len(review['claims_without_direct_body']) == 45
    assert len(review['claims_not_fully_content_checked']) == 165
    by_source = {s['source']: s for s in observations}
    by_claim = {c['id']: c for c in claims}
    corrected = {c['claim']: c for c in corrections}
    for row in review['rows']:
        assert row['source'] == by_claim[row['claim']]['source']
        source = by_source[row['source']]
        assert row['body_sha256'] == source['sha256']
        assert row['direct_status'] == source['status']
        anchors = {a['claim']: a['found'] for a in source.get('anchors', [])}
        expected = bool(anchors.get(row['claim'])) or bool(corrected.get(row['claim'], {}).get('found'))
        assert row['anchor_found'] == expected
        if row['claim'] in corrected:
            assert corrected[row['claim']]['body_sha256'] == source['sha256']
    print(f'PASS: {len(paths)} frozen files; 99 responses, 195 claim statuses, 150 anchors; research preparation only.')


if __name__ == '__main__':
    main()
