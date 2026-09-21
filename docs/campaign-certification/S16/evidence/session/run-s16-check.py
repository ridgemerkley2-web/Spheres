"""Source-pinned S16 qualification; new logs only, no repository mutation."""
import datetime, hashlib, json, os, pathlib, re, subprocess, sys
base=pathlib.Path(__file__).resolve().parent;repo=base/'integration'
pin,lane,label=sys.argv[1:4]
assert re.fullmatch('[0-9a-f]{40}',pin) and re.fullmatch('[A-Za-z0-9_-]+',label)
stem=base/'evidence'/('S16-'+label)
def git(*args):return subprocess.check_output(['git','-c','core.longpaths=true',*args],cwd=repo,text=True).strip()
def sha(p):
    with open(p,'rb') as f:return hashlib.file_digest(f,'sha256').hexdigest()
def now():return datetime.datetime.now(datetime.timezone.utc).isoformat()
assert git('rev-parse','HEAD')==pin and not git('status','--porcelain')
assert not stem.with_suffix('.log').exists() and not stem.with_suffix('.json').exists()
env=os.environ.copy();env.update(CARGO_TARGET_DIR=str(base/'integration-target'),CARGO_BUILD_JOBS='4')
targets=['ground_equipment_integration','military_operations','campaign_operations','equipment_integration','company_refits','equipment_supply_automation','company_ammunition','ammunition_reserves','aviation_ammunition','campaign_movement_audit','campaign_peace_audit','companies_integration','s08_supplier_lifecycle']
commands={
 'sim':['cargo','test','--locked','--release','-p','spheres-sim','--lib','--no-fail-fast'],
 'integration':['cargo','test','--locked','--release','-p','spheres-sim',*[arg for target in targets for arg in ['--test',target]],'--no-fail-fast'],
 'web':['cargo','test','--locked','--release','-p','spheres-web','--no-fail-fast'],
 'node':['node','tools/ui/run-unit.cjs'],
 'binary':['cargo','build','--locked','--release','-p','spheres-web'],
 'fixture':['cargo','test','--locked','--release','-p','spheres-web','s16_export_disposable_air_defense_fixture','--','--ignored','--nocapture'],
 'browser':['node','tools/ui/ci-air-defense.cjs'],
}
exe=base/'integration-target/release/spheres-web.exe'
proof={'revision':pin,'lane':lane,'clean_before':True,'command':commands[lane],'runner_sha256':sha(__file__),'started_utc':now()}
if lane in ['fixture','browser']:
    fixture=pathlib.Path(sys.argv[4]).resolve();env['SPHERES_S16_FIXTURE_DIR']=str(fixture);proof['fixture']=str(fixture)
if lane=='browser':
    env.update(SPHERES_BINARY=str(exe),SPHERES_EXPECTED_REVISION=pin,SPHERES_BROWSER_CHANNEL='chrome',SPHERES_S16_BROWSER_OUTPUT=str(base/'evidence'/('S16-browser-'+label)))
    proof['binary_sha256_before']=sha(exe)
with stem.with_suffix('.log').open('xb') as f:
    p=subprocess.run(commands[lane],cwd=repo,env=env,stdout=f,stderr=subprocess.STDOUT,creationflags=subprocess.CREATE_NO_WINDOW if os.name=='nt' else 0)
proof.update(exit_code=p.returncode,finished_utc=now(),revision_after=git('rev-parse','HEAD'),clean_after=not git('status','--porcelain'),log_sha256=sha(stem.with_suffix('.log')))
log=stem.with_suffix('.log').read_text(encoding='utf8',errors='replace')
if lane in ['sim','integration','web','fixture']:
    vals=[tuple(map(int,m)) for m in re.findall(r'test result: (?:ok|FAILED)\. (\d+) passed; (\d+) failed; (\d+) ignored;',log)]
    proof['totals']={'passed':sum(v[0] for v in vals),'failed':sum(v[1] for v in vals),'ignored':sum(v[2] for v in vals),'completed_targets':len(vals)}
    names=list(dict.fromkeys(re.findall(r'Running unittests[^\n]*\(([^\n]+spheres_(?:web|sim)-[^\n]+\.exe)\)',log)))
    proof['test_binaries']=[{'path':n,'sha256':sha(n)} for n in names]
if lane=='node':proof['totals']={k:int(re.findall(r'(?:ℹ|#)\s+'+k+r'\s+(\d+)',log)[-1]) for k in ['tests','pass','fail','skipped'] if re.findall(r'(?:ℹ|#)\s+'+k+r'\s+(\d+)',log)}
if lane in ['binary','browser']:proof['binary']={'path':str(exe),'sha256':sha(exe)}
proof['passed']=p.returncode==0 and proof['revision_after']==pin and proof['clean_after'] and sha(__file__)==proof['runner_sha256']
if lane in ['sim','integration','web','fixture']:proof['passed'] &= proof['totals']['passed']>0 and proof['totals']['failed']==0
if lane=='node':proof['passed'] &= proof['totals'].get('pass',0)>0 and proof['totals'].get('fail')==0
if lane=='browser':proof['passed'] &= proof['binary_sha256_before']==proof['binary']['sha256']
stem.with_suffix('.json').write_text(json.dumps(proof,indent=2)+'\n',encoding='utf8');print(json.dumps(proof,indent=2),flush=True)
sys.exit(0 if proof['passed'] else 1)

