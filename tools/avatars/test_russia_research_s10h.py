"""Keep Russia's dated ballot and faction observations separate from histories."""
import copy
import hashlib
import json
import unittest
from urllib.parse import urlsplit

import campaign_research as research


class RussiaDiscoveryTests(unittest.TestCase):
    @classmethod
    def setUpClass(cls):
        cls.packet = json.loads((research.ROOT / research.RESEARCH / 'russia.json').read_text(encoding='utf-8'))
        cls.sources = {source['id']: source for source in cls.packet['sources']}
        cls.extracts = {
            source['id']: json.loads((research.ROOT / source['snapshot']['path']).read_text(encoding='utf-8'))
            for source in cls.packet['sources']
        }

    def validate(self, packet=None):
        return research.validate(packet or self.packet, research.ROOT, {'Russia'}, {'Russia': set()})

    def factions(self):
        # CLAUDE-C01-05 added exactly one non-faction institution, the RSFSR presidency, pinned in
        # test_ussr_russia_transition_c01_05, and CLAUDE-C01-19 exactly one more, the Government, pinned in
        # test_russia_heads_of_government_c01_19. Faction checks keep their full strength on the five factions.
        rows = [e for e in self.packet['institutions'] if e['kind'] == 'parliamentary_faction']
        self.assertEqual(len(rows), 5)
        self.assertEqual([e['id'] for e in self.packet['institutions'] if e['kind'] != 'parliamentary_faction'],
                         ['ru_rsfsr_presidency', 'ru_government'])
        return rows

    def test_partial_intake_has_fourteen_lists_and_five_distinct_institutions(self):
        ids = self.validate()
        # CLAUDE-C01-05 added 13 sources, 24 claims, one institution and two roles.
        # CLAUDE-C01-14 added 53 sources, 94 claims and one role (ru_president) to that institution; no entry.
        # CLAUDE-C01-19 added 102 sources, 153 claims (two of them on a C01-14 source), one institution and one role.
        # CLAUDE-C01-46 added 7 sources and 27 claims on the five existing faction-head roles; no entry and no role.
        # CLAUDE-C01-51 added 16 sources and 20 claims on the three ru_rsfsr_presidency roles; no entry and no role.
        self.assertEqual(tuple(len(ids[key]) for key in ('entries', 'sources', 'claims', 'roles')), (24, 261, 479, 14))
        self.assertEqual(len(self.packet['organizations']), 17)
        self.assertEqual(len(self.packet['institutions']), 7)
        # C01-28 appends three distinct, unmapped organization observations and five party roles.
        self.assertEqual([e['kind'] for e in self.packet['organizations']], ['federal_election_ballot_party_list'] * 14 + [
            'political_party_self_record_observation', 'electoral_association_self_record_observation',
            'political_party_self_record_observation'])
        self.assertEqual({e['kind'] for e in self.packet['institutions']},
                         {'parliamentary_faction', 'executive_presidency_office', 'executive_institution'})
        self.assertEqual(len(self.factions()), 5)
        coverage = self.packet['coverage']
        self.assertFalse(coverage['exhaustive_organization_register_reviewed'])
        self.assertIsNone(coverage['unrepresented_organization_total'])
        self.assertFalse(coverage['art_completion_claim'])
        self.assertEqual([row['records'] for row in coverage['bounded_registers']], [14, 5])

    def test_complete_ballot_order_preserves_exact_printed_identity_labels(self):
        rows = self.extracts['ru_cec_ballot_order_20210816']['rows']
        self.assertEqual([row['ballot_position'] for row in rows], list(range(1, 15)))
        self.assertEqual([row['party_label'] for row in rows], [
            'Политическая партия «КОММУНИСТИЧЕСКАЯ ПАРТИЯ РОССИЙСКОЙ ФЕДЕРАЦИИ»',
            'Политическая партия «Российская экологическая партия «ЗЕЛЁНЫЕ»',
            'Политическая партия ЛДПР – Либерально-демократическая партия России',
            'Политическая партия «НОВЫЕ ЛЮДИ»',
            'Всероссийская политическая партия «ЕДИНАЯ РОССИЯ»',
            'Партия СПРАВЕДЛИВАЯ РОССИЯ – ЗА ПРАВДУ',
            'Политическая партия «Российская объединенная демократическая партия «ЯБЛОКО»',
            'Всероссийская политическая партия «ПАРТИЯ РОСТА»',
            'Политическая партия РОССИЙСКАЯ ПАРТИЯ СВОБОДЫ И СПРАВЕДЛИВОСТИ',
            'Политическая партия КОММУНИСТИЧЕСКАЯ ПАРТИЯ КОММУНИСТЫ РОССИИ',
            'Политическая партия «Гражданская Платформа»',
            'Политическая партия ЗЕЛЕНАЯ АЛЬТЕРНАТИВА',
            'ВСЕРОССИЙСКАЯ ПОЛИТИЧЕСКАЯ ПАРТИЯ «РОДИНА»',
            'ПАРТИЯ ПЕНСИОНЕРОВ'])
        entries = {e['id']: e for e in self.packet['organizations']}
        for row in rows:
            entry = entries[row['observation_id']]
            self.assertEqual(entry['name'], row['party_label'])
            self.assertEqual(entry['source_identifier']['value'], str(row['ballot_position']))
            self.assertEqual(entry['claim_ids'], [row['claim_id']])

    def test_resolution_observation_never_becomes_election_or_publication_day(self):
        source = self.sources['ru_cec_ballot_order_20210816']
        self.assertIsNone(source['published_date'])
        extract = self.extracts[source['id']]
        self.assertEqual(extract['document']['resolution'], '42/337-8')
        self.assertEqual(extract['document']['resolution_date'], '2021-08-16')
        self.assertEqual(extract['document']['named_election_date'], '2021-09-19')
        for claim in source['claims']:
            self.assertEqual(claim['attested_on'], '2021-08-16')
            self.assertEqual(claim['locator']['pdf_page_one_based'], 145)

    def test_faction_membership_is_not_silently_changed_into_party_election_seats(self):
        rows = self.extracts['ru_duma_factions_20211012']['rows']
        self.assertEqual([(r['faction_label'], r['members']) for r in rows], [
            ('Единая Россия', 324), ('КПРФ', 57), ('Справедливая Россия — За правду', 28),
            ('ЛДПР', 23), ('Новые люди', 15)])
        self.assertEqual(sum(row['members'] for row in rows), 447)
        for entry in self.factions():
            self.assertIsNone(entry['membership_observation']['party_election_seats'])
            self.assertEqual(entry['membership_observation']['attested_on'], '2021-10-12')
            self.assertEqual(entry['constituent_organization_ids'], [])

    def test_only_parliamentary_offices_are_attested_without_invented_starts(self):
        expected = {'Владимир Васильев', 'Геннадий Зюганов', 'Владимир Жириновский', 'Сергей Миронов', 'Алексей Нечаев'}
        # CLAUDE-C01-46 appends dated observations after each original holder, pinned exactly here and in
        # test_russia_duma_faction_heads_c01_46: no start anywhere; one end, Жириновский's death on 6 April 2022.
        vas, zyu, zhi, mir, nec, slu = ('Владимир Васильев', 'Геннадий Зюганов', 'Владимир Жириновский', 'Сергей Миронов',
                                        'Алексей Нечаев', 'Леонид Слуцкий')
        days = ('2021-12-22', '2022-04-06', '2022-07-07', '2026-07-27')
        c01_46 = {
            'ru_duma_faction_20211012_er_head': [(vas, day, None, None) for day in days],
            'ru_duma_faction_20211012_kprf_head': [(zyu, day, None, None) for day in days],
            'ru_duma_faction_20211012_srzp_head': [(mir, day, None, None) for day in days[1:]],
            'ru_duma_faction_20211012_ldpr_head': [(zhi, '2021-12-22', None, '2022-04-06'), (slu, '2022-07-07', None, None),
                                                   (slu, '2026-07-27', None, None)],
            'ru_duma_faction_20211012_nl_head': [(nec, day, None, None) for day in days],
        }
        actual = set()
        for entry in self.factions():
            self.assertEqual(len(entry['roles']), 1)
            role = entry['roles'][0]
            self.assertEqual(role['kind'], 'parliamentary_leader')
            self.assertEqual([(h['name'], h['attested_on'], h['from'], h['until']) for h in role['holder_claims'][1:]],
                             c01_46[role['id']])
            for later in role['holder_claims'][1:]:
                self.assertEqual(len(later['name'].split()), 2, 'Do not import dynamic hover biography names')
            holder = role['holder_claims'][0]
            actual.add(holder['name'])
            self.assertEqual(holder['attested_on'], '2021-10-12')
            self.assertIsNone(holder['from'])
            self.assertIsNone(holder['until'])
            self.assertEqual(len(holder['name'].split()), 2, 'Do not import dynamic hover biography names')
        self.assertEqual(actual, expected)
        # Exactly five party roles; the complete faction assertions above remain separate.
        self.assertEqual({e['id']: [(r['id'], r['kind']) for r in e['roles']] for e in self.packet['organizations'] if e['roles']}, {
            'ru_duma_ballot_list_2021_01': [('ru_kprf_chairman', 'party_leader')],
            'ru_duma_ballot_list_2021_03': [('ru_ldpr_chairman', 'party_leader')],
            'ru_duma_ballot_list_2021_07': [('ru_yabloko_chairman', 'party_leader')],
            'ru_apr_party_self_record': [('ru_apr_chairman', 'party_leader')],
            'ru_dvr_party_self_record': [('ru_dvr_chairman', 'party_leader')]})

    def test_downloaded_pdf_hash_is_distinct_from_the_derived_factual_extract(self):
        source = self.sources['ru_cec_ballot_order_20210816']
        extract = self.extracts[source['id']]
        self.assertEqual(extract['source_response_bytes'], 3838526)
        self.assertEqual(extract['source_response_sha256'], 'b6f39b95e37f16586826ac7fe1a4c06ff552ad00ed55d2ad94a538ba852fa5e3')
        self.assertFalse(extract['source_response_checked_in'])
        self.assertEqual(extract['visual_review']['pdf_pages_one_based'], [1, 144, 145])
        path = research.ROOT / source['snapshot']['path']
        self.assertEqual(hashlib.sha256(path.read_bytes()).hexdigest(), source['snapshot']['sha256'])
        self.assertNotEqual(source['snapshot']['sha256'], extract['source_response_sha256'])
        packet = copy.deepcopy(self.packet)
        packet['sources'][0]['snapshot']['sha256'] = '0' * 64
        with self.assertRaisesRegex(ValueError, 'checksum mismatch'):
            self.validate(packet)

    def test_indexed_duma_text_never_claims_a_downloaded_original_response(self):
        source = self.sources['ru_duma_factions_20211012']
        extract = self.extracts[source['id']]
        self.assertEqual(source['access_method'], 'official_dated_article_text_via_search_index')
        self.assertEqual(extract['access_method'], source['access_method'])
        self.assertIsNone(extract['source_response_bytes'])
        self.assertIsNone(extract['source_response_sha256'])
        self.assertFalse(extract['source_response_checked_in'])
        self.assertIn('timed out', extract['provenance_note'])
        self.assertIn('Original response verification remains open', ' '.join(self.packet['coverage']['unresolved']))

    def test_sources_and_claims_reconcile_with_factual_snapshots(self):
        # CLAUDE-C01-05 added the legal portal, Rosarkhiv and Presidential Library sources.
        # CLAUDE-C01-14 added Internet Archive raw captures, the State Duma transcripts server and the official
        # publication section of the legal portal.
        self.assertEqual({urlsplit(s['url']).hostname for s in self.packet['sources']},
                         {'www.rcoit.ru', 'duma.gov.ru', 'pravo.gov.ru', 'projects.rusarchives.ru', 'www.prlib.ru',
                          'web.archive.org', 'transcript.duma.gov.ru', 'publication.pravo.gov.ru'})
        # Access dates are pinned per packet: the two original sources, CLAUDE-C01-05's 13, CLAUDE-C01-14's 53 and
        # CLAUDE-C01-19's 102 and CLAUDE-C01-46's 7 (both use only the hosts above; C01-46 accessed 1 October 2026 UTC),
        # and CLAUDE-C01-51's 16 (legal portal only; accessed 1 October 2026 UTC), then C01-28's 68 (28 September).
        original = {'ru_cec_ballot_order_20210816', 'ru_duma_factions_20211012'}
        c01_05 = {s['id'] for s in self.packet['sources'][2:15]}
        c01_14 = {s['id'] for s in self.packet['sources'][15:68]}
        c01_19 = {s['id'] for s in self.packet['sources'][68:170]}
        c01_46 = {s['id'] for s in self.packet['sources'][170:177]}
        c01_51 = {s['id'] for s in self.packet['sources'][177:193]}
        c01_28 = {s['id'] for s in self.packet['sources'][193:]}
        self.assertEqual([s['id'] for s in self.packet['sources'][:2]], sorted(original))
        self.assertEqual((len(c01_05), len(c01_14), len(c01_19), len(c01_46), len(c01_51), len(c01_28)), (13, 53, 102, 7, 16, 68))
        self.assertTrue(all(sid.startswith('ru_duma_news_') for sid in c01_46))
        self.assertTrue(all(urlsplit(s['url']).hostname == 'pravo.gov.ru' for s in self.packet['sources'][177:193]))
        self.assertTrue(all(sid.startswith('ru_rsfsr_') or sid.startswith('ru_garf_') or sid == 'ru_prlib_inauguration_stenogram_19910710'
                            for sid in c01_05))
        # Exact undated exceptions: constitutional text, one retrospective court statement and C01-28 party claims.
        undated = {'ru_ks_134o_rsfsr_president_retitled_19981105', 'ru_const1993_entry_into_force_rule',
                   'ru_const1993_transitional_president_rule', 'ru_const1993_art80_head_of_state',
                   'ru_const1993_oath_and_term_rules', 'ru_portal_const1993_publication_citation'}
        c01_28_undated = {'ru_kprf_retro_congress_renamed_party_1993', 'ru_kprf_retro_congress_elected_cec_1993',
                          'ru_kprf_retro_zyuganov_elected_cec_chairman_1993',
                          'ru_kprf_reference_zyuganov_since_february_1993',
                          'ru_kprf_reference_registered_since_ii_congress_1993',
                          'ru_ldpr_history2010_iii_congress_founds_ldpr_1992',
                          'ru_ldpr_history2010_iii_congress_chairman_1992',
                          'ru_ldpr_history2010_iv_congress_chairman_1993', 'ru_ldpr_news_xxxiv_congress_announced',
                          'ru_ldpr_newspaper_chairman_speech_xxxiv_congress',
                          'ru_ldpr_newspaper_death_referenced_undated',
                          'ru_yabloko_reference_bloc_lists_autumn_1993',
                          'ru_yabloko_reference_former_names_1993_1994',
                          'ru_yabloko_reference_founding_congress_19950105_19950106',
                          'ru_yabloko_reference_yavlinsky_chairman_elected_1995',
                          'ru_yabloko_reference_yavlinsky_chairman_since_january_1995',
                          'ru_yabloko_x_congress_dates_20011222_20011223', 'ru_yabloko_chairman_vote_scheduled_2004',
                          'ru_yabloko_yavlinsky_reelected_chairman_2004',
                          'ru_yabloko_party_named_successor_of_1995_association',
                          'ru_yabloko_report_2008_chairman_title_block', 'ru_yabloko_report_2008_xii_congress_2004',
                          'ru_yabloko_report_2008_xiii_congress_2006', 'ru_yabloko_report_2008_bureau_list_chairman',
                          'ru_yabloko_xv_congress_dates_20080621_20080622',
                          'ru_yabloko_xv_congress_mitrokhin_elected_2008',
                          'ru_yabloko_retro_yavlinsky_chairman_2001_2008',
                          'ru_yabloko_xviii_congress_dates_20151219_20151220',
                          'ru_yabloko_xxi_congress_dates_20191214_20191215',
                          'ru_yabloko_slabunova_report_as_chairman_2015_2019',
                          'ru_yabloko_xxi_congress_vote_about_one_am', 'ru_apr_history_chairman_heading_2002',
                          'ru_apr_history_lapshin_elected_founding_congress_1993',
                          'ru_apr_history_iii_congress_lapshin_reelected_1994',
                          'ru_apr_history_v_congress_lapshin_remained_1997',
                          'ru_apr_leadership_page_plotnikov_chairman_2004', 'ru_apr_chairman_page_heading_2008',
                          'ru_dvr_supporters_founding_congress_planned',
                          'ru_dvr_political_council_gaidar_chairman_1994',
                          'ru_dvr_statement_signed_by_chairman_199412',
                          'ru_dvr_ii_congress_statement_signed_by_chairman_1995',
                          'ru_dvr_history_created_on_basis_of_movement', 'ru_dvr_history_founded_and_named_1994',
                          'ru_dvr_about_page_chairman_gaidar_2001',
                          'ru_dvr_x_congress_self_dissolution_decision_2001',
                          'ru_dvr_x_congress_gaidar_speech_dissolution_2001'}
        self.assertEqual({c['id'] for s in self.packet['sources'] for c in s['claims'] if 'attested_on' not in c},
                         undated | c01_28_undated)
        self.assertTrue(all(cid.startswith(('ru_kprf_', 'ru_ldpr_', 'ru_yabloko_', 'ru_apr_', 'ru_dvr_')) for cid in c01_28_undated))
        for source in self.packet['sources']:
            extract = self.extracts[source['id']]
            self.assertEqual(extract['source_url'], source['url'])
            expected = ('2026-09-13' if source['id'] in original else '2026-09-21' if source['id'] in c01_05
                        else '2026-09-24' if source['id'] in c01_14 else '2026-10-01' if source['id'] in c01_46 | c01_51
                        else '2026-09-28' if source['id'] in c01_28 else '2026-09-25')
            self.assertEqual(source['id'] in c01_19, expected == '2026-09-25')
            self.assertEqual(source['id'] in c01_46 | c01_51, expected == '2026-10-01')
            self.assertEqual(source['id'] in c01_28, expected == '2026-09-28')
            self.assertEqual(source['accessed_date'], expected)
            self.assertEqual(extract['format'], 'spheres-c01-derived-factual-table/v1')
            if 'claims' in extract:
                self.assertEqual(extract['claims'], source['claims'])
            else:
                self.assertEqual({r['claim_id'] for r in extract['rows']}, {c['id'] for c in source['claims']})
            self.assertTrue(all(c['attested_on'] <= research.CUTOFF for c in source['claims'] if c['id'] not in undated | c01_28_undated))
        packet = copy.deepcopy(self.packet)
        packet['sources'][0]['claims'][0]['attested_on'] = '2026-09-08'
        with self.assertRaisesRegex(ValueError, 'exceeds cutoff'):
            self.validate(packet)

    def test_unknown_identities_do_not_merge_factions_parties_or_predecessor_states(self):
        for entry in self.packet['organizations'] + self.packet['institutions']:
            self.assertEqual(entry['represented_party_ids'], [])
            self.assertIsNone(entry['reconciled_organization_id'])
            self.assertIsNone(entry['lifecycle']['from'])
            self.assertIsNone(entry['lifecycle']['until'])
        self.assertIn('Russia and USSR', ' '.join(self.packet['coverage']['unresolved']))
        packet = copy.deepcopy(self.packet)
        packet['organizations'][0]['represented_party_ids'] = ['USSR/cpsu']
        with self.assertRaisesRegex(ValueError, 'foreign represented party mapping'):
            self.validate(packet)

    def test_intake_build_produces_only_open_discovery_batches(self):
        index = research.build()
        country = next(p for p in index['countries'] if p['nation'] == 'Russia')
        self.assertFalse(country['country_census_complete'])
        self.assertEqual(country['mapping_pending'], 24)  # C01-28 adds three unmapped organization observations
        self.assertEqual(country['role_observations'], 14)  # Nine state/faction roles plus exactly five C01-28 party roles
        work = [row for row in index['work_orders'] if row['nation'] == 'Russia']
        self.assertEqual([len(row['members']) for row in work], [10, 10, 4])
        self.assertEqual({member for row in work for member in row['members']}, set(self.validate()['entries']))
        self.assertEqual({row['status'] for row in work}, {'open'})
        self.assertFalse(index['runtime_roster_modified'])
        self.assertFalse(index['c01_complete'])
        self.assertFalse(index['g2_prerequisite_satisfied'])


if __name__ == '__main__':
    unittest.main()
