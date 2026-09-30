"""One-off retained-evidence verification. Never launches native campaigns."""
from pathlib import Path
import argparse
import ctypes
from ctypes import wintypes
import datetime as dt
import hashlib
import json
import math
import os
import subprocess
import sys
import time

REVISION = '68ba0622ec709b78617aadd1f9198d18f532bb32'
COUNTRIES = ('France', 'Japan', 'India', 'Brazil', 'SouthAfrica', 'Tonga', 'SaudiArabia', 'USSR')
CELLS = [dict(id=f'{country.lower()}-{seed}', country=country, seed=seed, through='2035-12-31')
         for country in COUNTRIES for seed in (1990, 7, 42)]


def require(condition, message):
    if not condition:
        raise ValueError(message)


def utc_now():
    return dt.datetime.now(dt.timezone.utc).isoformat()


def unique_object(items):
    value = {}
    for key, item in items:
        require(key not in value, 'Duplicate JSON key: ' + key)
        value[key] = item
    return value


def read_json(path):
    return json.loads(Path(path).read_text(encoding='utf-8'), object_pairs_hook=unique_object,
                      parse_constant=lambda value: (_ for _ in ()).throw(ValueError('Nonfinite JSON: ' + value)))


def pin(path):
    path = Path(path).resolve(strict=True)
    require(path.is_file() and not path.is_symlink(), 'Pin must be a regular file')
    digest = hashlib.sha256()
    size = 0
    with path.open('rb') as stream:
        for block in iter(lambda: stream.read(1024 * 1024), b''):
            digest.update(block)
            size += len(block)
    return {'path': str(path), 'bytes': size, 'sha256': digest.hexdigest()}


def verify_pin(expected):
    actual = pin(expected['path'])
    require(actual == expected, 'Frozen file changed: ' + expected['path'])
    return actual


def write_new(path, value):
    with Path(path).open('x', encoding='utf-8', newline='\n') as stream:
        json.dump(value, stream, indent=2, allow_nan=False)
        stream.write('\n')
        stream.flush()
        os.fsync(stream.fileno())


def logical(path):
    return os.path.normcase(os.path.abspath(path))


def validate_plan(plan):
    require(plan.get('format') == 'spheres-stability-plan/v1' and plan.get('id') == 's25-full-v1'
            and plan.get('scope') == 'full' and plan.get('cells') == CELLS, 'Not the exact full 24-cell plan')


def validate_coverage(value):
    require(type(value) is dict, 'Missing coverage')
    for key in ('declared', 'attempted', 'passed'):
        require(type(value.get(key)) is int and value[key] == 24, 'Coverage is not 24: ' + key)
    for key in ('failed', 'missing_requested', 'missing_full_cases'):
        require(value.get(key) == [], 'Incomplete coverage: ' + key)
    require(value.get('full_matrix_passed') is True and value.get('requested_plan_passed') is True
            and value.get('pilot_passed') is False, 'Not a complete passing full matrix')
    require(value.get('qualification') is False and value.get('s25_complete') is False
            and value.get('controlled_ussr_to_russia_case_passed') is False, 'Unexpected qualification claim')


def validate_cells(rows):
    require(type(rows) is list and [r.get('id') for r in rows] == [c['id'] for c in CELLS],
            'Missing, duplicate, reordered, or unknown cells')
    require(all(row.get('passed') is True for row in rows), 'A cell did not pass')


def validate_exit(record, cfg, process_exit):
    require(type(process_exit) is int and type(record.get('exit_code')) is int
            and process_exit == record['exit_code'], 'Launcher and child exit records disagree')
    require(logical(record.get('result', '')) == logical(Path(cfg['retained']) / 'result.json'), 'Wrong completion result path')
    elapsed = record.get('elapsed_seconds')
    require(type(elapsed) in (int, float) and math.isfinite(elapsed) and elapsed > 0, 'Invalid elapsed completion time')
    finished = dt.datetime.fromisoformat(record['finished_utc'])
    started = dt.datetime.fromisoformat(cfg['launcher_started_utc'])
    require(finished.tzinfo is not None and started <= finished <= dt.datetime.now(dt.timezone.utc), 'Invalid completion timestamp')


