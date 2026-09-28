"""Offline evidence analysis only: no game, browser, build or source changes."""
from pathlib import Path
from collections import defaultdict
from datetime import datetime, timedelta, timezone
import csv
import hashlib
import json
import math
import statistics
import subprocess

BASE = Path(r'C:\Users\ridge\Documents\Codex\2026-09-05\pick-up-the-spheres-game-on\work\campaign-certification')
ROOT = BASE / 'integration'
INPUT = BASE / 'evidence/codex-next-native-profile-1994-01'
OUTPUT = Path(__file__).resolve().parent
REVISION = 'ae8084e853a8eb01ef4d834017af3e94c8e2cc82'


def pin(path):
    path = Path(path)
    digest = hashlib.sha256()
    with path.open('rb') as file:
        while chunk := file.read(1024 * 1024):
            digest.update(chunk)
    return {'path': str(path.resolve()), 'bytes': path.stat().st_size, 'sha256': digest.hexdigest()}


def write(name, value):
    with (OUTPUT / name).open('x', encoding='utf-8', newline='\n') as file:
        json.dump(value, file, indent=2, ensure_ascii=False, allow_nan=False)
        file.write('\n')


def stats(values):
    values = sorted(values)
    assert values and all(math.isfinite(v) and v >= 0 for v in values)
    return {'samples': len(values), 'total_ms': sum(values), 'mean_ms': statistics.mean(values),
            'median_ms': values[len(values)//2],
            'p95_ms': values[math.ceil(len(values)*.95)-1], 'max_ms': values[-1], 'min_ms': values[0]}


profile = json.loads((INPUT / 'profile.json').read_text(encoding='utf-8-sig'))
execution = json.loads((INPUT / 'execution.json').read_text(encoding='utf-8-sig'))
rows = profile['rows']
assert profile['passed'] is True and profile['status'] == 'complete'
assert execution['exit_code'] == 0 and execution['timed_out'] is False
assert profile['revision'] == REVISION[:12] and execution['source_revision'] == REVISION
assert profile['requested_days'] == len(rows) == 31 and profile['input_unchanged'] is True
assert profile['starting']['calendar'] == [1994, 1, 1] and profile['ending']['calendar'] == [1994, 2, 1]
assert profile['starting']['seed'] == profile['ending']['seed'] == 1990
assert profile['starting']['player'] == profile['ending']['player'] == 'France'
assert profile['starting']['alive'] is True and profile['ending']['alive'] is True
assert profile['certified_profile_required'] is True and profile['renew_existing_budget'] is True
assert execution['environment']['SPHERES_S22_ADOPT_COMPETITION'] is None

pins = {name: pin(execution[key]) for name, key in [('binary', 'binary'), ('input', 'input'), ('original_input', 'original_input')]}
assert pins['binary']['sha256'] == execution['binary_sha256_before'] == execution['binary_sha256_after']
assert pins['input']['sha256'] == pins['original_input']['sha256'] == execution['input_sha256_before'] == execution['original_input_sha256_after'] == execution['copied_input_sha256_after']
assert pins['input']['bytes'] == pins['original_input']['bytes'] == execution['input_bytes'] == profile['input_fingerprint']['bytes']
assert execution['inputs_unchanged'] is True

proof = []
for index, row in enumerate(rows):
    before = datetime(1994, 1, 1) + timedelta(days=index)
    after = before + timedelta(days=1)
    label = lambda d: f'{d.day} {d.strftime("%b")} {d.year}'
    assert row['index'] == index and row['date_before'] == label(before) and row['date_after'] == label(after)
    assert row['exact_native_world'] is True and row['exact_returned_headlines'] is True
    assert row['validation_error'] is None and row['first_difference_byte'] is None
    assert row['expected_bytes'] == row['actual_bytes'] > 0
    assert row['headlines'] == row['log_after'] - row['log_before']
    # Native history compaction may reduce row count at month end.
    assert row['history_after'] >= 0 and row['history_before'] >= 0
    if index:
        assert row['history_before'] == rows[index-1]['history_after']
        assert row['log_before'] == rows[index-1]['log_after']
        assert row['orders']['commands'] == []
    proof.append({k: row[k] for k in ['index','date_before','date_after','exact_native_world','exact_returned_headlines',
                                    'expected_bytes','actual_bytes','validation_error','first_difference_byte','event_pause','headlines']})
assert list(rows[0]['orders']['commands'][0]) == ['SetAnnualBudget']
assert len(rows[0]['orders']['commands']) == 1

timing_keys = ['instrumented_tick_ms','native_game_tick_log_history_ms','comparison_ms','command_preflight_ms']
groups = {'all_31': rows, 'annual_jan_1': rows[:1], 'ordinary_jan_2_to_30': rows[1:-1], 'month_end_jan_31': rows[-1:]}
group_stats = {name: {key: stats([row[key] for row in selected]) for key in timing_keys} for name, selected in groups.items()}
for key, existing in profile['timing_summaries'].items():
    expected = group_stats['all_31'][key]
    for field in ['samples','median_ms','p95_ms','max_ms']:
        assert math.isclose(expected[field], existing[field], abs_tol=1e-9), (key, field)

stage_samples = defaultdict(list)
for row in rows:
    for stage in row['stages']:
        stage_samples[stage['name']].append(stage['elapsed_ms'])
ranked = []
for name, samples in stage_samples.items():
    item = {'name': name, **stats(samples), 'mean_per_completed_day_ms': sum(samples)/31}
    for field, raw_field in [('total_ms','total_ms'), ('mean_ms','mean_per_observation_ms')]:
        assert math.isclose(item[field], profile['subsystem_summaries'][name][raw_field], abs_tol=1e-8), name
    ranked.append(item)
ranked.sort(key=lambda row: row['total_ms'], reverse=True)
exclusive = [row for row in ranked if not row['name'].startswith('detail.')]
nested = [row for row in ranked if row['name'].startswith('detail.')]
assert all(row['samples'] == 31 for row in exclusive)
stage_total = sum(row['total_ms'] for row in exclusive)
outer_wall = execution['elapsed_seconds'] * 1000
accounting = {key: group_stats['all_31'][key]['total_ms'] for key in timing_keys}
accounting['explicit_clock_total_ms'] = sum(accounting.values())
accounting['outer_process_wall_ms'] = outer_wall
accounting['unattributed_outer_wall_ms'] = outer_wall - accounting['explicit_clock_total_ms']
accounting['exclusive_stage_total_ms'] = stage_total
accounting['instrumented_minus_exclusive_stages_ms'] = accounting['instrumented_tick_ms'] - stage_total
accounting['comparison_percent_outer_wall'] = 100*accounting['comparison_ms']/outer_wall
accounting['unattributed_percent_outer_wall'] = 100*accounting['unattributed_outer_wall_ms']/outer_wall
accounting['comparison_serialized_bytes_both_worlds'] = sum(r['expected_bytes']+r['actual_bytes'] for r in rows)
accounting['comparison_world_bytes_first_last'] = [rows[0]['actual_bytes'], rows[-1]['actual_bytes']]

with (INPUT / 'process.csv').open(newline='', encoding='utf-8-sig') as file:
    process = [{k: float(v) for k,v in row.items()} for row in csv.DictReader(file)]
intervals = [b['elapsed_ms'] - a['elapsed_ms'] for a,b in zip(process,process[1:])]
assert all(v > 0 for v in intervals)
assert all(b['total_cpu_ms'] >= a['total_cpu_ms'] for a,b in zip(process,process[1:]))
process_stats = {'samples': len(process), 'first_sample_ms': process[0]['elapsed_ms'],
                 'last_sample_ms': process[-1]['elapsed_ms'], 'sample_intervals': stats(intervals),
                 'last_sample_before_outer_finish_ms': outer_wall-process[-1]['elapsed_ms'],
                 'last_sample_cpu_ms': process[-1]['total_cpu_ms'],
                 'sampled_cpu_divided_by_outer_wall_single_core_percent':100*process[-1]['total_cpu_ms']/outer_wall,
                 'maxima': {key: max(row[key] for row in process) for key in process[0] if key not in ['elapsed_ms','total_cpu_ms']}}

retained = OUTPUT / 'retained'
retained.mkdir()
artifact_pins = []
for name in ['execution-start.json','execution.json','profile.json','process.csv','stdout.log','stderr.log']:
    raw = (INPUT / name).read_bytes()
    with (retained / name).open('xb') as file:
        file.write(raw)
    artifact_pins.append({'source':pin(INPUT/name), 'retained':pin(retained/name)})
    assert artifact_pins[-1]['source']['sha256'] == artifact_pins[-1]['retained']['sha256']
stderr = (INPUT/'stderr.log').read_text(encoding='utf-8-sig')
assert len(stderr.strip().splitlines()) == 31
assert stderr.count('exact world/headlines') == 31
assert '1 passed; 0 failed' in (INPUT/'stdout.log').read_text(encoding='utf-8-sig')

source = OUTPUT / 'pinned-source'
source.mkdir()
source_pins = []
for name in ['spheres-web/src/s22_diagnosis.rs','spheres-web/src/s22_performance.rs',
             'spheres-web/src/s08_performance_diagnostics.rs','spheres-web/src/performance.rs',
             'spheres-web/src/s25_stability_tests.rs']:
    raw = subprocess.check_output(['git','show',f'{REVISION}:{name}'],cwd=ROOT)
    blob = subprocess.check_output(['git','rev-parse',f'{REVISION}:{name}'],cwd=ROOT,text=True).strip()
    target = source / Path(name).name
    with target.open('xb') as file:
        file.write(raw)
    source_pins.append({'revision':REVISION,'repository_path':name,'git_blob':blob, 'retained':pin(target)})

analysis = {'format':'spheres-native-diagnostic-independent-review/v1','reviewer':'Codex /root/s20_preflight',
            'reviewed_utc':datetime.now(timezone.utc).isoformat(),'qualification':False,
            'scope':'Offline retained evidence and pinned source inspection only; no new native run or change.',
            'source_revision':REVISION,'identity_pins':pins,'provenance_limit':'Execution receipt plus embedded short revision bind the retained binary to the declared source; this review does not rebuild or independently reproduce that binary.',
            'checks':{'31_ordered_actual_days':True,'31_exact_world_flags':True,'31_exact_headline_flags':True,
                      'all_validation_errors_null':True,'all_serialized_lengths_match':True,'input_and_binary_rehashed':True,
                      'summary_values_recomputed':True,'31_stderr_success_lines':True},
            'daily_proof':proof,'starting':profile['starting'],'ending':profile['ending'],
            'first_day_ordinary_annual_command':rows[0]['orders'], 'timing_groups':group_stats,
            'exclusive_top_level_ranking':exclusive,'nested_ranking_do_not_add_to_parents':nested,
            'hierarchy':profile['timing_hierarchy'],'wall_accounting':accounting,'process_samples':process_stats,
            'raw_artifacts':artifact_pins,'pinned_sources':source_pins,
            'limitations':['Independent instrumented schedule runs before native Game each day; neither is an isolated qualification cell.',
                           'comparison_ms includes two full pretty native-world serializations, equality, headline comparison and row assembly; serialization is not separately timed.',
                           'Exact equality covers the complete native WorldState serialization and returned headlines, not equality of two full campaign envelopes/history archives; only one Game owns history/log.',
                           'Unattributed wall includes load/clone, fingerprints, validation, facts, report construction/serialization/write, freeing buffers and test overhead; this trace cannot apportion it.',
                           'Process CPU is a last observed cumulative value, not final exit CPU or subsystem CPU; no systemwide utilization measurement.',
                           'Periodic private/working samples may miss peaks. OS peak working/paged counters are separate Windows counters; none is a production single-world server-memory qualification.',
                           'Source/input/binary remain external; compact review packet retains raw timing evidence and pinned source, not the 135 MB input or 353 MB executable.']}
write('analysis.json',analysis)
print(json.dumps({'pins':pins,'ordinary':group_stats['ordinary_jan_2_to_30'],'annual':group_stats['annual_jan_1'],
                  'month_end':group_stats['month_end_jan_31'],'accounting':accounting,'process':process_stats,
                  'exclusive_top_8':exclusive[:8],'nested_top_12':nested[:12]}, indent=2))
