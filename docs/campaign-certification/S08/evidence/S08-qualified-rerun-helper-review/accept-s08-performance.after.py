import datetime,hashlib,json,math,pathlib,re,sys
base=pathlib.Path(__file__).resolve().parent
pin=sys.argv[1]
assert re.fullmatch('[0-9a-f]{40}',pin)
assert len(sys.argv)<=4,'Use PIN [OUTPUT_DIRECTORY] [PROTOCOL_JSON]'
out=pathlib.Path(sys.argv[2]).resolve() if len(sys.argv)>2 else base/'evidence/S08-performance'
assert not (out/'acceptance.json').exists(),'Refuse to replace an earlier acceptance result'
sha=lambda path:hashlib.sha256(path.read_bytes()).hexdigest()
protocol_file=pathlib.Path(sys.argv[3]).resolve() if len(sys.argv)>3 else base/'evidence/S08-performance-protocol.json'
plan_file=base/'evidence/S08-performance-plan.json'
protocol=json.loads(protocol_file.read_text(encoding='utf-8-sig'))
plan=json.loads(plan_file.read_text(encoding='utf-8-sig'))
# This was the declared plan before the first S08 measurement. Rebinding a
# candidate is not permission to replace that plan or weaken its workload.
assert sha(plan_file)=='8d2cb23fe996ba9dc35da0aed5859072a83c2e61b55d4881cf047bc1ac195a28','Original S08 performance plan changed'
assert protocol['declared_plan_sha256']==sha(plan_file),'The candidate protocol must bind the original declared plan bytes'
assert all(protocol[key]==value for key,value in plan['legacy_regression'].items()),'The rebound protocol changed an original workload or acceptance bar'
runner=json.loads((out/'runner-result.json').read_text(encoding='utf-8-sig'))
profile=json.loads((out/'profile.json').read_text(encoding='utf-8-sig'))
assert protocol['candidate']==pin==runner['candidate_revision']
assert runner['test_binary_sha256']==protocol['test_binary_sha256']
assert protocol['simulation_p95_limit_ms']==300 and protocol['whole_turn_p95_limit_ms']==400 and protocol['whole_turn_max_limit_ms']==750
assert protocol['sampled_private_memory_limit_bytes']==1073741824
assert runner['exit_code']==0 and runner['passed'] is True
rows=[]
for row in profile['results']:
 item={'scenario':row['scenario'],'age':row['checkpoint_years'],'sim_p95_ms':row['simulation_and_history_recording']['p95_ms'],'whole_p95_ms':row['whole_server_turn']['p95_ms'],'whole_max_ms':row['whole_server_turn']['max_ms']}
 assert all(math.isfinite(item[k]) and item[k]>=0 for k in ['sim_p95_ms','whole_p95_ms','whole_max_ms'])
 item['passed']=item['sim_p95_ms']<=protocol['simulation_p95_limit_ms'] and item['whole_p95_ms']<=protocol['whole_turn_p95_limit_ms'] and item['whole_max_ms']<=protocol['whole_turn_max_limit_ms']
 rows.append(item)
assert {(r['scenario'],r['age']) for r in rows}=={(s,a) for s in ['idle_human','industry_and_war'] for a in [0,10,30]} and len(rows)==6
memory=runner['memory']['max_sampled_private_bytes']
assert isinstance(memory,(int,float)) and not isinstance(memory,bool) and math.isfinite(memory) and memory>0
assert runner['memory']['samples']>0
result={'protocol':str(protocol_file),'protocol_sha256':sha(protocol_file),'declared_plan':str(plan_file),'declared_plan_sha256':sha(plan_file),'runner_result_sha256':sha(out/'runner-result.json'),'profile_sha256':sha(out/'profile.json'),'runner_sha256':sha(pathlib.Path(__file__)),'candidate':pin,'evaluated_utc':datetime.datetime.now(datetime.timezone.utc).isoformat(),'rows':rows,'sampled_private_bytes':memory,'memory_passed':memory<=protocol['sampled_private_memory_limit_bytes'],'runner_passed':runner['passed']}
result['passed']=result['runner_passed'] and result['memory_passed'] and all(r['passed'] for r in rows)
with (out/'acceptance.json').open('x',encoding='utf-8') as output:json.dump(result,output,indent=2);output.write('\n')
print(json.dumps(result,indent=2));sys.exit(0 if result['passed'] else 1)