def validate_final(proof, verified, cfg, verifier_exit):
    require(type(verifier_exit) is int and verifier_exit == 0, 'Frozen retained verifier did not exit zero')
    require(proof.get('format') == 'spheres-stability-matrix-result/v1' and proof.get('revision') == REVISION
            and proof.get('scope') == 'full' and proof.get('plan_id') == 's25-full-v1', 'Wrong matrix identity')
    require(proof.get('passed') is True and proof.get('binary_unchanged') is True
            and proof.get('interrupted') is False and proof.get('resource_halted') is False,
            'Matrix failed, changed, interrupted, or resource-halted')
    require(proof.get('jobs') == 4, 'Changed matrix concurrency')
    for report in (proof, verified):
        require(not any(key in report for key in ('only_cell', 'selected_cell_passed', 'batch_sha256')), 'Shard cannot pass')
        require(report.get('qualification') is False and report.get('s25_complete') is False, 'Unexpected qualification claim')
        validate_coverage(report.get('coverage'))
        validate_cells(report.get('cells'))
    require(verified.get('format') == 'spheres-stability-retained-verification/v1'
            and verified.get('integrity_verified') is True and verified.get('passed') is True
            and verified.get('candidate_revision') == REVISION, 'Integrity or candidate mismatch')
    require(verified.get('native_reexecuted') is False and verified.get('binary_reexecuted_or_rebuilt') is False,
            'Verification unexpectedly executed native code')
    require(logical(verified.get('source', '')) == logical(cfg['retained']), 'Verifier checked another directory')
    require(verified.get('frozen_harness_sha256') == cfg['helper']['sha256']
            and verified.get('verifier') == cfg['helper'], 'Wrong retained verifier')
    require(verified['coverage'] == proof['coverage'], 'Verifier and matrix coverage disagree')
    require(type(verified.get('archives_verified')) is int and verified['archives_verified'] >= 96,
            'Fewer than 24 full final/sandbox archive pairs verified')
    require(all(type(row.get('archives_verified')) is int and row['archives_verified'] >= 4
                for row in verified['cells']), 'Missing final/sandbox cell archives')


class WindowsProcess:
    """Read-only handle retains exact process identity even if its PID is reused."""
    def __init__(self, pid, expected_creation=None):
        require(os.name == 'nt', 'This one-off dependent job requires Windows')
        self.api = ctypes.WinDLL('kernel32', use_last_error=True)
        self.api.OpenProcess.argtypes = [wintypes.DWORD, wintypes.BOOL, wintypes.DWORD]
        self.api.OpenProcess.restype = wintypes.HANDLE
        self.api.GetProcessTimes.argtypes = [wintypes.HANDLE] + [ctypes.POINTER(wintypes.FILETIME)] * 4
        self.api.GetProcessTimes.restype = wintypes.BOOL
        self.api.WaitForSingleObject.argtypes = [wintypes.HANDLE, wintypes.DWORD]
        self.api.WaitForSingleObject.restype = wintypes.DWORD
        self.api.GetExitCodeProcess.argtypes = [wintypes.HANDLE, ctypes.POINTER(wintypes.DWORD)]
        self.api.GetExitCodeProcess.restype = wintypes.BOOL
        self.api.CloseHandle.argtypes = [wintypes.HANDLE]
        self.api.CloseHandle.restype = wintypes.BOOL
        self.handle = self.api.OpenProcess(0x00100000 | 0x1000, False, pid)
        if not self.handle:
            raise ctypes.WinError(ctypes.get_last_error())
        times = [wintypes.FILETIME() for _ in range(4)]
        try:
            if not self.api.GetProcessTimes(self.handle, *(ctypes.byref(item) for item in times)):
                raise ctypes.WinError(ctypes.get_last_error())
            self.creation = (times[0].dwHighDateTime << 32) | times[0].dwLowDateTime
            require(expected_creation is None or self.creation == expected_creation, 'PID was reused / creation time mismatch')
        except BaseException:
            self.close()
            raise

    def poll(self, milliseconds=0):
        status = self.api.WaitForSingleObject(self.handle, milliseconds)
        require(status in (0, 258), 'Process wait failed')
        return status == 0

    def exit_code(self):
        require(self.poll(), 'Process has not exited')
        value = wintypes.DWORD()
        if not self.api.GetExitCodeProcess(self.handle, ctypes.byref(value)):
            raise ctypes.WinError(ctypes.get_last_error())
        return value.value

    def close(self):
        if self.handle:
            self.api.CloseHandle(self.handle)
            self.handle = None


