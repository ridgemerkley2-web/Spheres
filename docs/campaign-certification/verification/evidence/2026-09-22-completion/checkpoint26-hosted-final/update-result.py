from pathlib import Path
from datetime import datetime,timezone
import hashlib,json,re
p=Path(__file__).resolve().parent
keys=['passed','failed','ignored','measured','filtered']
def load(n): return json.loads((p/n).read_text(encoding='utf-8-sig'))
def save(n,x): (p/n).write_text(json.dumps(x,indent=2)+'\n',encoding='utf-8',newline='\n')
run=load('run.json'); snap=load('latest-jobs.json'); logs=[]
for f in sorted(p.glob('*.log')):
 b=f.read_bytes();s=b.decode('utf-8');rows=[list(map(int,x)) for x in re.findall(r'test result: (?:ok|FAILED)\. (\d+) passed; (\d+) failed; (\d+) ignored; (\d+) measured; (\d+) filtered out;',s)]
 r={'path':f.name,'bytes':len(b),'sha256':hashlib.sha256(b).hexdigest(),'expected_checkout_mentioned':run['head_sha'] in s,'failed_tests':re.findall(r'test ([^\r\n]+?) \.\.\. FAILED',s)}
 if f.name.startswith('native-') and len(rows)==67:
  r.update(workspace_targets=66,workspace_counts=dict(zip(keys,[sum(x[i] for x in rows[:-1]) for i in range(5)])),isolated_timing_counts=dict(zip(keys,rows[-1])))
 elif rows:r['counts']=dict(zip(keys,rows[-1]))
 metrics=re.findall(r'A1: [^\r\n]+',s)
 if metrics:r['A1_metrics']=metrics
 logs.append(r)
complete=len(snap['jobs'])==10 and all(j['status']=='completed' for j in snap['jobs'])
counts={v:sum(j['conclusion']==v for j in snap['jobs']) for v in ['success','failure']}
active=[{'id':j['id'],'name':j['name'],'status':j['status'],'step':next((s['name'] for s in j.get('steps',[]) if s['status']=='in_progress'),None)} for j in snap['jobs'] if j['status']!='completed']
result={'run':run,'observed_at':snap['observed_at'],'complete':complete,'job_counts':counts,'jobs':snap['jobs'],'expected_job_count':10,'logs':logs,'interpretation':'Both political jobs fail only original A1 (median 7 coups; top-three share 0.59); A2-A10 and attribution controls pass. This is not all-green certification. Inspect completed job steps and logs for exact coverage; an in-progress job is not a pass.','active':active}
save('result.json',result)
hist=load('poll-history.json') if (p/'poll-history.json').exists() else []
entry={'observed_at':snap['observed_at'],'jobs':[{'id':j['id'],'name':j['name'],'status':j['status'],'conclusion':j['conclusion'],'active_step':next((s['name'] for s in j.get('steps',[]) if s['status']=='in_progress'),None)} for j in snap['jobs']]}
if not hist or hist[-1]['jobs']!=entry['jobs']:hist.append(entry)
save('poll-history.json',hist)
save('artifact-hashes.json',{'algorithm':'sha256','files':[{'path':f.name,'bytes':f.stat().st_size,'sha256':hashlib.sha256(f.read_bytes()).hexdigest()} for f in sorted(p.iterdir()) if f.is_file() and f.name!='artifact-hashes.json']})
print(json.dumps({'complete':complete,'job_counts':counts,'active':active}))
