import hashlib,json,pathlib
p=pathlib.Path(__file__).resolve().parent
m=json.loads((p/'manifest.json').read_text(encoding='utf-8'))
assert m['format']=='spheres-research-review/v1' and m['status']=='accepted_for_integration'
for e in m['evidence']:
 f=(p/e['path']).resolve(); assert f.is_relative_to(p.resolve())
 b=f.read_bytes(); assert len(b)==e['bytes'] and hashlib.sha256(b).hexdigest()==e['sha256'], e['path']
r=json.loads((p/'retrievals.json').read_text(encoding='utf-8'))['results']
assert len(r)==m['response_count']
assert all(x['status']==200 and x['bytes']==x['expected_bytes'] and x['sha256']==x['expected_sha256'] and x['matches_claimed_response'] for x in r)
ids={json.loads((p/x).read_text(encoding='utf-8'))['source_id'] for x in m['reviewed_extracts']}
assert ids==set(m['source_ids'])
print(m['task']+': PASS '+str(len(m['evidence']))+' evidence pins; '+str(len(r))+' exact reported retrievals; '+str(len(ids))+' source snapshots (offline integrity only)')
