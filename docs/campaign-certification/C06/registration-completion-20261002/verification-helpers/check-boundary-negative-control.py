from pathlib import Path
import hashlib,json,os,subprocess,sys,time

base=Path(__file__).resolve().parent
root=base.parent/'claude-c06-review-20261002'
source=root/'spheres-web/src/person_portraits.rs'
out=root/'docs/campaign-certification/C06/registration-completion-20261002/checks'
before=source.read_bytes()
needle=b'exact_date(end) && date < end))'
assert before.count(needle)==1
changed=before.replace(needle,b'exact_date(end) && date <= end))',1)
env=os.environ.copy()
env['CARGO_TARGET_DIR']=str(base/'native-target')
env['CARGO_BUILD_JOBS']='2'
cmd=['cargo','test','--offline','--locked','--release','-p','spheres-web',
     'person_portraits::tests::registered_cartoon_eras_resolve_exact_assets_at_both_boundaries',
     '--','--exact','--nocapture']
log=out/'portrait-boundary-expected-negative.log'
assert not log.exists(), 'Preserve the prior negative-control run; do not overwrite it.'
start=time.monotonic()
try:
    source.write_bytes(changed)
    with log.open('wb') as stream:
        code=subprocess.run(cmd,cwd=root,env=env,stdout=stream,stderr=subprocess.STDOUT).returncode
finally:
    source.write_bytes(before)
assert source.read_bytes()==before
body=log.read_text(encoding='utf-8',errors='replace')
data={'name':'portrait-boundary-expected-negative','source_revision':subprocess.check_output(['git','rev-parse','HEAD'],cwd=root,text=True).strip(),
      'command':cmd,'mutation':'Change the historical portrait exclusive end comparison from date < end to date <= end.',
      'source_sha256_before':hashlib.sha256(before).hexdigest(),'mutated_sha256':hashlib.sha256(changed).hexdigest(),
      'restored_exact_bytes':source.read_bytes()==before,'exit_code':code,'elapsed_seconds':round(time.monotonic()-start,2),
      'expected_failure_observed':code==101 and 'assertion' in body and 'FAILED' in body,
      'log':log.relative_to(root).as_posix(),'log_sha256':hashlib.sha256(log.read_bytes()).hexdigest()}
log.with_suffix('.json').write_text(json.dumps(data,indent=2)+'\n',encoding='utf-8',newline='\n')
print(json.dumps({**data,'tail':body[-2000:]},indent=2))
sys.exit(0 if data['expected_failure_observed'] else 1)
