"""Verify recorded bytes and test outcomes, not historical interpretation."""
from pathlib import Path
import hashlib
import json
import subprocess

here = Path(__file__).resolve().parent
root = Path(subprocess.check_output(['git', 'rev-parse', '--show-toplevel'], cwd=here, text=True).strip())


def read(name):
    return json.loads((here / name).read_bytes())


def verify(pin, data):
    assert (len(data), hashlib.sha256(data).hexdigest()) == (pin['bytes'], pin['sha256']), pin['path']


manifest = read('manifest.json')
actual = {p.relative_to(here).as_posix() for p in here.rglob('*')
          if p.is_file() and p.name != 'manifest.json' and '__pycache__' not in p.parts}
assert actual == {p['path'] for p in manifest}
for pin in manifest:
    verify(pin, (here / pin['path']).read_bytes())
for pin in read('reviewed-inputs.json'):
    verify(pin, subprocess.check_output(['git', 'show', pin['revision'] + ':' + pin['path']], cwd=root))

required = ['avatar-suite-complete-fixture', 'planning-suite-complete-fixture', 'ui-suite',
            'campaign-tooling', 'metadata-tests-final', 'research-check', 'census-check',
            'gap-check', 'boundary-check-final', 'workboard-final', 'whitespace-final',
            'receipt-47-full', 'receipt-48-final', 'receipt-49-final', 'receipt-50', 'receipt-51-full']
for label in required:
    result = read('validation/' + label + '.json')
    assert result['exit_code'] == 0, label
    verify({'path': result['log'], 'bytes': result['bytes'], 'sha256': result['sha256']},
           (here / 'validation' / result['log']).read_bytes())
for label in ['avatar-suite', 'planning-suite']:
    assert read('validation/' + label + '.json')['exit_code'] != 0, 'Original failure must remain: ' + label

scope = read('scope.json')
assert (scope['new_sources'], scope['new_claims'], scope['accepted_office_observations']) == (80, 126, 41)
assert scope['canonical_qualification'] is False and scope['native_build_or_campaign_run'] is False
changed_runtime = subprocess.check_output([
    'git', 'diff', '--name-only', scope['base'], scope['reviewed_revision'], '--',
    'spheres-sim', 'spheres-web', '.github'], cwd=root).strip()
assert not changed_runtime
for rel in ['docs/planning/campaign-pathway.json']:
    assert subprocess.check_output(['git', 'show', scope['base'] + ':' + rel], cwd=root) == subprocess.check_output(
        ['git', 'show', scope['reviewed_revision'] + ':' + rel], cwd=root), rel
print('PASS: combined receipt, 145 Git inputs, passing checks and preserved initial failures; no runtime or qualification change.')
