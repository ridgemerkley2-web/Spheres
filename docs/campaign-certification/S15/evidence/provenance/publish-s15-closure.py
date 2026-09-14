"""Validate retained S12-S15 qualification, then optionally publish closure.

Usage: python publish-s15-closure.py CONFIG.json [--apply]
CONFIG needs visual_review: {passed: true, widths: [1440, 390],
screenshots: [browser-relative PNG names], notes: "Actual inspection findings"}.
Evidence paths have defaults below and may be overridden in CONFIG.
No Cargo, Git, server, collection or campaign command is executed. Validation
is read-only. --apply writes only the seven enumerated completion documents.
"""
import argparse
import copy
import datetime as dt
import hashlib
import json
import math
import os
import pathlib
import re
import zlib

BASE = pathlib.Path(__file__).resolve().parent
REPO = BASE / 'integration'
DEST = REPO / 'docs/campaign-certification/S15'
RUNTIME = '4c4129abeeec4e9e2987f121e875f5efc54b4c7f'
DRIVER = 'tools/ui/ci-flight-operations.cjs'
RUNTIME_PATHS = ['spheres-sim', 'spheres-web', 'Cargo.toml', 'Cargo.lock']
LANES = ('web', 'sim', 'integration', 'node', 'binary', 'fixture')
DEFAULTS = {
    'windows': {k: {'proof': f'evidence/S15-final1-{k}.json',
                    'log': f'evidence/S15-final1-{k}.log'} for k in LANES},
    'browser_proof': 'evidence/S15-final2-browser.json',
    'browser_log': 'evidence/S15-final2-browser.log',
    'linux_result': 'evidence/S15-final1-linux/result.json',
    'fixture_manifest': 'evidence/S15-fixture-final1/manifest.json',
    'review_launch': 'S15-review-launch.json',
    'preservation': 'evidence/S15-preservation-final.json',
    'inventory': 'integration/docs/campaign-certification/S15/evidence/inventory.json',
    'selection': 'S15-evidence-selection.json',
}


def need(condition, message):
    if not condition:
        raise ValueError(message)


def read(p):
    return json.loads(pathlib.Path(p).read_text(encoding='utf-8-sig'),
                      parse_constant=lambda value: (_ for _ in ()).throw(ValueError(value)))


def text_json(value):
    return json.dumps(value, ensure_ascii=False, indent=2, allow_nan=False) + '\n'


def path(value, parent=BASE):
    value = str(value)
    if os.name == 'nt' and re.match(r'^/mnt/[a-z]/', value):
        value = value[5].upper() + ':/' + value[7:]
    p = pathlib.Path(value)
    return (p if p.is_absolute() else parent / p).resolve()


def sha(p):
    with pathlib.Path(p).open('rb') as stream:
        return hashlib.file_digest(stream, 'sha256').hexdigest()


def digest(value):
    return isinstance(value, str) and re.fullmatch('[a-f0-9]{64}', value) is not None


def integer(value, minimum=0):
    return type(value) is int and value >= minimum


def finite(value, minimum=0):
    return type(value) in (int, float) and math.isfinite(value) and value >= minimum


def record(p):
    p = path(p)
    need(p.is_file() and not p.is_symlink(), f'Missing or linked evidence: {p}')
    return {'path': str(p), 'bytes': p.stat().st_size, 'sha256': sha(p)}


def exact(row, parent=BASE):
    p = path(row.get('path', row.get('file')), parent)
    found = record(p)
    need(found['sha256'] == row['sha256'], f'Changed evidence: {p}')
    need('bytes' not in row or row['bytes'] == found['bytes'], f'Changed evidence size: {p}')
    return p


def inventory_check(file, selection):
    inv = read(file)
    need(inv['selection_sha256'] == sha(selection) and inv['selection'] == read(selection),
         'Retained selection does not match its exact original')
    index, logical, names, stored = {}, set(), set(), 0
    for row in inv['files']:
        need(row['reconstruction_verified'] is True and row['encoding'] in ('raw', 'gzip')
             and row['storage'], 'Unverified retention row')
        need(row['logical_path'].casefold() not in logical, 'Duplicate logical evidence path')
        logical.add(row['logical_path'].casefold())
        size, result = 0, hashlib.sha256()
        decoder = zlib.decompressobj(31) if row['encoding'] == 'gzip' else None
        for part in row['storage']:
            rel = pathlib.PurePosixPath(part['file'])
            need(not rel.is_absolute() and '..' not in rel.parts and '\\' not in str(rel)
                 and ':' not in str(rel), 'Unsafe retained path')
            need(str(rel).casefold() not in names, 'Duplicate retained storage path')
            names.add(str(rel).casefold())
            p = exact(part, file.parent)
            need(p.is_relative_to(file.parent), 'Retained evidence escaped its directory')
            stored += part['bytes']
            with p.open('rb') as stream:
                while block := stream.read(65536):
                    raw = decoder.decompress(block) if decoder else block
                    result.update(raw)
                    size += len(raw)
        if decoder:
            raw = decoder.flush()
            result.update(raw)
            size += len(raw)
            need(decoder.eof and not decoder.unused_data and not decoder.unconsumed_tail,
                 'Incomplete or concatenated retained gzip stream')
        need(size == row['bytes'] and result.hexdigest() == row['sha256'],
             'Retained bytes cannot reconstruct ' + row['logical_path'])
        index.setdefault(str(path(row['source'])).casefold(), []).append(row)
    need(inv['files'] and stored == inv['stored_bytes'], 'Incorrect retention totals')
    actual = {p.relative_to(file.parent).as_posix().casefold()
              for p in file.parent.rglob('*') if p.is_file()}
    need(actual == names | {'inventory.json'}, 'Unlisted or missing retained storage files')
    return inv, index


