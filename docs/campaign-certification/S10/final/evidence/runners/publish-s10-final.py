"""Publish passed S10 gameplay qualification without inferring scope approval.

Default --qualification-only retains S10/G2 as open. A recorded, exact user
answer is required for --approval-receipt. --upgrade-approval changes only
closure metadata and stores that receipt; existing raw evidence is immutable.
No stage, commit, push, game command, test run or approval prompt is performed.
"""
import argparse
import datetime
import gzip
import hashlib
import importlib.util
import json
import pathlib
import re
import subprocess

BASE = pathlib.Path(__file__).resolve().parent
REPO = BASE / 'integration'
PIN = 'bcdf72bcbe8947966359deb95f715cb526a68def'
GAME = '0f4616477671bc01936fcc9f2877ae172b13e348'
DEST = REPO / 'docs/campaign-certification/S10/final'
QUESTION = ('The approved roadmap makes S10 depend on completing the worldwide party census. '
    'Should I finish S10’s government/diplomacy gameplay checks now and keep exhaustive leadership '
    'research in C01–C07, with full campaign certification still requiring that content?')
ANSWER = 'Finish S10 gameplay; keep content certification separate (Recommended)'
NATIONS = ['France', 'Japan', 'India', 'Brazil', 'SouthAfrica', 'Tonga', 'SaudiArabia', 'USSR']
SCOPE = ('Selected complete final S10 Windows/Linux/content/interface logs and proof records; '
    'all eight-country and ordinary France browser output files, original campaign captures, named '
    'saves and canonical worlds; scoped references to retained S10.d–g evidence; preservation and '
    'optional fixed review launch records. Large files are compressed with exact reconstruction. '
    'Compiled binaries are identified by SHA-256 and source revision, not repackaged. '
    'No new elapsed campaign, Russia activation, complete historical content or performance claim.')


def read(path):
    return json.loads(path.read_text(encoding='utf-8-sig'))


def sha(path):
    with pathlib.Path(path).open('rb') as stream:
        return hashlib.file_digest(stream, 'sha256').hexdigest()


def digest(raw):
    return hashlib.sha256(raw).hexdigest()


def write(path, value):
    path.write_text(json.dumps(value, ensure_ascii=False, indent=2) + '\n', encoding='utf-8', newline='\n')


def git(*args):
    return subprocess.check_output(['git', '-c', 'core.longpaths=true', *args], cwd=REPO)


def record(path):
    path = pathlib.Path(path).resolve()
    return {'path': str(path), 'bytes': path.stat().st_size, 'sha256': sha(path)}


def exact(row, parent=None):
    path = pathlib.Path(row.get('path', row.get('file', '')))
    if parent is not None and not path.is_absolute():
        path = parent / path
    assert path.is_file(), 'Missing required file: ' + str(path)
    assert sha(path) == row['sha256'], 'Changed bytes: ' + str(path)
    if 'bytes' in row:
        assert path.stat().st_size == row['bytes'], str(path)
    return path.resolve()


def same_pin(source):
    assert source['revision'] == PIN and source.get('runtime_source_equal') is True


def positive(value):
    assert isinstance(value, int) and not isinstance(value, bool) and value > 0


def rust_totals(text):
    rows = [tuple(map(int, x)) for x in re.findall(
        r'test result: (?:ok|FAILED)\. (\d+) passed; (\d+) failed; (\d+) ignored;', text)]
    assert rows, 'No native completion summaries'
    result = dict(passed=sum(r[0] for r in rows), failed=sum(r[1] for r in rows),
                  ignored=sum(r[2] for r in rows), completed_targets=len(rows))
    positive(result['passed'])
    assert result['failed'] == 0
    return result


def node_totals(text, linux=False):
    parsed = {}
    for key in ['tests', 'pass', 'fail', 'skipped']:
        values = re.findall(r'(?:ℹ|#)\s+' + key + r'\s+(\d+)', text)
        assert values, 'Missing Node ' + key
        parsed[key] = int(values[-1])
    assert parsed['fail'] == 0 and parsed['tests'] == parsed['pass'] + parsed['skipped']
    positive(parsed['pass'])
    if linux:
        return dict(tests=parsed['tests'], passed=parsed['pass'], failed=parsed['fail'], skipped=parsed['skipped'])
    return parsed


