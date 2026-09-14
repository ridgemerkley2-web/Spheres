"""Validate a configured final S11 evidence set; dry-run unless --apply.

No tests, Git commands, servers, collections, or campaign actions are performed.
Only the four documented closure files may be written. Large retained files
are independently reconstructed and hashed before any documentation changes.
"""
import argparse
import copy
import datetime as dt
import hashlib
import json
import pathlib
import re
import zlib

BASE = pathlib.Path(__file__).resolve().parent
LANES = ('web', 'sim', 'integration', 'node', 'binary', 'fixture', 'browser')


def read(p):
    return json.loads(pathlib.Path(p).read_text(encoding='utf-8-sig'))


def sha(p):
    with pathlib.Path(p).open('rb') as f:
        return hashlib.file_digest(f, 'sha256').hexdigest()


def serialized(value):
    return json.dumps(value, ensure_ascii=False, indent=2, allow_nan=False) + '\n'


def path(value, parent=BASE):
    p = pathlib.Path(value)
    return (parent / p).resolve() if not p.is_absolute() else p.resolve()


def record(p):
    p = path(p)
    assert p.is_file() and not p.is_symlink(), str(p)
    return {'path': str(p), 'bytes': p.stat().st_size, 'sha256': sha(p)}


def exact(row, parent=BASE):
    p = path(row.get('path', row.get('file')), parent)
    assert p.is_file() and not p.is_symlink(), str(p)
    assert sha(p) == row['sha256'], 'Changed evidence: ' + str(p)
    if 'bytes' in row:
        assert p.stat().st_size == row['bytes'], str(p)
    return p


def totals(value, node=False):
    if node:
        passed, failed = value.get('pass', value.get('passed')), value.get('fail', value.get('failed'))
        assert passed > 0 and failed == 0
        assert value['tests'] == passed + failed + value['skipped']
    else:
        assert value['passed'] > 0 and value['failed'] == 0 and value['ignored'] >= 0
        assert value['completed_targets'] > 0
    return value


def inventory_check(file, selection):
    data = read(file)
    assert data['selection_sha256'] == sha(selection)
    assert data['selection'] == read(selection)
    index, storage_names = {}, set()
    stored = 0
    for row in data['files']:
        assert row['reconstruction_verified'] is True
        assert row['encoding'] in ('raw', 'gzip') and row['storage']
        digest, size = hashlib.sha256(), 0
        decoder = zlib.decompressobj(31) if row['encoding'] == 'gzip' else None
        for part in row['storage']:
            relative = pathlib.PurePosixPath(part['file'])
            assert not relative.is_absolute() and '..' not in relative.parts
            assert str(relative).casefold() not in storage_names
            storage_names.add(str(relative).casefold())
            p = exact(part, file.parent)
            assert p.is_relative_to(file.parent)
            stored += part['bytes']
            with p.open('rb') as f:
                while block := f.read(65536):
                    raw = decoder.decompress(block) if decoder else block
                    digest.update(raw)
                    size += len(raw)
        if decoder:
            raw = decoder.flush()
            digest.update(raw)
            size += len(raw)
            assert decoder.eof and not decoder.unused_data and not decoder.unconsumed_tail
        assert size == row['bytes'] and digest.hexdigest() == row['sha256'], row['logical_path']
        index.setdefault(str(path(row['source'])).casefold(), []).append(row)
    assert data['files'] and stored == data['stored_bytes']
    return data, index