def totals(value, log, node=False):
    if node:
        observed = {k: int(re.findall(r'(?:ℹ|#)\s+' + k + r'\s+(\d+)', log)[-1])
                    for k in ('tests', 'pass', 'fail', 'skipped')}
        normalized = {'tests': value['tests'], 'pass': value.get('pass', value.get('passed')),
                      'fail': value.get('fail', value.get('failed')), 'skipped': value['skipped']}
        need(observed == normalized and observed['pass'] > 0 and observed['fail'] == 0
             and observed['tests'] == observed['pass'] + observed['skipped'], 'Node totals differ from log')
    else:
        rows = [tuple(map(int, row)) for row in re.findall(
            r'test result: (?:ok|FAILED)\. (\d+) passed; (\d+) failed; (\d+) ignored;', log)]
        observed = {'passed': sum(r[0] for r in rows), 'failed': sum(r[1] for r in rows),
                    'ignored': sum(r[2] for r in rows), 'completed_targets': len(rows)}
        need(observed == value and observed['passed'] > 0 and observed['failed'] == 0,
             'Native totals differ from log')
    return value


def qualify(config):
    cfg = copy.deepcopy(DEFAULTS)
    cfg.update(config)
    need(cfg.get('candidate_revision', RUNTIME) == RUNTIME, 'Unexpected S15 runtime candidate')
    invfile, selection = path(cfg['inventory']), path(cfg['selection'])
    need(invfile == DEST / 'evidence/inventory.json', 'Only the new S15 evidence tree is eligible')
    inv, retained = inventory_check(invfile, selection)

    def kept(p):
        p = path(p)
        r = record(p)
        matches = [x for x in retained.get(str(p).casefold(), [])
                   if x['sha256'] == r['sha256'] and x['bytes'] == r['bytes']]
        need(matches, f'Required original evidence was not retained: {p}')
        r.update(logical_path=matches[0]['logical_path'], storage=matches[0]['storage'])
        return r

    refs, windows, logs, ignored = {}, {}, {}, {}
    need(set(cfg['windows']) == set(LANES), 'All six final Windows native/Node lanes are required')
    runner = kept(BASE / 'run-s15-check.py')
    for lane in LANES:
        proofpath, logpath = path(cfg['windows'][lane]['proof']), path(cfg['windows'][lane]['log'])
        proof, log = read(proofpath), logpath.read_text(encoding='utf-8-sig')
        need(proof['lane'] == lane and proof['passed'] is True and proof['exit_code'] == 0,
             f'Windows {lane} did not pass')
        need(proof['revision'] == proof['revision_after'] == RUNTIME
             and proof['clean_before'] is True and proof['clean_after'] is True,
             f'Windows {lane} is not the clean runtime candidate')
        need(proof['runner_sha256'] == runner['sha256'] and proof['log_sha256'] == sha(logpath),
             f'Windows {lane} provenance changed')
        if lane != 'binary':
            totals(proof['totals'], log, lane == 'node')
        refs['windows_' + lane] = {'proof': kept(proofpath), 'log': kept(logpath)}
        if lane in ('web', 'sim', 'integration'):
            ignored[lane] = re.findall(r'^test (.+?) \.\.\. ignored([^\n]*)$', log, re.M)
        windows[lane], logs[lane] = proof, log
    binary = windows['binary']['binary']
    exact(binary)
    refs['windows_runner'] = runner

    linuxfile = path(cfg['linux_result'])
    linux = read(linuxfile)
    need(linux['format'] == 'spheres-s15-linux/v1' and linux['revision'] == RUNTIME
         and linux.get('passed') is True and linux['status'] == 'passed', 'Linux qualification is incomplete')
    expected_source = {'head': RUNTIME, 'status': ''}
    need(linux['source_before'] == linux['source_after'] == expected_source, 'Linux source changed')
    linuxrunner = kept(BASE / 'run-s15-linux.py')
    need(linux['runner_sha256'] == linux['runner_sha256_after'] == linuxrunner['sha256'],
         'Linux runner changed')
    need(len(linux['checks']) == 4 and {x['name'] for x in linux['checks']} == {'web', 'sim', 'integration', 'node'},
         'Missing or duplicate Linux lane')
    refs['linux_result'], refs['linux_runner'] = kept(linuxfile), linuxrunner
    for check in linux['checks']:
        need(check['passed'] is True and check['exit_code'] == 0
             and check['source_before'] == check['source_after'] == expected_source,
             'Linux lane is not a clean successful runtime check')
        p = path(check['log'], linuxfile.parent)
        need(check['log_sha256'] == sha(p), 'Linux log changed')
        totals(check['totals'], p.read_text(encoding='utf-8-sig'), check['name'] == 'node')
        refs['linux_' + check['name']] = kept(p)
    toolchain = linuxfile.parent / 'toolchain.json'
    need(sha(toolchain) == linux['toolchain_sha256'], 'Linux toolchain record changed')
    refs['linux_toolchain'] = kept(toolchain)

    browserproof, browserlog = path(cfg['browser_proof']), path(cfg['browser_log'])
    bridge = read(browserproof)
    need(bridge['passed'] is True and bridge['exit_code'] == 0 and bridge['runtime_revision'] == RUNTIME,
         'Final browser qualification did not pass')
    driver_revision = bridge['driver_revision']
    need(re.fullmatch('[a-f0-9]{40}', driver_revision) and driver_revision != RUNTIME
         and bridge['head_after'] == driver_revision and bridge['clean_before'] is True
         and bridge['clean_after'] is True, 'Browser driver source changed')
    need(bridge['runtime_paths'] == RUNTIME_PATHS and bridge['runtime_diff'] == []
         and bridge['changed_paths'] == [DRIVER], 'Browser driver revision changes runtime or other files')
    driver, browserrunner = kept(REPO / DRIVER), kept(BASE / 'run-s15-browser-final.py')
    need(bridge['driver_sha256'] == driver['sha256'] and bridge['runner_sha256'] == browserrunner['sha256']
         and bridge['binary_sha256'] == binary['sha256'] and bridge['log_sha256'] == sha(browserlog),
         'Browser driver, runner, runtime binary or log changed')
    browserfile = exact(bridge['result'])
    emitted = []
    for line in browserlog.read_text(encoding='utf-8-sig').splitlines():
        try:
            row = json.loads(line)
        except ValueError:
            continue
        if isinstance(row, dict) and row.get('passed') is True and isinstance(row.get('result'), str):
            emitted.append(path(row['result'], REPO))
    need(emitted == [browserfile], 'Selected browser result is not the qualified runner output')
    browser = read(browserfile)
    need(browser['passed'] is True and browser['errors'] == [] and not browser.get('diagnostic_only'),
         'Browser journey is incomplete or diagnostic')
    need(browser['source'] == {'revision': driver_revision, 'driver_sha256': driver['sha256']},
         'Browser result has a different driver')
    need(browser['build']['revision'] == RUNTIME and browser['build']['binary_sha256'] == binary['sha256']
         and browser['build']['build']['revision'] in (RUNTIME, RUNTIME[:12]), 'Browser served a different runtime')
    refs.update(browser_proof=kept(browserproof), browser_log=kept(browserlog), browser_result=kept(browserfile),
                browser_driver=driver, browser_runner=browserrunner)

    fixturefile = path(cfg['fixture_manifest'])
    fixture = read(fixturefile)
    need(fixture['compiled_revision'] in (RUNTIME, RUNTIME[:12]) and fixture['player'] == 'France'
         and fixture['fixture'] == 's15-authored-flight-operations', 'Wrong authored native fixture')
    need(path(windows['fixture']['fixture']) == fixturefile.parent
         and path(browser['fixture']['manifest_path']) == fixturefile
         and browser['fixture']['manifest_sha256'] == sha(fixturefile), 'Fixture binding changed')
    beforefile = exact(browser['fixture'])
    need(beforefile == fixturefile.parent / fixture['before_file'], 'Wrong authored opening save')
    need(fixture['scope'] == browser['fixture']['scope'] and fixture['authored_preconditions'],
         'Authored preconditions are missing')
    for directory in (fixturefile.parent, browserfile.parent):
        for p in directory.rglob('*'):
            if p.is_file() and p.suffix.lower() not in ('.exe', '.dll'):
                kept(p)
    refs['fixture_manifest'] = kept(fixturefile)

    steps = fixture['steps']
    need(all(isinstance(s, dict) and len(s) == 1 and set(s) <= {'command', 'days', 'checkpoint'} for s in steps),
         'Unrecognized authored journey step')
    checkpoints = [s['checkpoint'] for s in steps if 'checkpoint' in s]
    command_names = [s['command'] for s in steps if 'command' in s]
    days = sum(s.get('days', 0) for s in steps)
    need(integer(days, 1) and all(integer(s['days'], 1) for s in steps if 'days' in s)
         and days == fixture['days_advanced'], 'Authored elapsed days differ')
    need(len(set(checkpoints)) == len(checkpoints) and set(checkpoints) == set(fixture['expected_stages']),
         'Missing or duplicate native checkpoint')
    stages = browser['stages']
    need([s['id'] for s in stages] == ['loaded'] + checkpoints + ['reloaded', 'continued'],
         'Browser omitted or reordered native checkpoints')
    need(len(browser['commands']) == len(command_names), 'Browser command count differs')
    for observed, name in zip(browser['commands'], command_names):
        payload = observed['payload']
        need(observed['stage'] == name and len(payload['commands']) == 1, 'Browser command order differs')
        command = {k: v for k, v in payload['commands'][0].items() if k != 'quote'}
        need(command == fixture['commands'][name], 'Browser command differs from its native oracle')
        need(isinstance(payload.get('session_id'), str) and payload['session_id']
             and isinstance(payload.get('client_id'), str) and payload['client_id']
             and integer(payload.get('request_seq'), 1), 'Browser command lacks a protected receipt')
    need(len(browser['advances']) == days and all(x['payload']['days'] == 1 for x in browser['advances']),
         'Browser days differ from ordinary one-day advances')
    first, last = stages[0]['flight'], stages[-1]['flight']
    need(first['as_of_day'] == fixture['opening_day'] and last['as_of_day'] - first['as_of_day'] == days,
         'Browser dates differ from the native calendar')

    # Canonical files are full worlds. Bind comparison digests to their retained
    # native archive audits, not merely to a successful browser Boolean.
    canonical = {}
    for p in (browserfile.parent / 'archive-audit').glob('*.json'):
        audited = read(p)
        if 'input' not in audited or 'canonical' not in audited:
            continue
        exact(audited['input'])
        exact(audited['canonical'])
        need(audited['canonical']['ignored_paths'] == [], 'Archive audit omits world fields')
        source_hash, world_hash = audited['input']['sha256'], audited['canonical']['sha256']
        need(source_hash not in canonical or canonical[source_hash] == world_hash, 'Conflicting native archive audit')
        canonical[source_hash] = world_hash
    need(canonical and browser['world_comparisons'] and browser['envelope_comparisons'], 'Missing save comparisons')
    for comparison in browser['world_comparisons']:
        need(comparison['ignored_paths'] == [] and digest(comparison['sha256']), 'Invalid world comparison')
        for side in ('left', 'right'):
            kept(exact(comparison[side]))
            need(canonical.get(comparison[side]['sha256']) == comparison['sha256'], 'Comparison lacks matching canonical bytes')
    for checkpoint in checkpoints:
        matches = [c for c in browser['world_comparisons'] if c['message'] == 'Exact native checkpoint ' + checkpoint]
        need(len(matches) == 1 and path(matches[0]['right']['path']) ==
             path(fixture['expected_stages'][checkpoint], fixturefile.parent), 'Checkpoint oracle differs')
    for phrase in ('Ordinary Load', 'Save/Load preserves', 'Continue preserves'):
        need(any(c['message'].startswith(phrase) for c in browser['world_comparisons']), 'Missing ' + phrase + ' proof')
    for c in browser['envelope_comparisons']:
        left, right = c['left'], c['right']
        need(digest(left['sha256']) and left['sha256'] == right['sha256'] and left['bytes'] == right['bytes']
             and integer(left['saved_unix']) and integer(right['saved_unix']), 'History envelopes differ')

    required, outcome = fixture['required_outcomes'], browser['required_outcomes']
    need(outcome['days_advanced'] == days and first.get('aviation') is None
         and first.get('airbases') is None and first.get('missions') is None, 'Incorrect authored opening/outcome')
    purchase = fixture['commands']['purchase_aircraft']
    deliveries = [d for d in last['deliveries'] if d['company'] == required['company']
                  and d['product'] == required['aircraft_product'] and d['quantity'] == purchase['quantity']
                  and d['purchased_day'] >= first['as_of_day']]
    need(len(deliveries) == 1, 'Expected paid aircraft delivery missing or duplicated')
    delivery = deliveries[0]
    need(delivery == outcome['paid_aircraft_delivery'] and delivery['total_price_bn'] > 0
         and delivery['settled_day'] is not None and delivery['delivered_day'] > delivery['purchased_day'],
         'Aircraft purchase was not actually paid and delivered')
    holding = lambda f, key: sum(h['units'] for h in f['holdings'] if h.get('design_id') == key)
    need(holding(first, required['aircraft_revision']) == 0, 'Purchased aircraft were granted in the opening')
    squadrons = last['aviation']['squadrons']
    need(any(s['id'] == required['purchased_squadron'] and s['revision'] == required['aircraft_revision'] for s in squadrons),
         'Delivered aircraft were not assigned')
    need(last['support'] and any(d['purchased_day'] >= first['as_of_day'] and d['total_price_bn'] > 0
                                for d in last['ammunition_deliveries']), 'No paid routine store purchase')
    for key in ('home_base', 'foreign_base'):
        bases = [b for b in last['airbases']['bases'] if b['id'] == required[key]]
        need(len(bases) == 1 and bases[0]['capacity_level'] == 1, 'Expected geographic base missing')
        need(any(p['track'] == 'capacity' and p.get('completed_day') is not None
                 and p['completed_day'] > p['started_day'] and abs(p['paid_bn'] - p['total_cost_bn']) < 1e-10
                 for p in bases[0]['history']), 'Base capacity did not complete through paid work')
    home = next(b for b in last['airbases']['bases'] if b['id'] == required['home_base'])
    need(home['support_level'] == home['protection_level'] == 1 and outcome['foreign_base'] == required['foreign_base'],
         'Base improvements differ')
    for track in ('support', 'protection'):
        need(any(p['track'] == track and p.get('completed_day') is not None
                 and p['completed_day'] > p['started_day'] and abs(p['paid_bn'] - p['total_cost_bn']) < 1e-10
                 for p in home['history']), 'Unpaid base improvement')
    orders = last['missions']['orders']
    cancelled = [o for o in orders if o['id'] == required['cancelled_mission']]
    need(len(cancelled) == 1 and cancelled[0]['status'] == 'cancelled'
         and outcome['cancelled_mission'] == required['cancelled_mission'], 'Mission cancellation missing')
    flown = [o for o in orders if o['status'] == 'flown']
    need(len(flown) == 2 and {o['kind'] for o in flown} == {'support_army', 'strike_target'}
         and flown == outcome['flown_missions'], 'Expected two tactical mission results differ')
    for order in flown:
        r = order['report']
        need(r['contacted'] is True and finite(r['stores_used']) and r['stores_used'] > 0
             and finite(r['applied_power']) and r['applied_power'] > 0 and integer(r['aircraft_lost'])
             and order['launch_day'] <= r['day'] <= last['as_of_day'], 'Mission did not produce a dated paid contact result')
    whole_losses = sum(o['report']['aircraft_lost'] for o in flown)
    purchased_losses = sum(o['report']['aircraft_lost'] for o in flown if o['squadron'] == required['purchased_squadron'])
    need(holding(last, required['aircraft_revision']) == purchase['quantity'] - purchased_losses,
         'Delivered aircraft and whole losses do not reconcile')
    need(all(not s.get('transit') and s['service_days_left'] == 0 for s in squadrons)
         and any(s.get('transit') for stage in stages for s in (stage['flight'].get('aviation') or {}).get('squadrons', [])),
         'Visible transit or completed funded recovery is missing')

    direct = {
        'sim': ['s15_whole_aircraft_loss_debits_only_the_flight_and_preserves_other_claims',
                's15_two_missions_share_finite_stores_and_replay_the_same_campaign_result',
                's15_prepared_plan_save_and_resume_consumes_one_shared_ammunition_receipt',
                's15_frozen_launch_cannot_be_cancelled_reassigned_or_replayed_after_resume',
                's15_shared_family_coverage_cannot_be_forged_to_reuse_the_same_stores'],
        'web': ['browser_advance_lost_response_retry_is_exactly_once_and_session_bound',
                'economic_order_receipts_are_once_only_current_and_campaign_bound'],
    }
    for lane, names in direct.items():
        for name in names:
            need(re.search(r'^test [^\n]*\b' + re.escape(name) + r' \.\.\. ok$', logs[lane], re.M),
                 'Missing direct native regression: ' + name)
    shell = 's12-s15 air shell sends all reviewed kinds through protected receipts and returns to flight'
    retry = 'immediate response loss/reload/retry charges exactly once and preserves original command'
    for name in (shell, retry):
        need(re.search(r'^✔ ' + re.escape(name) + r' \(', logs['node'], re.M), 'Missing transport regression: ' + name)
    direct['node'] = [shell, retry]

    reviewfile, preservationfile = path(cfg['review_launch']), path(cfg['preservation'])
    review, preservation = read(reviewfile), read(preservationfile)
    need(review['runtime_revision'] == RUNTIME and review['executable_sha256'] == binary['sha256']
         and review['build']['revision'] in (RUNTIME, RUNTIME[:12]) and review['player'] == 'France'
         and review['date'] == stages[-1]['date'] and review['required_outcomes'] == outcome, 'Review campaign differs')
    need(len(review['evidence']) == 2 and {str(exact(r)) for r in review['evidence']} ==
         {str(path(cfg['windows']['binary']['proof'])), str(browserfile)}, 'Review proof links differ')
    need(sha(path(review['directory']) / 'spheres-web.exe') == binary['sha256'], 'Copied review executable changed')
    source, copied = path(review['save_copy']['source']), path(review['save_copy']['copy'])
    need(source == path(browser['run']) / 'saves/s15-flight-operations.json'
         and sha(source) == sha(copied) == review['save_copy']['sha256'], 'Copied review save changed')
    native = review['native_verification']
    continued = [c for c in browser['world_comparisons'] if c['message'].startswith('Continue preserves')]
    need(len(continued) == 1 and native['ignored_paths'] == [] and native['exact_bytes_match_final_browser'] is True
         and native['canonical_sha256'] == continued[0]['sha256'], 'Review world differs from Continue')
    audited_file = path(review['directory']) / 'native-verification/copied-save.json'
    audited = read(audited_file)
    need(audited['input']['sha256'] == review['save_copy']['sha256']
         and audited['canonical']['ignored_paths'] == [] and audited['canonical']['sha256'] == native['canonical_sha256']
         and audited['canonical']['bytes'] == native['bytes'], 'Copied save audit differs')
    worker = kept(REPO / 'tools/ui/archive-worker.py')
    need(worker['sha256'] == native['worker_sha256'], 'Native archive worker changed')
    refs.update(review_launch=kept(reviewfile), review_save=kept(copied), review_source_save=kept(source),
                review_audit=kept(audited_file), review_canonical=kept(exact(audited['canonical'])), archive_worker=worker)
    need(preservation['passed'] is True and len(preservation['files']) == 8 and len(preservation['worktrees']) == 2
         and all(r['unchanged'] is True for r in preservation['files'] + preservation['worktrees'])
         and all(r['head'] == r['expected'] for r in preservation['worktrees']), 'Original preservation proof failed')
    for row in preservation['files']:
        exact(row)
    baseline = BASE / 'evidence/S08-preservation-before.json'
    need(sha(baseline) == preservation['baseline_sha256'], 'Preservation baseline changed')
    refs.update(preservation=kept(preservationfile), preservation_baseline=kept(baseline), inventory=record(invfile))

    visual = cfg.get('visual_review')
    need(isinstance(visual, dict) and visual.get('passed') is True and set(visual.get('widths', [])) == {1440, 390}
         and visual.get('screenshots') and isinstance(visual.get('notes'), str) and visual['notes'].strip()
         and 'REPLACE' not in visual['notes'], 'Supply actual desktop/mobile visual inspection findings')
    for name in visual['screenshots']:
        rel = pathlib.PurePosixPath(name)
        need(not rel.is_absolute() and '..' not in rel.parts and name in browser['screenshots'], 'Unknown visual review capture')
        kept(browserfile.parent / name)
    need(any('1440' in x for x in visual['screenshots']) and any('390' in x for x in visual['screenshots']),
         'Retain the inspected desktop and mobile captures')
    completed = dt.datetime.fromisoformat(browser['finished_utc'].replace('Z', '+00:00'))
    need(completed.tzinfo is not None, 'Browser completion timestamp lacks timezone')
    return {
        'format': 'spheres-s15-closure/v1', 'sessions': ['S12', 'S13', 'S14', 'S15'], 'status': 'complete',
        'candidate_revision': RUNTIME, 'driver_revision': driver_revision,
        'completed_utc': browser['finished_utc'], 'completed_date': completed.astimezone(dt.timezone.utc).date().isoformat(),
        's16_started': False, 'scope': fixture['scope'],
        'qualification': {'passed': True, 'windows': {k: windows[k].get('totals', {'passed': True}) for k in LANES},
            'browser': {'passed': True, 'driver_revision': driver_revision, 'runtime_revision': RUNTIME,
                        'changed_paths': bridge['changed_paths'], 'runtime_paths': bridge['runtime_paths'], 'runtime_diff': []},
            'linux': {c['name']: c['totals'] for c in linux['checks']}, 'native_ignored_tests': ignored,
            'days_advanced': days, 'ordinary_commands': len(command_names), 'native_checkpoints': len(checkpoints),
            'browser_capture_stages': len(stages), 'world_comparisons': len(browser['world_comparisons']),
            'envelope_comparisons': len(browser['envelope_comparisons']), 'ignored_world_paths': [],
            'history_timestamp_exception': 'saved_unix only, retained and validated separately', 'direct_regressions': direct},
        'build': browser['build'], 'scenario': {'country': 'France', 'opening_date': stages[0]['date'],
            'final_date': stages[-1]['date'], 'authored_preconditions': fixture['authored_preconditions']},
        'outcomes': outcome, 'observed_whole_aircraft_losses': whole_losses,
        'review': {'url': review['url'], 'launch': refs['review_launch'], 'native_verification': native},
        'visual_review': visual, 'proofs': refs,
        'retention': {'files': len(inv['files']), 'stored_bytes': inv['stored_bytes'],
            'all_reconstructions_verified': True, 'inventory_sha256': sha(invfile), 'scope': inv['scope']},
        'limits': ['Authored France scenario and explicit setup; not historical opening aircraft or an unassisted campaign.',
            'Browser comparisons cover named native checkpoints and completed Save/Load/Continue, not every intermediate day.',
            'Positive whole-aircraft loss, pending-order resume and interrupted-response behavior have separate native/transport regressions; browser observed losses are reported literally.',
            'Visual evidence covers desktop 1440 and mobile 390 pixels. No 320-pixel or independent human playtest is claimed.',
            'The final browser-only driver revision changes only tools/ui/ci-flight-operations.cjs. Native/Node suites and executable qualify the unchanged runtime candidate.',
            'S16 fighters/Defend skies remains planned. G3 and CP1 are unearned; prior gate decisions are unchanged.',
            'Existing character/content, long-campaign, performance, human usability and release qualification requirements remain open.',
            'Preservation covers eight named files and two recorded worktree HEADs, not every file or live server.',
            'No remote push, new performance certification or complete campaign certificate is claimed by this closure.'],
    }


