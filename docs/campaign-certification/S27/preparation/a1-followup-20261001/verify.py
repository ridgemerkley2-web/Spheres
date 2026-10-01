"""Offline packet validation; does not rebuild, run simulation or fetch sources."""
import hashlib
import json
from pathlib import Path
import re
import subprocess

OUT = Path(__file__).resolve().parent
ROOT = next(p for p in OUT.parents if (p / '.git').exists())
def load(name):
    return json.loads((OUT / name).read_text(encoding='utf-8'))
def sha(path):
    with path.open('rb') as stream:
        return hashlib.file_digest(stream, 'sha256').hexdigest()
manifest = load('manifest.json')
for row in manifest['payloads']:
    path = OUT / row['path']
    assert path.stat().st_size == row['bytes'], row['path']
    assert sha(path) == row['sha256'], row['path']
build = load('build-02/build-result.json')
preflight = load('build-02/build-preflight.json')
timing = load('resource-timing-result.json')
analysis = load('existing-analysis.json')
window = json.loads((OUT / 'confirmation-window.json').read_text(encoding='utf-8-sig'))
assert window['status'] == 'confirmation_not_run'
assert load('build-result.json')['exit_code'] == 101
assert build['exit_code'] == 0 and build['source_pins_unchanged']
assert preflight['sim_tree'] == preflight['accepted_sim_tree'] == 'e40ea34de1ab2b07ae0b5d6cefc188d733bca05a'
assert timing['exit_code'] == 0 and timing['binary_unchanged']
assert timing['binary_before'] == build['binaries'][0]
assert timing['binary_sha256_after'] == build['binaries'][0]['sha256']
assert sha(OUT / 'resource-timing.log') == timing['log_sha256']
assert sha(OUT / 'timing-environment.json') == timing['environment_sha256']
log = (OUT / 'resource-timing.log').read_text(encoding='utf-8')
assert '1 passed; 0 failed' in log and '1118 filtered out' in log
assert 'budget 0.05 ms/month NOT met' in log
assert 'total           0.0623 ms/month' in log
assert 'Compiling ' not in log
assert analysis['accepted_cohort_summary']['median_top_three_share'] == 4 / 7
assert analysis['accepted_cohort_summary']['median_coups'] == 7.5
assert analysis['accepted_cohort_summary']['strict_share_pass_seeds'] == 1
assert analysis['observer_is_accepted_repaired_runtime'] is False
assert sum(analysis['observer_stage_counts'].values()) == 185260
assert sum(len(c['firing_dates']) for c in analysis['observer_country_summaries']) == 10
assert not subprocess.check_output(['git', 'diff', '7c6f112c', preflight['head'], '--',
                                   'spheres-sim', 'Cargo.toml', 'Cargo.lock'], cwd=ROOT)
links = 0
for name in ('README.md', 'mechanism-proposal.md'):
    for link in re.findall(r'\[[^\]]+\]\(([^)]+)\)', (OUT / name).read_text(encoding='utf-8')):
        if '://' not in link:
            assert (OUT / link.split('#')[0]).exists(), link
            links += 1
print(json.dumps({'passed': True, 'payloads': len(manifest['payloads']), 'local_links': links,
                  'resource_test_passed': 1, 'resource_test_failed': 0,
                  'total_ms_per_month': 0.0623, 'required_bar': 0.15,
                  'aspirational_budget_0_05_met': False, 'uncontended_confirmation': 'pending',
                  'A1_pass_claimed': False}))
