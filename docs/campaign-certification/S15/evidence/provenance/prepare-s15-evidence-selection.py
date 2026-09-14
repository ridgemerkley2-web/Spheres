"""Select complete S15 evidence only after all test and browser processes end."""
import importlib.util,json,pathlib
base=pathlib.Path(__file__).resolve().parent
output=base/'S15-evidence-selection.json';assert not output.exists()
launch=json.loads((base/'S15-review-launch.json').read_text(encoding='utf-8-sig'))
items=[]
def add(p,name):
    assert p.exists(),str(p)
    items.append({'source':str(p.resolve()),'name':name})
for p in sorted((base/'evidence').iterdir()):
    if p.name.startswith('S15-'):add(p,'attempts/'+p.name)
for p in sorted(base.iterdir()):
    if p.is_file() and ('s15' in p.name.lower() or p.name.startswith('dev-ci-flight-operations') or p.name=='ci-flight-operations.proposed.cjs') and p!=output:
        add(p,'provenance/'+p.name)
for name in ('collect-s09-evidence.py','verify-s08-preservation.py','run-s10-final-linux.py'):
    add(base/name,'provenance/'+name)
for name in ('archive-worker.py','ci-flight-operations.cjs','supplier-archive-audit.cjs','ci-integrated.cjs'):
    add(base/'integration/tools/ui'/name,'provenance/browser-tools/'+name)
add(base/'evidence/S08-preservation-before.json','preservation/original-baseline.json')
review=pathlib.Path(launch['directory'])
add(pathlib.Path(launch['save_copy']['copy']),'review/s15-flight-operations.json')
add(review/'native-verification','review/native-verification')
spec=importlib.util.spec_from_file_location('retention',base/'collect-s09-evidence.py')
module=importlib.util.module_from_spec(spec);spec.loader.exec_module(module)
plan=module.preflight(items,base/'integration/docs/campaign-certification/S15/evidence')
assert not any(p.suffix.lower() in ('.exe','.dll') for p,_ in plan)
output.write_text(json.dumps(items,indent=2)+'\n',encoding='utf8')
print(json.dumps({'selection':str(output),'sources':len(items),'files':len(plan),'raw_bytes':sum(p.stat().st_size for p,_ in plan)}),flush=True)
