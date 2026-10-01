import copy,datetime,hashlib,json,os,pathlib,subprocess,sys,time
ROOT=pathlib.Path(r'D:\spheres-offload\codex-next-20260928\review-c01-38-20261001')
OUT=pathlib.Path(__file__).parent/'validation';OUT.mkdir(exist_ok=True)
assert not list(OUT.iterdir()),'Preserve existing validation attempts'
BASE='f3e18e8306a0a7b1098b91f53f00efbb30da7990'
def before(path):return json.loads(subprocess.check_output(['git','show',BASE+':'+path],cwd=ROOT))
path='docs/campaign-certification/C01/research/france.json'
old=before(path);new=json.loads((ROOT/path).read_text(encoding='utf-8'))
assert new['sources'][:len(old['sources'])]==old['sources']
assert new['organizations']==old['organizations'] and len(new['organizations'])==635
assert new['institutions'][0]==old['institutions'][0]
sources=new['sources'][len(old['sources']):];sids={s['id'] for s in sources};cids={c['id'] for s in sources for c in s['claims']}
assert len(sids)==22 and len(cids)==33
restored=copy.deepcopy(new);restored['sources']=old['sources']
newpm=restored['institutions'][1];oldpm=old['institutions'][1]
assert newpm['sources']==oldpm['sources']+[s['id'] for s in sources]
assert newpm['claim_ids']==oldpm['claim_ids']+[c['id'] for s in sources for c in s['claims']]
newpm['sources']=oldpm['sources'];newpm['claim_ids']=oldpm['claim_ids']
oldrole=oldpm['roles'][0];role=newpm['roles'][0]
assert role['sources']==oldrole['sources']+[s['id'] for s in sources]
assert role['claim_ids']==oldrole['claim_ids']+[c['id'] for s in sources for c in s['claims']]
assert role['holder_claims'][:len(oldrole['holder_claims'])]==oldrole['holder_claims']
holders=role['holder_claims'][len(oldrole['holder_claims']):]
assert len(holders)==12 and len({h['name'] for h in holders})==9
assert all(h['from'] is None and h['until'] is None for h in holders)
role['sources']=oldrole['sources'];role['claim_ids']=oldrole['claim_ids'];role['holder_claims']=oldrole['holder_claims']
assert role['scope_note'].startswith(oldrole['scope_note']);role['scope_note']=oldrole['scope_note']
assert newpm['coverage']['unresolved'][:len(oldpm['coverage']['unresolved'])]==oldpm['coverage']['unresolved'];newpm['coverage']['unresolved']=oldpm['coverage']['unresolved']
assert restored['coverage']['unresolved'][:-1]==old['coverage']['unresolved'];restored['coverage']['unresolved']=old['coverage']['unresolved']
assert restored==old,'No other prior packet fields may change'
audit={'base':BASE,'submission':'acf33f09a5d28cbc9bf9f539e152e9846a7bf395','import':'9174c807',
 'prior_packet_reconstructed_exactly_after_removing_only_declared_additions':True,'prior_sources_identical':len(old['sources']),
 'financial_organizations_identical':635,'presidency_identical':True,'previous_pm_holder_observations_identical':len(oldrole['holder_claims']),
 'new_sources':len(sids),'new_claims':len(cids),'new_holder_observations':holders,'runtime_changed':False}
(OUT.parent/'scope-audit.json').write_text(json.dumps(audit,ensure_ascii=False,indent=2)+'\n',encoding='utf-8')
env=dict(os.environ,PYTHONDONTWRITEBYTECODE='1',GIT_OPTIONAL_LOCKS='0')
py=[sys.executable,'-B','-X','utf8']
commands=[('importer',py+['tools/avatars/import_cnccfp_census.py','--check']),
 ('research-regenerate',py+['tools/avatars/campaign_research.py']),('research-check',py+['tools/avatars/campaign_research.py','--check']),
 ('census',py+['tools/avatars/campaign_census.py','--check']),
 ('france',py+['-m','unittest','discover','-s','tools/avatars','-p','test_france*.py']),
 ('importer-tests',py+['-m','unittest','discover','-s','tools/avatars','-p','test_import_cnccfp_census.py']),
 ('research-tests',py+['-m','unittest','discover','-s','tools/avatars','-p','test_*research*.py']),
 ('campaign-tests',py+['-m','unittest','discover','-s','tools/avatars','-p','test_campaign*.py']),
 ('atlas',['node','--test','tools/ui/check_leadership_research_review.cjs']),
 ('workboard',py+['tools/planning/workboard.py','--check']),('diff-check',['git','diff','--check'])]
results=[]
for name,args in commands:
    start=time.monotonic();proc=subprocess.run(args,cwd=ROOT,env=env,capture_output=True);body=proc.stdout+proc.stderr
    (OUT/(name+'.log')).write_bytes(body)
    results.append({'name':name,'command':args,'exit_code':proc.returncode,'seconds':round(time.monotonic()-start,3),'log':name+'.log','sha256':hashlib.sha256(body).hexdigest()})
    print(name,proc.returncode,flush=True)
(OUT/'results.json').write_text(json.dumps({'source_revision':subprocess.check_output(['git','rev-parse','HEAD'],cwd=ROOT).decode().strip(),
 'checked_at_utc':datetime.datetime.now(datetime.timezone.utc).isoformat(),'commands':results,'passed':all(r['exit_code']==0 for r in results)},indent=2)+'\n',encoding='utf-8')
