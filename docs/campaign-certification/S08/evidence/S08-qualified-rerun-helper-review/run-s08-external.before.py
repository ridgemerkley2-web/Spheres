"""Run original archive checks against the exact successful S08 native test binary."""
import os,sys,re,subprocess,json,pathlib,hashlib,datetime,time
base=pathlib.Path(__file__).resolve().parent;repo=base/'integration';pin=sys.argv[1]
label=sys.argv[2] if len(sys.argv)>2 else 'final'
assert re.fullmatch('[0-9a-f]{40}',pin) and re.fullmatch('[A-Za-z0-9_-]+',label)
out=base/('evidence/S08-external-'+label)
def git(*args):return subprocess.check_output(['git','-c','core.longpaths=true',*args],cwd=repo,text=True).strip()
def sha(p):
 with pathlib.Path(p).open('rb') as handle:return hashlib.file_digest(handle,'sha256').hexdigest()
def stamp():return datetime.datetime.now(datetime.timezone.utc).isoformat()
assert git('rev-parse','HEAD')==pin and not git('status','--porcelain')
native_file=pathlib.Path(os.environ.get('SPHERES_S08_NATIVE_PROOF',base/'evidence/S08-final-native.json')).resolve()
native=json.loads(native_file.read_text(encoding='utf-8-sig'))
assert native['exit_code']==0 and native['clean_before'] and native['clean_after']
assert native['revision_before']==pin==native['revision_after']
binary=pathlib.Path(native['binary']).resolve();before=sha(binary)
assert before==native['binary_sha256'],'Native qualification binary changed'
out.mkdir(exist_ok=False)
env=os.environ.copy();env['SPHERES_S05_ACTIVE_FIXTURES']=str(base/'fixtures/s05-original-active');env['SPHERES_S05_MASTER_ARCHIVE']=str(base/'fixtures/s05-pinned-master/profile-campaigns/saves/profile-industry_and_war-0.json')
checks=['s05_master_migration_tests::actual_pinned_master_archive_preserves_property_history_and_next_day','s05_active_fixture_tests::original_active_company_stages_preserve_property_across_load_and_explicit_adoption','s05_save_matrix_tests::standalone_party_v1_preserves_real_supplier_books_for_equipment_versions_2_through_5']
proof={'candidate':pin,'clean_before':True,'binary':str(binary),'binary_sha256':before,'native_proof':str(native_file),'native_proof_sha256':sha(native_file),'runner_sha256':sha(__file__),'started_utc':stamp(),'checks':[],'passed':False}
try:
 for i,name in enumerate(checks,1):
  command=[str(binary),name,'--ignored','--exact','--nocapture','--test-threads=1'];start=time.monotonic()
  logpath=out/f'{i}.log'
  with logpath.open('xb') as log:r=subprocess.run(command,cwd=repo,env=env,stdout=log,stderr=subprocess.STDOUT,creationflags=subprocess.CREATE_NO_WINDOW if os.name=='nt' else 0)
  text=logpath.read_text(encoding='utf-8',errors='replace')
  rows=[tuple(map(int,m)) for m in re.findall(r'test result: (?:ok|FAILED)\. (\d+) passed; (\d+) failed; (\d+) ignored;',text)]
  executed=rows==[(1,0,0)]
  proof['checks'].append({'test':name,'command':command,'exit_code':r.returncode,'exact_one_test_passed':executed,'wall_seconds':time.monotonic()-start,'log':logpath.name,'log_sha256':sha(logpath)})
  print(name+': exit '+str(r.returncode),flush=True)
except BaseException as error:proof['runner_error']=str(error)
finally:
 proof.update(binary_sha256_after=sha(binary),binary_unchanged=sha(binary)==before,finished_utc=stamp(),candidate_after=git('rev-parse','HEAD'),clean_after=not git('status','--porcelain'))
 proof['passed']=len(proof['checks'])==3 and all(row['exit_code']==0 and row['exact_one_test_passed'] for row in proof['checks']) and proof['binary_unchanged'] and proof['candidate_after']==pin and proof['clean_after'] and 'runner_error' not in proof
 with (out/'result.json').open('x',encoding='utf-8') as output:json.dump(proof,output,indent=2);output.write('\n')
 print(json.dumps(proof,indent=2),flush=True)
sys.exit(0 if proof['passed'] else 1)
