"""Harness-only comparator probes, then a read-only existing failed-run audit."""
import ast, datetime, hashlib, json, pathlib, subprocess, sys, time

base = pathlib.Path(__file__).resolve().parent
worker = base / 'archive-worker.py'
ast.parse(worker.read_text(encoding='utf-8'))
out = base / 'diagnostic-2'
out.mkdir()
proof = {'started_utc': datetime.datetime.now(datetime.timezone.utc).isoformat(),
         'worker_sha256': hashlib.sha256(worker.read_bytes()).hexdigest(),
         'note': 'Tiny synthetic objects test only the audit tool. Native campaign observations below read retained failed-run files without modification.',
         'probes': [], 'archives': []}

def invoke(args):
    started = time.monotonic()
    result = subprocess.run([sys.executable, str(worker), *map(str, args)], capture_output=True,
                            creationflags=subprocess.CREATE_NO_WINDOW if sys.platform == 'win32' else 0)
    if result.returncode:
        raise RuntimeError(result.stderr.decode('utf-8', errors='replace')[:4000])
    return json.loads(result.stdout), time.monotonic() - started

try:
    template = {'nations': [{'id': 'Tonga', 'equipment': {'revisions': {}, 'learned': []},
                            'arsenal': {'held': []}, 'tech': {'v': 1}, 'treasury': -0.0}],
                'companies': {'firms': [], 'imports': {'contracts': []}}}
    probes = []
    for label, positive_zero, plan, ignore in [('zero', False, None, False), ('positive-zero', True, None, False),
                                             ('masked-before', False, None, True), ('masked-after', False, {'daily_limit_bn': 0.002}, True)]:
        world = json.loads(json.dumps(template))
        if positive_zero:
            world['nations'][0]['treasury'] = 0
        if plan is not None:
            world['nations'][0]['equipment']['maintenance_plan'] = plan
        source, canonical = out / (label + '.json'), out / (label + '.canonical')
        source.write_text(json.dumps(world), encoding='utf-8')
        value, duration = invoke([source, canonical, 'Tonga', *(['--ignore-maintenance-plan'] if ignore else [])])
        probes.append(value)
    zero_diff, _ = invoke(['--compare', probes[0]['canonical']['path'], probes[1]['canonical']['path']])
    assert zero_diff['equal'] is False and zero_diff['path'] == ['nations', 0, 'treasury']
    masked, _ = invoke(['--compare', probes[2]['canonical']['path'], probes[3]['canonical']['path']])
    assert masked['equal'] is True
    proof['probes'] = [{'signed_zero_is_distinct': zero_diff}, {'only_plan_is_ignored': masked}]
    repo = base.parent.parent / 'integration'
    failed = repo / 'artifacts/browser-supplier-imports-ci/supplier-g630Bz/server/saves'
    for label in ('delivered', 'maintenance-approved'):
        source = failed / ('s08-' + label + '.json')
        value, duration = invoke([source, out / (label + '.canonical'), 'Tonga', '--seller', 'France',
                                  '--company', '3', '--product', '48', '--revision', 'import-3-48',
                                  '--ignore-maintenance-plan'])
        (out / (label + '-projection.json')).write_text(json.dumps(value, indent=2) + '\n', encoding='utf-8')
        proof['archives'].append({'label': label, 'duration_seconds': duration, 'projection': value})
    comparison, duration = invoke(['--compare', *(a['projection']['canonical']['path'] for a in proof['archives'])])
    proof['comparison'] = comparison
    proof['comparison_seconds'] = duration
    proof['completed'] = True
except Exception as error:
    proof.update(completed=False, error=repr(error))
    raise
finally:
    proof['finished_utc'] = datetime.datetime.now(datetime.timezone.utc).isoformat()
    (out / 'result.json').write_text(json.dumps(proof, indent=2) + '\n', encoding='utf-8')
    print(json.dumps({key: proof.get(key) for key in ('completed', 'comparison', 'error')}, indent=2))
