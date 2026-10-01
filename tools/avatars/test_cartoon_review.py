"""Checks for the read-only cartoon review export (CLAUDE-C03-REVIEW-01).

Synthetic fixtures in temporary directories exercise missing files, invalid
intervals, duplicate bindings, unknown identities, rights gaps and the visual
review guard; repository checks confirm deterministic, current, read-only output.
Run: python -X utf8 -m unittest discover -s tools/avatars -p "test_cartoon_review.py"
"""
import contextlib
import copy
import hashlib
import io
import json
import shutil
import struct
import tempfile
import unittest
import zlib
from pathlib import Path

import cartoon_review as cr

PP = 'spheres-web/ui/person-portraits'


def png(width: int, height: int, seed: int) -> bytes:
    """A small valid RGB PNG whose pixels (and therefore hash) depend on seed."""
    def chunk(kind: bytes, payload: bytes) -> bytes:
        return struct.pack('>I', len(payload)) + kind + payload + struct.pack('>I', zlib.crc32(kind + payload) & 0xFFFFFFFF)
    rows = b''.join(b'\x00' + bytes((seed * 31 + x * 7 + y * 13) % 256 for x in range(width * 3)) for y in range(height))
    return (b'\x89PNG\r\n\x1a\n' + chunk(b'IHDR', struct.pack('>IIBBBBB', width, height, 8, 2, 0, 0, 0))
            + chunk(b'IDAT', zlib.compress(rows)) + chunk(b'IEND', b''))


def webp(width: int, height: int, seed: int) -> bytes:
    """A VP8L header (enough for header parsing; not a decodable picture)."""
    bits = (width - 1) | ((height - 1) << 14)
    payload = b'\x2f' + bits.to_bytes(4, 'little') + bytes([seed % 256]) * 11
    return b'RIFF' + struct.pack('<I', 4 + 8 + len(payload)) + b'WEBP' + b'VP8L' + struct.pack('<I', len(payload)) + payload


def jpeg(width: int, height: int, seed: int) -> bytes:
    app0 = b'\xff\xe0' + struct.pack('>H', 16) + b'JFIF\x00\x01\x01\x00\x00\x01\x00\x01\x00' + bytes([seed % 256])
    sof = b'\xff\xc0' + struct.pack('>HBHHB', 17, 8, height, width, 3) + b'\x01\x22\x00\x02\x11\x01\x03\x11\x01'
    return b'\xff\xd8' + app0 + sof + b'\xff\xd9'


def sha(data: bytes) -> str:
    return hashlib.sha256(data).hexdigest()


