"""Validate the two reviewed fragments in a disposable full-manifest merge."""
from pathlib import Path
from datetime import date, timedelta
import copy
import hashlib
import json
import subprocess
import sys
import tempfile

ROOT = next(p for p in Path(__file__).resolve().parents if (p / 'spheres-web/data/person_portraits.json').is_file())
sys.path.insert(0, str(ROOT / 'tools/avatars'))
import person_art_pipeline as pipeline


def sha(path):
    return hashlib.sha256((ROOT / path).read_bytes()).hexdigest()


def read(path):
    return json.loads((ROOT / path).read_text(encoding='utf-8'))


def main():
    manifest_path = 'spheres-web/data/person_portraits.json'
    registry_path = 'spheres-sim/data/party_leaders.json'
    base = read(manifest_path)
    before_hash = sha(manifest_path)
    known = pipeline.people_from_registry(read(registry_path))
    merged = copy.deepcopy(base)
    fragments = {}
    countries = ('india', 'saudi-arabia')
    pins = {}
    for country in countries:
        directory = f'docs/campaign-certification/C06/production/{country}/registration-completion-20261002'
        path = directory + '/manifest-fragment.json'
        fragment = read(path)
        fragments[country] = fragment
        pins[country] = {'fragment': {'path': path, 'sha256': sha(path)},
                         'generation': {'path': f'tools/avatars/person-prompts/{country}-cast-registration-20261002.json',
                                        'sha256': sha(f'tools/avatars/person-prompts/{country}-cast-registration-20261002.json')}}
        for pid, p in fragment['people'].items():
            assert merged['people'][pid]['name'] == p['name']
            assert not any(x['asset'] == old['asset'] for x in p['portraits'] for old in merged['people'][pid]['portraits']), 'already registered'
            for source in p['identity_sources']:
                if source not in merged['people'][pid]['identity_sources']:
                    merged['people'][pid]['identity_sources'].append(source)
            merged['people'][pid]['portraits'].extend(copy.deepcopy(p['portraits']))
    touched = {pid for f in fragments.values() for pid in f['people']}
    for pid, p in base['people'].items():
        assert merged['people'][pid]['portraits'][:len(p['portraits'])] == p['portraits']
        if pid not in touched:
            assert merged['people'][pid] == p
    with tempfile.TemporaryDirectory(prefix='spheres-india-saudi-registration-') as temp:
        temp_path = Path(temp) / 'manifest.json'
        merged_bytes = (json.dumps(merged, ensure_ascii=False, indent=2) + '\n').encode('utf-8')
        temp_path.write_bytes(merged_bytes)
        command = [sys.executable, '-X', 'utf8', str(ROOT/'tools/avatars/person_art_pipeline.py'), 'validate',
                   '--repo', str(ROOT), '--manifest', str(temp_path), '--registry', str(ROOT/registry_path)]
        result = subprocess.run(command, cwd=ROOT, capture_output=True, text=True, encoding='utf-8')
        assert result.returncode == 0, result.stderr + result.stdout
        checked = json.loads(result.stdout)
        assert checked['valid'] and not checked['errors']
        stdout_hash = hashlib.sha256(result.stdout.encode('utf-8')).hexdigest()
    boundaries = []
    for country, fragment in fragments.items():
        for pid, p in fragment['people'].items():
            small = {'version': 1, 'people': {pid: merged['people'][pid]}}
            for art in p['portraits']:
                start, end = date.fromisoformat(art['from']), date.fromisoformat(art['to'])
                for day, expected_this_art in ((start-timedelta(days=1), False), (start, True), (end-timedelta(days=1), True), (end, False)):
                    selected = pipeline.select_portrait(small, pid, day.isoformat(), ROOT, known)
                    assert bool(selected and selected['asset'] == art['asset']) == expected_this_art
                boundaries.append({'person_id': pid, 'asset': art['asset'], 'from': art['from'], 'exclusive_to': art['to'],
                                   'before_start_start_last_day_exclusive_end_pass': True})
    ogl = fragments['india']['people']['nitin_gadkari']['portraits'][0]['identity_source']
    assert pipeline.approved_source_license(ogl)
    assert not pipeline.approved_source_license(dict(ogl, third_party_material=True))
    assert not pipeline.approved_source_license(dict(ogl, licensor=''))
    assert not pipeline.approved_source_license(dict(ogl, license_url='https://example.test/licence'))
    for p in fragments['india']['people'].values():
        for art in p['portraits']:
            source = art['identity_source']['license']
            if 'BY-SA' in source or source == pipeline.OGL_V1_LICENSE:
                assert art['derivative_license'] == source
                assert art['derivative_license_url'] == art['identity_source']['license_url']
    assert sha(manifest_path) == before_hash, 'shared manifest changed while validating'
    report = {'version': 1, 'date': '2026-10-02', 'reviewer': 'Codex /root/tonga_royal_identities',
              'status': 'pass_temporary_merged_manifest_validation', 'shared_manifest_edited': False,
              'shared_manifest_sha256_before_and_after': before_hash, 'registry_sha256': sha(registry_path),
              'pipeline_sha256': sha('tools/avatars/person_art_pipeline.py'),
              'command': [part.replace(str(temp_path), '<disposable>/manifest.json') for part in command],
              'temporary_merged_manifest_sha256': hashlib.sha256(merged_bytes).hexdigest(),
              'pipeline_stdout_sha256': stdout_hash, 'pipeline_returncode': result.returncode,
              'pipeline_valid': checked['valid'], 'errors': checked['errors'],
              'existing_missing_portrait_warning_count': len(checked['warnings']),
              'warning_scope': 'Existing people without a portrait remain honest warnings; no batch result waives their gaps.',
              'all_preexisting_portraits_preserved': True, 'all_untouched_people_preserved': True,
              'new_portraits': 12, 'new_people': 0, 'total_proposed_portraits': sum(len(p['portraits']) for p in merged['people'].values()),
              'boundary_checks': boundaries, 'boundary_assertions_passed': len(boundaries)*4,
              'ogl_negative_controls': {'third_party': 'rejected', 'empty_licensor': 'rejected', 'incorrect_policy_url': 'rejected'},
              'exact_sharealike_and_ogl_licence_versions_preserved': True, 'artifact_pins': pins,
              'authority': {'human_approval': False, 'claude_approval': False, 'runtime_registration': False, 'country_complete': False}}
    for country in countries:
        destination = ROOT/f'docs/campaign-certification/C06/production/{country}/registration-completion-20261002/validation.json'
        destination.write_text(json.dumps(dict(report, country_fragment=country), ensure_ascii=False, indent=2)+'\n', encoding='utf-8', newline='\n')
    print(json.dumps({k: report[k] for k in ('status', 'pipeline_returncode', 'errors', 'new_portraits', 'total_proposed_portraits', 'boundary_assertions_passed', 'ogl_negative_controls')}))


if __name__ == '__main__':
    main()