def qualify(config):
    assert config['format'] == 'spheres-s11-closure-config/v1'
    pin = config['candidate_revision']
    assert re.fullmatch('[a-f0-9]{40}', pin)
    repo = path(config.get('repository', 'integration'))
    dest = repo / 'docs/campaign-certification/S11'
    invfile, selection = path(config['inventory']), path(config['selection'])
    assert invfile == dest / 'evidence/inventory.json'
    inv, retained = inventory_check(invfile, selection)

    def kept(p):
        p = path(p)
        r = record(p)
        rows = retained.get(str(p).casefold(), [])
        matches = [x for x in rows if x['sha256'] == r['sha256'] and x['bytes'] == r['bytes']]
        assert matches, 'Required raw evidence not retained: ' + str(p)
        r['logical_path'] = matches[0]['logical_path']
        r['storage'] = matches[0]['storage']
        return r

    windows, references = {}, {}
    assert set(config['windows']) == set(LANES)
    for lane in LANES:
        item = config['windows'][lane]
        proofpath, logpath = path(item['proof']), path(item['log'])
        proof = read(proofpath)
        assert proof['lane'] == lane and proof['passed'] is True and proof['exit_code'] == 0
        assert proof['revision'] == proof['revision_after'] == pin
        assert proof['clean_before'] is True and proof['clean_after'] is True
        assert proof['log_sha256'] == sha(logpath)
        if lane not in ('binary', 'browser'):
            totals(proof['totals'], lane == 'node')
        references['windows_' + lane] = {'proof': kept(proofpath), 'log': kept(logpath)}
        windows[lane] = proof
    binary = windows['binary']['binary']
    exact(binary)
    assert windows['browser']['binary']['sha256'] == windows['browser']['binary_sha256_before'] == binary['sha256']

    linuxfile = path(config['linux_result'])
    linux = read(linuxfile)
    assert linux['revision'] == pin and linux['passed'] is True and linux['status'] == 'passed'
    assert linux['runner_sha256'] == linux['runner_sha256_after']
    for tag in ('source_before', 'source_after'):
        assert linux[tag] == {'head': pin, 'status': ''}
    assert {c['name'] for c in linux['checks']} == {'web', 'sim', 'integration', 'node'} and len(linux['checks']) == 4
    references['linux_result'] = kept(linuxfile)
    for check in linux['checks']:
        assert check['passed'] is True and check['exit_code'] == 0
        for tag in ('source_before', 'source_after'):
            assert check[tag] == {'head': pin, 'status': ''}
        totals(check['totals'], check['name'] == 'node')
        logpath = path(check['log'], linuxfile.parent)
        assert sha(logpath) == check['log_sha256']
        references['linux_' + check['name']] = kept(logpath)
    if 'native_parent' in linux:
        # An environment-only Node retry may retain the successful native
        # lanes, provided their entire original proof is preserved unchanged.
        provenance = linux['native_parent']
        assert provenance['reused_checks'] == ['web', 'sim', 'integration']
        assert isinstance(provenance['reason'], str) and provenance['reason'].strip()
        assert linux['environment']['SPHERES_AUDIT_PYTHON'] == '/usr/bin/python3'
        parentfile = exact(provenance, linuxfile.parent)
        assert parentfile != linuxfile
        parent = read(parentfile)
        assert 'native_parent' not in parent and parent['revision'] == pin
        assert parent['passed'] is False and parent['status'] == 'failed'
        assert parent['runner_sha256'] == parent['runner_sha256_after']
        for tag in ('source_before', 'source_after'):
            assert parent[tag] == {'head': pin, 'status': ''}
        assert [c['name'] for c in parent['checks']] == ['web', 'sim', 'integration', 'node']
        assert [c['name'] for c in linux['checks']] == ['web', 'sim', 'integration', 'node']
        for original, observed in zip(parent['checks'][:3], linux['checks'][:3]):
            assert original['passed'] is True and original['exit_code'] == 0
            totals(original['totals'])
            for tag in ('source_before', 'source_after'):
                assert original[tag] == {'head': pin, 'status': ''}
            rebased = dict(original, log=observed['log'])
            assert observed == rebased, 'Reused native check changed beyond its log path'
            assert path(observed['log'], linuxfile.parent) == path(original['log'], parentfile.parent)
        failed_node = parent['checks'][3]
        assert failed_node['passed'] is False and failed_node['exit_code'] != 0
        for tag in ('source_before', 'source_after'):
            assert failed_node[tag] == {'head': pin, 'status': ''}
        assert failed_node['totals'].get('failed', failed_node['totals'].get('fail', 0)) > 0
        for check in parent['checks']:
            logpath = path(check['log'], parentfile.parent)
            assert sha(logpath) == check['log_sha256']
            references['linux_parent_' + check['name']] = kept(logpath)
        references['linux_native_parent'] = kept(parentfile)

    fixturefile, browserfile = path(config['fixture_manifest']), path(config['browser_result'])
    fixture, browser = read(fixturefile), read(browserfile)
    assert fixture['compiled_revision'] in (pin, pin[:12]) and fixture['player'] == 'France'
    assert path(windows['fixture']['fixture']) == path(windows['browser']['fixture']) == fixturefile.parent
    assert browser['passed'] is True and not browser.get('diagnostic_only') and browser['errors'] == []
    assert browser['test_source']['revision'] == browser['build']['revision'] == pin
    assert browser['test_source']['driver_sha256'] == sha(repo / 'tools/ui/ci-ground-operations.cjs')
    # Bind the selected result to the actual source-pinned Windows browser run.
    # An independently passing outside diagnostic cannot substitute for it.
    emitted_results = []
    for line in path(config['windows']['browser']['log']).read_text(encoding='utf-8-sig').splitlines():
        try:
            emitted = json.loads(line)
        except ValueError:
            continue
        if isinstance(emitted, dict) and emitted.get('passed') is True and isinstance(emitted.get('result'), str):
            emitted_results.append(path(emitted['result'], repo))
    assert emitted_results == [browserfile], 'Selected browser result is not the qualified runner result'
    assert browser['build']['build']['revision'] in (pin, pin[:12])
    assert browser['build']['binary_sha256'] == binary['sha256']
    assert path(browser['fixture']['manifest_path']) == fixturefile and browser['fixture']['manifest_sha256'] == sha(fixturefile)
    exact(browser['fixture'])
    steps, stages = fixture['steps'], browser['stages']
    assert len({s['id'] for s in steps}) == len(steps)
    assert set(fixture['expected_stages']) == {s['id'] for s in steps}
    expected_days = sum(s.get('days', 0) for s in steps)
    assert all(s.get('days', 1) == 1 for s in steps)
    assert expected_days == fixture['days_advanced'] == browser['elapsed_days'] > 0
    assert [s['id'] for s in stages] == ['loaded'] + [s['id'] for s in steps] + ['reloaded', 'continued']
    assert len(browser['advances']) == expected_days
    assert all(x['payload']['days'] == 1 for x in browser['advances'])
    expected_commands = [(s['id'], fixture['commands'][s['command']]) for s in steps if 'command' in s]
    assert len(browser['commands']) == len(expected_commands)
    for observed, (stage, command) in zip(browser['commands'], expected_commands):
        assert observed['stage'] == stage and len(observed['payload']['commands']) == 1
        assert {k: v for k, v in observed['payload']['commands'][0].items() if k != 'quote'} == command
    assert stages[0]['as_of_day'] == fixture['opening_day']
    assert stages[-1]['as_of_day'] - stages[0]['as_of_day'] == expected_days
    assert browser['final_state']['player'] == 'France'
    assert browser['final_state']['date'] == stages[-1]['date']
    assert browser['final_state']['as_of_day'] == stages[-1]['as_of_day']
    assert browser['world_comparisons'] and browser['envelope_comparisons']
    for comparison in browser['world_comparisons']:
        assert comparison['ignored_paths'] == []
        assert re.fullmatch('[a-f0-9]{64}', comparison['canonical_sha256'])
    for comparison in browser['envelope_comparisons']:
        assert comparison['left']['sha256'] == comparison['right']['sha256']
        assert comparison['left']['bytes'] == comparison['right']['bytes']
        for side in ('left', 'right'):
            assert isinstance(comparison[side]['saved_unix'], int) and comparison[side]['saved_unix'] >= 0
    # Native fixture comparisons are world-exact. Browser cancellation/reload
    # comparisons additionally retain the entire historical envelope.
    for phrase in ('Ordinary Load', 'Exact native fixture stage', 'Named Save/Load', 'Continue'):
        assert any(phrase in c['message'] for c in browser['world_comparisons']), phrase
    for step in steps:
        assert any(c['message'] == 'Exact native fixture stage ' + step['id'] for c in browser['world_comparisons'])

    required, outcome = fixture['required_outcomes'], browser['required_outcomes']
    assert outcome['days_advanced'] == expected_days
    refit = outcome['new_refit']
    refit_command = fixture['commands']['refit']
    assert refit['id'] == required['refit'] and refit['target_revision'] == required['target_revision']
    assert refit['quantity'] == refit['completed_units'] == refit_command['quantity'] > 0 and refit['cancelled_units'] == 0
    assert refit['product'] == refit_command['product'] and refit['source_revision'] == refit_command['source']
    assert refit['status'] == 'complete'
    assert refit['cancelled_day'] is None and refit['escrow_bn'] == 0
    assert stages[0]['as_of_day'] <= refit['booked_day'] < refit['closed_day'] <= stages[-1]['as_of_day']
    for key in ('vehicle_delivery', 'ammunition_delivery'):
        delivery = outcome['new_deliveries'][key]
        command = fixture['commands']['purchase' if key == 'vehicle_delivery' else 'resupply']
        assert delivery['id'] == required[key] and delivery['company'] == required['company']
        assert delivery['buyer'] == 'France' and delivery['quantity'] == command['quantity'] > 0 and delivery['status'] == 'delivered'
        assert delivery['company'] == command['company'] and delivery['product'] == command['product']
        assert stages[0]['as_of_day'] <= delivery['purchased_day'] < delivery['delivered_day'] <= stages[-1]['as_of_day']
    assert outcome['new_deliveries']['vehicle_delivery']['revision_id'] == required['target_revision']
    assert outcome['returned_target_units'] >= refit['completed_units'] + outcome['new_deliveries']['vehicle_delivery']['quantity']
    assert outcome['retirement']['revision'] == required['retired_revision']
    assert outcome['retirement']['quantity'] == required['retired_quantity'] > 0
    assert outcome['ammunition_consumed_delta'] > 0 and len(set(required['conflicts'])) == 2
    unique_receipts = {}
    for entry in outcome['shared_combat_receipts']:
        receipt = entry['receipt']
        assert receipt['conflicts'] == required['conflicts']
        assert stages[0]['as_of_day'] <= receipt['day'] <= stages[-1]['as_of_day']
        assert any(row['lost'] > 0 for row in receipt['revisions'])
        key = receipt['day']
        if key in unique_receipts:
            assert unique_receipts[key] == receipt
        unique_receipts[key] = receipt
        for row in receipt['revisions']:
            assert isinstance(row['lost'], int) and row['lost'] >= 0
            assert row['opening_delivered'] - row['lost'] == row['remaining_delivered']
            assert row['opening_available'] - row['lost'] == row['remaining_available']
            assert row['opening_reserved'] == row['remaining_reserved']
    assert unique_receipts
    # Every raw fixture/browser file must be collected, including the complete
    # worlds and canonical projections. No field-filtered substitute qualifies.
    for directory in (fixturefile.parent, browserfile.parent):
        for p in directory.rglob('*'):
            if p.is_file() and p.suffix.lower() not in ('.exe', '.dll'):
                kept(p)
    references['fixture_manifest'], references['browser_result'] = kept(fixturefile), kept(browserfile)

    reviewfile, preservationfile = path(config['review_launch']), path(config['preservation'])
    review, preservation = read(reviewfile), read(preservationfile)
    assert review['runtime_revision'] == pin and review['executable_sha256'] == binary['sha256']
    assert review['build']['revision'] in (pin, pin[:12])
    assert review['player'] == 'France' and review['date'] == stages[-1]['date']
    assert review['required_outcomes'] == outcome
    expected_review_evidence = {str(path(config['windows']['binary']['proof'])), str(browserfile)}
    assert len(review['evidence']) == 2
    assert {str(exact(row)) for row in review['evidence']} == expected_review_evidence
    native = review['native_verification']
    assert native['ignored_paths'] == [] and native['exact_bytes_match_final_browser'] is True
    continued = [c for c in browser['world_comparisons'] if c['message'].startswith('Continue preserves')]
    assert len(continued) == 1 and native['canonical_sha256'] == continued[0]['canonical_sha256']
    source, copied = path(review['save_copy']['source']), path(review['save_copy']['copy'])
    assert source == path(browser['run']) / 'saves' / (browser['saved_slot'] + '.json')
    assert sha(source) == sha(copied) == review['save_copy']['sha256']
    audits = list((browserfile.parent / 'archive-audit').glob('*-s11-audit-continued.json'))
    assert len(audits) == 1
    final_audit = read(audits[0])
    copied_audit_file = path(review['directory']) / 'native-verification/copied-save.json'
    copied_audit = read(copied_audit_file)
    assert copied_audit['input']['sha256'] == review['save_copy']['sha256']
    for audited in (final_audit, copied_audit):
        canonical = audited['canonical']
        assert canonical['ignored_paths'] == []
        assert canonical['sha256'] == native['canonical_sha256'] and canonical['bytes'] == native['bytes']
        kept(exact(canonical))
    assert native['worker_sha256'] == sha(repo / 'tools/ui/archive-worker.py')
    kept(copied_audit_file)
    kept(source)
    kept(copied)
    assert preservation['passed'] is True and len(preservation['files']) == 8 and len(preservation['worktrees']) == 2
    assert all(row['unchanged'] is True for row in preservation['files'] + preservation['worktrees'])
    for row in preservation['files']:
        exact(row)
    assert all(row['head'] == row['expected'] for row in preservation['worktrees'])
    references['review_launch'], references['preservation'] = kept(reviewfile), kept(preservationfile)
    references['inventory'] = record(invfile)
    screenshots = config['visual_review']['screenshots']
    assert config['visual_review']['passed'] is True and screenshots
    assert {1440, 390, 320}.issubset(set(config['visual_review']['widths']))
    for name in screenshots:
        assert name in browser['screenshots']
        kept(browserfile.parent / name)
    assert config['visual_review']['notes'].strip() and 'REPLACE' not in config['visual_review']['notes']
    return repo, dest, {
        'format': 'spheres-s11-closure/v1', 'session': 'S11', 'status': 'complete',
        'candidate_revision': pin, 's11_complete': True, 's12_started': False,
        'completed_utc': browser['finished_utc'], 'scope': fixture['scope'],
        'qualification': {'passed': True, 'windows': {k: windows[k].get('totals', {'passed': True}) for k in LANES},
            'linux': {c['name']: c['totals'] for c in linux['checks']},
            'linux_native_parent': linux.get('native_parent'),
            'browser_days': expected_days, 'browser_commands': len(expected_commands),
            'native_expected_stages': len(steps), 'browser_capture_stages': len(stages),
            'world_comparisons': len(browser['world_comparisons']), 'history_comparisons': len(browser['envelope_comparisons']),
            'ignored_world_paths': [], 'history_timestamp_exception': 'saved_unix only, retained and validated separately'},
        'build': browser['build'], 'scenario': {'country': 'France', 'opening_date': stages[0]['date'],
            'final_date': stages[-1]['date'], 'authored_preconditions': fixture['authored_preconditions']},
        'outcomes': outcome, 'whole_vehicle_losses': sum(r['lost'] for receipt in unique_receipts.values() for r in receipt['revisions']),
        'review': {'url': review['url'], 'launch': references['review_launch'], 'native_verification': native},
        'visual_review': config['visual_review'], 'proofs': references,
        'retention': {'files': len(inv['files']), 'stored_bytes': inv['stored_bytes'], 'all_reconstructions_verified': True,
            'scope': inv['scope'], 'inventory_sha256': sha(invfile)},
        'limits': ['Authored France scenario; not historical opening equipment or an unassisted campaign.',
            'National losses are shared across fronts; no invented per-theatre vehicle ledger or separate damage system.',
            'C01–C07 and later historical, long-campaign, human usability, performance and release requirements remain open.',
            'G2 is unchanged. S11 alone does not earn G3 or CP1. No remote CI, GitHub push or new performance result is claimed.',
            'Preservation covers eight named files and two worktree HEADs; it is not a byte audit of all other worktrees or servers.']}


