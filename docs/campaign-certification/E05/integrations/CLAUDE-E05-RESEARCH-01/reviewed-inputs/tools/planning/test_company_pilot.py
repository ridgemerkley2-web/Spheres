"""Fail-closed tests for the E05 company research pilot validator (isolated fixtures)."""
import copy
import importlib.util
import json
from pathlib import Path
import shutil
import subprocess
import sys
import tempfile
import unittest

HERE = Path(__file__).resolve().parent
REPO = HERE.parents[1]
spec = importlib.util.spec_from_file_location('check_company_pilot', HERE / 'check_company_pilot.py')
checker = importlib.util.module_from_spec(spec)
spec.loader.exec_module(checker)

GAME = {
    'spheres-sim/src/equipment.rs': (
        'pub const PLATFORMS: &[PlatformDef] = &[\n'
        '    PlatformDef { id:"tank_standard", name:"Main battle tank", capacity:12 },\n'
        '    PlatformDef { id:"ground_apc", name:"Armored personnel carrier", capacity:12 },\n'
        '    PlatformDef { id:"air_fighter", name:"Defensive fighter", capacity:21 },\n'
        '];\n'),
    'spheres-sim/src/arsenal.rs': (
        'pub const DECK: &[EquipmentDef] = &[\n'
        '    kit("air_gen4", "Fourth-Generation Combat Aircraft", Class::Air, None, 3.4, 0.085, 72, 480),\n'
        '    kit("nav_escort", "Escort Frigate or Destroyer", Class::Naval, None, 2.2, 0.180, 72, 480),\n'
        '];\n'),
    'spheres-sim/src/equipment_ammunition_production.rs': (
        'pub const AMMUNITION_CATALOG: &[AmmoDef] = &[\n'
        '    ammo!(\n        "tank_120_mixed",\n        "120 mm mixed-purpose",\n        1.0\n    ),\n'
        '    AmmoDef {\n        id: "air_bomb_guided",\n    },\n'
        '];\n'),
    'spheres-sim/src/supplier_catalogue.rs': (
        'const ROSTER: [(&str, &str, u32); 2] = [\n'
        '    ("France", "ground_apc", 4), ("Japan", "air_light_attack", 1),\n'
        '];\n'),
    'spheres-sim/src/sector_contractors.rs': (
        'impl CompanySector {\n'
        '    pub fn key(self) -> &\'static str {\n'
        '        match self {\n'
        '            Self::Construction => "construction",\n'
        '            Self::Manufacturing => "manufacturing",\n'
        '            Self::Defense => "defense",\n'
        '        }\n'
        '    }\n'
        '    pub fn name(self) -> &\'static str { "x" }\n'
        '}\n'),
    'spheres-sim/src/companies.rs': (
        'pub enum CompanyOrder {\n'
        '    EnableImports { quote: String },\n'
        '    Establish {\n        name: String,\n    },\n'
        '    Capitalize {\n        company: u32,\n    },\n'
        '    Develop {\n        company: u32,\n    },\n'
        '    Inventory {\n        company: u32,\n    },\n'
        '    Purchase {\n        company: u32,\n    },\n'
        '}\n'),
}
NATIONS = {
    'france': {'tech_1990': {'granted': [{'id': 'matl_composite_structures', 'source': 'Dassault flew a demonstrator.'}]}},
    'japan': {'tech_1990': {'granted': [{'id': 'matl_industrial_robotics', 'source': 'Kawasaki Heavy Industries built robots.'}]}},
}
PLAN = [('fr_alpha', 'France', 'ground'), ('fr_bravo', 'France', 'aerospace'), ('fr_charlie', 'France', 'naval'),
        ('fr_delta', 'France', 'civilian_industrial'), ('jp_echo', 'Japan', 'ground'), ('jp_foxtrot', 'Japan', 'aerospace'),
        ('jp_golf', 'Japan', 'naval'), ('jp_hotel', 'Japan', 'civilian_industrial')]
SHA = 'a' * 64


