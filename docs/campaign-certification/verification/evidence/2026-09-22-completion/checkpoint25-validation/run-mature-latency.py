"""Run the two existing exact latency tests only after explicit parent release."""
import argparse, importlib.util, json, pathlib, re, sys, uuid, datetime

HERE = pathlib.Path(__file__).resolve().parent
BASE = HERE.parent.parent
RUNNER = BASE / 'evidence/political-calibration-20260922/run-final-archive-correctness.py'
RUNNER_SHA = 'd112b48668544a9ee66b033a1658df5febc4089049369335ff528ded220e9794'
CASES = [
    ('decision-review', 'decision_review::tests::mature_review_latency',
     BASE / 'fixtures/lifetime-legacy/saves/profile-idle_human-10.json',
     '5d6b022e8273c5bbd569d96cc9ac4de1f4086fa69de110f835c63718d7469b53', 'S10_REVIEW_CONTEXT'),
    ('open-route-review', 'diplomatic_sanctions::tests::s10g_mature_open_route_preview_latency',
     BASE / 'evidence/S08-genuine-supplier-route-pool-1/export/2127-purchased.campaign.json',
     '73a8ed5bd81502bf37d77f374c23f18ed96644a0032ba295317b25a0d01133f9', 'S10G_SANCTIONS_PERF'),
]

def helpers():
    spec = importlib.util.spec_from_file_location('archive_helpers', RUNNER)
    mod = importlib.util.module_from_spec(spec)
    spec.loader.exec_module(mod)
    return mod

def main():
    ap = argparse.ArgumentParser(description=__doc__)
    ap.add_argument('--receipt', type=pathlib.Path, required=True)
    ap.add_argument('--released', action='store_true',
                    help='Parent explicitly released native execution and confirmed heavy work is idle')
    args = ap.parse_args()
    h = helpers()
    h.require(args.released, 'Native execution has not been explicitly released')
    h.require(h.sha(RUNNER) == RUNNER_SHA, 'Shared exact-test runner changed')
    receipt_path = args.receipt.resolve(strict=True)
    receipt = h.read_json(receipt_path)
    h.require(receipt.get('format') == 'spheres-final-archive-build/v1', 'Wrong build receipt')
    h.require(receipt.get('clean_source') is True and receipt['cargo']['exit_code'] == 0,
              'Successful clean build receipt is required')
    repo = pathlib.Path(receipt['repository']).resolve(strict=True)
    revision = receipt['revision']
    h.require(re.fullmatch('[a-f0-9]{40}', revision), 'Exact commit is required')
    manifest_path = h.verified_file(receipt['source_manifest'], 'Source manifest')
    manifest = h.read_json(manifest_path)
    binary = h.verified_file(receipt['binaries']['spheres_web'], 'Native web test binary')
    log = h.verified_file(receipt['cargo']['log'], 'Cargo log')
    h.require(binary.name in log.read_text(encoding='utf-8', errors='replace'), 'Unidentified test binary')
    source_before = h.source_check(repo, revision, manifest)
    out = HERE / ('latency-' + datetime.datetime.now(datetime.timezone.utc).strftime('%Y%m%dT%H%M%SZ') + '-' + uuid.uuid4().hex[:8])
    out.mkdir()
    report = {'format': 'spheres-checkpoint-mature-latency/v1', 'revision': revision,
              'scope': 'Existing native preview bars only; no HTTP/browser or campaign certificate.',
              'started_utc': h.now(), 'runner_sha256': h.sha(__file__),
              'receipt': {'path': str(receipt_path), 'sha256': h.sha(receipt_path)},
              'binary': {'path': str(binary), 'sha256': h.sha(binary)},
              'source_before': source_before, 'checks': [], 'passed': False}
    fixture_before = {}
    try:
        for _, _, path, expected, _ in CASES:
            actual = h.sha(path)
            fixture_before[str(path)] = actual
            h.require(actual == expected, 'Original mature archive changed: ' + str(path))
        report['input_sha256_before'] = fixture_before
        for name, test, path, _, marker in CASES:
            row = h.run_bounded([str(binary), test] + h.FLAGS, repo,
                {'SPHERES_S10_PERF_SAVE': str(path)}, out / (name + '.log'), 600)
            row['name'] = name
            body = pathlib.Path(row['log']).read_text(encoding='utf-8', errors='replace')
            rows = [json.loads(value) for value in re.findall(re.escape(marker) + r' (\{[^\r\n]+\})', body)]
            row['measurements'] = rows
            row['passed'] = row['passed'] and len(rows) == 1
            if rows:
                metrics = rows[0]['rows'] if name == 'decision-review' else rows
                row['passed'] = row['passed'] and all(m['p95_limit_ms'] == 300 and m['max_limit_ms'] == 750
                    and m['p95_ms'] <= 300 and m['max_ms'] <= 750 for m in metrics)
                if name == 'open-route-review':
                    row['passed'] = row['passed'] and rows[0]['passed'] is True
            report['checks'].append(row)
            print(json.dumps(row), flush=True)
    except Exception as exc:
        report['error'] = str(exc)
    finally:
        report['input_sha256_after'] = {str(path): h.sha(path) for _, _, path, _, _ in CASES}
        report['inputs_unchanged'] = len(fixture_before) == 2 and report['input_sha256_after'] == fixture_before
        report['binary_unchanged'] = h.sha(binary) == report['binary']['sha256']
        report['receipt_unchanged'] = h.sha(receipt_path) == report['receipt']['sha256']
        report['manifest_unchanged'] = h.sha(manifest_path) == receipt['source_manifest']['sha256']
        report['cargo_log_unchanged'] = h.sha(log) == receipt['cargo']['log']['sha256']
        try:
            report['source_after'] = h.source_check(repo, revision, manifest)
            report['source_unchanged'] = report['source_after'] == source_before
        except Exception as exc:
            report['source_after_error'] = str(exc)
            report['source_unchanged'] = False
        report['finished_utc'] = h.now()
        report['passed'] = not report.get('error') and len(report['checks']) == 2 and all(r['passed'] for r in report['checks']) \
            and all(report[x] for x in ['inputs_unchanged','binary_unchanged','receipt_unchanged','manifest_unchanged','cargo_log_unchanged','source_unchanged'])
        path = out / 'result.json'
        with path.open('x', encoding='utf-8', newline='\n') as stream:
            json.dump(report, stream, indent=2)
            stream.write('\n')
        print(json.dumps({'passed': report['passed'], 'result': str(path), 'error': report.get('error')}), flush=True)
    return int(not report['passed'])

if __name__ == '__main__':
    sys.exit(main())
