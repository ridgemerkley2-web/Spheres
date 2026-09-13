"""Checks for the 2024 financial-obligation join, not historical census closure."""
import copy
import importlib.util
import json
import re
import unicodedata
import unittest

import import_cnccfp_census as importer


class CnccfpImportTests(unittest.TestCase):
    @classmethod
    def setUpClass(cls):
        cls.csv = (importer.ROOT / importer.RAW).read_bytes()
        cls.table = (importer.ROOT / importer.TABLE_RAW).read_bytes()
        cls.snapshots = {
            name: (importer.ROOT / importer.SOURCES_DIR / name).read_bytes()
            for _, name, *_ in importer.SNAPSHOTS}
        cls.parsed = importer.parse_publication(cls.table)
        cls.packet = importer.build(cls.csv)
        cls.organizations = {o['external_ids']['CNCCFP']: o
                             for o in cls.packet['organizations']}

    def altered_table(self, mutate):
        data = json.loads(self.table.decode('utf-8-sig'))
        mutate(data)
        return json.dumps(data, ensure_ascii=False).encode('utf-8')

    def test_full_universe_is_575_submitted_plus_60_explicit_non_filings(self):
        organizations = self.packet['organizations']
        self.assertEqual(len(organizations), 635)
        self.assertEqual(len({o['id'] for o in organizations}), 635)
        submitted = {c for c, o in self.organizations.items()
                     if o['financial_reporting_observation']['in_submitted_csv']}
        absent = {c for c, o in self.organizations.items()
                  if o['financial_reporting_observation']['motif_as_published'] == 'AD'}
        self.assertEqual(len(submitted), 575)
        self.assertEqual(absent, set(importer.AD_PDF_PAGES))
        self.assertEqual(set(self.organizations), submitted | absent)
        self.assertFalse(submitted & absent)
        self.assertEqual(self.organizations['5']['name'], "L'ALLIANCE RÉGIONALE")
        self.assertEqual(self.organizations['1550']['name'], 'MARTINIQUE-ÉCOLOGIE')

    def test_existing_names_ids_and_claims_survive_discrepancies(self):
        original = importer._build_submitted(self.csv)
        for old in original['organizations']:
            new = self.organizations[old['external_ids']['CNCCFP']]
            self.assertEqual(new['id'], old['id'])
            self.assertEqual(new['name'], old['name'])
            self.assertTrue(set(old['claim_ids']) <= set(new['claim_ids']))
        names = self.organizations['1096']['name_observations']
        self.assertEqual([n['name'] for n in names],
                         ["PRENDRE UN TEMPS D'AVANCE", 'COEUR LYONNAIS'])
        self.assertEqual(len(self.packet['coverage']['financial_reporting_universe']
                             ['differing_name_observation_codes']), 12)
        # Preserve the CSV's actual control-code spelling as source evidence.
        self.assertIn('\x8c', self.organizations['1263']['name'])
        self.assertIn('Œ', self.organizations['1263']['name_observations'][1]['name'])

    def test_non_filing_never_becomes_a_lifespan_or_current_game_binding(self):
        for organization in self.packet['organizations']:
            self.assertEqual(organization['lifecycle']['status'], 'unknown')
            self.assertEqual(organization['roles'], [])
            self.assertEqual(organization['represented_party_ids'], [])
            self.assertEqual(organization['representation_status'], 'unreconciled')
            self.assertNotIn('from', organization['lifecycle'])
            self.assertNotIn('until', organization['lifecycle'])
        coverage = self.packet['coverage']
        self.assertEqual(coverage['status'], 'partial_primary_source_inventory')
        self.assertFalse(coverage['financial_reporting_universe']['country_census_complete'])
        self.assertTrue(coverage['unresolved'])

    def test_claim_locators_preserve_raw_row_order_and_source_membership(self):
        source_claims = {s['id']: {c['id']: c for c in s['claims']}
                         for s in self.packet['sources']}
        for code, organization in self.organizations.items():
            record = self.parsed[code]
            claim = source_claims[importer.TABLE_SOURCE][f'fr_cnccfp_{code}_publication_2024']
            self.assertEqual(claim['locator']['json_pointer'], f"/rows/{record['index']}")
            self.assertEqual(claim['locator']['CNCCFP_number'], code)
            for claim_id in organization['claim_ids']:
                owners = [s for s in organization['sources'] if claim_id in source_claims[s]]
                self.assertEqual(len(owners), 1)
        # Number 1550 is last in the raw publication, not its sorted import position.
        self.assertEqual(self.parsed['1550']['index'], 634)

    def test_all_source_bytes_are_pinned_not_just_csv_or_table(self):
        with self.assertRaisesRegex(ValueError, 'checksum mismatch'):
            importer.build(self.csv + b'\n')
        with self.assertRaisesRegex(ValueError, 'checksum mismatch'):
            importer.build(self.csv, publication=self.table + b'\n')
        for filename in self.snapshots:
            with self.subTest(filename=filename):
                snapshots = dict(self.snapshots)
                snapshots[filename] += b'\n'
                with self.assertRaisesRegex(ValueError, 'checksum mismatch'):
                    importer.build(self.csv, snapshot_bytes=snapshots)

    def test_duplicate_non_numeric_boolean_and_zero_identifiers_refused(self):
        def duplicate(data):
            data['rows'][1][0] = data['rows'][0][0]
        with self.assertRaisesRegex(ValueError, 'Duplicate'):
            importer.parse_publication(self.altered_table(duplicate))
        for invalid in (True, 0, -1, '5'):
            def mutate(data):
                data['rows'][0][0] = invalid
            with self.subTest(invalid=invalid):
                with self.assertRaisesRegex(ValueError, 'positive integer'):
                    importer.parse_publication(self.altered_table(mutate))

    def test_wrong_columns_rows_and_unknown_motifs_refused(self):
        mutations = [
            (lambda d: d['headers'][0].__setitem__(6, 'Unreviewed'), 'columns'),
            (lambda d: d['rows'][0].pop(), 'row width'),
            (lambda d: d['rows'][0].__setitem__(7, 'UNKNOWN'), 'motif'),
            (lambda d: d['rows'][0].__setitem__(6, 'Respect'), 'Inconsistent'),
            (lambda d: d['rows'][0].__setitem__(2, '<a>unexpected filing</a>'), 'Absent-filing'),
            (lambda d: d['rows'].pop(), '635'),
        ]
        for mutate, expected in mutations:
            with self.subTest(expected=expected):
                with self.assertRaisesRegex(ValueError, expected):
                    importer.parse_publication(self.altered_table(mutate))

    def test_same_name_does_not_merge_different_official_numbers(self):
        def mutate(data):
            data['rows'][1][1] = data['rows'][0][1]
        parsed = importer.parse_publication(self.altered_table(mutate))
        self.assertEqual(len(parsed), 635)
        self.assertEqual(parsed['5']['row'][1], parsed['13']['row'][1])
        self.assertNotEqual(parsed['5']['row'][0], parsed['13']['row'][0])

    def test_same_count_with_wrong_absence_or_csv_set_is_refused(self):
        submitted = {o['external_ids']['CNCCFP']: o
                     for o in importer._build_submitted(self.csv)['organizations']}
        bad = copy.deepcopy(self.parsed)
        bad['5']['row'][6:] = ['Respect', 'DC']
        bad['13']['row'][2:6] = [None] * 4
        bad['13']['row'][6:] = ['Non-respect', 'AD']
        with self.assertRaisesRegex(ValueError, 'official AD set'):
            importer.reconcile_publication(submitted, bad)
        old = submitted.pop('13')
        submitted['9999999'] = old
        with self.assertRaisesRegex(ValueError, 'publication subset'):
            importer.reconcile_publication(submitted, self.parsed)

    @unittest.skipUnless(importlib.util.find_spec('pypdf'), 'Optional PDF extraction audit needs pypdf')
    def test_all_sixty_pdf_absence_rows_match_the_numbered_table_and_locators(self):
        from pypdf import PdfReader
        reader = PdfReader(importer.ROOT / importer.SOURCES_DIR / 'cnccfp-obligations-2024.pdf')

        def normalize(name):
            # Typography only for PDF corroboration; never used for code assignment.
            return ''.join(c for c in unicodedata.normalize('NFKD', name).casefold()
                           if c.isalnum())
        absent = {normalize(record['row'][1]): code
                  for code, record in self.parsed.items() if record['row'][7] == 'AD'}
        self.assertEqual(len(absent), 60)
        seen = {}
        for page_index in range(24, 45):
            text = reader.pages[page_index].extract_text(extraction_mode='layout')
            for paragraph in re.split(r'\n\s*\n', text):
                if not re.search(r'\bAD\s+AD\b', paragraph):
                    continue
                row = re.search(
                    r'^(.*?)\s+(?:Oui|Non)\s+(?:Oui|Non)\s+(\d{4,5})\s+Non\s+-?respect\s+AD\s+AD\b',
                    paragraph.strip(), re.S)
                self.assertIsNotNone(row)
                # The annex prints 6400 for one postal code; it is not a party ID.
                code = absent.pop(normalize(row[1]))
                self.assertNotIn(code, seen)
                seen[code] = page_index + 1
        self.assertEqual(absent, {})
        self.assertEqual(seen, importer.AD_PDF_PAGES)


if __name__ == '__main__':
    unittest.main()
