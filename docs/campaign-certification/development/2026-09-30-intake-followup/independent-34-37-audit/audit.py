import collections, datetime, hashlib, json, pathlib, subprocess
ROOT=pathlib.Path(r'C:/Users/ridge/Documents/Codex/2026-09-05/pick-up-the-spheres-game-on/work/campaign-certification/integration')
OUT=pathlib.Path(__file__).parent
PREFIX='docs/campaign-certification/C01/'
def git(*args):return subprocess.check_output(['git','-C',str(ROOT),*args])
def blob(ref,p):return git('show',f'{ref}:{p}')
def sha(b):return hashlib.sha256(b).hexdigest()
def pin(p):
 b=p.read_bytes();return {'path':str(p),'bytes':len(b),'sha256':sha(b)}
def read(p):return json.loads((ROOT/p).read_bytes())
def rows(m):return m.get('files',m.get('evidence',[]))
def check_pin(b,r):return len(b)==r['bytes'] and sha(b)==r['sha256']
report={'format':'spheres-integration-audit/v1','created_utc':datetime.datetime.now(datetime.timezone.utc).isoformat(),'reviewer':'Codex /root/s20_preflight','scope':'Read-only integration/provenance/metadata/CI-dependency audit; no new independent source-content review, no Cargo or test execution. C01-37 was authored-reviewed by this same agent; this audit is byte/schema/status preservation only. Future C01-29 work excluded.','head_start':git('rev-parse','HEAD').decode().strip(),'checks':{},'findings':[]}
configs={
 '34':('brazil','255a66c0317fca81fcbac1461b986aa2b029162d','645fb9ff9171b92694f164f72f76d5de16cb4334','e8f38113b6dff6b5376b657dc3905ec5204b7b9a'),
 '35':('ussr','6582e01ca1a0fc8a497e15cab186ee53eeb7f8e5','fac43ace04ca3acb30e270e2572135593430863a','9d97caa982f9dfc03aeda4fdb87d6d584dfad6a4'),
 '36':('tonga','7caba83835ab5385a9e9b581f89920a5d717328b','17c9e1ab94f6179b6f18b17f3e4da8f45bb327d2','44098c5a48f491f43fbcb74d121f290380af0b12'),
 '37':('france','73d748ef541d4d1970a07b9050115cf52bb2ecee','56fba2c0d74877f42ac0705fb54a7e340e81efa1','44098c5a48f491f43fbcb74d121f290380af0b12')}
def prior_leaves(a,b,path=''):
 errors=[]
 if type(a)!=type(b):return [path+': type differs']
 if isinstance(a,dict):
  for k,v in a.items():
   if k not in b:errors.append(path+'/'+k+': removed')
   else:errors.extend(prior_leaves(v,b[k],path+'/'+k))
 elif isinstance(a,list):
  if path.endswith('/holder_claims'):
   cursor=0
   for old in a:
    while cursor<len(b) and b[cursor]!=old:cursor+=1
    if cursor==len(b):errors.append(path+': original holder missing or order changed');break
    cursor+=1
   return errors
  if len(a)>len(b):errors.append(path+': shortened')
  for i,v in enumerate(a):
   if i<len(b):errors.extend(prior_leaves(v,b[i],path+'/'+str(i)))
 elif a!=b:errors.append(path+': changed scalar')
 return errors
