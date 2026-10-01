"""Verify the retained CP1 preparation packet without claiming qualification."""
from pathlib import Path
import hashlib
import json
import re

HERE = Path(__file__).resolve().parent


def load(path):
    return json.loads((HERE / path).read_text(encoding='utf-8'))


manifest = load('manifest.json')
for item in manifest['files']:
    raw = (HERE / item['path']).read_bytes()
    assert len(raw) == item['bytes'], item['path']
    assert hashlib.sha256(raw).hexdigest() == item['sha256'], item['path']

for name in ('preparation-01', 'accepted-preparation-01', 'combined-metadata-01'):
    records = load(f'validation/{name}/results.json')
    assert all(row['exit_code'] == 0 for row in records), name
assert load('validation/combined-campaign-01/run-02/results.json')['all_commands_passed']
assert load('validation/combined-metadata-01/scope-and-links.json')['passed']
test_logs = {
    'validation/preparation-01/playtest-tool.log': 23,
    'validation/accepted-preparation-01/tonga-tests.log': 31,
    'validation/combined-metadata-01/planning-tests.log': 69,
    'validation/combined-metadata-01/ledger-boundary-tests.log': 65,
    'validation/combined-campaign-01/run-02/campaign-tooling.stderr.log': 105,
}
for path, expected in test_logs.items():
    text = (HERE / path).read_text(encoding='utf-8')
    assert re.search(rf'Ran {expected} tests? in ', text), path
    assert '\nOK' in text and '\nFAILED' not in text, path
assert sum(test_logs.values()) == manifest['test_executions'] == 293
assert manifest['test_passes'] == 292 and manifest['test_skips'] == 1
assert manifest['runtime_changed'] is False and manifest['parent_gates_closed'] is False
cleanup = load('validation/cleanup-result.json')
assert len(cleanup['candidates']) == 4 and all(row['removed'] for row in cleanup['candidates'])
assert sum(row['logicalBytes'] for row in cleanup['candidates']) == 6027670895
print(json.dumps({'passed': True, 'payloads': len(manifest['files']),
                  'test_executions': 293, 'test_passes': 292, 'test_skips': 1,
                  'closed_bounded_tasks': manifest['closed_bounded_tasks'],
                  'runtime_changed': False, 'parent_gates_closed': False}))
