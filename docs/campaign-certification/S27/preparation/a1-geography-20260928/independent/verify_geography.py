"""Independent retained-evidence audit. Runs only Python analysis; no native simulation."""
import collections,gzip,hashlib,json,pathlib,subprocess,sys
P=pathlib.Path
PACK=P("D:/spheres-offload/codex-next-20260928/a1-geography-analysis-01")
REPO=P("C:/Users/ridge/Documents/Codex/2026-09-05/pick-up-the-spheres-game-on/work/campaign-certification/integration")
OBS=REPO/"docs/campaign-certification/S27/preparation/a1-firing-observer-20260928"
OUT=P(__file__).resolve().parent
def pin(p):
 h=hashlib.sha256();n=0
 with p.open("rb") as f:
  while b:=f.read(1048576):h.update(b);n+=len(b)
 return {"bytes":n,"sha256":h.hexdigest()}
def expect(p,d,bytekey="bytes",hashkey="sha256"):
 a=pin(p);assert a=={"bytes":d[bytekey],"sha256":d[hashkey]},str(p);return a
def js(p):return json.loads(p.read_text(encoding="utf-8"))
m=js(PACK/"manifest.json");mpin=pin(PACK/"manifest.json")
assert len(m["files"])==28
assert {p.relative_to(PACK).as_posix() for p in PACK.rglob("*") if p.is_file()}=={"manifest.json",*[d["path"] for d in m["files"]]}
for d in m["files"]:expect(PACK/d["path"],d)
copied=[]
for d in m["copied_input_pins"]:
 expect(PACK/d["retained_path"],d);expect(P(d["original_path"]),d);copied.append(d["retained_path"])
mapping={"inputs/observer-result.json":"original-run/result.json","inputs/observer-execution.json":"execution/execution.json","inputs/prior-trigger-confidence-check.json":"analysis/trigger-confidence-check.json","inputs/observer-portable-manifest.json":"manifest.json"}
for a,b in mapping.items():assert (PACK/a).read_bytes()==(OBS/b).read_bytes(),a
obs=OBS/"original-run/observations.jsonl.gz"
expect(obs,m["observations_pin"]);expect(P(m["observations_pin"]["path"]),m["observations_pin"])
sources=[]
for d in m["source_files"]:
 raw=(OBS/"source"/d["repo_path"]).read_bytes()
 assert len(raw)==d["raw_bytes"] and hashlib.sha256(raw).hexdigest()==d["raw_sha256"]
 expect(P(d["path"]),d,"raw_bytes","raw_sha256")
 blob=subprocess.check_output(["git","show",m["source_revision"]+":"+d["repo_path"]],cwd=REPO)
 assert len(blob)==d["git_blob_bytes"] and hashlib.sha256(blob).hexdigest()==d["git_blob_sha256"]
 assert raw.replace(b"\r\n",b"\n")==blob
 sources.append(d["repo_path"])
cmd=[sys.executable,"-B","-X","utf8",str(PACK/"analyze_geography.py"),"--observations",str(obs),"--result",str(OBS/"original-run/result.json"),"--out",str(OUT/"reproduced")]
with (OUT/"reproduction.log").open("xb") as log:
 completed=subprocess.run(cmd,stdout=log,stderr=subprocess.STDOUT)
assert completed.returncode==0,"Main analysis failed; see reproduction.log"
for n in ("countries.json","all-funding-actions.json"):
 assert (OUT/"reproduced"/n).read_bytes()==(PACK/"reports"/n).read_bytes(),n
a=js(OUT/"reproduced/summary.json");b=js(PACK/"reports/summary.json")
pathdiffs=[]
for k in ("observations_input","result_input"):
 assert {x:y for x,y in a[k].items() if x!="path"}=={x:y for x,y in b[k].items() if x!="path"}
 pathdiffs.append({"field":k+".path","original":b[k]["path"],"reproduced":a[k]["path"]})
 a[k]["path"]=b[k]["path"]
