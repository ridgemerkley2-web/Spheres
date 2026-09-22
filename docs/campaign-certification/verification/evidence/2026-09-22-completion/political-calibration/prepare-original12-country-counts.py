from pathlib import Path
from datetime import datetime, timezone
from fractions import Fraction
import hashlib,json,subprocess

here=Path(__file__).resolve().parent
repo=here.parents[1]/'integration'
sha=lambda b:hashlib.sha256(b).hexdigest()
source=repo/'spheres-sim/tests/bloc_census.rs'
raw=source.read_bytes()
newline=b'\r\n' if b'\r\n' in raw else b'\n'
needle=b'    v.sort_by(|a, b| b.1.cmp(&a.1).then(a.0.cmp(b.0)));\n    out.top3_share = if out.coup_el > 0 {'.replace(b'\n',newline)
assert raw.count(needle)==1
insert=b'''    v.sort_by(|a, b| b.1.cmp(&a.1).then(a.0.cmp(b.0)));
    eprintln!("A1_COUNTRY_COUNTS {}", serde_json::json!({
        "seed": seed, "months": months, "elected_coups": out.coup_el,
        "country_counts": &v
    }));
    out.top3_share = if out.coup_el > 0 {'''.replace(b'\n',newline)
instrumented=raw.replace(needle,insert)
start=raw.index(b'fn a1_coups_against_elected_governments_are_spread_not_three_micro_polities()')
end=raw.index(b'\n}',start)+2
body=raw[start:end]
assert instrumented.count(body)==1
assert instrumented.replace(insert,needle)==raw
out=here/'iteration-25-original12-country-counts.rs'
assert not out.exists()
out.write_bytes(instrumented)
inputs=[]
paths=subprocess.check_output(['git','ls-files','-z'],cwd=repo).decode().split('\0')
for rel in sorted(p for p in paths if p and (p.startswith('spheres-sim/') or p in ['Cargo.toml','Cargo.lock','.gitattributes'])):
    p=repo/rel
    if not p.is_file():continue
    b=p.read_bytes();inputs.append({'path':rel,'bytes':len(b),'raw_sha256':sha(b),'canonical_sha256':sha(b.replace(b'\r\n',b'\n'))})
plan={'schema':1,'prepared_utc':datetime.now(timezone.utc).isoformat(),
 'status':'Prepared only; not compiled or executed',
 'source_commit':subprocess.check_output(['git','rev-parse','HEAD'],cwd=repo).decode().strip(),
 'source_original_path':'spheres-sim/tests/bloc_census.rs','source_original_sha256':sha(raw),
 'instrumented_source':out.name,'instrumented_sha256':sha(instrumented),'unchanged_A1_body_sha256':sha(body),
 'scope':'Original A1 invokes unchanged census(12,252), seeds0..11, no commands, default rules except both ideology switches. No new cohort or held-out seeds.',
 'only_change':'One stderr JSON emitter after sorting complete per-seed elected-coup country counts; identical run_seed, event accounting, output.top3_share and both original assertions.',
 'compile_recipe':'rustc --edition=2021 --test -O iteration-25-original12-country-counts.rs --extern spheres_sim=<verified current25 rlib> --extern serde_json=<verified serde_json rlib> -L dependency=<deps> -o <external executable>',
 'run_arguments':['--ignored','--exact','a1_coups_against_elected_governments_are_spread_not_three_micro_polities','--nocapture','--test-threads=1'],
 'environment':'Clear SPHERES_CENSUS_VERBOSE to keep log bounded. Seed/month environment variables do not affect original A1 census().',
 'expected_result':'The original top-three assertion is expected to fail; preserve that failure and exit code. The diagnostic does not approve or implement another criterion.',
 'comparison_definition':{'total':'N=sum(country counts)','top_three':'sum of three largest country counts / N, per original gate','HHI':'sum((count/N)^2)','effective_country_count':'1/HHI=N^2/sum(count^2) for N>0','zero_event_case':'Report undefined effective count; do not invent diversity or silently omit a seed','aggregation':'Report all12 per-seed values, number meeting each rule and their medians. Median effective-count >=4 is displayed only as a proposed comparison, never acceptance. Original count median4..14 remains separately visible.'},
 'invariance_checks':['Original A1 function body byte-identical','Removing exactly the emitter reconstructs original file byte-for-byte','All12 complete count totals must equal emitted elected_coups','Recomputed original median must equal its failing assertion diagnostic'],
 'inputs':inputs}
(here/'iteration-25-original12-country-counts-plan.json').write_text(json.dumps(plan,indent=2)+'\n',encoding='utf-8')
toys=[]
for label,counts in [('Four coups in four distinct countries',[1,1,1,1]),('Seven coups across six countries, one repeat',[2,1,1,1,1,1]),('Seven coups across three countries, distribution3/2/2',[3,2,2])]:
    n=sum(counts);ss=sum(c*c for c in counts);top=Fraction(sum(sorted(counts,reverse=True)[:3]),n);eff=Fraction(n*n,ss)
    toys.append({'label':label,'counts':counts,'total':n,'top3_exact':str(top),'top3':float(top),'original_top3_under_half':top<Fraction(1,2),'hhi_exact':str(Fraction(ss,n*n)),'effective_exact':str(eff),'effective':float(eff),'candidate_effective_at_least_four':eff>=4})
(here/'original12-diversity-toy-comparison.json').write_text(json.dumps({'schema':1,'purpose':'Mathematical examples only; no gameplay criterion approved','count_band':'All three totals lie in4..14; examples isolate diversity rule','cases':toys},indent=2)+'\n',encoding='utf-8')
note='''# Prospective comparison of two diversity statistics

No criterion change is approved or implemented. The original A1 code still
asserts median elected coups in 4..14 and median top-three share strictly below
.50. The external copy adds only complete count output for original seeds0..11.
Compilation and execution must wait for the parent to release the native build.

For counts c_i totaling N, top-three share is the three largest counts divided
by N. HHI is sum((c_i/N)^2), and its inverse N^2/sum(c_i^2) is the effective
country count. The latter measures the number of equally represented countries
with the same concentration; it is not the literal number of distinct countries.

| Toy distribution | Top-three share | Original <.50 | Effective countries | Proposed >=4 |
|---|---:|---|---:|---|
| Four unique coups: 1,1,1,1 | 3/4 = .750 | Fail | 4 | Pass |
| Seven across six: 2,1,1,1,1,1 | 4/7 = .571 | Fail | 49/9 = 5.444 | Pass |
| Seven across three: 3,2,2 | 1.000 | Fail | 49/17 = 2.882 | Fail |

These totals all satisfy the 4..14 count band. The proposed criterion plainly
accepts distributions rejected by the current gate; it is a substantive change
of the diversity requirement, not an equivalent formula or a repair of a
failing implementation. Four distinct countries alone also does not guarantee
effective count4: unequal event shares reduce the effective count.

The diagnostic will report every per-seed distribution and both statistics,
then the medians and numbers of seeds satisfying each rule. Any median of the
new statistic is a proposed interpretation for review, not an adopted gate.
The original failed test and its exit code remain part of the evidence.
'''
(here/'original12-diversity-toy-comparison.md').write_text(note,encoding='utf-8')
print(json.dumps({'source':out.name,'sha256':sha(instrumented),'unchanged_A1_sha256':sha(body),'input_count':len(inputs),'executed':False}))
