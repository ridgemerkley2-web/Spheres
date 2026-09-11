import os,sys,pathlib,json,hashlib,subprocess,datetime,time,re
source=pathlib.Path('/mnt/c/Users/ridge/Documents/Codex/2026-09-05/pick-up-the-spheres-game-on/work/campaign-certification/integration')
out=source.parent/'evidence/S06-linux-final'
out.mkdir(exist_ok=False)
pin=sys.argv[1]
env=dict(os.environ);env.update({'GIT_DIR':'/mnt/c/Users/ridge/Spheres/.git/worktrees/integration','GIT_WORK_TREE':str(source),'GIT_OPTIONAL_LOCKS':'0','CARGO_BUILD_JOBS':'4','GIT_CONFIG_COUNT':'1','GIT_CONFIG_KEY_0':'core.autocrlf','GIT_CONFIG_VALUE_0':'true'})
sha=lambda p:hashlib.sha256(pathlib.Path(p).read_bytes()).hexdigest()
stamp=lambda:datetime.datetime.now(datetime.timezone.utc).isoformat()
def write(name,value): (out/name).write_text(json.dumps(value,indent=2)+'\n',encoding='utf8')
def capture(args):return subprocess.check_output(args,cwd=source,env=env,text=True,stderr=subprocess.STDOUT)
def snapshot(name):
 d={'captured_utc':stamp(),'head':capture(['git','rev-parse','HEAD']).strip(),'status_porcelain':capture(['git','status','--porcelain']),'git_dir':env['GIT_DIR'],'git_work_tree':str(source)}
 write(name,d);assert d['head']==pin and not d['status_porcelain'],json.dumps(d)[:3000];return d
windows=lambda s:pathlib.Path('/mnt/'+s[0].lower()+'/'+s[3:].replace('\\','/'))
def fixture_snapshot(name):
 root=source.parent/'fixtures/s05-original-active';file=root/'active-fixture-manifest.json';manifest=json.loads(file.read_text(encoding='utf8'));items=[]
 assert manifest['source_revision']=='5f7f355502f17bd6bd8f0383a2d14f0024fa7884'
 for row in manifest['fixtures']:
  p=root/row['file'];actual=sha(p);assert actual==row['sha256'] and p.stat().st_size==row['bytes'];items.append({'path':str(p),'sha256':actual})
 assert len(items)==28
 for row in manifest['source_binaries']:
  p=windows(row['path']);actual=sha(p);assert actual==row['sha256'];items.append({'path':str(p),'sha256':actual})
 genfile=source.parent/'fixtures/s05-pinned-master/generation.json';gen=json.loads(genfile.read_text(encoding='utf8'))
 assert gen['source_revision']=='485c223f60d5ff6e46f6ae17164bf1ee3a8764d9'
 for field,expected in [('archive','archive_sha256'),('test_binary','test_binary_sha256')]:
  p=windows(gen[field]);actual=sha(p);assert actual==gen[expected];items.append({'path':str(p),'sha256':actual})
 d={'captured_utc':stamp(),'active_manifest_sha256':sha(file),'master_generation_sha256':sha(genfile),'items':items,'verification':'Explicit read-only Windows drive to /mnt mapping; recorded fixture/source hashes unchanged.'};write(name,d);return d
