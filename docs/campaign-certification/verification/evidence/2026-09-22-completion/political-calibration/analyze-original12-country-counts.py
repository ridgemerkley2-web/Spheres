"""Parse a completed fixed original-A1 diagnostic; never runs a simulation."""
from pathlib import Path
from fractions import Fraction
import hashlib,json,sys

if len(sys.argv)!=2:raise SystemExit('Usage: analyze-original12-country-counts.py <completed diagnostic log>')
path=Path(sys.argv[1]);raw=path.read_bytes()
lines=raw.decode('utf-16' if raw.startswith(b'\xff\xfe') else 'utf-8-sig').splitlines()
rows=[]
for line in lines:
    if 'A1_COUNTRY_COUNTS ' not in line:continue
    row=json.loads(line.split('A1_COUNTRY_COUNTS ',1)[1])
    assert row['months']==252
    pairs=row['country_counts'];counts=[c for _,c in pairs]
    assert len(set(n for n,_ in pairs))==len(pairs)
    assert all(isinstance(c,int) and c>0 for c in counts)
    n=sum(counts);assert n==row['elected_coups']
    top=Fraction(sum(sorted(counts,reverse=True)[:3]),n) if n else Fraction(0)
    square_sum=sum(c*c for c in counts)
    hhi=Fraction(square_sum,n*n) if n else None
    effective=Fraction(n*n,square_sum) if n else None
    rows.append({**row,'top_three_exact':str(top),'top_three':float(top),
        'HHI_exact':str(hhi) if hhi is not None else None,
        'effective_countries_exact':str(effective) if effective is not None else None,
        'effective_countries':float(effective) if effective is not None else None,
        'original_per_seed_top_three_under_half':top<Fraction(1,2),
        'proposed_per_seed_effective_at_least_four':effective>=4 if effective is not None else None})
rows.sort(key=lambda r:r['seed'])
assert [r['seed'] for r in rows]==list(range(12)),'Require the entire unchanged original12 cohort exactly once'
def median(values):
    values=sorted(values);assert len(values)==12
    return (values[5]+values[6])/2
count_median=median([Fraction(r['elected_coups']) for r in rows])
top_median=median([Fraction(r['top_three_exact']) for r in rows])
eff_median=None if any(r['effective_countries_exact'] is None for r in rows) else median([Fraction(r['effective_countries_exact']) for r in rows])
result={'schema':1,'scope':'Existing original A1 seeds0..11 over252months; no new sample',
    'source_log':str(path.resolve()),'source_log_sha256':hashlib.sha256(raw).hexdigest(),
    'original_A1_diagnostic':[line for line in lines if line.startswith('A1: ')],
    'source_test_failure_preserved':any('test result: FAILED.' in line for line in lines),
    'rows':rows,'median_coups':float(count_median),'median_top_three_exact':str(top_median),
    'median_top_three':float(top_median),'original_count_band_pass':4<=count_median<=14,
    'original_concentration_pass':top_median<Fraction(1,2),
    'median_effective_exact':str(eff_median) if eff_median is not None else None,
    'median_effective':float(eff_median) if eff_median is not None else None,
    'proposed_median_effective_at_least_four':eff_median>=4 if eff_median is not None else None,
    'per_seed_old_rule_passes':sum(r['original_per_seed_top_three_under_half'] for r in rows),
    'per_seed_proposed_rule_passes':sum(r['proposed_per_seed_effective_at_least_four'] is True for r in rows),
    'interpretation':'Comparison only. A new diversity criterion is not approved or implemented; original gate and failed exit remain unchanged. Per-seed pass counts are explanatory, not additional assertions.'}
output=path.with_name(path.stem+'-comparison.json')
assert not output.exists(),'Preserve an existing analysis'
output.write_text(json.dumps(result,indent=2)+'\n',encoding='utf-8')
print(json.dumps({k:result[k] for k in ['median_coups','median_top_three','original_concentration_pass','median_effective','proposed_median_effective_at_least_four','per_seed_old_rule_passes','per_seed_proposed_rule_passes']}))
