"""Tests for tools/avatars/check_successor_proposals.py (CLAUDE-C04-PREP-01).

Each test mutates a deep copy of the committed France/Tonga packet and asserts the
validator reports the specific violation, so every check is shown to go red.
Run: python -X utf8 -m unittest discover -s tools/avatars -p "test_successor_proposals.py"
"""
import copy
import json
import os
import pathlib
import re
import subprocess
import sys
import tempfile
import unittest

HERE = pathlib.Path(__file__).resolve().parent
sys.path.insert(0, str(HERE))
import check_successor_proposals as csp  # noqa: E402


class ProposalPacketTests(unittest.TestCase):
    @classmethod
    def setUpClass(cls):
        cls.repo = csp.Repo(csp.ROOT)
        cls.doc, cls.sources = csp.load_packet(csp.ROOT, csp.PACKET)

    # ---------- helpers ----------
    def errors(self, doc=None, sources=None):
        return csp.validate(doc if doc is not None else self.doc,
                            sources if sources is not None else self.sources, self.repo)

    def assertFlags(self, code, doc=None, sources=None, contains=None):
        errors = self.errors(doc, sources)
        hits = [e for e in errors if e[0] == code and (contains is None or contains in e[2] or contains in e[1])]
        self.assertTrue(hits, f'expected {code}{" containing " + contains if contains else ""}, got {errors}')

    def doc_copy(self):
        return copy.deepcopy(self.doc)

    def prop(self, doc, draft_id):
        return next(p for p in doc['proposals'] if p['draft_id'] == draft_id)

    # ---------- the committed packet ----------
    def test_committed_packet_passes(self):
        self.assertEqual(self.errors(), [])

    def test_eight_labelled_drafts_four_per_nation(self):
        counts = {}
        for p in self.doc['proposals']:
            counts[p['nation']] = counts.get(p['nation'], 0) + 1
            self.assertRegex(p['draft_id'], r'^draft_c04_(fr|to)_\d{2}$')
            self.assertTrue(p['fictional'])
            self.assertIn('not a real person', p['fiction_label'])
            self.assertFalse(p['appearance_brief']['image_generated'])
            self.assertFalse(p['contract_mapping']['runtime_installed'])
        self.assertEqual(counts, {'France': 4, 'Tonga': 4})

    def test_roles_distinguish_leadership_executive_collective_and_exclude_hereditary(self):
        catalog = {r['role_id']: r for r in self.doc['role_catalog']}
        for nation in ('France', 'Tonga'):
            used = {catalog[p['role']['role_id']]['category'] for p in self.doc['proposals'] if p['nation'] == nation}
            self.assertEqual(used, {'party_leadership', 'executive_eligibility', 'collective_institution'}, nation)
        excluded = {r['role_id']: r for r in self.doc['excluded_roles']}
        for rid in ('to_king', 'to_royal_family_and_regency', 'to_noble_title_holder', 'to_nobles_representative', 'to_speaker'):
            self.assertEqual(excluded[rid]['category'], 'hereditary_office')
            self.assertFalse(excluded[rid]['fictional_proposals_allowed'])
        self.assertFalse(any(r['category'] == 'hereditary_office' for r in catalog.values()))

    def test_constants_match_runtime_contract(self):
        src = (csp.ROOT / 'spheres-sim/src/party_leadership_future.rs').read_text(encoding='utf-8')
        consts = dict(re.findall(r'pub const (\w+): &str = "([0-9-]+)";', src))
        self.assertEqual(consts['HISTORICAL_THROUGH'], csp.CUTOFF.isoformat())
        self.assertEqual(consts['FROM'], csp.FROM.isoformat())
        self.assertEqual(consts['UNTIL'], csp.UNTIL_EXCLUSIVE.isoformat())
        census = json.loads((csp.ROOT / 'docs/campaign-certification/C01/census.json').read_text(encoding='utf-8'))
        self.assertEqual((census['existing_research_cutoff'], census['fictional_from'], census['until_exclusive']),
                         (csp.CUTOFF.isoformat(), csp.FROM.isoformat(), csp.UNTIL_EXCLUSIVE.isoformat()))

    def test_validation_does_not_mutate_inputs(self):
        before = json.dumps([self.doc, self.sources], sort_keys=True)
        self.errors()
        self.assertEqual(before, json.dumps([self.doc, self.sources], sort_keys=True))

    # ---------- historical-window contamination ----------
    def test_window_starting_on_cutoff_is_rejected(self):
        doc = self.doc_copy()
        self.prop(doc, 'draft_c04_to_01')['eligibility_window']['earliest_from'] = '2026-09-07'
        self.assertFlags('E_HISTORICAL', doc, contains='before 2026-09-08')

    def test_any_dated_event_before_window_is_rejected(self):
        doc = self.doc_copy()
        self.prop(doc, 'draft_c04_fr_02')['milestones'] = [{'kind': 'joined', 'on': '2019-05-01'}]
        self.assertFlags('E_HISTORICAL', doc, contains='precedes the fictional window')

    def test_narrative_year_is_rejected(self):
        doc = self.doc_copy()
        bio = self.prop(doc, 'draft_c04_fr_01')['biography']
        bio['hypothetical_path_after_cutoff'] += ' She first spoke at a rally in 2018.'
        self.assertFlags('E_HISTORICAL', doc, contains='pre-window year')

    def test_pre_cutoff_office_in_background_is_rejected(self):
        doc = self.doc_copy()
        bio = self.prop(doc, 'draft_c04_fr_04')['biography']
        bio['background_before_cutoff'] += ' He was a municipal councillor for a decade.'
        self.assertFlags('E_HISTORICAL', doc, contains='councillor')

    def test_pre_cutoff_party_membership_is_rejected(self):
        doc = self.doc_copy()
        bio = self.prop(doc, 'draft_c04_to_04')['biography']
        bio['background_before_cutoff'] += ' He joined the party as a student.'
        self.assertFlags('E_HISTORICAL', doc, contains='party')

    def test_dated_background_is_rejected(self):
        doc = self.doc_copy()
        self.prop(doc, 'draft_c04_to_02')['biography']['background_undated'] = False
        self.assertFlags('E_HISTORICAL', doc, contains='undated')

    def test_window_past_2035_is_rejected(self):
        doc = self.doc_copy()
        self.prop(doc, 'draft_c04_to_03')['eligibility_window']['until_inclusive'] = '2036-01-01'
        self.assertFlags('E_WINDOW', doc)

    def test_inverted_window_is_rejected(self):
        doc = self.doc_copy()
        w = self.prop(doc, 'draft_c04_fr_02')['eligibility_window']
        w['earliest_from'], w['until_inclusive'] = '2035-12-31', '2030-01-01'
        self.assertFlags('E_WINDOW', doc, contains='starts after it ends')

    # ---------- missing fictional labels ----------
    def test_unlabelled_person_is_rejected(self):
        for mutate, field in ((lambda p: p.__setitem__('fictional', False), 'fictional'),
                              (lambda p: p.pop('fiction_label'), 'fiction_label'),
                              (lambda p: p['name'].pop('authored'), 'name.authored'),
                              (lambda p: p['biography'].__setitem__('label', 'BIOGRAPHY'), 'biography.label'),
                              (lambda p: p['appearance_brief'].__setitem__('label', 'Appearance'), 'appearance_brief.label')):
            with self.subTest(field=field):
                doc = self.doc_copy()
                mutate(self.prop(doc, 'draft_c04_fr_03'))
                self.assertFlags('E_LABEL', doc, contains=field)

    def test_label_without_not_a_real_person_is_rejected(self):
        doc = self.doc_copy()
        self.prop(doc, 'draft_c04_to_01')['fiction_label'] = 'Fictional draft.'
        self.assertFlags('E_LABEL', doc, contains='not a real person')

    # ---------- role / age violations ----------
    def test_hereditary_roles_cannot_be_proposed(self):
        for rid in ('to_king', 'to_noble_title_holder', 'to_nobles_representative', 'to_speaker'):
            with self.subTest(role=rid):
                doc = self.doc_copy()
                self.prop(doc, 'draft_c04_to_02')['role'] = {'role_id': rid, 'category': 'hereditary_office'}
                self.assertFlags('E_ROLE', doc, contains='excluded role')

    def test_catalog_role_marked_hereditary_is_rejected(self):
        doc = self.doc_copy()
        role = next(r for r in doc['role_catalog'] if r['role_id'] == 'to_prime_minister_via_peoples_seat')
        role['hereditary'] = True
        self.assertFlags('E_ROLE', doc, contains='hereditary')

    def test_role_from_other_nation_is_rejected(self):
        doc = self.doc_copy()
        self.prop(doc, 'draft_c04_to_01')['role'] = {'role_id': 'fr_national_assembly_deputy', 'category': 'collective_institution'}
        self.assertFlags('E_ROLE', doc, contains='differs from')

    def test_role_category_mismatch_is_rejected(self):
        doc = self.doc_copy()
        self.prop(doc, 'draft_c04_fr_01')['role']['category'] = 'executive_eligibility'
        self.assertFlags('E_ROLE', doc, contains='category')

    def test_unknown_role_is_rejected(self):
        doc = self.doc_copy()
        self.prop(doc, 'draft_c04_fr_01')['role'] = {'role_id': 'fr_king', 'category': 'party_leadership'}
        self.assertFlags('E_ROLE', doc, contains='unknown role')

    def test_under_age_candidate_is_rejected(self):
        doc = self.doc_copy()
        p = self.prop(doc, 'draft_c04_to_01')
        p['age']['birth_year'] = 2006  # may be 19 on 2026-09-08; people's seats need 21
        p['age']['youngest_age_at_window_start'] = 19
        p['age']['oldest_age_at_window_end'] = 29
        self.assertFlags('E_AGE', doc, contains='below the sourced minimum 21')

    def test_implausibly_old_candidate_is_rejected(self):
        doc = self.doc_copy()
        p = self.prop(doc, 'draft_c04_fr_03')
        p['age']['birth_year'] = 1940
        p['age']['youngest_age_at_window_start'] = 88
        p['age']['oldest_age_at_window_end'] = 95
        self.assertFlags('E_AGE', doc, contains='above plausibility maximum')

    def test_declared_ages_must_match_birth_year(self):
        doc = self.doc_copy()
        self.prop(doc, 'draft_c04_fr_02')['age']['birth_year'] = 1990
        self.assertFlags('E_AGE', doc, contains='declared ages differ')

    def test_excluded_party_row_is_rejected(self):
        doc = self.doc_copy()
        p = self.prop(doc, 'draft_c04_fr_04')
        p['party_affiliation']['game_party_row'] = 'fr_rpr'
        self.assertFlags('E_PARTY', doc, contains='excluded')

    def test_party_row_must_exist_for_nation(self):
        doc = self.doc_copy()
        self.prop(doc, 'draft_c04_to_04')['party_affiliation']['game_party_row'] = 'fr_ps'
        self.assertFlags('E_PARTY', doc, contains='not a current Tonga game party row')

    def test_party_row_must_match_role(self):
        doc = self.doc_copy()
        self.prop(doc, 'draft_c04_fr_01')['party_affiliation']['game_party_row'] = 'fr_pcf'
        self.assertFlags('E_PARTY', doc, contains='differs from the role catalog')

    # ---------- duplicate IDs and names ----------
    def test_duplicate_draft_id_is_rejected(self):
        doc = self.doc_copy()
        self.prop(doc, 'draft_c04_fr_02')['draft_id'] = 'draft_c04_fr_01'
        self.assertFlags('E_DUP_ID', doc)

    def test_duplicate_name_is_rejected(self):
        doc = self.doc_copy()
        a, b = self.prop(doc, 'draft_c04_to_01'), self.prop(doc, 'draft_c04_to_03')
        b['name'] = copy.deepcopy(a['name'])
        self.assertFlags('E_NAME_DUP', doc)

    # ---------- collisions with existing fictional IDs or real names ----------
    def test_existing_fictional_id_collides(self):
        doc = self.doc_copy()
        self.prop(doc, 'draft_c04_fr_01')['draft_id'] = 'fictional_v1_france_fr_ps_main_01'
        self.assertFlags('E_ID_COLLISION', doc)
        self.assertFlags('E_ID', doc)

    def test_historical_person_id_collides(self):
        self.assertIn('olivier_faure', self.repo.existing_ids)
        doc = self.doc_copy()
        self.prop(doc, 'draft_c04_fr_01')['draft_id'] = 'olivier_faure'
        self.assertFlags('E_ID_COLLISION', doc)

    def test_nation_prefix_must_match(self):
        doc = self.doc_copy()
        self.prop(doc, 'draft_c04_to_01')['draft_id'] = 'draft_c04_fr_09'
        self.assertFlags('E_ID', doc, contains='prefix')

    def set_name(self, doc, draft_id, given, family):
        self.prop(doc, draft_id)['name'].update({'display': f'{given} {family}', 'given': given, 'family': family})

    def test_real_census_name_is_rejected(self):
        doc = self.doc_copy()
        self.set_name(doc, 'draft_c04_fr_01', 'Olivier', 'Faure')
        self.assertFlags('E_NAME_REAL', doc, contains='matches a real person')

    def test_close_resemblance_to_real_name_is_rejected(self):
        doc = self.doc_copy()
        self.set_name(doc, 'draft_c04_fr_02', 'Fabian', 'Roussel')
        self.assertFlags('E_NAME_REAL', doc, contains='resembles')

    def test_real_research_holder_is_rejected(self):
        doc = self.doc_copy()
        self.set_name(doc, 'draft_c04_to_04', 'Fatai', 'Helu')
        self.assertFlags('E_NAME_REAL', doc)

    def test_surname_of_office_holder_named_only_in_research_text_is_rejected(self):
        doc = self.doc_copy()
        self.set_name(doc, 'draft_c04_to_02', 'Sitani', 'Sovaleni')
        self.assertFlags('E_NAME_REAL', doc, contains='family name')

    def test_existing_fictional_name_is_rejected(self):
        doc = self.doc_copy()
        self.set_name(doc, 'draft_c04_fr_01', 'Camille', 'Renaud')
        self.assertFlags('E_NAME_FICTIONAL', doc)

    def test_template_pool_combination_is_rejected(self):
        doc = self.doc_copy()
        self.set_name(doc, 'draft_c04_to_03', 'Malia', 'Moala')
        self.assertFlags('E_NAME_FICTIONAL', doc, contains='pool')

    def test_tongan_noble_or_royal_name_is_rejected(self):
        for given, family in (('Sitani', 'Fakafanua'), ('Pisila', "Tupouto'a"), ('Kalolo', 'Vaea'), ('Sitani', "Ma'afu")):
            with self.subTest(name=f'{given} {family}'):
                doc = self.doc_copy()
                self.set_name(doc, 'draft_c04_to_02', given, family)
                self.assertFlags('E_HEREDITARY_NAME', doc)

    def test_honorific_in_name_is_rejected(self):
        doc = self.doc_copy()
        self.set_name(doc, 'draft_c04_to_01', 'Lady', 'Fotu')
        self.assertFlags('E_HEREDITARY_NAME', doc, contains='honorific')

    def test_real_person_mentioned_in_narrative_is_rejected(self):
        doc = self.doc_copy()
        bio = self.prop(doc, 'draft_c04_fr_01')['biography']
        bio['hypothetical_path_after_cutoff'] += ' She would succeed Olivier Faure.'
        self.assertFlags('E_REAL_REFERENCE', doc)

    def test_real_historical_term_record_is_rejected(self):
        doc = self.doc_copy()
        self.prop(doc, 'draft_c04_fr_01')['contract_mapping']['historical_term'] = 'fr_ps_olivier_faure_20180407_leader'
        self.assertFlags('E_OFFICE_RECORD', doc)

    # ---------- date-triggered incumbent replacement ----------
    def test_succession_contract_must_forbid_date_replacement(self):
        for field, value in (('replaces_incumbent_on_date', True), ('date_triggered', True),
                             ('predicted_appointment', True), ('incumbents_retained', False),
                             ('automatic_acting_role', True), ('selection', 'scheduled_on_date')):
            with self.subTest(field=field):
                doc = self.doc_copy()
                self.prop(doc, 'draft_c04_to_02')['succession_contract'][field] = value
                self.assertFlags('E_INCUMBENT', doc, contains='succession_contract')

    def test_appointment_date_key_is_rejected(self):
        doc = self.doc_copy()
        self.prop(doc, 'draft_c04_fr_03')['takes_office_on'] = '2032-05-14'
        self.assertFlags('E_INCUMBENT', doc, contains='takes_office_on')

    def test_replacement_key_nested_anywhere_is_rejected(self):
        doc = self.doc_copy()
        self.prop(doc, 'draft_c04_to_02')['contract_mapping']['replaces'] = 'current premier'
        self.assertFlags('E_INCUMBENT', doc, contains='replaces')

    def test_gates_need_vacancy_or_election(self):
        doc = self.doc_copy()
        w = self.prop(doc, 'draft_c04_to_03')['eligibility_window']
        w['gates'] = [g for g in w['gates'] if g['kind'] not in csp.TRIGGER_GATES]
        self.assertFlags('E_INCUMBENT', doc, contains='vacancy or election')

    def test_date_gate_is_rejected(self):
        doc = self.doc_copy()
        w = self.prop(doc, 'draft_c04_fr_02')['eligibility_window']
        w['gates'].append({'kind': 'date', 'text': 'Takes over automatically.'})
        self.assertFlags('E_INCUMBENT', doc, contains='gate kind')

    def test_dated_gate_text_is_rejected(self):
        doc = self.doc_copy()
        w = self.prop(doc, 'draft_c04_to_01')['eligibility_window']
        w['gates'][0]['text'] = 'Wins the seat on 2029-11-20.'
        self.assertFlags('E_INCUMBENT', doc, contains='may not carry a date')

    def test_packet_contract_must_forbid_date_triggers(self):
        doc = self.doc_copy()
        doc['selection_contract']['date_triggers_allowed'] = True
        self.assertFlags('E_INCUMBENT', doc, contains='selection_contract')

    # ---------- hereditary parentage ----------
    def test_manufactured_title_or_parentage_is_rejected(self):
        for field, value in (('holds_hereditary_title', True), ('claims_royal_or_noble_descent', True), ('parentage', 'son of a noble')):
            with self.subTest(field=field):
                doc = self.doc_copy()
                self.prop(doc, 'draft_c04_to_04')['hereditary_status'][field] = value
                self.assertFlags('E_HEREDITARY', doc)

    def test_hereditary_wording_in_biography_is_rejected(self):
        doc = self.doc_copy()
        bio = self.prop(doc, 'draft_c04_to_02')['biography']
        bio['hypothetical_path_after_cutoff'] += ' He is heir to a noble title.'
        self.assertFlags('E_HEREDITARY', doc)

    # ---------- sources ----------
    def test_unresolved_source_reference_is_rejected(self):
        doc = self.doc_copy()
        self.prop(doc, 'draft_c04_to_01')['source_refs'].append('src_to_constitution_2020#no_such_claim')
        self.assertFlags('E_SOURCE', doc, contains='unresolved')

    def test_unknown_c01_reference_is_rejected(self):
        doc = self.doc_copy()
        doc['role_catalog'][4]['c01_refs'].append('c01:tonga.json#to_no_such_role')
        self.assertFlags('E_SOURCE', doc, contains='unresolved')

    def test_download_without_valid_hash_is_rejected(self):
        sources = copy.deepcopy(self.sources)
        sources['sources'][0]['sha256'] = 'not-a-hash'
        self.assertFlags('E_SOURCE', sources=sources, contains='SHA-256')

    def test_download_not_in_fetch_log_is_rejected(self):
        sources = copy.deepcopy(self.sources)
        sources['sources'][0]['bytes'] += 1
        self.assertFlags('E_SOURCE', sources=sources, contains='fetch_log')

    def test_claim_from_blocked_page_is_rejected(self):
        sources = copy.deepcopy(self.sources)
        blocked = next(s for s in sources['sources'] if s['access_status'] == 'blocked')
        blocked['claims'] = [{'id': 'fr_invented_claim', 'text': 'x', 'locator': 'y'}]
        self.assertFlags('E_SOURCE', sources=sources, contains='blocked')

    def test_role_needs_primary_source(self):
        doc = self.doc_copy()
        role = next(r for r in doc['role_catalog'] if r['role_id'] == 'to_ptoa_party_leader')
        role['eligibility_rules'] = [{'text': 'x', 'source_refs': ['src_to_parliament_how_it_works#to_parl_no_party_system']}]
        self.assertFlags('E_SOURCE', doc, contains='primary')

    def test_every_download_records_bytes_and_sha256(self):
        for s in self.sources['sources']:
            if s['access_status'] == 'downloaded':
                self.assertGreater(s['bytes'], 0, s['id'])
                self.assertRegex(s['sha256'], r'^[0-9a-f]{64}$', s['id'])
            else:
                self.assertEqual(s['claims'], [], s['id'])
                self.assertTrue(s['block_note'], s['id'])

    # ---------- appearance, mapping and packet header ----------
    def test_generated_image_is_rejected(self):
        doc = self.doc_copy()
        self.prop(doc, 'draft_c04_fr_04')['appearance_brief']['image_generated'] = True
        self.assertFlags('E_APPEARANCE', doc)

    def test_attached_asset_is_rejected(self):
        doc = self.doc_copy()
        self.prop(doc, 'draft_c04_fr_04')['appearance_brief']['asset'] = 'spheres-web/ui/person-portraits/x.png'
        self.assertFlags('E_APPEARANCE', doc, contains='asset')

    def test_installed_mapping_is_rejected(self):
        doc = self.doc_copy()
        self.prop(doc, 'draft_c04_to_03')['contract_mapping']['runtime_installed'] = True
        self.assertFlags('E_MAPPING', doc)

    def test_packet_cannot_claim_c04_complete_or_installed(self):
        for field in ('installed', 'c04_complete'):
            with self.subTest(field=field):
                doc = self.doc_copy()
                doc[field] = True
                self.assertFlags('E_FORMAT', doc)

    def test_count_mismatch_is_rejected(self):
        doc = self.doc_copy()
        doc['proposals'] = [p for p in doc['proposals'] if p['draft_id'] != 'draft_c04_to_04']
        self.assertFlags('E_COUNT', doc)


    def test_packet_cannot_lower_its_own_pilot_count(self):
        doc = self.doc_copy()
        doc['proposals'] = doc['proposals'][:1]
        doc['expected_counts'] = {'France': 1}
        self.assertFlags('E_COUNT', doc)

    def test_role_specific_gates_cannot_be_replaced_by_vacancy_alone(self):
        for proposal in self.doc['proposals']:
            with self.subTest(role=proposal['role']['role_id']):
                doc = self.doc_copy()
                self.prop(doc, proposal['draft_id'])['eligibility_window']['gates'] = [
                    {'kind': 'vacancy', 'text': 'An actual vacancy occurs in play.'}]
                self.assertFlags('E_ROLE_GATE', doc)

    def test_required_gate_cannot_have_empty_text(self):
        doc = self.doc_copy()
        self.prop(doc, 'draft_c04_to_02')['eligibility_window']['gates'][0]['text'] = ' '
        self.assertFlags('E_ROLE_GATE', doc)

    def test_packet_cannot_weaken_sourced_role_age(self):
        doc = self.doc_copy()
        next(r for r in doc['role_catalog'] if r['role_id'] == 'to_peoples_representative')['legal_minimum_age'] = 1
        self.assertFlags('E_ROLE_CONTRACT', doc)

    def test_packet_cannot_invent_role_to_bypass_pilot_constraints(self):
        doc = self.doc_copy()
        role = copy.deepcopy(doc['role_catalog'][0])
        role['role_id'] = 'invented_eligible_role'
        doc['role_catalog'].append(role)
        self.prop(doc, 'draft_c04_fr_01')['role']['role_id'] = role['role_id']
        self.assertFlags('E_ROLE_CONTRACT', doc)

    def test_ps_pilot_cannot_use_pre_cutoff_membership_or_unreviewed_waiver(self):
        for draft in ('draft_c04_fr_01', 'draft_c04_fr_03'):
            with self.subTest(draft=draft):
                doc = self.doc_copy()
                proposal = self.prop(doc, draft)
                proposal['eligibility_window']['earliest_from'] = '2026-09-08'
                proposal['age']['youngest_age_at_window_start'] -= 3
                self.assertFlags('E_WINDOW', doc, contains='membership')


    def test_ps_role_affiliation_cannot_be_changed_to_skip_membership(self):
        doc = self.doc_copy()
        role = next(r for r in doc['role_catalog'] if r['role_id'] == 'fr_ps_first_secretary')
        role['game_party_row'] = 'fr_pcf'
        proposal = self.prop(doc, 'draft_c04_fr_01')
        proposal['party_affiliation']['game_party_row'] = 'fr_pcf'
        proposal['eligibility_window']['earliest_from'] = '2026-09-08'
        proposal['age']['youngest_age_at_window_start'] -= 3
        self.assertFlags('E_ROLE_CONTRACT', doc)

    def test_presidential_pilot_affiliation_cannot_skip_ps_constraints(self):
        doc = self.doc_copy()
        proposal = self.prop(doc, 'draft_c04_fr_03')
        proposal['party_affiliation']['game_party_row'] = 'fr_pcf'
        proposal['eligibility_window']['earliest_from'] = '2026-09-08'
        proposal['age']['youngest_age_at_window_start'] -= 3
        self.assertFlags('E_PARTY', doc)


