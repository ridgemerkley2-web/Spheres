"""Offline audit only: read and compare retained pilot evidence; never execute native code."""
import argparse
import gzip
import hashlib
import itertools
import json
import re
import subprocess
from pathlib import Path

BLOCK = 1024 * 1024
OLD = 'ae8084e853a8eb01ef4d834017af3e94c8e2cc82'
NEW = '68ba0622ec709b78617aadd1f9198d18f532bb32'


def read_json(path):
    return json.loads(path.read_text(encoding='utf-8'))


def pin(path):
    h, count = hashlib.sha256(), 0
    with path.open('rb') as stream:
        while chunk := stream.read(BLOCK):
            h.update(chunk)
            count += len(chunk)
    return {'path': path.as_posix(), 'bytes': count, 'sha256': h.hexdigest()}


def match_pin(actual, expected):
    assert actual['bytes'] == expected['bytes'], (actual, expected)
    assert actual['sha256'] == expected['sha256'], (actual, expected)


def confined(root, relative):
    path = (root / relative).resolve()
    assert path.is_relative_to(root.resolve()), relative
    return path


def canonical_chunks(path, observation):
    """Hash every decoded byte; strip only the compact final timestamp member.

    Fixed-sized emitted chunks permit a direct comparison even if timestamp
    decimal lengths differ. No JSON parsing or world normalization is used.
    """
    raw, canonical = hashlib.sha256(), hashlib.sha256()
    raw_count, canonical_count, buffered = 0, 0, b''
    with gzip.open(path, 'rb') as stream:
        while chunk := stream.read(BLOCK):
            raw.update(chunk)
            raw_count += len(chunk)
            buffered += chunk
            while len(buffered) > BLOCK + 128:
                emit, buffered = buffered[:BLOCK], buffered[BLOCK:]
                canonical.update(emit)
                canonical_count += len(emit)
                yield emit
    terminal = re.search(rb',"saved_unix":(0|[1-9][0-9]*)\}\Z', buffered)
    assert terminal is not None, f'No exact compact terminal timestamp: {path}'
    stamp = int(terminal.group(1))
    assert stamp <= 2**64 - 1
    final = buffered[:terminal.start()] + b'}'
    for offset in range(0, len(final), BLOCK):
        emit = final[offset:offset + BLOCK]
        canonical.update(emit)
        canonical_count += len(emit)
        yield emit
    observation.update(decoded={'bytes': raw_count, 'sha256': raw.hexdigest()},
                       canonical={'bytes': canonical_count, 'sha256': canonical.hexdigest()},
                       excluded_terminal_saved_unix=stamp)


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('--base', type=Path, required=True)
    parser.add_argument('--repo', type=Path, required=True)
    parser.add_argument('--out', type=Path, required=True)
    args = parser.parse_args()
    assert not args.out.exists(), 'Create-only review result'
    base, repo = args.base, args.repo
    validation = base / 'lazy-digest-validation-20260930'
    roots = {'old': base / 'lazy-digest-old-pilot-20260930', 'new': base / 'lazy-digest-pilot-20260930'}
    revisions = {'old': OLD, 'new': NEW}
    freezes = {k: read_json(v / 'freeze.json') for k, v in roots.items()}
    reports = {k: read_json(v / 'result.json') for k, v in roots.items()}
    inputs, native, archives, binaries = [], {}, {}, []
    manifest = read_json(validation / 'frozen/manifest.json')
    assert manifest['revision'] == NEW and manifest['qualification'] is False
    assert len(manifest['cells']) == 24
    for relative, expected in manifest['files'].items():
        actual = pin(confined(validation / 'frozen', relative))
        match_pin(actual, expected)
        inputs.append(actual)
    inputs.append(pin(validation / 'frozen/manifest.json'))
    for name, root in roots.items():
        freeze, report = freezes[name], reports[name]
        assert freeze['candidate_revision'] == revisions[name]
        assert freeze['expected_compiled_revision'] == revisions[name][:12]
        assert freeze['scope'] == report['scope'] == 'pilot'
        assert report['passed'] is True and report['binary_unchanged'] is True
        assert report['qualification'] is False and report['s25_complete'] is False
        assert report['coverage']['passed'] == report['coverage']['declared'] == 2
        assert report['coverage']['full_matrix_passed'] is False
        assert len(report['coverage']['missing_full_cases']) == 24
        assert not report['interrupted'] and not report['resource_halted']
        for key in ('binary', 'plan', 'harness'):
            actual = pin(Path(freeze[key]['path']))
            match_pin(actual, freeze[key])
            inputs.append(actual)
            if key == 'binary':
                binaries.append({'run': name, **actual})
        for key, filename in (('frozen_plan', 'plan.json'), ('frozen_harness', 'stability_matrix.py')):
            actual = pin(root / filename)
            match_pin(actual, freeze[key])
            inputs.append(actual)
        cell_ids = [row['id'] for row in report['cells']]
        assert cell_ids == ['france-1990', 'tonga-7']
        for row in report['cells']:
            cid = row['id']
            cell = root / 'cells' / cid
            assert row['passed'] and row['failure'] is None
            assert row['test_execution']['executed'] == row['test_execution']['passed'] == 1
            assert row['test_execution']['failed'] == row['test_execution']['ignored'] == 0
            for recorded in row['files']:
                actual = pin(confined(cell, recorded['relative']))
                match_pin(actual, recorded)
                inputs.append(actual)
            execution = read_json(cell / 'execution.json')
            assert execution['exit_code'] == 0
            assert execution['args'] == [freeze['binary']['path'], '--exact',
                's25_stability_tests::s25_stability_cell', '--ignored', '--nocapture', '--test-threads=1']
            match_pin(execution['binary_before'], freeze['binary'])
            match_pin(execution['binary_after'], freeze['binary'])
            request = read_json(cell / 'request.json')
            assert request['revision'] == revisions[name]
            n = read_json(cell / 'native/result.json')
            match_pin(pin(cell / 'native/result.json'), row['native_result'])
            assert n['revision'] == revisions[name] and n['compiled_revision'] == revisions[name][:12]
            assert n['passed'] and n['failure'] is None and not n['artifact_errors']
            assert n['days_each_leg'] == 398 and n['legs'] == 2
            assert n['start_native_date'] == '1990-01-01' and n['end_native_date'] == '1991-02-03'
            assert n['through'] == '1991-02-02'
            assert len(n['comparisons']) == 30 and all(c['matched'] for c in n['comparisons'])
            assert n['checks']['daily_invariant_checks'] == 796
            assert n['checks']['monthly_reloads'] == 13
            assert n['checks']['native_validation_checks'] == 60
            assert n['checks']['terminal_reloads'] == 2
            assert n['checks']['sandbox_continued'] is False
            native[name, cid] = n
            rows = read_json(cell / 'archive-manifest.json')['archives']
            assert rows == row['archives'] and len(rows) == 4
            assert {a['original_relative'] for a in rows} == {
                'uninterrupted/saves/final.json', 'resumed/saves/final.json',
                'resumed/saves/monthly.json', 'resumed/saves/monthly.json.bak'}
            for a in rows:
                path = confined(cell / 'native', a['gzip_relative'])
                actual = pin(path)
                match_pin(actual, a['gzip'])
                assert a['roundtrip_verified'] is True and a['raw_removed'] is True
                match_pin(a['decoded'], a['original'])
                archives[name, cid, a['original_relative']] = (path, a, actual)
            # Exact one-test evidence is checked independently of the native result.
            stdout = (cell / 'stdout.log').read_text(encoding='utf-8')
            assert re.search(r'test result: ok\. 1 passed; 0 failed; 0 ignored;', stdout)
            for p in [cell / 'verdict.json', cell / 'transfer.json', cell / 'stdout.log', cell / 'stderr.log']:
                inputs.append(pin(p))
        for p in [root / 'freeze.json', root / 'result.json', root / 'journal.jsonl', root / 'resource-setup.json']:
            inputs.append(pin(p))
    assert (roots['old'] / 'plan.json').read_bytes() == (roots['new'] / 'plan.json').read_bytes()
    assert (roots['old'] / 'stability_matrix.py').read_bytes() == (roots['new'] / 'stability_matrix.py').read_bytes()
    comparisons, archive_pairs = [], []
    for cid in ('france-1990', 'tonga-7'):
        old, new = native['old', cid], native['new', cid]
        equal_fields = ['actions', 'checks', 'comparisons', 'initial_rules', 'id', 'country', 'seed',
                        'through', 'start_native_date', 'end_native_date', 'days_each_leg', 'legs', 'scope']
        for field in equal_fields:
            assert old[field] == new[field], (cid, field)
        comparisons.append({'cell': cid, 'equal_fields': equal_fields, 'actions': len(new['actions']),
                            'comparisons': len(new['comparisons']), 'checks': new['checks'],
                            'complete_comparison_rows_equal': True})
        for relative in sorted(r for (name, cell_id, r) in archives if name == 'old' and cell_id == cid):
            observations = {}
            iterators = []
            for name in ('old', 'new'):
                path, _, compressed = archives[name, cid, relative]
                observations[name] = {'gzip': compressed}
                iterators.append(canonical_chunks(path, observations[name]))
            for left, right in itertools.zip_longest(*iterators):
                assert left == right, (cid, relative, 'complete canonical byte mismatch')
            for name in ('old', 'new'):
                _, row, _ = archives[name, cid, relative]
                match_pin(observations[name]['decoded'], row['decoded'])
                match_pin(observations[name]['decoded'], row['original'])
                if relative.endswith('/final.json'):
                    leg = relative.split('/')[0]
                    cell_row = next(c for c in reports[name]['cells'] if c['id'] == cid)
                    recorded = next(a for a in cell_row['native_validation']['final_archives'] if a['leg'] == leg)
                    match_pin(observations[name]['canonical'], recorded['canonical'])
            archive_pairs.append({'cell': cid, 'original_relative': relative,
                                  'whole_canonical_bytes_equal': True, **observations})
    # Both legs also match within each candidate, as complete decoded bytes.
    for cid in ('france-1990', 'tonga-7'):
        finals = [a for a in archive_pairs if a['cell'] == cid and a['original_relative'].endswith('/final.json')]
        for name in ('old', 'new'):
            assert finals[0][name]['canonical'] == finals[1][name]['canonical']
    git = lambda *cmd: subprocess.check_output(['git', '-C', str(repo), *cmd])
    changed = git('diff', '--name-only', OLD, NEW, '--', 'spheres-sim', 'spheres-web',
                  'spheres-cli', 'Cargo.toml', 'Cargo.lock').decode().splitlines()
    assert set(changed) == {'spheres-sim/src/government.rs', 'spheres-sim/src/government_a1_observer.rs',
                           'spheres-web/src/s25_stability_tests.rs', 'spheres-web/ui/globe3d.js', 'spheres-web/ui/index.html'}
    source_pins = []
    for path in changed + ['spheres-web/src/storage.rs', 'spheres-web/src/main.rs', 'spheres-web/build.rs']:
        row = {'path': path}
        for name, revision in revisions.items():
            if name == 'old' and path.endswith('government_a1_observer.rs'):
                row[name] = None
                continue
            data = git('show', f'{revision}:{path}')
            row[name] = {'bytes': len(data), 'sha256': hashlib.sha256(data).hexdigest(),
                         'git_blob': git('rev-parse', f'{revision}:{path}').decode().strip()}
        source_pins.append(row)
    frozen_sources = []
    for frozen, source in [('stability_matrix.py', 'tools/campaign/stability_matrix.py'),
                           ('distributed_stability.py', 'tools/campaign/distributed_stability.py'),
                           ('plan.json', 'tools/campaign/stability-full.json')]:
        actual = (validation / 'frozen' / frozen).read_bytes()
        original = git('show', f'{NEW}:{source}')
        assert actual.replace(b'\r\n', b'\n') == original
        frozen_sources.append({'frozen': frozen, 'source': source, 'revision': NEW,
            'git_blob': git('rev-parse', f'{NEW}:{source}').decode().strip(),
            'git_blob_bytes': len(original), 'git_blob_sha256': hashlib.sha256(original).hexdigest(),
            'relationship': 'Exact Git blob after CRLF-to-LF replacement; raw frozen bytes pinned separately.'})
    build_rows = [json.loads(x) for x in (validation / 'build.jsonl').read_text().splitlines()]
    assert build_rows[-1] == {'reason': 'build-finished', 'success': True}
    web = [x for x in build_rows if x.get('reason') == 'compiler-artifact' and x['target']['name'] == 'spheres-web']
    assert len(web) == 1 and web[0]['profile']['test'] and web[0]['profile']['opt_level'] == '3'
    sim = [x for x in build_rows if x.get('reason') == 'compiler-artifact' and x['target']['name'] == 'spheres_sim']
    assert len(sim) == 1 and sim[0]['profile']['test'] is False
    assert (validation / 'revision.txt').read_text().strip() == NEW
    assert 'test result: ok. 9 passed; 0 failed; 2 ignored;' in (validation / 'focused-native.log').read_text()
    assert 'Ran 71 tests' in (validation / 'tooling.log').read_text() and 'OK (skipped=1)' in (validation / 'tooling.log').read_text()
    retained_verifiers = []
    for filename, revision in [('pilot-verification.json', NEW), ('old-pilot-verification.json', OLD)]:
        report = read_json(validation / filename)
        assert report['integrity_verified'] and report['passed']
        assert report['candidate_revision'] == revision and report['archives_verified'] == 8
        assert report['native_reexecuted'] is False and report['binary_reexecuted_or_rebuilt'] is False
        assert report['qualification'] is False and report['s25_complete'] is False
        retained_verifiers.append({'file': pin(validation / filename), 'recorded_archives_verified': 8,
                                   'independently_reexecuted_by_this_review': False})
    for path in validation.iterdir():
        if path.is_file() and path.suffix in ('.json', '.jsonl', '.log', '.txt', '.py') and not path.name.startswith('full.'):
            inputs.append(pin(path))
    unique_inputs = {p['path']: p for p in inputs}
    result = {'format': 'spheres-pilot-equivalence-independent-review/v1', 'reviewer': 'Codex /root/review_gap_submission',
        'old_revision': OLD, 'new_revision': NEW, 'passed': True,
        'scope': 'Offline complete-byte equivalence of the two fresh native pilots; no native execution, benchmark, full-matrix or qualification claim.',
        'source_scope': 'Native changes are test-only observer instrumentation plus lazy mismatch diagnostics. Two additional embedded map UI changes are present and outside the native paired execution. Simulation dependency built without cfg(test); observer hooks are absent there.',
        'cells': comparisons, 'archive_pairs': archive_pairs,
        'totals': {'cells': 2, 'canonical_archive_pairs_compared_byte_for_byte': 8,
                   'gzip_files_independently_rehashed_and_fully_decoded': 16,
                   'decoded_bytes_read': sum(p[n]['decoded']['bytes'] for p in archive_pairs for n in ('old', 'new')),
                   'comparison_rows_compared': sum(c['comparisons'] for c in comparisons)},
        'binaries': binaries, 'source_pins': source_pins, 'frozen_source_relation': frozen_sources,
        'build_trace': {'build_records': len(build_rows), 'web_artifact': web[0], 'sim_artifact': sim[0]},
        'recorded_focused_tests': {'passed': 9, 'failed': 0, 'ignored_manual_preflights': 2},
        'recorded_tooling_tests': {'run': 71, 'passed': 70, 'skipped': 1},
        'root_retained_verifier_receipts': retained_verifiers,
        'limitation': 'This review rehashed binaries and all listed payloads, decoded all 16 gzip bodies, checked exact old/new canonical bytes and report fields. It did not rerun Cargo, native cells or the frozen strict verifier, or separately recalculate the diagnostic FNV. The preserved root verifier receipts checked native FNV linkage. Concurrent host work precludes a timing/speedup claim.',
        'inputs': list(unique_inputs.values()), 'qualification': False, 's25_complete': False}
    with args.out.open('x', encoding='utf-8', newline='\n') as f:
        json.dump(result, f, ensure_ascii=False, indent=2)
        f.write('\n')
    print(json.dumps({'passed': True, 'totals': result['totals'], 'result': pin(args.out)}))


if __name__ == '__main__':
    main()
