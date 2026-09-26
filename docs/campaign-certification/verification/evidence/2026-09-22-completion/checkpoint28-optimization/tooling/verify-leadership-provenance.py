import hashlib
import json
from pathlib import Path
import subprocess
import sys
import time

OUT = Path(__file__).resolve().parent
ROOT = Path.cwd()
if sys.argv[1:] != ['--run-after-source-freeze']:
    raise SystemExit('Preparation only: require --run-after-source-freeze after parent confirms final pins/source.')
FILES = ['spheres-web/data/leadership_production_2035.json'] + [
    'docs/campaign-certification/C01/' + name for name in
    ('census.json', 'countries.json', 'represented-organizations.json', 'roles-and-lifecycle.json', 'work-orders.json')]
FILES += ['docs/campaign-certification/C01/research-index.json']
FILES = [name for name in FILES if (ROOT / name).exists()]
INPUTS = ['spheres-sim/src/government.rs', 'spheres-sim/src/nations.rs',
          'tools/avatars/leadership_production.py', 'tools/avatars/campaign_census.py', 'tools/avatars/campaign_research.py']
sha = lambda data: hashlib.sha256(data).hexdigest()

def normalize_source_to_lf(path):
    raw = path.read_bytes()
    canonical = raw.replace(b'\r\n', b'\n')
    changed = canonical != raw
    if changed:
        raise RuntimeError("Government source must be canonical LF before the final source freeze; no source writes are allowed here.")
    return raw, changed

# Only a frozen-source preparation step may change bytes. An already canonical
# source must keep its mtime so a validation pass cannot invalidate Cargo inputs.
gov = ROOT / 'spheres-sim/src/government.rs'
raw, government_normalized = normalize_source_to_lf(gov)
INPUTS += ['spheres-sim/src/opening_mandates.rs', 'spheres-sim/data/opening_mandates_1990.json', 'spheres-sim/src/army_institutions.rs', 'spheres-sim/data/army_institutions_1990.json', 'spheres-sim/data/army_institution_coverage_1990.json', 'spheres-sim/src/army_authority.rs', 'spheres-sim/data/army_authority_1989.json', 'tools/politics/build_army_authority.py', 'tools/politics/extract_army_authority.py', 'tools/politics/fixtures/vdem-v16-1989-military-authority.json']
INPUTS = list(dict.fromkeys(INPUTS + ['spheres-sim/src/lib.rs', 'spheres-sim/src/blocs.rs', 'spheres-sim/src/politics.rs', 'spheres-sim/src/stratagems.rs', 'spheres-sim/src/world.rs', 'spheres-sim/src/economy.rs', 'spheres-sim/src/init.rs', 'spheres-sim/tests/treasury.rs', 'spheres-sim/data/resources_1990.json', 'spheres-web/data/district_resources.json', 'tools/resources/resource_coverage.py', 'tools/resources/make_resources.py', 'tools/resources/check.py', 'tools/resources/check_resources.py', 'spheres-sim/src/nations.rs', 'spheres-sim/src/government.rs', 'spheres-sim/data/party_leaders.json', 'spheres-sim/data/party_executive_eligibility.json', 'spheres-sim/data/future_party_leadership.json', 'spheres-sim/data/future_party_leadership_seats.json', 'spheres-sim/data/future_party_continuation.json', 'spheres-web/data/nation_figures.json', 'spheres-web/data/future_candidates_2035.json', 'spheres-web/data/person_portraits.json', 'spheres-web/data/fictional_portraits.json', 'spheres-web/data/leadership_production_2035.json', 'tools/avatars/leadership_production.py', 'tools/avatars/person_art_pipeline.py', 'tools/avatars/fictional_art_pipeline.py', 'tools/avatars/campaign_census.py']))
INPUTS += ['spheres-web/src/main.rs', 'spheres-sim/src/stratagems/tests.rs', 'spheres-sim/src/dyads.rs']
INPUTS = [name for name in INPUTS if name not in FILES]
input_mtimes_before = {name: (ROOT / name).stat().st_mtime_ns for name in INPUTS}
before = {name: (ROOT / name).read_bytes() for name in FILES}
inputs = {name: sha((ROOT / name).read_bytes()) for name in INPUTS}
git = lambda *args: subprocess.check_output(['git', *args], cwd=ROOT, text=True).strip()
result = {'revision': git('rev-parse', 'HEAD'), 'python': sys.version, 'scope': 'Checkpoint28 pure empty-stall-mask dyads optimization provenance validation; metadata coherence only, not campaign certification or expanded historical coverage. Parent reverted rejected candidate27 before this source freeze.', 'government_crlf_pairs_removed': raw.count(b'\r\n'), 'government_file_written': government_normalized, 'commands': [],
          'source_inputs_before': inputs, 'initial_status': git('status', '--short')}
