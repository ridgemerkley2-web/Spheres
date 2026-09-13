#!/usr/bin/env python3
"""Measure the declared S08 supplier case once, without building or launching a server.

Usage: python run-s08-feature-performance.py PIN TEST_BINARY SHA256 PURCHASED_SAVE
       EXPORT_RUNNER_RESULT UNIQUE_LABEL

The purchased save must be listed in a successful same-candidate genuine exporter
record. Original files remain untouched; a new evidence folder owns exact copies.
Failed runs are retained. Run only after other builds and simulations finish.
"""
import argparse
import ctypes
from ctypes import wintypes
import datetime
import hashlib
import json
import math
import os
from pathlib import Path
import re
import shutil
import subprocess
import time

BASE = Path(__file__).resolve().parent
REPO = BASE / 'integration'
PLAN = BASE / 'evidence/S08-performance-plan.json'
TEST = 'performance::s08_supplier_import_profile'
DECLARED = {
    'simulation_p95_limit_ms': 300,
    'whole_turn_p95_limit_ms': 400,
    'whole_turn_max_limit_ms': 750,
    'equipment_board_read_and_serialization_p95_limit_ms': 300,
    'purchase_quote_p95_limit_ms': 250,
}

def stamp(): return datetime.datetime.now(datetime.timezone.utc).isoformat()
def sha(path):
    with Path(path).open('rb') as handle: return hashlib.file_digest(handle, 'sha256').hexdigest()
def read(path): return json.loads(Path(path).read_text(encoding='utf-8-sig'))
def write(path, value):
    with Path(path).open('x', encoding='utf-8') as handle:
        json.dump(value, handle, indent=2); handle.write('\n')
def git(*args):
    return subprocess.check_output(['git', '-c', 'core.longpaths=true', *args], cwd=REPO, text=True).strip()
def source(): return {'captured_utc': stamp(), 'revision': git('rev-parse', 'HEAD'), 'status_porcelain': git('status', '--porcelain')}

class ProcessMemory:
    """Windows counters for this child PID only; no process-tree aggregation."""
    class Counters(ctypes.Structure):
        _fields_ = [('cb', wintypes.DWORD), ('PageFaultCount', wintypes.DWORD)] + [
            (name, ctypes.c_size_t) for name in ('PeakWorkingSetSize', 'WorkingSetSize',
            'QuotaPeakPagedPoolUsage', 'QuotaPagedPoolUsage', 'QuotaPeakNonPagedPoolUsage',
            'QuotaNonPagedPoolUsage', 'PagefileUsage', 'PeakPagefileUsage', 'PrivateUsage')]

    def __init__(self, pid):
        self.kernel = ctypes.WinDLL('kernel32', use_last_error=True)
        self.psapi = ctypes.WinDLL('psapi', use_last_error=True)
        self.kernel.OpenProcess.argtypes = [wintypes.DWORD, wintypes.BOOL, wintypes.DWORD]
        self.kernel.OpenProcess.restype = wintypes.HANDLE
        self.kernel.CloseHandle.argtypes = [wintypes.HANDLE]
        self.kernel.CloseHandle.restype = wintypes.BOOL
        self.psapi.GetProcessMemoryInfo.argtypes = [wintypes.HANDLE, ctypes.POINTER(self.Counters), wintypes.DWORD]
        self.psapi.GetProcessMemoryInfo.restype = wintypes.BOOL
        self.handle = self.kernel.OpenProcess(0x0400 | 0x0010, False, pid)
        if not self.handle: raise ctypes.WinError(ctypes.get_last_error())

    def sample(self):
        counters = self.Counters(); counters.cb = ctypes.sizeof(counters)
        if not self.psapi.GetProcessMemoryInfo(self.handle, ctypes.byref(counters), counters.cb):
            return None
        return {'private_bytes': counters.PrivateUsage, 'working_set_bytes': counters.WorkingSetSize,
                'os_peak_working_set_bytes': counters.PeakWorkingSetSize}

    def close(self):
        if self.handle: self.kernel.CloseHandle(self.handle); self.handle = None

def metric(profile, key, field):
    row = profile.get(key)
    assert isinstance(row, dict), 'Missing measured ' + key
    assert isinstance(row.get('samples'), int) and row['samples'] > 0, 'No samples for ' + key
    value = row.get(field)
    assert isinstance(value, (int, float)) and not isinstance(value, bool) and math.isfinite(value) and value >= 0, 'Invalid ' + key + '.' + field
    return value

