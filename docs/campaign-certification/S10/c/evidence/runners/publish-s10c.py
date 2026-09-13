"""Publish S10.c only after explicit final proofs pass. Prepared; never runs on import.

This writes NEW S10/c evidence and updates only the parent S10 status summary and
the two pathway representations. It does not stage, commit, push, build or run a
game. Runtime provenance is a supplied immutable pin; the containing publication
commit is resolved later through Git, never invented inside its own manifest.
"""
import argparse
import datetime
import hashlib
import importlib.util
import json
import pathlib
import re
import subprocess

BASE = pathlib.Path(__file__).resolve().parent
REPO = BASE / 'integration'
DEST = REPO / 'docs/campaign-certification/S10/c'
RUNTIME_PATHS = ('spheres-sim', 'spheres-cli', 'spheres-web', 'Cargo.toml', 'Cargo.lock')
CONTENT_PATHS = ('tools/avatars', 'docs/campaign-certification/C01', 'spheres-web/data')
SIM_BASELINE = 'd361c2d567b0704bdffb5160760c87a336b0c7c9'
COUNTRIES = {'France', 'Japan', 'India', 'Brazil', 'SouthAfrica', 'Tonga', 'SaudiArabia', 'USSR'}
EXPECTED_COUNTS = {'organization_observations': 644, 'institution_observations': 11,
                   'sources': 31, 'source_claims': 1315, 'discovery_batches': 67,
                   'exhaustive_country_censuses': 0}


def load(path):
    return json.loads(pathlib.Path(path).read_text(encoding='utf-8-sig'))


def sha(path):
    with pathlib.Path(path).open('rb') as stream:
        return hashlib.file_digest(stream, 'sha256').hexdigest()


def git(*args):
    return subprocess.check_output(['git', '-c', 'core.longpaths=true', *args], cwd=REPO, text=True).strip()


def path_arg(value):
    path = pathlib.Path(value)
    return (path if path.is_absolute() else BASE / path).resolve()


def source_equal(first, second, paths=RUNTIME_PATHS):
    assert re.fullmatch('[0-9a-f]{40}', first or ''), 'Missing exact source revision'
    assert not git('diff', '--name-only', first, second, '--', *paths), 'Source differs: ' + first


def passed(data, label):
    assert data.get('passed') is True or data.get('status') == 'passed', label + ' did not pass'
    assert data.get('exit_code', 0) == 0, label + ' returned failure'


def check_log(proof, path):
    log = path.with_suffix('.log')
    assert proof['log_sha256'] == sha(log), 'Log changed: ' + str(log)
    return log


def checked_native(proof, path, pin):
    passed(proof, str(path))
    assert proof['revision'] == pin and proof['revision_after'] == pin
    assert proof['clean_before'] is True and proof['clean_after'] is True
    check_log(proof, path)


class Selection:
    def __init__(self):
        self.items, self.by_source, self.names = [], {}, set()

    def add(self, path, name):
        supplied = pathlib.Path(path)
        assert supplied.is_file() and not supplied.is_symlink(), 'Missing or linked evidence: ' + str(path)
        path = supplied.resolve()
        key = str(path).casefold()
        if key in self.by_source:
            return self.by_source[key]
        assert name.casefold() not in self.names, 'Repeated logical path: ' + name
        self.names.add(name.casefold())
        self.by_source[key] = name
        self.items.append({'source': str(path), 'name': name})
        return name

    def tree(self, root, label, include_png=True):
        root = pathlib.Path(root).resolve()
        assert root.is_dir()
        for path in sorted(root.rglob('*')):
            assert not path.is_symlink(), 'Linked evidence: ' + str(path)
            if path.is_file() and (include_png or path.suffix.lower() != '.png'):
                assert path.suffix.lower() not in {'.exe', '.dll', '.pdb'}, 'Unexpected compiled binary in evidence'
                self.add(path, label + '/' + path.relative_to(root).as_posix())


