import datetime, hashlib, json, os, pathlib, subprocess, sys, time
ROOT=pathlib.Path(r'D:\spheres-offload\codex-next-20260928\claude-review-integration-20261001')
OUT=pathlib.Path(r'D:\spheres-offload\codex-next-20260928\pending-review-validation-20261001')/sys.argv[1]
OUT.mkdir(parents=True,exist_ok=False)
py=[sys.executable,'-B','-X','utf8']
phase=sys.argv[1]
if phase.startswith('final-metadata'):
    specs=[('research-index',py+['tools/avatars/campaign_research.py','--check']),
      ('gap-ledger',py+['tools/avatars/certified_gap_ledger.py','--check']),
      ('boundary-matrix',py+['tools/avatars/certified_boundary_matrix.py','--check']),
      ('ledger-boundary-tests',py+['-m','unittest','discover','-s','tools/avatars','-p','test_certified_*.py']),
      ('planning',py+['-m','unittest','discover','-s','tools/planning','-p','test_*.py']),
      ('workboard',py+['tools/planning/workboard.py','--check']),
      ('japan-receipt',py+['docs/campaign-certification/C01/reviews/CLAUDE-C01-31-resumed-20261001/verify.py','--external-originals','--repo',str(ROOT)]),
      ('da-receipt',py+['docs/campaign-certification/C01/reviews/CLAUDE-C01-39/verify_receipt.py','--external-evidence','D:/spheres-offload/codex-next-20260928/c01-39-review-evidence-20261001b']),
      ('diff-check',['git','diff','--check'])]
elif phase.startswith('generation'):
    specs=[('index-regenerate',py+['tools/avatars/campaign_research.py']),
      ('gap-attribution-regenerate',py+['tools/avatars/certified_gap_ledger.py','--refresh-attribution']),
      ('boundary-regenerate',py+['tools/avatars/certified_boundary_matrix.py'])]
else:
    specs=[('research-index',py+['tools/avatars/campaign_research.py','--check']),
      ('census',py+['tools/avatars/campaign_census.py','--check']),
      ('gap-ledger',py+['tools/avatars/certified_gap_ledger.py','--check']),
      ('boundary-matrix',py+['tools/avatars/certified_boundary_matrix.py','--check']),
      ('cartoon-review',py+['tools/avatars/cartoon_review.py','--check']),
      ('avatars',py+['-m','unittest','discover','-s','tools/avatars','-p','test_*.py']),
      ('planning',py+['-m','unittest','discover','-s','tools/planning','-p','test_*.py']),
      ('atlas',['node','--test','tools/ui/check_leadership_research_review.cjs']),
      ('workboard',py+['tools/planning/workboard.py','--check']),
      ('diff-check',['git','diff','--check'])]
results=[]
env=dict(os.environ,PYTHONDONTWRITEBYTECODE='1',GIT_OPTIONAL_LOCKS='0')
for name,cmd in specs:
    start=time.monotonic();p=subprocess.run(cmd,cwd=ROOT,env=env,capture_output=True)
    raw=p.stdout+p.stderr;(OUT/(name+'.log')).write_bytes(raw)
    result={'name':name,'command':cmd,'exit_code':p.returncode,'duration_seconds':round(time.monotonic()-start,3),'log':name+'.log','bytes':len(raw),'sha256':hashlib.sha256(raw).hexdigest()}
    results.append(result)
    print(name,p.returncode,flush=True)
    if p.returncode:
        print(raw.decode('utf-8','replace')[-6000:],flush=True)
        if phase.startswith('generation'):break
data={'source_revision':subprocess.check_output(['git','rev-parse','HEAD'],cwd=ROOT).decode().strip(),
      'phase':phase,'at_utc':datetime.datetime.now(datetime.timezone.utc).isoformat(),'commands':results,'passed':all(r['exit_code']==0 for r in results)}
(OUT/'results.json').write_text(json.dumps(data,indent=2)+'\n',encoding='utf-8')
sys.exit(0 if data['passed'] else 1)

