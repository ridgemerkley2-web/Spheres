"""Prepare reviewable future test-only patch and preserve failed-run context."""
import datetime, difflib, hashlib, json, pathlib
base = pathlib.Path(__file__).resolve().parent
repo = base.parent.parent / 'integration'
sha = lambda p: hashlib.sha256(p.read_bytes()).hexdigest()
items = [('tools/ui/ci-supplier-imports.cjs', base/'ci-supplier-imports.cjs'),
         ('tools/ui/supplier-archive-audit.cjs', base/'archive-audit.cjs'),
         ('tools/ui/archive-worker.py', base/'archive-worker.py')]
patch = []
for destination, source in items:
    old = (repo/destination).read_text(encoding='utf-8').splitlines(True) if (repo/destination).exists() else []
    patch.extend(difflib.unified_diff(old, source.read_text(encoding='utf-8').splitlines(True),
        fromfile='a/'+destination if old else '/dev/null', tofile='b/'+destination))
(base/'future-tracked-harness.patch').write_text(''.join(patch), encoding='utf-8', newline='\n')
diagnostic = base/'diagnostic-2/result.json'
result = json.loads(diagnostic.read_text(encoding='utf-8'))
failed = repo/'artifacts/browser-supplier-imports-ci/supplier-g630Bz'
report = {
    'record_origin': 'Supplementary bounded audit after the interrupted browser run; original termination-result.json is unchanged.',
    'passed': False,
    'full_journey_passed': False,
    'captured_utc': datetime.datetime.now(datetime.timezone.utc).isoformat(),
    'candidate': 'e6187df8044556a8262154e7a6c8baa32b283799',
    'binary_sha256': 'd878f36a4dec2c83cfb7477a49ce678d4579a26388b0b45f2504c974bba2ee90',
    'termination_record': {'path': str(failed/'termination-result.json'), 'sha256': sha(failed/'termination-result.json')},
    'diagnostic': {'path': str(diagnostic), 'sha256': sha(diagnostic), 'completed': result['completed'], 'comparison': result['comparison']},
    'finding': 'The retained delivered and maintenance-approved worlds match exactly across 121248846 typed canonical bytes after excluding only the approved Tonga equipment maintenance_plan. Signed zero is preserved. No other native state difference explains the interrupted assertion.',
    'limitations': ['No new browser was run.', 'The interrupted original journey did not reach a final result or prove later paid maintenance/save/load assertions.',
                    'The process crash cause remains unproven. Retained API bodies and full-world copies were concrete harness resource risks and are removed in the proposed candidate.'],
    'future_harness_files': {destination: {'source': str(source), 'sha256': sha(source)} for destination, source in items},
    'future_patch': {'path': str(base/'future-tracked-harness.patch'), 'sha256': sha(base/'future-tracked-harness.patch')}
}
target=base/'postmortem-result.json'
with target.open('x', encoding='utf-8') as stream:
    json.dump(report,stream,indent=2);stream.write('\n')
print(json.dumps({'postmortem':str(target),'future_patch':report['future_patch']},indent=2))
