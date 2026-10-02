"""Prepare only the explicitly reviewed India/Saudi fragments; never edit the manifest."""
from pathlib import Path
import copy
import hashlib
import json
import subprocess
import sys
from PIL import Image

ROOT = next(p for p in Path(__file__).resolve().parents if (p / 'spheres-web/data/person_portraits.json').is_file())
DATE = '2026-10-02'
OPERATOR = 'Codex agent /root/tonga_royal_identities'


def read(path):
    return json.loads((ROOT / path).read_text(encoding='utf-8'))


def sha(path):
    return hashlib.sha256((ROOT / path).read_bytes()).hexdigest()


def pin(path):
    return {'path': str(path).replace('\\', '/'), 'sha256': sha(path)}


def write(path, value):
    path = ROOT / path
    path.parent.mkdir(parents=True, exist_ok=True)
    path.write_text(json.dumps(value, ensure_ascii=False, indent=2) + '\n', encoding='utf-8', newline='\n')


def image_record(path):
    path = Path(path)
    with Image.open(ROOT / path) as image:
        result = {'asset': path.as_posix(), 'sha256': sha(path), 'bytes': (ROOT / path).stat().st_size,
                  'width': image.width, 'height': image.height, 'mode': image.mode}
        assert image.format in ('JPEG', 'PNG')
        image.verify()
    return result


