"""CLAUDE-C01-GAPS-01: the certified-country gap ledger keeps roles, evidence classes and registries apart."""
import copy
import hashlib
import json
import shutil
import tempfile
import unittest
from pathlib import Path
from unittest.mock import patch

import certified_gap_ledger as ledger

HEX = '0' * 40
CASES = ['France', 'Japan', 'India', 'Brazil', 'SouthAfrica', 'Tonga', 'SaudiArabia', 'USSR -> Russia']
IDENTITIES = ['France', 'Japan', 'India', 'Brazil', 'SouthAfrica', 'Tonga', 'SaudiArabia', 'USSR', 'Russia']


def source(sid, claims, path=None):
    row = {'id': sid, 'url': f'https://example.org/{sid}', 'title': sid, 'publisher': 'fixture',
           'accessed_date': '2026-09-20', 'claims': [{'id': c, 'text': c} for c in claims]}
    if path:
        row['snapshot'] = {'path': path}
    return row


def holder(name, sources, claims, **dates):
    row = {'name': name, 'attested_on': None, 'from': None, 'until': None, 'sources': sources, 'claim_ids': claims}
    row.update(dates)
    return row


def packet(nation, sources, organizations=(), institutions=()):
    return {'version': 1, 'nation': nation, 'research_cutoff': ledger.CUTOFF, 'sources': sources,
            'organizations': list(organizations), 'institutions': list(institutions),
            'coverage': {'status': 'partial_primary_source_inventory', 'unresolved': ['fixture']}}


