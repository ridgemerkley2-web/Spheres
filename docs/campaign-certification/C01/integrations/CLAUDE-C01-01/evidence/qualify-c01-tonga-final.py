import datetime,hashlib,json,os,pathlib,subprocess,sys
base=pathlib.Path(__file__).resolve().parent;repo=base/'integration'
out=base/'evidence/C01-tonga-review-20260921/qualification-final'
out.mkdir(exist_ok=False)
def git(*args):return subprocess.check_output(['git','-c','core.longpaths=true',*args],cwd=repo,text=True).strip()
def sha(p):return hashlib.sha256(p.read_bytes()).hexdigest()
pin=git('rev-parse','HEAD');assert pin=='fdb6d2c79209f3c441b8886a44f8e205c608452a' and not git('status','--porcelain')
env=dict(os.environ,SPHERES_EXPECTED_REVISION=pin,SPHERES_BROWSER_CHANNEL='chrome',SPHERES_RESEARCH_REVIEW_OUTPUT=str(out/'browser'))
checks=[('index',[sys.executable,'-X','utf8','tools/avatars/campaign_research.py','--check']),
('tonga',[sys.executable,'-X','utf8','-m','unittest','discover','-s','tools/avatars','-p','test_tonga_*.py']),
('campaign',[sys.executable,'-X','utf8','-m','unittest','discover','-s','tools/avatars','-p','test_campaign*.py']),
('atlas-unit',['node','--test','tools/ui/check_leadership_research_review.cjs']),
('atlas-browser',['node','tools/ui/ci-leadership-research-browser.cjs']),
('workboard',[sys.executable,'-X','utf8','tools/planning/workboard.py','--check'])]
record={'source_revision':pin,'started_utc':datetime.datetime.now(datetime.timezone.utc).isoformat(),'clean_before':True,'runner_sha256':sha(pathlib.Path(__file__)),'scope':'Tonga research intake and standalone read-only atlas. No campaign runtime changes.','checks':[]}
for name,command in checks:
    log=out/(name+'.log')
    print('START '+name,flush=True)
    with log.open('xb') as f:p=subprocess.run(command,cwd=repo,env=env,stdout=f,stderr=subprocess.STDOUT,creationflags=subprocess.CREATE_NO_WINDOW)
    record['checks'].append({'name':name,'command':command,'exit_code':p.returncode,'log':log.name,'sha256':sha(log),'passed':p.returncode==0})
    print('FINISH '+name+' '+str(p.returncode),flush=True)
    if p.returncode:print(log.read_text(encoding='utf8',errors='replace')[-5000:]);break
record.update(finished_utc=datetime.datetime.now(datetime.timezone.utc).isoformat(),revision_after=git('rev-parse','HEAD'),clean_after=not git('status','--porcelain'))
record['passed']=len(record['checks'])==len(checks) and all(x['passed'] for x in record['checks']) and record['revision_after']==pin and record['clean_after'] and record['runner_sha256']==sha(pathlib.Path(__file__))
assert not git('diff','--name-only','939e8f9',pin,'--','spheres-sim','spheres-web','Cargo.toml','Cargo.lock','docs/planning/campaign-pathway.json')
record['runtime_and_roadmap_unchanged_since']='939e8f9b3467ffc1666fea19fd4670257ea1e612'
(out/'result.json').write_text(json.dumps(record,indent=2)+'\n',encoding='utf8')
print(json.dumps(record,indent=2));sys.exit(0 if record['passed'] else 1)
