import json,gzip,collections,hashlib
from pathlib import Path
base=Path(r'D:/spheres-offload/codex-next-20260928');out=base/'a1-geography-analysis-01';ds=json.loads((out/'reports/countries.json').read_text());by={d['country']:d for d in ds};nocoup={d['country'] for d in ds if not d['firings']};witnesses=set()
for d in ds:
 for key in ['min_effective_loyalty','max_confidence_penalty','max_pressure']:
  witnesses.add(d['extrema'][key]['line'])
extra={};selected=[];salvador=[]
for line,raw in enumerate(gzip.open(base/'a1-portable-01/original-run/observations.jsonl.gz','rb'),1):
 r=json.loads(raw);c=r['country'];g=r['government'];a=r['army'];f=r['fiscal']
 if line in witnesses:selected.append({'line':line,'snapshot':r})
 if c=='ElSalvador' and r['stage'].startswith('trigger_') and [1991,9,1]<=r['date']<=[1992,5,1]:salvador.append({'line':line,'snapshot':r})
 if not r['stage'].startswith('trigger_') or not g['has_army'] or g['settled_months']<12 or g['awaiting_first_election'] or r['discontent']['total']<.25:continue
 d=extra.setdefault(c,{'country':c,'eligible_crisis_checks':0,'target_below_035':0,'resource_saturated_checks':0,'zero_penalty_checks':0,'zero_leverage_checks':0,'min_target':None,'min_resource_fraction':None,'max_resource_fraction':None})
 d['eligible_crisis_checks']+=1;d['zero_penalty_checks']+=a['civilian_confidence_penalty']==0;d['zero_leverage_checks']+=a['executive_leverage']==0
 if a['target'] is not None:
  d['target_below_035']+=a['target']<.35
  if d['min_target'] is None or a['target']<d['min_target']['value']:d['min_target']={'line':line,'date':r['date'],'value':a['target'],'snapshot':r}
 if a['resources_per_member'] is not None:
  fraction=a['resources_per_member']/(2*(2500+1.5*max(0,f['gdp_bn']*1000/f['population_m'])))
  d['resource_saturated_checks']+=fraction>=1
  d['min_resource_fraction']=min(fraction,d['min_resource_fraction']) if d['min_resource_fraction'] is not None else fraction
  d['max_resource_fraction']=max(fraction,d['max_resource_fraction']) if d['max_resource_fraction'] is not None else fraction
nc=[d for c,d in extra.items() if c in nocoup];summary={'no_coup_eligible_crisis_countries':len(nc),'no_coup_eligible_crisis_checks':sum(d['eligible_crisis_checks'] for d in nc),'no_coup_resource_saturated_checks':sum(d['resource_saturated_checks'] for d in nc),'no_coup_zero_penalty_checks':sum(d['zero_penalty_checks'] for d in nc),'no_coup_zero_leverage_checks':sum(d['zero_leverage_checks'] for d in nc),'no_coup_target_below_035_checks':sum(d['target_below_035'] for d in nc),'no_coup_countries_ever_saturated':sorted(d['country'] for d in nc if d['resource_saturated_checks']>0),'no_coup_countries_always_saturated':sorted(d['country'] for d in nc if d['resource_saturated_checks']==d['eligible_crisis_checks']),'no_crisis_at_any_check':sorted(d['country'] for d in ds if d['descriptive_classification']=='eligible_but_no_crisis' and not d['features'].get('crisis',0)),'crisis_only_outside_eligibility':sorted(d['country'] for d in ds if d['descriptive_classification']=='eligible_but_no_crisis' and d['features'].get('crisis',0))}
for name,data in [('resource-context.json',{'summary':summary,'countries':sorted(extra.values(),key=lambda d:d['country'])}),('extreme-witnesses.json',selected),('el-salvador-sequence.json',salvador)]:
 with (out/'reports'/name).open('x',encoding='utf8',newline='\n') as f:json.dump(data,f,indent=2);f.write('\n')
print(json.dumps(summary,indent=2));print('\nNo-coup target minima:')
for d in sorted(nc,key=lambda x:x['country']):print(d['country'],d['min_target']['value'],d['resource_saturated_checks'],d['eligible_crisis_checks'])
