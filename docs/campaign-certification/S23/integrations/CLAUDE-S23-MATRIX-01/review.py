"""Reproduce the bounded S23 review in a NEW output directory; never edit inputs.

The original-regressions mode intentionally expects the submitted implementation
to fail the added tests. It is a negative control, not a passing test result.
The current-integration audit overlays ONLY this reviewed tool's bytes onto the
read-only integration inputs because the tool has not yet been integrated there.
"""
import argparse
import gzip
import hashlib
import importlib
import json
from pathlib import Path
import subprocess
import sys
import types
import unittest
from datetime import datetime, timezone

SUBMISSION = '8c9f6ce1d533504143419af483e4435cde21835b'
TOOL = 'tools/avatars/certified_boundary_matrix.py'
TEST = 'tools/avatars/test_certified_boundary_matrix.py'


def now():
    return datetime.now(timezone.utc).isoformat()


def identity(raw):
    return {'bytes': len(raw), 'sha256': hashlib.sha256(raw).hexdigest()}


def git(root, *args):
    return subprocess.check_output(['git', '-C', str(root), *args])


def save(path, data):
    path.write_text(json.dumps(data, indent=2, ensure_ascii=False) + '\n', encoding='utf-8', newline='\n')


def main():
    parser = argparse.ArgumentParser()
    parser.add_argument('--root', type=Path, required=True)
    parser.add_argument('--integration', type=Path)
    parser.add_argument('--campaign', type=Path, action='append', default=[])
    parser.add_argument('--output', type=Path)
    parser.add_argument('--original-regressions', action='store_true')
    args = parser.parse_args()
    root = args.root.resolve()
    sys.path.insert(0, str(root / 'tools/avatars'))
    if args.original_regressions:
        module = types.ModuleType('certified_boundary_matrix')
        module.__file__ = str(root / TOOL)
        original = git(root, 'show', f'{SUBMISSION}:{TOOL}')
        exec(compile(original, f'git:{SUBMISSION}:{TOOL}', 'exec'), module.__dict__)
        sys.modules[module.__name__] = module
        tests = importlib.import_module('test_certified_boundary_matrix')
        result = unittest.TextTestRunner(verbosity=2).run(
            unittest.defaultTestLoader.loadTestsFromTestCase(tests.IndependentReviewRegressions))
        print(json.dumps({'scope': 'new regressions against exact submitted implementation; intentional negative control',
                          'submission': SUBMISSION, 'tests_run': result.testsRun,
                          'failed_assertions_including_subtests': len(result.failures), 'errors': len(result.errors)}))
        return 0 if result.wasSuccessful() else 1

    out = args.output.resolve()
    out.mkdir(parents=True, exist_ok=False)
    start = now()
    source_commit = git(root, 'rev-parse', 'HEAD').decode().strip()
    provenance = {'format': 'spheres-s23-preparation-review-validation/v1', 'qualification': False,
                  'reviewer': 'Codex /root/review_gap_submission', 'started_utc': start,
                  'source_commit': source_commit, 'submission': SUBMISSION,
                  'python': {'executable': sys.executable, 'version': sys.version},
                  'root': str(root), 'checks': [], 'source_files': [
                      {'path': rel, **identity((root / rel).read_bytes()),
                       'git_blob': git(root, 'rev-parse', f'{source_commit}:{rel}').decode().strip()}
                      for rel in (TOOL, TEST)]}
    checks = [
        ('submitted-implementation-negative-control', [sys.executable, '-X', 'utf8', str(Path(__file__).resolve()),
             '--root', str(root), '--original-regressions'], 1),
        ('reviewed-tests', [sys.executable, '-X', 'utf8', '-m', 'unittest', 'discover', '-s', 'tools/avatars',
             '-p', 'test_certified_boundary_matrix.py', '-v'], 0),
        ('matrix-check', [sys.executable, '-X', 'utf8', TOOL, '--check'], 0),
        ('census-check', [sys.executable, '-X', 'utf8', 'tools/avatars/campaign_census.py', '--check'], 0),
        ('research-check', [sys.executable, '-X', 'utf8', 'tools/avatars/campaign_research.py', '--check'], 0)]
    for name, command, expected in checks:
        began = now()
        result = subprocess.run(command, cwd=root, stdout=subprocess.PIPE, stderr=subprocess.STDOUT)
        logfile = name + '.log'
        (out / logfile).write_bytes(result.stdout)
        provenance['checks'].append({'name': name, 'command': command, 'cwd': str(root),
             'started_utc': began, 'finished_utc': now(), 'exit_code': result.returncode,
             'expected_exit_code': expected, 'expected_outcome_observed': result.returncode == expected,
             'log': logfile, **identity(result.stdout)})
        save(out / 'validation.json', provenance)
        if result.returncode != expected:
            raise RuntimeError(f'Unexpected outcome for {name}: {result.returncode}')

    matrix = importlib.import_module('certified_boundary_matrix')
    local = matrix.build(root)
    if args.integration:
        integration = args.integration.resolve()
        base_inputs = matrix.Inputs
        overlay_paths = []

        class CurrentIntegrationInputs(base_inputs):
            def _read(self, rel):
                if rel == TOOL:
                    overlay_paths.append(rel)
                    local_reader = base_inputs(root)
                    raw = local_reader._read(rel)
                    self.files[rel] = local_reader.files[rel]
                    return raw
                return super()._read(rel)

        current_before = git(integration, 'rev-parse', 'HEAD').decode().strip()
        matrix.Inputs = CurrentIntegrationInputs
        try:
            current = matrix.build(integration)
        finally:
            matrix.Inputs = base_inputs
        report = {'source_root': str(integration), 'revision_before': current_before,
                  'revision_after': git(integration, 'rev-parse', 'HEAD').decode().strip(),
                  'tool_overlay_only': sorted(set(overlay_paths)),
                  'method': 'Read-only build from actual current integration inputs; reviewed tool bytes overlaid only for its self-hash. No integration files changed.',
                  'files': [], 'inputs_unchanged_after': True}
        folder = out / 'current-integration'
        folder.mkdir()
        for name, value in current.items():
            raw = (value if isinstance(value, str) else matrix.canonical(value)).encode('utf-8')
            encoded = gzip.compress(raw, mtime=0)
            assert gzip.decompress(encoded) == raw
            (folder / (name + '.gz')).write_bytes(encoded)
            old = (local[name] if isinstance(local[name], str) else matrix.canonical(local[name])).encode('utf-8')
            report['files'].append({'path': name, 'raw': identity(raw), 'gzip': identity(encoded),
                                    'equals_review_checkout': raw == old})
        for row in current['summary.json']['inputs']:
            if row['path'] == TOOL:
                continue
            path = integration / row['path']
            if not row['exists']:
                assert not path.exists(), row['path']
            else:
                raw = path.read_bytes()
                if row['newline_normalized']:
                    raw = raw.replace(b'\r\n', b'\n')
                assert identity(raw) == {k: row[k] for k in ('bytes', 'sha256')}, row['path']
        save(out / 'current-integration-audit.json', report)

    for index, campaign in enumerate(args.campaign, 1):
        before = identity(campaign.read_bytes())
        report = matrix.campaign_report(root, campaign)
        after = identity(campaign.read_bytes())
        assert before == after
        save(out / f'actual-campaign-{index}.json', report)
        provenance.setdefault('actual_campaigns', []).append({'path': str(campaign), **before,
            'unchanged_after': True, 'report': f'actual-campaign-{index}.json',
            'date': report['campaign_date'], 'scope': 'read-only identity observation; no native loading or replay'})
    provenance['finished_utc'] = now()
    provenance['all_expected_outcomes_observed'] = all(c['expected_outcome_observed'] for c in provenance['checks'])
    save(out / 'validation.json', provenance)
    print(json.dumps({'source_commit': source_commit, 'output': str(out),
                      'all_expected_outcomes_observed': provenance['all_expected_outcomes_observed']}))
    return 0


if __name__ == '__main__':
    raise SystemExit(main())
