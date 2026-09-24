from pathlib import Path
from datetime import datetime, timezone
import hashlib, json, collections

root=Path(__file__).resolve().parent
sha=lambda b:hashlib.sha256(b).hexdigest()
def load(name):
    raw=(root/name).read_bytes()
    return [json.loads(line) for line in raw.decode('utf-16' if raw.startswith(b'\xff\xfe') else 'utf-8-sig').splitlines()]
def file_record(name):
    p=root/name;raw=p.read_bytes()
    return {'path':name,'bytes':len(raw),'sha256':sha(raw)}
original=load('iteration-24-seed0-trace.jsonl')
extra=load('iteration-24-seed0-trace-hysteresis.jsonl')
hysteresis=[r for r in extra if r['kind']=='below_hysteresis_snapshot']
assert [r for r in extra if r['kind']!='below_hysteresis_snapshot']==original
assert original[0]['seed']==0 and original[-1]['months']==252
assert original[-1]==extra[-1]
plan=json.loads((root/'iteration-24-seed0-trace-plan.json').read_text(encoding='utf-8-sig'))
run=json.loads((root/'iteration-24-seed0-trace-hysteresis-result.json').read_text(encoding='utf-8-sig'))
assert plan['source_sha256']==sha((root/'iteration-24-seed0-trace.rs').read_bytes())
assert run['rlib_sha256_before']==run['rlib_sha256_after']=='e507e7efc8649e7171d3f8787ec9926b47b065b07c9577e07bd90ab089320e85'
counts=collections.Counter(r['kind'] for r in original)
coups=[r for r in original if r['kind']=='electoral_coup']
summaries={r['country']:r for r in original if r['kind']=='country_summary'}
country_counts=collections.Counter(r['country'] for r in coups)
cases=[]
for r in coups:
    b=r['before']
    cases.append({'country':r['country'],'month':r['month'],'ordinal':r['ordinal_in_country'],
        'pre_month_date':b['date'],'discontent':b['discontent'],'raw_loyalty':b['loyalty'],
        'effective_loyalty':b['effective_loyalty'],'target':b['actual_resource_target'],
        'confidence_penalty':b['confidence_penalty'],'coup_pressure':b['coup_pressure'],
        'fiscal_balance_gdp':b['fiscal_balance_share'],'fiscally_affordable_share':b['fiscally_affordable_share'],
        'military_share':b['military_share'],'funding_floor':b['army_funding_floor'],
        'needed_unaffordable':b['needed_unaffordable'],'political_capital':b['political_capital'],
        'latest_prior_observed_increase':b['latest_prior_observed_increase']})
hcases=[]
for r in hysteresis:
    b=r['snapshot']
    hcases.append({'country':b['country'],'month':r['month'],'date':b['date'],
        'discontent':b['discontent'],'coup_pressure':b['coup_pressure'],
        'effective_loyalty':b['effective_loyalty'],'current_target':b['actual_resource_target'],
        'proposed_target':b['target_at_proposed_floor'],'delta_share':b['proposed_funding_delta'],
        'price_quote_pc':b['proposed_funding_price'],'held_pc':b['political_capital'],
        'actual_target_crossing':b['actual_resource_target']<0.35<=b['target_at_proposed_floor'],
        'latest_prior_observed_increase':b['latest_prior_observed_increase']})
assert len(hcases)==4 and not any(r['actual_target_crossing'] for r in hcases)
near=[]
for country in ['Haiti','Thailand','Pakistan','Bangladesh','Myanmar','Nigeria','Nepal']:
    s=summaries[country]
    near.append({'country':country,'electoral_months':s['electoral_months'],
        'crisis_months':s['electoral_army_crisis_months'],'pending_months':s['electoral_army_pending_months'],
        'maximum_electoral_discontent':None if s['max_discontent'] is None else s['max_discontent']['discontent'],
        'minimum_electoral_loyalty':None if s['min_effective_loyalty'] is None else s['min_effective_loyalty']['effective_loyalty'],
        'maximum_electoral_pressure':None if s['max_coup_pressure'] is None else s['max_coup_pressure']['coup_pressure'],
        'first_coups':s['first_coups'],'repeat_coups':s['repeat_coups']})
result={'schema':1,'recorded_utc':datetime.now(timezone.utc).isoformat(),'seed':0,'months':252,
    'conclusion':'No additional confirmed causal defect. Nine pre-month coup snapshots lack room under existing nondeficit funding policy; one recurring Myanmar coup retains low current loyalty despite an already recovering .40 target. Four below-hysteresis observations do not cross the target threshold.',
    'diagnostic_completion_is_not_A1_acceptance':True,'source_commit':plan['git_head'],
    'original_event_counts':dict(counts),'first_coups':original[-1]['first_coups'],'repeat_coups':original[-1]['repeat_coups'],
    'country_coups':dict(country_counts),'single_seed_top_three_share':sum(sorted(country_counts.values(),reverse=True)[:3])/len(coups),
    'coup_cases':cases,'raw_equals_effective_at_all_pre_month_coup_snapshots':all(c['raw_loyalty']==c['effective_loyalty'] for c in cases),
    'pre_month_needed_unaffordable_coups':sum(c['needed_unaffordable'] for c in cases),
    'hysteresis_cases':hcases,'actual_target_crossings':0,'near_misses':near,
    'hysteresis_emitter_all_original_records_identical':True,'final':original[-1],
    'limits':['Snapshots bracket the full monthly tick, not the internal event instant.',
        'Budget changes are observed net appropriations; public quotes do not establish exact within-tick paid-command receipts.',
        'An existing .40 target can coexist with actual loyalty below .35 due to the deliberate recovery rate.',
        'This one previously observed development seed cannot certify the ensemble A1 gate or infer other countries are impossible.',
        'The original four counter flags combine low actual loyalty with low target; the added snapshots distinguish them and find no actual target crossing.',
        'No simulation mechanics, parameters, gates, cohorts or reserved holdout were changed.']}
