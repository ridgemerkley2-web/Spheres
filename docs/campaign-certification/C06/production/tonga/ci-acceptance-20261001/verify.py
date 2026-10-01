"""Verify retained CI evidence and source pins; never run tests or use network."""
import argparse
import hashlib
import json
import subprocess
from pathlib import Path

HERE = Path(__file__).resolve().parent
DEFAULT_EXTERNAL = Path('D:/spheres-offload/codex-next-20260928/claude-review-20261001-05/tonga-ci-evidence')

def read(path): return json.loads(path.read_text(encoding='utf-8'))
def digest(data): return hashlib.sha256(data).hexdigest()

def main():
    p = argparse.ArgumentParser(description=__doc__)
    p.add_argument('--repo', type=Path)
    p.add_argument('--external-root', type=Path, default=DEFAULT_EXTERNAL)
    a = p.parse_args()
    root = a.repo or Path(subprocess.check_output(['git','rev-parse','--show-toplevel'],cwd=HERE,text=True).strip())
    for row in read(HERE/'integrity.json')['files']:
        data = (HERE/row['path']).read_bytes()
        assert len(data) == row['bytes'] and digest(data) == row['sha256'], row['path']
    receipt = read(HERE/'receipt.json')
    assert set(receipt['accepted_checks']) == {'boundary_dates','save_compatibility','institutional_rules'}
    assert set(receipt['not_accepted']) == {'production_browser','likeness_visual_review','country_signoff'}
    for path, pin in receipt['source_pins'].items():
        blob = subprocess.check_output(['git','show',receipt['candidate']+':'+path],cwd=root)
        assert digest(blob) == pin and digest((root/path).read_bytes()) == pin, path
    external = read(HERE/'external-evidence.json')['files']
    for row in external:
        path = a.external_root/Path(row['path']).relative_to(DEFAULT_EXTERNAL)
        data = path.read_bytes()
        assert len(data) == row['bytes'] and digest(data) == row['sha256'], str(path)
    native = read(a.external_root/'followup-01/native-linux.tool.json')['structuredContent']['content']
    assert receipt['candidate'] in native
    assert 'cargo test --locked --release --workspace --no-fail-fast -- --skip tests::the_resource_pass_stays_under_budget' in native
    for check in receipt['accepted_checks'].values():
        for name in check['test_names']:
            assert 'test '+name+' ... ok' in native, name
    job = next(j for j in read(HERE/'jobs-snapshot.json')['jobs'] if j['id'] == receipt['native_job_id'])
    assert job['status'] == 'completed' and job['conclusion'] == 'success'
    print(json.dumps({'receipt_files':len(read(HERE/'integrity.json')['files']),'source_pins':len(receipt['source_pins']),'external_files':len(external),'accepted_checks':list(receipt['accepted_checks']),'verified':True},indent=2))

if __name__ == '__main__':
    main()
