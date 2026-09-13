import datetime, hashlib, json, pathlib, subprocess, sys
base=pathlib.Path(__file__).resolve().parent; repo=base/'integration'; pin=sys.argv[1]
def git(*args):return subprocess.check_output(['git','-c','core.longpaths=true',*args],cwd=repo,text=True).strip()
def sha(path):return hashlib.sha256(path.read_bytes()).hexdigest()
assert git('rev-parse','HEAD')==pin and not git('status','--porcelain')
out=base/'evidence/S10d-content-final';out.mkdir(exist_ok=False)
commands={
 "japan-tests":["python", "-X", "utf8", "-m", "unittest", "discover", "-s", "tools/avatars", "-p", "test_japan_research_s10d.py"],
 'census-tests':['python','-X','utf8','-m','unittest','discover','-s','tools/avatars','-p','test_campaign*.py'],
 'france-tests':['python','-X','utf8','-m','unittest','discover','-s','tools/avatars','-p','test_import_cnccfp_census.py'],
 'portrait-tests':['python','-X','utf8','-m','unittest','discover','-s','tools/avatars','-p','test_tupou_portrait_s10c.py'],
 'production':['python','-X','utf8','tools/avatars/leadership_production.py','check'],
}
for name in ['build_person_avatar_assets','campaign_census','import_cnccfp_census','campaign_research']:
 commands[name]=['python','-X','utf8','tools/avatars/'+name+'.py','--check']
proof={'revision':pin,'started_utc':datetime.datetime.now(datetime.timezone.utc).isoformat(),'runner_sha256':sha(pathlib.Path(__file__)),'checks':[]}
for name,command in commands.items():
 log=out/(name+'.log')
 with log.open('xb') as stream:process=subprocess.run(command,cwd=repo,stdout=stream,stderr=subprocess.STDOUT,creationflags=subprocess.CREATE_NO_WINDOW)
 proof['checks'].append({'name':name,'command':command,'exit_code':process.returncode,'log':log.name,'log_sha256':sha(log)})
proof['clean_after']=not git('status','--porcelain');proof['revision_after']=git('rev-parse','HEAD')
proof['passed']=all(c['exit_code']==0 for c in proof['checks']) and proof['clean_after'] and proof['revision_after']==pin
(out/'result.json').write_text(json.dumps(proof,indent=2)+'\n',encoding='utf8')
print(json.dumps(proof,indent=2));sys.exit(0 if proof['passed'] else 1)