def process_metadata(pid):
    require(type(pid) is int and pid > 0, 'Invalid process ID')
    command = (f"Get-CimInstance Win32_Process -Filter 'ProcessId = {pid}' | "
               "Select-Object ProcessId,ParentProcessId,CreationDate,ExecutablePath,CommandLine | ConvertTo-Json -Compress")
    result = subprocess.run(['powershell.exe', '-NoProfile', '-NonInteractive', '-Command', command],
                            capture_output=True, text=True, check=True, creationflags=subprocess.CREATE_NO_WINDOW)
    require(result.stdout.strip(), 'Process is not available')
    return json.loads(result.stdout)


def verify_dependencies(cfg):
    for entry in cfg['pins']:
        verify_pin(entry)
    manifest = read_json(cfg['manifest'])
    require(manifest.get('revision') == REVISION and manifest.get('cells') == CELLS, 'Wrong frozen candidate/cells')
    validate_plan(read_json(cfg['plan']))
    freeze = read_json(Path(cfg['retained']) / 'freeze.json')
    require(freeze.get('candidate_revision') == REVISION and freeze.get('scope') == 'full'
            and freeze.get('binary') == cfg['binary'] and freeze.get('harness') == cfg['helper'], 'Live matrix freeze mismatch')
    require(not any(key in freeze for key in ('only_cell', 'batch_sha256')), 'Selected shard is not this matrix')
    launch = read_json(cfg['launch'])
    require(launch.get('revision') == REVISION and launch.get('complete_new_plan') is True
            and launch.get('earlier_cells_reused') is False and launch.get('qualification') is False,
            'Wrong launcher declaration')
    require(launch.get('args') == cfg['launcher_args'], 'Launch invocation changed')


