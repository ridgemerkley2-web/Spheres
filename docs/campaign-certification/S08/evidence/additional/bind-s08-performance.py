import datetime, hashlib, json, pathlib, re, subprocess, sys

base = pathlib.Path(__file__).resolve().parent
pin, native_name, label = sys.argv[1:]
assert re.fullmatch(r'[0-9a-f]{40}', pin)
assert re.fullmatch(r'[A-Za-z0-9_-]+', label)
native_path = pathlib.Path(native_name).resolve()
native = json.loads(native_path.read_text(encoding='utf-8-sig'))
sha = lambda p: hashlib.sha256(pathlib.Path(p).read_bytes()).hexdigest()
git = lambda *args: subprocess.check_output(['git', '-c', 'core.longpaths=true', *args], cwd=base/'integration', text=True).strip()
assert git('rev-parse', 'HEAD') == pin and not git('status', '--porcelain')
assert native['passed'] and native['preservation_passed'] and native['exit_code'] == 0
assert native['revision_before'] == native['revision_after'] == pin
assert native['clean_before'] and native['clean_after']
assert native['command'] == ['cargo', 'test', '--locked', '--release', '--workspace', '--no-fail-fast']
assert native['totals']['failed'] == 0 and native['totals']['completed_targets'] > 0
assert sha(native['binary']) == native['binary_sha256']
plan_path = base/'evidence/S08-performance-plan.json'
assert sha(plan_path) == '8d2cb23fe996ba9dc35da0aed5859072a83c2e61b55d4881cf047bc1ac195a28'
plan = json.loads(plan_path.read_text(encoding='utf-8-sig'))
protocol = dict(plan['legacy_regression'])
protocol.update(candidate=pin, test_binary=native['binary'], test_binary_sha256=native['binary_sha256'], declared_plan_sha256=sha(plan_path), declared_utc=plan['declared_utc'], bound_utc=datetime.datetime.now(datetime.timezone.utc).isoformat(), native_proof=str(native_path), native_proof_sha256=sha(native_path), binding_runner_sha256=sha(__file__), note='Bind the original six workloads and unchanged acceptance bars to the clean candidate and completed full native proof before measurement. The genuine supplier case retains its separate original limits.')
out = base/('evidence/S08-performance-protocol-'+label+'.json')
with out.open('x', encoding='utf-8') as stream:
    json.dump(protocol, stream, indent=2)
    stream.write('\n')
print(json.dumps(protocol, indent=2))
