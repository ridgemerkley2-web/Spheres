import os,sys,subprocess,json,pathlib,hashlib,datetime
root=pathlib.Path(__file__).resolve().parent;repo=root/'integration';e=root/'evidence/S07-external-final';e.mkdir(exist_ok=False)
rev=subprocess.check_output(['git','rev-parse','HEAD'],cwd=repo,text=True).strip();assert rev==sys.argv[1]
assert not subprocess.check_output(['git','status','--porcelain'],cwd=repo)
binary=root/'integration-target/release/deps/spheres_web-960ae70db4690868.exe';sha=lambda p:hashlib.sha256(p.read_bytes()).hexdigest(); before=sha(binary)
env=os.environ.copy();env['SPHERES_S05_ACTIVE_FIXTURES']=str(root/'fixtures/s05-original-active');env['SPHERES_S05_MASTER_ARCHIVE']=str(root/'fixtures/s05-pinned-master/profile-campaigns/saves/profile-industry_and_war-0.json')
checks=['s05_master_migration_tests::actual_pinned_master_archive_preserves_property_history_and_next_day','s05_active_fixture_tests::original_active_company_stages_preserve_property_across_load_and_explicit_adoption','s05_save_matrix_tests::standalone_party_v1_preserves_real_supplier_books_for_equipment_versions_2_through_5']
proof={'candidate':rev,'binary':str(binary),'binary_sha256':before,'started_utc':datetime.datetime.now(datetime.timezone.utc).isoformat(),'checks':[]}
for i,name in enumerate(checks,1):
    with (e/f'{i}.log').open('wb') as log:r=subprocess.run([str(binary),name,'--ignored','--exact','--nocapture','--test-threads=1'],cwd=repo,env=env,stdout=log,stderr=subprocess.STDOUT)
    proof['checks'].append({'test':name,'exit_code':r.returncode,'log':f'{i}.log'})
    print(name+': exit '+str(r.returncode),flush=True)
proof['binary_unchanged']=sha(binary)==before;proof['finished_utc']=datetime.datetime.now(datetime.timezone.utc).isoformat();proof['candidate_after']=subprocess.check_output(['git','rev-parse','HEAD'],cwd=repo,text=True).strip();proof['clean_after']=not subprocess.check_output(['git','status','--porcelain'],cwd=repo)
(e/'result.json').write_text(json.dumps(proof,indent=2)+'\n',encoding='utf-8')
sys.exit(any(row['exit_code'] for row in proof['checks']))
