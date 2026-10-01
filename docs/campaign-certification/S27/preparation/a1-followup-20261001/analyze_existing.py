"""Analyze existing outcomes and the retained pre-repair observer; runs no simulation."""
from collections import Counter, defaultdict
from fractions import Fraction
import gzip
import hashlib
import json
from pathlib import Path
import re
from statistics import median

OUT = Path(__file__).resolve().parent
PRIOR = OUT.parent / 'political-repairs-20260930'
TRACE = Path('C:/Users/ridge/spheres-political-repair-backup-20260930/retained-observation/opening-baseline/observations.jsonl.gz')
with TRACE.open('rb') as stream:
    compressed_hash = hashlib.file_digest(stream, 'sha256').hexdigest()
manifest = json.loads((PRIOR / 'baseline-manifest.json').read_text(encoding='utf-8'))
assert compressed_hash == manifest['opening-baseline/observations.jsonl']['compressed_sha256']

cohort = []
text = (PRIOR / 'institutions-distribution.log').read_text(encoding='utf-8')
for line in text.splitlines():
    m = re.match(r's\s+(\d+)\s+\|\s+(\d+) .*\|\s+(0\.\d+) (\[.*\])$', line)
    if not m:
        continue
    seed, total, printed, names = int(m[1]), int(m[2]), float(m[3]), json.loads(m[4])
    # Exact integer numerator recoverable uniquely from the rounded share and count.
    candidates = [n for n in range(total + 1) if abs(n / total - printed) <= 0.005000001]
    assert len(candidates) == 1
    share = Fraction(candidates[0], total)
    cohort.append({'seed': seed, 'coups': total, 'top_three_coups': candidates[0],
                   'exact_top_three_share': str(share), 'share': float(share),
                   'printed_top_five_nations': names, 'strict_share_pass': share < Fraction(1, 2)})
assert [r['seed'] for r in cohort] == list(range(12))

stage_counts = Counter()
countries = defaultdict(lambda: {'stages': Counter(), 'electoral_months': 0, 'trigger_branches': Counter(),
                                 'conditions': Counter(), 'first_witnesses': {}, 'max_pressure': 0.0,
                                 'minimum_effective_loyalty': 1.0, 'max_discontent': 0.0,
                                 'firing_dates': [], 'funding_before': Counter()})
decoded_hash = hashlib.sha256()
decoded_bytes = 0
with gzip.open(TRACE, 'rb') as stream:
    for line in stream:
        decoded_hash.update(line)
        decoded_bytes += len(line)
        row = json.loads(line)
        stage = row['stage']
        stage_counts[stage] += 1
        if row.get('missing_nation'):
            continue
        c = countries[row['country']]
        c['stages'][stage] += 1
        g, a, d, f, t = (row[k] for k in ('government', 'army', 'discontent', 'fiscal', 'thresholds'))
        if stage.startswith('trigger_'):
            c['trigger_branches'][stage] += 1
            if stage == 'trigger_firing':
                c['firing_dates'].append(row['date'])
        if stage == 'army_after_walk':
            c['electoral_months'] += 1
            c['max_pressure'] = max(c['max_pressure'], a['pressure'])
            c['minimum_effective_loyalty'] = min(c['minimum_effective_loyalty'], a['effective_loyalty'])
            c['max_discontent'] = max(c['max_discontent'], d['total'])
            conditions = {
                'hostile_army': a['effective_loyalty'] < t['army'],
                'crisis': d['total'] >= t['discontent'],
                'settled': g['settled_months'] >= t['settled_months'] and not g['awaiting_first_election'],
                'pressure_ready': a['pressure'] >= t['pressure'],
                'civilian_confidence_loss': a['civilian_confidence_penalty'] > 0,
                'zero_executive_leverage': a['executive_leverage'] == 0,
                'zero_fiscal_headroom': f['affordable_military_share'] <= 0,
            }
            conditions['live_conditions'] = conditions['hostile_army'] and conditions['crisis']
            conditions['settled_live'] = conditions['settled'] and conditions['live_conditions']
            conditions['settled_live_pressure_ready'] = conditions['settled_live'] and conditions['pressure_ready']
            conditions['crisis_but_loyal_army'] = conditions['crisis'] and not conditions['hostile_army']
            for key, present in conditions.items():
                if present:
                    c['conditions'][key] += 1
                    c['first_witnesses'].setdefault(key, row)
        if stage == 'ai_before_funding':
            q = f['funding_quote']
            if g['electoral'] and g['has_army'] and a['raw_loyalty'] < 0.40:
                c['funding_before']['threatened_months'] += 1
                if q is None:
                    c['funding_before']['no_quote'] += 1
                elif not q['clears_existing_hysteresis']:
                    c['funding_before']['no_useful_appropriation'] += 1
                elif not q['standing_affordable']:
                    c['funding_before']['political_capital_shortfall'] += 1
                else:
                    c['funding_before']['useful_affordable_appropriation'] += 1
