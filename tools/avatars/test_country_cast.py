"""Negative controls for the first bounded cast-to-game proposal."""
from copy import deepcopy
import tempfile
from pathlib import Path
import unittest

import check_country_cast as cast
import import_historical_people as importer


class TongaCastTests(unittest.TestCase):
    @classmethod
    def setUpClass(cls):
        cls.original = cast.read(cast.ROOT / cast.DEFAULT)
        cls.inputs = cast.load_inputs(snapshot=cast.Snapshot(cast.ROOT))

    def setUp(self):
        self.proposal = deepcopy(self.original)
        self.data = deepcopy(self.inputs)

    def check(self):
        return cast.check(self.proposal, **self.data)

    def rejects(self, message):
        with self.assertRaisesRegex(ValueError, message):
            self.check()

    def test_current_proposal_has_exact_bounded_scope(self):
        result = self.check()
        self.assertEqual(result['existing_opening_identity_references'], 3)
        self.assertEqual(result['reserved_uninstalled_person_ids'], 5)
        self.assertEqual(result['exact_research_holder_observations'], 10)
        self.assertEqual(result['held_alias_associations'], 2)
        self.assertEqual(result['existing_portraits_reused'], 1)
        self.assertEqual(result['runtime_records_changed'], 0)
        self.assertFalse(result['country_completed'])

    def test_check_does_not_mutate_any_inputs(self):
        before = deepcopy((self.proposal, self.data))
        self.check()
        self.assertEqual((self.proposal, self.data), before)

    def test_source_enrichment_preview_preserves_existing_facts_and_every_role(self):
        preview = self.check()['people_only_preview']
        registry, portraits, receipt = importer.merge(self.data['registry'], self.data['portraits'], preview)
        self.assertEqual(receipt['added_person_ids'], [])
        self.assertEqual(receipt['existing_fact_conflicts_preserved'], [])
        self.assertEqual(set(receipt['source_enriched_ids']), {'taufaahau_tupou_iv', 'fatafehi_tuipelehake'})
        self.assertEqual(registry['parties'], self.data['registry']['parties'])
        self.assertEqual(registry['office_links'], self.data['registry']['office_links'])
        for original, output in zip(self.data['registry']['people'], registry['people']):
            self.assertEqual({k: v for k, v in original.items() if k != 'sources'},
                             {k: v for k, v in output.items() if k != 'sources'})
        for pid, original in self.data['portraits']['people'].items():
            self.assertEqual(portraits['people'][pid]['portraits'], original['portraits'])

    def test_idempotent_source_preview(self):
        preview = self.check()['people_only_preview']
        r1, p1, _ = importer.merge(self.data['registry'], self.data['portraits'], preview)
        r2, p2, _ = importer.merge(r1, p1, preview)
        self.assertEqual((r1, p1), (r2, p2))

    def test_invented_precise_opening_pm_start_is_rejected(self):
        self.proposal['people'][1]['observations'][0]['holder']['from'] = '1990-01-01'
        self.rejects('observation differs')

    def test_year_window_cannot_be_silently_widened(self):
        self.proposal['people'][3]['observations'][0]['holder']['attested_period']['through'] = '1999-12-31'
        self.rejects('observation differs')

    def test_acting_sevele_cannot_become_substantive_start(self):
        self.proposal['people'][5]['observations'][0]['holder']['from'] = '2006-02-11'
        self.rejects('observation differs')

    def test_pohiva_two_observations_cannot_be_joined(self):
        self.proposal['people'][7]['observations'][0]['holder']['until'] = '2018-01-02'
        self.rejects('observation differs')

    def test_death_by_date_cannot_be_used_as_exact_end(self):
        self.proposal['people'][7]['observations'][1]['holder']['until'] = '2019-09-12'
        self.rejects('observation differs')

    def test_aliases_cannot_be_promoted_to_existing_identity_or_new_identity(self):
        for i, binding in [(2, 'existing_identity_proposal'), (4, 'new_identity_proposal')]:
            with self.subTest(person=i):
                self.proposal = deepcopy(self.original)
                self.proposal['people'][i]['observations'][-1]['binding'] = binding
                self.rejects('unsafe existing binding|holder name needs a separate alias review')

    def test_alias_needs_a_concrete_hold_reason(self):
        self.proposal['people'][2]['observations'][0]['hold_reason'] = ''
        self.rejects('alias requires an explicit hold')

    def test_inherited_title_cannot_be_bound_to_a_different_named_holder(self):
        self.proposal['people'][3]['name'] = 'Lord Vaea'
        self.rejects('holder name needs a separate alias review')

    def test_existing_saved_name_cannot_be_rewritten(self):
        self.proposal['people'][2]['identity']['record']['name'] = 'George Tupou V'
        self.rejects('existing display name differs|existing facts differ')

    def test_primary_link_cannot_be_changed_to_secondary_pm(self):
        link = next(x for x in self.data['registry']['office_links'] if x['nation'] == 'Tonga')
        link['person'] = 'fatafehi_tuipelehake'
        self.rejects('primary office link differs')

    def test_secondary_pm_cannot_be_given_primary_slot(self):
        self.proposal['people'][1]['opening_slot'] = 'primary'
        self.rejects('primary office link differs')

    def test_old_opening_record_is_not_silently_rebased(self):
        cast.opening_for(self.data['opening'], 'Tonga')['since'] = '1990-01-01'
        self.rejects('Opening record changed')

    def test_later_person_cannot_be_implicitly_imported(self):
        self.data['registry']['people'].append({'id': 'to_feleti_sevele', 'name': 'Feleti Sevele'})
        self.rejects('already installed')

    def test_later_person_cannot_duplicate_an_existing_name_under_new_id(self):
        self.data['registry']['people'].append({'id': 'another_sevele_id', 'name': 'Feleti Sevele'})
        self.rejects('named identity already exists')

    def test_held_heir_alias_cannot_enter_source_enrichment_payload(self):
        self.proposal['people'][2]['people_only_preview'] = True
        self.rejects('existing direct identity')

    def test_source_pin_detects_changed_claim_text_or_url(self):
        for field in ('text', 'url'):
            with self.subTest(field=field):
                self.data = deepcopy(self.inputs)
                source = next(s for s in self.data['research']['sources']
                              if s['id'] == 'to_pmo_20060410_marist_remarks')
                if field == 'url':
                    source['url'] += '?different-original'
                else:
                    source['claims'][0]['text'] = 'Changed claim'
                self.rejects('source or claim pin differs')

    def test_alias_bridge_text_is_pinned(self):
        source = next(s for s in self.data['research']['sources']
                      if s['id'] == 'to_pmo_20060928_crown_prince')
        next(c for c in source['claims'] if c['id'] == 'to_crown_crown_prince_tupouto_a_lavaka_2006')['text'] = 'New bridge'
        self.rejects('alias bridge claim changed')

    def test_additive_unrelated_research_does_not_force_reapproval(self):
        self.data['research']['sources'].append({'id': 'future_additive_packet', 'url': 'https://example.org',
            'claims': [{'id': 'unrelated_party_fact', 'text': 'Not part of this cast proposal'}]})
        self.assertEqual(self.check()['status'], 'valid_proposal_only')

    def test_accepted_art_window_cannot_be_widened(self):
        self.proposal['people'][0]['art']['to'] = '2000-01-01'
        self.rejects('existing portrait interval widened')

    def test_missing_art_cannot_be_marked_as_reuse(self):
        self.proposal['people'][1]['art']['status'] = 'reuse_existing'
        self.rejects('existing portrait differs')

    def test_uninstalled_identity_cannot_be_marked_ready_for_art(self):
        self.proposal['people'][5]['art']['disposition'] = 'ready_for_dated_reference'
        self.rejects('misleading artwork readiness')

    def test_cutoff_and_future_art_are_not_extended(self):
        self.proposal['historical_cutoff'] = '2035-12-31'
        self.rejects('Historical cutoff changed')
        self.proposal = deepcopy(self.original)
        self.proposal['people'][5]['art']['to'] = '2035-12-31'
        self.rejects('invalid historical art interval')

    def test_office_and_party_authority_cannot_be_claimed(self):
        for key in self.proposal['authority']:
            with self.subTest(authority=key):
                self.proposal = deepcopy(self.original)
                self.proposal['authority'][key] = True
                self.rejects('must grant no runtime')
        self.proposal = deepcopy(self.original)
        self.proposal['people'][7]['party_ids'] = ['to_dpfi']
        self.rejects('party mapping is outside this slice')

    def test_no_country_acceptance_from_proposal_check(self):
        self.proposal['status'] = 'accepted'
        self.rejects('cannot install or accept a country')

    def test_changed_review_receipt_is_rejected(self):
        self.proposal['reviews'][0]['sha256'] = '0' * 64
        self.rejects('Review receipt changed')

    def test_review_path_cannot_escape_repository(self):
        self.proposal['reviews'][0]['path'] = '../other-review.md'
        self.rejects('Review path escapes repository')

    def test_receipt_pins_survive_platform_newline_conversion(self):
        with tempfile.TemporaryDirectory() as temporary:
            root = Path(temporary)
            for review in self.proposal['reviews']:
                path = root / review['path']
                path.parent.mkdir(parents=True, exist_ok=True)
                text = (cast.ROOT / review['path']).read_text(encoding='utf-8')
                path.write_bytes(text.replace('\n', '\r\n').encode('utf-8'))
            cast.check(self.proposal, **self.data, root=root)
            for review in self.proposal['reviews']:
                path = root / review['path']
                path.write_bytes(path.read_bytes().replace(b'\r\n', b'\n'))
            cast.check(self.proposal, **self.data, root=root)


if __name__ == '__main__':
    unittest.main()