def source(sid):
    return {
        'id': sid, 'url': f'https://example.org/{sid}.pdf', 'title': f'Annual report {sid}', 'publisher': 'Example SA',
        'kind': 'annual_report', 'language': 'en', 'published': '2026-03-01', 'covers_through': '2026-03-01',
        'accessed': '2026-09-28', 'rights_note': 'Facts only; no logo or image rights are implied.',
        'response': {'status': 200, 'bytes': 1000, 'sha256': SHA, 'content_type': 'application/pdf',
                     'final_url': f'https://example.org/{sid}.pdf', 'fetched_utc': '2026-09-28T04:00:00+00:00'},
        'byte_stability': {'status': 'byte_stable', 'recheck_utc': '2026-09-28T04:45:00+00:00', 'recheck_bytes': 1000,
                           'recheck_sha256': SHA},
    }


def dossier(did, nation, activity, n):
    def c(k):
        return f'{did}_c{k}'
    sid = f'{did}_src'
    old, new = f'Old {did}', f'New {did}'
    return {
        'schema': 'spheres-company-pilot-dossier-v1', 'id': did, 'display_name': f'Company {did.upper()}', 'nation': nation,
        'research_window': {'from': '1990-01-01', 'through': '2026-09-07'}, 'activities': [activity],
        'future_policy': {'after': '2026-09-07', 'real_history': 'not_predicted', 'later_products': 'fictional_or_unknown'},
        'entities': [{'id': f'{did}_sa', 'kind': 'company', 'subject': True,
                      'registration': {'register': 'RCS', 'number': f'{n:03d} 000 000', 'claims': [c('01')]},
                      'names': [{'name': old, 'from': '1990-01-01', 'from_basis': 'window_start', 'until': '2000-06-01', 'claims': [c('01'), c('02')]},
                                {'name': new, 'from': '2000-06-01', 'until': None, 'claims': [c('02'), c('03')]}],
                      'roles': [{'role': 'manufacturer', 'from': '1990-01-01', 'from_basis': 'window_start', 'until': None, 'claims': [c('01'), c('03')]}]}],
        'lineage': [{'id': f'{did}_l01', 'kind': 'rename', 'entity': f'{did}_sa', 'date': '2000-06-01', 'from_name': old, 'to_name': new, 'claims': [c('02')]},
                    {'id': f'{did}_l02', 'kind': 'acquisition', 'entity': f'{did}_sa', 'date': '2000-06-01', 'counterparties': [{'name': f'Target of {did}', 'relation': 'bought'}], 'claims': [c('02')]}],
        'country_links': [{'country': nation, 'kind': 'headquarters', 'date': '2026-01-15', 'claims': [c('03')]}],
        'ownership': [{'holder': 'Founders', 'share': '51%', 'date': '2026-01-15', 'claims': [c('03')]}],
        'products': [{'id': f'{did}_p1', 'name': f'Product {did}', 'reality': 'historical', 'category': 'vehicle',
                      'milestones': [{'event': 'first_delivery', 'date': '1995-05', 'maker_entity': f'{did}_sa', 'maker_name': old, 'claims': [c('04')]}]},
                     {'id': f'{did}_p2', 'name': f'Next {did}', 'reality': 'announced', 'category': 'vehicle', 'outcome': 'unknown_after_cutoff',
                      'milestones': [{'event': 'announced', 'date': '2026-01-15', 'maker_entity': f'{did}_sa', 'maker_name': new, 'claims': [c('03')]}]},
                     {'id': f'{did}_p3', 'name': f'Successor {did} (fictional)', 'reality': 'fictional', 'fictional': True, 'available_from': '2030'}],
        'game_mapping': {
            'proposed': [{'target_kind': 'designer_platform', 'target_id': 'tank_standard', 'flow': 'supplier_equipment_flow', 'confidence': 'medium',
                          'basis_products': [f'{did}_p1'], 'uncertainty': 'generic envelope, not a replica'},
                         {'target_kind': 'sector_contractor_slot', 'target_id': f'{nation}:defense', 'flow': 'sector_contractor_service',
                          'confidence': 'low', 'uncertainty': 'generated contractors are fictional'}],
            'integration_flow': {'steps': [{'order': 'Establish'}, {'order': 'Develop'}, {'order': 'Purchase'}]},
            'constraints': {'government_production_capacity': False, 'free_stock': False, 'parallel_financial_ledger': False,
                            'opening_balances_or_stock': 'not_sourced_not_proposed'}},
        'claims': [
            {'id': c('01'), 'source': sid, 'attests': ['1990'], 'text': 'Operating in 1990.', 'locator': {'page': 1}, 'anchor': 'in 1990'},
            {'id': c('02'), 'source': sid, 'attests': ['2000-06-01'], 'text': 'Renamed; bought a target.', 'locator': {'page': 2}, 'anchor': 'on 1 June 2000'},
            {'id': c('03'), 'source': sid, 'attests': ['2026-01-15'], 'text': 'Operating in 2026.', 'locator': {'page': 3}, 'anchor': 'January 2026'},
            {'id': c('04'), 'source': sid, 'attests': ['1995-05'], 'text': 'First delivery.', 'locator': {'page': 4}, 'anchor': 'May 1995'}],
    }