class Fixture:
    """A miniature repository with every input the export reads."""

    def __init__(self):
        self.files: dict[str, bytes] = {}
        alice, bob, fic = png(4, 6, 1), png(4, 6, 2), png(4, 6, 3)
        ref = jpeg(8, 10, 4)
        photo, leader = webp(6, 8, 5), png(4, 6, 6)
        photo_display, leader_display = webp(3, 4, 7), webp(2, 3, 8)
        self.photo_name = f'Testland-{sha(photo)[:12]}.webp'
        self.leader_name = f'Testland-leader-{sha(leader)[:12]}.png'
        self.photo_display = f'spheres-web/ui/display-art/Testland-{sha(photo)[:12]}-{sha(photo_display)[:16]}.webp'
        self.leader_display = f'spheres-web/ui/display-art/Testland-{sha(leader_display)[:16]}.webp'
        self.files.update({
            f'{PP}/alice-cartoon-1990-v1.png': alice, f'{PP}/bob-cartoon-1990-v1.png': bob,
            f'{PP}/fictional-testland-01-cartoon-2030-v1.png': fic, f'{PP}/references/alice-1989.jpg': ref,
            f'spheres-web/ui/portraits/{self.photo_name}': photo, f'spheres-web/ui/leader-art/{self.leader_name}': leader,
            self.photo_display: photo_display, self.leader_display: leader_display,
            'tools/avatars/prompts/Testland-founder-one-v1.md':
                f'# Testland\r\n\r\nStyle reference: `spheres-web/ui/leader-art/{self.leader_name}`\r\n'.encode(),
        })
        self.registry = {'version': 1, 'people': [
            {'id': 'alice', 'name': 'Alice Example', 'sources': ['https://example.org/alice']},
            {'id': 'bob', 'name': 'Bob Example', 'sources': ['https://example.org/bob'], 'died': {'kind': 'day', 'value': '1999-01-01'}},
            {'id': 'carol', 'name': 'Carol Missing', 'sources': ['https://example.org/carol']},
            {'id': 'dave', 'name': 'Dave Unwindowed', 'sources': ['https://example.org/dave']}], 'parties': []}
        review = {'identity': True, 'likeness': True, 'era': True, 'visual': True, 'reviewer': 'Fixture QA', 'reviewed_at': '2026-09-07'}
        prompt = 'tools/avatars/person-prompts/batch.json'
        self.portraits = {'version': 1, 'people': {
            'alice': {'name': 'Alice Example', 'identity_sources': ['https://example.org/alice'], 'portraits': [{
                'from': '1990-01-01', 'to': '1995-01-01', 'method': 'generated', 'style': 'cartoon', 'status': 'illustrated-likeness',
                'asset': f'{PP}/alice-cartoon-1990-v1.png', 'sha256': sha(alice), 'width': 4, 'height': 6, 'license': 'generated',
                'generator': cr_generator(), 'generated_at': '2026-09-07', 'credit': 'Fixture cartoon.', 'era_note': 'Fixture era.',
                'identity_source': {'person_id': 'alice', 'kind': 'observed_portrait', 'source_url': 'https://example.org/alice-photo',
                                    'asset': f'{PP}/references/alice-1989.jpg', 'sha256': sha(ref), 'license': 'CC BY 4.0'},
                'prompt_record': prompt, 'generation_record': 'Fixture.', 'review': dict(review)}]},
            'bob': {'name': 'Bob Example', 'identity_sources': ['https://example.org/bob'], 'portraits': [{
                'from': '1990-01-01', 'to': '1990-07-01', 'method': 'generated', 'style': 'cartoon', 'status': 'illustrated-likeness',
                'asset': f'{PP}/bob-cartoon-1990-v1.png', 'sha256': sha(bob), 'width': 4, 'height': 6, 'license': 'generated',
                'generator': cr_generator(), 'generated_at': '2026-09-07', 'credit': 'Fixture cartoon.', 'era_note': 'Fixture era.',
                'identity_source': {'person_id': 'bob', 'kind': 'authored_identity', 'source_url': 'https://example.org/bob'},
                'prompt_record': prompt, 'generation_record': 'Fixture.', 'review': dict(review)}]},
            'carol': {'name': 'Carol Missing', 'identity_sources': ['https://example.org/carol'], 'portraits': []},
            'dave': {'name': 'Dave Unwindowed', 'identity_sources': ['https://example.org/dave'], 'portraits': []}}}
        fid = 'fictional_v1_testland_p_01'
        self.fictional = {'version': 1, 'people': {fid: {'name': 'Nia Invented', 'appearance_seed': '0123456789abcdef', 'portraits': [{
            'from': '2026-09-08', 'to': '2036-01-01', 'method': 'generated', 'style': 'cartoon', 'status': 'fictional-character',
            'asset': f'{PP}/fictional-testland-01-cartoon-2030-v1.png', 'sha256': sha(fic), 'width': 4, 'height': 6,
            'license': 'generated', 'generator': cr_generator(), 'generated_at': '2026-09-07', 'credit': 'Invented.', 'era_note': 'Invented.',
            'prompt_record': prompt, 'generation_record': 'Fixture.',
            'design_source': {'kind': 'authored_fiction', 'person_id': fid, 'appearance_seed': '0123456789abcdef',
                              'catalog': 'spheres-web/data/future_candidates_2035.json'},
            'review': {'design': True, 'visual': True, 'reviewer': 'Fixture QA', 'reviewed_at': '2026-09-07'}}]}}}
        self.catalog = {'version': 1, 'from': '2026-09-08', 'until_exclusive': '2036-01-01', 'historical_reference_through': '2026-09-07',
                        'candidates': [{'person_id': fid, 'name': 'Nia Invented', 'appearance_seed': '0123456789abcdef',
                                        'origin': 'fictional_successor', 'nation': 'Testland', 'party': 'tl_party'}]}
        self.figures = {'version': 3, 'nations': {'Testland': {
            'figure': 'Founder One', 'canonical_lookup': 'Founder One', 'wikidata': 'Q1', 'years': '1800–1870',
            'portrait': {'asset': self.photo_name, 'sha256': sha(photo), 'license': 'Public domain',
                         'review': 'automated exact Wikipedia-title candidate'},
            'leader_art': {'asset': self.leader_name, 'sha256': sha(leader), 'identity_source_asset': self.photo_name,
                           'style': 'soft-arcade-historical-leader-v1', 'generated': '2026-09-02',
                           'prompt_record': 'tools/avatars/prompts/Testland-founder-one-v1.md', 'review': 'Reviewed at full size.'}}}}
        self.display = [
            {'source': f'spheres-web/ui/leader-art/{self.leader_name}', 'source_sha256': sha(leader), 'display': self.leader_display,
             'source_size': [4, 6], 'display_size': [2, 3], 'source_bytes': len(leader), 'display_bytes': len(leader_display)},
            {'source': f'spheres-web/ui/portraits/{self.photo_name}', 'source_sha256': sha(photo), 'display': self.photo_display,
             'source_size': [6, 8], 'display_size': [3, 4], 'source_bytes': len(photo), 'display_bytes': len(photo_display)}]
        self.production = {
            'version': 1, 'style': {'anchor': f'{PP}/alice-cartoon-1990-v1.png', 'anchor_sha256': sha(alice)}, 'input_hashes': {},
            'countries': [{'id': 'Testland', 'name': 'Testland Republic'}, {'id': 'Otherland', 'name': 'Otherland'}],
            'people': [
                {'id': 'alice', 'name': 'Alice Example', 'period_kind': 'historical', 'reasons': [{'kind': 'sourced_party_term', 'nation': 'Testland', 'party': 'tl_party'}],
                 'required_art_windows': [{'from': '1990-01-01', 'to': '1993-01-01'}], 'pending_art_job_ids': []},
                {'id': 'bob', 'name': 'Bob Example', 'period_kind': 'historical', 'reasons': [{'kind': 'sourced_party_term', 'nation': 'Otherland', 'party': 'ol_party'}],
                 'required_art_windows': [{'from': '1990-01-01', 'to': '1991-01-01'}], 'pending_art_job_ids': []},
                {'id': 'carol', 'name': 'Carol Missing', 'period_kind': 'historical', 'reasons': [{'kind': 'sourced_party_term', 'nation': 'Testland', 'party': 'tl_party'}],
                 'required_art_windows': [{'from': '1990-01-01', 'to': '1990-01-02'}], 'pending_art_job_ids': ['job:carol']},
                {'id': 'dave', 'name': 'Dave Unwindowed', 'period_kind': 'historical', 'reasons': [], 'required_art_windows': [], 'pending_art_job_ids': []}]}
        self.prompts = {'jobs': [
            {'person_id': 'alice', 'asset': f'{PP}/alice-cartoon-1990-v1.png', 'prompt': 'Draw Alice.', 'source': 'https://example.org/alice-photo',
             'reference_note': 'Alice is on the left.', 'refs': ['C:\\work\\anchor.png', 'C:\\work\\alice-1989.jpg']},
            {'person_id': 'bob', 'asset': f'{PP}/bob-cartoon-1990-v1.png', 'prompt': 'Draw Bob.', 'source': 'https://example.org/bob'},
            {'person_id': fid, 'asset': f'{PP}/fictional-testland-01-cartoon-2030-v1.png', 'prompt': 'Invent Nia.'}]}
        self.sample = None

    def documents(self) -> dict:
        docs = {'spheres-web/data/person_portraits.json': self.portraits, 'spheres-web/data/fictional_portraits.json': self.fictional,
                'spheres-web/data/nation_figures.json': self.figures, 'spheres-web/ui/display-art/manifest.json': self.display,
                'spheres-web/data/leadership_production_2035.json': self.production,
                'spheres-web/data/future_candidates_2035.json': self.catalog, 'spheres-sim/data/party_leaders.json': self.registry,
                'tools/avatars/person-prompts/batch.json': self.prompts}
        if self.sample is not None:
            docs[cr.VISUAL_REVIEW.as_posix()] = self.sample
        return docs

    def write(self, root: Path):
        """Mirror the fixture into root: files dropped from the fixture are removed; export outputs are kept."""
        docs = self.documents()
        for path in [p for p in root.rglob('*') if p.is_file()]:
            rel = path.relative_to(root).as_posix()
            if rel not in self.files and rel not in docs and not rel.startswith(cr.OUTPUT.as_posix() + '/'):
                path.unlink()
        for rel, data in self.files.items():
            (root / rel).parent.mkdir(parents=True, exist_ok=True)
            (root / rel).write_bytes(data)
        for rel, doc in docs.items():
            (root / rel).parent.mkdir(parents=True, exist_ok=True)
            (root / rel).write_text(json.dumps(doc, ensure_ascii=False, indent=1) + '\n', encoding='utf-8', newline='\n')


