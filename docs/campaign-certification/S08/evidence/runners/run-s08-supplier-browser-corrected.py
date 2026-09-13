#!/usr/bin/env python3
"""Run an explicitly identified corrected supplier harness on a qualified runtime.

Usage: python run-s08-supplier-browser-corrected.py PIN NATIVEPROOF BUILDPROOF
       EXPORTPROOF PATCHED_HARNESS EXPECTED_ORIGINAL_SHA EXPECTED_PATCHED_SHA UNIQUE_LABEL

Creates a new outside evidence folder. Does not edit the repository or old
runners. No build, mocked browser response, changed timeout or global override.
Do not run during compilation, other browser journeys or performance measurement.
"""
import argparse
import datetime
import hashlib
import json
import os
from pathlib import Path
import re
import shutil
import subprocess
import sys
import time

BASE = Path(__file__).resolve().parent
REPO = BASE / 'integration'
ORIGINAL = REPO / 'tools/ui/ci-supplier-imports.cjs'
OVERLAY = BASE / 's08-supplier-harness-overlay.cjs'
NATIVE_COMMAND = ['cargo', 'test', '--locked', '--release', '--workspace', '--no-fail-fast']
BUILD_COMMAND = ['cargo', 'build', '--locked', '--release', '-p', 'spheres-web']


def require(ok, why):
    if not ok:
        raise ValueError(why)


def stamp():
    return datetime.datetime.now(datetime.timezone.utc).isoformat()


def read(path):
    return json.loads(Path(path).read_text(encoding='utf-8-sig'))


def sha(path):
    with Path(path).open('rb') as stream:
        return hashlib.file_digest(stream, 'sha256').hexdigest()


def write(path, value):
    with Path(path).open('x', encoding='utf-8') as stream:
        json.dump(value, stream, indent=2)
        stream.write('\n')


def git(*args):
    return subprocess.check_output(['git', '-c', 'core.longpaths=true', *args],
        cwd=REPO, text=True, stderr=subprocess.PIPE).strip()


def source():
    return {'captured_utc': stamp(), 'revision': git('rev-parse', 'HEAD'),
        'status_porcelain': git('status', '--porcelain')}


def qualified_proof(proof, pin, command):
    require(proof.get('passed') is True and proof.get('preservation_passed') is True
        and proof.get('exit_code') == 0, 'Qualified proof did not pass with preservation')
    require(proof.get('clean_before') is True and proof.get('clean_after') is True
        and proof.get('revision_before') == proof.get('revision_after') == pin,
        'Qualified proof does not identify the same clean candidate')
    require(proof.get('command') == command, 'Qualified proof used a different command')
    if command == NATIVE_COMMAND:
        totals = proof.get('totals', {})
        require(totals.get('failed') == 0 and totals.get('passed', 0) > 0
            and totals.get('completed_targets', 0) > 0, 'Full native suite totals are not successful')
    binary = Path(proof['binary']).resolve()
    require(sha(binary) == proof['binary_sha256'], 'Qualified executable bytes changed: ' + str(binary))
    return binary


