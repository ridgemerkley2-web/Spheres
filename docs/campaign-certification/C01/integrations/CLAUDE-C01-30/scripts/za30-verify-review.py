"""Read-only checks of the preserved C01-30 source bodies and inherited packet."""
import argparse, base64, copy, gzip, hashlib, json, subprocess
from pathlib import Path

def sha(b): return hashlib.sha256(b).hexdigest()
def read(p): return json.loads(p.read_text(encoding='utf-8'))
def write(p, value): p.write_text(json.dumps(value, ensure_ascii=False, indent=2)+'\n', encoding='utf-8', newline='\n')

p=argparse.ArgumentParser();p.add_argument('--root',type=Path,required=True);p.add_argument('--evidence',type=Path,required=True);p.add_argument('--out',type=Path,required=True);a=p.parse_args();a.out.mkdir(exist_ok=False)
relative='docs/campaign-certification/C01/research/south-africa.json'
raw=(a.root/relative).read_bytes(); current=json.loads(raw)
base_raw=subprocess.check_output(['git','show','033f098d^:'+relative],cwd=a.root);base=json.loads(base_raw)
submitted=json.loads(subprocess.check_output(['git','show','cb0f1153:'+relative],cwd=a.root))
base_ids={s['id'] for s in base['sources']};sources=[s for s in current['sources'] if s['id'] not in base_ids]
assert len(sources)==39; source_ids={s['id'] for s in sources};claim_ids={c['id'] for s in sources for c in s['claims']};assert len(claim_ids)==64
first=read(a.evidence/'retrieval.json')['sources'];retry=read(a.evidence/'retrieval-retry.json')['sources']
assert len(first)==39 and len(retry)==19 and sum(r['exact'] for r in first)==20 and all(r['exact'] for r in retry)
assert {r['source_id'] for r in retry}=={r['source_id'] for r in first if not r['exact']}
retrieved={r['source_id']:r for r in first+retry if r['exact']};pins=[]
for s in sources:
 r=retrieved[s['id']];b=Path(r['body_path']).read_bytes();ep=a.root/s['snapshot']['path'];eb=ep.read_bytes();e=json.loads(eb)
 assert r['exit_code']==0 and r['response'].splitlines()[0]=='200' and r['url']==s['url']==e['source_url']
 assert len(b)==e['source_response_bytes'] and sha(b)==e['source_response_sha256']
 digest=base64.b32encode(hashlib.sha1(b).digest()).decode('ascii');assert digest==e['source_response_sha1_base32']
 assert len(eb)==s['snapshot']['bytes'] and sha(eb)==s['snapshot']['sha256']
 decoded=gzip.decompress(b) if b.startswith(b'\x1f\x8b') else b
 if 'decoded_response_sha256' in e: assert sha(decoded)==e['decoded_response_sha256']
 if 'decoded_response_bytes' in e: assert len(decoded)==e['decoded_response_bytes']
 pins.append({'source_id':s['id'],'url':s['url'],'original_url':s['original_url'],'body_path':r['body_path'],'bytes':len(b),'sha256':sha(b),'sha1_base32':digest,'content_encoding':e['source_response_content_encoding'],'decoded_bytes':len(decoded),'decoded_sha256':sha(decoded),'snapshot':s['snapshot'],'claims':[c['id'] for c in s['claims']]})
stripped=copy.deepcopy(current);stripped['sources']=[s for s in stripped['sources'] if s['id'] not in source_ids]
roles={'za_acdp_president','za_ff_leader','za_ifp_president'};removed_roles=[]
for o in stripped['organizations']:
 new=[r for r in o['roles'] if r['id'] in roles];removed_roles+=new;o['roles']=[r for r in o['roles'] if r['id'] not in roles]
 o['sources']=[i for i in o['sources'] if i not in source_ids];o['claim_ids']=[i for i in o['claim_ids'] if i not in claim_ids]
 o['coverage']['unresolved']=[i for i in o['coverage']['unresolved'] if '(CLAUDE-C01-30,' not in i]
stripped['coverage']['unresolved']=[i for i in stripped['coverage']['unresolved'] if '(CLAUDE-C01-30,' not in i]
assert stripped==base, 'Inherited parsed packet changed'
assert len(removed_roles)==3 and sum(len(r['holder_claims']) for r in removed_roles)==22
comparison=copy.deepcopy(current);comparison['coverage']['unresolved']=submitted['coverage']['unresolved'];assert comparison==submitted, 'Repair changed more than country coverage prose'
write(a.out/'source-pins.json',{'format':'spheres-source-review-pins/v1','reviewed_commit':'cb0f1153f0d26729445acbc05843c3f16bf78ee7','source_count':39,'claim_count':64,'raw_sha256_and_sha1_and_bytes_match':True,'derived_snapshot_hashes_match':True,'sources':pins})
write(a.out/'baseline-isolation.json',{'format':'spheres-research-isolation/v1','passed':True,'baseline_commit':subprocess.check_output(['git','rev-parse','033f098d^'],cwd=a.root,text=True).strip(),'baseline_country_git_blob_sha256':sha(base_raw),'reviewed_submission':'cb0f1153f0d26729445acbc05843c3f16bf78ee7','reviewed_worktree_country_sha256':sha(raw),'removed_new_sources':sorted(source_ids),'removed_new_claims':sorted(claim_ids),'removed_new_roles':sorted(roles),'removed_only_C01_30_coverage_notes':True,'all_remaining_parsed_fields_equal_baseline':True,'repair_changes_only_country_coverage_prose':True,'new_holder_count':22,'source_and_holder_data_unchanged_by_repair':True})
print(json.dumps({'passed':True,'sources':39,'claims':64,'snapshots':39,'inherited_packet_equal':True,'holders_unchanged':22}))
