import json,time
from retrieve import OUT,sources,run
original=json.loads((OUT/'source-verification.json').read_text(encoding='utf-8'))
lookup={s['id']:s for s in sources};output=[];limited=False
for row in original['sources']:
    if row['exact'] or limited:output.append(row);continue
    result=run(lookup[row['source_id']],(3,))
    result['attempts']=row['attempts']+result['attempts'];output.append(result)
    if result['attempts'][-1]['response'].splitlines()[0:1]==['429']:limited=True
    elif not result['exact']:time.sleep(10)
    else:time.sleep(10)
original['sources']=output;original['exact']=sum(x['exact'] for x in output)
original['retained_prior_receipt']='source-verification.json';original['retry_scope']='One paced missing-only pass after coordinated cooldown; stop on429'
(OUT/'source-verification-final.json').write_text(json.dumps(original,ensure_ascii=False,indent=2)+'\n',encoding='utf-8')
print(json.dumps({'exact':original['exact'],'total':len(output),'rate_limited':limited}),flush=True)
