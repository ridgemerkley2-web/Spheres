from pathlib import Path
import sys,subprocess,time,json,hashlib,os
base=Path(__file__).resolve().parent;root=base.parent/'claude-c06-review-20261002'
out=root/'docs/campaign-certification/C06/registration-completion-20261002/checks'
out.mkdir(parents=True,exist_ok=True)
env=os.environ.copy();env['CARGO_TARGET_DIR']=str(base/'native-target');env['CARGO_BUILD_JOBS']='2'
commands={
 'native':['cargo','test','--offline','--locked','--release','--workspace','--no-fail-fast','--','--skip','tests::the_resource_pass_stays_under_budget'],
 'resource-timing':['cargo','test','--offline','--locked','--release','-p','spheres-sim','--lib','tests::the_resource_pass_stays_under_budget','--','--exact','--nocapture','--test-threads=1'],
 'web-build':['cargo','build','--offline','--locked','--release','-p','spheres-web'],
 'portrait-boundary-positive':['cargo','test','--offline','--locked','--release','-p','spheres-web','person_portraits::tests::registered_cartoon_eras_resolve_exact_assets_at_both_boundaries','--','--exact','--nocapture'],
 'web-unit':['node','tools/ui/run-unit.cjs'],
 'avatar-suite':[sys.executable,'-B','-X','utf8','-m','unittest','discover','-s','tools/avatars'],
 'planning-suite':[sys.executable,'-B','-X','utf8','-m','unittest','discover','-s','tools/planning'],
 'avatar-allowlist':[sys.executable,'-B','-X','utf8','tools/avatars/build_person_avatar_assets.py','--check'],
 'registered-art':[sys.executable,'-B','-X','utf8','tools/avatars/person_art_pipeline.py','validate'],
 'art-self-test':[sys.executable,'-B','-X','utf8','tools/avatars/person_art_pipeline.py','self-test'],
 'leadership-production':[sys.executable,'-B','-X','utf8','tools/avatars/leadership_production.py','check'],
 'cartoon-review':[sys.executable,'-B','-X','utf8','tools/avatars/cartoon_review.py','--check'],
 'campaign-census':[sys.executable,'-B','-X','utf8','tools/avatars/campaign_census.py','--check'],
 'gap-ledger':[sys.executable,'-B','-X','utf8','tools/avatars/certified_gap_ledger.py','--check'],
 'boundary-matrix':[sys.executable,'-B','-X','utf8','tools/avatars/certified_boundary_matrix.py','--check'],
 'tonga-coverage':[sys.executable,'-B','-X','utf8','tools/avatars/tonga_cast_coverage.py'],
 'workboard':[sys.executable,'-B','-X','utf8','tools/planning/workboard.py','--check'],
 'original-render-integrity':[sys.executable,'-B','-X','utf8','docs/campaign-certification/C06/render-return-20261002/verify_outputs.py','--require-originals'],
}
names=sys.argv[1:];failed=False
for name in names:
 command=commands[name];log=out/(name+'.log');suffix=1
 while log.exists():suffix+=1;log=out/(name+'-'+str(suffix)+'.log')
 start=time.monotonic()
 with log.open('wb') as f:code=subprocess.run(command,cwd=root,env=env,stdout=f,stderr=subprocess.STDOUT).returncode
 data={'name':name,'command':command,'source_revision':subprocess.check_output(['git','rev-parse','HEAD'],cwd=root,text=True).strip(),'exit_code':code,'elapsed_seconds':round(time.monotonic()-start,2),'log':log.relative_to(root).as_posix(),'log_sha256':hashlib.sha256(log.read_bytes()).hexdigest(),'cargo_target':env['CARGO_TARGET_DIR'] if command[0]=='cargo' else None}
 log.with_suffix('.json').write_text(json.dumps(data,indent=2)+'\n',encoding='utf-8',newline='\n')
 print(json.dumps({**data,'tail':log.read_text(encoding='utf-8',errors='replace')[-1300:]},ensure_ascii=False),flush=True)
 failed=failed or code!=0
sys.exit(int(failed))