(root/'iteration-24-seed0-trace-analysis.json').write_text(json.dumps(result,indent=2)+'\n',encoding='utf-8')
receipt={'schema':1,'recorded_utc':datetime.now(timezone.utc).isoformat(),
    'source_commit':plan['git_head'],'source_plan':'iteration-24-seed0-trace-plan.json',
    'source_scope':'Prospective 348-input manifest recorded before root execution; later test-only edits are not attributed to this binary.',
    'original_execution':{'build_exit':0,'run_exit':0,'authority':'Reported by parent root; complete seed receipt independently parsed; empty compiler log retained'},
    'followup_execution':run,
    'link_inputs':[{'path':'integration-target/release/deps/libspheres_sim-de80fa3e4f034d8c.rlib','sha256':run['rlib_sha256_before']},
        {'path':'integration-target/release/deps/libserde_json-019015e08a7ac1fe.rlib','sha256':'61f181ea73dfdeaf1e51c2355a919581074c1cc5152691c0937322690a0e5a34'}],
    'files':[file_record(n) for n in ['iteration-24-seed0-trace.rs','iteration-24-seed0-trace-plan.json','iteration-24-seed0-trace.exe','iteration-24-seed0-trace-build.log','iteration-24-seed0-trace.jsonl',
        'iteration-24-seed0-trace-hysteresis.rs','iteration-24-seed0-trace-hysteresis-plan.json','iteration-24-seed0-trace-hysteresis.exe','iteration-24-seed0-trace-hysteresis-build.log','iteration-24-seed0-trace-hysteresis.jsonl','iteration-24-seed0-trace-hysteresis-result.json','iteration-24-seed0-trace-analysis.json']],
    'read_only_confirmation':'Every original parsed output record is identical after removing four new emitter rows; both final state and RNG hashes match.',
    'status':'Diagnostic complete; remaining A1 failure not repaired or waived'}
(root/'iteration-24-seed0-trace-receipt.json').write_text(json.dumps(receipt,indent=2)+'\n',encoding='utf-8')
note='''# Iteration 24: fixed seed-0 coup diagnostic

This diagnostic completed against the exact checkpoint-24 compiled library.
It is one existing development seed over 252 months, not a new calibration
cohort or proof that the remaining A1 assertion passes.

There were ten elected coups: six first coups and four repeats. Guyana,
Guatemala, Sao Tome and Myanmar each had two; Ecuador and Comoros had one.
The seed's top-three share is .60. At the beginning of nine coup months, the
desired appropriation exceeded the existing nondeficit budget and the AI floor
was zero. This indicates the model's real fiscal constraint, not an ignored
affordable command. The snapshots cannot establish the exact internal firing
conditions after the economy and other systems update within that month.

Myanmar's second coup, month 144, is different: its preceding snapshot already
targets loyalty .40 while actual loyalty is only .302. Its latest observed
appropriation increase was month 138. The deliberately slow recovery leaves
the current Army disloyal long enough for pressure to keep building. This is
the existing time model; no forgotten appropriation or stale-pressure trigger
was demonstrated. All ten pre-month snapshots have identical raw and effective
loyalty, so no foreign-backing subtraction explains those observations.

The high-leverage controls identify different barriers. Pakistan and Thailand
remain electoral but never cross the .25 crisis requirement: maximum D is
.2107 and .1820. Bangladesh has 28 electoral months, 18 pending, and maximum
D .1904. Haiti and Nigeria never become electoral in this seed. Their missing
elected coups do not show a failed leverage hook.

## The four below-hysteresis observations

The original trace counted four affordable funding deltas below .001 GDP while
current loyalty was low. Its extrema did not retain their exact snapshots, so
an explicitly authorized follow-up added only four print rows to the same
fixed seed and binary. Every original parsed record, final state and RNG value
is identical. No alternative seed or mechanics change was introduced.

| Country / month | Actual loyalty | Current target | Proposed target | Delta GDP share | Quoted PC |
|---|---:|---:|---:|---:|---:|
| Nepal / 126 | .34992 | .39764 | .40000 | .00013323 | .01998 |
| Myanmar / 139 | .27633 | .39771 | .40000 | .00038063 | .05709 |
| Myanmar / 140 | .28183 | .39848 | .40000 | .00025108 | .03766 |
| Myanmar / 141 | .28711 | .39922 | .40000 | .00012815 | .01922 |

None would move the modeled target from below .35 to above it: all four
targets are already above .397. Nepal also has D .057 and zero pressure after
an increase in month 125. Myanmar is recovering after its month-138 increase.
These cases therefore do not establish that the spending hysteresis suppresses
a meaningful target rescue. The quoted PC is the public price at the observed
snapshot, not a receipt for an internal command.

No further confirmed defect or parameter proposal follows from this trace.
The original A1 concentration failure remains open. Full logs, prospective
plans, source and executable/library hashes are named in the compact receipt.
'''
(root/'iteration-24-seed0-trace-analysis.md').write_text(note,encoding='utf-8')
print(json.dumps({'coup_counts':dict(country_counts),'first':result['first_coups'],'repeat':result['repeat_coups'],'hysteresis_cases':len(hcases),'target_crossings':0,'all_original_records_identical':True,'receipt':'iteration-24-seed0-trace-receipt.json'}))
