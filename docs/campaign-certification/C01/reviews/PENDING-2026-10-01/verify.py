"""Verify the compact pending-review evidence offline; never grant campaign gates."""
import hashlib
import json
from pathlib import Path

root = Path(__file__).resolve().parent
manifest = json.loads((root / 'manifest.json').read_text(encoding='utf-8'))
expected = {row['path'] for row in manifest['files']}
actual = {p.relative_to(root).as_posix() for p in root.rglob('*')
          if p.is_file() and p.name != 'manifest.json' and '__pycache__' not in p.parts}
assert expected == actual, (expected - actual, actual - expected)
for row in manifest['files']:
    raw = (root / row['path']).read_bytes()
    assert len(raw) == row['bytes'], row['path']
    assert hashlib.sha256(raw).hexdigest() == row['sha256'], row['path']
for phase in ('generation-01', 'validation-01', 'generation-02', 'final-metadata-01'):
    folder = root / 'validation' / phase
    results = json.loads((folder / 'results.json').read_text(encoding='utf-8'))
    assert results['passed'] and all(c['exit_code'] == 0 for c in results['commands'])
    for command in results['commands']:
        raw = (folder / command['log']).read_bytes()
        assert len(raw) == command['bytes'] and hashlib.sha256(raw).hexdigest() == command['sha256']
assert manifest['combined_tests'] == {'avatars': 640, 'planning': 69, 'node': 11, 'total': 720}
assert manifest['decisions'] == {'CLAUDE-C01-31': 'accepted_bounded_research',
                                  'CLAUDE-C01-28': 'held', 'CLAUDE-C01-39': 'held'}
assert not manifest['runtime_changed'] and not manifest['parent_gates_closed']
print('Verified', len(expected), 'receipt files; 720 combined tests recorded; one bounded acceptance and two holds.')