def render(manifest):
    roadmapfile, pathwayfile = REPO / 'docs/planning/campaign-pathway.json', REPO / 'docs/CERTIFIED_CAMPAIGN_PATHWAY.md'
    original = read(roadmapfile)
    data = copy.deepcopy(original)
    target_ids = set(manifest['sessions'])
    need(next(s for s in data['sessions'] if s['id'] == 'S16')['status'] == 'planned', 'S16 has already started')
    need(data['execution']['authorized_through'] == data['execution']['stop_boundary'] == 'S15', 'Unexpected execution authorization')
    for session in data['sessions']:
        if session['id'] not in target_ids:
            continue
        need(session['status'] in ('in_progress', 'complete'), 'Session was not implemented: ' + session['id'])
        session.update(status='complete', evidence=f"../campaign-certification/{session['id']}/README.md",
                       completed_date=manifest['completed_date'], closure={'evidence': 'docs/campaign-certification/S15/manifest.json',
                           'candidate_revision': RUNTIME, 'driver_revision': manifest['driver_revision'],
                           'completed_utc': manifest['completed_utc']})
    data['last_completed_session'], data['next_session'] = 'S15', 'S16'
    data['date'] = manifest['completed_date']
    data['execution'].update(status='stopped', next_session_requires_instruction=True)
    need(data['gate_decisions'] == original['gate_decisions'], 'Gate decisions changed')
    need([s for s in data['sessions'] if s['id'] not in target_ids] ==
         [s for s in original['sessions'] if s['id'] not in target_ids], 'Unrelated session changed')
    pathway = pathwayfile.read_text(encoding='utf-8-sig')
    pathway = pathway.replace('S01–S11 complete; S12–S30 planned.', 'S01–S15 complete; S16–S30 planned.')
    pathway = pathway.replace('S01–S11 are complete; S12–S30 remain planned.', 'S01–S15 are complete; S16–S30 remain planned.')
    pathway = pathway.replace('records the qualified build and complete evidence. S12 remains planned.',
        'records the qualified ground build and evidence. Subsequent flight work is qualified in the [S12–S15 record](campaign-certification/S15/README.md).')
    for number in range(12, 16):
        start, end = pathway.index(f'#### S{number} —'), pathway.index(f'<a id="s{number + 1}"></a>')
        section = pathway[start:end]
        need(section.count('- [ ]') in (0, 3), 'Unexpected session acceptance section')
        section = re.sub(r'\*\*Status:\*\* (?:Planned|In progress|Complete)', '**Status:** Complete', section, count=1)
        section = section.replace('- [ ]', '- [x]')
        evidence = '**Evidence:** [Combined S12–S15 flight qualification](campaign-certification/S15/README.md).'
        if evidence not in section:
            section = section.rstrip() + '\n\n' + evidence + '\n\n'
        pathway = pathway[:start] + section + pathway[end:]
    old = 'Execution stopped after S11; S12 and later sessions require a new instruction. No later campaign, content or release certificate is awarded.'
    new = ('The later instruction “Continue through S15” authorized S12–S15. They are complete on the recorded runtime and '
           'authored France flight journey. Execution stopped after S15; S16 and later sessions require a new instruction. '
           'G3 and CP1 remain unearned. No later campaign, content or release certificate is awarded.')
    need(old in pathway or new in pathway, 'Cannot locate prior execution boundary')
    pathway = pathway.replace(old, new)
    q, scenario = manifest['qualification'], manifest['scenario']
    rows = []
    for platform, checks in [('Windows', q['windows']), ('Linux', q['linux'])]:
        for name in ('web', 'sim', 'integration', 'node'):
            t = checks[name]
            rows.append(f"| {platform} {name} | {t.get('pass', t.get('passed'))} | {t.get('skipped', t.get('ignored', 0))} |")
    readme = f"""# S15 — Tactical missions and campaign results

Status: **complete**, together with S12–S14, on runtime `{RUNTIME}`. Execution stops after S15; S16 remains planned.

Air command connects owned aircraft, saved squadrons, financially funded geographic bases, routine upkeep and compatible stores to **Support army** and **Strike target**. Both use the existing campaign contact and casualty resolver. Aircraft alone do not capture territory. New aircraft, transfers, service and refit reservations retain their separate ownership and readiness records.

The disclosed authored France journey ran from **{scenario['opening_date']}** to **{scenario['final_date']}** through **{q['days_advanced']} days**, **{q['ordinary_commands']} ordinary reviewed commands** and **{q['native_checkpoints']} named native checkpoints**. It paid for and received {manifest['outcomes']['paid_aircraft_delivery']['quantity']} supplier aircraft, assigned purchased aircraft, completed home and consenting-host bases, completed Support and Protection improvements, displayed foreign transit, bought compatible stores under the support cap, cancelled one mission and flew both tactical mission kinds before funded recovery. The browser observed **{manifest['observed_whole_aircraft_losses']} whole-aircraft losses**; this is the recorded result, not a claim that a positive loss occurred.

| Qualification | Passed | Ignored / skipped |
|---|---:|---:|
{chr(10).join(rows)}

The fixture export passed separately. Existing ignored native tests and the skipped Node case remain listed in the [manifest](manifest.json); they are not counted as passes. The browser completed **{q['world_comparisons']} full-world comparisons** and **{q['envelope_comparisons']} historical-envelope comparisons**. No world fields are ignored; only the separately validated save timestamp may differ between historical envelopes. These comparisons cover named native checkpoints and final Save/Load/Continue, not every intermediate day.

Direct native tests separately cover positive whole-aircraft loss, finite stores shared by simultaneous missions, prepared-order save/resume and frozen-launch replay prevention. The shared transport tests cover lost-response retries and the four Air command kinds through protected session/client/sequence receipts. The authored browser journey does not simulate a lost network response.

The final browser driver is `{manifest['driver_revision']}`. Its only change from the runtime candidate is `{DRIVER}`; recorded diffs for simulation, web runtime and Cargo files are empty. Windows/Linux native and Node qualification remains bound to the unchanged runtime candidate. The final driver is qualified by the recorded final browser journey.

Desktop **1440px** and mobile **390px** captures were inspected: {manifest['visual_review']['notes']}

The [evidence inventory](evidence/inventory.json) retains {manifest['retention']['files']} complete selected input files, with verified byte reconstruction. Large files may be gzip-compressed and split; concatenate numbered pieces before decompression. Full saves, canonical worlds, proof records and selected original failed attempts are retained without filtering native fields. Executables are identified by source and SHA-256 rather than repackaged. Prior session evidence is preserved.

[Open the separate review campaign]({manifest['review']['url']}). Its copied final save and native canonical audit match the browser's Continue result. Preservation evidence covers the existing eight named files and two worktree HEADs.

The manifest lists every authored precondition, including funded starting fixtures, certified starting revisions, the France–Italy conflict and UK host consent. This is not historical opening equipment, an unassisted campaign or complete campaign certification. Whole-aircraft loss and upkeep rates, base capacity and range are explicit game assumptions. No 320px visual result, independent human playtest, new performance qualification or remote push is claimed. Existing historical/content, long-campaign and release requirements remain open. **G3 and CP1 remain unearned. Next: S16 — fighters and Defend skies**, requiring a new instruction.
"""
    outputs = {DEST / 'manifest.json': text_json(manifest), DEST / 'README.md': readme,
               roadmapfile: text_json(data), pathwayfile: pathway}
    for number in range(12, 15):
        p = REPO / f'docs/campaign-certification/S{number}/README.md'
        body = p.read_text(encoding='utf-8-sig')
        need(len(re.findall(r'^Status:.*$', body, re.M)) == 1, 'Unexpected session README status')
        status = (f'Status: **complete** on runtime `{RUNTIME}`, qualified with S12–S15. '
                  'See the [combined acceptance manifest](../S15/manifest.json).')
        outputs[p] = re.sub(r'^Status:.*$', lambda _: status, body, count=1, flags=re.M)
    return outputs


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('config', type=pathlib.Path)
    parser.add_argument('--apply', action='store_true')
    args = parser.parse_args()
    manifest = qualify(read(args.config))
    outputs = render(manifest)
    allowed = {REPO / 'docs/planning/campaign-pathway.json', REPO / 'docs/CERTIFIED_CAMPAIGN_PATHWAY.md',
               DEST / 'manifest.json', *[REPO / f'docs/campaign-certification/S{n}/README.md' for n in range(12, 16)]}
    need(set(outputs) == allowed and all(p.is_relative_to(REPO) for p in outputs), 'Unexpected closure write target')
    before = {p: p.read_bytes() if p.exists() else None for p in outputs}
    changes = [{'path': p.relative_to(REPO).as_posix(),
                'before_sha256': hashlib.sha256(before[p]).hexdigest() if before[p] is not None else None,
                'after_sha256': hashlib.sha256(value.encode('utf-8')).hexdigest(),
                'changed': before[p] != value.encode('utf-8')} for p, value in outputs.items()]
    if args.apply:
        for p in outputs:
            need(p.parent.is_dir() and not p.is_symlink(), 'Missing or linked completion document')
            need((p.read_bytes() if p.exists() else None) == before[p], 'Completion document changed during validation')
        for p, value in outputs.items():
            p.write_text(value, encoding='utf-8', newline='\n')
    print(text_json({'passed': True, 'applied': args.apply, 'candidate_revision': RUNTIME,
                     'driver_revision': manifest['driver_revision'], 'qualification': manifest['qualification'],
                     'changes': changes}))


if __name__ == '__main__':
    main()
