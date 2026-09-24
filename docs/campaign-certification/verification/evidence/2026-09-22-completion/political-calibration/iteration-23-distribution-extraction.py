from pathlib import Path
from collections import Counter
import ast, hashlib, json, re, statistics

here = Path(__file__).resolve().parent
source = here / 'iteration-23-development-n200.log'
raw = source.read_bytes()
body = raw.decode('utf-8-sig').replace('\r\n', '\n')
assert 'test result: ok. 1 passed; 0 failed;' in body, 'Cohort is not complete'
rows = []
for line in body.splitlines():
    if not re.match(r'^s\s*\d+\s*\|', line):
        continue
    fields = [part.strip() for part in line.split('|')]
    seed = int(fields[0][1:])
    coups, annulments, regime_coups = map(int, fields[1].split())
    a4 = int(fields[6].split()[0])
    final = re.fullmatch(r'([\d.]+) (\[.*\])', fields[-1])
    assert final, line
    rows.append(dict(seed=seed, elected_coups=coups, annulments=annulments,
        regime_coups=regime_coups, a4_opened_1996=a4,
        top3_share_printed=float(final[1]),
        reported_top5_coup_country_names=json.loads(final[2])))
assert [row['seed'] for row in rows] == list(range(200))
pool = re.search(r'^  elcoup\s+total\s+(\d+)\s+distinct\s+(\d+):\s*(\[.*\])$', body, re.M)
assert pool
total, distinct = map(int, pool.group(1,2))
countries = ast.literal_eval(pool[3])
assert sum(row['elected_coups'] for row in rows) == total
reported_sum = sum(count for _,count in countries)
complete = len(countries) == distinct and reported_sum == total
rank_appearances = Counter(name for row in rows for name in row['reported_top5_coup_country_names'])
result = dict(iteration=23,source_log_sha256=hashlib.sha256(raw).hexdigest(),
    source_manifest='iteration-23-source-input-manifest.json', seeds='0..199',months=252,
    scope='Extraction only, no new native simulation or trajectory',
    total_elected_coups=total,distinct_coup_nations=distinct,
    pooled_top3_share=sum(count for _,count in countries[:3])/total,
    pooled_country_counts_complete=complete,
    pooled_country_counts=[dict(country=name,events=count) for name,count in countries],
    median_per_seed_coups=statistics.median(row['elected_coups'] for row in rows),
    median_per_seed_top3_from_rounded_printed_values=statistics.median(row['top3_share_printed'] for row in rows),
    a4_openings=dict(median=statistics.median(row['a4_opened_1996'] for row in rows),
                     minimum=min(row['a4_opened_1996'] for row in rows),
                     maximum=max(row['a4_opened_1996'] for row in rows),
                     seeds_below_15=sum(row['a4_opened_1996']<15 for row in rows),
                     note='A4 constrains the median, not every individual campaign.'),
    first_and_repeat_coups=None,
    first_repeat_limit='Existing diagnostic prints only top-five country names per seed, with no per-country event counts; exact first/repeat decomposition is not recoverable. No additional trajectory was launched.',
    country_top5_appearances=dict(rank_appearances.most_common()),
    country_rank_limit='Top-five list is truncated; appearances are not complete country participation counts or event counts.',
    per_seed=rows)
(here/'iteration-23-coup-distribution.json').write_text(json.dumps(result,indent=2)+'\n',encoding='utf-8')
print(json.dumps({k:v for k,v in result.items() if k not in ('per_seed','country_top5_appearances')},indent=2))