record={'format':'spheres-s06-linux-qualification','version':1,'candidate':pin,'started_utc':stamp(),'status':'running','qualification_scope':'Complete workspace native suite, full Node and three external archive/lifecycle checks on this exact clean runtime candidate. Dated money recording touches existing payment owners, so the full native suite is rerun.','source':str(source),'environment':{k:env[k] for k in ['GIT_DIR','GIT_WORK_TREE','GIT_OPTIONAL_LOCKS','GIT_CONFIG_COUNT','GIT_CONFIG_KEY_0','GIT_CONFIG_VALUE_0','CARGO_BUILD_JOBS','CARGO_HOME','RUSTUP_HOME','CARGO_TARGET_DIR','CARGO_TARGET_X86_64_UNKNOWN_LINUX_GNU_LINKER','CC','TMPDIR']},'checks':[]}
def persist():write('runner-result.json',record)
def run(name,args,extra=None):
 local=dict(env);local.update(extra or {});path=out/(name+'.log');began=stamp();clock=time.perf_counter()
 print('START '+name,flush=True)
 with path.open('wb') as log:process=subprocess.run(args,cwd=source,env=local,stdout=log,stderr=subprocess.STDOUT)
 text=path.read_text(encoding='utf8',errors='replace');result={'name':name,'command':args,'extra_environment':extra or {},'started_utc':began,'elapsed_seconds':time.perf_counter()-clock,'exit_code':process.returncode,'log':path.name,'log_sha256':sha(path),'source_after':snapshot(name+'-source-after.json')}
 if name=='native':
  totals=[tuple(map(int,m)) for m in re.findall(r'test result: (?:ok|FAILED)\. (\d+) passed; (\d+) failed; (\d+) ignored;',text)];result['totals']={'passed':sum(t[0] for t in totals),'failed':sum(t[1] for t in totals),'ignored':sum(t[2] for t in totals),'completed_targets':len(totals)}
 elif name=='node':
  result['summary_tail']=text[-1800:]
 record['checks'].append(result);persist();print('FINISH '+name+' exit='+str(process.returncode)+' seconds='+str(round(result['elapsed_seconds'],2)),flush=True)
 if process.returncode:print('FAILURE TAIL\n'+text[-3500:],flush=True)
 return result
try:
 record['source_before']=snapshot('source-before.json');diff=capture(['git','diff','--name-only','1b985107ef3b5c6aa31fec130568d0286a126f5d',pin,'--','*.rs']).splitlines();record['rust_diff_from_full_candidate']=diff;record['fixtures_before']=fixture_snapshot('fixtures-before.json')
 tools='\n'.join(capture(cmd) for cmd in [['uname','-a'],['cat','/etc/os-release'],['rustc','-Vv'],['cargo','-Vv'],['node','-v'],['npm','-v'],[env['CC'],'--version']]);(out/'toolchain-versions.log').write_text(tools,encoding='utf8');record['toolchain_log_sha256']=sha(out/'toolchain-versions.log');persist()
 run('native',['cargo','test','--locked','--release','--workspace','--no-fail-fast'])
 native=(out/'native.log').read_text(encoding='utf8',errors='replace');found=re.findall(r'Running unittests src/main\.rs \(([^\r\n)]*/spheres_web-[0-9a-f]+)\)',native);found=list(dict.fromkeys(found))
 if len(found)==1:
  binary=pathlib.Path(found[0]);record['web_test_binary']={'path':str(binary),'bytes':binary.stat().st_size,'sha256':sha(binary)}
 else:record['web_test_binary_missing_or_ambiguous']=found
 persist()
 run('node',['node','tools/ui/run-unit.cjs'])
 if len(found)==1:
  tests=[('master-archive','s05_master_migration_tests::actual_pinned_master_archive_preserves_property_history_and_next_day',{'SPHERES_S05_MASTER_ARCHIVE':str(source.parent/'fixtures/s05-pinned-master/profile-campaigns/saves/profile-industry_and_war-0.json')}),('active-fixtures','s05_active_fixture_tests::original_active_company_stages_preserve_property_across_load_and_explicit_adoption',{'SPHERES_S05_ACTIVE_FIXTURES':str(source.parent/'fixtures/s05-original-active')}),('party-versions','s05_save_matrix_tests::standalone_party_v1_preserves_real_supplier_books_for_equipment_versions_2_through_5',{'SPHERES_S05_ACTIVE_FIXTURES':str(source.parent/'fixtures/s05-original-active')})]
  for name,test,extra in tests:run(name,[str(binary),test,'--exact','--ignored','--nocapture','--test-threads=1'],extra)
  record['web_test_binary']['sha256_after']=sha(binary);assert record['web_test_binary']['sha256']==record['web_test_binary']['sha256_after']
 record['fixtures_after']=fixture_snapshot('fixtures-after.json');assert record['fixtures_before']['items']==record['fixtures_after']['items'];record['source_after']=snapshot('source-after.json')
 record['status']='passed' if len(record['checks'])==5 and all(x['exit_code']==0 for x in record['checks']) else 'failed';record['finished_utc']=stamp();persist();print('FINAL '+record['status'],flush=True)
except BaseException as exc:
 record['status']='runner_error';record['error']=str(exc)[:5000];record['finished_utc']=stamp();persist();print('RUNNER ERROR '+record['error'],flush=True);raise