def current_runner(lane):
    path = BASE / ('evidence/S10-final-' + lane + '.json')
    data = read(path)
    assert data['passed'] is True and data['exit_code'] == 0
    assert data['revision'] == data['revision_after'] == PIN
    assert data['clean_before'] is True and data['clean_after'] is True
    assert data['runner_sha256'] == sha(BASE / 'run-s10-final-check.py')
    log = path.with_suffix('.log')
    assert sha(log) == data['log_sha256']
    if lane in ('web', 'agency', 'leadership'):
        assert rust_totals(log.read_text(encoding='utf-8')) == data['totals']
    if lane in ('binary', 'browser', 'matrix'):
        exact(data['binary'])
    if lane in ('browser', 'matrix'):
        assert data['binary_sha256_before'] == data['binary']['sha256']
    return path, data


def summarized_results(log):
    results = []
    for line in log.read_text(encoding='utf-8-sig').splitlines():
        try:
            row = json.loads(line)
        except ValueError:
            continue
        if isinstance(row, dict) and row.get('result'):
            path = pathlib.Path(row['result']).resolve()
            assert path.is_relative_to(BASE / 'evidence') and row.get('passed') is True
            if row.get('result_sha256'):
                assert sha(path) == row['result_sha256']
            results.append(path)
    assert results, 'No declared completed browser result'
    return results


def build_assets(data, driver, binary):
    same_pin(data['test_source'])
    assert data['test_source']['driver_sha256'] == sha(REPO / driver)
    assert data['test_source']['review_assertions_sha256'] == sha(REPO / 'tools/ui/government-review-assertions.cjs')
    build = data['build']
    assert build['revision'] == PIN and build['build']['revision'] == PIN[:12]
    assert build['binary_sha256'] == binary
    for name, asset in build['assets'].items():
        path = REPO / 'spheres-web/ui' / name
        assert sha(path) == asset['checkout_sha256'] == asset['served_sha256']
        assert path.stat().st_size == asset['served_bytes']
        if 'checkout_bytes' in asset:
            assert path.stat().st_size == asset['checkout_bytes']
        committed = git('show', PIN + ':spheres-web/ui/' + name)
        assert digest(committed) == asset['committed_sha256']
        assert path.read_bytes().replace(b'\r\n', b'\n') == committed


def audit_archives(directory, expected, pairs):
    paths = sorted((directory / 'archive-audit').glob('*.json'))
    assert len(paths) == expected, 'Unexpected native capture count: ' + str(directory)
    rows = []
    for path in paths:
        data = read(path)
        assert data['canonical']['ignored_paths'] == []
        for key in ('input', 'canonical'):
            actual = exact(data[key])
            assert actual.is_relative_to(directory), 'Capture escapes selected browser output'
        rows.append(data)
    for left, right in pairs:
        a, b = (pathlib.Path(rows[i - 1]['canonical']['path']) for i in (left, right))
        assert a.read_bytes() == b.read_bytes(), f'Native world differs: {directory} {left}/{right}'
    return {'inspections': expected, 'exact_comparisons': len(pairs), 'ignored_paths': []}


def content_ui():
    directory = BASE / 'evidence/S10-final-content-ui'
    data = read(directory / 'result.json')
    assert data['passed'] is True and data['source_revision'] == data['source_revision_after'] == PIN
    assert data['source_clean_before'] is True and data['source_clean_after'] is True
    assert data['runner_sha256'] == sha(BASE / 'run-s10-final-content-ui.py')
    assert data['game_runtime_unchanged'] is True and data['game_runtime_revision'] == GAME
    assert len(data['checks']) == 11
    content = 0
    node = None
    for check in data['checks']:
        path = directory / check['log']
        assert check['exit_code'] == 0 and sha(path) == check['log_sha256']
        text = path.read_text(encoding='utf-8')
        if check['name'].endswith('-tests'):
            assert check['reported_ok'] and re.search(r'^OK\s*$', text, re.M)
            count = int(re.findall(r'Ran (\d+) tests? in', text)[-1])
            assert count == check['tests']
            content += count
        if check['name'] == 'interface':
            node = node_totals(text)
            assert node == check['totals']
    assert content == 102 and node is not None
    browser = read(directory / 'browser/result.json')
    assert browser['passed'] and browser['clean_before'] and browser['clean_after']
    assert browser['revision'] == browser['revision_after'] == PIN
    assert not browser['errors'] and browser['source_files_unchanged'] and len(browser['countries']) == 9
    assert all(r['method'] == 'GET' and '/api/' not in r['url'] for r in browser['requests'])
    assert browser['keyboard']['summary_reached_with_tab'] and browser['keyboard']['opened_with_enter']
    assert browser['empty_country']['party_count_matches_inventory']
    assert browser['empty_country']['undiscovered_organization_count'] == 'Unknown'
    assert len(browser['authored_error_cases']) == 2
    for name, hashed in browser['source_files'].items():
        assert sha(REPO / name) == hashed
    return directory, data, content, node