def render(repo, dest, manifest):
    roadmapfile = repo / 'docs/planning/campaign-pathway.json'
    pathwayfile = repo / 'docs/CERTIFIED_CAMPAIGN_PATHWAY.md'
    original = read(roadmapfile)
    data = copy.deepcopy(original)
    session = next(s for s in data['sessions'] if s['id'] == 'S11')
    assert session['status'] in ('in_progress', 'complete')
    assert next(s for s in data['sessions'] if s['id'] == 'S12')['status'] == 'planned'
    session.update(status='complete', evidence='../campaign-certification/S11/README.md',
        closure={'evidence': 'docs/campaign-certification/S11/manifest.json', 'candidate_revision': manifest['candidate_revision']})
    data['last_completed_session'], data['next_session'] = 'S11', 'S12'
    data['execution'].update(authorized_through='S11', stop_boundary='S11', status='stopped', next_session_requires_instruction=True)
    assert data['gate_decisions'] == original['gate_decisions']
    assert [s for s in data['sessions'] if s['id'] != 'S11'] == [s for s in original['sessions'] if s['id'] != 'S11']
    text = pathwayfile.read_text(encoding='utf-8-sig')
    text = text.replace('S01–S10 complete; S11 in progress; S12–S30 planned.', 'S01–S11 complete; S12–S30 planned.')
    text = text.replace('S01–S10 are complete; S11 is in progress; S12–S30 remain planned.', 'S01–S11 are complete; S12–S30 remain planned.')
    start, end = text.index('#### S11 —'), text.index('<a id="s12"></a>')
    section = text[start:end]
    assert section.count('- [ ]') in (0, 3)
    section = section.replace('**Status:** In progress', '**Status:** Complete')
    section = section.replace('tracks implementation and qualification. S12 remains planned.', 'records the qualified build and complete evidence. S12 remains planned.')
    section = section.replace('- [ ]', '- [x]')
    text = text[:start] + section + text[end:]
    old = 'Execution stopped after S10; S11 and later sessions require a new instruction. No later campaign, content or release certificate is awarded.'
    new = ('The subsequent “Next” authorized S11. S11 is complete on its exact recorded build and authored France journey. '
        'Execution stopped after S11; S12 and later sessions require a new instruction. No later campaign, content or release certificate is awarded.')
    assert old in text or new in text
    text = text.replace(old, new)
    q, out = manifest['qualification'], manifest['outcomes']
    linux_note = ('\nLinux retains the three successful native lanes from the same clean source revision. '
        'The full Node suite was rerun with the explicit Python 3 interpreter after an interpreter lookup failure; '
        'the original failed result and all logs remain in the evidence.\n') if q['linux_native_parent'] else ''
    rows = []
    for platform, checks in [('Windows', q['windows']), ('Linux', q['linux'])]:
        for name in ('web', 'sim', 'integration', 'node'):
            t = checks[name]
            rows.append(f"| {platform} {name} | {t.get('pass', t.get('passed'))} | {t.get('skipped', t.get('ignored', 0))} |")
    readme = f"""# S11 — Ground equipment and operations

Status: **complete** on `{manifest['candidate_revision']}`. Execution stops after S11; S12 remains planned.

The equipment desk connects delivered tanks and specialist vehicles to the shared national force, support funding, compatible ammunition and manufacturer services. National combat reports retain actual whole-vehicle losses and protected refit reservations. Later purchases, conversions and retirement preserve the dated report. Explicit operational adoption retains identifiers from ended legacy wars.

The authored France journey ran from **{manifest['scenario']['opening_date']}** through **{manifest['scenario']['final_date']}**, advancing **{q['browser_days']} days** through **{q['browser_commands']} ordinary commands** and **{q['native_expected_stages']} independently generated command/day checkpoints**. It completed {out['new_refit']['completed_units']} manufacturer refit, delivered {out['new_deliveries']['vehicle_delivery']['quantity']} purchased vehicle and {out['new_deliveries']['ammunition_delivery']['quantity']} purchased ammunition rounds, retired {out['retirement']['quantity']} vehicle, and recorded {manifest['whole_vehicle_losses']} whole-vehicle combat loss. Both intended fronts shared the national settlement. Maintenance and fractional attrition remain existing capability/accounting rules; this adds no separate vehicle damage system.

| Qualification | Passed | Ignored / skipped |
|---|---:|---:|
{chr(10).join(rows)}
{linux_note}

The browser verified {q['world_comparisons']} full-world comparisons and {q['history_comparisons']} historical-envelope comparisons. Only the independently validated save timestamp differs in the latter. Review/cancel, completed supplier services, physical stocks, dated losses and Save/Load/Continue are covered. Desktop, 390px and 320px layouts were reviewed: {manifest['visual_review']['notes']}

The [manifest](manifest.json) records exact proof paths, counts, build hashes, scenario preconditions and limitations. The [evidence inventory](evidence/inventory.json) retains {manifest['retention']['files']} complete input files with verified byte reconstruction; large files are gzip-compressed, sometimes split into numbered pieces. Concatenate pieces in recorded order before decompressing. No native fields were removed. Binaries are identified by hash and source, not repackaged in this evidence tree.

[Open the separate review campaign]({manifest['review']['url']}). This is a copy of the tested authored scenario. The preservation receipt verifies eight named protected files and two worktree HEADs.

The authored setup is not historical opening equipment, an unassisted campaign or campaign certification. C01–C07 remain open, G2 is unchanged, and S11 alone earns neither G3 nor CP1. No remote push, new performance qualification or human playtest is claimed here. Next: **S12 — real squadrons from owned aircraft**.
"""
    return {dest / 'manifest.json': serialized(manifest), dest / 'README.md': readme,
        roadmapfile: serialized(data), pathwayfile: text}


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('config', type=pathlib.Path)
    parser.add_argument('--apply', action='store_true')
    args = parser.parse_args()
    config = read(args.config)
    repo, dest, manifest = qualify(config)
    outputs = render(repo, dest, manifest)
    before = {p: p.read_bytes() if p.exists() else None for p in outputs}
    changes = [{'path': str(p.relative_to(repo)), 'before_sha256': hashlib.sha256(before[p]).hexdigest() if before[p] is not None else None,
        'after_sha256': hashlib.sha256(text.encode('utf-8')).hexdigest(), 'changed': before[p] != text.encode('utf-8')} for p, text in outputs.items()]
    if args.apply:
        for p in outputs:
            assert (p.read_bytes() if p.exists() else None) == before[p], 'Closure document changed during validation'
        assert dest.is_dir()
        for p, text in outputs.items():
            p.write_text(text, encoding='utf-8', newline='\n')
    print(serialized({'passed': True, 'applied': args.apply, 'candidate_revision': manifest['candidate_revision'],
        'qualification': manifest['qualification'], 'changes': changes}))


if __name__ == '__main__':
    main()