def cr_generator():
    return 'OpenAI built-in image_gen'


class FixtureCase(unittest.TestCase):
    def setUp(self):
        self.tmp = Path(tempfile.mkdtemp(prefix='c03-cartoon-review-'))
        self.fx = Fixture()

    def tearDown(self):
        shutil.rmtree(self.tmp, ignore_errors=True)

    def export(self) -> dict:
        self.fx.write(self.tmp)
        return cr.build(self.tmp)

    @staticmethod
    def codes(export, item=None, severity=None):
        return sorted(f['code'] for f in export['findings']
                      if (item is None or f.get('item') == item) and (severity is None or f['severity'] == severity))

    @staticmethod
    def item(export, item_id):
        return next(i for i in export['items'] if i['id'] == item_id)

    @staticmethod
    def archived(export, item_id):
        return next(i for i in export['archived_selector'] if i['id'] == item_id)


class CleanFixture(FixtureCase):
    def test_archive_only_nation_keeps_roster_metadata_without_an_active_figure(self):
        self.fx.production['countries'].append({'id': 'Retiredland', 'name': 'Retired Republic'})
        self.fx.figures['nations']['Retiredland'] = {
            'figure': 'Former Founder', 'wikidata': 'Q2', 'years': '1800–1870'}
        export = self.export()
        self.assertEqual(export['nations']['Retiredland'], 'Retired Republic')
        self.assertFalse(any('Retiredland' in i.get('countries', []) for i in export['items']))
        self.assertEqual(self.archived(export, 'selector:Retiredland')['name'], 'Former Founder')
        self.assertEqual(export['summary']['countries_with_items'], 2)
        self.assertEqual(export['summary']['cartoon_items_with_files'], 3)

    def test_clean_fixture_has_no_errors_and_labels_every_collection(self):
        export = self.export()
        self.assertEqual(self.codes(export, severity='error'), [])
        self.assertEqual(export['file_inventory'], 'filesystem')
        self.assertEqual(export['summary']['items'], {'historical': 2, 'fictional': 1, 'unregistered': 0, 'missing': 1})
        self.assertEqual(export['summary']['cartoon_items_with_files'], 3)
        self.assertEqual(export['summary']['archived_selector_items'], 1)
        self.assertEqual(export['summary']['archived_selector_items_with_files'], 1)
        self.assertNotIn('selector', export['collections'])
        self.assertFalse(any(i['id'].startswith('selector:') for i in export['items']))
        self.assertEqual(export['summary']['approved_by_this_export'], 0)
        self.assertIn('not visual approval', export['statement'])
        self.assertIn('fictional', self.item(export, 'fictional:fictional_v1_testland_p_01#0')['labels'])
        self.assertIn('missing-art', self.item(export, 'missing:carol')['labels'])
        self.assertNotIn('missing:dave', [i['id'] for i in export['items']], 'no art window means no missing-art card')
        selector = self.archived(export, 'selector:Testland')
        self.assertEqual(selector['card'], self.fx.leader_display, 'the retired display derivative remains pinned for audit')
        self.assertTrue(selector['archived'])
        self.assertEqual(selector['labels'][0], 'archived-selector')
        self.assertEqual(export['nations']['Testland'], 'Testland Republic')
        roles = [ref['role'] for ref in export['style_references']]
        self.assertTrue(any('person-cartoon style anchor' in r for r in roles))
        self.assertFalse(any('country-selector' in r for r in roles))
        self.assertTrue(all(ref['path'].startswith(PP + '/') for ref in export['style_references']))
        self.assertEqual(self.item(export, 'historical:alice#0')['labels'][:2], ['historical', 'style-reference'])

    def test_file_facts_come_from_headers_for_png_webp_and_jpeg(self):
        files = {f['path']: f for f in self.export()['files']}
        self.assertEqual((files[f'{PP}/alice-cartoon-1990-v1.png']['format'], files[f'{PP}/alice-cartoon-1990-v1.png']['width']), ('PNG', 4))
        self.assertEqual((files[f'{PP}/references/alice-1989.jpg']['format'], files[f'{PP}/references/alice-1989.jpg']['height']), ('JPEG', 10))
        photo = files[f'spheres-web/ui/portraits/{self.fx.photo_name}']
        self.assertEqual((photo['format'], photo['width'], photo['height']), ('WEBP', 6, 8))
        self.assertEqual(photo['bindings'], [['identity-reference-photo', 'figure:Q1', 'selector:Testland']])
        for bad in (b'not an image', b'\x89PNG\r\n\x1a\n\x00\x00', b'RIFF\x00\x00\x00\x00WEBPVP8 '):
            with self.assertRaises(ValueError):
                cr.image_info(bad)

    def test_dated_person_and_reviewed_tupou_style_references_remain(self):
        self.fx.files['docs/campaign-certification/S10/c/manifest.json'] = json.dumps({
            'portrait_scope': {'person_id': 'bob', 'from': '1990-01-01', 'to': '1990-07-01'}
        }).encode('utf-8')
        export = self.export()
        self.assertEqual([ref['item'] for ref in export['style_references']],
                         ['historical:alice#0', 'historical:bob#0'])
        self.assertIn('S10.c Tupou IV review', export['style_references'][1]['role'])
        self.assertNotIn('style-reference', self.archived(export, 'selector:Testland')['labels'])