def fixture_tree():
    """A minimal checked-in tree with every ledger input, for the eight certified cases."""
    za = packet('SouthAfrica', [
        source('s_anc', ['c_anc_mandela'], 'docs/campaign-certification/C01/research/sources/za-anc.json'),
        source('s_pres', ['c_pres_mandela'], 'docs/campaign-certification/C01/research/sources/za-pres.json'),
        source('s_ff', ['c_ff'])],
        organizations=[
            {'id': 'za_org_anc', 'name': 'AFRICAN NATIONAL CONGRESS', 'kind': 'x', 'represented_party_ids': [],
             'roles': [{'id': 'za_anc_president', 'title': 'President of the ANC', 'kind': 'party_leader', 'sources': ['s_anc'],
                        'holder_claims': [holder('Nelson Mandela', ['s_anc'], ['c_anc_mandela'], attested_on='1995-01-01')]}]},
            {'id': 'za_org_ff1', 'name': 'Freedom Front', 'kind': 'x', 'represented_party_ids': [], 'roles': []},
            {'id': 'za_org_ff2', 'name': 'FREEDOM  FRONT', 'kind': 'x', 'represented_party_ids': [], 'roles': []}],
        institutions=[{'id': 'za_presidency', 'name': 'Presidency', 'kind': 'executive_institution', 'roles': [
            {'id': 'za_president_election', 'title': 'President', 'kind': 'head_of_state', 'sources': ['s_pres'],
             'holder_claims': [holder('Nelson Mandela', ['s_pres'], ['c_pres_mandela'], **{'from': '1994-05-10', 'until': '1999-06-16'})]}]}])
    to = packet('Tonga', [source('s_to', ['c_king', 'c_speaker'])], institutions=[
        {'id': 'to_crown', 'name': 'Crown of Tonga', 'kind': 'x', 'roles': [
            {'id': 'to_king', 'title': 'King', 'kind': 'head_of_state', 'sources': ['s_to'],
             'holder_claims': [holder('Tupou VI', ['s_to'], ['c_king'], **{'from': '2012-03-18'})]}]},
        {'id': 'to_legislative_assembly', 'name': 'Legislative Assembly', 'kind': 'x', 'roles': [
            {'id': 'to_speaker', 'title': 'Speaker', 'kind': 'institutional_office', 'sources': ['s_to'], 'holder_claims': ['c_speaker']}]}])
    sa = packet('SaudiArabia', [source('s_sa', ['c_sa'])], institutions=[
        {'id': 'sa_shura', 'name': 'Shura Council', 'kind': 'x', 'roles': [
            {'id': 'sa_shura_chair', 'title': 'Chairman', 'kind': 'institutional_office', 'sources': ['s_sa'], 'holder_claims': []},
            {'id': 'sa_shura_members', 'title': 'Members', 'kind': 'collective_seat', 'sources': ['s_sa'], 'holder_claims': []}]}])
    su = packet('USSR', [source('s_su', ['c_su'])], institutions=[
        {'id': 'su_presidency', 'name': 'Presidency of the USSR', 'kind': 'x', 'roles': [
            {'id': 'su_president', 'title': 'President of the USSR', 'kind': 'head_of_state', 'sources': ['s_su'],
             'holder_claims': [holder('Mikhail Gorbachev', ['s_su'], ['c_su'], attested_on='1990-03-20')]}]}])
    packets = {'SouthAfrica': za, 'Tonga': to, 'SaudiArabia': sa, 'USSR': su}
    attribution = {'sources': {
        'SouthAfrica': {'s_anc': {'packet': 'CLAUDE-C01-16'}, 's_pres': {'packet': 'CLAUDE-C01-01'}, 's_ff': {'packet': 'S10'}},
        'Tonga': {'s_to': {'packet': 'CLAUDE-C01-01'}}, 'SaudiArabia': {'s_sa': {'packet': 'S10'}}, 'USSR': {'s_su': {'packet': 'S10'}}}}
    represented = [
        {'id': 'SouthAfrica/za_anc', 'nation': 'SouthAfrica', 'party': 'za_anc', 'name': 'African National Congress', 'native': None,
         'representation': 'simulation_party_row', 'organization_kind': 'party', 'history_status': 'partial', 'component_ids': [],
         'sources': ['https://example.org/anc-row'], 'declared_gaps': []},
        {'id': 'SouthAfrica/za_new', 'nation': 'SouthAfrica', 'party': 'za_new', 'name': 'New Party', 'native': None,
         'representation': 'simulation_party_row', 'organization_kind': 'party', 'history_status': 'gap', 'component_ids': [],
         'sources': [], 'declared_gaps': [{'from': {'kind': 'day', 'value': '1990-01-01'}, 'until': {'kind': 'open', 'value': None},
                                          'reason': 'No leader terms ingested.', 'sources': ['https://example.org/gap']}]},
        {'id': 'France/fr_udf', 'nation': 'France', 'party': 'fr_udf', 'name': 'Union for French Democracy', 'native': None,
         'representation': 'simulation_party_row', 'organization_kind': 'coalition', 'history_status': 'partial',
         'component_ids': ['fr_udf_cds'], 'sources': [], 'declared_gaps': []},
        {'id': 'France/fr_udf/fr_udf_cds', 'nation': 'France', 'party': 'fr_udf', 'name': 'CDS', 'native': None,
         'representation': 'registered_component_or_historical_name_phase', 'organization_kind': 'not_separately_typed',
         'history_status': 'partial', 'component_ids': [], 'sources': [], 'declared_gaps': []},
        {'id': 'India/in_bjp', 'nation': 'India', 'party': 'in_bjp', 'name': 'Bharatiya Janata Party', 'native': None,
         'representation': 'simulation_party_row', 'organization_kind': 'party', 'history_status': 'partial', 'component_ids': [],
         'sources': [], 'declared_gaps': []}]
    terms = [
        {'id': 'za_anc_mandela', 'nation': 'SouthAfrica', 'organization_id': 'SouthAfrica/za_anc', 'person': 'nelson_mandela',
         'role': 'President of the ANC', 'kind': 'leader', 'from': {'kind': 'day', 'value': '1991-07-05'},
         'until': {'kind': 'day', 'value': '1997-12-20'}},
        {'id': 'za_anc_mbeki', 'nation': 'SouthAfrica', 'organization_id': 'SouthAfrica/za_anc', 'person': 'thabo_mbeki',
         'role': 'President of the ANC', 'kind': 'leader', 'from': {'kind': 'day', 'value': '1997-12-20'},
         'until': {'kind': 'year', 'value': '2007'}},
        {'id': 'fr_cds_leader', 'nation': 'France', 'organization_id': 'France/fr_udf/fr_udf_cds', 'person': 'pierre_mehaignerie',
         'role': 'President of the CDS', 'kind': 'leader', 'from': {'kind': 'year', 'value': '1982'},
         'until': {'kind': 'month', 'value': '1994-12'}},
        {'id': 'in_bjp_advani', 'nation': 'India', 'organization_id': 'India/in_bjp', 'person': 'l_k_advani',
         'role': 'National President of the BJP', 'kind': 'leader', 'from': {'kind': 'year', 'value': '1986'},
         'until': {'kind': 'unknown', 'value': None}}]
    roles = {'term_records': terms,
             'seed_executive_observations': [{'nation': 'SouthAfrica', 'person': 'nelson_mandela', 'since': '1994-05-10', 'sources': []}],
             'historical_executive_gameplay_grants': [],
             'institution_policy_records': [{'nation': 'France', 'fact': 'Article 8: the President appoints the Prime Minister.',
                                             'gameplay_assumption': 'fixture', 'sources': ['https://example.org/constitution']}],
             'lifecycle_disclosures': []}
    countries = [{'id': i, 'party_rows': {'France': 1, 'SouthAfrica': 2, 'India': 1}.get(i, 0)} for i in IDENTITIES]
    census = {'certified_country_cases': CASES, 'certified_identity_ids': IDENTITIES,
              'historical_from': ledger.PERIOD_FROM, 'existing_research_cutoff': ledger.CUTOFF}
    index = {'countries': [{'nation': n, 'packet': f'docs/campaign-certification/C01/research/{n.lower()}.json'} for n in packets],
             'work_orders': []}
    queue = {'tasks': [{'id': tid, 'owner': 'Claude', 'state': 'claimed', 'branch': f'claude/{tid.lower()}'}
                       for tid in ledger.IN_FLIGHT]}
    record = '| Packet | Reviewed head |\n|---|---|\n| C01-16 | `' + HEX + '` |\n'
    files = {
        ledger.C01 / 'census.json': census, ledger.C01 / 'countries.json': countries,
        ledger.C01 / 'represented-organizations.json': represented, ledger.C01 / 'roles-and-lifecycle.json': roles,
        ledger.C01 / 'work-orders.json': [{'id': 'CH-France-B001', 'members': ['France/fr_udf', 'France/fr_udf/fr_udf_cds']}],
        ledger.C01 / 'research-index.json': index, ledger.SOURCE_AUDIT: [{'packet': 16, 'own_changed_paths': []}],
        ledger.TASK_QUEUE: queue}
    for nation, data in packets.items():
        files[Path(f'docs/campaign-certification/C01/research/{nation.lower()}.json')] = data
    return files, record, attribution