assert a==b,"Summary changes beyond input paths"
# Independently count recorded branches and actual before/after expenditure changes.
stages=collections.Counter();branches=collections.Counter();blockers=collections.Counter();joints=collections.Counter()
eligible=collections.Counter();countries=set();seen=set();firers=set();no_coup_branches=collections.Counter()
before={};actual_funding=[];trigger_rows=[];witnesses={};resource={}
sha=hashlib.sha256();size=0;count=0
wanted={d["line"]:d["snapshot"] for d in js(PACK/"reports/extreme-witnesses.json")}
wanted.update({d["line"]:d["snapshot"] for d in js(PACK/"reports/el-salvador-sequence.json")})
for line,raw in enumerate(gzip.open(obs,"rb"),1):
 count+=1;size+=len(raw);sha.update(raw);r=json.loads(raw);s=r["stage"];c=r["country"];stages[s]+=1;seen.add(c)
 if line in wanted:assert r==wanted[line];witnesses[line]=r
 key=(c,tuple(r["date"]))
 if s=="ai_before_funding":before[key]=(r,line)
 elif s=="ai_after_funding":
  prior,priorline=before.pop(key);old=prior["fiscal"]["mil_spend_gdp"];new=r["fiscal"]["mil_spend_gdp"]
  if old!=new:
   assert new>old
   q=prior["fiscal"]["funding_quote"];assert q and q["share"]==new
   actual_funding.append((c,priorline,line))
 if not s.startswith("trigger_"):continue
 countries.add(c);branches[s]+=1;g=r["government"];ar=r["army"];D=r["discontent"]["total"]
 assert r["rules"]=={"crisis_intensity":1.0,"daily_simulation":False,"ideology_blocs":True,"ideology_takeover":True}
 assert r["thresholds"]=={"army":.35,"discontent":.25,"pressure":1.0,"settled_months":12}
 assert g["electoral"]
 flags=[("no_army",not g["has_army"]),("unsettled",g["settled_months"]<12),("interim",g["awaiting_first_election"]),("loyalty_ge_0_35",ar["effective_loyalty"]>=.35),("discontent_lt_0_25",D<.25),("pressure_below_threshold",ar["pressure"]<1)]
 reasons=[n for n,yes in flags if yes];blockers.update(reasons);joints["+".join(reasons) or "none"]+=1
 expected_branch="trigger_unsettled_interim_or_no_army" if any(yes for _,yes in flags[:3]) else "trigger_live_conditions_inactive" if any(yes for _,yes in flags[3:5]) else "trigger_pressure_not_ready" if flags[-1][1] else "trigger_firing"
 assert s==expected_branch,(line,s,expected_branch)
 if s=="trigger_firing":firers.add(c)
 trigger_rows.append((c,s))
 if not any(yes for _,yes in flags[:3]):
  eligible[("loyal" if flags[3][1] else "hostile")+"+"+("low_discontent" if flags[4][1] else "crisis")]+=1
  if D>=.25:
   d=resource.setdefault(c,collections.Counter());d["checks"]+=1;d["target_below"]+=ar["target"]<.35
   d["zero_penalty"]+=ar["civilian_confidence_penalty"]==0;d["zero_leverage"]+=ar["executive_leverage"]==0
   f=r["fiscal"];fraction=ar["resources_per_member"]/(2*(2500+1.5*max(0,f["gdp_bn"]*1000/f["population_m"])))
   d["saturated"]+=fraction>=1
assert not before and len(witnesses)==len(wanted)
assert (size,sha.hexdigest())==(m["observations_decoded_bytes"],m["observations_decoded_sha256"])
assert count==146525 and len(countries)==118 and len(seen-countries)==40 and len(firers)==6
assert dict(branches)==b["actual_branches"]
assert dict(blockers)==b["all_electoral_nonexclusive_guards"] and dict(eligible)==b["all_electoral_eligible_joint"]
global_report=js(PACK/"reports/global-joint-guards.json")
assert dict(joints)==global_report["all_electoral_joint_blockers"] and len(joints)==18
for c,s in trigger_rows:
 if c not in firers:no_coup_branches[s]+=1