def select_input(export, suffix):
    matches = [row for row in export['archive_phases'] if row['file'].endswith(suffix)]
    require(len(matches) == 1, 'Exporter phase missing or ambiguous: ' + suffix)
    row = dict(matches[0])
    directory = Path(export['export_directory']).resolve()
    path = (directory / row['file']).resolve()
    require(path.is_relative_to(directory), 'Exporter phase path escapes its directory')
    require(path.stat().st_size == row['bytes'] and sha(path) == row['sha256'],
        'Genuine input phase bytes changed: ' + str(path))
    row['path'] = str(path)
    return row


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('pin')
    parser.add_argument('native_proof', type=Path)
    parser.add_argument('build_proof', type=Path)
    parser.add_argument('export_proof', type=Path)
    parser.add_argument('patched_harness', type=Path)
    parser.add_argument('original_sha256')
    parser.add_argument('patched_sha256')
    parser.add_argument('label')
    args = parser.parse_args()
    require(re.fullmatch('[0-9a-f]{40}', args.pin), 'Use the full lowercase runtime revision')
    for digest in [args.original_sha256, args.patched_sha256]:
        require(re.fullmatch('[0-9a-f]{64}', digest), 'Use full lowercase harness SHA256 values')
    require(re.fullmatch('[A-Za-z0-9_-]+', args.label), 'Use a simple unique output label')
    patched = args.patched_harness.resolve()
    require(not patched.is_relative_to(REPO.resolve()), 'Corrected harness must remain outside the repository')
    out = BASE / ('evidence/S08-supplier-browser-corrected-' + args.label)
    out.mkdir(exist_ok=False)
    record = {'format': 'spheres-s08-corrected-supplier-browser', 'version': 1,
        'candidate': args.pin, 'started_utc': stamp(), 'passed': False, 'exit_code': None,
        'scope': 'Explicit corrected test-harness source on the unchanged qualified runtime; not execution of the committed harness bytes.',
        'runner': str(Path(__file__).resolve()), 'runner_sha256': sha(__file__),
        'overlay_entry': str(OVERLAY), 'overlay_entry_sha256': sha(OVERLAY),
        'output_directory': str(out), 'proofs': [], 'phases': []}
    watched = []
    started = time.monotonic()
    try:
        before = source()
        record['source_before'] = before
        require(before['revision'] == args.pin and not before['status_porcelain'],
            'Corrected browser requires the exact clean qualified runtime')
        require(sha(ORIGINAL) == args.original_sha256, 'Original tracked harness differs from expected bytes')
        require(sha(patched) == args.patched_sha256, 'Corrected harness differs from expected bytes')
        # Git clean status binds the original checkout bytes (including its
        # CRLF form) to this commit; record the blob identity independently.
        original_blob = git('rev-parse', args.pin + ':tools/ui/ci-supplier-imports.cjs')
        proof_paths = [path.resolve() for path in [args.native_proof, args.build_proof, args.export_proof]]
        record['proofs'] = [{'path': str(path), 'sha256': sha(path)} for path in proof_paths]
        native, build, export = map(read, proof_paths)
        test_binary = qualified_proof(native, args.pin, NATIVE_COMMAND)
        game_binary = qualified_proof(build, args.pin, BUILD_COMMAND)
        require(export.get('passed') is True and export.get('integrity_passed') is True
            and export.get('native_validation_passed') is True and export.get('exit_code') == 0
            and export.get('candidate') == args.pin, 'Genuine exporter did not qualify this candidate')
        require(export['test_binary_sha256_before'] == export['test_binary_sha256_after'] == native['binary_sha256']
            and Path(export['test_binary']).resolve() == test_binary, 'Exporter used a different qualified test executable')
        for key in ['source_before', 'source_after']:
            require(export[key]['revision'] == args.pin and not export[key]['status_porcelain'],
                'Exporter source was not the same clean candidate')
        provenance = export['native_provenance']
        outcome = export['native_outcome']
        require(provenance['source_head'] == args.pin and provenance['source_status'] == ''
            and provenance['build']['revision'] == outcome['build']['revision'] == args.pin[:12],
            'Native exporter provenance differs from the qualified source')
        require(outcome.get('passed') is True and outcome.get('no_synthetic_endowments') is True
            and 1 <= outcome.get('actual_days_advanced', 0) <= 3000, 'Exporter did not earn stock through its ordinary journey')
        for suffix in ['-first-finished-stock.campaign.json', '-ready-before-purchase.campaign.json',
            '-purchased.campaign.json', '-delivered.campaign.json']:
            require(sum(row['file'].endswith(suffix) for row in export['archive_phases']) == 1,
                'Qualified exporter lacks its named phase: ' + suffix)
        phases = [select_input(export, '-ready-before-purchase.campaign.json'),
            select_input(export, '0360-progress.campaign.json')]
        record['phases'] = phases
        deal = outcome['contract']
        require(deal['seller'] == 'France' and deal['source_revision']['spec']['platform'] == 'ground_apc',
            'This supplier browser requires the same genuine French APC fixture')
        executed = out / 'executed-supplier-harness.cjs'
        with patched.open('rb') as input_stream, executed.open('xb') as output_stream:
            shutil.copyfileobj(input_stream, output_stream)
        require(sha(executed) == args.patched_sha256, 'Harness snapshot bytes changed while copying')
        record['harness_overlay'] = {'original_filename': str(ORIGINAL),
            'original_checkout_sha256': args.original_sha256, 'original_commit_blob': original_blob,
            'patched_source': str(patched), 'patched_sha256': args.patched_sha256,
            'executed_snapshot': str(executed), 'executed_sha256': sha(executed),
            'method': 'Node Module._compile(exact patched bytes, originalFilename); unchanged relative helpers and runtime',
            'final_commit_requirement': 'Apply the exact executed harness patch transparently in the later test/docs commit; runtime proof remains on this candidate.'}
        record['binaries'] = {'game': {'path': str(game_binary), 'sha256': build['binary_sha256']},
            'test': {'path': str(test_binary), 'sha256': native['binary_sha256']}}
        env = dict(os.environ)
        removed = sorted(key for key in env if key.startswith('SPHERES_') or key in ['NODE_OPTIONS', 'NODE_PATH'])
        for key in removed:
            env.pop(key)
        bound = {'SPHERES_BINARY': str(game_binary), 'SPHERES_EXPECTED_REVISION': args.pin,
            'SPHERES_BROWSER_CHANNEL': 'chrome', 'SPHERES_SUPPLIER_SAVE': phases[0]['path'],
            'SPHERES_SUPPLIER_PREPARATION_SAVE': phases[1]['path'], 'SPHERES_SUPPLIER_PROVENANCE': str(proof_paths[2]),
            'SPHERES_SUPPLIER_OUTPUT': str(out), 'SPHERES_SUPPLIER_SLOT': 's08-earned-stock',
            'SPHERES_SUPPLIER_MAX_DAYS': '120', 'SPHERES_AUDIT_PYTHON': sys.executable,
            'SPHERES_SUPPLIER_OFFER': f"supplier:{deal['seller']}:{deal['company']}:equipment:{deal['product']}"}
        env.update(bound)
        record['bound_environment'] = bound
        record['removed_environment_keys'] = removed
        record['working_directory'] = str(REPO)
        node = shutil.which('node', path=env.get('PATH'))
        require(node, 'Node executable is not available')
        overlay_proof = out / 'overlay-execution.json'
        command = [node, str(OVERLAY), str(ORIGINAL), str(executed), args.original_sha256,
            args.patched_sha256, str(overlay_proof)]
        record['command'] = command
        watched = record['proofs'] + phases + list(record['binaries'].values()) + [
            {'path': str(ORIGINAL), 'sha256': args.original_sha256},
            {'path': str(patched), 'sha256': args.patched_sha256},
            {'path': str(executed), 'sha256': args.patched_sha256},
            {'path': str(OVERLAY), 'sha256': record['overlay_entry_sha256']},
            {'path': str(Path(__file__).resolve()), 'sha256': record['runner_sha256']}]
        write(out / 'launch.json', record)
        with (out / 'stdout-stderr.log').open('xb') as log:
            process = subprocess.run(command, cwd=REPO, env=env, stdout=log, stderr=subprocess.STDOUT,
                creationflags=subprocess.CREATE_NO_WINDOW if os.name == 'nt' else 0)
        record['exit_code'] = process.returncode
        if overlay_proof.exists():
            actual = read(overlay_proof)
            record['overlay_execution'] = {'path': str(overlay_proof), 'sha256': sha(overlay_proof), 'record': actual}
            require(actual['original_filename'] == actual['module_filename'] == str(ORIGINAL)
                and actual['original_sha256'] == args.original_sha256
                and actual['evaluated_source'] == str(executed)
                and actual['evaluated_sha256'] == args.patched_sha256, 'Actual Node overlay source identity differs')
        else:
            raise ValueError('Node did not record its actual harness source identity')
        results = sorted(out.glob('supplier-*/result.json'))
        record['inner_result_candidates'] = [str(path) for path in results]
        require(len(results) == 1, 'Supplier browser did not produce exactly one actual inner result')
        inner = read(results[0])
        record['inner_result'] = {'path': str(results[0]), 'sha256': sha(results[0]),
            'passed': inner.get('passed'), 'revision': inner.get('build', {}).get('revision'),
            'binary_sha256': inner.get('build', {}).get('binary_sha256'),
            'failed_stage': inner.get('failed_stage'), 'failure': inner.get('failure')}
        require(process.returncode == 0 and inner.get('passed') is True,
            'Actual supplier browser did not complete all assertions successfully')
        require(inner.get('build', {}).get('revision') == args.pin
            and inner['build']['binary_sha256'] == build['binary_sha256'], 'Actual served build differs from the qualified game')
        require(inner['fixture']['sha256'] == phases[0]['sha256']
            and inner['preparation']['sha256'] == phases[1]['sha256'], 'Actual browser input differs from qualified exporter phases')
        record['browser_acceptance_passed'] = True
    except BaseException as error:
        record['runner_error'] = str(error)
    finally:
        record['finished_utc'] = stamp()
        record['wall_seconds'] = time.monotonic() - started
        try:
            after = source()
            record['source_after'] = after
            preservation = [{'path': item['path'], 'expected_sha256': item['sha256'],
                'actual_sha256': sha(item['path'])} for item in watched]
            record['preservation'] = preservation
            record['integrity_passed'] = (bool(watched) and after['revision'] == args.pin
                and not after['status_porcelain']
                and all(item['actual_sha256'] == item['expected_sha256'] for item in preservation))
            log = out / 'stdout-stderr.log'
            if log.exists():
                record['log_sha256'] = sha(log)
        except BaseException as error:
            record['integrity_passed'] = False
            record['integrity_error'] = str(error)
        record['passed'] = (record.get('browser_acceptance_passed') is True
            and record.get('integrity_passed') is True and 'runner_error' not in record)
        write(out / 'runner-result.json', record)
        print(json.dumps({'passed': record['passed'], 'evidence': str(out),
            'exit_code': record['exit_code'], 'inner_result': record.get('inner_result', {}).get('path'),
            'error': record.get('runner_error')}, indent=2), flush=True)
    return 0 if record['passed'] else 1


if __name__ == '__main__':
    raise SystemExit(main())