for num,(country,review,receipt,base) in configs.items():
 p=PREFIX+'research/'+country+'.json'; b=(ROOT/p).read_bytes(); rb=blob(review,p); d=json.loads(b); old=json.loads(blob(base,p))
 folder=ROOT/(PREFIX+'integrations/CLAUDE-C01-'+num); m=json.loads((folder/'manifest.json').read_bytes()); om=json.loads((folder/'original-review-manifest.json').read_bytes())
 current_errors=[x['path'] for x in rows(m) if not check_pin((folder/x['path']).read_bytes(),x)]
 original_errors=[]
 for x in rows(om):
  target='original-review-README.md' if x['path']=='README.md' else x['path']
  if not check_pin((folder/target).read_bytes(),x):original_errors.append(x['path'])
 originals_match={name:(folder/('original-review-'+name)).read_bytes()==blob(receipt,PREFIX+'integrations/CLAUDE-C01-'+num+'/'+name) for name in ['README.md','manifest.json']}
 prior=prior_leaves(old,d)
 snapshot_checks=[]
 for source in d['sources']:
  snap=source.get('snapshot')
  if isinstance(snap,dict) and 'path' in snap:
   current=(ROOT/snap['path']).read_bytes(); reviewed=blob(review,snap['path'])
   snapshot_checks.append({'id':source['id'],'path':snap['path'],'exact_review_git_bytes':current==reviewed,'declared_pin_matches':check_pin(current,snap)})
 source_old={s['id']:s for s in old['sources']}; source_new={s['id']:s for s in d['sources']}
 preserved_old_sources=all(source_new.get(k)==v for k,v in source_old.items())
 result={'country':country,'reviewed_repaired_revision':review,'original_receipt_revision':receipt,'baseline_revision':base,'git_blob_equal_to_review':blob('HEAD',p)==rb,'json_equal_to_review':json.loads(b)==json.loads(rb),'current_working_bytes':pin(ROOT/p),'review_git_blob_bytes':len(rb),'review_git_blob_sha256':sha(rb),'current_manifest_payloads':len(rows(m)),'current_manifest_errors':current_errors,'original_manifest_payloads':len(rows(om)),'original_manifest_errors':original_errors,'original_receipt_bytes_equal_to_review':originals_match,'prior_source_count':len(source_old),'all_prior_sources_exact':preserved_old_sources,'prior_leaf_changes':prior,'source_snapshot_count':len(snapshot_checks),'source_snapshot_failures':[x for x in snapshot_checks if not x['exact_review_git_bytes'] or not x['declared_pin_matches']]}
 if prior or result['source_snapshot_failures']:report['findings'].append('C01-'+num+' earlier-data or snapshot mismatch')
 report['checks']['C01-'+num]=result
 if not all([result['git_blob_equal_to_review'],result['json_equal_to_review'],not current_errors,not original_errors,all(originals_match.values()),preserved_old_sources]):report['findings'].append('C01-'+num+' byte/isolation verification failure')
# SOURCE-17 retained reviewed extracts.
extract_dir=ROOT/(PREFIX+'integrations/CLAUDE-C01-SOURCE-17/reviewed-extracts')
report['checks']['SOURCE-17']=[{'filename':p.name,'reviewed_bytes':pin(p),'current_equal':p.read_bytes()==(ROOT/(PREFIX+'research/sources')/p.name).read_bytes()} for p in sorted(extract_dir.glob('*.json'))]
if not all(x['current_equal'] for x in report['checks']['SOURCE-17']):report['findings'].append('SOURCE-17 extract mismatch')
# Generated input pins. Exact raw bytes unless the producer explicitly declares text normalization.
for label,path in [('gap-ledger',PREFIX+'gap-ledger/ledger.json'),('boundary','docs/campaign-certification/S23/preparation/boundary-matrix/summary.json')]:
 d=read(path); errors=[]
 for row in d['inputs']:
  p=ROOT/row['path']
  if not row.get('exists',True):
   if p.exists():errors.append(row['path']+': appeared')
   continue
  b=p.read_bytes()
  if row.get('newline_normalized'):b=b.replace(b'\r\n',b'\n')
  if row.get('hash_encoding'):b=b.decode('utf8').replace('\r\n','\n').replace('\r','\n').encode('utf8')
  if not check_pin(b,row):errors.append(row['path'])
 report['checks'][label+'-inputs']={'count':len(d['inputs']),'errors':errors}
 if errors:report['findings'].append(label+' stale input pins')