def main():
    parser = argparse.ArgumentParser(description=__doc__)
    for name in ('candidate', 'test_binary', 'binary_sha256', 'purchased_save', 'export_runner_result', 'label'):
        parser.add_argument(name)
    args = parser.parse_args()
    assert os.name == 'nt', 'This runner measures Windows private memory; use it on Windows'
    assert re.fullmatch('[0-9a-f]{40}', args.candidate)
    assert re.fullmatch('[0-9a-f]{64}', args.binary_sha256)
    assert re.fullmatch('[A-Za-z0-9_-]+', args.label)
    binary = Path(args.test_binary).resolve(); original = Path(args.purchased_save).resolve()
    proof_path = Path(args.export_runner_result).resolve()
    before = source(); plan = read(PLAN); proof = read(proof_path)
    assert before['revision'] == args.candidate and not before['status_porcelain'], 'Requires the exact clean candidate'
    assert sha(binary) == args.binary_sha256, 'Wrong already-built test binary'
    bars = plan['s08_feature_case']
    assert {key: bars.get(key) for key in DECLARED} == DECLARED, 'Declared S08 limits changed'
    assert proof.get('passed') is True and proof.get('integrity_passed') is True, 'Genuine export was not qualified'
    assert proof['candidate'] == args.candidate and proof['test_binary_sha256_before'] == args.binary_sha256, 'Export provenance differs from measured candidate/binary'
    assert proof.get('test_binary_sha256_after') == args.binary_sha256
    assert proof['native_outcome']['no_synthetic_endowments'] is True
    assert original.name.endswith('-purchased.campaign.json'), 'Use the genuine paid, still-in-transit checkpoint'
    assert os.path.samefile(original, Path(proof['export_directory']) / original.name), 'Input is not the original qualified export archive'
    input_hash = sha(original)
    phase = next((row for row in proof['archive_phases'] if row['file'] == original.name), None)
    assert phase and phase['sha256'] == input_hash, 'Purchased bytes do not match the genuine export record'
    out = BASE / 'evidence' / ('S08-feature-performance-' + args.label)
    out.mkdir(exist_ok=False)
    copied_binary = out / binary.name; copied_input = out / 'purchased.campaign.json'
    shutil.copyfile(binary, copied_binary); shutil.copyfile(original, copied_input)
    assert sha(copied_binary) == args.binary_sha256 and sha(copied_input) == input_hash
    shutil.copyfile(PLAN, out / 'declared-plan.json')
    output = out / 'profile.json'
    env = os.environ.copy(); removed = []
    for key in list(env):
        if key.startswith('SPHERES_PROFILE') or key.startswith('SPHERES_S08_') or re.fullmatch(r'SPHERES_S0[234]_UI_FIXTURE', key):
            removed.append(key); env.pop(key)
    env.update(SPHERES_S08_PERFORMANCE_INPUT=str(copied_input), SPHERES_S08_PERFORMANCE_OUT=str(output))
    command = [str(copied_binary), TEST, '--ignored', '--exact', '--nocapture', '--test-threads=1']
    record = {'format': 'spheres-s08-feature-performance-qualification', 'version': 1,
        'candidate': args.candidate, 'source_before': before, 'started_utc': stamp(),
        'test_binary': str(binary), 'executed_copy': str(copied_binary), 'binary_sha256_before': args.binary_sha256,
        'input': str(original), 'input_copy': str(copied_input), 'input_sha256_before': input_hash,
        'export_proof': str(proof_path), 'export_proof_sha256_before': sha(proof_path),
        'plan': str(PLAN), 'plan_sha256_before': sha(PLAN), 'declared_limits': bars,
        'runner': str(Path(__file__).resolve()), 'runner_sha256': sha(__file__),
        'command': command, 'working_directory': str(REPO),
        'environment': {key: env[key] for key in ('SPHERES_S08_PERFORMANCE_INPUT', 'SPHERES_S08_PERFORMANCE_OUT')},
        'sanitized_environment_keys': sorted(removed), 'exit_code': None, 'passed': False,
        'memory': {'requested_interval_ms': 100, 'samples': 0, 'failed_reads': 0,
            'note': 'Child PID only. Sampled maxima can miss short peaks. OS peak working set is a separate Windows counter, not private-memory peak. No feature-case memory acceptance bar was declared; these observations do not imply throughput or browser performance.'}}
    write(out / 'launch.json', record)
    started = time.monotonic(); process = None; monitor = None; samples = []
    try:
        with (out / 'stdout.log').open('xb') as stdout, (out / 'stderr.log').open('xb') as stderr, (out / 'memory.jsonl').open('x', encoding='utf-8') as memory_log:
            process = subprocess.Popen(command, cwd=REPO, env=env, stdout=stdout, stderr=stderr, creationflags=subprocess.CREATE_NO_WINDOW)
            record['pid'] = process.pid; monitor = ProcessMemory(process.pid); next_sample = time.monotonic()
            while True:
                sample = monitor.sample()
                if sample:
                    sample['elapsed_seconds'] = time.monotonic() - started
                    samples.append(sample); memory_log.write(json.dumps(sample) + '\n'); memory_log.flush()
                else: record['memory']['failed_reads'] += 1
                if process.poll() is not None: break
                next_sample += 0.1
                time.sleep(max(0, next_sample - time.monotonic()))
            record['exit_code'] = process.wait()
        text = (out / 'stdout.log').read_text(encoding='utf-8', errors='replace')
        totals = [tuple(map(int, row)) for row in re.findall(r'test result: (?:ok|FAILED)\. (\d+) passed; (\d+) failed; (\d+) ignored;', text)]
        record['test_summaries'] = totals
        assert record['exit_code'] == 0 and totals == [(1, 0, 0)], 'The requested ignored profile did not pass exactly once'
        profile = read(output)
        assert profile['revision'] == git('rev-parse', '--short=12', args.candidate), 'Wrong embedded candidate'
        assert os.path.samefile(profile['input'], copied_input) and profile['source_unchanged'] is True
        for key in ('simulation_and_history_recording', 'whole_server_turn', 'equipment_market_read_and_serialization'):
            assert profile[key]['samples'] == 31, 'Expected 31 consecutive measured days'
        assert len(profile['sample_activity']) == 31
        assert any(row['delivered_day'] is None and row['cancelled_day'] is None for row in profile['initial_imports']), 'No paid import in transit'
        checks = []
        for key, field, bar in [
            ('simulation_and_history_recording', 'p95_ms', 'simulation_p95_limit_ms'),
            ('whole_server_turn', 'p95_ms', 'whole_turn_p95_limit_ms'),
            ('whole_server_turn', 'max_ms', 'whole_turn_max_limit_ms'),
            ('equipment_market_read_and_serialization', 'p95_ms', 'equipment_board_read_and_serialization_p95_limit_ms'),
            ('purchase_quote', 'p95_ms', 'purchase_quote_p95_limit_ms')]:
            value = metric(profile, key, field)
            checks.append({'metric': key + '.' + field, 'measured_ms': value, 'limit_ms': bars[bar], 'passed': value <= bars[bar]})
        record['acceptance_checks'] = checks
        record['native_and_thresholds_passed'] = all(row['passed'] for row in checks)
        assert samples, 'No process-memory observations were collected'
    except BaseException as error:
        record['runner_error'] = str(error)
        if process and process.poll() is None:
            process.terminate(); process.wait(); record['exit_code'] = process.returncode
            record['terminated_by_runner'] = True
    finally:
        if monitor: monitor.close()
        record.update(finished_utc=stamp(), wall_seconds=time.monotonic() - started)
        record['memory']['samples'] = len(samples)
        for key in ('private_bytes', 'working_set_bytes', 'os_peak_working_set_bytes'):
            record['memory']['maximum_' + key] = max((row[key] for row in samples), default=None)
        record['memory']['maximum_observed_interval_ms'] = max(((b['elapsed_seconds'] - a['elapsed_seconds']) * 1000 for a, b in zip(samples, samples[1:])), default=None)
        try:
            record['source_after'] = source()
            record['binary_sha256_after'] = sha(binary); record['executed_copy_sha256_after'] = sha(copied_binary)
            record['input_sha256_after'] = sha(original); record['input_copy_sha256_after'] = sha(copied_input)
            record['plan_sha256_after'] = sha(PLAN); record['export_proof_sha256_after'] = sha(proof_path)
            record['integrity_passed'] = (record['source_after']['revision'] == args.candidate and not record['source_after']['status_porcelain']
                and record['binary_sha256_after'] == record['executed_copy_sha256_after'] == args.binary_sha256
                and record['input_sha256_after'] == record['input_copy_sha256_after'] == input_hash
                and record['plan_sha256_after'] == record['plan_sha256_before']
                and record['export_proof_sha256_after'] == record['export_proof_sha256_before'])
        except BaseException as error:
            record['integrity_passed'] = False; record['integrity_error'] = str(error)
        record['passed'] = record.get('native_and_thresholds_passed', False) and record['integrity_passed'] and 'runner_error' not in record
        record['files'] = [{'path': p.relative_to(out).as_posix(), 'sha256': sha(p), 'bytes': p.stat().st_size} for p in sorted(out.rglob('*')) if p.is_file()]
        write(out / 'runner-result.json', record)
        print(json.dumps({'passed': record['passed'], 'evidence': str(out), 'exit_code': record['exit_code'], 'error': record.get('runner_error')}, indent=2))
    return 0 if record['passed'] else 1

if __name__ == '__main__': raise SystemExit(main())
