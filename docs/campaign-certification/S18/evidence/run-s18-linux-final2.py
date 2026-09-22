import datetime
import hashlib
import json
import os
import pathlib
import re
import subprocess
import sys
import time

base = pathlib.Path(__file__).resolve().parent
repo = base / 'integration'
pin = sys.argv[1]
assert re.fullmatch('[0-9a-f]{40}', pin)
out = base / 'evidence/S18-final2-linux'
out.mkdir(exist_ok=False)
env = dict(os.environ)
env['SPHERES_AUDIT_PYTHON'] = '/usr/bin/python3'
env.update(GIT_DIR='/mnt/c/Users/ridge/Spheres/.git/worktrees/integration',
           GIT_WORK_TREE=str(repo), GIT_OPTIONAL_LOCKS='0', CARGO_BUILD_JOBS='4',
           GIT_CONFIG_COUNT='1', GIT_CONFIG_KEY_0='core.autocrlf', GIT_CONFIG_VALUE_0='true')
git_exe = '/mnt/c/Program Files/Git/cmd/git.exe'
git_repo = 'C:/Users/ridge/Documents/Codex/2026-09-05/pick-up-the-spheres-game-on/work/campaign-certification/integration'
git_env = {k: v for k, v in env.items() if k not in ['GIT_DIR', 'GIT_WORK_TREE']}
git_env['GIT_OPTIONAL_LOCKS'] = '0'


def sha(path):
    result = hashlib.sha256()
    with pathlib.Path(path).open('rb') as stream:
        for chunk in iter(lambda: stream.read(1024 * 1024), b''):
            result.update(chunk)
    return result.hexdigest()


def stamp():
    return datetime.datetime.now(datetime.timezone.utc).isoformat()


def git(*args):
    return subprocess.check_output([git_exe, '-C', git_repo, '-c', 'core.longpaths=true', *args],
                                   env=git_env, text=True).strip()


def snapshot():
    value = {'head': git('rev-parse', 'HEAD'), 'status': git('status', '--porcelain')}
    assert value == {'head': pin, 'status': ''}, value
    return value


record = {
    'format': 'spheres-s18-linux/v1', 'revision': pin, 'started_utc': stamp(), 'status': 'running',
    'scope': 'S18 corrected full Node suite on Linux; native web is qualified at the unchanged runtime source 902b820. No simulation code changes; S17 retains its separate full simulation/integration evidence. No external old-save, elapsed campaign, historical-content or gate-completion claim.',
    'runner_sha256': sha(__file__),
    'derived_from': {'path': str(base / 'run-s10-final-linux.py'), 'sha256': sha(base / 'run-s10-final-linux.py')},
    'source_verification': 'Windows git.exe on the same complete NTFS checkout; test and compile processes use the existing Linux toolchain and target directory.',
    'environment': {k: env[k] for k in ['GIT_DIR', 'GIT_WORK_TREE', 'GIT_OPTIONAL_LOCKS', 'CARGO_BUILD_JOBS',
                    'GIT_CONFIG_COUNT', 'GIT_CONFIG_KEY_0', 'GIT_CONFIG_VALUE_0', 'CARGO_HOME', 'RUSTUP_HOME',
                    'CARGO_TARGET_DIR', 'CARGO_TARGET_X86_64_UNKNOWN_LINUX_GNU_LINKER', 'CC', 'TMPDIR']},
    'checks': []
}


def persist():
    (out / 'result.json').write_text(json.dumps(record, indent=2) + '\n', encoding='utf8')


def totals_of(name, text):
    if name == 'node':
        # Node uses either the spec reporter's glyph or TAP's # summary prefix.
        totals = {}
        for label, aliases in {'tests': ['tests'], 'passed': ['pass'], 'failed': ['fail'], 'skipped': ['skipped']}.items():
            matches = []
            for alias in aliases:
                matches.extend(re.findall(r'(?:\u2139|#)\s+' + alias + r'\s+(\d+)', text))
            if matches:
                totals[label] = int(matches[-1])
        return totals
    values = [tuple(map(int, match)) for match in re.findall(
        r'test result: (?:ok|FAILED)\. (\d+) passed; (\d+) failed; (\d+) ignored;', text)]
    return {'passed': sum(v[0] for v in values), 'failed': sum(v[1] for v in values),
            'ignored': sum(v[2] for v in values), 'completed_targets': len(values)}


try:
    record['source_before'] = snapshot()
    versions = []
    for command in [['uname', '-a'], ['rustc', '-Vv'], ['cargo', '-Vv'], ['node', '-v'], ['npm', '-v']]:
        result = subprocess.run(command, cwd=repo, env=env, text=True, stdout=subprocess.PIPE, stderr=subprocess.STDOUT)
        versions.append({'command': command, 'exit_code': result.returncode, 'output': result.stdout})
        result.check_returncode()
    toolchain = out / 'toolchain.json'
    toolchain.write_text(json.dumps(versions, indent=2) + '\n', encoding='utf8')
    record['toolchain_sha256'] = sha(toolchain)
    persist()
    commands = [

        ('node', ['node', 'tools/ui/run-unit.cjs']),
    ]
    for name, command in commands:
        print('START ' + name, flush=True)
        before = snapshot()
        log = out / (name + '.log')
        began = stamp()
        timer = time.perf_counter()
        with log.open('xb') as stream:
            result = subprocess.run(command, cwd=repo, env=env, stdout=stream, stderr=subprocess.STDOUT)
        text = log.read_text(encoding='utf8', errors='replace')
        totals = totals_of(name, text)
        parsed = (totals.get('tests', 0) > 0 and 'passed' in totals and 'failed' in totals and 'skipped' in totals) if name == 'node' else totals['completed_targets'] > 0
        check = {'name': name, 'command': command, 'started_utc': began, 'finished_utc': stamp(),
                 'elapsed_seconds': time.perf_counter() - timer, 'exit_code': result.returncode,
                 'log': log.name, 'log_sha256': sha(log), 'totals': totals,
                 'source_before': before, 'source_after': snapshot(),
                 'passed': result.returncode == 0 and parsed and totals.get('failed') == 0}
        if name == 'web':
            matches = list(dict.fromkeys(re.findall(r'Running unittests src/main\.rs \(([^\r\n)]*/spheres_web-[0-9a-f]+)\)', text)))
            if len(matches) == 1:
                binary = pathlib.Path(matches[0])
                check['test_binary'] = {'path': str(binary), 'bytes': binary.stat().st_size, 'sha256': sha(binary)}
            else:
                check['test_binary_unresolved'] = matches
        record['checks'].append(check)
        persist()
        print('FINISH ' + name + ' exit=' + str(result.returncode) + ' totals=' + json.dumps(totals), flush=True)
        if not check['passed']:
            print(text[-5000:], flush=True)
            break
    record['source_after'] = snapshot()
    record['runner_sha256_after'] = sha(__file__)
    record['passed'] = (len(record['checks']) == len(commands) and all(c['passed'] for c in record['checks'])
                        and record['runner_sha256_after'] == record['runner_sha256'])
    record['status'] = 'passed' if record['passed'] else 'failed'
except BaseException as error:
    record['passed'] = False
    record['status'] = 'runner_error'
    record['error'] = str(error)[:5000]
    raise
finally:
    record['finished_utc'] = stamp()
    persist()
    print(json.dumps(record, indent=2), flush=True)

sys.exit(0 if record['passed'] else 1)






