#!/usr/bin/env python3
"""Prepare S07 completion documents from a verified evidence package.

No writes unless --apply is supplied. Refuses missing country journeys,
qualification, protected-file evidence, performance acceptance, visual review,
or source anchors. Does not run tests, launch the game, or alter game/save data.
"""
from pathlib import Path, PurePosixPath
import argparse
import copy
import datetime
import hashlib
import json
import re
import subprocess
import sys

BASE = Path(__file__).resolve().parent
PIN = '041007fbfda48cc027d0c24a1189705a1dcaa89b'
SERVER_SHA = 'e845d3179f72e1ec49ae68197b48cad94dcfd0648ab5f358e0e9d28115d6d3c7'
TEST_SHA = 'f9495c660ba1204413d7962d0a19b587c6d5f293bc77cd7137672303f9195d85'


def require(condition, message):
    if not condition:
        raise ValueError(message)


def sha(path):
    with Path(path).open('rb') as handle:
        return hashlib.file_digest(handle, 'sha256').hexdigest()


def read(path):
    return json.loads(Path(path).read_text(encoding='utf-8-sig'))


def pretty(value):
    return json.dumps(value, indent=2, ensure_ascii=False) + '\n'


def one_replace(text, before, after):
    require(text.count(before) == 1, f'Missing or ambiguous source anchor: {before[:110]}')
    return text.replace(before, after, 1)


def node_totals(path):
    text = Path(path).read_text(encoding='utf-8', errors='replace')
    result = {}
    for name in ['tests', 'pass', 'fail', 'skipped', 'cancelled']:
        matches = re.findall(r'(?:ℹ|#)\s+' + name + r'\s+(\d+)', text)
        require(matches, f'Missing Node {name} total: {path}')
        result[name] = int(matches[-1])
    require(result == {'tests': 1497, 'pass': 1496, 'fail': 0, 'skipped': 1, 'cancelled': 0},
            f'Unexpected final Node qualification totals: {result}')
    return result


