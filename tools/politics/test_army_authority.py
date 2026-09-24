import copy
import hashlib
import json
import pathlib
import unittest

from build_army_authority import build, EXTRACT_SHA

ROOT = pathlib.Path(__file__).resolve().parents[2]
SOURCE = ROOT / 'tools/politics/fixtures/vdem-v16-1989-military-authority.json'

class ArmyAuthoritySourceTests(unittest.TestCase):
    @classmethod
    def setUpClass(cls):
        cls.raw = json.loads(SOURCE.read_bytes())
        cls.nations = [json.loads(f.read_text(encoding='utf-8-sig'))
                       for f in (ROOT / 'spheres-sim/data/nations').glob('*.json')]

    def test_catalog_exactly_reproduces_pinned_source_and_complete_roster_accounting(self):
        self.assertEqual(hashlib.sha256(SOURCE.read_bytes()).hexdigest(), EXTRACT_SHA)
        actual = build(self.raw, self.nations, EXTRACT_SHA)
        stored = json.loads((ROOT / 'spheres-sim/data/army_authority_1989.json').read_text(encoding='utf-8'))
        self.assertEqual(actual, stored)
        sourced = {r['nation'] for r in actual['rows']}
        unknown = {r['nation'] for r in actual['not_imported']}
        self.assertFalse(sourced & unknown)
        self.assertEqual(sourced | unknown, {r['id'] for r in self.nations})
        self.assertEqual((len(sourced), len(unknown)), (130, 7))
        self.assertIn('Germany', unknown)
        self.assertIn('Yemen', unknown)

    def test_relevant_executive_and_dated_crosswalks_are_preserved(self):
        result = build(self.raw, self.nations, EXTRACT_SHA)
        rows = {r['nation']:r for r in result['rows']}
        self.assertEqual(rows['Thailand']['assessment'], 0.833)
        self.assertEqual(rows['Thailand']['hos_removal'], 0.143)
        self.assertEqual(rows['USA']['assessment'], 0.0)
        self.assertIsNone(rows['USA']['hog_removal'])
        for country, code in [('USSR','RUS'), ('Czechoslovakia','CZE'), ('Yugoslavia','SRB')]:
            self.assertEqual(rows[country]['source_key'], f'vdem-v16-1989-{code}')
        self.assertNotIn('Russia', rows)
        self.assertNotIn('Serbia', rows)

    def test_future_or_duplicate_source_rows_are_rejected(self):
        for raw in [self.raw + [self.raw[0]], [dict(self.raw[0], year=1990)]]:
            with self.assertRaises(ValueError):
                build(raw, self.nations, EXTRACT_SHA)

    def test_missing_required_power_is_not_silently_zero_or_renormalized(self):
        for field, value in [('v2exrmhgnp_4', None), ('v2ex_hogw', 0.6),
                             ('v2exrmhgnp_4', float('nan')), ('v2ex_hogw', -1),
                             ('v2exrmhgnp_4', 1.1)]:
            raw = copy.deepcopy(self.raw)
            row = next(r for r in raw if r['country_text_id'] == 'THA')
            row[field] = value
            with self.subTest(field=field, value=value), self.assertRaises(ValueError):
                build(raw, self.nations, EXTRACT_SHA)

    def test_combined_executive_cannot_be_counted_twice(self):
        raw = copy.deepcopy(self.raw)
        row = next(r for r in raw if r['country_text_id'] == 'USA')
        row.update(v2ex_hosw=0.5, v2ex_hogw=0.5, v2exrmhgnp_4=0.0)
        with self.assertRaisesRegex(ValueError, 'exactly once'):
            build(raw, self.nations, EXTRACT_SHA)

    def test_duplicate_roster_identity_cannot_create_extra_source_coverage(self):
        with self.assertRaisesRegex(ValueError, 'Duplicate game roster'):
            build(self.raw, self.nations + [self.nations[0]], EXTRACT_SHA)

    def test_ambiguous_source_name_cannot_silently_replace_a_country(self):
        rival = dict(next(r for r in self.raw if r['country_text_id'] == 'SUR'))
        rival.update(country_text_id='XXX', country_name='Suri-name', v2exrmhsol_4=1.0)
        with self.assertRaisesRegex(ValueError, 'Ambiguous normalized'):
            build(self.raw + [rival], self.nations, EXTRACT_SHA)

if __name__ == '__main__':
    unittest.main()
