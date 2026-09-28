"""Read-only A1 geography analysis. No simulation. Accepts original or portable gzip observations."""
import argparse,collections,gzip,hashlib,json,math
from pathlib import Path

def pin(path):
 h=hashlib.sha256();n=0
 with path.open('rb') as f:
  while b:=f.read(1024*1024):h.update(b);n+=len(b)
 return {'path':str(path.resolve()),'bytes':n,'sha256':h.hexdigest()}
def witness(row,line,value):return {'date':row['date'],'line':line,'value':value}
def starting(country):
 return {'country':country,'checks':0,'firings':0,'actual_branches':collections.Counter(),'blocking_conditions_nonexclusive':collections.Counter(),'blocking_joint_conditions':collections.Counter(),'eligible_live_joint_conditions':collections.Counter(),'features':collections.Counter(),'extrema':{},'funding':collections.Counter(),'funding_examples':[]}
def extreme(d,key,value,r,line,is_min=False):
 if value is None:return
 if key not in d['extrema'] or (value < d['extrema'][key]['value'] if is_min else value > d['extrema'][key]['value']):d['extrema'][key]=witness(r,line,value)
def analyze(obs,result_path,out):
 out.mkdir(exist_ok=False)
 expected=json.loads(result_path.read_text(encoding='utf-8'));countries={};seen=set();before={};actions=[];stage_counts=collections.Counter();hashraw=hashlib.sha256();nraw=0;firing=[];alltriggers=[]
 def get(c):return countries.setdefault(c,starting(c))
 with (gzip.open(obs,'rb') if obs.suffix=='.gz' else obs.open('rb')) as f:
  for line,raw in enumerate(f,1):
   hashraw.update(raw);nraw+=len(raw);r=json.loads(raw);c=r['country'];seen.add(c);st=r['stage'];stage_counts[st]+=1;a=r['army'];g=r['government'];fi=r['fiscal'];key=(c,tuple(r['date']))
   if st=='ai_before_funding':before[key]=(r,line)
   elif st=='ai_after_funding':
    b,bl=before.pop(key);bf=b['fiscal'];q=bf['funding_quote'];low=b['army']['raw_loyalty'] is not None and b['army']['raw_loyalty']<(.5 if b['date'][1]==1 else .4)
    reason='not_low_loyalty' if not low else 'no_policy_floor' if q is None else 'no_useful_increase' if not q['clears_existing_hysteresis'] else 'no_standing' if not q['standing_affordable'] else 'paid_increase'
    d=get(c);d['funding'][reason]+=1
    if reason=='paid_increase':
     assert fi['mil_spend_gdp']==q['share']
     aa=b['army'];res=aa['resources_per_member'];material=None;compensating=None
     if res and bf['mil_spend_gdp']>0:
      allowance=2500+1.5*max(0,bf['gdp_bn']*1000/bf['population_m'])
      fraction=min(1,max(0,(.4-.2+.45*aa['war_exhaustion'])/.65))
      material=min(max(0,fraction*2*allowance*bf['mil_spend_gdp']/res),bf['affordable_military_share'],.35)
      compensating=max(0,q['share']-max(bf['mil_spend_gdp'],material))
     action={'country':c,'date':r['date'],'before_line':bl,'after_line':line,'electoral':b['government']['electoral'],'before_share':bf['mil_spend_gdp'],'after_share':fi['mil_spend_gdp'],'price_pc':q['price_pc'],'penalty':aa['civilian_confidence_penalty'],'before_target':aa['target'],'after_target':a['target'],'effective_loyalty':aa['effective_loyalty'],'discontent':b['discontent']['total'],'pure_algebra_material_only_floor':material,'paid_share_above_current_and_material_only_floor':compensating}
     actions.append(action)
     if b['government']['electoral']:d['funding']['electoral_paid_increase']+=1
     if compensating is not None and compensating>1e-10:
      d['funding']['positive_political_compensation_action']+=1
      d['funding_examples'].append(action)
   if not st.startswith('trigger_'):continue
   assert g['electoral'];d=get(c);d['checks']+=1;d['actual_branches'][st]+=1;loy=a['effective_loyalty'];dis=r['discontent']['total'];pressure=a['pressure'];threshold=1/max(.1,r['rules']['crisis_intensity'])
   blockers={'no_army':not g['has_army'],'unsettled':g['settled_months']<12,'interim':g['awaiting_first_election'],'loyalty_ge_0_35':loy>=.35,'discontent_lt_0_25':dis<.25,'pressure_below_threshold':pressure<threshold}
   present=[k for k,v in blockers.items() if v];d['blocking_conditions_nonexclusive'].update(present);d['blocking_joint_conditions']['+'.join(present) or 'none']+=1
   eligible=not any(blockers[k] for k in ['no_army','unsettled','interim']);crisis=dis>=.25;hostile=loy<.35;pen=a['civilian_confidence_penalty'];target=a['target']
   for k,v in {'has_army':g['has_army'],'crisis':crisis,'hostile':hostile,'penalty_positive':pen>0,'eligible':eligible,'eligible_crisis':eligible and crisis,'eligible_hostile':eligible and hostile,'eligible_both_live':eligible and crisis and hostile,'eligible_crisis_loyal':eligible and crisis and not hostile,'eligible_crisis_loyal_positive_penalty':eligible and crisis and not hostile and pen>0,'eligible_crisis_loyal_positive_penalty_target_ge_035':eligible and crisis and not hostile and pen>0 and target is not None and target>=.35,'eligible_crisis_loyal_positive_penalty_target_below_035':eligible and crisis and not hostile and pen>0 and target is not None and target<.35,'pressure_ge_threshold_but_live_inactive':eligible and pressure>=threshold and (not crisis or not hostile)}.items():
    if v:d['features'][k]+=1
   if eligible:d['eligible_live_joint_conditions'][('hostile' if hostile else 'loyal')+'+'+('crisis' if crisis else 'low_discontent')]+=1
   extreme(d,'min_effective_loyalty',loy,r,line,True);extreme(d,'min_raw_loyalty',a['raw_loyalty'],r,line,True);extreme(d,'max_pressure',pressure,r,line);extreme(d,'max_confidence_penalty',pen,r,line);extreme(d,'max_discontent',dis,r,line)
   if eligible and crisis:extreme(d,'min_effective_loyalty_in_eligible_crisis',loy,r,line,True);extreme(d,'max_confidence_penalty_in_eligible_crisis',pen,r,line)
   if st=='trigger_firing':d['firings']+=1;firing.append(r)
 assert not before and dict(stage_counts)==expected['stage_counts'] and firing==expected['firing_cases']
 assert (nraw,hashraw.hexdigest())==(240585752,'13676236845d5c814e640520d322227769d4a7e16c685a237a6ce99a2ca46b49')
 classifications=collections.defaultdict(list);electoral=[r for r in countries.values() if r['checks']]
 for d in electoral:
  f=d['features'];c=d['country']
  group='fired' if d['firings'] else 'no_army_at_any_electoral_check' if not f['has_army'] else 'army_never_passed_eligibility' if not f['eligible'] else 'eligible_but_no_crisis' if not f['eligible_crisis'] else 'eligible_crisis_but_never_hostile_in_crisis' if not f['eligible_both_live'] else 'both_live_but_pressure_not_ready'
  d['descriptive_classification']=group;classifications[group].append(c)
 for d in countries.values():
  for k in ['actual_branches','blocking_conditions_nonexclusive','blocking_joint_conditions','eligible_live_joint_conditions','features','funding']:d[k]=dict(sorted(d[k].items()))
  d['funding_examples']=d['funding_examples'][:3]
 summary={'format':'spheres-a1-geography-readonly/v1','scope':'Recorded fixed development seed0/252-month legacy-monthly observer; no simulation or holdout. Frequency counts are country-month exposure, not independent trials.','qualification':False,'a1_pass_claimed':False,'source_revision':'6818e4f0d94b01c86d7a9acc4252260947d13504','observations_input':pin(obs),'observations_decoded_bytes':nraw,'observations_decoded_sha256':hashraw.hexdigest(),'result_input':pin(result_path),'rows':sum(stage_counts.values()),'country_ids_seen_all_stages':len(seen),'countries_with_electoral_trigger_checks':len(electoral),'electoral_trigger_checks':sum(d['checks'] for d in electoral),'country_ids_without_trigger_checks':sorted(seen-{d['country'] for d in electoral}),'classifications':{k:sorted(v) for k,v in sorted(classifications.items())},'classification_counts':{k:len(v) for k,v in sorted(classifications.items())},'actual_branches':dict(sum((collections.Counter(d['actual_branches']) for d in electoral),collections.Counter())),'all_electoral_nonexclusive_guards':dict(sum((collections.Counter(d['blocking_conditions_nonexclusive']) for d in electoral),collections.Counter())),'all_electoral_eligible_joint':dict(sum((collections.Counter(d['eligible_live_joint_conditions']) for d in electoral),collections.Counter())),'all_electoral_features':dict(sum((collections.Counter(d['features']) for d in electoral),collections.Counter())),'no_coup_features':dict(sum((collections.Counter(d['features']) for d in electoral if not d['firings']),collections.Counter())),'funding_actions':len(actions),'positive_political_compensation_actions':sum((a['paid_share_above_current_and_material_only_floor'] or 0)>1e-10 for a in actions),'algebra_note':'The material-only inverse is a same-snapshot decomposition, not a new policy run, causal treatment effect, or recommendation to repeat rejected trial01. Personnel/GDP is inferred from recorded positive military share divided by resources per member; no missing quantities are invented.'}
 for name,data in [('summary.json',summary),('countries.json',sorted(electoral,key=lambda d:d['country'])),('all-funding-actions.json',actions)]:
  with (out/name).open('x',encoding='utf8',newline='\n') as f:json.dump(data,f,ensure_ascii=False,indent=2);f.write('\n')
 print(json.dumps(summary,indent=2))
if __name__=='__main__':
 p=argparse.ArgumentParser(description=__doc__);p.add_argument('--observations',type=Path,required=True);p.add_argument('--result',type=Path,required=True);p.add_argument('--out',type=Path,required=True);a=p.parse_args();analyze(a.observations,a.result,a.out)