def audit_chain(case):
    records = sorted((case / 'archive-audit').glob('*.json'))
    assert records, 'Missing native archive audit: ' + str(case)
    for path in records:
        record = load(path)
        assert record['canonical']['ignored_paths'] == [], 'Ignored native world fields'
        for key in ['input', 'canonical']:
            part = record[key]
            source = pathlib.Path(part['path']).resolve()
            assert source.is_relative_to(case.resolve()), 'Audit input outside disposable case'
            assert source.is_file() and sha(source) == part['sha256']
            assert source.stat().st_size == part['bytes']
    progress = case / 'progress.jsonl'
    comparisons = [json.loads(line) for line in progress.read_text(encoding='utf-8').splitlines()
                   if line.strip() and json.loads(line).get('event') == 'exact-native-comparison']
    assert comparisons, 'Missing exact native comparisons'
    for record in comparisons:
        assert record['equal'] is True
        assert record['left']['ignored_paths'] == record['right']['ignored_paths'] == []
        assert record['left']['sha256'] == record['right']['sha256']
    return {'inspections': len(records), 'exact_comparisons': len(comparisons), 'ignored_paths': []}


def validate_browser(data, path, pin, binary_hash, wrapper, head):
    passed(data, str(path))
    assert data['build']['revision'] == pin and data['build']['binary_sha256'] == binary_hash
    assert data['build']['build']['revision'] == pin[:12]
    driver = data['test_source']['revision']
    assert driver == wrapper.get('driver_revision', wrapper.get('revision'))
    assert data['test_source']['runtime_source_equal'] is True
    source_equal(pin, driver)
    source_equal(driver, head)
    assert not data.get('errors'), 'Browser page errors'
    assert data['saved_slot'] and (path.parent / 'server/saves' / (data['saved_slot'] + '.json')).is_file()
    return audit_chain(path.parent)


def validate_wrapper(data, path, pin, binary_hash, head):
    passed(data, str(path)); check_log(data, path)
    driver = data.get('driver_revision', data.get('revision'))
    assert data.get('runtime_revision', pin) == pin
    source_equal(pin, driver); source_equal(driver, head)
    assert data['revision_after'] == driver and data['clean_after'] is True
    assert data['binary_sha256_before'] == binary_hash
    assert data.get('binary_sha256_after', data.get('binary', {}).get('sha256')) == binary_hash


def replace_once(text, before, after):
    assert text.count(before) == 1, 'Publication template changed: ' + before[:90]
    return text.replace(before, after, 1)


def arguments():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('--runtime-pin', required=True)
    defaults = {'web-proof': 'evidence/S10-c-web-1.json', 'node-proof': 'evidence/S10-c-node-1.json',
                'binary-proof': 'evidence/S10-c-binary-1.json',
                'linux-proof': 'evidence/S10c-linux-final-1/runner-result.json',
                'content-proof': 'evidence/S10c-content-final/result.json',
                'visual-proof': 'evidence/S10c-visual-review.json',
                'preservation-proof': 'evidence/S10c-preservation-final.json',
                'launch-proof': 'S10c-review-launch.json',
                'matrix-wrapper': 'evidence/S10c-matrix-driver-final-1.json',
                'browser-wrapper': 'evidence/S10-c-browser-1.json'}
    for name, default in defaults.items():
        parser.add_argument('--' + name, type=path_arg, default=path_arg(default))
    parser.add_argument('--matrix-result', type=path_arg, required=True)
    parser.add_argument('--browser-result', type=path_arg, required=True)
    parser.add_argument('--failed-attempt', action='append', default=[], metavar='LABEL=PATH',
                        help='Explicit failed wrapper/result JSON, or failed run directory; recursively preserved with its log')
    parser.add_argument('--additional-evidence', action='append', default=[], metavar='LABEL=PATH',
                        help='Explicit supporting record/directory; never interpreted as acceptance')
    return parser.parse_args()