commands = [
    ('production-build', ['tools/avatars/leadership_production.py', 'build']),
    ('production-check', ['tools/avatars/leadership_production.py', 'check']),
    ('production-self-test', ['tools/avatars/leadership_production.py', 'self-test']),
    ('census-build', ['tools/avatars/campaign_census.py']),
    ('census-check', ['tools/avatars/campaign_census.py', '--check']),
    ('census-unit', ['-m', 'unittest', 'discover', '-s', 'tools/avatars', '-p', 'test_campaign_census.py', '-v']),
    ('research-check', ['tools/avatars/campaign_research.py', '--check']),
]
for name, args in commands:
    command = [sys.executable, '-B', '-X', 'utf8', *args]
    started = time.monotonic()
    run = subprocess.run(command, cwd=ROOT, stdout=subprocess.PIPE, stderr=subprocess.STDOUT)
    (OUT / (name + '.log')).write_bytes(run.stdout)
    row = {'name': name, 'command': command, 'exit_code': run.returncode,
           'seconds': round(time.monotonic() - started, 3), 'log_sha256': sha(run.stdout)}
    result['commands'].append(row)
    print(json.dumps(row), flush=True)

def differences(a, b, path=''):
    if isinstance(a, dict) and isinstance(b, dict):
        for key in sorted(set(a) | set(b)):
            yield from differences(a.get(key), b.get(key), path + '/' + str(key))
    elif isinstance(a, list) and isinstance(b, list) and len(a) == len(b):
        for i, (x, y) in enumerate(zip(a, b)):
            yield from differences(x, y, path + '/' + str(i))
    elif a != b:
        yield {'field': path, 'before': a, 'after': b}

result['files'] = []
for name in FILES:
    after = (ROOT / name).read_bytes()
    row = {'path': name, 'before_sha256': sha(before[name]), 'after_sha256': sha(after),
           'byte_identical': before[name] == after,
           'differences': list(differences(json.loads(before[name]), json.loads(after)))}
    result['files'].append(row)
result['source_inputs_after'] = {name: sha((ROOT / name).read_bytes()) for name in INPUTS}
result['sources_unchanged_during_run'] = result['source_inputs_before'] == result['source_inputs_after']
result['source_input_mtimes_unchanged'] = input_mtimes_before == {name: (ROOT / name).stat().st_mtime_ns for name in INPUTS}
result['passed'] = all(row['exit_code'] == 0 for row in result['commands']) and result['sources_unchanged_during_run'] and result['source_input_mtimes_unchanged']
result['final_status'] = git('status', '--short')
(OUT / 'leadership-provenance-result.json').write_text(json.dumps(result, indent=2) + '\n', encoding='utf-8', newline='\n')
(OUT / 'leadership-provenance.diff').write_text(git('diff', '--', *FILES) + '\n', encoding='utf-8', newline='\n')
print(json.dumps({'passed': result['passed'], 'files': result['files'], 'sources_unchanged': result['sources_unchanged_during_run']}))
sys.exit(0 if result['passed'] else 1)