class MissingFiles(FixtureCase):
    def test_missing_file_is_an_error_labelled_on_the_item(self):
        del self.fx.files[f'{PP}/bob-cartoon-1990-v1.png']
        export = self.export()
        self.assertIn('missing_file', self.codes(export, 'historical:bob#0', 'error'))
        bob = self.item(export, 'historical:bob#0')
        self.assertIn('file-missing', bob['labels'])
        self.assertIn('integrity-error', bob['labels'])
        self.assertFalse(next(f for f in export['files'] if f['path'] == bob['asset'])['exists'])

    def test_hash_dimension_and_unsafe_path_findings(self):
        portrait = self.fx.portraits['people']['alice']['portraits'][0]
        portrait['sha256'] = '0' * 64
        portrait['width'] = 5
        self.fx.portraits['people']['bob']['portraits'][0]['asset'] = f'{PP}/../../secret.png'
        export = self.export()
        self.assertIn('hash_mismatch', self.codes(export, 'historical:alice#0', 'error'))
        self.assertIn('dimension_mismatch', self.codes(export, 'historical:alice#0', 'error'))
        self.assertIn('unsafe_path', self.codes(export, 'historical:bob#0', 'error'))

    def test_missing_display_derivative_and_stale_source_are_reported(self):
        del self.fx.files[self.fx.photo_display]
        self.fx.display[0]['source_sha256'] = 'f' * 64
        export = self.export()
        paths = {(f['code'], f.get('path')) for f in export['findings']}
        self.assertIn(('missing_file', self.fx.photo_display), paths)
        self.assertIn(('display_derivative_stale', self.fx.leader_display), paths)

    def test_archived_integrity_errors_keep_identity_and_reference_bindings(self):
        self.fx.figures['nations']['Testland']['leader_art']['sha256'] = '0' * 64
        self.fx.files[self.fx.photo_display] = b'broken old derivative'
        export = self.export()
        selector = self.archived(export, 'selector:Testland')
        self.assertIn('integrity-error', selector['labels'])
        failures = [f for f in export['findings'] if f['severity'] == 'error']
        self.assertEqual({f['code'] for f in failures}, {'hash_mismatch', 'unreadable_image'})
        self.assertTrue(all(f['scope'] == 'archived_selector' for f in failures))
        self.assertEqual(export['summary']['findings_by_severity']['error'], 2)
        self.assertEqual(export['summary']['active_findings_by_severity']['error'], 0)
        self.assertEqual(export['summary']['archived_selector_findings_by_severity']['error'], 2)
        files = {f['path']: f for f in export['files']}
        self.assertEqual(files[self.fx.photo_display]['bindings'],
                         [['identity-photo-display-derivative', 'figure:Q1', 'selector:Testland']])
        self.assertEqual(files[f'spheres-web/ui/portraits/{self.fx.photo_name}']['bindings'],
                         [['identity-reference-photo', 'figure:Q1', 'selector:Testland']])
        self.assertNotIn('unbound_file', self.codes(export))
        rendered = cr.render(export)
        self.assertIn('hash_mismatch@selector:Testland', rendered)
        self.assertIn('| Archived national icons | 2 |', rendered)


