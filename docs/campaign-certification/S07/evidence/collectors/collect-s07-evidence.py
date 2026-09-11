#!/usr/bin/env python3
"""Collect S07 evidence after qualification; never mark the session complete.

Default is a read-only dry run. --apply creates only S07/evidence, refuses an
existing destination, and writes inventory.json last. Original saves/binaries
remain external hash references. No builds, servers, tests, or save operations.
"""
from pathlib import Path, PurePosixPath
import argparse
import datetime
import hashlib
import json
import re
import subprocess
import sys

BASE = Path(__file__).resolve().parent
ALLOWED = {'.json', '.log', '.txt', '.png', '.html', '.exit', '.py', '.cjs', '.patch', '.ps1', '.md', '.csv'}


def require(condition, message):
    if not condition:
        raise ValueError(message)


def digest(data):
    return hashlib.sha256(data).hexdigest()


def read(path):
    return json.loads(Path(path).read_text(encoding='utf-8-sig'))


def file_hash(path):
    with Path(path).open('rb') as handle:
        return hashlib.file_digest(handle, 'sha256').hexdigest()


def git(repo, *args):
    return subprocess.check_output(['git', *args], cwd=repo, stderr=subprocess.PIPE)


def passed_proof(path, pin):
    result = read(path)
    require(result.get('exit_code') == 0, f'Unpassed qualification: {path}')
    require(result.get('revision_before') == pin == result.get('revision_after'), f'Revision mismatch: {path}')
    require(result.get('clean_before') is True and result.get('clean_after') is True, f'Dirty qualification source: {path}')
    require(result.get('finished_utc'), f'Incomplete qualification: {path}')
    return result


def rust_totals(path):
    rows = [tuple(map(int, match)) for match in re.findall(
        r'test result: (?:ok|FAILED)\. (\d+) passed; (\d+) failed; (\d+) ignored;',
        Path(path).read_text(encoding='utf-8', errors='replace'))]
    require(rows and sum(r[1] for r in rows) == 0, f'Missing or failed native summaries: {path}')
    return dict(passed=sum(r[0] for r in rows), failed=sum(r[1] for r in rows),
                ignored=sum(r[2] for r in rows), targets=len(rows))


def save_or_binary(path, data):
    """Conservative exclusion; API observations are not campaign archives."""
    if path.suffix.lower() not in ALLOWED:
        return True
    if any(part.lower() in {'saves', 'fixtures', 'profile-campaigns'} for part in path.parts):
        return True
    if path.name.lower() in {'save.json', 'save.json.bak'}:
        return True
    if path.suffix.lower() == '.json':
        try:
            value = json.loads(data)
        except (ValueError, UnicodeDecodeError):
            return False
        if isinstance(value, dict):
            if isinstance(value.get('format'), str) and re.search(r'spheres-.*save', value['format']):
                return True
            if isinstance(value.get('world'), dict) and 'nations' in value['world']:
                return True
            if 'nations' in value and any(k in value for k in ['rng', 'rules', 'production']):
                return True
    return False


