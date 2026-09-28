import json,pathlib,hashlib,collections,math,subprocess
root=pathlib.Path(r'D:/spheres-offload/codex-next-20260928/a1-firing-observer')
src=root.parent/'a1-firing-observer-run-01';out=root.parent/'a1-firing-analysis-01'
result=json.loads((src/'result.json').read_text());windows=json.loads((out/'firing-windows.json').read_text());funds=json.loads((out/'firing-country-funding.json').read_text())
issues=[];triggers=collections.Counter();checks=collections.Counter(); needed_windows={}
for line,raw in enumerate((src/'observations.jsonl').open('rb'),1):
 r=json.loads(raw);a=r['army'];g=r['government'];f=r['fiscal'];stage=r['stage'];d=r['discontent']['total']
 if g is None:continue
 rec=g['record'];pen=0.
 if a['executive_leverage']>0 and rec is not None and rec['months']>=6 and rec['government']=='party:'+str(g['leader']) and d>=.25:
  pen=.65*a['executive_leverage']*(.5*d+.5*min(1,max(0,-rec['performance'])))*(1-a['public_mandate'])
 checks['confidence_rows']+=1
 if abs(pen-a['civilian_confidence_penalty'])>1e-14:issues.append({'line':line,'check':'confidence','expected':pen,'actual':a['civilian_confidence_penalty']})
 if a['target'] is not None and a['resources_per_member'] is not None:
  income=f['gdp_bn']*1000/f['population_m'];mat=.2+min(1,max(0,a['resources_per_member']/(2*(2500+1.5*max(0,income)))))*.65-a['war_exhaustion']*.45;target=min(1,max(0,mat-pen));checks['resource_target_rows']+=1
  if abs(target-a['target'])>1e-14:issues.append({'line':line,'check':'target','expected':target,'actual':a['target']})
 if stage.startswith('trigger_'):
  if not r['rules']['ideology_takeover']:expected='trigger_takeover_disabled'
  elif not g['has_army'] or g['settled_months']<12 or g['awaiting_first_election']:expected='trigger_unsettled_interim_or_no_army'
  elif a['effective_loyalty']>=.35 or d<.25:expected='trigger_live_conditions_inactive'
  elif a['pressure']<1/r['rules']['crisis_intensity']:expected='trigger_pressure_not_ready'
  else:expected='trigger_firing'
  checks['trigger_branches']+=1;triggers[stage]+=1
  if expected!=stage:issues.append({'line':line,'check':'trigger','expected':expected,'actual':stage})

cases=[]
for firing in result['firing_cases']:
 country=firing['country'];date=firing['date'];mi=date[0]*12+date[1]-1
 rows=[r for r in windows if r['country']==country and r['date']==date];before=next(r for r in rows if r['stage']=='army_before_walk');after=next(r for r in rows if r['stage']=='army_after_walk');reset=next(r for r in rows if r['stage']=='ai_before_funding')
 assert reset['army']['raw_loyalty']==.9 and reset['army']['pressure']==0 and reset['government']['months_in_office']==0 and not reset['government']['elected']
 preceding=[r for r in funds if r['country']==country and 0<mi-(r['date'][0]*12+r['date'][1]-1)<=12]
 q=firing['fiscal']['funding_quote'];assert q is not None and not q['clears_existing_hysteresis']
 cases.append({'country':country,'date':date,'trigger_line':next(r['line'] for r in rows if r['stage']=='trigger_firing'),'before_line':before['line'],'after_line':after['line'],'post_coup_line':reset['line'],'before_loyalty':before['army']['raw_loyalty'],'after_loyalty':after['army']['raw_loyalty'],'target':after['army']['target'],'confidence_penalty':after['army']['civilian_confidence_penalty'],'discontent':after['discontent']['total'],'pressure_before':before['army']['pressure'],'pressure_after':after['army']['pressure'],'settled_months':firing['government']['settled_months'],'affordable_military_share':firing['fiscal']['affordable_military_share'],'current_military_share':firing['fiscal']['mil_spend_gdp'],'funding_quote':q,'preceding_12_funding_decisions':dict(collections.Counter(r['reason'] for r in preceding)),'preceding_12_funding_changes':sum(r['changed'] for r in preceding),'post_coup_reset_verified':True})
source_files=[]
for rel in ['spheres-sim/src/government.rs','spheres-sim/src/government_a1_observer.rs','spheres-sim/src/politics.rs','spheres-sim/src/economy.rs','spheres-sim/src/clock.rs','spheres-sim/src/lib.rs']:
 raw=(root/rel).read_bytes();blob=subprocess.check_output(['git','show','6818e4f0d94b01c86d7a9acc4252260947d13504:'+rel],cwd=root);source_files.append({'path':str(root/rel),'repo_path':rel,'raw_bytes':len(raw),'raw_sha256':hashlib.sha256(raw).hexdigest(),'git_blob_bytes':len(blob),'git_blob_sha256':hashlib.sha256(blob).hexdigest(),'raw_equals_git_after_crlf_to_lf':raw.replace(b'\r\n',b'\n')==blob.replace(b'\r\n',b'\n')})
assert all(x['raw_equals_git_after_crlf_to_lf'] for x in source_files)
review={'checks':dict(checks),'issues':issues,'firing_cases':cases,'source_files':source_files,'recomputation_tolerance':1e-14,'not_an_independent_simulation':True}
out.joinpath('trigger-confidence-check.json').write_text(json.dumps(review,indent=2)+'\n',encoding='utf8',newline='\n')
print(json.dumps({'checks':dict(checks),'issues':issues,'cases':cases},indent=2))