def run_waiter(cfg_path, cfg_sha):
    require(pin(cfg_path)['sha256'] == cfg_sha, 'Dependent configuration changed')
    cfg = read_json(cfg_path)
    root = Path(cfg_path).resolve().parent
    require(logical(cfg['output']) == logical(root), 'Wrong dependent output directory')
    require(not (root / 'result.json').exists() and not (root / 'pending.json').exists(), 'Dependent job already started')
    process = None
    started = utc_now()
    with (root / 'events.jsonl').open('x', encoding='utf-8', newline='\n') as journal:
        def event(kind, **fields):
            journal.write(json.dumps({'utc': utc_now(), 'event': kind, **fields}, allow_nan=False) + '\n')
            journal.flush()
            os.fsync(journal.fileno())
        try:
            process = WindowsProcess(cfg['parent_pid'], cfg['parent_creation_filetime'])
            require(not process.poll(), 'Exact parent already exited before job armed')
            actual = process_metadata(cfg['parent_pid'])
            require(actual['CommandLine'] == cfg['parent_command_line']
                    and actual['ExecutablePath'] == cfg['parent_executable'], 'Wrong live parent command/image')
            verify_dependencies(cfg)
            pending = {'format': 'spheres-dependent-verification-pending/v1', 'status': 'waiting_for_exact_launcher',
                       'started_utc': started, 'worker_pid': os.getpid(), 'parent_pid': cfg['parent_pid'],
                       'parent_creation_filetime': process.creation, 'candidate_revision': REVISION,
                       'retained': cfg['retained'], 'config_sha256': cfg_sha, 'preflight_passed': False,
                       'qualification': False, 's25_complete': False}
            write_new(root / 'pending.json', pending)
            event('waiting_for_exact_launcher', **pending)
            while not process.poll(30000):
                pass
            parent_exit = process.exit_code()
            event('launcher_exited', exit_code=parent_exit)
            exit_path = Path(cfg['completion'])
            require(exit_path.is_file(), 'Launcher exited without full-exit.json; no verification started')
            completion = read_json(exit_path)
            # Abnormal exit must not trigger reads while orphaned native children could remain active.
            validate_exit(completion, cfg, parent_exit)
            write_new(root / 'dependency-completion.json', {'process_exit': parent_exit, 'record': completion, 'pin': pin(exit_path)})
            verify_dependencies(cfg)
            require(pin(cfg_path)['sha256'] == cfg_sha, 'Dependent configuration changed while waiting')
            result_path = Path(cfg['retained']) / 'result.json'
            proof_pin = pin(result_path)
            argv = [cfg['python'], '-B', '-X', 'utf8', cfg['helper']['path'], '--verify', cfg['retained']]
            require(argv[-2] == '--verify' and '--binary' not in argv, 'Verification invocation is not read-only')
            write_new(root / 'verification-invocation.json', {'args': argv, 'cwd': cfg['output'],
                      'started_utc': utc_now(), 'matrix_result_before': proof_pin})
            event('retained_verification_started', args=argv)
            began = time.monotonic()
            with (root / 'verify.stdout.json').open('xb') as stdout, (root / 'verify.stderr.log').open('xb') as stderr:
                child = subprocess.Popen(argv, cwd=root, stdout=stdout, stderr=stderr,
                                         creationflags=subprocess.CREATE_NO_WINDOW)
                event('retained_verifier_process', pid=child.pid)
                code = child.wait()
            write_new(root / 'verification-exit.json', {'exit_code': code, 'elapsed_seconds': time.monotonic() - began,
                       'finished_utc': utc_now(), 'stdout': pin(root / 'verify.stdout.json'), 'stderr': pin(root / 'verify.stderr.log')})
            verify_dependencies(cfg)
            verify_pin(proof_pin)
            require(pin(cfg_path)['sha256'] == cfg_sha, 'Dependent configuration changed during verification')
            verified = read_json(root / 'verify.stdout.json')
            proof = read_json(result_path)
            validate_final(proof, verified, cfg, code)
            require(parent_exit == 0, 'Matrix launcher did not exit zero')
            verdict = {'status': 'full_24_cell_preflight_passed', 'preflight_passed': True,
                       'coverage': verified['coverage'], 'archives_verified': verified['archives_verified'],
                       'files_verified': verified['files_verified'], 'verification_exit_code': code}
            result_code = 0
        except BaseException as error:
            verdict = {'status': 'dependent_verification_failed', 'preflight_passed': False,
                       'failure': f'{type(error).__name__}: {error}'}
            result_code = 1
        finally:
            if process is not None:
                process.close()
        verdict.update(format='spheres-dependent-verification-result/v1', started_utc=started, finished_utc=utc_now(),
                       candidate_revision=REVISION, config_sha256=cfg_sha, native_reexecuted=False,
                       qualification=False, s25_complete=False)
        write_new(root / 'result.json', verdict)
        event('dependent_job_finished', **verdict)
    return result_code


if __name__ == '__main__':
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('--config', type=Path, required=True)
    parser.add_argument('--config-sha256', required=True)
    args = parser.parse_args()
    raise SystemExit(run_waiter(args.config, args.config_sha256))