def main(args):
    pin = args.runtime_pin
    assert re.fullmatch('[0-9a-f]{40}', pin)
    assert not DEST.exists(), 'S10.c already published; refuse overwrite'
    assert not git('status', '--porcelain'), 'Require clean committed source before publication'
    head = git('rev-parse', 'HEAD')
    source_equal(pin, head)
    source_equal(SIM_BASELINE, pin, ('spheres-sim', 'spheres-cli', 'Cargo.toml', 'Cargo.lock'))
    inputs = {key: value for key, value in vars(args).items() if key.endswith('_proof') or key.endswith('_wrapper') or key.endswith('_result')}
    data = {key: load(path) for key, path in inputs.items()}
    web, node, binary = (data[key] for key in ['web_proof', 'node_proof', 'binary_proof'])
    for key in ['web_proof', 'binary_proof']:
        checked_native(data[key], inputs[key], pin)
    passed(node, 'Windows Node'); check_log(node, args.node_proof)
    source_equal(pin, node['revision']); source_equal(node['revision'], head)
    assert node['clean_before'] is True and node['clean_after'] is True and node['revision_after'] == node['revision']
    assert node['totals']['pass'] > 0 and node['totals']['fail'] == 0
    assert web['totals']['passed'] > 0 and web['totals']['failed'] == 0 and web['totals']['completed_targets'] > 0
    binary_hash = binary['binary']['sha256']
    assert sha(binary['binary']['path']) == binary_hash, 'Qualified executable was replaced'
    linux, content = data['linux_proof'], data['content_proof']
    passed(linux, 'Linux'); passed(content, 'Content')
    source_equal(pin, linux['candidate']); source_equal(linux['candidate'], head)
    source_equal(content['revision'], head, CONTENT_PATHS)
    assert content['clean_after'] is True and content['revision_after'] == content['revision']
    expected_linux = {'native', 'node', 'c01-tests', 'c01-campaign_census', 'c01-import_cnccfp_census',
                      'c01-campaign_research', 'master-archive', 'active-fixtures', 'party-versions'}
    assert len(linux['checks']) == 9 and {row['name'] for row in linux['checks']} == expected_linux
    assert {row['name'] for row in content['checks']} == {'census-tests', 'france-tests', 'portrait-tests',
            'production', 'build_person_avatar_assets', 'campaign_census', 'import_cnccfp_census', 'campaign_research'}
    for proof, path in [(linux, args.linux_proof), (content, args.content_proof)]:
        for row in proof['checks']:
            assert row['exit_code'] == 0 and row.get('exact_one_test_passed', True) is True
            assert sha(path.parent / row['log']) == row['log_sha256']
    linux_native = next(row for row in linux['checks'] if row['name'] == 'native')['totals']
    assert linux_native['failed'] == 0 and linux_native['passed'] > 0
    for snapshot in ['source_before', 'source_after']:
        assert linux[snapshot]['head'] == linux['candidate'] and not linux[snapshot]['status_porcelain']
    assert linux['fixtures_before']['items'] == linux['fixtures_after']['items']
    for key in ['matrix_wrapper', 'browser_wrapper']:
        validate_wrapper(data[key], inputs[key], pin, binary_hash, head)
    matrix = data['matrix_result']
    passed(matrix, 'Country matrix')
    assert matrix['runtime_revision'] == pin and matrix['binary_sha256'] == binary_hash
    assert matrix['full_eight_country_matrix'] is True and len(matrix['cases']) == 8
    assert {row['nation'] for row in matrix['cases']} == COUNTRIES
    assert matrix['test_source']['revision'] == data['matrix_wrapper']['driver_revision']
    assert data['visual_proof'].get('reviewed') is True, 'Visual review not complete'
    assert data['visual_proof'].get('runtime_revision') == pin, 'Visual review is for another runtime'
    preservation = data['preservation_proof']
    passed(preservation, 'Preservation')
    assert len(preservation['files']) == 8 and len(preservation['worktrees']) == 2
    assert all(row['unchanged'] is True for row in preservation['files'] + preservation['worktrees'])
    launch = data['launch_proof']
    assert launch['runtime_revision'] == pin and launch['executable_sha256'] == binary_hash
    assert launch['build']['revision'] == pin[:12]
    assert launch['native_verification']['exact_bytes_match_final_browser'] is True
    assert launch['native_verification']['ignored_paths'] == []
    assert re.fullmatch(r'http://(?:127\.0\.0\.1|localhost):[0-9]+/?', launch['url'])
    assert launch['save_copy']['sha256'] == sha(launch['save_copy']['source']) == sha(launch['save_copy']['copy'])
    research_path = REPO / 'docs/campaign-certification/C01/research-index.json'
    research = load(research_path)
    assert all(research['counts'][key] == value for key, value in EXPECTED_COUNTS.items())
    production_path = REPO / 'spheres-web/data/leadership_production_2035.json'
    production = load(production_path)['counts']
    assert (production['validated_cartoon_assets'], production['validated_fictional_cartoon_assets'],
            production['pending_historical_art_jobs']) == (54, 4, 704)

    selection, proofs, screenshots, audits = Selection(), {}, [], {}
    for key, path in inputs.items():
        name = selection.add(path, 'proofs/' + key + '.json')
        proofs[key] = {'source': str(path), 'logical_path': name, 'sha256': sha(path), 'bytes': path.stat().st_size}
    for key in ['web_proof', 'node_proof', 'binary_proof', 'matrix_wrapper', 'browser_wrapper']:
        selection.add(inputs[key].with_suffix('.log'), 'checks/' + key + '.log')
    selection.tree(args.linux_proof.parent, 'linux')
    selection.tree(args.content_proof.parent, 'content')

    def include_case(result_path, label, keep_png, wrapper):
        detail = load(result_path)
        audits[label] = validate_browser(detail, result_path, pin, binary_hash, wrapper, head)
        for shot in detail.get('screenshots', []):
            record = {'file': shot} if isinstance(shot, str) else shot
            image = (result_path.parent / record['file']).resolve()
            assert image.is_relative_to(result_path.parent.resolve()) and image.is_file()
            actual = sha(image)
            assert record.get('sha256', actual) == actual, 'Screenshot changed: ' + str(image)
            screenshots.append({'case': label, 'file': record['file'], 'bytes': image.stat().st_size,
                                'sha256': actual, 'included': keep_png,
                                'reason': 'Selected France/Tonga/policy visual coverage' if keep_png else
                                          'Repetitive other-country screenshots omitted; original result and this hash retained'})
        selection.tree(result_path.parent, label, include_png=keep_png)
        return detail

    matrix_details = []
    for row in matrix['cases']:
        result_path = pathlib.Path(row['result']).resolve()
        assert result_path.is_relative_to(args.matrix_result.parent.resolve())
        assert row['passed'] is True and sha(result_path) == row['result_sha256']
        matrix_details.append(include_case(result_path, 'browser/matrix/' + row['nation'],
                                          row['nation'] in {'France', 'Tonga'}, data['matrix_wrapper']))
    # Preserve matrix-level files and logs as well as all case-level records.
    for path in sorted(args.matrix_result.parent.iterdir()):
        if path.is_file(): selection.add(path, 'browser/matrix/' + path.name)
    journey = include_case(args.browser_result, 'browser/france-policy', True, data['browser_wrapper'])
    assert pathlib.Path(launch['save_copy']['source']).resolve() == (args.browser_result.parent / 'server/saves' / (journey['saved_slot'] + '.json')).resolve()
    assert len(journey['commands']) == 3 and journey['stale_refusal']['requires_review'] is True
    assert len(journey['mobile_policy_review']) >= 2 and len(journey['mobile_government_review']) >= 2
    assert len(journey['government_notice_visibility']) == 3
    final_audits = list((args.browser_result.parent / 'archive-audit').glob('*-s10-after-continue.json'))
    assert len(final_audits) == 1, 'Missing unique final Continue audit'
    final_canonical = load(final_audits[0])['canonical']
    assert launch['native_verification']['canonical_sha256'] == final_canonical['sha256']
    assert launch['native_verification']['bytes'] == final_canonical['bytes']
    failures = []
    for argument, failed_only in [(args.failed_attempt, True), (args.additional_evidence, False)]:
        for value in argument:
            label, sep, raw_path = value.partition('=')
            assert sep and re.fullmatch('[A-Za-z0-9_-]+', label)
            path = path_arg(raw_path)
            prefix = ('failed-attempts/' if failed_only else 'supporting/') + label
            if path.is_dir():
                results = list(path.rglob('result.json')) + list(path.rglob('runner-result.json'))
                assert not failed_only or any(load(p).get('passed') is False or load(p).get('status') in {'failed', 'runner_error'} for p in results), 'Failed directory lacks a failure record'
                selection.tree(path, prefix)
            else:
                if failed_only:
                    failed = load(path)
                    assert failed.get('passed') is False or failed.get('status') in {'failed', 'runner_error'} or failed.get('exit_code', 0) != 0
                selection.add(path, prefix + '/' + path.name)
                if path.with_suffix('.log').is_file(): selection.add(path.with_suffix('.log'), prefix + '/' + path.with_suffix('.log').name)
            if failed_only: failures.append({'label': label, 'source': str(path), 'classification': 'Failed attempt retained; not passing qualification'})
    runner_names = ['run-s10-check.py', 'run-s10-browser.py', 'run-s10c-linux.py', 'run-s10c-content.py',
                    'run-s10c-matrix.py', 'launch-s10c-review.py', 'run-publish-s10c.py', 'publish-s10c.py', 'verify-s10c-publication.py', 'collect-s09-evidence.py', 'verify-s08-preservation.py']
    runner_hashes = {sha(BASE / name) for name in runner_names}
    for key in ['web_proof', 'node_proof', 'binary_proof', 'linux_proof', 'content_proof', 'matrix_wrapper', 'browser_wrapper']:
        assert data[key]['runner_sha256'] in runner_hashes, 'Exact executed runner source is missing: ' + key
    for name in runner_names:
        selection.add(BASE / name, 'runners/' + name)

    # Prepare all text and marker changes before any repository write.
    pathway_path = REPO / 'docs/planning/campaign-pathway.json'
    pathway = load(pathway_path)
    session = next(row for row in pathway['sessions'] if row['id'] == 'S10')
    assert session['status'] == 'in_progress'
    assert next(row for row in pathway['sessions'] if row['id'] == 'C01')['status'] == 'in_progress'
    assert pathway['last_completed_session'] == 'S09' and pathway['next_session'] == 'S10'
    assert pathway['gate_decisions'].get('G2', {}).get('status') != 'earned'
    assert next(row for row in pathway['sessions'] if row['id'] == 'S11')['status'] == 'planned'
    increment = {'id': 'S10.c', 'status': 'complete', 'title': 'Readable political reviews, sourced Tonga cartoon and France reporting universe',
                 'evidence': 'docs/campaign-certification/S10/c/manifest.json'}
    prior = [row for row in session['increments'] if row['id'] == 'S10.c']
    assert len(prior) <= 1 and (not prior or prior[0]['status'] != 'complete')
    if prior: prior[0].update(increment)
    else: session['increments'].append(increment)
    pathway['execution'].update(status='stopped', stop_boundary='S10.c', previous_stop='S10.b',
                                next_session_requires_instruction=True)
    summary_path = REPO / 'docs/campaign-certification/S10/README.md'
    summary = summary_path.read_text(encoding='utf-8')
    summary = replace_once(summary, 'Status: **S10.a and S10.b complete; S10 and C01 remain in progress**.',
                           'Status: **S10.a, S10.b and S10.c complete; S10 and C01 remain in progress**.')
    block = ('## S10.c — Readable reviews and bounded content\n\n'
             'The [S10.c report](c/README.md) records desktop and 390/320px decision cards,\n'
             'visible completion notices, the source-attributed Tupou IV cartoon for 1990,\n'
             'and all 635 identities in the pinned French 2024 filing-obligation universe.\n'
             'The complete eight-country startup matrix and two-tab France policy/save\n'
             'journey were requalified. These checks advance zero campaign days; C01,\n'
             'S10 and G2 remain open. Earlier S10.a/b evidence below remains historical.\n\n')
    summary = replace_once(summary, '## S10.a — Reviewed political decisions\n', block + '## S10.a — Reviewed political decisions\n')
    roadmap_path = REPO / 'docs/CERTIFIED_CAMPAIGN_PATHWAY.md'
    roadmap = roadmap_path.read_text(encoding='utf-8')
    anchor = 'evidence are complete. See [S10 progress and limits](campaign-certification/S10/README.md).'
    roadmap = replace_once(roadmap, anchor, anchor + '\n\n**S10.c complete:** responsive before/after decision cards and completion notices,\na sourced 1990 Tonga cartoon, and the 635-identity France reporting-obligation\nuniverse. The [S10.c proof](campaign-certification/S10/c/README.md) retains the\neight-country and two-tab save/load journeys. S10, C01 and G2 remain open;\nexecution stopped after this increment, before S11.')
    roadmap = replace_once(roadmap,
        'S10.a is in progress and C01 remains incomplete; S11 and later sessions require a new instruction.',
        'S10.a, S10.b and S10.c are complete as bounded increments; S10 and C01 remain incomplete. Execution stopped after S10.c; S11 and later sessions require a new instruction.')

    spec = importlib.util.spec_from_file_location('s10c_collector', BASE / 'collect-s09-evidence.py')
    collector = importlib.util.module_from_spec(spec); spec.loader.exec_module(collector)
    collector.preflight(selection.items, DEST / 'evidence')
    assert not git('status', '--porcelain') and git('rev-parse', 'HEAD') == head
    selection_path = BASE / 'evidence/S10c-publication-selection-final.json'
    assert not selection_path.exists()
    selection_path.write_text(json.dumps(selection.items, indent=2) + '\n', encoding='utf-8')
    DEST.mkdir(parents=True, exist_ok=False)
    collector.collect(selection_path, DEST / 'evidence')
    inventory_path = DEST / 'evidence/inventory.json'
    inventory = load(inventory_path)
    inventory['scope'] = ('S10.c exact-runtime Windows/Linux web, Node and content checks; all eight-country and France-policy '
                          'browser raw/canonical native proof chains; selected France/Tonga/policy screenshots. Other six country '
                          'screenshots are omitted with original result hashes and explicit selection ledger retained. No elapsed-day, '
                          'performance, historical-census, C01, S10 or G2 completion is claimed.')
    inventory_path.write_text(json.dumps(inventory, indent=2) + '\n', encoding='utf-8')
    inventory_rows = {row['logical_path']: row for row in inventory['files']}
    for proof in proofs.values():
        assert inventory_rows[proof['logical_path']]['sha256'] == proof['sha256']
    manifest = {'format': 'spheres-s10c-increment/v1', 'status': 'increment_complete_parent_open',
                'completed_utc': datetime.datetime.now(datetime.timezone.utc).isoformat(),
                'runtime_revision': pin, 'publication_parent_revision': head,
                'publication_commit': {'resolution': 'git log -1 --format=%H -- docs/campaign-certification/S10/c/manifest.json',
                                       'note': 'Containing test/documentation commit resolved after commit; Git transport recorded separately.'},
                'complete_runtime_source_equal': True, 'unchanged_simulation_baseline': SIM_BASELINE,
                'simulation_source_unchanged': True, 'saved_role_or_gameplay_authorization_changed': False,
                'c01_complete': False, 's10_complete': False, 'g2_earned': False, 's11_started': False,
                'research_counts': research['counts'], 'research_index_sha256': sha(research_path),
                'art_counts': {key: production[key] for key in ['validated_cartoon_assets', 'validated_fictional_cartoon_assets', 'pending_historical_art_jobs']},
                'production_index_sha256': sha(production_path),
                'france_scope': {'filing_exercise': 2024, 'obligation_identities': 635, 'submitted_accounts_identities': 575,
                                 'additional_nonfiling_identities': 60, 'all_political_organizations_or_history_complete': False},
                'portrait_scope': {'person_id': 'taufaahau_tupou_iv', 'from': '1990-01-01', 'until_exclusive': '1991-01-01',
                                   'reference_year': 1985, 'appearance': 'Explicit artistic interpretation near 1990',
                                   'derivative_license': 'CC BY-SA 4.0'},
                'qualification': {'windows_web': web['totals'], 'windows_node': node['totals'],
                                  'linux_web': linux_native, 'linux_checks': len(linux['checks']),
                                  'content_checks': len(content['checks']), 'browser_countries': 8,
                                  'browser_elapsed_days': 0, 'binary_sha256': binary_hash,
                                  'new_performance_claim': False, 'full_sim_suite_rerun': False},
                'proofs': proofs, 'native_audit_chains': audits, 'screenshot_selection': screenshots,
                'failed_attempts': failures, 'review_url': launch['url'],
                'evidence': {'inventory': 'evidence/inventory.json', 'inventory_sha256': sha(inventory_path),
                             'logical_files': len(inventory['files']), 'stored_bytes': inventory['stored_bytes']},
                'remaining': ['Complete C01 organization censuses, role histories and jurisdiction reconciliation before S10/G2 closure.',
                              'Reconcile sourced historical and fictional eligibility with actual game roles; art counts are not historical completeness.',
                              'Expose the portrait derivative_license separately through public API/UI metadata; full existing attribution already states CC BY-SA 4.0.']}
    write_json = lambda path, value: path.write_text(json.dumps(value, indent=2, ensure_ascii=False) + '\n', encoding='utf-8', newline='\n')
    write_json(DEST / 'manifest.json', manifest)
    (DEST / '.gitattributes').write_text('# Preserve exact evidence bytes and original whitespace.\nevidence/** -text -whitespace\n', encoding='utf-8')
    failures_text = ('Explicit failed attempts are retained under their labels in the manifest; none is counted as passing qualification.'
                     if failures else 'No failed attempt was supplied for this final evidence set.')
    readme = f'''# S10.c — Readable decisions and bounded content

**S10.c complete; S10, C01 and G2 remain open.** Execution stopped after this
increment. S11 has not started.

Government and diplomatic reviews now show each native before/after value in
readable cards on desktop and at 390px/320px. Government completion notices stay
visible and receive keyboard focus. Confirmation, campaign preconditions and
gameplay authorizations are unchanged.

Tonga has a source-attributed cartoon of Tupou IV for **1990 only**, with an
exclusive end of 1 January 1991. It openly interprets a 1985 Comet Photo/ETH
reference near the 1990 era. The reference and adaptation carry CC BY-SA 4.0;
this is not an exact 1990 photograph or a new historical officeholder claim.
The production ledger now has 54 historical and four fictional accepted cartoon
assets; 704 historical art jobs remain. Art acceptance is separate from census
and complete leadership-history acceptance.

The France importer enumerates **635 official CNCCFP identities subject to 2024
filing obligations**: the previous 575 submitted-account identities plus 60
nonfiling identities. Codes come from the official publication, with exact
source/row provenance and independent annex corroboration. Nonfiling does not
mean dissolution. This one-year financial universe is not every French political
organization or a 1990–present leadership census. The wider discovery intake has
644 organization and 11 institution observations, 31 sources, 1,315 claims and
67 work batches; zero exhaustive country censuses are closed.

## Verification

- Windows web: {web['totals']['passed']} passed, zero failed, {web['totals']['ignored']} ignored.
  Full Windows Node: {node['totals']['pass']} passed, zero failed, {node['totals'].get('skipped', 0)} skipped.
- Linux web: {linux_native['passed']} passed, zero failed, {linux_native['ignored']} ignored.
  The full Linux Node suite, content reproducibility and three external archive
  checks passed in nine declared Linux checks. Eight focused content checks passed.
- All eight independent ordinary startup government journeys passed, including
  review/cancel/confirm, foreign inspection and named Save/Load/Continue.
- The France policy journey passed real second-tab staleness, refusal without
  effects, fresh confirmation, readable narrow reviews and save/load equality.
  Every native raw save, canonical audit and exact comparison is preserved.
- Protected original files and source worktrees passed preservation. The
  [review build]({launch['url']}) uses an independently verified save copy;
  this link records the local launch, not a promise that its process stays live.

These journeys advanced **zero campaign days** and granted no resources. This
is not a new performance benchmark, a full simulation rerun, a Russia activation
test, or a certified campaign through 2035. Simulation/CLI/workspace sources are
unchanged from `{SIM_BASELINE}`; this increment requalified the web/content layer.

## Provenance and evidence selection

Runtime: `{pin}`. Executable SHA-256: `{binary_hash}`.
Later test-only driver fixes, if any, must retain exact runtime source equality.
The [manifest](manifest.json) binds every final proof and the
[evidence inventory](evidence/inventory.json) reconstructs original bytes,
including compressed/chunked native archives. The publication commit is the
Git commit containing this manifest; transport is recorded separately afterward.

All result files, logs and native proof chains are included. France, Tonga and
policy screenshots are included; repetitive screenshots for the other six
countries are omitted, with their original result hashes and explicit selection
ledger retained. {failures_text}

## Remaining work

C01 still needs complete organization discovery, jurisdiction reconciliation and
separate sourced party, parliamentary and executive histories. The new cartoon
does not close Tonga's cast or later eras. No S10/G2 or worldwide-content gate is
earned here. A nonblocking metadata follow-up remains: expose `derivative_license`
through the normal portrait API/UI as its own artwork-license link. The current
credit already gives the adaptation's CC BY-SA 4.0 attribution and license URL.
At 320px, long prose in the Decisions room's two-column cards still wraps into
short lines; all quoted text remains available without horizontal scrolling.
The government cards use stacked values at this width. Further Decisions-room
typography refinement remains a small visual follow-up.
'''
    (DEST / 'README.md').write_text(readme, encoding='utf-8', newline='\n')
    write_json(pathway_path, pathway)
    summary_path.write_text(summary, encoding='utf-8', newline='\n')
    roadmap_path.write_text(roadmap, encoding='utf-8', newline='\n')
    allowed = {'docs/planning/campaign-pathway.json', 'docs/CERTIFIED_CAMPAIGN_PATHWAY.md', 'docs/campaign-certification/S10/README.md'}
    changed = git('diff', '--name-only', 'HEAD').splitlines()
    assert all(path in allowed or path.startswith('docs/campaign-certification/S10/c/') for path in changed)
    assert git('rev-parse', 'HEAD') == head
    source_equal(pin, 'HEAD')
    print(json.dumps({'published': str(DEST), 'runtime_revision': pin, 'inventory_sha256': sha(inventory_path),
                      'logical_files': len(inventory['files']), 'stored_bytes': inventory['stored_bytes'],
                      'c01_complete': False, 's10_complete': False, 'g2_earned': False}, indent=2))


if __name__ == '__main__':
    main(arguments())
