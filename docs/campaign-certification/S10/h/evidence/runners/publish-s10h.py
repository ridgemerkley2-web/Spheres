"""Publish evidence for the static research increment after all declared checks."""
import datetime,hashlib,json,pathlib,re,shutil,subprocess,sys
BASE=pathlib.Path(__file__).resolve().parent;REPO=BASE/'integration';PIN=sys.argv[1];DEST=REPO/'docs/campaign-certification/S10/h';QUAL=BASE/'evidence/S10h-qualified'
def read(p):return json.loads(p.read_text(encoding='utf8'))
def sha(p):return hashlib.sha256(p.read_bytes()).hexdigest()
def git(*args):return subprocess.check_output(['git','-c','core.longpaths=true',*args],cwd=REPO,text=True).strip()
def write(p,v):p.write_text(json.dumps(v,indent=2)+'\n',encoding='utf8',newline='\n')
assert git('rev-parse','HEAD')==PIN and not git('status','--porcelain') and not DEST.exists()
proof=read(QUAL/'result.json');assert proof['passed'] and proof['source_revision']==proof['source_revision_after']==PIN and proof['source_clean_before'] and proof['source_clean_after']
assert proof['runner_sha256']==sha(BASE/'qualify-s10h.py') and proof['game_runtime_unchanged']
assert not git('diff','--name-only',proof['previous_publication'],PIN,'--','spheres-sim','spheres-web','Cargo.toml','Cargo.lock')
assert len(proof['checks'])==11
content=0
for check in proof['checks']:
 log=QUAL/check['log'];text=log.read_text(encoding='utf8')
 assert check['exit_code']==0 and sha(log)==check['log_sha256']
 if check['name'].endswith('-tests'):
  count=int(re.findall(r'Ran (\d+) tests? in',text)[-1]);assert check['reported_ok'] and re.search(r'^OK\s*$',text,re.M) and count==check['tests'];content+=count
 if check['name']=='interface':
  node={k:int(re.findall(r'ℹ '+k+r' (\d+)',text)[-1]) for k in ['tests','pass','fail','skipped']};assert node==check['totals'] and node['fail']==0 and node['pass']>0
browser=read(QUAL/'browser/result.json');assert browser['passed'] and browser['clean_before'] and browser['clean_after'] and browser['revision']==browser['revision_after']==PIN
assert not browser['errors'] and browser['source_files_unchanged'] and len(browser['countries'])==9
assert all(r['method']=='GET' and '/api/' not in r['url'] for r in browser['requests'])
assert browser['keyboard']['summary_reached_with_tab'] and browser['keyboard']['opened_with_enter']
assert browser['empty_country']['party_count_matches_inventory'] and browser['empty_country']['undiscovered_organization_count']=='Unknown'
assert len(browser['authored_error_cases'])==2
for name,digest in browser['source_files'].items():assert sha(REPO/name)==digest
index=read(REPO/'docs/campaign-certification/C01/research-index.json');counts=index['counts']
assert counts=={'country_packets':9,'countries_without_new_discovery_packet':151,'organization_observations':841,'institution_observations':27,'sources':58,'source_claims':1606,'discovery_batches':92,'exhaustive_country_censuses':0}
countries=read(REPO/'docs/campaign-certification/C01/countries.json')
assert {c['id'] for c in countries if c['certified_case']}=={c['nation'] for c in index['countries']}
for r in browser['countries']:
 current=next(c for c in index['countries'] if c['nation']==r['nation'])
 assert r['first_claim_exact'] and r['organization_observations']==current['organization_observations'] and r['institution_observations']==current['institution_observations']
before=read(BASE/'evidence/S10h-preservation-before.json');after=read(BASE/'evidence/S10h-preservation-final.json');assert before['passed'] and after['passed'] and before['baseline_sha256']==after['baseline_sha256']
visual=read(BASE/'evidence/S10h-visual-review.json');assert visual['passed'] and visual['source_revision']==PIN
for item in visual['screenshots']:assert sha(pathlib.Path(item['path']))==item['sha256']
launch=read(BASE/'S10h-atlas-launch.json');assert launch['passed'] and launch['source_revision']==PIN
for name,digest in launch['served_assets'].items():assert sha(REPO/name)==digest
assert launch['server_script_sha256']==sha(BASE/'serve-s10h-atlas.py')
selected=[]
for source in sorted(QUAL.rglob('*')):
 if source.is_file():selected.append((source,'qualification/'+source.relative_to(QUAL).as_posix()))
for name in ['S10h-preservation-before.json','S10h-preservation-final.json','S10h-visual-review.json']:selected.append((BASE/'evidence'/name,'review/'+name))
selected.append((BASE/'S10h-atlas-launch.json','review/launch.json'))
for name in ['qualify-s10h.py','publish-s10h.py','verify-s10h-publication.py','serve-s10h-atlas.py','launch-s10h-atlas.py','verify-s08-preservation.py']:selected.append((BASE/name,'runners/'+name))
assert len({n for _,n in selected})==len(selected)
for source,name in selected:assert source.is_file() and not source.is_symlink()
DEST.mkdir();(DEST/'.gitattributes').write_text('evidence/** -text -whitespace\n',encoding='utf8',newline='\n');files=[]
for source,name in selected:
 target=DEST/'evidence'/name;target.parent.mkdir(parents=True,exist_ok=True);shutil.copyfile(source,target);assert target.read_bytes()==source.read_bytes()
 files.append({'path':name,'source':str(source),'bytes':target.stat().st_size,'sha256':sha(target)})
inventory={'format':'spheres-s10h-evidence/v1','files':files,'stored_bytes':sum(f['bytes'] for f in files),'scope':'Complete declared Windows content/interface logs, read-only research browser results and screenshots, preservation, manual visual review and server launch. Original source responses remain outside this evidence; checked-in factual extracts are in the pinned research packets.'}
write(DEST/'evidence/inventory.json',inventory)
manifest={'format':'spheres-s10h-increment/v1','status':'increment_complete_parent_open','created_utc':datetime.datetime.now(datetime.timezone.utc).isoformat(),'source_revision':PIN,
 'game_runtime_revision':proof['game_runtime_revision'],'game_runtime_changed':False,'historical_roles_or_gameplay_grants_changed':False,
 's10_complete':False,'c01_complete':False,'g2_earned':False,'s11_started':False,
 'qualification':{'content_tests':content,'content_and_reproduction_commands':9,'windows_interface':node,'browser_country_packets':len(browser['countries']),'browser_widths':[1440,390,320],'browser_read_only':True,'keyboard_verified':True,'authored_error_cases':browser['authored_error_cases'],'new_campaign_or_linux_or_performance_claim':False},
 'research_counts':counts,'review_url':launch['url'],'evidence':{'files':len(files),'stored_bytes':inventory['stored_bytes'],'inventory_sha256':sha(DEST/'evidence/inventory.json')}}
write(DEST/'manifest.json',manifest);print(json.dumps(manifest,indent=2))