assert dict(no_coup_branches)==global_report["actual_branches_no_coup_countries"]
reported_actions=js(PACK/"reports/all-funding-actions.json")
assert actual_funding==[(d["country"],d["before_line"],d["after_line"]) for d in reported_actions]
nonfire_resources={c:d for c,d in resource.items() if c not in firers}
total=sum(nonfire_resources.values(),collections.Counter())
assert len(nonfire_resources)==29 and dict(total)=={"checks":3552,"target_below":2,"zero_penalty":629,"zero_leverage":433,"saturated":2390}
assert not ({x[0] for x in actual_funding}&nonfire_resources.keys())
assert len(actual_funding)==151 and len({x[0] for x in actual_funding})==13
comp=collections.Counter(d["country"] for d in reported_actions if (d["paid_share_above_current_and_material_only_floor"] or 0)>1e-10)
assert dict(comp)=={"Myanmar":9,"Comoros":1,"Guatemala":1,"SaoTome":1}
# Exact retained El Salvador chronology and original line identities.
sequence=js(PACK/"reports/el-salvador-sequence.json"); bydate={tuple(x["snapshot"]["date"]):x for x in sequence}
dec=bydate[(1991,12,1)];jan=bydate[(1992,1,1)]["snapshot"];feb=bydate[(1992,2,1)]["snapshot"];mar=bydate[(1992,3,1)]["snapshot"]
assert dec["line"]==11740
r=dec["snapshot"];assert r["government"]["leader"]=="sv_pdc" and r["government"]["settled_months"]==12
assert r["army"]["effective_loyalty"]==.33880055075691395 and r["discontent"]["total"]==.40406468087279657 and r["army"]["pressure"]==.16178330466356525
assert r["stage"]=="trigger_pressure_not_ready"
assert jan["government"]["leader"]=="sv_arena" and jan["government"]["record"]["months"]==1 and jan["army"]["civilian_confidence_penalty"]==0
assert jan["stage"]=="trigger_unsettled_interim_or_no_army"
assert feb["army"]["pressure"]==.24942763380274532 and feb["stage"]=="trigger_unsettled_interim_or_no_army"
assert mar["army"]["effective_loyalty"]>=.35 and mar["army"]["pressure"]<feb["army"]["pressure"]
assert all(x["snapshot"]["fiscal"]["mil_spend_gdp"]==.034 and x["snapshot"]["fiscal"]["affordable_military_share"]==0 for x in sequence)
assert pin(PACK/"manifest.json")==mpin
for d in m["files"]:expect(PACK/d["path"],d)
report={"format":"spheres-a1-geography-independent/v1","reviewer":"Codex /root/review_source05","passed":True,"packet_manifest":mpin,"payload_count_verified":28,"copied_input_pins_verified":len(copied),"raw_and_git_source_files_verified":sources,"observer_packet":str(OBS),"analysis_command":cmd,"reproduction_exit_code":completed.returncode,"reproduction_byte_exact":["countries.json","all-funding-actions.json"],"summary_only_differences":pathdiffs,"independent_raw_checks":{"rows":count,"decoded_bytes":size,"decoded_sha256":sha.hexdigest(),"electoral_countries":len(countries),"electoral_checks":sum(branches.values()),"firing_countries":sorted(firers),"actual_branches":dict(branches),"nonfiring_branches":dict(no_coup_branches),"joint_guard_patterns":len(joints),"eligible_joint":dict(eligible),"actual_funding_changes":len(actual_funding),"funding_country_count":len({x[0] for x in actual_funding}),"nonfiring_eligible_crisis_countries":len(nonfire_resources),"nonfiring_eligible_crisis_totals":dict(total),"political_increment_countries":dict(comp),"exact_witness_rows_checked":len(witnesses),"el_salvador_december_line":dec["line"]},"defects":[],"limits":["One already-used seed0/252-month legacy trace; no simulations, Cargo or holdouts.","No causal treatment effect or A1 success follows from the descriptive counts.","Main analysis reproduced; supplementary fixed-path resource script was inspected, not executed. Its published resource/witness values were independently recomputed from original rows.","Compiler/execution binding retained and source pins verified; no reproducible-build attestation or native replay."]}
with (OUT/"verification.json").open("x",encoding="utf-8",newline="\n") as f:json.dump(report,f,indent=2);f.write("\n")
print(json.dumps({"passed":True,"payloads":28,"copied_pins":len(copied),"source_files":len(sources),"rows":count,"main_reproduction":"pass","witness_rows":len(witnesses),"defects":[]},indent=2))