ledger=read(PREFIX+'gap-ledger/ledger.json'); boundary=read('docs/campaign-certification/S23/preparation/boundary-matrix/summary.json'); queue=read('docs/planning/ai-task-queue.json'); index=read(PREFIX+'research-index.json')
expected={'CLAUDE-C01-'+x for x in ['01','02','03','04','07','08','23','24','25','27','30','32','33','34','35','36','37']}
actual={p['packet'] for p in boundary['packets'] if p['status']=='accepted'}
expected_recent={'CLAUDE-C01-'+x for x in ['23','24','25','27','30','32','33','34','35','36','37']}
complete={p['task'] for p in ledger['completed_research_intakes']}
qrows=queue.get('tasks',queue.get('queue',[])); relevant=[x for x in qrows if x['id'] in {'CLAUDE-C01-'+n for n in configs}]
report['checks']['acceptance']={'boundary_accepted':sorted(actual),'matches_expected':actual==expected,'gap_completed':sorted(complete),'matches_expected_recent':complete==expected_recent,'in_flight':[p['task'] for p in ledger['in_flight']],'unearned_parent_markers':{'s23_complete':boundary['s23_complete'],'c06_complete':boundary['c06_complete'],'c01_complete':index['c01_complete'],'g2_prerequisite_satisfied':index['g2_prerequisite_satisfied'],'runtime_roster_modified':index['runtime_roster_modified']},'no_runtime_or_full_history_acceptance':all(not x['runtime_mapping_accepted'] and not x['historical_period_complete'] for x in ledger['completed_research_intakes']),'queue_rows':relevant,'boundary_case_count':sum(i['statistics']['cases'] for c in boundary['cases'] for i in c['identities'])}
# New source attribution remains exact and all older source records are unchanged.
attr=read(PREFIX+'gap-ledger/inputs/source-attribution.json'); old_attr=json.loads(blob('HEAD',PREFIX+'gap-ledger/inputs/source-attribution.json')); attrchecks=[]
for num,(country,review,receipt,base) in configs.items():
 d=read(PREFIX+'research/'+country+'.json'); nation=d['nation']; before=json.loads(blob(base,PREFIX+'research/'+country+'.json')); old_ids={x['id'] for x in before['sources']}; newids={x['id'] for x in d['sources']}-old_ids
 expected_packet='CLAUDE-C01-'+num; wrong=[sid for sid in sorted(newids) if attr['sources'][nation][sid]['packet']!=expected_packet]
 older_changed=[sid for sid,row in old_attr['sources'][nation].items() if attr['sources'][nation].get(sid)!=row]
 commits=collections.Counter(attr['sources'][nation][sid]['commit'] for sid in newids)
 attrchecks.append({'packet':expected_packet,'new_source_count':len(newids),'attribution_commits':dict(commits),'wrong_packet_ids':wrong,'changed_old_attributions':older_changed})
 if wrong or older_changed:report['findings'].append(expected_packet+' attribution mismatch')
report['checks']['attribution']=attrchecks
# Hash exactly inspected pending metadata and CI evidence for replay of this audit scope.
paths=['.github/workflows/verify.yml','tools/avatars/requirements-test.txt','tools/avatars/person_art_pipeline.py','tools/avatars/test_tupou_portrait_s10c.py','docs/AI_WORKSTREAMS.md','docs/planning/ai-task-queue.json','docs/planning/ai-handoffs/CODEX-NEXT-ENGINEERING.md',PREFIX+'research-index.json',PREFIX+'gap-ledger/inputs/source-attribution.json',PREFIX+'gap-ledger/ledger.json',PREFIX+'gap-ledger/ledger.md','docs/campaign-certification/S23/preparation/boundary-matrix/summary.json','docs/campaign-certification/S23/preparation/boundary-matrix/README.md','tools/avatars/certified_gap_ledger.py','tools/avatars/test_certified_gap_ledger.py','tools/avatars/test_certified_boundary_matrix.py']
report['inspected_file_pins']=[pin(ROOT/p) for p in paths]
VD=pathlib.Path(r'D:/spheres-offload/codex-next-20260928/combined-intakes-34-37-validation')
report['validation_log_pins']=[pin(VD/n) for n in ['ci-3947-javascript-linux.log','ci-3947-javascript-windows.log','ci-3947-political-linux.log','clean-pillow-install.log','clean-tupou-tests.log','clean-certified-checks.log','clean-certified-checks-corrected.log','clean-certified-checks-final.log','workboard-check.log']]
report['head_end']=git('rev-parse','HEAD').decode().strip()
(OUT/'audit.json').write_text(json.dumps(report,indent=2,ensure_ascii=False)+'\n',encoding='utf8')
print(json.dumps({'head':report['head_end'],'countries':{k:{'exact_review':v['git_blob_equal_to_review'],'receipt_payloads':v['current_manifest_payloads'],'prior_changes':v['prior_leaf_changes']} for k,v in report['checks'].items() if k.startswith('C01-')},'input_pins':{k:v for k,v in report['checks'].items() if k.endswith('-inputs')},'attribution':attrchecks,'findings':report['findings'],'accepted':report['checks']['acceptance']['matches_expected'],'case_count':report['checks']['acceptance']['boundary_case_count']},indent=2))
