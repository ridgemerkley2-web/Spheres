"""Create-only configuration for this exact already-running launcher; never starts campaigns."""
from pathlib import Path
import datetime as dt
import os
import subprocess
import sys

import dependent_verify as d

ROOT = Path(__file__).resolve().parent
VALIDATION = ROOT.parent / 'lazy-digest-validation-20260930'
FROZEN = VALIDATION / 'frozen'
RETAINED = ROOT.parent / 'full-matrix-local-20260930-01'
EXPECTED_COMMAND = ('"C:\\Users\\ridge\\AppData\\Local\\Microsoft\\WindowsApps\\python.exe" '
                    '-B -X utf8 D:/spheres-offload/codex-next-20260928/lazy-digest-validation-20260930/run_full.py')


def main():
    d.require(not (ROOT / 'config.json').exists(), 'Configuration already exists')
    process = d.WindowsProcess(9040)
    try:
        d.require(not process.poll(), 'Expected launcher is no longer running')
        metadata = d.process_metadata(9040)
        d.require(metadata['CommandLine'] == EXPECTED_COMMAND, 'Wrong launcher command line')
        creation = dt.datetime(1601, 1, 1, tzinfo=dt.timezone.utc) + dt.timedelta(microseconds=process.creation // 10)
        d.require(creation == dt.datetime(2026, 9, 30, 9, 3, 11, 78360, tzinfo=dt.timezone.utc),
                  'Not the specified launcher creation time')
        manifest = d.read_json(FROZEN / 'manifest.json')
        plan = d.read_json(FROZEN / 'plan.json')
        d.validate_plan(plan)
        d.require(manifest['revision'] == d.REVISION and manifest['cells'] == d.CELLS, 'Wrong frozen source')
        d.require(set(manifest['files']) == {'native-test', 'plan.json', 'stability_matrix.py', 'distributed_stability.py'},
                  'Unexpected frozen file inventory')
        pins = []
        for name, declaration in manifest['files'].items():
            observed = d.pin(FROZEN / name)
            d.require({k: observed[k] for k in ('bytes', 'sha256')} == declaration, 'Frozen manifest mismatch: ' + name)
            pins.append(observed)
        binary = d.pin(VALIDATION / 'spheres-web-test.exe')
        d.require({k: binary[k] for k in ('bytes', 'sha256')} == manifest['files']['native-test'], 'Launcher binary mismatch')
        pins.append(binary)
        helper = d.pin(FROZEN / 'stability_matrix.py')
        launch = d.read_json(VALIDATION / 'full-launch.json')
        # Store-package aliases cannot be resolved by pathlib; pin and invoke the actual current image.
        python_image = d.process_metadata(os.getpid())['ExecutablePath']
        for path in [FROZEN / 'manifest.json', VALIDATION / 'run_full.py', VALIDATION / 'full-launch.json',
                     RETAINED / 'freeze.json', RETAINED / 'plan.json', RETAINED / 'stability_matrix.py',
                     ROOT / 'dependent_verify.py', ROOT / 'test_dependent_verify.py', ROOT / 'prepare.py',
                     Path(python_image)]:
            pins.append(d.pin(path))
        snapshots = ROOT / 'captured'
        snapshots.mkdir()
        for path in [FROZEN / 'manifest.json', VALIDATION / 'full-launch.json', RETAINED / 'freeze.json']:
            with (snapshots / path.name).open('xb') as stream:
                stream.write(path.read_bytes())
            pins.append(d.pin(snapshots / path.name))
        cfg = {'format': 'spheres-dependent-verification-config/v1', 'created_utc': d.utc_now(),
               'output': str(ROOT), 'parent_pid': 9040, 'parent_creation_filetime': process.creation,
               'parent_creation_utc': creation.isoformat(), 'parent_command_line': metadata['CommandLine'],
               'parent_executable': metadata['ExecutablePath'], 'python': python_image,
               'launcher_started_utc': launch['started_utc'], 'launcher_args': launch['args'],
               'manifest': str(FROZEN / 'manifest.json'), 'plan': str(FROZEN / 'plan.json'),
               'launch': str(VALIDATION / 'full-launch.json'), 'completion': str(VALIDATION / 'full-exit.json'),
               'retained': str(RETAINED), 'binary': binary, 'helper': helper, 'pins': pins,
               'qualification': False, 's25_complete': False}
        d.verify_dependencies(cfg)
        with (ROOT / 'guard-tests.stdout.log').open('xb') as stdout, (ROOT / 'guard-tests.stderr.log').open('xb') as stderr:
            check = subprocess.run([sys.executable, '-B', '-X', 'utf8', '-m', 'unittest', '-v', 'test_dependent_verify'],
                                   cwd=ROOT, stdout=stdout, stderr=stderr, creationflags=subprocess.CREATE_NO_WINDOW)
        d.write_new(ROOT / 'guard-tests-result.json', {'exit_code': check.returncode, 'finished_utc': d.utc_now(),
                    'scope': 'Synthetic guards, plus owned short Python sleeper for Windows process identity/wait. No campaign execution.',
                    'stdout': d.pin(ROOT / 'guard-tests.stdout.log'), 'stderr': d.pin(ROOT / 'guard-tests.stderr.log')})
        d.require(check.returncode == 0, 'Guard tests failed')
        d.require(not process.poll(), 'Launcher exited during preparation; dependent job not armed')
        d.write_new(ROOT / 'config.json', cfg)
        d.write_new(ROOT / 'prepared.json', {'prepared_utc': d.utc_now(), 'config': d.pin(ROOT / 'config.json'),
                    'parent_metadata': metadata, 'guard_tests_passed': True, 'launched': False,
                    'qualification': False, 's25_complete': False})
        print(d.pin(ROOT / 'config.json'))
    finally:
        process.close()


if __name__ == '__main__':
    main()