def main():
    manifest_path = 'spheres-web/data/person_portraits.json'
    registry_path = 'spheres-sim/data/party_leaders.json'
    manifest = read(manifest_path)
    manifest_pin = pin(manifest_path)
    results = []
    for country in ('india', 'saudi-arabia'):
        base = f'docs/campaign-certification/C06/production/{country}'
        dest = base + '/registration-completion-20261002'
        identity_path = base + '/identity-review-batch-01.json'
        return_path = base + '/render-return-batch-01-20261002.json'
        review_path = 'docs/campaign-certification/C06/render-return-20261002/' + ('india-review.json' if country == 'india' else 'za-sa-ru-review.json')
        generation_path = f'tools/avatars/person-prompts/{country}-cast-registration-20261002.json'
        identity, returned, reviewed = read(identity_path), read(return_path), read(review_path)
        jobs = identity['batches'][0].get('portraits', identity['batches'][0]['people'])
        jobs = [j for j in jobs if j.get('prompt', {}).get('sha256')]
        references = {r['id']: r for r in identity['likeness_references']}
        if country == 'india':
            returned_jobs = {e['prompt_path']: e for e in returned['attempts']}
            review_jobs = {e['person_id']: e for e in reviewed['jobs']}
            request_tip = returned['reviewed_request_commit']
            independent_reviewer = reviewed['reviewer']
            assert reviewed['counts']['hold'] == 0
        else:
            returned_jobs = {e['prompt_file']: e for e in returned['entries']}
            review_jobs = {e['job_id']: e for c in reviewed['countries'] if c['country'] == country for e in c['entries']}
            request_tip = returned['reviewed_source_tip']
            independent_reviewer = 'Codex agent /root/tonga_royal_identities (independent of Saudi renderer /root/selector_leader_api)'
        assert len(jobs) == len(returned_jobs) == (8 if country == 'india' else 4)
        fragment = {'version': 1, 'people': {}}
        generation = {
            'version': 1, 'batch': identity['batches'][0]['id'], 'nation': identity['nation'], 'task': identity['task'],
            'reviewed_at': DATE, 'registration_operator': OPERATOR,
            'status': 'reviewed_fragment_ready_for_root_merge',
            'review_owner': independent_reviewer,
            'render_operator': OPERATOR if country == 'india' else 'Codex agent /root/selector_leader_api',
            'human_approval_claimed': False, 'claude_approval_claimed': False,
            'registration_applied_by_this_record': False,
            'source_request_commit': request_tip,
            'review_checkout_commit': subprocess.check_output(['git', 'rev-parse', 'HEAD'], cwd=ROOT, text=True).strip(),
            'authorization_scope': 'User requested filling the current render-batch gaps and completing registration. These twelve existing requested windows only; no new historical boundaries or country completion.',
            'render_return': pin(return_path), 'independent_render_review': pin(review_path),
            'manifest_before_sha256': manifest_pin['sha256'], 'registry_sha256': sha(registry_path),
            'source_review_sha256': {identity_path: sha(identity_path)},
            'source_review_basis': 'Immutable pre-registration identity preparation, preserved unchanged. This new record records the actual Codex review and requested-window ruling; old render-return flags remain historical.',
            'source_reference_records': {}, 'generations': [],
            'limits': ['Appearance bounds are exclusive art-selection windows, not new office terms, death dates or historical acceptance.',
                       'Age, color, pose, clothing and full-body extrapolations remain exactly as disclosed in the source preparation and prompt.',
                       'All original reference and generated bytes are retained. No new generation, image transformation or human/Claude approval is claimed.',
                       'Only the active current batch is proposed; excluded, superseded, unprepared reserve and later country work remains open.']
        }
        for job in jobs:
            pid, prompt_path = job['person_id'], job['prompt']['path']
            ret = returned_jobs[prompt_path]
            window = job['requested_window']
            ref_id = job['likeness_reference_ids'][0]
            ref = references[ref_id]
            assert ref['person_id'] == pid
            assert sha(prompt_path) == job['prompt']['sha256']
            prompt_bytes = (ROOT / prompt_path).read_bytes()
            assert b'\r' not in prompt_bytes
            assert hashlib.sha256(subprocess.check_output(['git', 'show', request_tip + ':' + prompt_path], cwd=ROOT)).hexdigest() == sha(prompt_path)
            stem = Path(job['planned_output']).stem
            if country == 'india':
                assert ret['prompt_sha256_submitted'] == sha(prompt_path)
                inputs = ret['inputs']
                original = ret['output']['original_tool_output_path']
                assert ret['appearance_window'] == window
                vr = review_jobs[pid]['visual_review']
                visual = vr['likeness_and_composition']
                visual_limit = vr['retained_limit']
                deviations = ['M. A. Baby has a slightly parted smile rather than the strictly closed mouth requested; the independent review finds this non-blocking.'] if pid == 'm_a_baby' else []
            else:
                assert ret['prompt_sha256'] == sha(prompt_path)
                inputs = ret['referenced_images']
                original = ret['generated_original_path']
                assert ret['requested_appearance_window'] == window
                vr = review_jobs[stem]['visual_review']
                assert vr['disposition'] == 'no_blocking_discrepancy_in_bounded_review'
                visual, visual_limit = vr['observations'], ' '.join(vr.get('limitations', []))
                deviations = vr.get('deviations', [])
            assert len(inputs) == 2 and [i['order'] for i in inputs] == [1, 2]
            assert inputs[0]['path'] == 'spheres-web/ui/person-portraits/margaret-thatcher-cartoon-1990-v3.png'
            assert inputs[1]['path'] == ref['asset']
            submitted_inputs = []
            for n, inp in enumerate(inputs):
                assert sha(inp['path']) == inp['sha256']
                ir = image_record(inp['path'])
                ir.update(order=n+1, role='style_only' if n == 0 else 'identity_reference')
                if n:
                    ir.update(person_id=pid, reference_id=ref_id)
                submitted_inputs.append(ir)
            assert sha(ref['asset']) == ref['sha256'] == sha(ref['image_original']['path'])
            assert sha(ref['metadata_original']['path']) == ref['metadata_original']['sha256']
            out = image_record(job['planned_output'])
            assert out['sha256'] == ret['output']['sha256'] == sha(original)
            assert out['width'] == 1024 and out['height'] == 1536 and out['mode'] == 'RGB'
            with Image.open(ROOT / job['planned_output']) as im:
                assert 'transparency' not in im.info
            source = {k: ref[k] for k in ('person_id', 'kind', 'asset', 'sha256', 'bytes', 'width', 'height', 'mode', 'license', 'license_url', 'creator', 'credit', 'rights_statement', 'source_license_statement')}
            source.update(reference_id=ref_id, source_url=ref['source_page_url'], depiction_date=ref.get('depiction_date', ref['photograph_date']),
                          photograph_date=ref['photograph_date'], date_precision=ref.get('date_precision'), source_review=identity_path,
                          source_metadata_sha256=ref['metadata_original']['sha256'])
            for key in ('licensor', 'third_party_material', 'original_source', 'container_format', 'reference_frame'):
                if key in ref:
                    source[key] = copy.deepcopy(ref[key])
            source_license = ref['license']
            derivative = 'CC BY 4.0' if source_license.lower() in ('public domain', 'cc0', 'cc0 1.0') else source_license
            derivative_url = 'https://creativecommons.org/licenses/by/4.0/' if derivative == 'CC BY 4.0' else ref['license_url']
            era = (f"Observed reference date: {ref['photograph_date']}. Approved illustrative appearance interval {window['from']} to {window['to']} (exclusive), exactly as requested. "
                   + ref.get('age_gap_note', '') + ' ' + ref.get('limitation', '')
                   + ' Art availability only: no new office boundary, tenure or historical acceptance. ' + visual_limit)
            reviewer = (independent_reviewer + '; Codex /root/tonga_royal_identities registration review of source, likeness, era and visual quality on 2026-10-02; no human or Claude approval claimed')
            record = {
                'from': window['from'], 'to': window['to'], 'method': 'generated', 'style': 'cartoon', 'status': 'illustrated-likeness',
                **out, 'composition': 'full-body', 'background_mode': 'opaque', 'generator': 'OpenAI built-in image_gen',
                'generated_at': DATE, 'license': 'generated', 'source_url': ref['source_page_url'],
                'credit': 'Spheres cartoon generated with OpenAI image_gen; artistic adaptation and full-body/age interpretation. Reference: ' + ref['credit'],
                'era_note': era, 'identity_source': source, 'supplemental_identity_sources': [],
                'prompt_record': prompt_path,
                'generation_record': f'Exact submitted prompt, ordered input hashes, source rights, retained original and Codex review in {generation_path}, job {stem}; original return {return_path} remains unchanged.',
                'review': {'identity': True, 'likeness': True, 'era': True, 'visual': True, 'reviewer': reviewer, 'reviewed_at': DATE},
                'visual_note': visual + (' Non-blocking deviations: ' + ' '.join(deviations) if deviations else '') + ' No human or Claude approval claimed.',
                'derivative_license': derivative, 'derivative_license_url': derivative_url
            }
            if pid not in fragment['people']:
                old = manifest['people'][pid]
                fragment['people'][pid] = {'name': old['name'], 'identity_sources': copy.deepcopy(old['identity_sources']), 'portraits': []}
            if ref['source_page_url'] not in fragment['people'][pid]['identity_sources']:
                fragment['people'][pid]['identity_sources'].append(ref['source_page_url'])
            fragment['people'][pid]['portraits'].append(record)
            generation['source_reference_records'][ref_id] = {'record': source, 'original': ref['image_original'],
                'retained_metadata': ref['metadata_original'], 'retained_file_page': ref.get('metadata_file_page')}
            generation['generations'].append({
                'job': {'stem': stem, 'person_id': pid, 'from': window['from'], 'to': window['to'], 'requested_window': window,
                        'references': [ref_id], 'era_note': era, 'visual_review': record['visual_note'], **out,
                        'original_output': str(Path(original)).replace('\\', '/'), 'byte_identical_to_original': True},
                'exact_prompt': prompt_bytes.decode('utf-8'), 'prompt_record': prompt_path, 'prompt_file_sha256': sha(prompt_path),
                'generator': 'OpenAI built-in image_gen', 'generated_at': DATE, 'submitted_reference_inputs': submitted_inputs,
                'attempt': 1, 'failures': [], 'source_rights': {'source_page_url': ref['source_page_url'], 'original_reference': ref['asset'],
                    'original_reference_sha256': ref['sha256'], 'creator': ref['creator'], 'credit': ref['credit'],
                    'source_license': ref['license'], 'license_url': ref['license_url'], 'rights_statement': ref['rights_statement'],
                    'derivative_license': derivative, 'derivative_license_url': derivative_url,
                    'adaptation': 'AI-generated full-body cartoon; age/color/pose extensions explicitly disclosed in prompt and era note; no endorsement implied.'},
                'review': record['review'], 'visual_deviations': deviations,
                'appearance_window_decision': 'Accepted by Codex for this exact illustrative art interval only; no alteration of historical records or requested bounds.'
            })
        fragment_path = dest + '/manifest-fragment.json'
        write(generation_path, generation)
        write(fragment_path, fragment)
        receipt = {'version': 1, 'date': DATE, 'country': identity['nation'], 'operator': OPERATOR,
                   'status': 'reviewed_fragment_pending_root_merge', 'authority': {'human_approval': False, 'claude_approval': False, 'runtime_registration_applied': False, 'country_complete': False},
                   'base_manifest': manifest_pin, 'registry': pin(registry_path), 'source_review': pin(identity_path),
                   'old_render_return': pin(return_path), 'independent_render_review': pin(review_path),
                   'generation_record': pin(generation_path), 'manifest_fragment': pin(fragment_path),
                   'counts': {'people': len(fragment['people']), 'portraits_to_append': len(generation['generations'])},
                   'merge_rule': 'Match exact person IDs and saved names, union identity_sources, append these portraits without deleting or replacing any existing portrait. Validate the full merged manifest before writing it.',
                   'historical_and_appearance_limits': generation['limits']}
        write(dest + '/receipt.json', receipt)
        results.append(receipt)
    assert sha(manifest_path) == manifest_pin['sha256'], 'shared manifest changed during preparation; no write was made here'
    print(json.dumps([{'country': r['country'], 'counts': r['counts'], 'fragment': r['manifest_fragment'], 'generation': r['generation_record']} for r in results], ensure_ascii=False))


if __name__ == '__main__':
    main()