class CompanyPilotTests(unittest.TestCase):
    def setUp(self):
        self.temp = tempfile.TemporaryDirectory()
        self.addCleanup(self.temp.cleanup)
        self.root = Path(self.temp.name)
        for rel, text in GAME.items():
            path = self.root / rel
            path.parent.mkdir(parents=True, exist_ok=True)
            path.write_text(text, encoding='utf-8')
        for name, data in NATIONS.items():
            path = self.root / f'spheres-sim/data/nations/{name}.json'
            path.parent.mkdir(parents=True, exist_ok=True)
            path.write_text(json.dumps(data), encoding='utf-8')
        self.dossiers = [dossier(did, nation, act, i + 1) for i, (did, nation, act) in enumerate(PLAN)]
        self.registry = {'schema': 'spheres-company-pilot-sources-v1', 'research_cutoff': '2026-09-07',
                         'sources': [source(f'{did}_src') for did, _, _ in PLAN], 'inaccessible': [], 'leads': []}
        self.catalog = 'Research cutoff 2026-09-07.\n' + '\n'.join(f'- {d["id"]}: {d["display_name"]}' for d in self.dossiers) + '\n'

    def write(self):
        base = self.root / 'docs/research/company-pilot'
        if base.exists():
            shutil.rmtree(base)
        (base / 'dossiers').mkdir(parents=True)
        (base / 'sources.json').write_text(json.dumps(self.registry), encoding='utf-8')
        for d in self.dossiers:
            (base / 'dossiers' / f'{d["id"]}.json').write_text(json.dumps(d), encoding='utf-8')
        (base / 'README.md').write_text(self.catalog, encoding='utf-8')
        return base

    def problems(self):
        self.write()
        found, _, _ = checker.run(self.root)
        return found.items

    def assertViolation(self, category, fragment):
        items = self.problems()
        hits = [m for cat, m in items if cat == category and fragment in m]
        self.assertTrue(hits, f'expected [{category}] containing {fragment!r}; got {items}')

    def d(self, i=0):
        return self.dossiers[i]

    # --- baseline ---------------------------------------------------------------
    def test_valid_fixture_passes_and_cli_exits_zero(self):
        self.assertEqual(self.problems(), [])
        shutil.copyfile(HERE / 'check_company_pilot.py', self.root / 'check.py')
        run = subprocess.run([sys.executable, '-X', 'utf8', str(self.root / 'check.py'), '--root', str(self.root)],
                             capture_output=True, text=True, encoding='utf-8')
        self.assertEqual(run.returncode, 0, run.stdout + run.stderr)
        self.assertIn('PASS: 8 dossiers', run.stdout)

    def test_cli_exits_nonzero_and_lists_violations(self):
        self.d()['claims'][0]['locator'] = {}
        self.write()
        shutil.copyfile(HERE / 'check_company_pilot.py', self.root / 'check.py')
        run = subprocess.run([sys.executable, '-X', 'utf8', str(self.root / 'check.py'), '--root', str(self.root)],
                             capture_output=True, text=True, encoding='utf-8')
        self.assertEqual(run.returncode, 1)
        self.assertIn('[provenance]', run.stdout)

    def test_committed_pilot_passes(self):
        found, stats, _ = checker.run(REPO)
        self.assertEqual(found.items, [], 'the committed pilot must validate')
        self.assertEqual(stats['dossiers'], 8)

    # --- duplicate IDs ----------------------------------------------------------
    def test_duplicate_claim_id_across_dossiers(self):
        self.d(1)['claims'][3]['id'] = self.d(0)['claims'][3]['id']
        self.assertViolation('duplicate_id', 'duplicate claim ID')

    def test_duplicate_dossier_id_in_second_file(self):
        base = self.write()
        clone = copy.deepcopy(self.d(0))
        (base / 'dossiers' / 'zz_copy.json').write_text(json.dumps(clone), encoding='utf-8')
        found, _, _ = checker.run(self.root)
        self.assertTrue(any(c == 'duplicate_id' and 'duplicate dossier ID' in m for c, m in found.items), found.items)

    def test_duplicate_source_id(self):
        extra = source(self.registry['sources'][0]['id'])
        extra['url'] = 'https://example.org/other.pdf'
        self.registry['sources'].append(extra)
        self.assertViolation('duplicate_id', 'duplicate source ID')

    def test_duplicate_product_and_entity_ids(self):
        self.d(1)['products'][0]['id'] = self.d(0)['products'][0]['id']
        self.d(1)['entities'][0]['id'] = self.d(0)['entities'][0]['id']
        items = self.problems()
        self.assertTrue(any(c == 'duplicate_id' and 'duplicate product ID' in m for c, m in items), items)
        self.assertTrue(any(c == 'duplicate_id' and 'duplicate entity ID' in m for c, m in items), items)

    # --- unsupported periods ----------------------------------------------------
    def test_milestone_after_cutoff(self):
        self.d()['products'][0]['milestones'][0]['date'] = '2027-01'
        self.assertViolation('period', 'must lie within 1990-01-01..2026-09-07')

    def test_year_precision_reaching_past_cutoff(self):
        self.d()['country_links'][0]['date'] = '2026'
        self.assertViolation('period', 'must lie within the research window')

    def test_claim_before_window_without_context(self):
        self.d()['claims'][0]['attests'] = ['1989-12']
        self.assertViolation('period', 'outside the research window')

    def test_claim_not_covered_by_its_source(self):
        self.registry['sources'][0]['covers_through'] = '2020-12-31'
        self.registry['sources'][0]['published'] = '2020-12-31'
        self.assertViolation('period', 'is not covered by source')

    def test_endpoint_not_attested_by_cited_claims(self):
        self.d()['entities'][0]['names'][1]['claims'] = [self.d()['claims'][2]['id']]
        self.assertViolation('period', 'no cited claim attests 2000-06-01')

    def test_ongoing_period_needs_recent_evidence(self):
        self.d()['claims'][2]['attests'] = ['2024-01-15']
        self.assertViolation('period', 'ongoing period needs evidence dated on or after 2025-09-07')

    def test_window_start_needs_1990_or_earlier_evidence(self):
        self.d()['claims'][0]['attests'] = ['1995-05']
        self.assertViolation('period', 'window-start period needs 1990 or pre-1990 context evidence')

    def test_pre_window_context_must_predate_1990(self):
        self.d()['claims'][3]['context_before_window'] = True
        self.assertViolation('period', 'pre-window context must predate 1990')

    # --- merger / rename identity confusion ------------------------------------------
    def test_name_used_before_rename(self):
        self.d()['products'][0]['milestones'][0]['maker_name'] = self.d()['entities'][0]['names'][1]['name']
        self.assertViolation('identity', 'was not the name of')

    def test_old_name_is_not_valid_on_exact_rename_day(self):
        d = self.d()
        m = d['products'][0]['milestones'][0]
        m.update(date='2000-06-01', claims=[d['claims'][1]['id']])
        self.assertViolation('identity', 'was not the name of')

    def test_new_name_is_valid_on_exact_rename_day(self):
        d = self.d()
        m = d['products'][0]['milestones'][0]
        m.update(date='2000-06-01', claims=[d['claims'][1]['id']],
                 maker_name=d['entities'][0]['names'][1]['name'])
        self.assertEqual(self.problems(), [])

    def test_last_observed_name_includes_the_observation_day(self):
        d = self.d()
        d['entities'][0]['names'][0]['until_basis'] = 'last_observed'
        d['products'][0]['milestones'][0].update(date='2000-06-01', claims=[d['claims'][1]['id']])
        self.assertEqual(self.problems(), [])

    def test_rename_across_legal_entities(self):
        d = self.d()
        d['entities'].append({'id': f'{d["id"]}_holding', 'kind': 'holding', 'subject': False,
                              'names': [{'name': 'Parent Holding', 'from': '2000-06-01', 'until': None, 'claims': [f'{d["id"]}_c02', f'{d["id"]}_c03']}]})
        d['lineage'][0]['to_name'] = 'Parent Holding'
        self.assertViolation('identity', 'must connect two names of the same entity')

    def test_merger_counterparty_is_subject_itself(self):
        d = self.d()
        d['lineage'][1]['counterparties'] = [{'name': d['entities'][0]['names'][0]['name'], 'relation': 'merged'}]
        self.assertViolation('identity', 'is a name of the subject entity itself')

    def test_merger_recorded_as_rename(self):
        self.d()['lineage'][1]['to_name'] = 'Target renamed'
        self.assertViolation('identity', 'is not a rename')

    def test_one_business_in_two_dossiers(self):
        self.d(1)['entities'][0]['registration']['number'] = self.d(0)['entities'][0]['registration']['number']
        self.assertViolation('identity', 'one business, one dossier')

    def test_same_name_overlapping_across_dossiers(self):
        self.d(1)['entities'][0]['names'][1]['name'] = self.d(0)['entities'][0]['names'][1]['name']
        self.d(1)['lineage'][0]['to_name'] = self.d(0)['entities'][0]['names'][1]['name']
        self.assertViolation('identity', 'overlapping periods')

    def test_name_intervals_must_be_contiguous(self):
        self.d()['entities'][0]['names'][1]['from'] = '2000-07-01'
        self.assertViolation('identity', 'must start exactly where the previous name ended')

    def test_rename_date_must_match_name_boundary(self):
        self.d()['lineage'][0]['date'] = '1995-05'
        self.d()['lineage'][0]['claims'] = [self.d()['claims'][3]['id']]
        self.assertViolation('identity', 'date must end the old name and start the new one')

    def test_counterparty_entity_must_be_another_entity(self):
        d = self.d()
        d['lineage'][1]['counterparties'] = [{'name': 'Target', 'entity': d['entities'][0]['id'], 'relation': 'merged'}]
        self.assertViolation('identity', 'counterparty entity must be a different recorded entity')

    def test_retrospective_wording_must_be_marked(self):
        m = self.d()['products'][0]['milestones'][0]
        m['name_as_written'] = self.d()['entities'][0]['names'][1]['name']
        self.assertViolation('identity', 'mark it retrospective')

    # --- real / fictional mixing --------------------------------------------------
    def test_fictional_product_cannot_cite_claims(self):
        self.d()['products'][2]['milestones'] = [{'event': 'first_delivery', 'date': '1995-05', 'maker_entity': f'{self.d()["id"]}_sa', 'maker_name': 'x', 'claims': []}]
        self.assertViolation('reality', 'cannot cite real sources, claims or milestones')

    def test_fictional_product_needs_label_and_future_availability(self):
        p = self.d()['products'][2]
        p['name'] = 'Successor model'
        p['available_from'] = '2020'
        items = self.problems()
        self.assertTrue(any('labelled "(fictional)"' in m for _, m in items), items)
        self.assertTrue(any('must follow the 2026-09-07 cutoff' in m for _, m in items), items)

    def test_real_product_cannot_be_marked_fictional(self):
        self.d()['products'][0]['fictional'] = True
        self.assertViolation('reality', 'cannot be flagged fictional')

    def test_fictional_product_cannot_reuse_real_name(self):
        self.d()['products'][2]['name'] = self.d()['products'][0]['name'] + ' (fictional)'
        self.assertViolation('reality', 'reuses a real product name')

    def test_announced_product_cannot_be_delivered(self):
        self.d()['products'][1]['milestones'][0]['event'] = 'first_delivery'
        self.assertViolation('reality', 'cannot record delivery or service entry')

    def test_announced_product_outcome_stays_unknown(self):
        self.d()['products'][1]['outcome'] = 'delivered_in_2030'
        self.assertViolation('reality', 'must leave its outcome unknown after the cutoff')

    def test_cancelled_product_needs_sourced_ending(self):
        p = self.d()['products'][1]
        p['reality'] = 'cancelled'
        p.pop('outcome')
        self.assertViolation('reality', 'needs a sourced cancellation or termination')

    def test_ended_undelivered_programme_is_not_historical(self):
        p = self.d()['products'][0]
        p['milestones'][0]['event'] = 'development_discontinued'
        self.assertViolation('reality', 'is cancelled, not historical')

    def test_future_policy_must_not_predict(self):
        self.d()['future_policy']['real_history'] = 'predicted_to_2035'
        self.assertViolation('reality', 'future policy')

    # --- missing provenance ------------------------------------------------------
    def test_claim_without_locator(self):
        self.d()['claims'][0]['locator'] = {'page': ''}
        self.assertViolation('provenance', 'missing locator')

    def test_claim_with_unknown_source(self):
        self.d()['claims'][0]['source'] = 'nowhere'
        self.assertViolation('provenance', 'is not in the registry')

    def test_source_without_hash(self):
        self.registry['sources'][0]['response']['sha256'] = ''
        self.assertViolation('provenance', 'SHA-256 missing')

    def test_source_not_rechecked(self):
        self.registry['sources'][0]['byte_stability'] = {'status': 'not_rechecked'}
        self.assertViolation('provenance', 'byte stability must be checked')

    def test_recheck_too_soon(self):
        self.registry['sources'][0]['byte_stability']['recheck_utc'] = '2026-09-28T04:10:00+00:00'
        self.assertViolation('provenance', 'recheck must follow the first download by 30+ minutes')

    def test_changed_response_needs_reverified_anchors(self):
        b = self.registry['sources'][0]['byte_stability']
        b.update(status='changed', recheck_sha256='b' * 64, anchors_reverified=False)
        self.assertViolation('provenance', 'needs re-verified anchors')

    def test_changed_response_requires_complete_recheck_identity(self):
        original = copy.deepcopy(self.registry['sources'][0]['byte_stability'])
        for field, value in [('recheck_sha256', None), ('recheck_sha256', ''),
                             ('recheck_sha256', 'not-a-sha'), ('recheck_bytes', None),
                             ('recheck_bytes', 0), ('recheck_bytes', -1),
                             ('recheck_bytes', True), ('recheck_bytes', '1200')]:
            with self.subTest(field=field, value=value):
                b = copy.deepcopy(original)
                b.update(status='changed', recheck_sha256='b' * 64,
                         anchors_reverified=True, note='regenerated page')
                b[field] = value
                self.registry['sources'][0]['byte_stability'] = b
                self.assertViolation('provenance', 'recheck identity')

    def test_changed_status_with_identical_recheck(self):
        b = self.registry['sources'][0]['byte_stability']
        b.update(status='changed', anchors_reverified=True, note='regenerated page')
        self.assertViolation('provenance', 'marked changed but the recheck hash is identical')

    def test_byte_stable_status_with_different_recheck(self):
        self.registry['sources'][0]['byte_stability']['recheck_sha256'] = 'b' * 64
        self.assertViolation('provenance', 'marked byte_stable but the recheck differs')

    def test_registered_source_without_claims(self):
        extra = source('unused_src')
        self.registry['sources'].append(extra)
        self.assertViolation('provenance', 'supports no claim')

    def test_lead_cannot_be_evidence(self):
        self.registry['leads'].append({'url': self.registry['sources'][0]['url'], 'kind': 'news'})
        self.assertViolation('provenance', 'lead cannot be evidence')

    def test_inaccessible_source_cannot_support_claims(self):
        self.registry['inaccessible'].append({'url': self.registry['sources'][0]['url'], 'attempted_utc': '2026-09-28T04:00:00+00:00', 'result': 'HTTP 403'})
        self.assertViolation('provenance', 'recorded as inaccessible')

    def test_encyclopaedia_kind_is_not_evidence(self):
        self.registry['sources'][0]['kind'] = 'encyclopaedia'
        self.assertViolation('provenance', 'not an official evidence kind')

    def test_long_quotation_is_refused(self):
        self.d()['claims'][0]['anchor'] = ' '.join(['word'] * 13)
        self.assertViolation('provenance', 'anchor exceeds')

    def test_period_without_claims(self):
        self.d()['country_links'][0]['claims'] = []
        self.assertViolation('provenance', 'cites no claims')

    # --- game mappings and forbidden shortcuts ----------------------------------------
    def test_unknown_platform_target(self):
        self.d()['game_mapping']['proposed'][0]['target_id'] = 'tank_imaginary'
        self.assertViolation('mapping', 'does not exist in current game source')

    def test_government_capacity_shortcut_refused(self):
        self.d()['game_mapping']['constraints']['government_production_capacity'] = True
        self.assertViolation('mapping', 'government_production_capacity must be false')

    def test_unknown_company_order_in_flow(self):
        self.d()['game_mapping']['integration_flow']['steps'].append({'order': 'GrantStock'})
        self.assertViolation('mapping', "'GrantStock' is not a current CompanyOrder")

    def test_sector_slot_of_other_nation(self):
        self.d()['game_mapping']['proposed'][1]['target_id'] = 'Japan:defense'
        self.assertViolation('mapping', 'not the dossier nation')

    def test_one_supplier_programme_for_two_firms(self):
        for i in (0, 1):
            self.d(i)['game_mapping']['proposed'].append({'target_kind': 'supplier_catalogue_roster', 'target_id': 'France:ground_apc',
                'flow': 'supplier_programme_slot', 'confidence': 'low', 'basis_products': [f'{self.d(i)["id"]}_p1'], 'uncertainty': 'modeled slot'})
        self.assertViolation('mapping', 'one existing programme cannot become two firms')

    def test_tech_reference_must_match_existing_evidence(self):
        self.d()['game_mapping']['proposed'].append({'target_kind': 'tech_node', 'target_id': 'France:matl_composite_structures', 'flow': 'reference_only',
            'confidence': 'high', 'uncertainty': 'reference only', 'existing_reference': {'match_text': 'Airbus'}})
        self.assertViolation('mapping', 'must appear in that technology source')

    def test_mapping_flow_must_match_target(self):
        self.d()['game_mapping']['proposed'][0]['flow'] = 'no_company_consumer'
        self.assertViolation('mapping', 'flow must be supplier_equipment_flow')

    # --- catalog -----------------------------------------------------------------
    def test_catalog_must_list_every_dossier(self):
        self.catalog = self.catalog.replace(self.d(7)['id'], 'removed')
        self.assertViolation('catalog', 'catalog omits dossier')

    def test_pilot_scope_four_per_nation_and_activity_mix(self):
        self.dossiers[7]['nation'] = 'France'
        self.dossiers[7]['game_mapping']['proposed'][1]['target_id'] = 'France:defense'
        self.dossiers[3]['activities'] = ['ground']
        self.dossiers[7]['activities'] = ['ground']
        items = self.problems()
        self.assertTrue(any('pilot needs 4 Japan dossiers' in m for _, m in items), items)
        self.assertTrue(any("missing ['civilian_industrial']" in m for _, m in items), items)


if __name__ == '__main__':
    unittest.main()
