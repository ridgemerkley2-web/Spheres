import json, pathlib, statistics, collections, hashlib
base = pathlib.Path(__file__).parent
raw = base / 'iteration-20-confidence-diagnostic.jsonl'
rows = [json.loads(s) for s in raw.read_text(encoding='utf-8-sig').splitlines() if s.strip()]
summaries = [r for r in rows if r['kind'] == 'country_summary']
events = [r for r in rows if r['kind'] == 'electoral_coup']
by_country = collections.defaultdict(list)
for r in summaries: by_country[r['country']].append(r)
counts = collections.Counter(r['country'] for r in events)
def rank(r):
    p = r['closest_pre_first']
    if not p: return (-1, -1, -1, -1)
    return (p['discontent'] >= .25 and p['settled_months'] >= 12 and not p['awaiting_first_election'], p['coup_pressure'], -p['effective_loyalty'], p['confidence_penalty'])
results = []
for country, ss in sorted(by_country.items()):
    high = sum(s['source_high_leverage_months'] for s in ss)
    if not high: continue
    best = max(ss, key=rank)
    p = best['closest_pre_first']
    example = None if not p else {k:p[k] for k in ['date','authoritarianism','army_authority','record','settled_months','discontent','loyalty','effective_loyalty','coup_pressure','confidence_penalty','political_only_full_provisions_target','actual_resource_target','resources_per_member','useful_resources_per_member_cap','fiscally_affordable_share','military_share','army_funding_floor','funding_command_affordable','needed_unaffordable','eligible_public_mandate_proxy','opening_mandate']}
    results.append(dict(country=country, events=counts[country], source_high_electoral_months=high, source_high_crisis_months=sum(s['source_high_leverage_crisis_months'] for s in ss), max_confidence_penalty=max(s['max_confidence_penalty'] for s in ss), min_political_only_full_provision_target=.85-max(s['max_confidence_penalty'] for s in ss), observed_funding_increases=sum(s['funding_increases'] for s in ss), needed_unaffordable_electoral_months=sum(s['unaffordable_needed_months'] for s in ss), example_seed=best['seed'], closest_pre_first=example))
no_coups = sorted([r for r in results if not r['events']], key=lambda r:r['max_confidence_penalty'], reverse=True)
result = dict(iteration=20,seeds=list(range(12)),months=252,source_log_sha256=hashlib.sha256(raw.read_bytes()).hexdigest(),first_coups=sum(e['first'] for e in events),repeat_coups=sum(not e['first'] for e in events),events=len(events),countries=results, high_source_without_coups=[r['country'] for r in no_coups],limitations=['High source means opening expert-response proxy >=.5; live leverage may subsequently change.','Closest snapshot prioritizes settled electoral crisis, then pressure, then low effective loyalty, before first coup in that country. It is before the full monthly tick, not the exact internal event instant.','Maximum confidence penalty is across all observed months; full-provision target removes war and material shortage by algebra only, without rerunning a counterfactual.','This is the existing development seed block, not independent validation.'])
(base/'iteration-20-high-leverage-near-misses.json').write_text(json.dumps(result,indent=2)+'\n',encoding='utf-8')
print(json.dumps(dict(events=len(events),first=result['first_coups'],repeat=result['repeat_coups'],high_source_without_coups=[{k:r[k] for k in ['country','source_high_electoral_months','source_high_crisis_months','max_confidence_penalty','min_political_only_full_provision_target','observed_funding_increases','needed_unaffordable_electoral_months']} for r in no_coups]),indent=2))