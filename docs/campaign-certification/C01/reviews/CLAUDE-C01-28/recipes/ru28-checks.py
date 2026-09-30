import pathlib,subprocess,json,copy,hashlib
W=pathlib.Path('D:/spheres-offload/codex-next-20260928/review-ru28');E=W.parent/'ru28-review'
p='docs/campaign-certification/C01/research/russia.json';ref='fa19d1beecff97eb584cc267b8f83d22c0444fed'
d=json.loads((W/p).read_text(encoding='utf8'));raw=subprocess.check_output(['git','show',ref+':'+p],cwd=W);base=json.loads(raw)
before=json.loads(subprocess.check_output(['git','show','cc06f873^:'+p],cwd=W));assert before==base
ids={s['id'] for s in base['sources']};ss=[s for s in d['sources'] if s['id'] not in ids];assert len(ss)==68
roles={'ru_kprf_chairman','ru_ldpr_chairman','ru_yabloko_chairman','ru_apr_chairman','ru_dvr_chairman'}
orgs={'ru_apr_party_self_record','ru_dvr_party_self_record','ru_vybor_rossii_bloc_1993'}
t=copy.deepcopy(d);t['sources']=[s for s in t['sources'] if s['id'] in ids]
t['organizations']=[o for o in t['organizations'] if o['id'] not in orgs]
for o in t['organizations']:
    o['roles']=[r for r in o.get('roles',[]) if r['id'] not in roles]
    o['coverage']['unresolved']=[n for n in o['coverage']['unresolved'] if 'CLAUDE-C01-28' not in n]
t['coverage']['unresolved']=[n for n in t['coverage']['unresolved'] if 'CLAUDE-C01-28' not in n]
assert t==base
(E/'baseline-isolation.json').write_text(json.dumps({'passed':True,'accepted_integration_reference':ref,'pre_submission_packet_equals_accepted_integration':True,'removed_new_sources':[s['id'] for s in ss],'removed_new_roles':sorted(roles),'removed_new_organizations':sorted(orgs),'removed_only_C01_28_coverage_notes':True,'all_remaining_fields_equal':True,'SOURCE_05_and_other_inherited_Russia_content_preserved':True,'baseline_git_blob_sha256':hashlib.sha256(raw).hexdigest()},indent=2)+'\n')
commands=[('russia',['python','-B','-m','unittest','discover','-s','tools/avatars','-p','test_russia*.py']),('ussr',['python','-B','-m','unittest','discover','-s','tools/avatars','-p','test_ussr*.py']),('research',['python','-B','-m','unittest','discover','-s','tools/avatars','-p','test_*research*.py']),('campaign',['python','-B','-m','unittest','discover','-s','tools/avatars','-p','test_campaign*.py']),('atlas',['node','--test','tools/ui/check_leadership_research_review.cjs']),('index',['python','-B','tools/avatars/campaign_research.py','--check']),('census',['python','-B','tools/avatars/campaign_census.py','--check']),('workboard',['python','-B','tools/planning/workboard.py','--check'])]
rs=[]
for name,cmd in commands:
    with (E/(name+'.log')).open('wb') as f:r=subprocess.run(cmd,cwd=W,stdout=f,stderr=subprocess.STDOUT)
    rs.append({'name':name,'command':cmd,'exit_code':r.returncode,'log':name+'.log'});print(name,r.returncode,flush=True)
(E/'validation-commands.json').write_text(json.dumps(rs,indent=2)+'\n')
(E/'new-sources.json').write_text(json.dumps(ss,ensure_ascii=False,indent=2)+'\n',encoding='utf8')
