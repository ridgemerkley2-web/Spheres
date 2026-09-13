"""Read-only verification of published S10.c evidence; optional exact staged Git coverage.

No builds, games, source writes, commits or network operations. This helper is
prepared separately from publication and must be invoked explicitly afterward.
"""
import argparse
import gzip
import hashlib
import json
import pathlib
import subprocess

BASE = pathlib.Path(__file__).resolve().parent
REPO = BASE / 'integration'
ROOT = REPO / 'docs/campaign-certification/S10/c'
EVIDENCE = ROOT / 'evidence'
RUNTIME_PATHS = ('spheres-sim', 'spheres-cli', 'spheres-web', 'Cargo.toml', 'Cargo.lock')


def digest(raw):
    return hashlib.sha256(raw).hexdigest()


def load(path):
    return json.loads(path.read_text(encoding='utf-8-sig'))


def git(*args):
    return subprocess.check_output(['git', '-c', 'core.longpaths=true', *args], cwd=REPO)


def source_key(value):
    return str(value).replace('\\', '/').casefold()


def main(require_index=False):
    manifest = load(ROOT / 'manifest.json')
    inventory_path = EVIDENCE / 'inventory.json'
    inventory = load(inventory_path)
    assert manifest['format'] == 'spheres-s10c-increment/v1'
    assert manifest['status'] == 'increment_complete_parent_open'
    assert manifest['evidence']['inventory_sha256'] == digest(inventory_path.read_bytes())
    assert manifest['evidence']['logical_files'] == len(inventory['files'])
    assert manifest['evidence']['stored_bytes'] == inventory['stored_bytes']
    for key in ['c01_complete', 's10_complete', 'g2_earned', 's11_started', 'saved_role_or_gameplay_authorization_changed']:
        assert manifest[key] is False, 'Incorrect parent gate: ' + key
    assert manifest['qualification']['browser_elapsed_days'] == 0
    assert manifest['qualification']['new_performance_claim'] is False
    assert manifest['qualification']['full_sim_suite_rerun'] is False
    rows, sources, storage = {}, {}, set()
    stored_bytes = 0

    def reconstruct(row):
        pieces = []
        for item in row['storage']:
            name = pathlib.PurePosixPath(item['file'])
            assert not name.is_absolute() and '..' not in name.parts and ':' not in str(name)
            path = EVIDENCE / name
            assert not path.is_symlink() and path.is_file()
            raw = path.read_bytes()
            assert len(raw) == item['bytes'] and digest(raw) == item['sha256'], 'Changed stored evidence: ' + str(name)
            pieces.append(raw)
        encoded = b''.join(pieces)
        assert row['encoding'] in {'raw', 'gzip'}
        raw = gzip.decompress(encoded) if row['encoding'] == 'gzip' else encoded
        assert len(raw) == row['bytes'] and digest(raw) == row['sha256'], 'Reconstruction differs: ' + row['logical_path']
        return raw

    for row in inventory['files']:
        assert row['logical_path'] not in rows
        assert row['reconstruction_verified'] is True
        rows[row['logical_path']] = row
        key = source_key(row['source'])
        assert key not in sources, 'A source was duplicated under another logical path'
        sources[key] = row
        reconstruct(row)
        for item in row['storage']:
            assert item['file'] not in storage
            storage.add(item['file']); stored_bytes += item['bytes']
    assert stored_bytes == inventory['stored_bytes']
    physical = {path.relative_to(EVIDENCE).as_posix() for path in EVIDENCE.rglob('*') if path.is_file()}
    assert physical == storage | {'inventory.json'}, 'Unexpected/missing physical evidence'
    for key, proof in manifest['proofs'].items():
        row = rows[proof['logical_path']]
        assert row['sha256'] == proof['sha256'] and row['bytes'] == proof['bytes']
        assert source_key(row['source']) == source_key(proof['source'])
        record = json.loads(reconstruct(row))
        if key not in {'visual_proof', 'launch_proof'}:
            assert record.get('passed') is True or record.get('status') == 'passed', 'Failed final proof: ' + key
    for shot in manifest['screenshot_selection']:
        logical = shot['case'] + '/' + shot['file'].replace('\\', '/')
        if shot['included']:
            assert logical in rows and rows[logical]['sha256'] == shot['sha256'] and rows[logical]['bytes'] == shot['bytes']
        else:
            assert logical not in rows, 'An omitted screenshot was actually included'
            assert shot['case'].startswith('browser/matrix/')
            assert shot['case'].split('/')[-1] not in {'France', 'Tonga'}

    # Bind each whole-world inspection to the packaged original save and canonical
    # bytes by its ORIGINAL source path, without rewriting the proof itself.
    inspections = comparisons = 0
    for name, row in rows.items():
        if not name.startswith('browser/'):
            continue
        if '/archive-audit/' in name and name.endswith('.json'):
            record = json.loads(reconstruct(row))
            assert record['canonical']['ignored_paths'] == []
            for field in ['input', 'canonical']:
                part = record[field]
                source = sources[source_key(part['path'])]
                assert source['sha256'] == part['sha256'] and source['bytes'] == part['bytes']
            inspections += 1
        elif name.endswith('/progress.jsonl'):
            for line in reconstruct(row).decode('utf-8').splitlines():
                if not line.strip(): continue
                record = json.loads(line)
                if record.get('event') != 'exact-native-comparison': continue
                assert record['equal'] is True
                assert record['left']['ignored_paths'] == record['right']['ignored_paths'] == []
                assert record['left']['sha256'] == record['right']['sha256']
                for field in ['left', 'right']:
                    part = record[field]
                    source = sources[source_key(part['path'])]
                    assert source['sha256'] == part['sha256'] and source['bytes'] == part['bytes']
                comparisons += 1
    assert inspections == sum(row['inspections'] for row in manifest['native_audit_chains'].values())
    assert comparisons == sum(row['exact_comparisons'] for row in manifest['native_audit_chains'].values())

    runtime = manifest['runtime_revision']
    assert not git('diff', '--name-only', runtime, 'HEAD', '--', *RUNTIME_PATHS).strip()
    assert not git('diff', '--name-only', 'HEAD', '--', *RUNTIME_PATHS).strip()
    assert not git('diff', '--name-only', manifest['unchanged_simulation_baseline'], runtime, '--',
                   'spheres-sim', 'spheres-cli', 'Cargo.toml', 'Cargo.lock').strip()
    pathway = load(REPO / 'docs/planning/campaign-pathway.json')
    session = next(row for row in pathway['sessions'] if row['id'] == 'S10')
    assert session['status'] == 'in_progress'
    assert next(row for row in session['increments'] if row['id'] == 'S10.c')['status'] == 'complete'
    assert next(row for row in pathway['sessions'] if row['id'] == 'C01')['status'] == 'in_progress'
    assert next(row for row in pathway['sessions'] if row['id'] == 'S11')['status'] == 'planned'
    assert pathway['last_completed_session'] == 'S09' and pathway['next_session'] == 'S10'
    assert pathway['execution']['status'] == 'stopped' and pathway['execution']['stop_boundary'] == 'S10.c'
    assert pathway['gate_decisions'].get('G2', {}).get('status') != 'earned'
    roadmap = (REPO / 'docs/CERTIFIED_CAMPAIGN_PATHWAY.md').read_text(encoding='utf-8')
    summary = (REPO / 'docs/campaign-certification/S10/README.md').read_text(encoding='utf-8')
    assert '**S10.c complete:**' in roadmap and 'S10, C01 and G2 remain open' in roadmap
    assert 'S10.a, S10.b and S10.c complete; S10 and C01 remain in progress' in summary
    assert 'c/README.md' in summary
    assert digest((REPO / 'docs/campaign-certification/C01/research-index.json').read_bytes()) == manifest['research_index_sha256']
    assert digest((REPO / 'spheres-web/data/leadership_production_2035.json').read_bytes()) == manifest['production_index_sha256']
    allowed = {'docs/planning/campaign-pathway.json', 'docs/CERTIFIED_CAMPAIGN_PATHWAY.md', 'docs/campaign-certification/S10/README.md'}
    # Also covers a completed publication commit after HEAD moves. Earlier S10.a/b
    # archives and historical manifests may not be rewritten by this increment.
    changed = git('diff', '--name-only', manifest['publication_parent_revision']).decode().splitlines()
    if '.gitattributes' in changed:
        # This post-qualification checkout rule preserves the exact submitted
        # prompt bytes. No other attribute or runtime change is accepted.
        before = git('show', manifest['publication_parent_revision'] + ':.gitattributes')
        current = (REPO / '.gitattributes').read_bytes().replace(b'\r\n', b'\n')
        suffix = (b"# Retain the submitted portrait prompt's recorded checksum on Windows checkout.\n"
                  b'tools/avatars/person-prompts/taufaahau-tupou-iv-cartoon-1990-v1.txt text eol=lf\n')
        assert current == before + suffix
        prompt_path = 'tools/avatars/person-prompts/taufaahau-tupou-iv-cartoon-1990-v1.txt'
        provenance = json.loads((REPO / prompt_path.replace('.txt', '.json')).read_text(encoding='utf8'))
        assert digest((REPO / prompt_path).read_bytes()) == provenance['prompt']['sha256']
        assert git('show', 'HEAD:' + prompt_path) == (REPO / prompt_path).read_bytes()
        allowed.add('.gitattributes')
    assert all(name in allowed or name.startswith('docs/campaign-certification/S10/c/') for name in changed)

    index_count = None
    if require_index:
        index = git('ls-files', '--stage', '-z', '--', 'docs/campaign-certification/S10/c/evidence')
        files = {}
        for entry in index.split(b'\0'):
            if not entry: continue
            metadata, name = entry.split(b'\t', 1)
            mode, object_id, stage = metadata.decode().split()
            assert stage == '0' and mode == '100644'
            files[name.decode()] = object_id
        expected = {'docs/campaign-certification/S10/c/evidence/' + name for name in storage | {'inventory.json'}}
        assert set(files) == expected, 'Exact evidence set is not present in the Git index'
        algorithm = git('rev-parse', '--show-object-format').decode().strip()
        for name, object_id in files.items():
            raw = (REPO / name).read_bytes()
            actual = hashlib.new(algorithm, b'blob ' + str(len(raw)).encode() + b'\0' + raw).hexdigest()
            assert actual == object_id, 'Git changed evidence bytes: ' + name
        index_count = len(files)
    print(json.dumps({'passed': True, 'logical_files': len(rows), 'stored_bytes': stored_bytes,
                      'native_inspections': inspections, 'exact_comparisons': comparisons,
                      'index_required': require_index, 'git_blobs': index_count,
                      'runtime_revision': runtime, 'inventory_sha256': digest(inventory_path.read_bytes()),
                      'c01_complete': False, 's10_complete': False, 'g2_earned': False}, indent=2))


if __name__ == '__main__':
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('--require-index', action='store_true', help='Require all evidence staged/committed with unchanged Git blob bytes')
    main(parser.parse_args().require_index)
