import datetime,json,math,pathlib,sys
base=pathlib.Path(__file__).resolve().parent
pin=sys.argv[1]
out=base/'evidence/S07-performance'
protocol=json.loads((base/'evidence/S07-performance-protocol.json').read_text(encoding='utf-8'))
runner=json.loads((out/'runner-result.json').read_text(encoding='utf-8-sig'))
profile=json.loads((out/'profile.json').read_text(encoding='utf-8-sig'))
assert protocol['candidate']==pin==runner['candidate_revision']
assert runner['test_binary_sha256']==protocol['test_binary_sha256']
rows=[]
for row in profile['results']:
 item={'scenario':row['scenario'],'age':row['checkpoint_years'],'sim_p95_ms':row['simulation_and_history_recording']['p95_ms'],'whole_p95_ms':row['whole_server_turn']['p95_ms'],'whole_max_ms':row['whole_server_turn']['max_ms']}
 assert all(math.isfinite(item[k]) and item[k]>=0 for k in ['sim_p95_ms','whole_p95_ms','whole_max_ms'])
 item['passed']=item['sim_p95_ms']<=protocol['simulation_p95_limit_ms'] and item['whole_p95_ms']<=protocol['whole_turn_p95_limit_ms'] and item['whole_max_ms']<=protocol['whole_turn_max_limit_ms']
 rows.append(item)
assert {(r['scenario'],r['age']) for r in rows}=={(s,a) for s in ['idle_human','industry_and_war'] for a in [0,10,30]} and len(rows)==6
memory=runner['memory']['max_sampled_private_bytes']
result={'candidate':pin,'evaluated_utc':datetime.datetime.now(datetime.timezone.utc).isoformat(),'rows':rows,'sampled_private_bytes':memory,'memory_passed':memory<=protocol['sampled_private_memory_limit_bytes'],'runner_passed':runner['passed']}
result['passed']=result['runner_passed'] and result['memory_passed'] and all(r['passed'] for r in rows)
(out/'acceptance.json').write_text(json.dumps(result,indent=2)+'\n',encoding='utf-8')
print(json.dumps(result,indent=2));sys.exit(0 if result['passed'] else 1)
