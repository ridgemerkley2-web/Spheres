from pathlib import Path
import json,hashlib,subprocess
here=Path(__file__).resolve().parent
repo=Path(subprocess.check_output(['git','rev-parse','--show-toplevel'],cwd=here,text=True).strip())
load=lambda f:json.loads((here/f).read_text(encoding='utf-8'))
def check(p,b):assert (len(b),hashlib.sha256(b).hexdigest())==(p['bytes'],p['sha256']),p['path']
for p in load('external-evidence.json'):check(p,Path(p['path']).read_bytes())
for p in load('git-inputs.json'):check(p,subprocess.check_output(['git','show',p['revision']+':'+p['path']],cwd=repo))
for p in load('integrity.json'):check(p,(here/p['path']).read_bytes())
assert len(load('source-review.json'))==6 and len(load('claim-review.json'))==17
assert load('scope-review.json')['new_holder_observations']==4
print('PASS: six USSR originals, renders, 17 claims, four observations and Git/review byte integrity; not historical re-review.')