class FixtureLedger(unittest.TestCase):
    def setUp(self):
        self.root = Path(tempfile.mkdtemp(prefix='gap-ledger-'))
        self.addCleanup(shutil.rmtree, self.root, ignore_errors=True)
        files, record, self.attribution = fixture_tree()
        for path, data in files.items():
            (self.root / path).parent.mkdir(parents=True, exist_ok=True)
            (self.root / path).write_text(json.dumps(data, ensure_ascii=False, indent=2), encoding='utf-8')
        (self.root / ledger.INTEGRATION_RECORD).parent.mkdir(parents=True, exist_ok=True)
        (self.root / ledger.INTEGRATION_RECORD).write_text(record, encoding='utf-8')
        (self.root / ledger.INTEGRATIONS / 'CLAUDE-C01-01').mkdir(parents=True, exist_ok=True)
        (self.root / ledger.INTEGRATIONS / 'CLAUDE-C01-01' / 'README.md').write_text(
            '**Decision: accepted as bounded research intake.**\n', encoding='utf-8')
        self.result = ledger.build(self.root, self.attribution)
        self.cases = {c['case']: c for c in self.result['cases']}

    def chain(self, case, cid):
        return next(c for c in self.cases[case]['party_chains'] if c['id'] == cid)

    def role(self, case, rid):
        return next(r for r in self.cases[case]['research_roles'] if r['id'] == rid)

    def items(self):
        return [i for b in self.result['next_batches'] for i in b['items']]

    def complete_source_repair(self, tid):
        spec = ledger.SOURCE_REPAIRS[tid]
        queue_path = self.root / ledger.TASK_QUEUE
        queue = json.loads(queue_path.read_text(encoding='utf-8'))
        next(t for t in queue['tasks'] if t['id'] == tid)['state'] = 'complete'
        queue_path.write_text(json.dumps(queue), encoding='utf-8')
        integration = self.root / ledger.INTEGRATION_RECORD
        integration.write_text(integration.read_text(encoding='utf-8')
                               + f'| C01-{tid[-2:]} | `{HEX}` |\n', encoding='utf-8')
        folder = self.root / ledger.INTEGRATIONS / tid
        folder.mkdir(parents=True)
        (folder / 'README.md').write_text('Bounded source identity accepted; parent remains pending.\n', encoding='utf-8')
        payload = b'Independently verified fixture source identity.\n'
        (folder / 'receipt.txt').write_bytes(payload)
        review = {'format': 'spheres-research-review/v1', 'task': tid, 'status': 'accepted_for_integration',
                  'reviewer': 'Codex', 'reviewed_commit': HEX, 'decision': 'Bounded source identity only.',
                  'evidence': [{'path': 'receipt.txt', 'bytes': len(payload),
                                'sha256': hashlib.sha256(payload).hexdigest()}]}
        if 'source_snapshots' in spec:
            for sid, relative in spec['source_snapshots'].items():
                snapshot = folder / relative
                snapshot.parent.mkdir(parents=True, exist_ok=True)
                data = json.dumps({'source_id': sid}).encode('utf-8')
                snapshot.write_bytes(data)
                review['evidence'].append({'path': relative, 'bytes': len(data),
                                           'sha256': hashlib.sha256(data).hexdigest()})
        else:
            snapshot = self.root / spec['snapshot']
            snapshot.parent.mkdir(parents=True, exist_ok=True)
            snapshot.write_text('{}\n', encoding='utf-8')
        (folder / 'manifest.json').write_text(json.dumps(review), encoding='utf-8')
        return folder

    def test_completed_source_repairs_are_pinned_without_accepting_parent_or_adding_work(self):
        for tid in ledger.SOURCE_REPAIRS:
            self.complete_source_repair(tid)
        result = ledger.build(self.root, self.attribution)
        self.assertEqual(result['next_batches'], self.result['next_batches'])
        self.assertEqual(result['totals'], self.result['totals'])
        self.assertFalse(set(ledger.SOURCE_REPAIRS) & {t['task'] for t in result['in_flight']})
        self.assertEqual({r['task'] for r in result['completed_source_repairs']}, set(ledger.SOURCE_REPAIRS))
        for repair in result['completed_source_repairs']:
            spec = ledger.SOURCE_REPAIRS[repair['task']]
            expected_sources = list(spec['source_snapshots']) if 'source_snapshots' in spec else [spec['source']]
            self.assertEqual(repair['source_ids'], expected_sources)
            self.assertEqual(repair['parent_evidence_class'], 'c01_integrated_pending')
            self.assertFalse(repair['parent_acceptance_changed'])
            self.assertFalse(repair['historical_coverage_changed'])
            self.assertTrue(all(e in result['inputs'] for e in repair['evidence']))
            self.assertTrue(any(e['path'].endswith('/manifest.json') for e in repair['evidence']))
            self.assertTrue(any(e['path'].endswith('/receipt.txt') for e in repair['evidence']))
        self.assertIn('Completed source repairs', ledger.render(result))

    def test_group_source_repair_requires_every_reviewed_snapshot(self):
        tid = 'CLAUDE-C01-SOURCE-26'
        folder = self.complete_source_repair(tid)
        manifest = folder / 'manifest.json'
        review = json.loads(manifest.read_text(encoding='utf-8'))
        missing = review['evidence'].pop()
        manifest.write_text(json.dumps(review), encoding='utf-8')
        with self.assertRaisesRegex(ValueError, 'lacks reviewed extract evidence'):
            ledger.build(self.root, self.attribution)
        review['evidence'].append(missing)
        manifest.write_text(json.dumps(review), encoding='utf-8')
        (folder / missing['path']).write_bytes(b'Changed reviewed extract')
        with self.assertRaisesRegex(ValueError, 'Source repair evidence changed'):
            ledger.build(self.root, self.attribution)

    def test_completed_source_repair_requires_accepted_intact_review(self):
        tid = 'CLAUDE-C01-SOURCE-05'
        folder = self.complete_source_repair(tid)
        manifest = folder / 'manifest.json'
        review = json.loads(manifest.read_text(encoding='utf-8'))
        review['status'] = 'ready_for_review'
        manifest.write_text(json.dumps(review), encoding='utf-8')
        with self.assertRaisesRegex(ValueError, 'lacks a bounded accepted review'):
            ledger.build(self.root, self.attribution)
        review['status'] = 'accepted_for_integration'
        manifest.write_text(json.dumps(review), encoding='utf-8')
        (folder / 'receipt.txt').write_bytes(b'Changed receipt')
        with self.assertRaisesRegex(ValueError, 'Source repair evidence changed'):
            ledger.build(self.root, self.attribution)
        manifest.unlink()
        with self.assertRaisesRegex(ValueError, 'Missing completed source repair review'):
            ledger.build(self.root, self.attribution)

    def test_research_packet_completion_still_requires_explicit_review_rule(self):
        queue_path = self.root / ledger.TASK_QUEUE
        queue = json.loads(queue_path.read_text(encoding='utf-8'))
        next(t for t in queue['tasks'] if t['id'] == 'CLAUDE-C01-23')['state'] = 'complete'
        queue_path.write_text(json.dumps(queue), encoding='utf-8')
        with self.assertRaisesRegex(ValueError, 'needs an explicit ledger review rule'):
            ledger.build(self.root, self.attribution)

    def test_completed_research_intake_removes_pending_status_but_preserves_registry_gaps(self):
        tid = 'CLAUDE-C01-27'
        queue_path = self.root / ledger.TASK_QUEUE
        queue = json.loads(queue_path.read_text(encoding='utf-8'))
        task = next(t for t in queue['tasks'] if t['id'] == tid)
        task.update(state='complete', review_revision=HEX)
        queue_path.write_text(json.dumps(queue), encoding='utf-8')
        review = self.root / ledger.INTEGRATIONS / tid / 'README.md'
        review.parent.mkdir()
        review.write_text('Accepted as a bounded research intake. No runtime mapping or period completeness.\n', encoding='utf-8')
        result = ledger.build(self.root, self.attribution)
        self.assertNotIn(tid, {row['task'] for row in result['in_flight']})
        self.assertEqual(len(result['completed_research_intakes']), 1)
        intake = result['completed_research_intakes'][0]
        self.assertEqual(intake['task'], tid)
        self.assertFalse(intake['runtime_mapping_accepted'])
        self.assertFalse(intake['historical_period_complete'])
        self.assertEqual(intake['integration_revision'], HEX)
        self.assertIn(ledger.sha(review.relative_to(self.root), self.root), result['inputs'])
        chain = next(chain for case in result['cases'] for chain in case['party_chains'] if chain['id'] == 'India/in_bjp')
        self.assertIsNone(chain['in_flight'])
        self.assertEqual(chain['terms'], self.chain('India', 'India/in_bjp')['terms'])
        self.assertEqual(chain['coverage'], self.chain('India', 'India/in_bjp')['coverage'])
        self.assertIn('Completed research intakes', ledger.render(result))

    def test_completed_intake_requires_pinned_review_and_cannot_borrow_another_acceptance(self):
        task = {'id': 'CLAUDE-C01-27', 'review_revision': HEX}
        with self.assertRaisesRegex(ValueError, 'explicit ledger review rule'):
            ledger.completed_research_intake(task, {'CLAUDE-C01-27': 'c01_accepted'}, self.root)
        review = self.root / ledger.INTEGRATIONS / task['id'] / 'README.md'
        review.parent.mkdir()
        review.write_text('Accepted as a bounded research intake.\n', encoding='utf-8')
        for revision in (None, '', 'not-a-revision', 'A' * 40):
            with self.subTest(revision=revision), self.assertRaisesRegex(ValueError, 'pinned integration review revision'):
                ledger.completed_research_intake({**task, 'review_revision': revision}, ledger.acceptance_classes(self.root), self.root)

    def test_all_eight_cases_and_both_ussr_russia_identities(self):
        self.assertEqual([c['case'] for c in self.result['cases']], CASES)
        self.assertEqual(self.cases['USSR -> Russia']['identities'], ['USSR', 'Russia'])
        self.assertEqual(self.result['period'], {'from': '1990-01-01', 'through': '2026-09-07', 'cutoff_frozen': True})

    def test_executive_observations_never_count_as_party_leader_coverage(self):
        new = self.chain('SouthAfrica', 'SouthAfrica/za_new')
        self.assertEqual(new['coverage']['days']['definite'], 0)
        self.assertEqual(new['coverage']['unresolved_days'], new['coverage']['window_days'])
        anc = self.chain('SouthAfrica', 'SouthAfrica/za_anc')
        self.assertEqual({t['record'] for t in anc['terms']}, {'za_anc_mandela', 'za_anc_mbeki'})
        # The research presidency (an executive office) and the seed executive observation stay out of the chain.
        self.assertNotIn('1994-05-10', json.dumps(anc['terms']))
        self.assertEqual(self.cases['SouthAfrica']['totals']['seed_executive_observations'], 1)
        people = self.cases['SouthAfrica']['cross_role_people']
        self.assertEqual([p['person_key'] for p in people], ['nelsonmandela'])
        self.assertEqual(people[0]['office_roles'], [['za_presidency', 'za_president_election']])

    def test_research_organization_is_an_uncertain_candidate_not_coverage(self):
        anc = self.chain('SouthAfrica', 'SouthAfrica/za_anc')
        self.assertEqual(anc['uncertain_research_candidates'],
                         [{'organization': 'za_org_anc', 'name': 'AFRICAN NATIONAL CONGRESS', 'roles': ['za_anc_president'],
                           'counted': False, 'mapped_by_packet': False}])
        # Mandela's 1995 research attestation falls inside the census interval, but the chain's days come from terms only.
        self.assertEqual(anc['coverage']['days']['attested_day'], 0)
        folded = [i for i in self.items() if i['entity'] == 'za_org_anc#za_anc_president']
        self.assertEqual(folded, [])  # folded into the represented row's item as a candidate, never a separate chain

    def test_alias_collisions_are_reported_and_never_merged(self):
        collisions = {tuple(m['id'] for m in c['members']): c for c in self.cases['SouthAfrica']['alias_collisions']}
        self.assertEqual(collisions[('za_org_ff1', 'za_org_ff2')]['classification'], 'same_name_distinct_ids')
        self.assertEqual(collisions[('SouthAfrica/za_anc', 'za_org_anc')]['classification'], 'uncertain_registry_research_match')
        self.assertTrue(all(c['merged'] is False for c in collisions.values()))

    def test_missing_and_imprecise_dates_are_named_exactly(self):
        mbeki = next(t for t in self.chain('SouthAfrica', 'SouthAfrica/za_anc')['terms'] if t['record'] == 'za_anc_mbeki')
        self.assertEqual(mbeki['missing_fields'], ['until (known only to year)'])
        advani = self.chain('India', 'India/in_bjp')['terms'][0]
        self.assertEqual(advani['missing_fields'], ['from (known only to year)', 'until'])
        gorbachev = self.role('USSR -> Russia', 'su_president')['holders'][0]
        self.assertEqual(gorbachev['missing_fields'], ['from', 'until'])
        speaker = self.role('Tonga', 'to_speaker')['holders'][0]
        self.assertEqual(speaker['missing_fields'], ['holder identity and dates (claim-only observation)'])
        segments = self.chain('SouthAfrica', 'SouthAfrica/za_anc')['coverage']['segments']
        # Inclusive ends: a term ending 'in 2007' is definitely held through its earliest possible last day.
        self.assertIn({'from': '1991-07-05', 'through': '2007-01-01', 'status': 'definite'}, segments)
        self.assertIn({'from': '2007-01-02', 'through': '2007-12-31', 'status': 'boundary_imprecise'}, segments)

    def test_pending_accepted_and_in_flight_evidence_stay_distinct(self):
        self.assertEqual(self.role('SouthAfrica', 'za_president_election')['holders'][0]['evidence'], ['c01_accepted'])
        self.assertEqual(self.role('SouthAfrica', 'za_anc_president')['holders'][0]['evidence'], ['c01_integrated_pending'])
        self.assertEqual(self.role('USSR -> Russia', 'su_president')['holders'][0]['evidence'], ['s10_discovery_intake'])
        self.assertTrue(all(row['accepted'] is False for row in self.result['in_flight']))
        entities = {i['entity'] for i in self.items()}
        for excluded in ('India/in_bjp', 'to_legislative_assembly#to_speaker', 'sa_shura#sa_shura_chair', 'institution:fr_presidency'):
            self.assertNotIn(excluded, entities)
        self.assertEqual(self.chain('India', 'India/in_bjp')['in_flight'], 'CLAUDE-C01-27')

    def test_submitted_deputy_role_reserves_only_that_role_without_granting_coverage(self):
        path = self.root / ledger.RESEARCH / 'tonga.json'
        data = json.loads(path.read_text(encoding='utf-8'))
        data['institutions'].append({'id': 'to_cabinet', 'name': 'Cabinet', 'kind': 'x', 'roles': [
            {'id': 'to_deputy_pm', 'title': 'Deputy Prime Minister', 'kind': 'institutional_office',
             'sources': ['s_to'], 'holder_claims': []},
            {'id': 'to_pm', 'title': 'Prime Minister', 'kind': 'head_of_government',
             'sources': ['s_to'], 'holder_claims': []}]})
        path.write_text(json.dumps(data), encoding='utf-8')
        result = ledger.build(self.root, self.attribution)
        roles = {r['id']: r for c in result['cases'] for r in c['research_roles'] if r['nation'] == 'Tonga'}
        self.assertEqual(roles['to_deputy_pm']['in_flight'], 'CLAUDE-C01-36')
        self.assertEqual(roles['to_deputy_pm']['coverage']['days']['definite'], 0)
        self.assertIsNone(roles['to_pm']['in_flight'])
        entities = {i['entity'] for b in result['next_batches'] for i in b['items']}
        self.assertNotIn('to_cabinet#to_deputy_pm', entities)
        self.assertIn('to_cabinet#to_pm', entities)
        self.assertFalse(next(r for r in result['in_flight'] if r['task'] == 'CLAUDE-C01-36')['accepted'])

    def test_acceptance_requires_an_explicit_decision_not_a_directory(self):
        review = self.root / ledger.INTEGRATIONS / 'CLAUDE-C01-01' / 'README.md'
        review.unlink()
        with self.assertRaisesRegex(ValueError, 'Missing explicit acceptance record'):
            ledger.build(self.root, self.attribution)
        review.write_text('Ready for review. Acceptance is pending.\n', encoding='utf-8')
        with self.assertRaisesRegex(ValueError, 'No explicit accepted decision'):
            ledger.build(self.root, self.attribution)

    def test_acceptance_records_are_pinned_and_changes_stale_the_ledger(self):
        relative = ledger.INTEGRATIONS / 'CLAUDE-C01-01' / 'README.md'
        self.assertIn(ledger.sha(relative, self.root), self.result['inputs'])
        before = ledger.outputs(self.root, self.attribution)
        ledger.write(before, self.root)
        review = self.root / relative
        review.write_text(review.read_text(encoding='utf-8') + 'Scope clarification.\n', encoding='utf-8')
        after = ledger.outputs(self.root, self.attribution)
        self.assertIn((ledger.OUTPUT / 'ledger.json').as_posix(), ledger.stale(after, self.root))

    def test_markdown_input_hashes_ignore_checkout_line_endings(self):
        relative = ledger.INTEGRATION_RECORD
        review = self.root / relative
        lf = review.read_text(encoding='utf-8').encode('utf-8')
        review.write_bytes(lf)
        before = ledger.outputs(self.root, self.attribution)
        review.write_bytes(lf.replace(b'\n', b'\r\n'))
        self.assertEqual(ledger.outputs(self.root, self.attribution), before)
        self.assertEqual(ledger.sha(relative, self.root)['hash_encoding'], 'UTF-8 text with LF line endings')
        self.assertNotIn('hash_encoding', ledger.sha(ledger.C01 / 'census.json', self.root))

    def test_components_never_fill_the_parent_chain(self):
        parent = self.chain('France', 'France/fr_udf')
        component = self.chain('France', 'France/fr_udf/fr_udf_cds')
        self.assertGreater(component['coverage']['days']['definite'], 0)
        self.assertEqual(parent['coverage']['days']['definite'], 0)
        item = next(i for i in self.items() if i['entity'] == 'France/fr_udf')
        self.assertEqual(item['components_with_terms'], ['France/fr_udf/fr_udf_cds (1 terms)'])
        self.assertIn('does not fill this parent chain', item['note'])
        self.assertIn('not bounded', next(i for i in self.items() if i['entity'] == 'France/fr_udf/fr_udf_cds')['note'])

    def test_windows_exceptions_and_missing_offices(self):
        self.assertEqual(self.role('USSR -> Russia', 'su_president')['window']['until'], '1991-12-25')
        self.assertEqual(self.role('SouthAfrica', 'za_president_election')['window']['from'], '1994-05-09')
        kinds = {(e['kind'], e.get('role') or e.get('institution')) for c in self.result['cases'] for e in c['institutional_exceptions']}
        self.assertIn(('hereditary_office', 'to_king'), kinds)
        self.assertIn(('collective_institution', 'sa_shura_members'), kinds)
        self.assertIn(('no_simulation_party_rows', None), kinds)
        missing = [i for i in self.items() if i['item'] == 'missing_institution']
        self.assertEqual(missing, [])  # The submitted C01-37 reserves this missing institution, not its history.
        queue_path = self.root / ledger.TASK_QUEUE
        queue = json.loads(queue_path.read_text(encoding='utf-8'))
        queue['tasks'] = [t for t in queue['tasks'] if t['id'] != 'CLAUDE-C01-37']
        queue_path.write_text(json.dumps(queue), encoding='utf-8')
        without_claim = {tid: spec for tid, spec in ledger.IN_FLIGHT.items() if tid != 'CLAUDE-C01-37'}
        with patch.dict(ledger.IN_FLIGHT, without_claim, clear=True):
            unclaimed = ledger.build(self.root, self.attribution)
        missing = [i for b in unclaimed['next_batches'] for i in b['items'] if i['item'] == 'missing_institution']
        self.assertEqual([i['entity'] for i in missing], ['institution:fr_prime_minister'])
        self.assertEqual(missing[0]['leads'], ['https://example.org/constitution'])

    def test_batches_are_bounded_and_every_item_names_missing_fields_and_leads(self):
        for batch in self.result['next_batches']:
            self.assertLessEqual(len(batch['items']), ledger.MAX_BATCH)
            for item in batch['items']:
                self.assertTrue(item['missing_fields'])
                self.assertTrue(item['leads'])
        self.assertTrue(all(b['id'].startswith('GAP-') for b in self.result['next_batches']))

    def test_regeneration_is_deterministic_and_check_detects_stale_output(self):
        first = ledger.outputs(self.root, self.attribution)
        self.assertEqual(first, ledger.outputs(self.root, copy.deepcopy(self.attribution)))
        ledger.write(first, self.root)
        self.assertEqual(ledger.stale(first, self.root), [])
        path = self.root / ledger.OUTPUT / 'ledger.md'
        path.write_text(path.read_text(encoding='utf-8') + 'edited\n', encoding='utf-8')
        self.assertEqual(ledger.stale(first, self.root), [(ledger.OUTPUT / 'ledger.md').as_posix()])

    def test_unattributed_sources_unknown_packets_and_queue_drift_fail_loudly(self):
        broken = copy.deepcopy(self.attribution)
        del broken['sources']['USSR']['s_su']
        with self.assertRaisesRegex(ValueError, 'no pinned attribution'):
            ledger.build(self.root, broken)
        broken = copy.deepcopy(self.attribution)
        broken['sources']['USSR']['s_su']['packet'] = 'CLAUDE-C01-99'
        with self.assertRaisesRegex(ValueError, 'no acceptance class'):
            ledger.build(self.root, broken)
        queue = json.loads((self.root / ledger.TASK_QUEUE).read_text(encoding='utf-8'))
        queue['tasks'].append({'id': 'CLAUDE-C01-99', 'owner': 'Claude', 'state': 'claimed'})
        (self.root / ledger.TASK_QUEUE).write_text(json.dumps(queue), encoding='utf-8')
        with self.assertRaisesRegex(ValueError, 'In-flight table differs'):
            ledger.build(self.root, self.attribution)