def linux_checks():
    directory = BASE / 'evidence/S10-final-linux'
    data = read(directory / 'result.json')
    assert data['passed'] is True and data['status'] == 'passed' and data['revision'] == PIN
    assert data['runner_sha256'] == data['runner_sha256_after'] == sha(BASE / 'run-s10-final-linux.py')
    assert data['source_before'] == data['source_after'] == {'head': PIN, 'status': ''}
    assert sha(directory / 'toolchain.json') == data['toolchain_sha256']
    assert [r['name'] for r in data['checks']] == ['web', 'agency', 'leadership', 'node']
    result = []
    for check in data['checks']:
        assert check['passed'] is True and check['exit_code'] == 0
        assert check['source_before'] == check['source_after'] == {'head': PIN, 'status': ''}
        path = directory / check['log']
        assert sha(path) == check['log_sha256']
        text = path.read_text(encoding='utf-8')
        totals = node_totals(text, linux=True) if check['name'] == 'node' else rust_totals(text)
        assert totals == check['totals']
        result.append({'name': check['name'], 'totals': totals})
    return directory, data, result


def acceptance_audit():
    path = BASE / 'evidence/S10-final-acceptance-audit.json'
    data = read(path)
    assert data['format'] == 'spheres-s10-final-acceptance-source-audit/v1'
    assert data['passed'] is True and data['status'] == 'passed'
    assert data['source_revision'] == PIN and data['source_clean'] is True
    assert data['gate_decision'] == 'not_awarded_by_this_receipt'
    assert all(data[k] is False for k in ['s10_complete', 'g2_earned', 'c01_complete', 's11_started'])
    exact(data['runner'])
    assert pathlib.Path(data['runner']['path']).resolve() == (BASE / 'record-s10-final-acceptance-audit.py').resolve()
    assert len(data['acceptance_clauses']) == 3 and all(r['status'] == 'passed' for r in data['acceptance_clauses'])
    assert all(r['status'] == 'passed' for r in data['current_qualification'])
    for check in data['current_qualification']:
        if 'receipt' in check:
            exact(check['receipt'])
        for receipt in check.get('receipts', []):
            path_receipt = exact(receipt)
            assert path_receipt.name == 'S10-final-' + check['id'].removeprefix('windows_') + '.json', 'Historical/repeated lane not selected'
            exact(receipt['log'])
    retained = data['retained_authored_evidence']
    assert [r['increment'] for r in retained] == ['S10.d', 'S10.e', 'S10.f', 'S10.g']
    for previous in retained:
        assert previous['new_execution'] is False
        for key in ('report', 'manifest', 'inventory'):
            exact(previous[key])
        for result in previous['authored_results']:
            assert result['passed'] and result['reconstruction_verified']
            raw = b''.join(exact(part).read_bytes() for part in result['storage'])
            if result['encoding'] == 'gzip':
                raw = gzip.decompress(raw)
            assert len(raw) == result['bytes'] and digest(raw) == result['sha256']
        for comparison in previous['comparisons']:
            assert comparison['current_revision'] == PIN
            patch = git(*comparison['git_arguments'])
            assert len(patch) == comparison['git_diff_bytes']
            assert digest(patch) == comparison['git_diff_sha256']
            assert (not patch) == comparison['matches']
        comp = {c['label']: c['matches'] for c in previous['comparisons']}
        assert comp['scoped_feature_source'] and comp['authored_driver_direct_relative_require_graph']
        if previous['increment'] == 'S10.g':
            assert comp['complete_game_runtime']
    return path, data


