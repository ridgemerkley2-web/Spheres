from pathlib import Path
import sys,json
W=Path('D:/spheres-offload/codex-next-20260928/review-su35');sys.path.insert(0,str(W/'tools/avatars'));import campaign_census
now=campaign_census.build(W);rows=[]
def compare(a,b,p):
 if isinstance(a,dict) and isinstance(b,dict):
  for k in sorted(set(a)|set(b)):compare(a.get(k),b.get(k),p+'/'+k)
 elif isinstance(a,list) and isinstance(b,list) and len(a)==len(b):
  for i,(x,y) in enumerate(zip(a,b)):compare(x,y,p+'/'+str(i))
 elif a!=b:rows.append({'path':p,'recorded':a,'current':b})
for n,v in now.items():
 p=W/'docs/campaign-certification/C01'/n
 if p.exists():compare(json.loads(p.read_text(encoding='utf8')),v,n)
 else:rows.append({'path':n,'absent_in_sparse_worktree':True})
E=W.parent/'su35-review';(E/'census-difference.json').write_text(json.dumps(rows,indent=2)+'\n');print(json.dumps(rows,indent=2))