class CoverageUnits(unittest.TestCase):
    WINDOW = {'from': '1990-01-01', 'until': '1990-12-31', 'pinned': False, 'anchors': [], 'note': None}

    def entry(self, **kw):
        row = {'kind': 'holder', 'from': None, 'until': None, 'points': [], 'periods': []}
        row.update(kw)
        return row

    def test_year_and_month_precision_trim_definite_coverage(self):
        cov = ledger.coverage([self.entry(**{'from': ('1990-03-01', '1990-03-31'), 'until': ('1990-06-01', '1990-06-30')})], self.WINDOW)
        segments = [s for s in cov['segments'] if s['status'] != 'no_evidence']
        self.assertEqual(segments, [
            {'from': '1990-03-01', 'through': '1990-03-30', 'status': 'boundary_imprecise'},
            {'from': '1990-03-31', 'through': '1990-06-01', 'status': 'definite'},
            {'from': '1990-06-02', 'through': '1990-06-30', 'status': 'boundary_imprecise'}])

    def test_isolated_attestation_and_open_start_never_become_intervals(self):
        cov = ledger.coverage([self.entry(points=['1990-05-05']), self.entry(**{'from': ('1990-11-01', '1990-11-01')})], self.WINDOW)
        self.assertEqual(cov['days']['definite'], 0)
        self.assertEqual(cov['days']['attested_day'], 1)
        self.assertEqual(cov['days']['open_end'], 61)
        self.assertEqual(cov['unresolved_days'], 365)

    def test_acting_is_kept_apart_from_substantive_coverage(self):
        cov = ledger.coverage([self.entry(kind='acting', **{'from': ('1990-01-10', '1990-01-10'), 'until': ('1990-01-19', '1990-01-19')})], self.WINDOW)
        self.assertEqual(cov['days']['acting_definite'], 10)
        self.assertEqual(cov['days']['definite'], 0)

    def test_date_kinds(self):
        self.assertEqual(ledger.bounds({'kind': 'month', 'value': '1996-02'}), ('1996-02-01', '1996-02-29'))
        self.assertEqual(ledger.bounds({'kind': 'year', 'value': '2001'}), ('2001-01-01', '2001-12-31'))
        self.assertIsNone(ledger.bounds({'kind': 'open', 'value': None}))
        self.assertIsNone(ledger.bounds({'kind': 'unknown', 'value': None}))
        with self.assertRaisesRegex(ValueError, 'Unsupported'):
            ledger.bounds({'kind': 'decade', 'value': '1990s'})