assert decoded_hash.hexdigest() == manifest['opening-baseline/observations.jsonl']['decoded_sha256']
assert decoded_bytes == manifest['opening-baseline/observations.jsonl']['decoded_bytes']
observer = json.loads((PRIOR / 'observer-result.json').read_text(encoding='utf-8'))
assert dict(stage_counts) == observer['stage_counts']
assert sum(len(c['firing_dates']) for c in countries.values()) == 10
witnesses = {}
summaries = []
for name, c in sorted(countries.items()):
    witnesses[name] = c.pop('first_witnesses')
    summaries.append({'country': name, **c})
result = {
    'scope': 'Offline analysis only; accepted repaired 12-seed outcomes are separate from the pre-repair seed0 observer. No new seed or simulation, no coefficient fitting, no historical inference.',
    'accepted_runtime': '7c6f112cbfb7e1ae02e70dcc2986da66f425aa49',
    'accepted_outcome_input': 'political-repairs-20260930/institutions-distribution.log',
    'accepted_cohort': cohort,
    'accepted_cohort_summary': {'median_coups': median(r['coups'] for r in cohort),
        'median_top_three_share': median(r['share'] for r in cohort),
        'strict_share_pass_seeds': sum(r['strict_share_pass'] for r in cohort),
        'exactly_half_seeds': sum(r['share'] == 0.5 for r in cohort),
        'fewer_than_seven_coups_seeds': sum(r['coups'] < 7 for r in cohort)},
    'observer_source': 'c41376f534a746b3e36633457e313176aeb1c461',
    'observer_is_accepted_repaired_runtime': False,
    'observer_trace_pin': {'path': str(TRACE), 'compressed_sha256': compressed_hash,
        'decoded_sha256': decoded_hash.hexdigest(), 'decoded_bytes': decoded_bytes},
    'observer_stage_counts': stage_counts, 'observer_country_summaries': summaries,
    'limits': 'Observer values describe the earlier baseline, not proof of a present residual defect. Stage observations are within-month and not independent trials; pooled country counts do not replace the per-seed A1 statistic.'
}
(OUT / 'existing-analysis.json').write_text(json.dumps(result, indent=2) + '\n', encoding='utf-8')
# Compact selected witnesses; full original evidence is retained and hash-bound above.
selected = ['Guatemala', 'Myanmar', 'SaoTome', 'Ecuador', 'Guyana', 'Comoros', 'Chad', 'Mozambique',
            'Haiti', 'Pakistan', 'Nigeria', 'Thailand', 'Sudan']
(OUT / 'mechanism-witnesses.json').write_text(json.dumps({n: witnesses.get(n, {}) for n in selected}, indent=2) + '\n', encoding='utf-8')
print(json.dumps(result['accepted_cohort_summary']))
for c in summaries:
    if c['country'] in selected:
        print(json.dumps({k: v for k, v in c.items() if k != 'stages'}))
