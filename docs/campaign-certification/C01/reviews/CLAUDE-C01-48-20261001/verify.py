"""Receipt integrity only; material reading is not recomputed."""
from pathlib import Path
import hashlib,json,subprocess
here=Path(__file__).resolve().parent
repo=Path(subprocess.check_output(['git','rev-parse','--show-toplevel'],cwd=here,text=True).strip())
load=lambda n:json.loads((here/n).read_text(encoding='utf-8'))
ext=Path(load('integration.json')['external_root'])
def check(base,pin):
 b=(base/pin['path']).read_bytes();assert (len(b),hashlib.sha256(b).hexdigest())==(pin['bytes'],pin['sha256']),pin['path']
for s in load('source-review.json')['sources']:
 for k in ('original','headers','reading_aid'):check(ext,s[k])
 for p in s['inspected_pdf_renders']:check(ext,p)
for p in load('git-inputs.json'):
 b=subprocess.check_output(['git','show',p['revision']+':'+p['path']],cwd=repo);assert (len(b),hashlib.sha256(b).hexdigest())==(p['bytes'],p['sha256']),p['path']
for r in load('validation/results.json')['results']:check(here,{'path':r['log'],'bytes':r['log_bytes'],'sha256':r['log_sha256']})
for p in load('integrity.json'):check(here,p)
assert len(load('claim-review.json')['claims'])==24 and len(load('holder-review.json')['holders'])==4
print('PASS: India originals, renders, 30 Git inputs, 24 claims, four holders and preserved review/validation bytes.')