def main(argv=None):
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('runtime', help='Exact 40-character qualified runtime revision')
    parser.add_argument('--repo', type=Path, default=BASE / 'integration')
    parser.add_argument('--france', type=Path, required=True, help='Chosen passed France result.json')
    parser.add_argument('--tonga', type=Path, required=True, help='Chosen passed Tonga result.json')
    parser.add_argument('--binary-proof', type=Path, default=BASE / 'evidence/S07-final-binary.json')
    parser.add_argument('--native-proof', type=Path, default=BASE / 'evidence/S07-final-native.json')
    parser.add_argument('--node-proof', type=Path, default=BASE / 'evidence/S07-final-node.json')
    parser.add_argument('--linux', type=Path, default=BASE / 'evidence/S07-linux-final')
    parser.add_argument('--external', type=Path, default=BASE / 'evidence/S07-external-final')
    parser.add_argument('--performance-result', type=Path, required=True, help='Chosen six-case acceptance.json')
    parser.add_argument('--performance-protocol', type=Path, required=True)
    parser.add_argument('--preservation', type=Path, required=True, help='Final protected-file audit JSON')
    parser.add_argument('--include', type=Path, action='append', default=[], help='Additional evidence file/directory; repeatable')
    parser.add_argument('--failed-attempt', type=Path, action='append', default=[], help='Additional failed browser result; repeatable')
    parser.add_argument('--apply', action='store_true', help='Write the fully preflighted evidence package')
    args = parser.parse_args(argv)
    pin, repo = args.runtime, args.repo.resolve()
    require(re.fullmatch('[0-9a-f]{40}', pin), 'Runtime must be a full lowercase Git SHA')
    require(git(repo, 'rev-parse', 'HEAD').decode().strip() == pin, 'Checkout HEAD differs from requested runtime')
    destination = repo / 'docs/campaign-certification/S07/evidence'
    require(not destination.exists(), f'Refusing to overwrite an existing package: {destination}')

    binary = passed_proof(args.binary_proof, pin)
    binary_path = Path(binary['binary'])
    require(file_hash(binary_path) == binary['binary_sha256'], 'Built executable changed after qualification')
    native = passed_proof(args.native_proof, pin)
    node = passed_proof(args.node_proof, pin)
    require(file_hash(native['binary']) == native['binary_sha256'], 'Windows test binary changed after qualification')
    native_totals = rust_totals(args.native_proof.with_suffix('.log'))
    linux = read(args.linux / 'runner-result.json')
    require(linux.get('candidate') == pin and linux.get('status') == 'passed', 'Linux qualification is not passed on this runtime')
    require(len(linux.get('checks', [])) == 5 and all(c['exit_code'] == 0 for c in linux['checks']), 'Linux full/native/three external checks incomplete')
    for key in ['source_before', 'source_after']:
        require(linux[key]['head'] == pin and not linux[key]['status_porcelain'], f'Linux {key} mismatch')
    require(linux['web_test_binary']['sha256'] == linux['web_test_binary']['sha256_after'], 'Linux test binary changed')
    for check in linux['checks']:
        require(file_hash(args.linux / check['log']) == check['log_sha256'], f'Linux log changed: {check["name"]}')
    linux_totals = rust_totals(args.linux / 'native.log')
    external = read(args.external / 'result.json')
    require(external.get('candidate') == pin == external.get('candidate_after') and external.get('clean_after') is True,
            'Windows external fixture source mismatch')
    require(len(external.get('checks', [])) == 3 and all(c['exit_code'] == 0 for c in external['checks']), 'Windows external fixtures incomplete')
    require(external.get('binary_unchanged') is True and external['binary_sha256'] == native['binary_sha256'],
            'Windows external fixtures did not use the qualified test binary')

    performance = read(args.performance_result)
    require(performance.get('candidate') == pin and performance.get('passed') is True, 'Chosen performance result is not passed on this runtime')
    expected_cases = {(s, age) for s in ['idle_human', 'industry_and_war'] for age in [0, 10, 30]}
    rows = performance.get('rows', [])
    require(len(rows) == 6 and {(r['scenario'], r['age']) for r in rows} == expected_cases and all(r.get('passed') is True for r in rows),
            'Performance acceptance must include all six frozen cases')
    require(args.performance_protocol.is_file(), 'Missing performance protocol')
    preservation = read(args.preservation)
    protected = preservation.get('files', [])
    require(len(protected) == 8 and all(r.get('unchanged') is True for r in protected), 'Protected campaign audit is incomplete or failed')
    for row in protected:
        p = Path(row['path'])
        require(p.stat().st_size == row['bytes'] and file_hash(p) == row['sha256'], f'Protected campaign changed after audit: {p}')

    # Preflight every source byte before creating any destination. References
    # preserve original paths; inventory hashes apply to raw copied bytes.
    pending, references, browser = {}, [], {}

    def collect(path, relative):
        path = Path(path).resolve()
        require(path.is_file() and not path.is_symlink(), f'Expected ordinary evidence file: {path}')
        relative = PurePosixPath(relative)
        require(not relative.is_absolute() and '..' not in relative.parts, f'Unsafe package path: {relative}')
        data = path.read_bytes()
        row = {'original_path': str(path), 'bytes': len(data), 'sha256': digest(data)}
        if save_or_binary(path, data):
            references.append(dict(row, reason='Original campaign/archive/binary or unsupported artifact; not copied'))
            return
        key = str(relative)
        require(key not in pending or pending[key]['data'] == data, f'Conflicting evidence destination: {key}')
        pending[key] = dict(row, data=data)

    def tree(path, relative):
        path = Path(path).resolve()
        if path.is_file():
            collect(path, relative)
        else:
            require(path.is_dir() and not path.is_symlink(), f'Missing evidence directory: {path}')
            for child in sorted(path.rglob('*')):
                if child.is_file():
                    collect(child, str(PurePosixPath(relative) / child.relative_to(path).as_posix()))

    short = git(repo, 'rev-parse', '--short=12', pin).decode().strip()
    chosen_paths = {args.france.resolve(), args.tonga.resolve()}
    for nation, path in [('France', args.france), ('Tonga', args.tonga)]:
        result = read(path)
        require(result.get('passed') is True and result.get('nation') == nation, f'Chosen {nation} browser result did not pass')
        build = result['build']
        require(build['revision'] == pin and build['build']['revision'] == short, f'{nation} browser build revision mismatch')
        require(build['binary_sha256'] == binary['binary_sha256'], f'{nation} browser executable hash mismatch')
        require(result.get('finished_utc') and not result.get('errors'), f'{nation} browser result is incomplete or has errors')
        require(Path(build['build']['save_directory']).resolve() == path.resolve().parent, f'{nation} browser used a different save directory')
        for assets in [build.get('assets', {}), result.get('assets', {})]:
            require(bool(assets), f'{nation} browser lacks captured asset hashes')
            for name, asset in assets.items():
                require(PurePosixPath(name).name == name, 'Unexpected browser asset path')
                committed = git(repo, 'show', f'{pin}:spheres-web/ui/{name}')
                checkout = (repo / 'spheres-web/ui' / name).read_bytes()
                require(digest(committed) == asset['committed_sha256'], f'{nation} committed asset hash mismatch: {name}')
                require(digest(checkout) == asset['served_sha256'] and checkout.replace(b'\r\n', b'\n') == committed,
                        f'{nation} served asset differs from the pinned checkout: {name}')
        target = f'browser/{nation.lower()}'
        tree(path.parent, target)
        browser[nation] = {'result': f'evidence/{target}/{path.name}', 'original_result': str(path.resolve()),
                          'result_sha256': file_hash(path), 'binary_sha256': build['binary_sha256'],
                          'screenshots': result.get('screenshots', [])}
        for screenshot in result.get('screenshots', []):
            require((path.parent / screenshot).is_file(), f'Missing {nation} screenshot: {screenshot}')

    # Discovery deliberately retains all prior S07 development/performance
    # attempts; the explicitly chosen successes above alone qualify the result.
    source_evidence = BASE / 'evidence'
    for path in sorted(source_evidence.glob('S07-*')):
        tree(path, 'qualification/' + path.name)
    for path in [args.binary_proof, args.native_proof, args.node_proof, args.performance_result,
                 args.performance_protocol, args.preservation]:
        collect(path, 'selected/' + path.name)
    for path in [args.linux, args.external]:
        tree(path, 'selected/' + path.name)
    for path in args.include:
        tree(path, 'additional/' + path.name)
    attempts = set(args.failed_attempt)
    for directory in {args.france.resolve().parent.parent, args.tonga.resolve().parent.parent,
                      repo / 'artifacts/browser-construction-ci'}:
        if directory.exists():
            attempts.update(directory.glob('*/result.json'))
    for path in sorted(attempts):
        if path.resolve() in chosen_paths:
            continue
        result = read(path)
        if result.get('passed') is not True:
            tree(path.parent, 'failed-browser-attempts/' + path.parent.name)
    for path in sorted(BASE.glob('run-s07-*.py')):
        collect(path, 'collectors/' + path.name)
    collect(Path(__file__), 'collectors/' + Path(__file__).name)
    for name in ['ci-construction.cjs', 'ci-integrated.cjs']:
        collect(repo / 'tools/ui' / name, 'collectors/' + name)
    for title, path, expected in [('server', binary_path, binary['binary_sha256']),
                                  ('windows_web_tests', Path(native['binary']), native['binary_sha256'])]:
        references.append({'kind': title, 'original_path': str(path), 'bytes': path.stat().st_size,
                           'sha256': expected, 'reason': 'Qualified executable; hash-only provenance'})
    references.append(dict(linux['web_test_binary'], kind='linux_web_tests',
                           reason='Linux before/after captured hashes; executable not copied or read from Windows'))

    inventory = {'format': 'spheres-s07-evidence-inventory', 'version': 1, 'session': 'S07',
        'status': 'evidence_collected_only', 'runtime_revision': pin,
        'collected_utc': datetime.datetime.now(datetime.timezone.utc).isoformat(),
        'scope': 'Evidence collection is not a completion decision, G2 award or CP1 certificate.',
        'qualification': {'windows_native': native_totals, 'linux_native': linux_totals,
                          'windows_node_proof': str(args.node_proof.resolve()), 'linux_status': linux['status'],
                          'windows_external_checks': len(external['checks']), 'performance': performance},
        'browser': browser, 'external_references': references,
        'evidence_files': [{'file': 'evidence/' + name, **{k: row[k] for k in ['original_path', 'bytes', 'sha256']}}
                           for name, row in sorted(pending.items())],
        'inventory_note': 'This inventory excludes itself to avoid a recursive hash. No save or binary bytes are duplicated.'}
    serialized = (json.dumps(inventory, indent=2, ensure_ascii=False) + '\n').encode('utf-8')
    if args.apply:
        # All checks and source reads above have completed successfully.
        destination.mkdir(parents=True, exist_ok=False)
        for name, row in sorted(pending.items()):
            target = destination / name
            target.parent.mkdir(parents=True, exist_ok=True)
            with target.open('xb') as handle:
                handle.write(row['data'])
            require(file_hash(target) == row['sha256'], f'Copied evidence byte mismatch: {target}')
        with (destination / 'inventory.json').open('xb') as handle:
            handle.write(serialized)
    print(json.dumps({'mode': 'applied' if args.apply else 'dry_run', 'runtime_revision': pin,
                      'destination': str(destination), 'evidence_files': len(pending),
                      'evidence_bytes': sum(len(r['data']) for r in pending.values()),
                      'external_references': len(references)}, indent=2))


if __name__ == '__main__':
    try:
        main()
    except (ValueError, KeyError, OSError, subprocess.CalledProcessError) as exc:
        print(f'S07 evidence preflight refused: {exc}', file=sys.stderr)
        sys.exit(1)
