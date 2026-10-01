"""Verify recorded evidence and exact tested scope; do not recreate source judgment."""
import hashlib
import json
import subprocess
from pathlib import Path

here = Path(__file__).resolve().parent
root = Path(subprocess.check_output(['git', 'rev-parse', '--show-toplevel'], cwd=here, text=True).strip())


def read(name):
    return json.loads((here / name).read_bytes())


def git(*args):
    return subprocess.check_output(['git', *args], cwd=root)


def verify(pin, data):
    assert (len(data), hashlib.sha256(data).hexdigest()) == (pin['bytes'], pin['sha256']), pin['path']


manifest = read('manifest.json')
actual = {p.relative_to(here).as_posix() for p in here.rglob('*')
          if p.is_file() and p.name != 'manifest.json' and '__pycache__' not in p.parts}
assert actual == {p['path'] for p in manifest}
for pin in manifest:
    verify(pin, (here / pin['path']).read_bytes())
for pin in read('reviewed-inputs.json'):
    verify(pin, git('show', pin['revision'] + ':' + pin['path']))

scope = read('scope.json')
for label in scope['required_validation_labels']:
    result = read('validation/' + label + '.json')
    assert result['exit_code'] == 0, label
    verify({'path': result['log'], 'bytes': result['bytes'], 'sha256': result['sha256']},
           (here / 'validation' / result['log']).read_bytes())
for label in ('avatar-suite', 'census-check', 'leadership-production-before'):
    assert read('validation/' + label + '.json')['exit_code'] != 0, label
assert (scope['new_sources'], scope['new_claims'], scope['accepted_office_observations']) == (92, 166, 36)
assert scope['avatar_tests_passed'] == 827 and scope['planning_tests_passed'] == 69
assert all(scope[k] is False for k in ('canonical_qualification', 'native_build_or_campaign_run',
                                     'human_approval_claimed', 'artwork_changed', 'gameplay_changed'))
base, revision = scope['base'], scope['reviewed_revision']
allowed = scope['allowed_game_tree_metadata_path']
changed_game = git('diff', '--name-only', base, revision, '--', 'spheres-sim', 'spheres-web', '.github').decode().splitlines()
assert changed_game == [allowed], changed_game
before = json.loads(git('show', base + ':' + allowed))
after = json.loads(git('show', revision + ':' + allowed))
source_path = 'spheres-sim/data/tonga_institutional_leadership.json'
current_hash = hashlib.sha256(git('show', revision + ':' + source_path)).hexdigest()
assert after['input_hashes'][source_path] == current_hash
assert after['future']['institutional_source_sha256'] == current_hash
assert before['input_hashes'][source_path] != current_hash
after['input_hashes'][source_path] = before['input_hashes'][source_path]
after['future']['institutional_source_sha256'] = before['future']['institutional_source_sha256']
assert before == after, 'Only two inherited source pins may change'
for path in ('docs/planning/campaign-pathway.json',
             'docs/campaign-certification/C06/countries/tonga/manifest.json'):
    assert git('show', base + ':' + path) == git('show', revision + ':' + path), path
queue = json.loads(git('show', revision + ':docs/planning/ai-task-queue.json'))
prior_queue = json.loads(git('show', base + ':docs/planning/ai-task-queue.json'))
updated = {'CLAUDE-C01-28', 'CLAUDE-C01-39'}
assert [t for t in queue['tasks'] if t['id'] not in updated] == [t for t in prior_queue['tasks'] if t['id'] not in updated]
claude = [t for t in queue['tasks'] if t['owner'] == 'Claude']
assert len(claude) == 38 and all(t['state'] == 'complete' for t in claude)
assert len(queue['tasks']) == 59
remote = read('final-remote-check.json')
assert remote['fetch_exit_code'] == 0 and remote['new_unreviewed_deliveries'] == 0
assert remote['remote_claude_tips'] == read('remote-inventory-proof.json')['raw_ref_inventory']
assert len(remote['remote_claude_tips']) == 17
assert read('composition-audit.json')['passed'] and read('final-scope-docs-audit.json')['passed']
print('PASS: exact tested research, 827 avatar / 69 planning tests, preserved failures, 38 resolved Claude tasks; metadata-only Tonga refresh and no campaign qualification.')
