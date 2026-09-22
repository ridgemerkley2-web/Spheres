"""Receipt the parent's completed checkpoint26 workspace build; never builds/runs binaries."""
import argparse, importlib.util, json, pathlib, re

HERE = pathlib.Path(__file__).resolve().parent
BASE = HERE.parent.parent
REPO = BASE / 'integration'
REVISION = '9f7faed5f541e14545e01caf1616ba08c6ad8ff9'
MANIFEST = HERE / 'source-manifest-9f7faed5.json'
MANIFEST_SHA = 'de8bb46eb286b4d6dc7ed9874618416ae3ae1b3c56cf210b85cf5be6a1eec0ad'
LOG = BASE / 'evidence/completion-checkpoint26-workspace-20260922.log'
RUNNER = BASE / 'evidence/political-calibration-20260922/run-final-archive-correctness.py'

def helpers():
    spec = importlib.util.spec_from_file_location('archive_helpers', RUNNER)
    mod = importlib.util.module_from_spec(spec)
    spec.loader.exec_module(mod)
    return mod

def main():
    ap = argparse.ArgumentParser(description=__doc__)
    ap.add_argument('--population-binary', required=True)
    ap.add_argument('--web-test-binary', required=True)
    ap.add_argument('--cargo-exit-code', type=int, required=True,
                    help='Actual parent-observed completed Cargo exit code, not a prediction')
    args = ap.parse_args()
    h = helpers()
    h.require(args.cargo_exit_code == 0, 'Parent workspace Cargo did not succeed')
    h.require(h.sha(MANIFEST) == MANIFEST_SHA, 'Original clean source snapshot changed')
    manifest = h.read_json(MANIFEST)
    source = h.source_check(REPO, REVISION, manifest)
    log = LOG.read_text(encoding='utf-8', errors='replace')
    summaries = re.findall(r'test result: (ok|FAILED)\. (\d+) passed; (\d+) failed;', log)
    h.require(summaries and all(s[0] == 'ok' and s[2] == '0' for s in summaries),
              'Build log does not contain only successful test summaries')
    h.require('error: test failed' not in log, 'Cargo log records a test failure')
    binaries = {}
    for key, value in [('s02_population', args.population_binary), ('spheres_web', args.web_test_binary)]:
        path = h.absolute_file(value, key)
        h.require(path.name.startswith(key + '-') and path.name in log,
                  'Explicit test binary is not identified by the completed Cargo log: ' + str(path))
        binaries[key] = {'path': str(path), 'sha256': h.sha(path)}
    receipt = {
        'format': 'spheres-final-archive-build/v1',
        'scope': 'Checkpoint26 regression validation only; A1 remains pending, no campaign certification claim.',
        'revision': REVISION, 'repository': str(REPO), 'clean_source': True,
        'created_utc': h.now(),
        'cargo': {'command': ['cargo', 'test', '--locked', '--release', '--workspace', '--no-fail-fast',
                              '--', '--skip', 'tests::the_resource_pass_stays_under_budget'],
                  'exit_code': args.cargo_exit_code,
                  'log': {'path': str(LOG), 'sha256': h.sha(LOG)}},
        'source_manifest': {'path': str(MANIFEST), 'sha256': MANIFEST_SHA},
        'source_inputs_verified': source['checked_files'], 'binaries': binaries,
    }
    path = HERE / 'build-receipt-9f7faed5.json'
    with path.open('x', encoding='utf-8', newline='\n') as stream:
        json.dump(receipt, stream, indent=2)
        stream.write('\n')
    print(json.dumps({'receipt': str(path), 'sha256': h.sha(path), 'binaries': binaries}))

if __name__ == '__main__':
    main()
