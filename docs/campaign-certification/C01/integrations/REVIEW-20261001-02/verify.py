from pathlib import Path
import hashlib,json,subprocess
here=Path(__file__).resolve().parent
root=next(p for p in here.parents if (p/'.git').exists())
def load(p):return json.loads(p.read_text(encoding='utf-8'))
def check(data,row):
    assert len(data)==row['bytes'] and hashlib.sha256(data).hexdigest()==row['sha256'],row['path']
manifest=load(here/'manifest.json')
actual={p.relative_to(here).as_posix() for p in here.rglob('*') if p.is_file() and p.name!='manifest.json' and '__pycache__' not in p.parts}
assert actual=={r['path'] for r in manifest['files']}
for row in manifest['files']:
    path=(here/row['path']).resolve();assert path.is_relative_to(here)
    check(path.read_bytes(),row)
scope=load(here/'scope.json');rev=scope['reviewed_integration_revision']
inputs=load(here/'reviewed-inputs.json')
for row in inputs:check(subprocess.check_output(['git','show',rev+':'+row['path']],cwd=root),row)
for receipt in scope['receipt_revisions'].values():subprocess.run(['git','merge-base','--is-ancestor',receipt,rev],cwd=root,check=True)
print(json.dumps({'receipt_files':len(actual),'reviewed_git_inputs':len(inputs),'qualification':False,'historical_reading_repeated':False}))