class CheckedInLedger(unittest.TestCase):
    """The committed ledger against current integration data."""

    @classmethod
    def setUpClass(cls):
        cls.files = ledger.outputs()
        cls.data = json.loads(cls.files[ledger.OUTPUT / 'ledger.json'])

    def test_committed_output_is_current(self):
        self.assertEqual(ledger.stale(self.files), [])

    def test_real_cases_windows_and_in_flight_exclusions(self):
        self.assertEqual([c['case'] for c in self.data['cases']], CASES)
        items = [i for b in self.data['next_batches'] for i in b['items']]
        entities = {i['entity'] for i in items}
        completed = {row['task']: row for row in self.data['completed_research_intakes']}
        self.assertEqual(set(completed), {'CLAUDE-C01-23', 'CLAUDE-C01-24', 'CLAUDE-C01-25',
                                         'CLAUDE-C01-27', 'CLAUDE-C01-30', 'CLAUDE-C01-32', 'CLAUDE-C01-33'})
        self.assertTrue(all(row['runtime_mapping_accepted'] is False and row['historical_period_complete'] is False
                            for row in completed.values()))
        self.assertFalse(set(completed) & {row['task'] for row in self.data['in_flight']})
        claims = {row['task']: row for row in self.data['in_flight']}
        self.assertEqual(set(claims), {f'CLAUDE-C01-{n}' for n in (28, 29, 34, 35, 36, 37)})
        chains = {row['id']: row for case in self.data['cases'] for row in case['party_chains']}
        for tid, claim in claims.items():
            self.assertEqual(claim['state'], 'ready_for_review')
            self.assertFalse(claims[tid]['accepted'])
            for target in claim['targets']:
                if target.startswith('party:'):
                    entity = target.removeprefix('party:')
                    self.assertEqual(chains[entity]['in_flight'], tid)
                    self.assertNotIn(entity, entities)
        self.assertNotIn('institution:fr_prime_minister', entities)
        self.assertNotIn('to_cabinet#to_deputy_pm', entities)
        self.assertTrue(all(len(b['items']) <= ledger.MAX_BATCH for b in self.data['next_batches']))
        roles = {(r['nation'], r['id']): r for c in self.data['cases'] for r in c['research_roles']}
        self.assertEqual(roles[('USSR', 'su_president')]['window']['until'], '1991-12-25')
        self.assertEqual(roles[('Russia', 'ru_president')]['window']['from'], '1991-12-25')
        self.assertEqual(roles[('Tonga', 'to_deputy_pm')]['in_flight'], 'CLAUDE-C01-36')

    def test_party_chains_hold_only_their_own_census_terms(self):
        census = json.loads((ledger.ROOT / ledger.C01 / 'roles-and-lifecycle.json').read_text(encoding='utf-8'))
        owner = {t['id']: t['organization_id'] for t in census['term_records']}
        for case in self.data['cases']:
            for chain in case['party_chains']:
                self.assertTrue(all(owner[t['record']] == chain['id'] for t in chain['terms']))
                self.assertTrue(all(c['counted'] is False for c in chain['uncertain_research_candidates']))

    def test_every_certified_source_is_attributed_to_a_classified_packet(self):
        attribution = json.loads((ledger.ROOT / ledger.ATTRIBUTION).read_text(encoding='utf-8'))
        classes = ledger.acceptance_classes()
        for nation, path in ledger.certified_packets().items():
            packet = json.loads((ledger.ROOT / path).read_text(encoding='utf-8'))
            self.assertEqual(set(attribution['sources'][nation]), {s['id'] for s in packet['sources']})
            self.assertTrue(all(row['packet'] in classes for row in attribution['sources'][nation].values()))
        self.assertEqual(classes['CLAUDE-C01-06'], 'c01_integrated_pending')  # renumbered from its first-commit label 03
        self.assertEqual(classes['CLAUDE-C01-03'], 'c01_accepted')
        expected = {'CLAUDE-C01-23': ('09b27c49', 49), 'CLAUDE-C01-24': ('6af1e942', 49),
                    'CLAUDE-C01-25': ('61a3402d', 71), 'CLAUDE-C01-27': ('644ce003', 74),
                    'CLAUDE-C01-30': ('1b2c1ae2', 39), 'CLAUDE-C01-32': ('62f6be6c', 38)}
        for packet, (commit, count) in expected.items():
            rows = [row for sources in attribution['sources'].values() for row in sources.values() if row['packet'] == packet]
            self.assertEqual(len(rows), count)
            self.assertTrue(all(row['commit'] == commit and row['via'] == 'extract_first_added' for row in rows))
            self.assertEqual(classes[packet], 'c01_accepted')


if __name__ == '__main__':
    unittest.main()