def preservation(path):
    data = read(path)
    assert data['passed'] is True and len(data['files']) == 8 and len(data['worktrees']) == 2
    for row in data['files']:
        assert row['unchanged'] is True
        exact(row)
    for row in data['worktrees']:
        assert row['unchanged'] is True and row['head'] == row['expected']
        head = subprocess.check_output(['git', '-c', 'core.longpaths=true', '-C', row['path'], 'rev-parse', 'HEAD'], text=True).strip()
        assert head == row['expected']
    return data


def approval_receipt(path):
    data = read(path)
    assert data['question'] == QUESTION, 'Receipt does not match the pending question'
    assert data['answer'] == ANSWER, 'The scope change was not explicitly accepted'
    received = datetime.datetime.fromisoformat(data['received_utc'].replace('Z', '+00:00'))
    assert received.tzinfo is not None
    assert received <= datetime.datetime.now(datetime.timezone.utc), 'Approval cannot be future-dated'
    return {key: data[key] for key in ('question', 'answer', 'received_utc')}


def selected_tree(inputs, source, name):
    source = pathlib.Path(source).resolve()
    assert source.is_dir()
    files = list(source.rglob('*'))
    assert any(p.is_file() for p in files)
    assert not any(p.suffix.lower() in ('.exe', '.dll', '.pdb') for p in files if p.is_file()), 'Compiled binary in evidence selection'
    inputs.append({'source': str(source), 'name': name})


def selected_file(inputs, source, name):
    path = pathlib.Path(source).resolve()
    assert path.is_file()
    inputs.append({'source': str(path), 'name': name})


def verify_inventory(directory, expected_hash=None):
    path = directory / 'inventory.json'
    if expected_hash is not None:
        assert sha(path) == expected_hash
    assert b'\r' not in path.read_bytes(), 'Inventory must use LF'
    inventory = read(path)
    assert inventory['scope'] == SCOPE
    seen = {'inventory.json'}
    logical = set()
    total = 0
    for row in inventory['files']:
        assert row['logical_path'] not in logical
        logical.add(row['logical_path'])
        pieces = []
        for part in row['storage']:
            assert part['file'] not in seen
            target = (directory / part['file']).resolve()
            assert target.is_relative_to(directory.resolve())
            seen.add(part['file'])
            exact(part, directory)
            total += part['bytes']
            pieces.append(target.read_bytes())
        raw = b''.join(pieces)
        assert row['encoding'] in ['raw', 'gzip']
        if row['encoding'] == 'gzip':
            raw = gzip.decompress(raw)
        assert digest(raw) == row['sha256'] and len(raw) == row['bytes']
        assert row['reconstruction_verified'] is True
    assert total == inventory['stored_bytes']
    assert seen == {p.relative_to(directory).as_posix() for p in directory.rglob('*') if p.is_file()}
    return inventory


