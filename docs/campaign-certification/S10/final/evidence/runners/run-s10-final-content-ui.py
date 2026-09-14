"""Qualify a clean S10.h static research/tooling revision without a game server."""
import datetime,hashlib,json,os,pathlib,re,subprocess,sys
BASE=pathlib.Path(__file__).resolve().parent
REPO=BASE/'integration'
PIN=sys.argv[1]
OUT=BASE/'evidence/S10-final-content-ui'
def git(*args):return subprocess.check_output(['git','-c','core.longpaths=true',*args],cwd=REPO,text=True).strip()
def sha(p):return hashlib.sha256(p.read_bytes()).hexdigest()
assert git('rev-parse','HEAD')==PIN and not git('status','--porcelain')
unchanged=git('diff','--name-only','d80abb8fb43957b77b5ab08023c2e66eb5fdda0e',PIN,'--','spheres-sim','spheres-web','Cargo.toml','Cargo.lock')
assert not unchanged,'No runtime changes are qualified by this tooling-only runner'
OUT.mkdir(exist_ok=False)
commands={
 'research-tests':['python','-X','utf8','-m','unittest','discover','-s','tools/avatars','-p','test_*research*.py'],
 'census-tests':['python','-X','utf8','-m','unittest','discover','-s','tools/avatars','-p','test_campaign_census.py'],
 'france-tests':['python','-X','utf8','-m','unittest','discover','-s','tools/avatars','-p','test_import_cnccfp_census.py'],
 'portrait-tests':['python','-X','utf8','-m','unittest','discover','-s','tools/avatars','-p','test_tupou_portrait_s10c.py'],
 'production':['python','-X','utf8','tools/avatars/leadership_production.py','check'],
 'art-assets':['python','-X','utf8','tools/avatars/build_person_avatar_assets.py','--check'],
 'census':['python','-X','utf8','tools/avatars/campaign_census.py','--check'],
 'france-importer':['python','-X','utf8','tools/avatars/import_cnccfp_census.py','--check'],
 'research-index':['python','-X','utf8','tools/avatars/campaign_research.py','--check'],
 'interface':['node','tools/ui/run-unit.cjs'],
 'browser':['node','tools/ui/ci-leadership-research-browser.cjs'],
}
proof={'format':'spheres-s10-final-content-ui/v1','source_revision':PIN,'source_clean_before':True,'started_utc':datetime.datetime.now(datetime.timezone.utc).isoformat(),'runner_sha256':sha(pathlib.Path(__file__)),
 'game_runtime_unchanged':True,'previous_publication':'d80abb8fb43957b77b5ab08023c2e66eb5fdda0e','game_runtime_revision':'0f4616477671bc01936fcc9f2877ae172b13e348','scope':'Windows content/interface and static browser research qualification. No campaign, Linux or new performance qualification claimed.','checks':[]}
env=dict(os.environ,SPHERES_EXPECTED_REVISION=PIN,SPHERES_RESEARCH_REVIEW_OUTPUT=str(OUT/'browser'),SPHERES_BROWSER_CHANNEL='chrome')
for name,command in commands.items():
 log=OUT/(name+'.log')
 with log.open('xb') as stream:process=subprocess.run(command,cwd=REPO,env=env,stdout=stream,stderr=subprocess.STDOUT,creationflags=subprocess.CREATE_NO_WINDOW)
 row={'name':name,'command':command,'exit_code':process.returncode,'log':log.name,'log_sha256':sha(log)}
 text=log.read_text(encoding='utf8',errors='replace')
 if name.endswith('-tests'):
  matches=re.findall(r'Ran (\d+) tests? in',text);row['tests']=int(matches[-1]) if matches else None
  row['reported_ok']=bool(re.search(r'^OK\s*$',text,re.M))
 if name=='interface':row['totals']={key:int(re.findall(r'ℹ '+key+r' (\d+)',text)[-1]) for key in ['tests','pass','fail','skipped']}
 proof['checks'].append(row)
 print(name,process.returncode,flush=True)
proof['source_revision_after']=git('rev-parse','HEAD');proof['source_clean_after']=not git('status','--porcelain')
proof['passed']=all(c['exit_code']==0 for c in proof['checks']) and proof['source_clean_after'] and proof['source_revision_after']==PIN
(OUT/'result.json').write_text(json.dumps(proof,indent=2)+'\n',encoding='utf8',newline='\n')
print(json.dumps(proof,indent=2));sys.exit(0 if proof['passed'] else 1)
