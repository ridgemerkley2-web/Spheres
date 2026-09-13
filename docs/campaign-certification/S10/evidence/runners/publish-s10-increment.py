import datetime,gzip,hashlib,importlib.util,json,pathlib,subprocess
base=pathlib.Path(__file__).resolve().parent;repo=base/'integration';dest=repo/'docs/campaign-certification/S10'
pin='7aaa4517fbcc1c256481c2eefc5ef5e1c4153303'
def sha(p):
 with open(p,'rb') as f:return hashlib.file_digest(f,'sha256').hexdigest()
def read(p):return json.loads(p.read_text(encoding='utf-8-sig'))
def write(p,v):p.write_text(json.dumps(v,ensure_ascii=False,indent=2)+'\n',encoding='utf-8')
proofs=['final-web-1','final-node-1','final-agency-1','final-binary-1','performance-2000-2','performance-tonga-1']
for label in proofs:
 value=read(base/f'evidence/S10-{label}.json');assert value['passed'] and value['revision']==pin
 assert sha(base/f'evidence/S10-{label}.log')==value['log_sha256']
browserdir=base/'evidence/S10-browser-final-2/france-XCMBuc'
browser=read(browserdir/'result.json');assert browser['passed'] and browser['build']['revision']==pin
assert read(base/'evidence/S10-browser-driver-final-2.json')['passed']
assert read(base/'evidence/S10-preservation-closeout.json')['passed']
launch=read(base/'S10-review-launch.json')
assert launch['runtime_revision']==pin and launch['executable_sha256']==browser['build']['binary_sha256']
assert launch['native_verification']['exact_bytes_match_final_browser']
root_visual={'runtime_revision':pin,'source_result_sha256':sha(browserdir/'result.json'),'method':'Root visually inspected the two actual browser captures with view_image. No image transformation or alternate campaign state.',
 'images':[{'name':name,'sha256':sha(browserdir/name)} for name in ['policy-review-desktop.png','policy-review-narrow.png']],
 'findings':['Desktop review is readable over the existing diplomacy art with clear before/after and confirmation actions.','390px review wraps into a tall scrollable table. Bottom controls require scrolling and were reached by the real browser cancellation path. No horizontal panel overflow.']}
write(base/'evidence/S10-policy-visual-review.json',root_visual)
selection=[]
def add(p,name):assert p.is_file() or p.is_dir();selection.append({'source':str(p),'name':name})
for label in proofs+['browser-driver-final-2']:
 for ext in ['json','log']:add(base/f'evidence/S10-{label}.{ext}',f'checks/{label}.{ext}')
for label in ['census-regeneration','census-check','census-tests']:add(base/f'evidence/S10-{label}.log',f'checks/{label}.log')
for label in ['preservation-closeout','visual-review','policy-visual-review']:add(base/f'evidence/S10-{label}.json',f'checks/{label}.json')
for name in ['result.json','progress.jsonl','server.log']:add(browserdir/name,'browser/'+name)
for p in sorted(browserdir.glob('*.png')):add(p,'browser/'+p.name)
add(browserdir/'archive-audit','browser/archive-audit')
add(browserdir/'server/audit-captures','browser/audit-captures')
add(browserdir/'server/saves/s10-government-decisions.json','browser/named-campaign.json')
add(base/'S10-review-launch.json','review/launch.json')
add(pathlib.Path(launch['save_copy']['copy']),'review/copied-campaign.json')
add(pathlib.Path(launch['directory'])/'native-verification','review/native-verification')
for name in ['run-s10-check.py','run-s10-browser.py','launch-s10-review.py','publish-s10-increment.py','verify-s08-preservation.py','collect-s09-evidence.py']:
 add(base/name,'runners/'+name)
for label in ['focused-native-1','focused-native-2','node-development-1','performance-2000-1','browser-1']:
 p=base/f'evidence/S10-{label}.log';add(p,'development-attempts/'+p.name)
 p=base/f'evidence/S10-{label}.json'
 if p.exists():add(p,'development-attempts/'+p.name)
