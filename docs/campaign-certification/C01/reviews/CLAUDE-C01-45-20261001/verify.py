#!/usr/bin/env python3
"""Offline C01-45 receipt verification; no network or historical rereview."""
import argparse
import hashlib
import json
import os
from pathlib import Path
import re
import subprocess

HERE = Path(__file__).resolve().parent


def load(name):
    return json.loads((HERE / name).read_text(encoding='utf-8'))


def inside(root, relative):
    path = (root / relative).resolve()
    assert path.is_relative_to(root.resolve()), relative
    return path


def check(raw, row):
    assert len(raw) == row['bytes'], ('bytes', row.get('path'))
    assert hashlib.sha256(raw).hexdigest() == row['sha256'], ('sha256', row.get('path'))


def blob(repo, rev, path):
    assert re.fullmatch('[0-9a-f]{40}', rev), rev
    assert not Path(path).is_absolute() and '..' not in Path(path).parts, path
    return subprocess.check_output(['git', 'show', rev + ':' + path], cwd=repo,
                                   env=dict(os.environ, GIT_OPTIONAL_LOCKS='0'))


def repository_checks(repo, scope):
    base, rev = scope['integration_base'], scope['corrective_commit']
    for row in scope['reviewed_files']:
        check(blob(repo, rev, row['path']), row)
        assert blob(repo, scope['first_source_import'], row['path']) == blob(
            repo, scope['incoming_substantive_commit'], row['path']), row['path']
    path = 'docs/campaign-certification/C01/research/saudi-arabia.json'
    old, new = json.loads(blob(repo, base, path)), json.loads(blob(repo, rev, path))
    assert len(old['sources']) == 103 and len(new['sources']) == 121
    assert old['sources'] == new['sources'][:103]
    assert sum(len(s['claims']) for s in old['sources']) == 158
    assert sum(len(s['claims']) for s in new['sources']) == 188
    assert old['research_cutoff'] == new['research_cutoff'] == '2026-09-07'
    assert old['organizations'] == new['organizations']
    assert old['coverage'] == new['coverage']
    oldroles = {r['id']: r for i in old['institutions'] for r in i['roles']}
    newroles = {r['id']: r for i in new['institutions'] for r in i['roles']}
    for rid, previous in oldroles.items():
        current = newroles[rid]
        if rid in ('sa_king', 'sa_crown_prince', 'sa_succession_secretary'):
            for field in ('holder_claims', 'sources', 'claim_ids'):
                assert current[field][:len(previous[field])] == previous[field], (rid, field)
        else:
            assert previous == current, rid
    secretary = newroles['sa_succession_secretary']['holder_claims']
    assert len(secretary) == 1 and secretary[0]['from'] is None and secretary[0]['until'] is None
    assert secretary[0]['attested_on'] == '2006-10-20'
    new_sources = new['sources'][103:]
    assert [s['id'] for s in new_sources] == scope['new_source_ids']
    assert [c['id'] for s in new_sources for c in s['claims']] == scope['new_claim_ids']
    for source in new_sources:
        raw = blob(repo, rev, source['snapshot']['path'])
        check(raw, source['snapshot'])
        assert b'\r' not in raw
        extract = json.loads(raw)
        assert extract['scope_note'] == source['scope_note']
        assert len(source['claims']) == len(extract['rows'])
        for claim, row in zip(source['claims'], extract['rows']):
            assert claim['id'] == row['claim_id']
            assert claim['text'] == row['text']
            assert claim['locator'] == row['locator']
            assert claim.get('attested_on') == row['attested_on']


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('--repo', type=Path)
    parser.add_argument('--originals', type=Path)
    parser.add_argument('--require-accepted', action='store_true')
    args = parser.parse_args()
    manifest = load('manifest.json')
    expected = {r['path'] for r in manifest['files']}
    assert len(expected) == len(manifest['files'])
    actual = {p.relative_to(HERE).as_posix() for p in HERE.rglob('*') if p.is_file()
              and p.name != 'manifest.json' and '__pycache__' not in p.parts}
    assert actual == expected, {'missing': sorted(expected-actual), 'extra': sorted(actual-expected)}
    for row in manifest['files']:
        check(inside(HERE, row['path']).read_bytes(), row)
    assert (HERE/'.gitattributes').read_text().splitlines() == [
        '* -text whitespace=cr-at-eol', '*.log -whitespace', '*.patch -whitespace']
    sources, claims, holders = load('source-review.json'), load('claim-review.json'), load('holder-review.json')
    assert len(sources['sources']) == 18 and sources['sources_held'] == 0
    assert len(claims['claims']) == 30 and claims['claims_held'] == 0
    assert len(holders['observations']) == 18 and holders['holder_observations_held'] == 0
    assert {c['claim_id'] for c in claims['claims']} == {
        cid for s in sources['sources'] for cid in s['claim_ids']}
    retrieval = load('retrieval.json')
    assert len(retrieval) == 18
    for row in retrieval:
        assert row['exit_code'] == 0 and row['http_status'] == '200' and row['matches_submitted']
        assert row['bytes'] == row['expected_bytes'] and row['sha256'] == row['expected_sha256']
        original = next(s for s in sources['sources'] if s['source_id'] == row['source_id'])
        assert original['sha256'] == row['sha256'] and original['bytes'] == row['bytes']
    validation = load('validation/status.json')
    for label, result in validation['observed_results'].items():
        command = load('validation/' + label + '.json')
        check(inside(HERE/'validation', command['log']).read_bytes(), command)
        assert command['exit_code'] == result['exit_code'], label
    checked = 0
    if args.originals:
        for row in sources['external_artifacts']:
            check(inside(args.originals, row['path']).read_bytes(), row)
            checked += 1
    scope = load('scope-review.json')
    if args.repo:
        repository_checks(args.repo, scope)
    decision = load('decision.json')
    if args.require_accepted:
        assert decision['status'] == 'accepted_bounded_research'
        assert validation['status'] == 'passed_focused_checks'
    print(json.dumps(dict(receipt_files_checked=len(expected), external_artifacts_checked=checked,
        repository_scope_checked=bool(args.repo), decision=decision['status'],
        historical_content_review_repeated=False, parent_qualification_claimed=False), indent=2))


if __name__ == '__main__':
    main()
