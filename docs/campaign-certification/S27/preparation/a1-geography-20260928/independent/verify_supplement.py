import collections,gzip,hashlib,json,pathlib
P=pathlib.Path;OUT=P(__file__).resolve().parent
PACK=P("D:/spheres-offload/codex-next-20260928/a1-geography-analysis-01")
OBS=P("C:/Users/ridge/Documents/Codex/2026-09-05/pick-up-the-spheres-game-on/work/campaign-certification/integration/docs/campaign-certification/S27/preparation/a1-firing-observer-20260928")
def j(p):return json.loads(p.read_text(encoding="utf8"))
declared={d["country"]:d for d in j(PACK/"reports/resource-context.json")["countries"]};data={}
for line,raw in enumerate(gzip.open(OBS/"original-run/observations.jsonl.gz","rb"),1):
 r=json.loads(raw);g=r["government"];a=r["army"];f=r["fiscal"]
 if not r["stage"].startswith("trigger_") or not g["has_army"] or g["settled_months"]<12 or g["awaiting_first_election"] or r["discontent"]["total"]<.25:continue
 c=r["country"];d=data.setdefault(c,{"checks":0,"saturated":0,"target_below":0,"zero_penalty":0,"zero_leverage":0,"min_target":None,"min_fraction":float("inf"),"max_fraction":float("-inf")})
 d["checks"]+=1;d["target_below"]+=a["target"]<.35;d["zero_penalty"]+=a["civilian_confidence_penalty"]==0;d["zero_leverage"]+=a["executive_leverage"]==0
 fraction=a["resources_per_member"]/(2*(2500+1.5*max(0,f["gdp_bn"]*1000/f["population_m"])))
 d["saturated"]+=fraction>=1;d["min_fraction"]=min(d["min_fraction"],fraction);d["max_fraction"]=max(d["max_fraction"],fraction)
 if d["min_target"] is None or a["target"]<d["min_target"]["value"]:d["min_target"]={"line":line,"date":r["date"],"value":a["target"],"snapshot":r}
assert data.keys()==declared.keys()
keys={"checks":"eligible_crisis_checks","saturated":"resource_saturated_checks","target_below":"target_below_035","zero_penalty":"zero_penalty_checks","zero_leverage":"zero_leverage_checks","min_fraction":"min_resource_fraction","max_fraction":"max_resource_fraction","min_target":"min_target"}
for c,d in data.items():
 for a,b in keys.items():assert d[a]==declared[c][b],(c,a)
countries={d["country"]:d for d in j(PACK/"reports/countries.json")}
rows=[x for x in (PACK/"reports/all-country-table.md").read_text().splitlines() if x.startswith("| ")][1:]
assert len(rows)==118
for row in rows:
 fields=[x.strip() for x in row.strip("|").split("|")];d=countries[fields[0]];g=d["blocking_conditions_nonexclusive"];f=d["features"];ex=d["extrema"]
 expected=[d["country"],str(d["checks"]),str(d["firings"]),"/".join(str(g.get(k,0)) for k in ("no_army","unsettled","interim")),*[str(g.get(k,0)) for k in ("loyalty_ge_0_35","discontent_lt_0_25","pressure_below_threshold")],str(f.get("eligible_crisis",0)),str(f.get("eligible_both_live",0)),*[format(ex[k]["value"],".9f") for k in ("min_effective_loyalty","max_pressure","max_confidence_penalty")],str(d["funding"].get("paid_increase",0))]
 assert fields==expected,d["country"]
s=j(PACK/"reports/resource-context.json")["summary"];assert len(s["no_coup_countries_always_saturated"])==16 and len(s["no_crisis_at_any_check"])==55 and len(s["crisis_only_outside_eligibility"])==7
for c,checks,min_target in [("Peru",211,.36976686879625886),("Philippines",169,.643270892),("Azerbaijan",160,.3665625)]:
 assert data[c]["checks"]==checks and abs(data[c]["min_target"]["value"]-min_target)<.0000000005
 assert countries[c]["extrema"]["max_pressure"]["value"]==0 and countries[c]["funding"].get("paid_increase",0)==0
assert data["Philippines"]["saturated"]==169 and data["Azerbaijan"]["saturated"]==160
for c in ("Pakistan","Thailand"):
 assert countries[c]["checks"]==252 and countries[c]["features"].get("crisis",0)==0 and countries[c]["extrema"]["max_pressure"]["value"]==0 and countries[c]["funding"].get("paid_increase",0)==0
r=j(PACK/"inputs/observer-result.json");assert r["passed"] and r["failure"] is None and r["seed"]==0 and r["months_compared"]==r["months_requested"]==252 and not r["qualification"] and not r["a1_pass_claimed"]
assert len(r["comparisons"])==252 and all(x["equal"] and x["observed_bytes"]==x["control_bytes"] and x["observed_fnv64"]==x["control_fnv64"] for x in r["comparisons"])
m=j(OBS/"manifest.json");b=m["binary_external_reference"];bp=P(b["path"]);h=hashlib.sha256()
with bp.open("rb") as f:
 while chunk:=f.read(1048576):h.update(chunk)
assert h.hexdigest()==b["sha256"] and bp.stat().st_size==b["bytes"]
ex=j(OBS/"execution/execution.json");assert h.hexdigest()==ex["binary_sha256"]==ex["binary_sha256_after"]
report={"passed":True,"resource_context_countries_compared":len(data),"full_country_table_rows_compared":len(rows),"published_examples_checked":["Peru","Philippines","Azerbaijan","Pakistan","Thailand"],"native_result_reported_comparisons":252,"reported_observer_parity_scope":"Retained report checked; native execution not repeated.","external_binary_sha256":h.hexdigest(),"external_binary_bytes":bp.stat().st_size,"binary_executed":False,"defects":[]}
with (OUT/"supplement-verification.json").open("x",encoding="utf8") as f:json.dump(report,f,indent=2);f.write("\n")
print(json.dumps(report,indent=2))