class Intervals(FixtureCase):
    def test_invalid_empty_and_absent_intervals_are_errors(self):
        alice = self.fx.portraits['people']['alice']['portraits'][0]
        bob = self.fx.portraits['people']['bob']['portraits'][0]
        alice['to'] = '1990-01-01'
        bob['from'] = '1990-13-01'
        export = self.export()
        self.assertIn('invalid_interval', self.codes(export, 'historical:alice#0', 'error'))
        self.assertIn('invalid_interval', self.codes(export, 'historical:bob#0', 'error'))
        self.assertIn('interval-issue', self.item(export, 'historical:alice#0')['labels'])
        del self.fx.portraits['people']['alice']['portraits'][0]['to']
        self.assertIn('invalid_interval', self.codes(self.export(), 'historical:alice#0', 'error'))

    def test_overlapping_appearances_and_period_boundaries(self):
        alice = self.fx.portraits['people']['alice']['portraits']
        second = copy.deepcopy(alice[0])
        second.update(**{'from': '1994-01-01', 'to': '2027-01-01'})
        alice.append(second)
        self.fx.fictional['people']['fictional_v1_testland_p_01']['portraits'][0]['from'] = '2020-01-01'
        export = self.export()
        self.assertIn('overlapping_intervals', self.codes(export, 'historical:alice#1', 'error'))
        outside = {f.get('item'): f['severity'] for f in export['findings'] if f['code'] == 'interval_outside_period'}
        self.assertEqual(outside, {'historical:alice#1': 'warning', 'fictional:fictional_v1_testland_p_01#0': 'error'})

    def test_coverage_gap_missing_art_and_interval_after_death(self):
        self.fx.registry['people'][1]['died'] = {'kind': 'month', 'value': '1990-03'}
        export = self.export()
        gap = next(f for f in export['findings'] if f['code'] == 'coverage_gap')
        self.assertEqual(gap['item'], 'historical:bob#0')
        self.assertIn('1990-07-01 → 1991-01-01', gap['message'])
        self.assertEqual(self.item(export, 'historical:bob#0')['coverage_gaps'], [{'from': '1990-07-01', 'to': '1991-01-01'}])
        self.assertEqual(self.item(export, 'historical:alice#0').get('coverage_gaps'), None)
        death = next(f for f in export['findings'] if f['code'] == 'interval_after_death')
        self.assertEqual((death['item'], death['severity']), ('historical:bob#0', 'notice'))
        self.assertIn('1990-04-01', death['message'], 'month precision clips at the end of the death month')
        self.assertEqual(self.codes(export, 'missing:carol'), ['missing_art'])


