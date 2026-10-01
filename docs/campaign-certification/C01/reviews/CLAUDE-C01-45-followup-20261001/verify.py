from pathlib import Path
import hashlib,json,subprocess
here=Path(__file__).resolve().parent
repo=next(p for p in here.parents if (p/'.git').exists())
manifest=json.loads((here/'manifest.json').read_text(encoding='utf-8'))
actual={p.relative_to(here).as_posix() for p in here.rglob('*') if p.is_file() and p.name!='manifest.json' and '__pycache__' not in p.parts}
assert actual=={r['path'] for r in manifest['files']}
for row in manifest['files']:
    p=(here/row['path']).resolve();assert p.is_relative_to(here)
    data=p.read_bytes();assert len(data)==row['bytes'] and hashlib.sha256(data).hexdigest()==row['sha256'],row['path']
d=json.loads((here/'decision.json').read_text(encoding='utf-8'))
assert subprocess.check_output(['git','diff','--name-only',d['base'],d['repair_revision']],cwd=repo).decode().splitlines()==d['adopted_paths']
assert d['source_claim_holder_data_changed'] is False and d['historical_acceptance_extended'] is False
print(json.dumps({'files':len(actual),'accepted_scope':'one stricter test with three adverse mutations','historical_or_parent_acceptance_extended':False}))