def readme(manifest):
    q = manifest['qualification']
    approved = manifest['approval'] is not None
    status = ('**S10 gameplay complete; G2 earned.** C01 remains incomplete and S11 is planned.' if approved else
        '**Gameplay qualification passed; S10 and G2 remain open pending the scope decision.** C01 is incomplete and S11 is planned.')
    scope = ('The recorded user answer explicitly separates S10 government/diplomacy gameplay closure from '
        'the C01 worldwide census. C01–C07 continue the historical and cartoon work. C06 and S23 still require '
        'the eight certified country casts before CP1 qualification. No missing identity, historical interval '
        'or portrait is waived.' if approved else
        'The approved roadmap still lists C01 as an S10 prerequisite. These passed gameplay checks do not '
        'remove that dependency or award G2. The user’s decision on separating gameplay closure from '
        'content certification is still pending; no answer is inferred from elapsed time.')
    linux = {r['name']: r['totals'] for r in q['linux_checks']}
    link = f"[Open the current review build]({manifest['review_url']}).\n\n" if manifest.get('review_url') else ''
    return f'''# S10 — Final government, succession and diplomacy gameplay qualification

{status}

{link}{scope}

## What is qualified

Ordinary government decisions are reviewed, confirmed and reported at the current date.
Parliamentary, presidential, authoritarian and institutional cases preserve saved incumbents;
historical browsing and future candidate eligibility do not silently appoint someone.
Native checks cover offers, deadlines, standing policies, sanctions, party/executive roles,
unaffordable or stale reviews and foreign inspection. Existing military and content sessions
retain their original downstream requirements.

The eight ordinary starting cases are France, Japan, India, Brazil, South Africa, Tonga,
Saudi Arabia and USSR. Each confirms one available paid native decision without grants,
retains its incumbent and survives named Save/Load/Continue. These are 1 January 1990
checks with **zero elapsed campaign days**. USSR is a starting case; this is not a Russia
activation test or the later eight-country, three-seed 1990–2035 campaign matrix.

## Current candidate and checks

Candidate: `{PIN}`. Windows executable SHA-256:
`{manifest['binary_sha256']}`.

The complete game source matches `{GAME}` exactly. The new executable embeds the current
candidate revision; its hash is therefore distinct from the earlier S10.g executable.
The following checks were run again on the clean current candidate:

| Check | Result |
| --- | --- |
| Windows native web | {q['windows_web']['passed']} passed; {q['windows_web']['ignored']} ignored |
| Windows agency/succession | {q['windows_agency']['passed']} passed; {q['windows_agency']['ignored']} ignored |
| Windows party leadership | {q['windows_leadership']['passed']} passed; {q['windows_leadership']['ignored']} ignored |
| Windows interface | {q['windows_interface']['pass']} passed; {q['windows_interface']['skipped']} skipped |
| Linux native web | {linux['web']['passed']} passed; {linux['web']['ignored']} ignored |
| Linux agency/succession | {linux['agency']['passed']} passed; {linux['agency']['ignored']} ignored |
| Linux party leadership | {linux['leadership']['passed']} passed; {linux['leadership']['ignored']} ignored |
| Linux interface | {linux['node']['passed']} passed; {linux['node']['skipped']} skipped |
| Content | {q['content_tests']} tests; five reproduction checks |
| Government browser matrix | Eight countries; 32 complete native-world inspections; 16 exact comparisons |
| Ordinary France browser | Eight inspections; five exact comparisons; real second-tab stale refusal |

All reported checks pass; ignored and skipped tests are not counted as passed. The
read-only research atlas was also tested across all nine partial certification-identity
packets, including 1440/390/320px layouts, keyboard behavior and authored integrity/race
refusals. It grants no leadership eligibility and changes no campaign.

## Reused evidence and limits

The [acceptance audit](evidence/review/S10-final-acceptance-audit.json) links each written
S10 gameplay clause to the current checks and scoped retained S10.d–g evidence. Original
manifests, inventories and authored result bytes are verified and linked; old raw archives
remain in their original checkpoints. They are not copied or counted as new runs here.
S10.d/e retain their original feature scope because later shared presentation changed.
S10.f permits narrower exact succession-source reuse. S10.g permits complete-game-source
reuse. Its preview benchmark remains its original run; no new performance measurement,
long campaign, old-save matrix, Russia activation or complete historical-content claim is made.

Research remains nine partial packets, 841 organization observations, 27 institutions,
58 sources, 1,606 claims and 92 open discovery batches. There are zero completed exhaustive
censuses. Those figures are observations, not distinct-party totals or completed casts.

Eight protected save files retain their recorded bytes and two baseline worktree HEADs
remain unchanged. Those checks do not assert that every file in either worktree or every
previous live server was inventoried. A supplied review launch is a separate copied France
save whose canonical world exactly matches the final ordinary browser result.

The [manifest](manifest.json) records all status flags and exact scope. The
[inventory](evidence/inventory.json) retains {manifest['evidence']['logical_files']} logical
files and {manifest['evidence']['stored_bytes']:,} stored bytes, excluding itself. Large
files reconstruct exactly; binaries are identified rather than copied. S11 has not started,
and full campaign and release certification are not awarded by this report.
'''


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    mode = parser.add_mutually_exclusive_group()
    mode.add_argument('--qualification-only', action='store_true')
    mode.add_argument('--approval-receipt', type=pathlib.Path)
    parser.add_argument('--upgrade-approval', action='store_true')
    parser.add_argument('--launch', type=pathlib.Path)
    parser.add_argument('--visual-review', type=pathlib.Path)
    parser.add_argument('--preflight', action='store_true', help='Validate only; write nothing')
    args = parser.parse_args()
    approval = approval_receipt(args.approval_receipt.resolve()) if args.approval_receipt else None
    assert git('rev-parse', 'HEAD').decode().strip() == PIN
    if args.upgrade_approval:
        assert approval is not None and not args.qualification_only
        assert not args.launch and not args.visual_review, 'Raw evidence cannot be added during approval upgrade'
        manifest = read(DEST / 'manifest.json')
        assert manifest['status'] == 'gameplay_qualified_scope_pending'
        assert manifest['candidate_revision'] == PIN and manifest['qualification']['passed'] is True
        assert all(manifest[k] is False for k in ['s10_complete', 'c01_complete', 'g2_earned', 's11_started'])
        inventory = verify_inventory(DEST / 'evidence', manifest['evidence']['inventory_sha256'])
        assert not (DEST / 'approval.json').exists()
        manifest.update(status='s10_gameplay_complete', s10_complete=True, g2_earned=True, approval=approval)
        manifest['approval_receipt'] = {**record(args.approval_receipt), 'stored_path': 'approval.json'}
        manifest['scope_amended_utc'] = datetime.datetime.now(datetime.timezone.utc).isoformat()
        if not args.preflight:
            (DEST / 'approval.json').write_bytes(args.approval_receipt.read_bytes())
            write(DEST / 'manifest.json', manifest)
            (DEST / 'README.md').write_text(readme(manifest), encoding='utf-8', newline='\n')
            assert sha(DEST / 'evidence/inventory.json') == manifest['evidence']['inventory_sha256']
        print(json.dumps({'upgrade_applied': not args.preflight, 'raw_evidence_unchanged': True,
            'status': manifest['status'], 'inventory_sha256': manifest['evidence']['inventory_sha256']}, indent=2))
        return
    assert not DEST.exists(), 'Final checkpoint exists; use the explicit approval-only upgrade when appropriate'
    assert not git('status', '--porcelain'), 'Initial publication requires the exact clean candidate'
    assert not git('diff', '--binary', '--no-ext-diff', '--no-textconv', GAME, PIN, '--',
                   'Cargo.toml', 'Cargo.lock', 'spheres-sim', 'spheres-web')
    windows = {name: current_runner(name) for name in ['web', 'agency', 'leadership', 'binary', 'browser', 'matrix']}
    binary = windows['binary'][1]['binary']['sha256']
    assert all(windows[name][1]['binary']['sha256'] == binary for name in ['browser', 'matrix'])
    content_dir, content, content_count, node = content_ui()
    linux_dir, linux, linux_summary = linux_checks()
    audit_path, audit = acceptance_audit()
    before = BASE / 'evidence/S10-final-preservation-before.json'
    after = BASE / 'evidence/S10-final-preservation-final.json'
    preservation_before, preservation_after = preservation(before), preservation(after)
    assert preservation_before['baseline_sha256'] == preservation_after['baseline_sha256']
    matrix_paths = summarized_results(windows['matrix'][0].with_suffix('.log'))
    assert len(matrix_paths) == 9
    matrix_path = matrix_paths[-1]
    matrix = read(matrix_path)
    assert matrix['passed'] and matrix['full_eight_country_matrix'] and matrix['full_matrix_requested']
    assert matrix['runtime_revision'] == PIN and matrix['binary_sha256'] == binary
    same_pin(matrix['test_source'])
    assert matrix['requested_nations'] == NATIONS and [r['nation'] for r in matrix['cases']] == NATIONS
    assert len(matrix['cases']) == 8
    matrix_details = []
    for case, expected_path in zip(matrix['cases'], matrix_paths[:-1]):
        assert case['passed'] and pathlib.Path(case['result']).resolve() == expected_path
        assert sha(expected_path) == case['result_sha256']
        data = read(expected_path)
        assert data['passed'] and not data['errors'] and data['owned_server_stopped']
        assert data['nation'] == case['nation'] and len(data['commands']) == 1
        assert data['final_state']['date'] == '1 Jan 1990'
        build_assets(data, 'tools/ui/ci-government-country-matrix.cjs', binary)
        captures = audit_archives(expected_path.parent, 4, [(1, 2), (3, 4)])
        matrix_details.append({'nation': case['nation'], 'result': record(expected_path), 'archives': captures,
            'confirmed_commands': 1, 'elapsed_days': 0, 'date': data['final_state']['date']})
    ordinary_paths = summarized_results(windows['browser'][0].with_suffix('.log'))
    assert len(ordinary_paths) == 1
    ordinary_path = ordinary_paths[0]
    ordinary = read(ordinary_path)
    assert ordinary['passed'] and not ordinary['errors'] and len(ordinary['commands']) == 3
    assert ordinary['final_state']['date'] == '1 Jan 1990'
    build_assets(ordinary, 'tools/ui/ci-government-decisions.cjs', binary)
    ordinary_archives = audit_archives(ordinary_path.parent, 8, [(1, 2), (3, 4), (5, 6), (5, 7), (5, 8)])
    inputs = []
    for name, (path, _) in windows.items():
        selected_file(inputs, path, 'windows/' + path.name)
        selected_file(inputs, path.with_suffix('.log'), 'windows/' + path.with_suffix('.log').name)
    selected_tree(inputs, content_dir, 'content-ui')
    selected_tree(inputs, linux_dir, 'linux')
    selected_tree(inputs, matrix_path.parent, 'government-matrix/' + matrix_path.parent.name)
    selected_tree(inputs, ordinary_path.parent, 'ordinary-browser/' + ordinary_path.parent.name)
    for path in [audit_path, before, after]:
        selected_file(inputs, path, 'review/' + path.name)
    launch = None
    if args.launch:
        args.launch = args.launch.resolve()
        launch = read(args.launch)
        assert launch['runtime_revision'] == PIN and launch['executable_sha256'] == binary
        assert launch['native_verification']['exact_bytes_match_final_browser'] is True
        assert launch['native_verification']['ignored_paths'] == []
        for proof in launch['evidence']:
            exact(proof)
        source = exact({'path': launch['save_copy']['source'], 'sha256': launch['save_copy']['sha256']})
        copy = exact({'path': launch['save_copy']['copy'], 'sha256': launch['save_copy']['sha256']})
        assert source.is_relative_to(ordinary_path.parent)
        directory = pathlib.Path(launch['directory'])
        selected_file(inputs, args.launch, 'review/' + args.launch.name)
        selected_file(inputs, copy, 'review/review-save.json')
        selected_tree(inputs, directory / 'native-verification', 'review/native-verification')
    visual = None
    if args.visual_review:
        args.visual_review = args.visual_review.resolve()
        visual = read(args.visual_review)
        assert visual['passed'] is True
        assert visual.get('candidate_revision', visual.get('runtime_revision')) == PIN
        images = visual.get('images', visual.get('screenshots'))
        assert images
        for screenshot in images:
            path = exact(screenshot)
            assert any(path.is_relative_to(pathlib.Path(i['source'])) for i in inputs if pathlib.Path(i['source']).is_dir()), 'Visual image is outside selected evidence'
        selected_file(inputs, args.visual_review, 'review/' + args.visual_review.name)
    runner_names = ['run-s10-final-check.py', 'run-s10-final-content-ui.py', 'run-s10-final-linux.py',
        'record-s10-final-acceptance-audit.py', 'verify-s08-preservation.py', 'collect-s09-evidence.py',
        'publish-s10-final.py', 'S10-final-doc-updates.py',
        'verify-s10-final-publication.py', 'record-s10-final-pending-status.py']
    if launch:
        runner_names.append('launch-s10-final-review.py')
    if visual:
        runner_names.append('record-s10-final-visual-review.py')
    for name in runner_names:
        selected_file(inputs, BASE / name, 'runners/' + name)
    counts = read(REPO / 'docs/campaign-certification/C01/research-index.json')['counts']
    assert counts == {'country_packets': 9, 'countries_without_new_discovery_packet': 151,
        'organization_observations': 841, 'institution_observations': 27, 'sources': 58,
        'source_claims': 1606, 'discovery_batches': 92, 'exhaustive_country_censuses': 0}
    q = {'passed': True, 'windows_web': windows['web'][1]['totals'],
        'windows_agency': windows['agency'][1]['totals'], 'windows_leadership': windows['leadership'][1]['totals'],
        'windows_interface': node, 'linux_checks': linux_summary, 'content_tests': content_count,
        'content_reproduction_checks': 5, 'static_research_browser_country_packets': 9,
        'browser_countries': 8, 'browser_elapsed_days': 0, 'ordinary_browser_passed': True,
        'ordinary_browser': ordinary_archives, 'ordinary_browser_elapsed_days': 0,
        'government_country_matrix': matrix_details, 'matrix_inspections': 32, 'matrix_exact_comparisons': 16,
        'manual_visual_review': visual is not None, 'new_performance_claim': False, 'new_long_campaign_claim': False,
        'new_historical_coverage_claim': False, 'new_russia_activation_claim': False,
        'not_refreshed': ['External old-save matrix', 'Performance measurements', 'Long campaigns', 'Russia activation']}
    manifest = {'format': 'spheres-s10-final/v1',
        'status': 's10_gameplay_complete' if approval else 'gameplay_qualified_scope_pending',
        'created_utc': datetime.datetime.now(datetime.timezone.utc).isoformat(), 'candidate_revision': PIN,
        'game_source_identical_to': GAME, 'complete_game_source_changed': False,
        'compiled_revision': PIN, 'binary_sha256': binary, 's10_complete': approval is not None,
        'c01_complete': False, 'g2_earned': approval is not None, 's11_started': False,
        'approval': approval, 'pending_scope_question': None if approval else QUESTION,
        'qualification': q, 'research_counts': counts,
        'acceptance_audit': record(audit_path), 'retained_authored_evidence': audit['retained_authored_evidence'],
        'content_gate_requirements_retained': ['C01–C07', 'C06', 'S23', 'G5', 'CP1', 'WC1'],
        'review_url': launch['url'] if launch else None, 'scope': SCOPE}
    if approval:
        manifest['approval_receipt'] = {**record(args.approval_receipt), 'stored_path': 'approval.json'}
    spec = importlib.util.spec_from_file_location('s10_final_evidence_collector', BASE / 'collect-s09-evidence.py')
    collector = importlib.util.module_from_spec(spec)
    spec.loader.exec_module(collector)
    plan = collector.preflight(inputs, DEST / 'evidence')
    if args.preflight:
        print(json.dumps({'preflight_passed': True, 'status_if_published': manifest['status'],
            'candidate_revision': PIN, 'selected_logical_files': len(plan), 'qualification': q}, indent=2))
        return
    # Every required proof is complete before the first repository write.
    selection = BASE / 'evidence/S10-final-evidence-selection.json'
    assert not selection.exists(), 'Evidence selection already exists'
    write(selection, inputs)
    DEST.mkdir(exist_ok=False)
    collector.collect(selection, DEST / 'evidence')
    inventory_path = DEST / 'evidence/inventory.json'
    inventory = read(inventory_path)
    inventory['scope'] = SCOPE
    write(inventory_path, inventory)  # Metadata only; collected source bytes remain untouched.
    inventory = verify_inventory(DEST / 'evidence')
    manifest['evidence'] = {'inventory_sha256': sha(inventory_path),
        'logical_files': len(inventory['files']), 'stored_bytes': inventory['stored_bytes']}
    (DEST / '.gitattributes').write_text('evidence/** -text -whitespace\n', encoding='utf-8', newline='\n')
    if approval:
        (DEST / 'approval.json').write_bytes(args.approval_receipt.read_bytes())
    write(DEST / 'manifest.json', manifest)
    (DEST / 'README.md').write_text(readme(manifest), encoding='utf-8', newline='\n')
    assert git('rev-parse', 'HEAD').decode().strip() == PIN
    print(json.dumps({'published': str(DEST), 'status': manifest['status'],
        'candidate_revision': PIN, 'evidence': manifest['evidence'],
        'staged': False, 'committed': False, 's10_complete': manifest['s10_complete'],
        'c01_complete': False, 'g2_earned': manifest['g2_earned']}, indent=2))


if __name__ == '__main__':
    main()
