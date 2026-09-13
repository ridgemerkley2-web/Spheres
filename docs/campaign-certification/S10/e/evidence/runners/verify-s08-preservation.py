"""Read-only protected originals/worktrees check. Writes a new evidence record."""
import datetime,hashlib,json,pathlib,subprocess,sys
base=pathlib.Path(__file__).resolve().parent
baseline=base/'evidence/S08-preservation-before.json'
output=pathlib.Path(sys.argv[1]).resolve() if len(sys.argv)>1 else base/'evidence/S08-preservation-after.json'
assert not output.exists()
def sha(p):
    with p.open('rb') as f:return hashlib.file_digest(f,'sha256').hexdigest()
before=json.loads(baseline.read_text(encoding='utf-8-sig'))
record={'captured_utc':datetime.datetime.now(datetime.timezone.utc).isoformat(),'baseline_sha256':sha(baseline),'files':[],'worktrees':[],'passed':False}
for row in before['files']:
    p=pathlib.Path(row['path']);actual=sha(p)
    record['files'].append({'path':str(p),'bytes':p.stat().st_size,'sha256':actual,'unchanged':actual==row['sha256'] and p.stat().st_size==row['bytes']})
for p,expected in [(base.parent/'Spheres','038fe0b1a3edb1772586ec0b5a7451629b7297c4'),(base/'master-baseline','485c223f60d5ff6e46f6ae17164bf1ee3a8764d9')]:
    actual=subprocess.check_output(['git','-c','core.longpaths=true','-C',str(p),'rev-parse','HEAD'],text=True).strip()
    record['worktrees'].append({'path':str(p),'head':actual,'expected':expected,'unchanged':actual==expected})
record['passed']=len(record['files'])==8 and all(r['unchanged'] for r in record['files']+record['worktrees'])
with output.open('x',encoding='utf-8') as f:json.dump(record,f,indent=2);f.write('\n')
print(json.dumps({'passed':record['passed'],'evidence':str(output),'files':len(record['files'])}),flush=True)
sys.exit(0 if record['passed'] else 1)
