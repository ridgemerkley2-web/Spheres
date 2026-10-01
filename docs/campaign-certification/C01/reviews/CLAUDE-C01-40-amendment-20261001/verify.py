#!/usr/bin/env python3
"""Offline receipt integrity and optional original/Git-blob checks.

This does not repeat historical content review or run the recorded commands.
Paths to retained originals may change; their recorded byte identities may not.
"""
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


def check_bytes(raw, row):
    assert len(raw) == row['bytes'], ('bytes', row.get('path'))
    assert hashlib.sha256(raw).hexdigest() == row['sha256'], ('sha256', row.get('path'))


def within(root, relative):
    path = (root / relative).resolve()
    assert path.is_relative_to(root.resolve()), ('outside evidence directory', relative)
    return path


def git_blob(repo, revision, path):
    assert re.fullmatch(r'[0-9a-f]{40}', revision), revision
    assert not Path(path).is_absolute() and '..' not in Path(path).parts, path
    return subprocess.check_output(['git', 'show', revision + ':' + path], cwd=repo,
                                   env=dict(os.environ, GIT_OPTIONAL_LOCKS='0'))


def check_repo(repo, scope):
    revision, baseline = scope['reviewed_source_commit'], scope['integration_base']
    for row in scope['reviewed_source_files']:
        check_bytes(git_blob(repo, revision, row['path']), row)
    for row in scope['validation_followup_files']:
        check_bytes(git_blob(repo, scope['validation_followup_commit'], row['path']), row)
    path = 'docs/campaign-certification/C01/research/india.json'
    old = json.loads(git_blob(repo, baseline, path))
    new = json.loads(git_blob(repo, revision, path))
    old_sources = {s['id']: s for s in old['sources']}
    new_sources = {s['id']: s for s in new['sources']}
    assert len(old_sources) == 315 and len(new_sources) == 317
    assert set(new_sources) - set(old_sources) == set(scope['new_sources'])
    changed = {
        'in_pd_20050417_rally_concludes_18th_congress',
        'in_pd_20120415_rally_concludes_20th_congress',
        'in_pd_20150426_join_to_bring_forth_change',
    }
    for sid, source in old_sources.items():
        if sid not in changed:
            assert source == new_sources[sid], sid
    old_claims = {c['id']: c for s in old['sources'] for c in s['claims']}
    new_claims = {c['id']: c for s in new['sources'] for c in s['claims']}
    assert len(old_claims) == 623 and len(new_claims) == 625
    for cid, claim in old_claims.items():
        for field in ('text', 'attested_on', 'locator'):
            assert claim.get(field) == new_claims[cid].get(field), (cid, field)
    assert old['institutions'] == new['institutions']
    org_id = 'in_eci_20240323_np_04'
    assert [o for o in old['organizations'] if o['id'] != org_id] == [
        o for o in new['organizations'] if o['id'] != org_id]
    assert old['coverage']['unresolved'][:-1] == new['coverage']['unresolved'][:-1]
    assert old['research_cutoff'] == new['research_cutoff'] == '2026-09-07'

    def holders(packet):
        org = next(o for o in packet['organizations'] if o['id'] == org_id)
        return next(r for r in org['roles'] if r['id'] == 'in_cpm_general_secretary')['holder_claims']

    before, after = holders(old), holders(new)
    assert len(before) == len(after) == 4
    for previous, current in zip(before, after):
        for field in ('name', 'from', 'until'):
            assert previous[field] == current[field], (field, previous['name'])
        if previous['name'] in scope['selected_dates']:
            dates = scope['selected_dates'][previous['name']]
            assert previous['attested_on'] == dates['before']
            assert current['attested_on'] == dates['after']
        else:
            assert previous == current
    for sid in changed | set(scope['new_sources']):
        source = new_sources[sid]
        raw = git_blob(repo, revision, source['snapshot']['path'])
        check_bytes(raw, source['snapshot'])
        assert b'\r' not in raw, 'Factual extract must retain LF canonical JSON'
        extract = json.loads(raw)
        assert extract['scope_note'] == source['scope_note']
        for row in extract['rows']:
            claim = new_claims[row['claim_id']]
            assert row['text'] == claim['text'] and row['locator'] == claim['locator']
        if sid in changed:
            prior = json.loads(git_blob(repo, baseline, source['snapshot']['path']))
            assert len(prior['rows']) == len(extract['rows'])
            for a, b in zip(prior['rows'], extract['rows']):
                assert {k: v for k, v in a.items() if k != 'event_kind'} == {
                    k: v for k, v in b.items() if k != 'event_kind'}
                if a['claim_id'] in scope['reclassified_claim_ids']:
                    assert b['event_kind'] == 'newly_elected_styling'
                else:
                    assert a['event_kind'] == b['event_kind']


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('--repo', type=Path, help='Repository containing the pinned source and base commits')
    parser.add_argument('--new-originals', type=Path, help='Relocated two-source amendment evidence directory')
    parser.add_argument('--prior-originals', type=Path, help='Relocated prior evidence directory with attempt-01')
    parser.add_argument('--require-validated', action='store_true', help='Fail if integration validation is pending')
    args = parser.parse_args()
    manifest = load('manifest.json')
    expected = {row['path'] for row in manifest['files']}
    assert len(expected) == len(manifest['files'])
    actual = {p.relative_to(HERE).as_posix() for p in HERE.rglob('*') if p.is_file()
              and p.name != 'manifest.json' and '__pycache__' not in p.parts}
    assert actual == expected, {'missing': sorted(expected - actual), 'extra': sorted(actual - expected)}
    for row in manifest['files']:
        check_bytes(within(HERE, row['path']).read_bytes(), row)
    sources, claims, scope = load('source-review.json'), load('claim-review.json'), load('scope-review.json')
    assert len(sources['sources']) == 5 and sources['new_originals'] == 2
    assert len(claims['claims']) == 9 and claims['new_claims_reviewed'] == 2
    assert {c['claim_id'] for c in claims['claims']} == {
        cid for source in sources['sources'] for cid in source['claim_ids']}
    assert len(claims['holder_changes']) == 2
    assert (HERE / '.gitattributes').read_text().splitlines() == [
        '* -text whitespace=cr-at-eol', '*.log -whitespace', '*.patch -whitespace']
    guard = load('guard/result.json')
    assert [(r['tests_run'], r['failures'], r['errors']) for r in guard['runs']] == [(1, 3, 0), (3, 0, 0)]
    for run in guard['runs']:
        raw = (HERE / 'guard' / (run['label'] + '.log')).read_bytes()
        assert hashlib.sha256(raw).hexdigest() == run['log_sha256']
    assert hashlib.sha256((HERE / 'guard/claim-driven-guard.patch').read_bytes()).hexdigest() == guard['patch_sha256']
    external_checked = 0
    roots = {'new_originals': args.new_originals, 'prior_originals': args.prior_originals}
    for row in sources['external_artifacts']:
        root = roots[row['group']]
        if root is not None:
            check_bytes(within(root, row['path']).read_bytes(), row)
            external_checked += 1
    if args.repo:
        check_repo(args.repo, scope)
    validation = load('validation/status.json')
    for recorded in validation['known_results']:
        runs = load('validation/' + recorded['receipt'])
        for run in runs:
            check_bytes(within(HERE / 'validation', run['log']).read_bytes(), run)
            assert run['exit_code'] == recorded['exit_code'], recorded['receipt']
    if args.require_validated:
        assert validation['status'] == 'passed_bounded_integration_checks', validation['status']
    print(json.dumps({'receipt_files_checked': len(expected), 'external_artifacts_checked': external_checked,
        'reviewed_source_commit': scope['reviewed_source_commit'], 'repository_scope_checked': bool(args.repo),
        'integration_validation_status': validation['status'], 'historical_content_review_repeated': False,
        'parent_qualification_claimed': False}, indent=2))


if __name__ == '__main__':
    main()