def main(argv=None):
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('--repo', type=Path, default=BASE / 'integration')
    parser.add_argument('--runtime', default=PIN)
    parser.add_argument('--inventory', type=Path)
    parser.add_argument('--general-browser', default='evidence/qualification/S07-general-browser/result.json')
    parser.add_argument('--harness-proof', default='evidence/qualification/S07-harness-correction.json')
    parser.add_argument('--visual-review', default='evidence/qualification/S07-visual-review.json')
    parser.add_argument('--reviewed-screenshot', action='append', required=True,
                        help='Evidence-relative screenshot personally inspected; repeat for both countries')
    parser.add_argument('--completion-date', default=datetime.datetime.now(datetime.timezone.utc).date().isoformat())
    parser.add_argument('--apply', action='store_true')
    args = parser.parse_args(argv)
    repo, pin = args.repo.resolve(), args.runtime
    require(pin == PIN, 'This finalizer is pinned to the reviewed S07 runtime; review it again for a different candidate')
    document_source = subprocess.check_output(['git', 'rev-parse', 'HEAD'], cwd=repo, text=True).strip()
    require(document_source == pin,
            'Checkout HEAD differs from the qualified S07 runtime')
    session = repo / 'docs/campaign-certification/S07'
    inventory_path = args.inventory.resolve() if args.inventory else session / 'evidence/inventory.json'
    require(inventory_path == session / 'evidence/inventory.json', 'Use the collected in-repository evidence inventory')
    inv = read(inventory_path)
    require(inv.get('runtime_revision') == pin and inv.get('status') == 'evidence_collected_only', 'Inventory is not for this qualified runtime')
    require(inv.get('format') == 'spheres-s07-evidence-inventory', 'Unrecognized inventory schema')
    require(not (session / 'manifest.json').exists(), 'Refusing to replace an existing completion manifest')
    files = {}
    for row in inv['evidence_files']:
        relative = PurePosixPath(row['file'])
        require(relative.parts[0] == 'evidence' and not relative.is_absolute() and '..' not in relative.parts,
                'Unsafe evidence inventory path')
        path = session / str(relative)
        require(path.is_file() and not path.is_symlink() and path.stat().st_size == row['bytes']
                and sha(path) == row['sha256'], f'Collected evidence changed: {path}')
        require(row['file'] not in files, 'Duplicate evidence inventory entry')
        files[row['file']] = path
    actual = {p.relative_to(session).as_posix() for p in (session / 'evidence').rglob('*') if p.is_file()}
    require(actual == set(files) | {'evidence/inventory.json'}, 'Evidence contains untracked or missing artifacts')

    def evidence(relative):
        require(relative in files, f'Required evidence missing: {relative}')
        return files[relative]

    def original_file(original):
        matches = [row['file'] for row in inv['evidence_files']
                   if Path(row['original_path']).resolve() == Path(original).resolve()]
        require(matches, f'Harness provenance source/result was not collected: {original}')
        return files[matches[0]]

    def proof(name):
        value = read(evidence('evidence/selected/' + name))
        require(value.get('exit_code') == 0 and value.get('revision_before') == pin == value.get('revision_after')
                and value.get('clean_before') is True and value.get('clean_after') is True, f'Invalid final proof: {name}')
        return value

    binary = proof('S07-final-binary.json')
    native = proof('S07-final-native.json')
    proof('S07-final-node.json')
    require(binary['binary_sha256'] == SERVER_SHA and native['binary_sha256'] == TEST_SHA, 'Qualified executable/test binary mismatch')
    expected_native = dict(passed=1514, failed=0, ignored=75, targets=63)
    require(inv['qualification']['windows_native'] == expected_native
            and inv['qualification']['linux_native'] == expected_native, 'Unexpected full native totals')
    windows_node = node_totals(evidence('evidence/qualification/S07-final-node.log'))
    linux_node = node_totals(evidence('evidence/selected/S07-linux-final/node.log'))
    linux = read(evidence('evidence/selected/S07-linux-final/runner-result.json'))
    require(linux.get('status') == 'passed' and linux.get('candidate') == pin and len(linux.get('checks', [])) == 5
            and all(c['exit_code'] == 0 for c in linux['checks']), 'Linux qualification is incomplete')
    external = read(evidence('evidence/selected/S07-external-final/result.json'))
    require(external.get('candidate') == pin == external.get('candidate_after') and external.get('clean_after') is True
            and external.get('binary_sha256') == TEST_SHA and external.get('binary_unchanged') is True
            and len(external.get('checks', [])) == 3 and all(c['exit_code'] == 0 for c in external['checks']),
            'Windows external-save qualification is incomplete')

    performance = inv['qualification']['performance']
    require(performance.get('candidate') == pin and performance.get('passed') is True
            and performance.get('memory_passed') is True, 'Performance acceptance is missing or failed')
    rows = performance.get('rows', [])
    expected_cases = {(s, age) for s in ['idle_human', 'industry_and_war'] for age in [0, 10, 30]}
    require(len(rows) == 6 and {(r['scenario'], r['age']) for r in rows} == expected_cases, 'Missing frozen performance cases')
    for row in rows:
        require(row.get('passed') is True and row['sim_p95_ms'] <= 300 and row['whole_p95_ms'] <= 400
                and row['whole_max_ms'] <= 750, 'A frozen performance bar failed')
    require(0 < performance.get('sampled_private_bytes', 0) <= 1073741824, 'Missing or failed sampled-memory acceptance')
    perf_artifacts = [key for key, path in files.items() if path.name == 'acceptance.json' and read(path) == performance]
    require(perf_artifacts, 'Performance summary is not backed by an inventoried acceptance artifact')
    protocols = [key for key, path in files.items() if 'protocol' in path.name.lower() and path.suffix == '.json']
    require(protocols, 'Missing retained performance protocol')
    matching_protocol = False
    for key in protocols:
        value = read(files[key])
        if value.get('candidate') == pin and value.get('test_binary_sha256') == TEST_SHA:
            require(value.get('simulation_p95_limit_ms') == 300 and value.get('whole_turn_p95_limit_ms') == 400
                    and value.get('whole_turn_max_limit_ms') == 750
                    and value.get('sampled_private_memory_limit_bytes') == 1073741824, 'Performance protocol changed the frozen bars')
            matching_protocol = True
    require(matching_protocol, 'Performance protocol is not bound to the qualified test binary')
    audits = [key for key, path in files.items() if 'preservation' in path.name.lower() and path.suffix == '.json']
    require(audits, 'Missing preservation evidence')
    valid_audits = [key for key in audits if key.startswith('evidence/selected/') and len(read(files[key]).get('files', [])) == 8
                   and all(row.get('unchanged') is True for row in read(files[key])['files'])]
    require(valid_audits, 'No selected final eight-file preservation audit')

    journeys, browser_rows = {}, []
    for nation in ['France', 'Tonga']:
        meta = inv['browser'][nation]
        result = read(evidence(meta['result']))
        require(result.get('passed') is True and result.get('nation') == nation and not result.get('errors')
                and result['build']['revision'] == pin and result['build']['binary_sha256'] == SERVER_SHA,
                f'{nation} journey acceptance is missing or mismatched')
        require(sha(files[meta['result']]) == meta['result_sha256'], f'{nation} result hash changed')
        require(result.get('pause', {}).get('no_spending') is True
                and result.get('cancellation', {}).get('no_refund') is True
                and result.get('paid_save', {}).get('exact_construction') is True
                and result.get('completed_save', {}).get('exact_industry') is True
                and result.get('operating', {}).get('exact_response') is True
                and result.get('operating', {}).get('matched_focus') is True,
                f'{nation} required player journey markers are missing')
        journeys[nation] = result
        op = result['operating']['operation']
        browser_rows.append(f'| {nation} | {result["completed_date"]} | {op["output"]:g} {op["output_unit"]} on {op["date"]} | [Journey]({meta["result"]}) |')
    general = read(evidence(args.general_browser))
    require(general.get('passed') is True and general.get('build_evidence', {}).get('revision') == pin
            and general['build_evidence']['binary_sha256'] == SERVER_SHA,
            'Current general browser regression is missing, failed or uses a different runtime')
    require(general.get('delayed_boot_navigation_preserved') is True
            and general.get('pending_receipt_reload_recovered') is True and general.get('save_history_roundtrip') is True,
            'General browser transaction/navigation regression is incomplete')
    harness_proof = read(evidence(args.harness_proof))
    require(harness_proof.get('runtime_revision') == pin and harness_proof.get('runtime_unchanged') is True,
            'Harness correction does not preserve the qualified runtime')
    require(harness_proof.get('correction_summary') and harness_proof.get('runtime_limitation'),
            'Harness correction needs an honest correction and retained runtime limitation')
    for name in ['France', 'Tonga', 'general']:
        row = harness_proof['runs'][name]
        harness_path = original_file(row['harness'])
        result_path = original_file(row['result'])
        selected_result = evidence(inv['browser'][name]['result']) if name != 'general' else evidence(args.general_browser)
        require(sha(harness_path) == row['harness_sha256'], f'{name} executed harness source hash mismatch')
        require(sha(result_path) == sha(selected_result), f'{name} harness association names a different selected run')
        require(sha(result_path) == row['result_sha256'], f'{name} result differs from its harness provenance hash')
    wrapper = harness_proof.get('wrapper')
    if wrapper:
        require(sha(original_file(wrapper['path'])) == wrapper['sha256'], 'Candidate harness wrapper hash mismatch')
    renewals = journeys['Tonga'].get('annual_renewals', [])
    require(len(renewals) == 1 and renewals[0].get('date') == '1 Jan 1991'
            and renewals[0].get('renewed') is True
            and renewals[0].get('queue_progress_and_paid_work_unchanged') is True
            and renewals[0].get('control') == 'constructionBudgetForm Apply budget',
            'Selected Tonga journey lacks the real annual-authority renewal and unchanged-work assertion')
    require(journeys['Tonga'].get('advance_count') == 635 and journeys['Tonga'].get('completed_date') == '28 Sep 1991',
            'Tonga selected-run summary differs from the reviewed journey')
    reviewed = list(dict.fromkeys(args.reviewed_screenshot))
    for key in reviewed:
        require(key in files and files[key].suffix.lower() == '.png', f'Reviewed screenshot is not inventoried: {key}')
    for nation in ['france', 'tonga']:
        require(any(key.startswith(f'evidence/browser/{nation}/') for key in reviewed), f'Missing explicit {nation} screenshot review')
    visual = read(evidence(args.visual_review))
    require(visual.get('runtime_revision') == pin, 'Visual review is for a different runtime')
    for nation in ['France', 'Tonga']:
        row = visual.get(nation, {})
        require('blocking_defects' in row and row['blocking_defects'] == [] and row.get('screenshots_reviewed'),
                f'{nation} saved visual review is absent or has a blocker')
        reviewed_result = Path(row.get('result', ''))
        if not reviewed_result.is_absolute():
            reviewed_result = repo / reviewed_result
        require(sha(original_file(reviewed_result)) == inv['browser'][nation]['result_sha256'], f'{nation} visual review names another attempt')
        for key in reviewed:
            if key.startswith(f'evidence/browser/{nation.lower()}/'):
                require(PurePosixPath(key).name in row['screenshots_reviewed'], f'Screenshot not present in saved visual review: {key}')

    failed_attempts = []
    for key, path in files.items():
        if key.startswith('evidence/failed-browser-attempts/') and path.name == 'result.json':
            result = read(path)
            require(result.get('passed') is not True, 'Passed run mislabeled as failed attempt')
            failed_attempts.append({'file': key, 'nation': result.get('nation'), 'stage': result.get('failed_stage'),
                                    'advance_count': result.get('advance_count'),
                                    'failure': str(result.get('failure', '')).splitlines()[0][:350]})
    require(any(row['nation'] == 'Tonga' and row['advance_count'] == 803 for row in failed_attempts),
            'The failed Tonga journey that omitted annual renewal must remain in the evidence package')
    failures_text = '\n'.join(f'- [{row["nation"] or "Browser"}: {row["stage"] or "unrecorded stage"}]({row["file"]}): '
                              + row['failure'].replace('`', '') for row in failed_attempts)
    if not failures_text:
        failures_text = 'No failed browser attempts were recorded in the selected evidence package.'
    limits = [
        'Construction remains funding-only. Operation uses its own workforce, qualifications, inputs, power, storage and department authority.',
        'The daily construction ceiling persists across years, but annual capital authority requires renewal through Apply funding. The queue can report budget exhausted without distinguishing expired authority; the detailed cash-flow alert explains it. This wording limitation remains.',
        'Current readiness, installed capacity and dated operation are separate; older receipts are not given invented workers, owners, completion dates or paid history.',
        'Research and equipment readiness depends on selected work; generic staffing does not certify a project or production line.',
        'Facility power totals include only displayed same-date receipts, not all national consumption.',
        'GDP valuation is the existing province/national contribution counted once; inherited value is excluded from incremental additions, and value added is not Treasury revenue.',
        'France and Tonga are bounded construction journeys, not the full eight-country 1990–2035 campaign matrix.',
        'The six frozen regression workloads and sampled memory do not certify all supplier-heavy or 2035 workloads; S22 retains broader performance ownership.',
        'No G2, CP1 or full campaign certificate is awarded. Stop after S07; S08 requires a new instruction.'
    ]
    manifest = {'session': 'S07', 'status': 'complete', 'completed_date': args.completion_date,
        'runtime_revision': pin, 'prior_runtime_revision': 'e2f66bca0614cce254f3bb15122cff0dadff128b',
        'baseline_source_revision': 'e0b117d9466b006c230882558154d2ffdb45fad3',
        'documentation_source_revision': document_source,
        'binary': binary, 'windows_native': expected_native, 'linux_native': expected_native,
        'node': {'windows': windows_node, 'linux': linux_node}, 'external_save_checks_each_platform': 3,
        'browser': inv['browser'], 'general_browser': args.general_browser,
        'harness_correction': harness_proof, 'visual_review': visual,
        'tonga_annual_renewals': renewals,
        'reviewed_screenshots': reviewed, 'failed_browser_attempts': failed_attempts,
        'performance': performance, 'performance_acceptance': perf_artifacts[0],
        'preservation_evidence': valid_audits, 'limits': limits, 'next_session': 'S08', 'stop_after': 'S07',
        'evidence_inventory': {'file': 'evidence/inventory.json', 'bytes': inventory_path.stat().st_size, 'sha256': sha(inventory_path)},
        'evidence_files': inv['evidence_files'] + [{'file': 'evidence/inventory.json', 'bytes': inventory_path.stat().st_size, 'sha256': sha(inventory_path)}],
        'external_references': inv['external_references'],
        'authoring_tool': {'original_path': str(Path(__file__).resolve()), 'sha256': sha(__file__)}}
    max_sim = max(r['sim_p95_ms'] for r in rows)
    max_whole = max(r['whole_p95_ms'] for r in rows)
    max_turn = max(r['whole_max_ms'] for r in rows)
    readme = f'''# S07 — Construction, staffing and operating outcomes

Status: complete on `{pin}`. Execution stopped after S07, before S08.

Construction now leads directly to the installed facility and its operating explanation. Current staffing and rated operating requirements are separate from the last dated output, workers used and charges. Expansion does not rewrite old receipts; new unmatched assignments stay unknown until the normal workforce match. Construction continues to require financial funding only.

The Industry view includes Office District, Advanced Industry and dock services alongside existing processing and workshop operations. Current requirements use the native recipes and contractor terms. Value added is the existing province/national contribution counted once, with inherited GDP distinguished from incremental output. The power summary counts displayed receipts on the same date only.

## Verification

[Machine-readable manifest](manifest.json) · [Raw evidence inventory](evidence/inventory.json).

| Check | Result |
|---|---|
| Full native suite, Windows and Linux | 1,514 passed per platform; 0 failed, 75 ignored, 63 completed targets |
| Full UI suite, Windows and Linux | 1,496 passed per platform; 0 failed, 1 skipped |
| Original master, active supplier stages and party/equipment versions | Three external checks passed per platform |
| General browser regression | Passed on the same runtime: saved command response recovery, delayed boot navigation, named load and history roundtrip |
| France and Tonga actual browser journeys | Financial previews, priorities, paid work, shared pause, reviewed cancellation without refund, paid save/load, visible annual-authority renewal in Tonga, completion, exact facility focus and saved outcomes |
| Six frozen performance workloads | All passed; worst simulation p95 {max_sim:.2f} ms, whole-turn p95 {max_whole:.2f} ms, maximum {max_turn:.2f} ms |
| Sampled private memory | {performance['sampled_private_bytes']:,} bytes, below 1 GiB; sampling can miss short peaks |
| Protected campaign archives | All eight original hashes unchanged |

| Country | Completed construction date | Recorded workshop result | Evidence |
|---|---|---|---|
{chr(10).join(browser_rows)}

These browser journeys use fresh campaigns and actual controls without granting funds, inputs, workers or installed capacity. The server receipt supplies the displayed output or blocked reason; installed capacity is not presented as production. Native/server tests cover scaled and ordinary projects, contractor-aware estimates, actual charges and inputs, staffing attribution, dated ownership, expansion, pure reads, inherited GDP and bundled workshop components. Desktop and narrow screenshots were reviewed: {', '.join(f'[{PurePosixPath(p).name}]({p})' for p in reviewed)}.

## Retained attempts and limits

{failures_text}

Development logs and all discovered S07 qualification/performance attempts are retained in the evidence inventory. The selected passed journeys do not remove earlier failed attempts; their result files retain the failure stage and diagnostic. No cause is inferred from a timeout alone.

The separately recorded startup diagnosis found an unconsumed roughly 2 MB state response holding the synchronous server response open. Consuming that body restored immediate small build responses. A later Tonga attempt omitted the existing annual capital renewal and reached the unchanged 800-day completion bound after 803 total advances. That failed run and its funding diagnosis are retained.

The selected Tonga journey reapplied the same daily ceiling through the visible funding control on 1 January 1991. It verified no immediate date, project-progress or paid-work change, then continued real daily advances and completed on 28 September 1991 after 635 total advances. These are two explicit harness corrections: startup polling/response handling and an additional annual-renewal player action. The accounting, save, outcome and completion-bound assertions remain; the harnesses are not claimed to have identical action sequences. The authoritative correction record is: {harness_proof['correction_summary']}

Neither correction changes the qualified game executable. The successful France journey retains its unchanged original harness association; corrected Tonga and general journeys retain theirs. Existing runtime limitation: {harness_proof['runtime_limitation']} See [harness provenance]({args.harness_proof}) and [visual review]({args.visual_review}). The slow-reader response behavior and the annual-renewal queue wording are not claimed to be fixed by this session.

{chr(10).join('- ' + item for item in limits)}

## Continue from here

S07 is complete and execution is stopped. **S08 — Complete supplier choice and reviewed imports** is next and remains planned. The existing G1 award remains bound to its S05 evidence; this session awards no later campaign, content or release certificate.
'''

    # Build every edit in memory and verify anchors before writing any source.
    existing_readme = (session / 'README.md').read_text(encoding='utf-8')
    require(existing_readme.startswith('# S07 —') and 'Status: in progress.' in existing_readme, 'S07 README already finalized or unexpected')
    roadmap_path = repo / 'docs/CERTIFIED_CAMPAIGN_PATHWAY.md'
    roadmap = roadmap_path.read_text(encoding='utf-8')
    roadmap = one_replace(roadmap, 'S01–S06 complete; execution stopped after S06.', 'S01–S07 complete; execution stopped after S07.')
    roadmap = one_replace(roadmap, 'S01–S06 are complete; S07–S30 remain planned. Execution is stopped after S06.',
                          'S01–S07 are complete; S08–S30 remain planned. Execution is stopped after S07.')
    start, end = roadmap.index('#### S07 —'), roadmap.index('<a id="s08"></a>')
    card = roadmap[start:end]
    require(card.count('- [ ]') == 3 and '**Status:** In progress' in card, 'Unexpected S07 acceptance card')
    card = card.replace('**Status:** In progress', '**Status:** Complete').replace('- [ ]', '- [x]')
    card += 'Evidence: [S07 construction, staffing and operating outcomes](campaign-certification/S07/README.md).\n\n'
    roadmap = roadmap[:start] + card + roadmap[end:]
    roadmap = one_replace(roadmap,
        'S06 is complete on its recorded runtime and evidence; execution has stopped after S06. S07 and later sessions still require a new instruction.',
        'S06 is complete on its recorded runtime and evidence. On 11 September 2026, “Next” authorized S07. S07 is now complete on its recorded runtime and evidence; execution has stopped after S07. S08 and later sessions still require a new instruction.')
    pathway_path = repo / 'docs/planning/campaign-pathway.json'
    pathway = read(pathway_path)
    before_gates = copy.deepcopy(pathway.get('gate_decisions'))
    sessions = {s['id']: s for s in pathway['sessions']}
    require(sessions['S07']['status'] == 'in_progress' and sessions['S08']['status'] == 'planned', 'Unexpected S07/S08 pathway status')
    require(pathway['execution']['authorized_through'] == 'S07' and pathway['execution']['stop_boundary'] == 'S07', 'Authorized execution boundary changed')
    sessions['S07']['status'] = 'complete'
    sessions['S07']['output'] = 'Build, staff and operate facilities, then understand their province and national effects through one connected flow.'
    pathway['last_completed_session'] = 'S07'
    pathway['next_session'] = 'S08'
    pathway['execution'].update(status='stopped', stopped_after='S07', next_session_requires_instruction=True)
    require(pathway.get('gate_decisions') == before_gates, 'Finalizer must not award another gate')
    outputs = {session / 'README.md': readme, roadmap_path: roadmap, pathway_path: pretty(pathway),
               session / 'manifest.json': pretty(manifest)}
    if args.apply:
        # The completion manifest is the final write after every preflight and
        # every other generated document has succeeded.
        for path, text in outputs.items():
            path.write_text(text, encoding='utf-8', newline='\n')
    print(pretty({'mode': 'applied' if args.apply else 'dry_run', 'runtime_revision': pin,
                  'qualified': True, 'stop_after': 'S07', 'next_session': 'S08',
                  'outputs': [str(path) for path in outputs], 'evidence_files': len(manifest['evidence_files'])}))


if __name__ == '__main__':
    try:
        main()
    except (ValueError, KeyError, OSError, subprocess.CalledProcessError) as exc:
        print(f'S07 finalization refused: {exc}', file=sys.stderr)
        sys.exit(1)
