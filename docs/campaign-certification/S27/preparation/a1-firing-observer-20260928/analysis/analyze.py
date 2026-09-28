import json,hashlib,collections,pathlib,math,sys
src=pathlib.Path(r'D:/spheres-offload/codex-next-20260928/a1-firing-observer-run-01')
out=pathlib.Path(r'D:/spheres-offload/codex-next-20260928/a1-firing-analysis-01')
result=json.loads((src/'result.json').read_text())
firings=result['firing_cases']; countries={r['country'] for r in firings}
stages=collections.Counter(); pending_walk={};pending_fund={}; windows=[];funds=[]; issues=[];walks=0;decisions=collections.Counter(); exact_firings=[]; digest=hashlib.sha256(); sizes=0

def month(r): return r['date'][0]*12+r['date'][1]-1

def compact(r,line):
    return {'line':line,'country':r['country'],'date':r['date'],'stage':r['stage'],'army':r['army'],'fiscal':r['fiscal'],'discontent':r['discontent'],'government':r['government']}
with (src/'observations.jsonl').open('rb') as f:
    for line,raw in enumerate(f,1):
        digest.update(raw);sizes+=len(raw); r=json.loads(raw); stage=r['stage']; key=(r['country'],tuple(r['date'])); stages[stage]+=1
        if r['country'] in countries and any(a['country']==r['country'] and 0<=month(a)-month(r)<=12 for a in firings): windows.append(compact(r,line))
        if stage=='trigger_firing': exact_firings.append(r)
        if stage=='army_before_walk':
            if key in pending_walk: issues.append({'kind':'duplicate_walk_before','line':line})
            pending_walk[key]=(r,line)
        elif stage=='army_after_walk':
            b,bl=pending_walk.pop(key);walks+=1
            target=b['army']['target'];loy=b['army']['raw_loyalty']; want=max(0,min(1,loy+(target-loy)*(.1 if target<loy else .045)))
            eff=r['army']['effective_loyalty'];d=r['discontent']['total'];pressure=b['army']['pressure'];army=.35;dis=.25
            p=min(1.5,pressure+.3*(2*(army-eff)+(d-dis))) if eff<army and d>=dis else max(0,pressure-.03)
            if abs(want-r['army']['raw_loyalty'])>1e-14 or abs(p-r['army']['pressure'])>1e-14: issues.append({'kind':'walk_or_pressure','line':line,'want_loyalty':want,'actual_loyalty':r['army']['raw_loyalty'],'want_pressure':p,'actual_pressure':r['army']['pressure']})
        elif stage=='ai_before_funding':pending_fund[key]=(r,line)
        elif stage=='ai_after_funding':
            b,bl=pending_fund.pop(key); q=b['fiscal']['funding_quote'];l=b['army']['raw_loyalty']; eligible=l is not None and l < (.5 if b['date'][1]==1 else .4)
            reason='no_army_or_no_low_loyalty' if not eligible else 'no_floor' if q is None else 'below_hysteresis' if not q['clears_existing_hysteresis'] else 'no_standing' if not q['standing_affordable'] else 'command_expected'
            decisions[reason]+=1
            changed=b['fiscal']['mil_spend_gdp']!=r['fiscal']['mil_spend_gdp']
            if reason=='command_expected':
                if r['fiscal']['mil_spend_gdp']!=q['share']: issues.append({'kind':'funding_not_applied','country':r['country'],'line':line,'before':b['fiscal'],'after':r['fiscal']})
                if not math.isclose(b['fiscal']['political_capital']-q['price_pc'],r['fiscal']['political_capital'],rel_tol=0,abs_tol=1e-12):issues.append({'kind':'funding_charge','country':r['country'],'line':line})
            elif changed:issues.append({'kind':'unexpected_funding','line':line,'reason':reason})
            if r['country'] in countries:
                funds.append({'before_line':bl,'after_line':line,'country':r['country'],'date':r['date'],'reason':reason,'changed':changed,'before_mil':b['fiscal']['mil_spend_gdp'],'after_mil':r['fiscal']['mil_spend_gdp'],'loyalty':b['army']['raw_loyalty'],'target_before':b['army']['target'],'target_after':r['army']['target'],'pressure':b['army']['pressure'],'quote':q,'balance':b['fiscal']['balance_gdp'],'debt':b['fiscal']['debt_gdp'],'civilian_penalty':b['army']['civilian_confidence_penalty']})
assert exact_firings==firings
assert dict(stages)==result['stage_counts']
assert len(result['comparisons'])==252 and all(c['equal'] for c in result['comparisons'])
assert not pending_walk and not pending_fund
files=[]
for p in sorted(src.iterdir()):
    if p.name=='observations.jsonl':size,sha=sizes,digest.hexdigest()
    else:raw=p.read_bytes();size,sha=len(raw),hashlib.sha256(raw).hexdigest()
    files.append({'path':str(p),'bytes':size,'sha256':sha})
assert files[0]['sha256']==next(x['sha256'] for x in files if x['path'].endswith('observed-final.json'))
summary={'format':'spheres-a1-firing-readonly-analysis/v1','reviewer':'Codex /root/review_gap_submission','scope':'Existing seed 0 /252 months diagnostic only, no native execution, acceptance change or holdout use','source_commit':'6818e4f0d94b01c86d7a9acc4252260947d13504','qualification':False,'a1_pass_claimed':False,'rows':sum(stages.values()),'stage_counts':dict(stages),'monthly_comparisons':len(result['comparisons']),'raw_firings_match_result':True,'final_files_byte_identical':True,'walk_pairs_checked':walks,'funding_decisions':dict(decisions),'mechanical_mismatches':issues,'input_files':files}
for name,data in [('mechanical-check.json',summary),('firing-windows.json',windows),('firing-country-funding.json',funds)]:out.joinpath(name).write_text(json.dumps(data,ensure_ascii=False,indent=2)+'\n',encoding='utf8',newline='\n')
print(json.dumps(summary,indent=2))
print('\nFIRING COUNTRY FUNDING CHANGES:')
for r in funds:
    if r['changed']:print(json.dumps(r))
print('\nMYANMAR LAST 18 MONTHS BEFORE LAST COUP:')
for r in windows:
    if r['country']=='Myanmar' and r['date']>=[2001,1,1] and r['stage'] in ('army_before_walk','army_after_walk','trigger_firing','ai_before_funding','ai_after_funding'):print(json.dumps({'date':r['date'],'stage':r['stage'],'line':r['line'],'loyalty':r['army']['raw_loyalty'],'target':r['army']['target'],'pressure':r['army']['pressure'],'d':r['discontent']['total'],'mil':r['fiscal']['mil_spend_gdp'],'room':r['fiscal']['affordable_military_share'],'quote':r['fiscal']['funding_quote']}))