class Duplicates(FixtureCase):
    def test_identical_bytes_for_different_people_are_warnings_not_errors(self):
        self.fx.files[f'{PP}/bob-cartoon-1990-v1.png'] = self.fx.files[f'{PP}/alice-cartoon-1990-v1.png']
        self.fx.portraits['people']['bob']['portraits'][0]['sha256'] = sha(self.fx.files[f'{PP}/alice-cartoon-1990-v1.png'])
        export = self.export()
        self.assertEqual(self.codes(export, severity='error'), [])
        for item in ('historical:alice#0', 'historical:bob#0'):
            self.assertIn('duplicate_content_different_identities', self.codes(export, item, 'warning'))
            self.assertIn('duplicate', self.item(export, item)['labels'])
        group = next(g for g in export['duplicates'] if g['different_identities'])
        self.assertEqual(group['identities'], ['person:alice', 'person:bob'])
        self.assertIn('not proof of a wrong identity', next(f for f in export['findings'] if f['code'] == 'duplicate_content_different_identities')['message'])

    def test_one_path_bound_to_two_people_is_reported(self):
        self.fx.portraits['people']['bob']['portraits'][0].update(
            asset=f'{PP}/alice-cartoon-1990-v1.png', sha256=sha(self.fx.files[f'{PP}/alice-cartoon-1990-v1.png']))
        del self.fx.files[f'{PP}/bob-cartoon-1990-v1.png']
        self.assertIn('duplicate_asset_path_different_identities', self.codes(self.export(), 'historical:bob#0', 'warning'))

    def test_same_identity_duplicates_are_grouped_without_a_finding(self):
        photo = self.fx.files[f'spheres-web/ui/portraits/{self.fx.photo_name}']
        other = copy.deepcopy(self.fx.figures['nations']['Testland'])
        other_photo = f'Otherland-{sha(photo)[:12]}.webp'
        other['portrait']['asset'] = other['leader_art']['identity_source_asset'] = other_photo
        self.fx.files[f'spheres-web/ui/portraits/{other_photo}'] = photo
        self.fx.figures['nations']['Otherland'] = other
        export = self.export()
        group = next(g for g in export['duplicates'] if f'spheres-web/ui/portraits/{other_photo}' in g['paths'])
        self.assertFalse(group['different_identities'])
        self.assertNotIn('duplicate_content_different_identities', self.codes(export))


class Identities(FixtureCase):
    def test_unknown_and_mismatched_identities_are_errors(self):
        self.fx.portraits['people']['zed'] = copy.deepcopy(self.fx.portraits['people']['bob'])
        self.fx.portraits['people']['zed']['portraits'][0]['identity_source']['person_id'] = 'zed'
        self.fx.portraits['people']['bob']['portraits'][0]['identity_source']['person_id'] = 'alice'
        export = self.export()
        self.assertIn('unknown_identity', self.codes(export, 'historical:zed#0', 'error'))
        self.assertIn('unknown-identity', self.item(export, 'historical:zed#0')['labels'])
        self.assertIn('identity_binding_mismatch', self.codes(export, 'historical:bob#0', 'error'))

    def test_fictional_identities_must_stay_fictional(self):
        people = self.fx.fictional['people']
        people['fictional_v1_unknown'] = copy.deepcopy(people['fictional_v1_testland_p_01'])
        people['fictional_v1_unknown']['portraits'][0]['design_source']['person_id'] = 'fictional_v1_unknown'
        people['alice'] = copy.deepcopy(people['fictional_v1_testland_p_01'])
        people['alice']['portraits'][0]['design_source']['person_id'] = 'alice'
        export = self.export()
        self.assertIn('unknown_identity', self.codes(export, 'fictional:fictional_v1_unknown#0', 'error'))
        self.assertIn('fictional_identity_collision', self.codes(export, 'fictional:alice#0', 'error'))

    def test_selector_nation_and_unbound_files_are_visible_unknowns(self):
        self.fx.figures['nations']['Atlantis'] = copy.deepcopy(self.fx.figures['nations']['Testland'])
        self.fx.files[f'{PP}/stray-study-v1.png'] = png(2, 2, 9)
        export = self.export()
        self.assertIn('unknown_identity', self.codes(export, 'selector:Atlantis', 'error'))
        stray = self.item(export, f'file:{PP}/stray-study-v1.png')
        self.assertEqual(stray['collection'], 'unregistered')
        self.assertIn('unknown-identity', stray['labels'])
        self.assertEqual(self.codes(export, stray['id']), ['unbound_file'])
        self.assertEqual(self.codes(export, 'missing:carol'), ['missing_art'])

    def test_country_unbound_person_is_labelled(self):
        self.fx.production['people'][1]['reasons'] = []
        bob = self.item(self.export(), 'historical:bob#0')
        self.assertIn('country-unknown', bob['labels'])


class Rights(FixtureCase):
    def test_rights_gaps_are_reported_without_legal_conclusions(self):
        self.fx.portraits['people']['alice']['portraits'][0]['identity_source']['license'] = 'CC BY-SA 4.0'
        self.fx.figures['nations']['Testland']['portrait']['license'] = 'CC BY-SA 3.0'
        self.fx.prompts['jobs'][1]['identity_reference'] = f'{PP}/bob-study-1990-v1.png'
        self.fx.files[f'{PP}/bob-study-1990-v1.png'] = png(2, 2, 10)
        export = self.export()
        self.assertIn('sharealike_derivative_license_not_recorded', self.codes(export, 'historical:alice#0', 'warning'))
        self.assertIn('sharealike_derivative_license_not_recorded', self.codes(export, 'selector:Testland', 'warning'))
        self.assertIn('reference_rights_not_recorded', self.codes(export, 'historical:bob#0', 'notice'))
        self.assertIn('identity_reference_generated', self.codes(export, 'historical:bob#0', 'warning'))
        self.assertNotIn('unbound_file', self.codes(export), 'a generated identity study is bound through its prompt record')
        self.fx.portraits['people']['alice']['portraits'][0]['derivative_license'] = 'CC BY-SA 4.0'
        self.assertNotIn('sharealike_derivative_license_not_recorded', self.codes(self.export(), 'historical:alice#0'))

    def test_prompt_records_are_summarized_without_local_paths(self):
        export = self.export()
        prompt = self.item(export, 'historical:alice#0')['prompt']
        self.assertEqual(prompt['reference_files'], ['anchor.png', 'alice-1989.jpg'])
        self.assertNotIn('C:\\', json.dumps(export))
        selector_prompt = self.archived(export, 'selector:Testland')['prompt']
        text = self.fx.files['tools/avatars/prompts/Testland-founder-one-v1.md'].replace(b'\r\n', b'\n')
        self.assertEqual(selector_prompt['sha256_lf'], sha(text), 'text provenance is hashed after CRLF normalization')
        del self.fx.files['tools/avatars/prompts/Testland-founder-one-v1.md']
        self.fx.portraits['people']['bob']['portraits'][0]['prompt_record'] = 'tools/avatars/person-prompts/absent.json'
        export = self.export()
        self.assertIn('prompt_record_missing', self.codes(export, 'selector:Testland', 'error'))
        self.assertIn('prompt_record_missing', self.codes(export, 'historical:bob#0', 'error'))


