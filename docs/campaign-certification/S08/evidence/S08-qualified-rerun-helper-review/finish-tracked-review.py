"""Small applied-source/collector evidence; no builds, browser or native tests."""
import ast, datetime, difflib, hashlib, json, pathlib
base=pathlib.Path(__file__).resolve().parent.parent.parent
folder=base/'evidence/S08-qualified-rerun-helper-review'
repo=base/'integration'
sha=lambda p:hashlib.sha256(p.read_bytes()).hexdigest()
collector=base/'collect-s08-evidence.py'
ast.parse(collector.read_text(encoding='utf-8'))
before=folder/'collect-s08-evidence.before.py'
with (folder/'collect-s08-evidence.after.py').open('xb') as stream:stream.write(collector.read_bytes())
with (folder/'collector-dependencies.patch').open('x',encoding='utf-8') as stream:
    stream.write(''.join(difflib.unified_diff(before.read_text(encoding='utf-8').splitlines(True),
        collector.read_text(encoding='utf-8').splitlines(True),fromfile='before/collect-s08-evidence.py',tofile='after/collect-s08-evidence.py')))
names=['ci-supplier-imports.cjs','supplier-archive-audit.cjs','archive-worker.py']
record={'captured_utc':datetime.datetime.now(datetime.timezone.utc).isoformat(),
 'tracked_files':{name:{'path':str(repo/'tools/ui'/name),'sha256':sha(repo/'tools/ui'/name)} for name in names},
 'applied_patch':str(base/'evidence/S08-supplier-browser-memory-fix/future-tracked-harness.patch'),
 'checks':['git apply --check passed before apply','Node --check passed for runner and audit helper','Python ast.parse passed for tracked worker',
           'Small tracked-helper probes passed: exact whole-world mismatch; only selected plan excluded; unrelated cash change rejected; null legacy holdings retained; equal canonical bytes'],
 'tracked_helper_probe':str(base/'evidence/S08-supplier-browser-memory-fix/helper-probes-zUdukT/result.json'),
 'qualification_executed':False,'browser_executed':False,
 'collector':{'before_sha256':sha(before),'after_sha256':sha(collector),'syntax':'ast.parse passed','collect_or_apply_executed':False,
              'change':'Include exact tracked audit helper/worker and actual Money journey runner as package dependencies.'}}
with (folder/'tracked-harness-audit.json').open('x',encoding='utf-8') as stream:json.dump(record,stream,indent=2);stream.write('\n')
print(json.dumps(record,indent=2))
