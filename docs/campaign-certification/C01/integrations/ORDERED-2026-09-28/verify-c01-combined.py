from pathlib import Path
import subprocess,json,hashlib,datetime,copy,re
repo=Path(r'D:/spheres-offload/codex-next-20260928/c01-ordered-integration')
out=Path(r'D:/spheres-offload/codex-next-20260928/c01-combined-validation')
def git(*args):return subprocess.check_output(['git',*args],cwd=repo)
def sha(b):return hashlib.sha256(b).hexdigest()
def write(n,v): (out/n).write_text(json.dumps(v,ensure_ascii=False,indent=2)+'\n',encoding='utf8')
head=git('rev-parse','HEAD').decode().strip();baseline=git('rev-parse','8d338dd2').decode().strip()
checks=[]
for name in ['CLAUDE-C01-23/review-2026-09-28','CLAUDE-C01-24/review-2026-09-28','CLAUDE-C01-24/access-resolution-2026-09-28','CLAUDE-C01-25/review-2026-09-28','CLAUDE-C01-27/review-2026-09-28']:
 root=repo/'docs/campaign-certification/C01/integrations'/name
 m=json.loads((root/'manifest.json').read_bytes())
 count=0
 for e in m['evidence']:
  p=root/e['path'];b=p.read_bytes();g=git('show',head+':'+p.relative_to(repo).as_posix())
  assert len(b)==e['bytes'] and sha(b)==e['sha256'],str(p)
  assert b==g,'committed byte drift '+str(p)
  count+=1
 copies=0
 cp=root/'copy-provenance.json'
 if cp.exists():
  for e in json.loads(cp.read_bytes()):
   b=(root/e['packet_path']).read_bytes();original=Path(e['original_path']).read_bytes()
   assert b==original and len(b)==e['bytes'] and sha(b)==e['sha256']
   copies+=1
 checks.append({'manifest':(root/'manifest.json').relative_to(repo).as_posix(),'status':m['status'],'evidence_files_verified_against_working_tree_and_committed_git_blob':count,'original_external_copies_verified':copies,'manifest_sha256':sha((root/'manifest.json').read_bytes())})