class VisualReviewGuard(FixtureCase):
    def sample(self, **entry):
        base = {'item': 'historical:alice#0', 'sha256': sha(self.fx.files[f'{PP}/alice-cartoon-1990-v1.png']),
                'decision': 'fixes_proposed', 'fixes': ['Raise the hairline.']}
        base.update(entry)
        return {'format': cr.SAMPLE_FORMAT, 'reviewer': 'Fixture', 'reviewed_on': '2026-09-27', 'statement': 'Notes only.',
                'method': 'Viewed.', 'entries': [base]}

    def test_sample_entries_attach_to_items_and_detect_changed_bytes(self):
        self.fx.sample = self.sample()
        export = self.export()
        alice = self.item(export, 'historical:alice#0')
        self.assertEqual(alice['visual_review'], {'decision': 'fixes_proposed', 'fixes': 1, 'current_sha256_matches': True})
        self.assertIn('sample-reviewed', alice['labels'])
        self.fx.sample = self.sample(sha256='1' * 64)
        self.assertIn('visual_review_stale', self.codes(self.export(), 'historical:alice#0', 'warning'))
        self.fx.sample = self.sample(item='historical:nobody#0')
        self.assertIn('visual_review_unknown_item', self.codes(self.export(), severity='error'))

    def test_retired_visual_sample_is_audited_without_becoming_active(self):
        asset = f'spheres-web/ui/leader-art/{self.fx.leader_name}'
        self.fx.sample = self.sample(item='selector:Testland', sha256=sha(self.fx.files[asset]))
        original_sample = copy.deepcopy(self.fx.sample)
        export = self.export()
        self.assertEqual(self.fx.sample, original_sample)
        self.assertNotIn('visual_review_unknown_item', self.codes(export))
        self.assertTrue(self.archived(export, 'selector:Testland')['visual_review']['current_sha256_matches'])
        self.assertIn('sample-reviewed', self.archived(export, 'selector:Testland')['labels'])
        self.assertFalse(any(i['id'] == 'selector:Testland' for i in export['items']))
        self.assertEqual(export['visual_review_sample']['entries'][0],
                         {**original_sample['entries'][0], 'current_sha256_matches': True})
        self.fx.sample['entries'][0]['sha256'] = '1' * 64
        stale = next(f for f in self.export()['findings'] if f['code'] == 'visual_review_stale')
        self.assertEqual((stale['item'], stale['scope']), ('selector:Testland', 'archived_selector'))

    def test_unknown_archived_sample_retains_context_and_remains_an_error(self):
        self.fx.sample = self.sample(item='selector:Atlantis')
        export = self.export()
        finding = next(f for f in export['findings'] if f['code'] == 'visual_review_unknown_item')
        self.assertEqual(finding['item'], 'selector:Atlantis')
        self.assertEqual(finding['severity'], 'error')
        self.assertEqual(finding['scope'], 'active', 'unknown records must not disappear into a trusted archive')
        self.assertEqual(export['summary']['active_findings_by_severity']['error'], 1)
        self.assertIn('selector:Atlantis', cr.render(export))

    def test_sample_can_never_record_approval(self):
        for bad in ({'decision': 'approved'}, {'decision': 'accepted'}, {'approved': True}, {'fixes': []}, {'fixes': ['  ']}):
            with self.subTest(bad=bad):
                self.fx.sample = self.sample(**bad)
                with self.assertRaises(SystemExit):
                    self.export()


