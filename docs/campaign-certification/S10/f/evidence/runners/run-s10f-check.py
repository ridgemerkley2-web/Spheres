import datetime, hashlib, json, os, pathlib, re, subprocess, sys
base=pathlib.Path(__file__).resolve().parent
repo=base/'integration'
pin,lane,label=sys.argv[1:4]
assert re.fullmatch('[0-9a-f]{40}',pin)
assert re.fullmatch('[A-Za-z0-9_-]+',label)
stem=base/'evidence'/('S10-'+label)
def git(*args): return subprocess.check_output(['git','-c','core.longpaths=true',*args],cwd=repo,text=True).strip()
def sha(path):
    h=hashlib.sha256()
    with open(path,'rb') as f:
        for b in iter(lambda:f.read(1024*1024),b''): h.update(b)
    return h.hexdigest()
assert git('rev-parse','HEAD')==pin and not git('status','--porcelain')
assert not stem.with_suffix('.log').exists() and not stem.with_suffix('.json').exists()
env=os.environ.copy(); env['CARGO_TARGET_DIR']=str(base/'integration-target')
commands={
 'web':['cargo','test','--locked','--release','-p','spheres-web','--no-fail-fast'],
 'leadership':['cargo','test','--locked','--release','-p','spheres-sim','--lib','party_leadership::','--no-fail-fast'],
 'agency':['cargo','test','--locked','--release','-p','spheres-sim','--test','agency','--test','s02_succession','--no-fail-fast'],
 'node':['node','tools/ui/run-unit.cjs'],
 'binary':['cargo','build','--locked','--release','-p','spheres-web'],
 'browser':['node','tools/ui/ci-government-decisions.cjs'],
 'performance':['cargo','test','--locked','--release','-p','spheres-web','--bin','spheres-web','decision_review::tests::mature_review_latency','--','--ignored','--nocapture'],
}
exe=base/'integration-target/release/spheres-web.exe'
proof={'revision':pin,'clean_before':True,'command':commands[lane],'runner_sha256':sha(__file__),'started_utc':datetime.datetime.now(datetime.timezone.utc).isoformat()}
if lane=='browser':
    env.update(SPHERES_BINARY=str(exe),SPHERES_EXPECTED_REVISION=pin,SPHERES_BROWSER_CHANNEL='chrome',SPHERES_DECISIONS_OUTPUT=str(base/'evidence'/('S10-browser-'+label)))
    proof['binary_sha256_before']=sha(exe)
if lane=='performance':
    source=pathlib.Path(sys.argv[4]); assert source.is_file()
    env['SPHERES_S10_PERF_SAVE']=str(source)
    proof['source']={'path':str(source),'bytes':source.stat().st_size,'sha256_before':sha(source)}
with stem.with_suffix('.log').open('xb') as out:
    p=subprocess.run(commands[lane],cwd=repo,env=env,stdout=out,stderr=subprocess.STDOUT,creationflags=subprocess.CREATE_NO_WINDOW if os.name=='nt' else 0)
proof.update(exit_code=p.returncode,finished_utc=datetime.datetime.now(datetime.timezone.utc).isoformat(),revision_after=git('rev-parse','HEAD'),clean_after=not git('status','--porcelain'),log_sha256=sha(stem.with_suffix('.log')))
log=stem.with_suffix('.log').read_text(encoding='utf-8',errors='replace')
if lane in ('web','agency','leadership','performance'):
    totals=[tuple(map(int,m)) for m in re.findall(r'test result: (?:ok|FAILED)\. (\d+) passed; (\d+) failed; (\d+) ignored;',log)]
    proof['totals']={'passed':sum(t[0] for t in totals),'failed':sum(t[1] for t in totals),'ignored':sum(t[2] for t in totals),'completed_targets':len(totals)}
    matches=list(dict.fromkeys(re.findall(r'Running unittests[^\n]*\(([^\n]+spheres_web-[^\n]+\.exe)\)',log)))
    if len(matches)==1: proof['test_binary']={'path':matches[0],'sha256':sha(matches[0])}
if lane=='node': proof['totals']={k:int(re.findall(r'ℹ '+k+r' (\d+)',log)[-1]) for k in ['tests','pass','fail','skipped'] if re.findall(r'ℹ '+k+r' (\d+)',log)}
if lane in ('binary','browser'): proof['binary']={'path':str(exe),'sha256':sha(exe)}
if lane=='performance':
    proof['source']['sha256_after']=sha(source)
    proof['measurements']=[json.loads(s.split('S10_REVIEW_CONTEXT ',1)[1]) for s in log.splitlines() if 'S10_REVIEW_CONTEXT ' in s]
proof['passed']=p.returncode==0 and proof['revision_after']==pin and proof['clean_after']
if lane=='browser': proof['passed'] &= proof['binary_sha256_before']==proof['binary']['sha256']
if lane=='performance': proof['passed'] &= proof['source']['sha256_before']==proof['source']['sha256_after'] and bool(proof['measurements'])
stem.with_suffix('.json').write_text(json.dumps(proof,indent=2)+'\n',encoding='utf-8')
print(json.dumps(proof,indent=2),flush=True)
sys.exit(0 if proof['passed'] else 1)
