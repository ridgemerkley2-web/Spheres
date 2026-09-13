"""Bounded physical-file and exact-person checks for Tonga's 1990 illustration."""
import copy
import hashlib
import unittest

import person_art_pipeline as pipeline


class TupouPortraitChecks(unittest.TestCase):
    @classmethod
    def setUpClass(cls):
        cls.person_id = 'taufaahau_tupou_iv'
        cls.manifest = pipeline.read_json(pipeline.DEFAULT_MANIFEST)
        cls.known = pipeline.people_from_registry(pipeline.read_json(pipeline.DEFAULT_REGISTRY))
        cls.person = cls.manifest['people'][cls.person_id]
        cls.portrait = cls.person['portraits'][0]
        cls.scoped = {'version': 1, 'people': {cls.person_id: cls.person}}
        cls.provenance = pipeline.read_json(pipeline.ROOT / 'tools/avatars/person-prompts/taufaahau-tupou-iv-cartoon-1990-v1.json')

    def test_reviewed_art_has_valid_real_files_and_exact_identity(self):
        result = pipeline.validate_manifest(self.scoped, pipeline.ROOT, self.known)
        self.assertTrue(result['valid'], result['errors'])
        self.assertEqual(list(result['ready']), [self.person_id])
        self.assertEqual(self.portrait['identity_source']['person_id'], self.person_id)
        self.assertEqual(self.known[self.person_id]['name'], self.person['name'])
        self.assertEqual((self.portrait['width'], self.portrait['height']), (1024, 1536))

    def test_conservative_half_open_appearance_window(self):
        for date in ('1990-01-01', '1990-07-04', '1990-12-31'):
            with self.subTest(date=date):
                self.assertIsNotNone(pipeline.select_portrait(self.scoped, self.person_id, date, pipeline.ROOT, self.known))
        for date in ('1989-12-31', '1991-01-01', '2006-09-10', '2035-01-01'):
            with self.subTest(date=date):
                self.assertIsNone(pipeline.select_portrait(self.scoped, self.person_id, date, pipeline.ROOT, self.known))

    def test_never_substitutes_nation_or_other_tupou_identity(self):
        for person in ('Tonga', 'salote_tupou_iii', 'george_tupou_v', 'tupou_vi'):
            with self.subTest(person=person):
                self.assertIsNone(pipeline.select_portrait(self.scoped, person, '1990-01-01', pipeline.ROOT, self.known))
        for person_id, person in self.manifest['people'].items():
            if person_id != self.person_id:
                self.assertTrue(all(p['sha256'] != self.portrait['sha256'] for p in person['portraits']))

    def test_provenance_binds_prompt_style_source_and_exact_output(self):
        for record in [self.provenance['final_art'], self.provenance['prompt'], *self.provenance['inputs']]:
            data = (pipeline.ROOT / record['asset']).read_bytes()
            self.assertEqual(hashlib.sha256(data).hexdigest(), record['sha256'])
            self.assertEqual(len(data), record['bytes'])
        self.assertEqual(self.provenance['final_art']['sha256'], self.portrait['sha256'])
        self.assertEqual(self.provenance['source_image']['sha256'], self.portrait['identity_source']['sha256'])
        self.assertNotEqual(self.portrait['sha256'], self.portrait['identity_source']['sha256'])
        self.assertEqual(self.provenance['exact_submitted_prompt'], (pipeline.ROOT / self.portrait['prompt_record']).read_text(encoding='utf-8').rstrip('\n'))
        self.assertEqual(self.provenance['built_in_tool'], pipeline.BUILTIN_GENERATOR)

    def test_source_and_adaptation_keep_share_alike_attribution(self):
        self.assertEqual(self.portrait['license'], 'generated')  # Existing pipeline method marker.
        self.assertEqual(self.portrait['derivative_license'], 'CC BY-SA 4.0')
        self.assertEqual(self.portrait['identity_source']['license'], 'CC BY-SA 4.0')
        self.assertEqual(self.portrait['license_url'], 'https://creativecommons.org/licenses/by-sa/4.0/')
        self.assertEqual(self.provenance['rights']['credit'], self.portrait['credit'])
        self.assertIn('Comet Photo', self.portrait['credit'])
        self.assertIn('1985', self.portrait['era_note'])
        self.assertIn('no exact 1990 photograph established', self.portrait['era_note'])

    def test_incorrect_identity_or_file_hash_cannot_authorize_this_art(self):
        for key, value in [('sha256', '0' * 64), ('identity_source', {**self.portrait['identity_source'], 'person_id': 'salote_tupou_iii'})]:
            with self.subTest(field=key):
                changed = copy.deepcopy(self.scoped)
                changed['people'][self.person_id]['portraits'][0][key] = value
                result = pipeline.validate_manifest(changed, pipeline.ROOT, self.known)
                self.assertFalse(result['valid'])
                self.assertEqual(result['ready'], {})


if __name__ == '__main__':
    unittest.main()