class Regeneration(FixtureCase):
    def test_json_input_hashes_are_portable_without_changing_art_or_source_bytes(self):
        self.fx.write(self.tmp)
        expected = cr.outputs(self.tmp)
        for _role, rel, _required in cr.INPUTS:
            path = self.tmp / rel
            if path.is_file():
                path.write_bytes(path.read_bytes().replace(b'\n', b'\r\n'))
        before = {p: p.read_bytes() for p in self.tmp.rglob('*') if p.is_file()}
        self.assertEqual(cr.outputs(self.tmp), expected)
        self.assertEqual({p: p.read_bytes() for p in before}, before)
        # A real content change must still invalidate the export.
        registry = self.tmp / cr.REGISTRY
        registry.write_bytes(registry.read_bytes().replace(b'Alice Example', b'Alice Changed'))
        self.assertNotEqual(cr.outputs(self.tmp), expected)

    def test_regeneration_is_deterministic_and_check_detects_staleness(self):
        self.fx.write(self.tmp)
        first = cr.outputs(self.tmp)
        self.assertEqual(first, cr.outputs(self.tmp))
        cr.write(first, self.tmp)
        self.assertEqual(cr.stale(cr.outputs(self.tmp), self.tmp), [])
        with contextlib.redirect_stdout(io.StringIO()):
            self.assertEqual(cr.main(['--check', '--root', str(self.tmp)]), 0)
        markdown = self.tmp / cr.EXPORT_MD
        markdown.write_bytes(markdown.read_bytes().replace(b'\n', b'\r\n'))
        self.assertEqual(cr.stale(cr.outputs(self.tmp), self.tmp), [], 'a CRLF checkout of the Markdown is not stale')
        self.fx.portraits['people']['bob']['portraits'][0]['to'] = '1990-08-01'
        self.fx.write(self.tmp)
        self.assertEqual(cr.stale(cr.outputs(self.tmp), self.tmp), [cr.EXPORT_JSON.as_posix(), cr.EXPORT_MD.as_posix()])
        with contextlib.redirect_stdout(io.StringIO()), contextlib.redirect_stderr(io.StringIO()):
            self.assertEqual(cr.main(['--check', '--root', str(self.tmp)]), 1)

    def test_export_never_changes_its_inputs(self):
        self.fx.write(self.tmp)
        before = {p.relative_to(self.tmp).as_posix(): sha(p.read_bytes()) for p in self.tmp.rglob('*') if p.is_file()}
        with contextlib.redirect_stdout(io.StringIO()):
            cr.main(['--root', str(self.tmp)])
        after = {p.relative_to(self.tmp).as_posix(): sha(p.read_bytes()) for p in self.tmp.rglob('*') if p.is_file()}
        written = {cr.EXPORT_JSON.as_posix(), cr.EXPORT_MD.as_posix()}
        self.assertEqual(set(after) - set(before), written)
        self.assertEqual({k: v for k, v in after.items() if k not in written}, before)
        json.loads((self.tmp / cr.EXPORT_JSON).read_text(encoding='utf-8'))


class Repository(unittest.TestCase):
    def test_committed_export_is_current_deterministic_and_read_only(self):
        inputs = [rel for _role, rel, _req in cr.INPUTS if (cr.ROOT / rel).is_file()]
        before = {rel: sha((cr.ROOT / rel).read_bytes()) for rel in inputs}
        files = cr.outputs(cr.ROOT)
        self.assertEqual(files, cr.outputs(cr.ROOT))
        self.assertEqual(cr.stale(files, cr.ROOT), [], 'regenerate with: python -X utf8 tools/avatars/cartoon_review.py')
        self.assertEqual(before, {rel: sha((cr.ROOT / rel).read_bytes()) for rel in inputs})
        export = json.loads(files[cr.EXPORT_JSON])
        self.assertEqual(export['summary']['approved_by_this_export'], 0)
        self.assertEqual(export['summary']['findings_by_severity']['error'], 0)
        sample = export['visual_review_sample']
        self.assertTrue(6 <= len(sample['entries']) <= 8)
        for entry in sample['entries']:
            self.assertIn(entry['decision'], cr.ALLOWED_DECISIONS)
            self.assertTrue(entry['current_sha256_matches'], entry['item'])
            self.assertTrue(entry['fixes'])
        recorded = {i['path']: i['sha256'] for i in export['inputs']}
        for rel in inputs:
            self.assertEqual(recorded[rel], sha((cr.ROOT / rel).read_bytes().replace(b'\r\n', b'\n')))
        self.assertTrue(all(i['hash_scope'] == 'utf8-lf' for i in export['inputs']))

    @unittest.skipUnless(__import__('importlib').util.find_spec('PIL'), 'Pillow not installed')
    def test_header_dimensions_match_pillow_on_repository_images(self):
        from PIL import Image
        export = json.loads(cr.outputs(cr.ROOT)[cr.EXPORT_JSON])
        seen = set()
        for record in export['files']:
            data = (cr.ROOT / record['path']).read_bytes()
            kind = (record.get('format'), Path(record['path']).parts[2], data[12:16] if record.get('format') == 'WEBP' else b'')
            if kind in seen:
                continue
            seen.add(kind)
            with Image.open(io.BytesIO(data)) as image:
                self.assertEqual((record['width'], record['height']), image.size, record['path'])
                self.assertEqual(record['format'], image.format)
        self.assertTrue({'PNG', 'WEBP', 'JPEG'} <= {k[0] for k in seen})
        self.assertTrue({b'VP8 ', b'VP8X'} <= {k[2] for k in seen}, 'both lossy and extended WebP headers are compared')


if __name__ == '__main__':
    unittest.main()