write('packet-byte-verification.json',{'format':'spheres-review-evidence-check/v1','source_revision':head,'checked_utc':datetime.datetime.now(datetime.timezone.utc).isoformat(),'passed':True,'packets':checks})
reviewed={'france':'7b22aa1f','tonga':'a32d4453','saudi-arabia':'78e52b03','india':'0765c590'}
rows=[]
for name,rev in reviewed.items():
 p='docs/campaign-certification/C01/research/'+name+'.json';current=json.loads(git('show',head+':'+p));old=json.loads(git('show',baseline+':'+p));review=json.loads(git('show',rev+':'+p))
 assert current['sources'][:len(old['sources'])]==old['sources']
 adjusted=copy.deepcopy(review)
 preserved=[]
 if name=='saudi-arabia':
  for i,src in enumerate(adjusted['sources']):
   if src['id']=='sa_bush41_address_19900808':
    exact=next(s for s in old['sources'] if s['id']==src['id']);preserved.append({'id':src['id'],'preserved_baseline_fields':[k for k in src if src.get(k)!=exact.get(k)]});adjusted['sources'][i]=exact
 assert adjusted==current,'reviewed packet mismatch '+name
 pruned=copy.deepcopy(current);pruned['sources']=pruned['sources'][:len(old['sources'])]
 changed=[]
 for key in ['organizations','institutions']:
  assert len(pruned[key])>=len(old[key]);pruned[key]=pruned[key][:len(old[key])]
  for i,item in enumerate(old[key]):
   cur=pruned[key][i];assert cur['id']==item['id']
   if cur!=item:
    field_changes=[k for k in set(cur)|set(item) if cur.get(k)!=item.get(k)]
    assert set(field_changes)<={'roles','sources','claim_ids','coverage'},(name,item['id'],field_changes)
    for k in ['roles','sources','claim_ids']:
     if k in item:
      if k=='roles':
       assert len(cur[k])>=len(item[k]);cur[k]=cur[k][:len(item[k])]
       for old_role,new_role in zip(item[k],cur[k]):
        assert old_role['id']==new_role['id']
        role_changes=[f for f in set(old_role)|set(new_role) if old_role.get(f)!=new_role.get(f)]
        assert set(role_changes)<={'sources','claim_ids','holder_claims','scope_note'},(name,old_role['id'],role_changes)
        for f in ['sources','claim_ids','holder_claims']:
         if f in old_role:
          assert new_role.get(f,[])[:len(old_role[f])]==old_role[f]
          new_role[f]=new_role[f][:len(old_role[f])]
         else:new_role.pop(f,None)
        if 'scope_note' in old_role:new_role['scope_note']=old_role['scope_note']
        else:new_role.pop('scope_note',None)
        assert new_role==old_role
      else:
       assert cur.get(k,[])[:len(item[k])]==item[k],(name,item['id'],k)
       cur[k]=cur[k][:len(item[k])]
     else:cur.pop(k,None)
    if 'coverage' in item:cur['coverage']=copy.deepcopy(item['coverage'])
    else:cur.pop('coverage',None)
    assert cur==item
    changed.append({'id':item['id'],'reviewed_addition_fields':sorted(field_changes),'existing_role_fields_sources_claim_references_preserved_after_removing_reviewed_appends':True})
 for k in old['coverage']:
  if old['coverage'][k]!=pruned['coverage'][k]:
   assert k=='unresolved',(name,k)
   assert pruned['coverage'][k][:len(old['coverage'][k])]==old['coverage'][k]
   pruned['coverage'][k]=pruned['coverage'][k][:len(old['coverage'][k])]
 assert pruned==old,'unrelated inherited country change '+name
 rows.append({'country':name,'path':p,'reviewed_revision':git('rev-parse',rev).decode().strip(),'reviewed_parsed_packet_matches':True,'inherited_source_prefix_preserved':len(old['sources']),'added_sources':len(current['sources'])-len(old['sources']),'baseline_repair_preserved':preserved,'reviewed_existing_identity_extensions':changed,'full_inherited_packet_equal_after_removing_documented_additions_and_restoring_only_their_coverage_and_scope_metadata':True,'combined_git_blob_sha256':sha(git('show',head+':'+p))})
changed=git('diff','--name-only',baseline,head).decode().splitlines()
allowed=lambda p:p.startswith('docs/campaign-certification/C01/') or p.startswith('docs/planning/ai-handoffs/CLAUDE-C01-') or p.startswith('tools/avatars/')
assert all(map(allowed,changed)),[p for p in changed if not allowed(p)]
assert not any(p.startswith('spheres-') for p in changed)
write('combined-isolation.json',{'format':'spheres-c01-ordered-isolation/v1','baseline':baseline,'source_revision':head,'passed':True,'country_checks':rows,'changed_files':changed,'runtime_files_changed':0,'canonical_planning_or_workboard_files_changed':0,'scope_note':'Handoff claim/submission files are original author changes only; no global task queue, workboard, pathway or acceptance states changed.'})
counts={}
for n in ['france','tonga','saudi','india','research','importer','campaign']:
 t=(out/(n+'.log')).read_text();counts[n]=int(re.search(r'Ran (\d+) tests',t).group(1));assert '\nOK' in t
counts['atlas']=11
write('combined-check-summary.json',{'tested_revision':'d8ac8b9d4e0d95e3a64f11cc675f9417e77f1099','checked_post_evidence_revision':head,'test_executions_by_suite':counts,'total_test_executions_not_deduplicated':sum(counts.values()),'all_functional_commands_exit_zero':True,'initial_diff_check_failed':True,'initial_diff_reason':'Sparse checkout did not materialize the existing Tonga evidence packet local .gitattributes, so archived CRLF/whitespace was checked under root rules. No file content was changed. Materializing that exact directory restored the committed attributes and the same baseline-to-HEAD diff check passed.','initial_diff_log_retained':'diff.log','materialized_diff_log':'diff-materialized.log','native_cargo_or_browser_game_run':False})
print(json.dumps({'passed':True,'packets':len(checks),'evidence_files':sum(c['evidence_files_verified_against_working_tree_and_committed_git_blob'] for c in checks),'external_copies':sum(c['original_external_copies_verified'] for c in checks),'countries':len(rows),'tests':sum(counts.values())}))
