"""Bounded parser/classification check; never invoke collector.main or read campaigns."""
import ast
import datetime
import hashlib
import json
from pathlib import Path
import runpy

STAGING = Path(__file__).resolve().parent
BASE = STAGING.parent
SOURCE = BASE / 'collect-s08-evidence.py'
PIN = 'd770592aeead51f1313d507edd26b02d75a69bba'
INITIAL = 'e6187df8044556a8262154e7a6c8baa32b283799'
text = SOURCE.read_text(encoding='utf-8-sig')
tree = ast.parse(text)
module = runpy.run_path(str(SOURCE), run_name='collector_discovery_check')
main = next(node for node in tree.body if isinstance(node, ast.FunctionDef) and node.name == 'main')
parser_nodes = []
for node in main.body:
    if isinstance(node, ast.Assign) and any(isinstance(target, ast.Name) and target.id == 'args' for target in node.targets):
        break
    parser_nodes.append(node)
scope = dict(module)
exec(compile(ast.Module(body=parser_nodes, type_ignores=[]), str(SOURCE), 'exec'), scope)
parser = scope['parser']
args = parser.parse_args([PIN, '--supplier-browser-result', 'synthetic-current/result.json',
    '--supplier-browser-result', 'synthetic-initial/result.json', '--browser-result', 'synthetic-money/result.json'])
assert len(args.supplier_browser_result) == 2 and len(args.browser_result) == 1
assert parser.parse_args([PIN]).supplier_browser_result == []

fixtures = {}
roots = []
records = []
scope = dict(module, repo=BASE / 'integration', pin=PIN, browser_records=records, excluded_roots=[],
    read=lambda path: fixtures[str(path)], add=lambda path, logical, kind: roots.append((path, logical, kind)))
browser = next(node for node in main.body if isinstance(node, ast.FunctionDef) and node.name == 'add_browser')
exec(compile(ast.Module(body=[browser], type_ignores=[]), str(SOURCE), 'exec'), scope)
checks = ['parser repeat/default options']
for revision in [PIN, INITIAL]:
    path = Path('synthetic-' + revision[:7]) / 'result.json'
    fixtures[str(path)] = {'passed': False, 'build': {'revision': revision}, 'screenshots': []}
    scope['add_browser'](path, 'supplier_browser', verify_supplier=True)
    assert roots[-1][2] == 'supplier_browser'
    assert records[-1]['passed'] is False
    assert records[-1]['same_final_candidate'] == (revision == PIN)
    assert records[-1]['revision'] == revision
    assert 'revision checked' in records[-1]['scope']
    checks.append('verified supplier result preserves failed status and source ' + revision)
for revision in [None, PIN[:12], module['S08_BASE'], '0' * 40]:
    path = Path('synthetic-refused/result.json')
    fixtures[str(path)] = {'passed': False, 'build': {'revision': revision}}
    count = len(roots)
    try:
        scope['add_browser'](path, 'supplier_browser', verify_supplier=True)
    except ValueError as error:
        assert 'does not identify an S08 revision' in str(error)
    else:
        raise AssertionError('unverified supplier accepted: ' + str(revision))
    assert len(roots) == count
    checks.append('reject missing/short/pre-S08/unresolvable source ' + str(revision))

# Execute only the new option-dispatch loop from the actual source, never
# directory discovery, collector enumeration, hashing or package creation.
dispatch = next(node for node in main.body if isinstance(node, ast.For)
    and isinstance(node.iter, ast.Attribute) and node.iter.attr == 'supplier_browser_result')
for path, revision in zip(args.supplier_browser_result, [PIN, INITIAL]):
    fixtures[str(path.resolve())] = {'passed': False, 'build_evidence': {'revision': revision}, 'screenshots': []}
scope['args'] = args
exec(compile(ast.Module(body=[dispatch], type_ignores=[]), str(SOURCE), 'exec'), scope)
assert all(kind == 'supplier_browser' for _, _, kind in roots)
checks.append('actual option loop uses verified supplier classification')

named_save = Path('synthetic/s08-delivered.json')
assert module['campaign_file'](named_save, b'{"format":"spheres-campaign"}')
assert module['snapshot_allowed'](named_save, named_save.parent, 'supplier_browser')
assert not module['snapshot_allowed'](named_save, named_save.parent, 'records')
assert not module['snapshot_allowed'](named_save, named_save.parent, 'general_browser')
assert not module['snapshot_allowed'](Path('synthetic/unrelated.json'), named_save.parent, 'supplier_browser')
checks.append('named supplier campaigns are retained without widening ordinary/general roots')

# Only these small source files are hashed. No evidence archive is opened.
sha = lambda path: hashlib.sha256(path.read_bytes()).hexdigest()
result = {'captured_utc': datetime.datetime.now(datetime.timezone.utc).isoformat(),
    'scope': 'Parser and extracted add_browser/option-loop classification only; no collector execution or campaign reads.',
    'collector_before_sha256': sha(STAGING / 'collector-supplier-discovery.before.py'),
    'collector_after_sha256': sha(SOURCE), 'verification_runner_sha256': sha(Path(__file__)),
    'checks': checks, 'passed': True}
output = STAGING / 'collector-supplier-discovery-verification.json'
with output.open('x', encoding='utf-8') as handle:
    json.dump(result, handle, indent=2)
    handle.write('\n')
print(json.dumps(result, indent=2))
