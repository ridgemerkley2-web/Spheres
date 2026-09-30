"""Boundaries that prevent Japanese election lists becoming invented party histories."""
import copy
import json
import unittest

import campaign_research as research

# The seven sources of the original S10d intake keep every assertion below by id. CLAUDE-C01-12 adds 119 sources and
# 174 claims for the jp_prime_minister institution, whose response identities are pinned exactly (bytes and SHA-256) in
# test_japan_prime_ministers_c01_12.py; CLAUDE-C01-13 (stacked on it) adds 211 sources and 293 claims to the same
# institution and role, pinned in test_japan_prime_ministers_c01_13.py; CLAUDE-C01-18 (stacked on CLAUDE-C01-13) adds 81
# sources and 192 claims to the existing LDP presidency role, pinned in test_japan_ldp_presidents_c01_18.py; CLAUDE-C01-29
# adds 54 sources and 111 claims for a new party role, jp_sdp_chair, on the 社会民主党 observation, pinned in
# test_japan_sdp_chairs_c01_29.py; CLAUDE-C01-31 adds 31 sources and 64 claims for a new party role,
# jp_komeito_representative, on the 公明党 observation, pinned in test_japan_komeito_representatives_c01_31.py.
ORIGINAL_SOURCES = ('jp_tokyo_pr_2025', 'jp_shugiin_groups_20260218', 'jp_shugiin_group_definition',
                    'jp_ldp_ishiba_elected_2024', 'jp_ldp_takaichi_elected_2025', 'jp_jcp_chairs_2024',
                    'jp_dpfp_tamaki_elected_2026')
# The seven named lower-house groups of the original intake, which every group assertion below applies to by id.
GROUP_IDS = ['jp_shugiin_group_20260218_011', 'jp_shugiin_group_20260218_020', 'jp_shugiin_group_20260218_030',
             'jp_shugiin_group_20260218_040', 'jp_shugiin_group_20260218_050', 'jp_shugiin_group_20260218_060',
             'jp_shugiin_group_20260218_070']
C01_12_SOURCE_COUNT = 119
C01_12_CLAIM_COUNT = 174
C01_13_SOURCE_COUNT = 211
C01_13_CLAIM_COUNT = 293
C01_18_SOURCE_COUNT = 81
C01_18_CLAIM_COUNT = 192
C01_29_SOURCE_COUNT = 54
C01_29_CLAIM_COUNT = 111
C01_31_SOURCE_COUNT = 31
C01_31_CLAIM_COUNT = 64
# The LDP presidency's holder observations: CLAUDE-C01-18's fourteen (1990-2009), then the original 2024 and 2025 ones.
LDP_ATTESTED = ['1990-05-14', '1992-11-30', '1994-11-25', '1995-10-02', '1998-07-24', '1999-09-22', None, '2001-04-24',
                '2001-08-10', '2003-09-20', '2006-09-20', '2007-09-23', '2008-09-22', '2009-09-28', '2024-09-27', '2025-10-04']
# The SDP chair's holder observations (CLAUDE-C01-29): attested_on, None for the one holder dated by a stated start.
SDP_ATTESTED = ['1990-04-06', '1991-08-20', '1993-01-25', '1994-10-13', '1996-11-30', '1998-01-21', '2000-01-21', None,
                '2009-12-09', '2012-01-24', '2013-10-24', '2018-02-25', '2020-02-28', '2022-01-14', '2023-12-01', '2026-04-08']
# The Komeito representative's holder observations (CLAUDE-C01-31): attested_on, None for holders dated by a stated start.
KOMEITO_ATTESTED = ['1993-01-29', '1998-11-08', '2002-11-03', '2004-10-31', None, '2009-09-08', '2012-09-22',
                    '2014-09-21', '2016-09-17', '2018-09-30', '2020-09-27', '2022-09-25', '2024-09-28', '2024-11-09',
                    None]