class CommandLineTests(unittest.TestCase):
    SCRIPT = HERE / 'check_successor_proposals.py'

    def run_cli(self, *args):
        env = dict(os.environ, PYTHONDONTWRITEBYTECODE='1')
        return subprocess.run([sys.executable, '-X', 'utf8', str(self.SCRIPT), *args],
                              capture_output=True, text=True, encoding='utf-8', env=env)

    def test_committed_packet_exits_zero(self):
        result = self.run_cli()
        self.assertEqual(result.returncode, 0, result.stdout + result.stderr)
        self.assertIn('PASS: 8 proposal-only fictional drafts (France 4, Tonga 4)', result.stdout)

    def test_violation_exits_non_zero(self):
        doc, sources = csp.load_packet(csp.ROOT, csp.PACKET)
        doc['proposals'][0]['eligibility_window']['earliest_from'] = '2020-01-01'
        doc['proposals'][1]['succession_contract']['replaces_incumbent_on_date'] = True
        with tempfile.TemporaryDirectory() as tmp:
            (pathlib.Path(tmp) / 'proposals.json').write_text(json.dumps(doc), encoding='utf-8')
            (pathlib.Path(tmp) / 'sources.json').write_text(json.dumps(sources), encoding='utf-8')
            result = self.run_cli('--packet', tmp, '--json')
        self.assertEqual(result.returncode, 1, result.stdout + result.stderr)
        report = json.loads(result.stdout)
        self.assertFalse(report['pass'])
        codes = {e['code'] for e in report['errors']}
        self.assertTrue({'E_HISTORICAL', 'E_INCUMBENT'} <= codes, codes)

    def test_missing_packet_exits_two(self):
        with tempfile.TemporaryDirectory() as tmp:
            result = self.run_cli('--packet', tmp)
        self.assertEqual(result.returncode, 2, result.stdout + result.stderr)


if __name__ == '__main__':
    unittest.main()
