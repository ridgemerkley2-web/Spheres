"""Recheck the preserved recorded responses, extract pins and inherited India packet."""
import argparse, copy, hashlib, json, subprocess
from pathlib import Path

p=argparse.ArgumentParser();p.add_argument('--root',type=Path,required=True);p.add_argument('--retrieval',type=Path,required=True);a=p.parse_args()
relative='docs/campaign-certification/C01/research/india.json'
def git_json(revision):return json.loads(subprocess.check_output(['git','show',revision+':'+relative],cwd=a.root))
current=json.loads((a.root/relative).read_text(encoding='utf-8'));before=git_json('738e6610^');accepted=git_json('8f6627af67dfce1d9026bbe1353c9c1f34828ca7');assert before==accepted
ids={s['id'] for s in before['sources']};new=[s for s in current['sources'] if s['id'] not in ids];assert len(new)==22 and sum(len(s['claims']) for s in new)==41
rows=json.loads(a.retrieval.read_text(encoding='utf-8'))['sources'];assert len(rows)==22 and {r['source_id'] for r in rows}=={s['id'] for s in new}
for source in new:
 row=next(r for r in rows if r['source_id']==source['id']);body=Path(row['body_path']).read_bytes();snapshot=(a.root/source['snapshot']['path']).read_bytes();extract=json.loads(snapshot)
 assert row['exit_code']==0 and row['response'].splitlines()[0]=='200' and row['url']==source['url']==extract['source_url']
 assert len(body)==extract['source_response_bytes']==row['expected_bytes'] and hashlib.sha256(body).hexdigest()==extract['source_response_sha256']==row['expected_sha256'] and body.startswith(b'%PDF-')
 assert len(snapshot)==source['snapshot']['bytes'] and hashlib.sha256(snapshot).hexdigest()==source['snapshot']['sha256']
stripped=copy.deepcopy(current);stripped['sources']=[s for s in stripped['sources'] if s['id'] in ids];stripped['organizations']=[o for o in stripped['organizations'] if o['id']!='in_eci_19980110_np_06'];stripped['coverage']['unresolved']=[s for s in stripped['coverage']['unresolved'] if 'CLAUDE-C01-33' not in s]
assert stripped==accepted
print(json.dumps({'passed':True,'recorded_responses':22,'extracts':22,'claims':41,'all_inherited_India_fields_equal_accepted_integration':True}))