class JapanDiscoveryTests(unittest.TestCase):
    @classmethod
    def setUpClass(cls):
        cls.packet = json.loads((research.ROOT / research.RESEARCH / 'japan.json').read_text(encoding='utf-8'))

    def validate(self, packet=None):
        return research.validate(packet or self.packet, research.ROOT, {'Japan'}, {'Japan': set()})

    def test_two_official_universes_remain_bounded_and_valid(self):
        ids = self.validate()
        # 23 original observations plus the CLAUDE-C01-12 prime-ministership (119 sources, 174 claims, one role), extended by
        # CLAUDE-C01-13 (211 sources, 293 claims) and by CLAUDE-C01-18's LDP presidents (81 sources, 192 claims on the existing
        # party role), with no new entry, by CLAUDE-C01-29's SDP chairs (54 sources, 111 claims, one new party role) and by
        # CLAUDE-C01-31's Komeito representatives (31 sources, 64 claims, one new party role).
        self.assertEqual((len(ids['entries']), len(ids['sources']), len(ids['claims']), len(ids['roles'])),
                         (24, 503, 863, 7))
        self.assertEqual((len(self.packet['organizations']), len(self.packet['institutions'])), (16, 8))
        self.assertEqual([s['id'] for s in self.packet['sources']][:7], list(ORIGINAL_SOURCES))
        self.assertEqual(sum(len(s['claims']) for s in self.packet['sources'] if s['id'] in ORIGINAL_SOURCES), 29)
        self.assertEqual(len(self.packet['sources']) - len(ORIGINAL_SOURCES),
                         C01_12_SOURCE_COUNT + C01_13_SOURCE_COUNT + C01_18_SOURCE_COUNT + C01_29_SOURCE_COUNT +
                         C01_31_SOURCE_COUNT)
        self.assertEqual(len(ids['claims']) - 29,
                         C01_12_CLAIM_COUNT + C01_13_CLAIM_COUNT + C01_18_CLAIM_COUNT + C01_29_CLAIM_COUNT + C01_31_CLAIM_COUNT)
        c01_18_start = 7 + C01_12_SOURCE_COUNT + C01_13_SOURCE_COUNT
        self.assertEqual(sum(len(s['claims']) for s in self.packet['sources'][c01_18_start:c01_18_start + C01_18_SOURCE_COUNT]),
                         C01_18_CLAIM_COUNT)
        c01_29_start = c01_18_start + C01_18_SOURCE_COUNT
        self.assertEqual(sum(len(s['claims']) for s in self.packet['sources'][c01_29_start:c01_29_start + C01_29_SOURCE_COUNT]),
                         C01_29_CLAIM_COUNT)
        self.assertEqual(sum(len(s['claims']) for s in self.packet['sources'][c01_29_start + C01_29_SOURCE_COUNT:]),
                         C01_31_CLAIM_COUNT)
        self.assertEqual(sum(len(s['claims']) for s in self.packet['sources'][7:7 + C01_12_SOURCE_COUNT]), C01_12_CLAIM_COUNT)
        # Exactly the seven groups and one executive institution, the prime-ministership, with exactly one role.
        self.assertEqual([e['id'] for e in self.packet['institutions']], GROUP_IDS + ['jp_prime_minister'])
        self.assertEqual([r['id'] for r in self.packet['institutions'][-1]['roles']], ['jp_pm'])
        coverage = self.packet['coverage']
        self.assertFalse(coverage['exhaustive_organization_register_reviewed'])
        self.assertIsNone(coverage['unrepresented_organization_total'])
        self.assertFalse(coverage['art_completion_claim'])
        self.assertEqual([(r['kind'], r['records']) for r in coverage['bounded_registers']], [
            ('proportional_election_list_submissions', 16), ('named_lower_house_parliamentary_groups', 7)])

    def test_independent_named_list_is_not_confused_with_independent_member_total(self):
        orgs = self.packet['organizations']
        self.assertEqual({int(o['source_identifier']['value']) for o in orgs}, set(range(1, 17)))
        self.assertTrue(all(o['source_identifier']['kind'] == 'election_list_submission_number' for o in orgs))
        self.assertEqual([o['name'] for o in orgs if o['source_identifier']['value'] == '3'], ['無所属連合'])
        self.assertEqual([o['name'] for o in orgs if o['source_identifier']['value'] == '16'], ['ＮＨＫ党'])
        self.assertNotIn('無所属', {o['name'] for o in orgs + self.packet['institutions']})
        self.assertTrue(all(o['kind'] == 'electoral_list_political_party_or_other_political_organization' for o in orgs))

    def test_same_name_in_two_lists_never_silently_merges_group_and_party(self):
        jcp = [e for e in self.packet['organizations'] + self.packet['institutions'] if e['name'] == '日本共産党']
        self.assertEqual(len(jcp), 2)
        self.assertEqual(len({e['id'] for e in jcp}), 2)
        groups = [e for e in self.packet['institutions'] if e['id'] in GROUP_IDS]
        self.assertEqual([g['id'] for g in groups], GROUP_IDS)
        self.assertTrue(all(g['kind'] == 'parliamentary_group' and not g['constituent_organization_ids'] for g in groups))
        self.assertEqual([e['kind'] for e in self.packet['institutions'] if e['id'] not in GROUP_IDS], ['executive_institution'])
        self.assertTrue(all(not g['roles'] for g in groups))
        self.assertTrue(all('jp_house_group_not_party' in g['claim_ids'] for g in groups))

    def test_party_chair_titles_and_party_presidency_do_not_grant_executive_office(self):
        roles = {r['id']: r for o in self.packet['organizations'] for r in o['roles']}
        chairs = [roles['jp_jcp_executive_committee_chair'], roles['jp_jcp_central_committee_chair']]
        self.assertEqual([r['title'].split(' — ')[0] for r in chairs], ['幹部会委員長', '中央委員会議長'])
        self.assertEqual([r['holder_claims'][0]['name'] for r in chairs], ['田村智子', '志位和夫'])
        self.assertEqual({r['kind'] for r in roles.values()}, {'party_chair', 'party_leader'})
        holders = [h for r in roles.values() for h in r['holder_claims']]
        self.assertEqual(len(holders), 50)
        # The one party holder with an end is 福島瑞穂's 2012 observation, ended by her own words on 25 July 2013
        # (CLAUDE-C01-29); the stated starts are 福島瑞穂's own statement of 15 November 2003 (CLAUDE-C01-29), 森喜朗's
        # own statement of 5 April 2000 (CLAUDE-C01-18), and 太田昭宏's of 30 September 2006 and 竹谷とし子's of 14 March
        # 2026 (CLAUDE-C01-31). No other party holder has a start or an end.
        self.assertEqual([(h['name'], h['until']) for h in holders if h['until']], [('福島瑞穂', '2013-07-25')])
        self.assertEqual([(h['name'], h['from']) for h in holders if h['from']],
                         [('福島瑞穂', '2003-11-15'), ('森喜朗', '2000-04-05'), ('太田昭宏', '2006-09-30'), ('竹谷とし子', '2026-03-14')])
        self.assertEqual([h['attested_on'] for h in roles['jp_ldp_party_president']['holder_claims']], LDP_ATTESTED)
        self.assertEqual([h['attested_on'] for h in roles['jp_sdp_chair']['holder_claims']], SDP_ATTESTED)
        self.assertEqual([h['attested_on'] for h in roles['jp_komeito_representative']['holder_claims']], KOMEITO_ATTESTED)

    def test_postcutoff_holder_cannot_enter_through_an_access_date(self):
        p = copy.deepcopy(self.packet)
        tamaki = next(o for o in p['organizations'] if o['name'] == '国民民主党')['roles'][0]['holder_claims'][0]
        self.assertEqual(tamaki['attested_on'], '2026-09-06')
        self.assertTrue(all(s['accessed_date'] == '2026-09-13' for s in p['sources'] if s['id'] in ORIGINAL_SOURCES))
        # CLAUDE-C01-12's sources were all accessed on 24 September 2026, CLAUDE-C01-13's on 24 or 25 September 2026 and
        # CLAUDE-C01-18's on 25 September 2026, CLAUDE-C01-29's on 28 September 2026 and CLAUDE-C01-31's on 30 September
        # 2026 (each pinned in its own test); no historical date comes from an access date.
        self.assertEqual({s['accessed_date'] for s in p['sources'][7:7 + C01_12_SOURCE_COUNT]}, {'2026-09-24'})
        self.assertEqual({s['accessed_date'] for s in p['sources'][7 + C01_12_SOURCE_COUNT:7 + C01_12_SOURCE_COUNT +
                                                                   C01_13_SOURCE_COUNT]}, {'2026-09-24', '2026-09-25'})
        c01_18_start = 7 + C01_12_SOURCE_COUNT + C01_13_SOURCE_COUNT
        self.assertEqual({s['accessed_date'] for s in p['sources'][c01_18_start:c01_18_start + C01_18_SOURCE_COUNT]},
                         {'2026-09-25'})
        c01_29_start = c01_18_start + C01_18_SOURCE_COUNT
        self.assertEqual({s['accessed_date'] for s in p['sources'][c01_29_start:c01_29_start + C01_29_SOURCE_COUNT]},
                         {'2026-09-28'})
        self.assertEqual({s['accessed_date'] for s in p['sources'][c01_29_start + C01_29_SOURCE_COUNT:]}, {'2026-09-30'})
        tamaki['attested_on'] = '2026-09-08'
        with self.assertRaisesRegex(ValueError, 'exceeds cutoff'):
            self.validate(p)

    def test_role_observations_do_not_fill_lifespans_or_game_mappings(self):
        for entry in self.packet['organizations'] + self.packet['institutions']:
            self.assertEqual(entry['represented_party_ids'], [])
            self.assertIsNone(entry['lifecycle']['from'])
            self.assertIsNone(entry['lifecycle']['until'])
        p = copy.deepcopy(self.packet)
        p['organizations'][0]['represented_party_ids'] = ['Japan/guessed_from_name']
        with self.assertRaisesRegex(ValueError, 'foreign represented party mapping'):
            self.validate(p)

    def test_offline_factual_extracts_match_each_source_and_detect_byte_change(self):
        snapshots = [s for s in self.packet['sources'] if 'snapshot' in s]
        # The two original extracts plus one derived extract per CLAUDE-C01-12, CLAUDE-C01-13, CLAUDE-C01-18, CLAUDE-C01-29
        # and CLAUDE-C01-31 source.
        self.assertEqual(len(snapshots), 2 + C01_12_SOURCE_COUNT + C01_13_SOURCE_COUNT + C01_18_SOURCE_COUNT +
                         C01_29_SOURCE_COUNT + C01_31_SOURCE_COUNT)
        self.assertEqual([s['id'] for s in snapshots][:2], ['jp_tokyo_pr_2025', 'jp_shugiin_groups_20260218'])
        for source in snapshots:
            data = json.loads((research.ROOT / source['snapshot']['path']).read_text(encoding='utf-8'))
            self.assertEqual(data['format'], 'spheres-c01-derived-factual-table/v1')
            self.assertEqual(data['source_url'], source['url'])
            self.assertEqual(len(data['rows']), len(source['claims']))
            self.assertRegex(data['source_response_sha256'], r'^[0-9a-f]{64}$')
        p = copy.deepcopy(self.packet)
        p['sources'][0]['snapshot']['bytes'] += 1
        with self.assertRaisesRegex(ValueError, 'checksum mismatch'):
            self.validate(p)

    def test_discovery_work_remains_separate_from_campaign_certification(self):
        index = research.build()
        japan = next(p for p in index['countries'] if p['nation'] == 'Japan')
        self.assertFalse(japan['country_census_complete'])
        # 23 original observations and the CLAUDE-C01-12 prime-ministership, none mapped to a game identity.
        self.assertEqual(japan['mapping_pending'], 24)
        self.assertEqual(self.packet['institutions'][-1]['represented_party_ids'], [])
        self.assertFalse(index['runtime_roster_modified'])
        self.assertFalse(index['c01_complete'])
        self.assertFalse(index['g2_prerequisite_satisfied'])
        work = [w for w in index['work_orders'] if w['nation'] == 'Japan']
        self.assertEqual([len(w['members']) for w in work], [10, 10, 4])
        self.assertEqual(work[-1]['members'], ['jp_shugiin_group_20260218_050', 'jp_shugiin_group_20260218_060',
                                               'jp_shugiin_group_20260218_070', 'jp_prime_minister'])
        self.assertEqual({m for w in work for m in w['members']}, set(self.validate()['entries']))


if __name__ == '__main__':
    unittest.main()