failure=base/'evidence/S10-browser-browser-1/france-P9edYf'
for name in ['result.json','failure.png']:add(failure/name,'development-attempts/browser-context/'+name)
chosen=base/'S10-publication-selection.json';assert not chosen.exists();write(chosen,selection)
spec=importlib.util.spec_from_file_location('s10collector',base/'collect-s09-evidence.py');collector=importlib.util.module_from_spec(spec);spec.loader.exec_module(collector)
collector.collect(chosen,dest/'evidence')
inventory_path=dest/'evidence/inventory.json';inventory=read(inventory_path)
inventory['scope']='Selected S10.a Windows checks, real two-tab browser journey, complete native archive comparisons, visual observations, preserved development failures and separate review launch. S10 and C01 remain open; this is not G2 or CP1 certification.'
write(inventory_path,inventory)
manifest={'status':'increment_complete_parent_open','increment':'S10.a','parent_session':'S10','runtime_revision':pin,'browser_driver_revision':browser['test_source']['revision'],'binary_sha256':browser['build']['binary_sha256'],
 'completed_utc':datetime.datetime.now(datetime.timezone.utc).isoformat(),
 'checks':{'windows_web':read(base/'evidence/S10-final-web-1.json')['totals'],'windows_agency_succession':read(base/'evidence/S10-final-agency-1.json')['totals'],'windows_node':read(base/'evidence/S10-final-node-1.json')['totals'],'census_tests':7,'real_browser':True,'native_archive_comparisons':'Whole native campaign; no ignored paths. Review/cancel, stale refusal, Load/Continue.','protected_originals':8,'protected_worktrees':2},
 'browser_actions':browser['government_result'],'review':{'url':launch['url'],'player':launch['player'],'date':launch['date']},
 'performance':{'scope':'Native preview construction only, including cloned commands and full-world token; excludes HTTP/UI. Three warmups and21 samples per endpoint per save.','p95_limit_ms':300,'max_limit_ms':750,'cases':[{'nation':v['nation'],'date':v['date'],'rows':[{'kind':r['kind'],'p95_ms':r['p95_ms'],'max_ms':r['max_ms']} for r in v['rows']]} for label in ['performance-2000-2','performance-tonga-1'] for v in read(base/f'evidence/S10-{label}.json')['measurements']]},
 'coverage_limits':['C01 exhaustive organization/institution inventory remains incomplete.','S10 and G2 remain open; S11 was not started.','This increment has Windows qualification; Linux and full certified-country end-to-end sessions remain to be refreshed before closing S10.','The live browser journey is France on1Jan1990. Other government roles, accepted offers, conflict replies and peg caps use explicit native boundary fixtures.','Existing peg dispatch text still describes nominal penalties; the new review displays actual capped effects.','Visual observations retain a transient old toast and horizontally scrolling narrow government tabs.'],
 'development_failures_retained':['Rust borrow-check error in initial test setup, then repaired.','Mixed index.html line endings after edits; original CRLF convention restored.','Mature-save benchmark initially supplied a relative path; rerun with exact absolute source.','Browser driver initially used an owned one-page context; explicit shared context fixed the second-tab test.'],
 'evidence':{'inventory':'evidence/inventory.json','sha256':sha(inventory_path),'files':len(inventory['files']),'stored_bytes':inventory['stored_bytes']}}
write(dest/'manifest.json',manifest)
road=repo/'docs/planning/campaign-pathway.json';d=read(road);s=next(s for s in d['sessions'] if s['id']=='S10')
s['status']='in_progress';s['increments']=[{'id':'S10.a','status':'complete','title':'Reviewed political decisions and current-source census audit','evidence':'docs/campaign-certification/S10/manifest.json'}]
s['remaining']=['Complete the C01 organization and institution census prerequisite.','Finish the remaining S10 country/platform qualification before claiming G2.']
d['execution'].update(status='stopped',stop_boundary='S10.a');write(road,d)
print(json.dumps({'manifest':str(dest/'manifest.json'),'files':len(inventory['files']),'stored_bytes':inventory['stored_bytes']},indent=2))
