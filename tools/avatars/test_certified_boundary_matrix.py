"""CLAUDE-S23-MATRIX-01: boundary matrix semantics on fixtures, then the real committed output.

Fixture trees are synthetic and live only in temporary directories; their
assertions never touch the committed matrix. The final class checks the actual
production observations that the committed matrix records.
"""
import contextlib
from datetime import date
import gzip
import io
import hashlib
import json
from pathlib import Path
import shutil
import tempfile
import unittest
from unittest.mock import patch

import certified_boundary_matrix as matrix

DAY = lambda value: {'kind': 'day', 'value': value}
OPEN = {'kind': 'open'}
PNG = b'\x89PNG\r\n\x1a\n'


def png(name):
    return PNG + name.encode()


def sha(data):
    return hashlib.sha256(data).hexdigest()


def term(tid, person, kind, start, until, component=None):
    row = {'id': tid, 'person': person, 'role': 'Fixture leader', 'kind': kind,
           'from': start, 'until': until, 'affiliation_from': start, 'affiliation_until': until,
           'sources': ['fixture']}
    if component:
        row['component'] = component
    return row


def record(person, start, end, asset, **overrides):
    row = {'from': start, 'to': end, 'method': 'generated', 'style': 'cartoon',
           'status': 'illustrated-likeness', 'asset': matrix.PORTRAIT_PREFIX + asset,
           'sha256': sha(png(asset)),
           'identity_source': {'person_id': person},
           'review': {'identity': True, 'likeness': True, 'era': True, 'visual': True}}
    row.update(overrides)
    return row


def fictional_record(person, seed, asset, **overrides):
    row = {'from': '2026-09-08', 'to': '2036-01-01', 'method': 'generated', 'style': 'cartoon',
           'status': 'fictional-character', 'asset': matrix.PORTRAIT_PREFIX + asset, 'sha256': sha(png(asset)),
           'design_source': {'kind': 'authored_fiction', 'person_id': person, 'appearance_seed': seed,
                             'catalog': matrix.FUTURE_CATALOG},
           'review': {'design': True, 'visual': True}, 'identity_source': None, 'source_url': None}
    row.update(overrides)
    return row


def fixture_files():
    """A tiny repository: Alpha (parties + research), Beta -> Gamma (transition)."""
    people = [
        {'id': 'p_old', 'name': 'Old Leader', 'born': DAY('1930-01-01'), 'died': DAY('1995-06-10'), 'sources': ['f']},
        {'id': 'p_new', 'name': 'New Leader', 'sources': ['f']},
        {'id': 'p_act', 'name': 'Acting Person', 'sources': ['f']},
        {'id': 'p_co1', 'name': 'Co One', 'sources': ['f']},
        {'id': 'p_co2', 'name': 'Co Two', 'sources': ['f']},
        {'id': 'p_x', 'name': 'Gone Leader', 'sources': ['f']},
        {'id': 'p_sha', 'name': 'Hash Person', 'sources': ['f']},
        {'id': 'p_exec', 'name': 'Exec Person', 'native': 'Exec Native', 'sources': ['f']},
        {'id': 'p_wrong', 'name': 'Misfiled Person', 'sources': ['f']},
        {'id': 'p_dies', 'name': 'Mortal Leader', 'died': DAY('2003-03-03'), 'sources': ['f']},
    ]
    parties = [
        {'nation': 'Alpha', 'party': 'al_main', 'kind': 'party', 'coverage': 'partial', 'components': [],
         'terms': [term('al_main_old', 'p_old', 'leader', DAY('1985-01-01'), DAY('1995-06-10')),
                   term('al_main_new', 'p_new', 'leader', DAY('1995-06-10'), OPEN),
                   term('al_main_act', 'p_act', 'acting', DAY('1989-06-01'), DAY('1990-01-02'))],
         'gaps': [{'from': DAY('2000-01-01'), 'until': DAY('2000-12-31'), 'reason': 'fixture gap'}]},
        {'nation': 'Alpha', 'party': 'al_duo', 'kind': 'party', 'coverage': 'partial', 'components': [],
         'terms': [term('al_duo_1', 'p_co1', 'co_leader', DAY('1988-01-01'), OPEN),
                   term('al_duo_2', 'p_co2', 'co_leader', DAY('1988-01-01'), OPEN)], 'gaps': []},
        {'nation': 'Alpha', 'party': 'al_gone', 'kind': 'party', 'coverage': 'partial', 'components': [],
         'dissolved': DAY('2001-03-04'),
         'terms': [term('al_gone_t', 'p_x', 'leader', DAY('1980-01-01'), DAY('2001-03-04'))], 'gaps': []},
        {'nation': 'Alpha', 'party': 'al_holey', 'kind': 'party', 'coverage': 'partial', 'components': [],
         'terms': [term('al_holey_t', 'p_sha', 'leader', DAY('1990-01-01'), DAY('1992-01-01')),
                   term('al_holey_w', 'p_wrong', 'leader', DAY('1992-01-01'), DAY('1994-01-01'))], 'gaps': []},
        {'nation': 'Alpha', 'party': 'al_blank', 'kind': 'party', 'coverage': 'gap', 'components': [],
         'terms': [], 'gaps': []},
        # An open term that only a recorded death can end.
        {'nation': 'Alpha', 'party': 'al_mortal', 'kind': 'party', 'coverage': 'partial', 'components': [],
         'terms': [term('al_mortal_t', 'p_dies', 'leader', DAY('2000-01-01'), OPEN)], 'gaps': []},
        {'nation': 'Beta', 'party': 'be_unknown', 'kind': 'unknown', 'coverage': 'gap', 'components': [],
         'terms': [], 'gaps': []},
    ]
    registry = {'version': 1, 'reference_from': '1990-01-01', 'reference_through': '2026-09-07',
                'people': people, 'parties': parties,
                'office_links': [{'nation': 'Alpha', 'person': 'p_exec', 'since': '1988-01-01', 'sources': ['f']}]}
    leaders = {'version': 1, 'rows': [
        {'nation': 'Alpha', 'name': 'Exec Person', 'office': 'Prime Minister', 'since': '1988-01-01', 'tie': {'party': 'al_main'}},
        {'nation': 'Beta', 'name': 'Beta Chief', 'office': 'Chairman', 'since': '1980-01-01', 'tie': None}]}
    portraits = {'version': 1, 'people': {
        'p_old': {'name': 'Old Leader', 'portraits': [record('p_old', '1990-01-01', '1996-01-01', 'p-old-cartoon.png')]},
        'p_new': {'name': 'New Leader', 'portraits': []},
        'p_act': {'name': 'Acting Person', 'portraits': [record('p_act', '1990-01-01', '1995-01-01', 'p-act-a.png'),
                                                         record('p_act', '1989-01-01', '1991-01-01', 'p-act-b.png')]},
        'p_co1': {'name': 'Co One', 'portraits': [record('someone_else', '1990-01-01', '2000-01-01', 'p-co1-cartoon.png')]},
        # The wrong person's image, and not fully reviewed: it must never bind.
        'p_co2': {'name': 'Co Two', 'portraits': [record('p_co2', '1990-01-01', '2000-01-01', 'p-old-cartoon.png',
                                                         review={'identity': True, 'likeness': True, 'era': True, 'visual': False})]},
        'p_x': {'name': 'Gone Leader', 'portraits': [record('p_x', '1990-01-01', '2002-01-01', 'p-x-cartoon.png')]},
        'p_sha': {'name': 'Hash Person', 'portraits': [record('p_sha', '1990-01-01', '1995-01-01', 'p-sha-cartoon.png', sha256='0' * 64)]},
        'p_exec': {'name': 'Exec Person', 'portraits': [record('p_exec', '1990-01-01', '1991-01-01', 'p-exec-1990.png'),
                                                        record('p_exec', '1991-01-01', '1992-01-01', 'p-exec-1991.png')]},
        # Fully reviewed and served, but the file is named for somebody else.
        'p_wrong': {'name': 'Misfiled Person', 'portraits': [record('p_wrong', '1990-01-01', '1995-01-01', 'someone-else-cartoon.png')]},
    }}
    candidates = [
        {'person_id': 'fictional_v1_alpha_al_main_main_01', 'name': 'Future One', 'appearance_seed': 'seed01',
         'nation': 'Alpha', 'party': 'al_main', 'component': None, 'component_available': True,
         'executive_eligibility': {'authorized': False}},
        {'person_id': 'fictional_v1_alpha_al_main_main_02', 'name': 'Future Two', 'appearance_seed': 'seed02',
         'nation': 'Alpha', 'party': 'al_main', 'component': None, 'component_available': True,
         'executive_eligibility': {'authorized': True}},
    ]
    fictional = {'version': 1, 'people': {
        'fictional_v1_alpha_al_main_main_01': {'name': 'Future One', 'appearance_seed': 'seed01', 'portraits': [
            fictional_record('fictional_v1_alpha_al_main_main_01', 'seed01', 'fictional-alpha-01.png')]},
        'fictional_v1_alpha_al_main_main_02': {'name': 'Renamed Elsewhere', 'appearance_seed': 'seed02', 'portraits': [
            fictional_record('fictional_v1_alpha_al_main_main_02', 'seed02', 'fictional-alpha-02.png')]},
    }}
    allowlisted = ['p-old-cartoon.png', 'p-act-a.png', 'p-act-b.png', 'p-co1-cartoon.png', 'p-sha-cartoon.png',
                   'p-exec-1990.png', 'p-exec-1991.png', 'someone-else-cartoon.png',
                   'fictional-alpha-01.png', 'fictional-alpha-02.png']
    allowlist = 'fn cartoon_asset(name: &str) -> Option<&\'static [u8]> {\n    match name {\n' + ''.join(
        f'        "{n}" => Some(include_bytes!("../ui/person-portraits/{n}")),\n' for n in allowlisted) + \
        '        _ => None,\n    }\n}\n'
    sources = [
        {'id': 'src_acc', 'claims': [{'id': 'c_acc_1', 'text': 'first'}, {'id': 'c_acc_2', 'text': 'second'}]},
        {'id': 'src_pend', 'claims': [{'id': 'c_pend_1', 'text': 'third'}]},
        {'id': 'src_s10', 'claims': [{'id': 'c_s10_att', 'text': 'Attests the Prime Minister.', 'attested_on': '2005-05-05'}]},
        {'id': 'src_uncl', 'claims': [{'id': 'c_uncl', 'text': 'unclassified'}]},
    ]
    pm_holders = [
        {'name': 'First PM', 'from': None, 'until': '1996-04-02', 'attested_on': '1991-03-03',
         'sources': ['src_acc'], 'claim_ids': ['c_acc_1']},
        {'name': 'Second PM', 'from': '1996-04-02', 'until': '1999-09-09', 'attested_on': None,
         'sources': ['src_acc'], 'claim_ids': ['c_acc_2']},
        {'name': 'Third PM', 'from': '1999-09-09', 'until': None, 'attested_on': None,
         'sources': ['src_pend'], 'claim_ids': ['c_pend_1']},
        'c_s10_att',
        {'name': 'Mixed PM', 'from': None, 'until': None, 'attested_on': '2010-10-10',
         'sources': ['src_acc', 'src_pend'], 'claim_ids': ['c_acc_1', 'c_pend_1']},
        {'name': 'Unclassified PM', 'from': None, 'until': None, 'attested_on': '2012-12-12',
         'sources': ['src_uncl'], 'claim_ids': ['c_uncl']},
    ]
    packet = {'version': 1, 'nation': 'Alpha', 'research_cutoff': '2026-09-07', 'sources': sources,
              'organizations': [],
              'institutions': [
                  {'id': 'al_pm_office', 'name': 'Prime Minister (office)', 'kind': 'executive_office',
                   'lifecycle': {'status': 'unknown', 'from': None, 'until': None},
                   'roles': [{'id': 'al_pm', 'title': 'Prime Minister', 'kind': 'head_of_government', 'holder_claims': pm_holders},
                             {'id': 'al_council', 'title': 'Council members', 'kind': 'collective_seat', 'holder_claims': []}]},
                  {'id': 'al_presidency', 'name': 'Presidency', 'kind': 'head_of_state_office',
                   'lifecycle': {'status': 'creation', 'from': '1991-07-01', 'until': None},
                   'roles': [{'id': 'al_president', 'title': 'President', 'kind': 'head_of_state', 'holder_claims': [
                       {'name': 'Founding President', 'from': None, 'until': None, 'attested_on': '1992-02-02',
                        'sources': ['src_acc'], 'claim_ids': ['c_acc_2']}]}]}]}

    def report(number, source):
        return (f'# Fixture packet {number}\n\nPacket: **CLAUDE-C01-{number}**. State: fixture.\n\n'
                f'## Sources added\n\n| Source ID | What |\n|---|---|\n| `{source}` | fixture source |\n\n## Checks\n\nnone\n')

    files = {
        matrix.CENSUS: {'historical_from': '1990-01-01', 'existing_research_cutoff': '2026-09-07',
                        'fictional_from': '2026-09-08', 'until_exclusive': '2036-01-01',
                        'certified_country_cases': ['Alpha', 'Beta -> Gamma'],
                        'certified_identity_ids': ['Alpha', 'Beta', 'Gamma']},
        matrix.COUNTRIES: [{'id': 'Alpha', 'name': 'Alpha', 'start_1990': True},
                           {'id': 'Beta', 'name': 'Beta', 'start_1990': True},
                           {'id': 'Gamma', 'name': 'Gamma', 'start_1990': False}],
        matrix.ORGANIZATIONS: [{'id': f"{p['nation']}/{p['party']}", 'name': p['party'], 'history_status': p['coverage']}
                               for p in parties],
        matrix.ROLES_AND_LIFECYCLE: {'term_records': [{'id': t['id']} for p in parties for t in p['terms']],
                                     'lifecycle_disclosures': [], 'appearance_eligibility_research_backlog': []},
        matrix.RESEARCH + '/alpha.json': packet,
        matrix.RESEARCH + '/alpha-accepted-31.md': report(31, 'src_acc'),
        matrix.RESEARCH + '/alpha-pending-32.md': report(32, 'src_pend'),
        matrix.RESEARCH + '/alpha-unclassified-33.md': report(33, 'src_uncl'),
        matrix.INTEGRATIONS + '/CLAUDE-C01-31/README.md': '# Fixture\n\n**Decision: accepted as bounded research.**\n',
        # A bounded source repair of packet 32 must not promote the whole packet.
        matrix.INTEGRATIONS + '/CLAUDE-C01-SOURCE-32/README.md': '# Fixture\n\n**Accept this bounded source-identity repair.**\n',
        matrix.PENDING_NOTE: ('# Fixture integration\n\n## Integrated submissions\n\n| Packet | Reviewed head |\n|---|---|\n'
                              f"| C01-32 | `{'a' * 40}` |\n"),
        matrix.REGISTRY: registry,
        matrix.LEADERS_1990: leaders,
        matrix.ELIGIBILITY: {'historical_grants': [], 'future_office_grants': []},
        matrix.PORTRAITS: portraits,
        matrix.FICTIONAL_PORTRAITS: fictional,
        matrix.FIGURES: {'nations': {n: {'figure': f'Emblem {n}', 'years': '1900-1950'} for n in ('Alpha', 'Beta', 'Gamma')}},
        matrix.FUTURE: {'historical_reference_through': '2026-09-07', 'from': '2026-09-08',
                        'until_exclusive': '2036-01-01', 'candidates': candidates},
        matrix.BOARD: {'historical_art_jobs': [{'id': 'cartoon:p_new:1995-06-10:1997-01-01:v1', 'person_id': 'p_new',
                                                'from': '1995-06-10', 'to': '1997-01-01'}],
                       'people': [{'id': 'p_new', 'required_art_windows': [{'from': '1995-06-10', 'to': '1997-01-01'}],
                                   'status': 'art_pending'}]},
        matrix.ALLOWLIST: allowlist,
        matrix.SELF: '# fixture stand-in for the tool source\n',
    }
    for rel in matrix.MIRRORED_SEMANTICS:
        files[rel] = f'// fixture stand-in for {rel}\n'
    # Every allowlisted asset exists except p-exec-1991.png; p-x-cartoon.png exists but is not served.
    for name in allowlisted + ['p-x-cartoon.png']:
        if name != 'p-exec-1991.png':
            files[matrix.PORTRAIT_PREFIX + name] = png(name)
    return files


def write_tree(root, files):
    for rel, value in files.items():
        path = Path(root) / rel
        path.parent.mkdir(parents=True, exist_ok=True)
        if isinstance(value, bytes):
            path.write_bytes(value)
        elif isinstance(value, str):
            path.write_text(value, encoding='utf-8', newline='\n')
        else:
            path.write_text(json.dumps(value, ensure_ascii=False, indent=1) + '\n', encoding='utf-8', newline='\n')


PAIRING = {'Alpha': ('al_pm',), 'Beta': (), 'Gamma': ()}


def quiet_main(argv):
    with contextlib.redirect_stdout(io.StringIO()), contextlib.redirect_stderr(io.StringIO()):
        return matrix.main(argv)


class FixtureTree(unittest.TestCase):
    """Synthetic fixture assertions only; no production observation is read."""

    def setUp(self):
        self.tmp = Path(tempfile.mkdtemp(prefix='s23-matrix-'))
        self.addCleanup(shutil.rmtree, self.tmp, ignore_errors=True)
        write_tree(self.tmp, fixture_files())
        pairing = patch.dict(matrix.EXECUTIVE_PAIRING, PAIRING, clear=True)
        pairing.start()
        self.addCleanup(pairing.stop)
        self.outputs = matrix.build(self.tmp)
        self.alpha = self.outputs['cases-alpha.json']
        self.transition = self.outputs['cases-beta-gamma.json']

    def role(self, key, doc=None):
        return next(r for r in (doc or self.alpha)['roles'] if r['key'] == key)

    def case(self, key, when, doc=None):
        return next(c for c in self.role(key, doc)['cases'] if c['date'] == when)

    def holders(self, key, when, group='holders', doc=None):
        cell = self.case(key, when, doc)
        return [ref for ref, _ in cell['historical'].get(group, [])]


class OffByOneHandovers(FixtureTree):
    def test_production_half_open_handover_on_a_death_day(self):
        before, day_of, after = (self.case('party:al_main', d) for d in ('1995-06-09', '1995-06-10', '1995-06-11'))
        self.assertEqual({'handover_day_before', 'death_day_before'} - set(before['kinds']), set())
        self.assertIn('death_day_of', day_of['kinds'])
        self.assertEqual(self.holders('party:al_main', '1995-06-09'), ['al_main_old'])
        # The outgoing leader died that day: never a holder on the day itself.
        self.assertEqual(self.holders('party:al_main', '1995-06-10'), ['al_main_new'])
        self.assertEqual(self.holders('party:al_main', '1995-06-11'), ['al_main_new'])
        self.assertEqual(after['historical']['status'], 'established')

    def test_plain_production_handover_belongs_to_the_successor_on_the_day(self):
        self.assertEqual(self.holders('party:al_holey', '1991-12-31'), ['al_holey_t'])
        self.assertEqual(self.holders('party:al_holey', '1992-01-01'), ['al_holey_w'])
        self.assertEqual(self.holders('party:al_holey', '1992-01-02'), ['al_holey_w'])
        self.assertIn('handover_day_of', self.case('party:al_holey', '1992-01-01')['kinds'])

    def test_recorded_death_ends_an_open_term(self):
        self.assertEqual(self.holders('party:al_mortal', '2003-03-02'), ['al_mortal_t'])
        for when in ('2003-03-03', '2003-03-04'):
            cell = self.case('party:al_mortal', when)
            self.assertEqual(cell['historical'], {'status': 'unknown', 'reason': 'no_registry_term_covers_date'})
        self.assertIn('death_day_of', self.case('party:al_mortal', '2003-03-03')['kinds'])

    def test_research_boundary_day_reports_both_sides_without_inventing_ends(self):
        self.assertEqual(self.case('research:al_pm', '1996-04-01')['historical']['status'], 'bracketed')
        day_of = self.case('research:al_pm', '1996-04-02')['historical']
        self.assertEqual(day_of['status'], 'boundary_day')
        self.assertEqual(day_of['holders'], [['al_pm#1', 'stated_end_day'], ['al_pm#2', 'stated_start_day']])
        self.assertEqual(self.case('research:al_pm', '1996-04-03')['historical']['holders'],
                         [['al_pm#2', 'within_stated_interval']])
        # A stated start with no stated end stays uncertain; it is a pointer, not an incumbent.
        after = self.case('research:al_pm', '1999-09-10')['historical']
        self.assertEqual(after['status'], 'uncertain')
        self.assertEqual(after['possible'], [['al_pm#3', 'after_stated_start']])
        self.assertNotIn('holders', after)

    def test_isolated_attestation_covers_its_own_day_only(self):
        attested = self.case('research:al_pm', '2005-05-05')
        self.assertIn('attestation_day', attested['kinds'])
        self.assertEqual(attested['historical']['holders'], [['al_pm#4', 'attested_on_day']])
        self.assertEqual(attested['acceptance'], ['unattributed_intake'])
        following = self.case('research:al_pm', '2005-05-06')
        self.assertIn('attestation_day_after', following['kinds'])
        self.assertNotIn('holders', following['historical'])
        self.assertEqual(following['historical']['status'], 'uncertain')


class CoLeadersAndOverlaps(FixtureTree):
    def test_co_leaders_are_both_listed_and_both_seated_at_campaign_start(self):
        cell = self.case('party:al_duo', '1995-01-01')
        self.assertEqual(cell['historical']['multiple_holders'], 'co_leaders')
        self.assertEqual(sorted(i['person'] for i in cell['image']), ['p_co1', 'p_co2'])
        start = self.case('party:al_duo', '1990-01-01')['campaign']
        self.assertEqual(sorted(start['holders']), ['p_co1', 'p_co2'])
        self.assertEqual(start['verdict'], 'campaign_matches_reference')

    def test_acting_overlap_is_reported_and_campaign_seats_by_rank(self):
        cell = self.case('party:al_main', '1990-01-01')
        self.assertEqual(cell['historical']['multiple_holders'], 'overlapping_acting_leader')
        self.assertEqual(cell['campaign'], {'observed': 'fresh_campaign_start', 'holders': ['p_old'],
                                            'verdict': 'campaign_subset_by_rank_selection'})
        self.assertIn('handover_day_before', cell['kinds'])  # the acting term ends on 2 January


class MissingVersusInapplicable(FixtureTree):
    def test_dissolution_makes_a_role_inapplicable_from_the_stated_day(self):
        self.assertEqual(self.holders('party:al_gone', '2001-03-03'), ['al_gone_t'])
        for when in ('2001-03-04', '2001-03-05', '2002-01-01'):
            self.assertEqual(self.case('party:al_gone', when)['historical'],
                             {'status': 'inapplicable', 'reason': 'organization_not_existing'})
        self.assertIn('dissolution_day_of', self.case('party:al_gone', '2001-03-04')['kinds'])

    def test_unknown_unresearched_and_inapplicable_stay_distinct(self):
        self.assertEqual(self.case('party:al_holey', '1995-01-01')['historical']['status'], 'unknown')
        self.assertEqual(self.case('party:al_blank', '1993-01-01')['historical']['status'], 'unresearched')
        beta = self.case('party:be_unknown', '1993-01-01', self.transition)['historical']
        self.assertEqual(beta, {'status': 'unresearched', 'reason': 'party_kind_unknown_no_researched_chain'})
        self.assertEqual(self.case('research:al_council', '1993-01-01')['historical']['status'], 'unresearched')
        # Evidence on both sides of a date interpolates; one isolated attestation does not.
        self.assertEqual(self.case('research:al_pm', '1993-01-01')['historical']['status'], 'bracketed')
        self.assertEqual(self.case('research:al_president', '1993-01-01')['historical'],
                         {'status': 'unknown', 'reason': 'no_observation_covers_date'})
        created = self.case('research:al_president', '1991-06-30')
        self.assertIn('creation_day_before', created['kinds'])
        self.assertEqual(created['historical']['reason'], 'institution_not_yet_created_per_source')
        for key in ('party:al_main', 'research:al_pm', 'executive'):
            self.assertEqual(self.case(key, '1989-12-31')['historical']['reason'], 'before_reference_period')
            self.assertEqual(self.case(key, '2026-09-08')['historical']['reason'], 'after_historical_cutoff')
        self.assertEqual(self.holders('party:al_main', '2026-09-07'), ['al_main_new'])

    def test_successor_identity_has_no_campaign_start_presence(self):
        gamma = self.role('executive', {'roles': [r for r in self.transition['roles'] if r['identity'] == 'Gamma']})
        start = next(c for c in gamma['cases'] if c['date'] == '1990-01-01')
        self.assertEqual(start['campaign']['verdict'], 'identity_not_present_at_campaign_start')
        self.assertTrue(self.transition['case_notes'])


class PendingEvidence(FixtureTree):
    def test_acceptance_classes_are_kept_apart_and_conservative(self):
        obs = self.alpha['observations']
        self.assertEqual(obs['al_pm#1']['acceptance'], 'accepted')
        self.assertEqual(obs['al_pm#3']['acceptance'], 'pending')
        self.assertEqual(obs['al_pm#4']['acceptance'], 'unattributed_intake')
        self.assertEqual(obs['al_pm#5']['acceptance'], 'pending')  # accepted + pending sources
        self.assertEqual(obs['al_pm#6']['acceptance'], 'unclassified_packet')
        self.assertEqual(obs['al_main_old']['acceptance'], 'production_registry')
        statuses = {p['packet']: p['status'] for p in self.outputs['summary.json']['packets']}
        self.assertEqual(statuses, {'CLAUDE-C01-31': 'accepted', 'CLAUDE-C01-32': 'pending',
                                    'CLAUDE-C01-33': 'unclassified_packet'})
        self.assertEqual(self.outputs['summary.json']['integration_records_not_packet_acceptance'],
                         ['CLAUDE-C01-SOURCE-32'])
        self.assertEqual(self.case('research:al_pm', '1999-09-09')['acceptance'], ['accepted', 'pending'])


class ImageBindings(FixtureTree):
    def test_bound_missing_and_rejected_images_are_reported_separately(self):
        production = matrix.Production(matrix.Inputs(self.tmp))
        at = date(1993, 1, 1)
        self.assertEqual(production.portrait('p_new', at)['reason'], 'no_portrait_record')
        self.assertEqual(production.portrait('p_co1', at)['reason'], 'record_identity_mismatch')
        self.assertEqual(production.portrait('p_co2', at)['reason'], 'record_not_reviewed_or_wrong_style')
        self.assertEqual(production.portrait('p_act', date(1990, 6, 1))['reason'], 'ambiguous_multiple_records')
        self.assertEqual(production.portrait('p_x', at)['reason'], 'asset_not_on_served_allowlist')
        # Half-open windows: the end day belongs to the next record.
        self.assertTrue(production.portrait('p_exec', date(1990, 12, 31))['asset'].endswith('p-exec-1990.png'))
        self.assertTrue(production.portrait('p_exec', date(1991, 1, 1))['asset'].endswith('p-exec-1991.png'))
        self.assertEqual(production.portrait('p_exec', date(1992, 1, 1))['reason'], 'no_record_covers_date')

    def test_asset_availability_is_a_separate_check(self):
        assets = {a['asset'].rsplit('/', 1)[1]: a for a in self.outputs['summary.json']['assets']}
        self.assertEqual(assets['p-exec-1990.png']['status'], 'available')
        self.assertEqual(assets['p-exec-1991.png']['status'], 'missing_file')
        self.assertEqual(assets['p-sha-cartoon.png']['status'], 'sha256_mismatch')
        self.assertEqual(assets['p-x-cartoon.png']['status'], 'not_on_served_allowlist')
        self.assertEqual(assets['p-old-cartoon.png']['status'], 'possible_wrong_person_binding')
        self.assertTrue(assets['p-old-cartoon.png']['shared_across_people'])
        self.assertEqual(self.outputs['summary.json']['assets_shared_across_people'],
                         [matrix.PORTRAIT_PREFIX + 'p-old-cartoon.png'])
        # The game would serve this binding; the audit still flags the misnamed file.
        misfiled = self.case('party:al_holey', '1993-01-01')
        self.assertEqual(misfiled['image'][0]['binding'], 'bound')
        self.assertEqual(misfiled['asset'], [[matrix.PORTRAIT_PREFIX + 'someone-else-cartoon.png', 'possible_wrong_person_binding']])
        self.assertFalse(assets['someone-else-cartoon.png']['filename_matches_person'])
        retained = self.case('executive', '1991-01-01')
        self.assertEqual(retained['image'][0]['context'], 'if_original_executive_retained')
        self.assertEqual(retained['image'][0]['binding'], 'bound')
        self.assertEqual(retained['asset'], [[matrix.PORTRAIT_PREFIX + 'p-exec-1991.png', 'missing_file']])

    def test_unbound_holders_carry_their_art_job_or_its_absence(self):
        day_of = self.case('party:al_main', '1995-06-10')['image']
        self.assertEqual(day_of, [{'person': 'p_new', 'as': 'holder', 'binding': 'unbound',
                                   'reason': 'no_portrait_record', 'art_job': 'cartoon:p_new:1995-06-10:1997-01-01:v1'}])
        self.assertEqual(self.case('party:al_main', '1997-01-01')['image'][0]['art_job'], 'none_covers_date')
        self.assertIn('p_old', self.alpha['identities'][0]['statistics']['windows_extending_past_recorded_death'])

    def test_fictional_art_only_inside_the_window_and_for_the_exact_identity(self):
        production = matrix.Production(matrix.Inputs(self.tmp))
        one, two = 'fictional_v1_alpha_al_main_main_01', 'fictional_v1_alpha_al_main_main_02'
        for when in (date(2026, 9, 8), date(2030, 1, 1), date(2035, 12, 31)):
            self.assertEqual(production.portrait(one, when)['binding'], 'bound')
        for when in (date(2026, 9, 7), date(2036, 1, 1)):
            self.assertEqual(production.portrait(one, when)['reason'], 'outside_fictional_window')
        self.assertEqual(production.portrait(two, date(2030, 1, 1))['reason'], 'record_identity_mismatch')
        future = self.case('party:al_main', '2030-01-01')['future']
        self.assertEqual(future['portraits_bound'], [one])
        self.assertEqual(future['portraits_unbound'], {'record_identity_mismatch': 1})
        self.assertEqual(future['executive_authorized'], 1)
        self.assertEqual(self.case('party:al_main', '2026-09-07')['future'],
                         {'eligible': False, 'reason': 'before_fictional_window'})
        self.assertEqual(self.case('party:al_main', '2036-01-01')['future'],
                         {'eligible': False, 'reason': 'after_fictional_window'})


class CampaignComparison(FixtureTree):
    def test_divergence_is_expected_and_history_is_not_overwritten(self):
        obs = {'t_a': {'person': 'p_a'}, 'r_1': {'name': 'Research Name'}}
        known = {'status': 'established', 'holders': [['t_a', 'historical_on']]}
        self.assertEqual(matrix.compare_incumbent(known, obs, {'person': 'p_a', 'name': 'A'}), 'campaign_matches_reference')
        self.assertEqual(matrix.compare_incumbent(known, obs, {'person': 'p_b', 'name': 'B'}), 'campaign_diverged_expected')
        self.assertEqual(matrix.compare_incumbent(known, obs, {'person': None, 'name': 'Emergent'}), 'campaign_incumbent_unnamed')
        self.assertEqual(matrix.compare_incumbent(known, obs, {'person': None, 'name': None}), 'campaign_role_vacant')
        self.assertEqual(matrix.compare_incumbent({'status': 'unknown'}, obs, {'person': 'p_a'}), 'historical_identity_not_established')
        research = {'status': 'established', 'holders': [['r_1', 'within_stated_interval']]}
        self.assertEqual(matrix.compare_incumbent(research, obs, {'person': 'p_a'}), 'identity_reconciliation_required')
        self.assertEqual(matrix.compare_incumbent(known, obs, None), 'no_campaign_observation')

    def test_saved_campaign_report_keeps_fixture_divergence_out_of_the_matrix(self):
        world = {'year': 2000, 'month': 6, 'day': 1,
                 'rules': {'daily_simulation': True, 'historical_party_leadership': True},
                 'leadership': [{'nation': 'Alpha', 'name': None, 'since': '1998-02-02',
                                 'emergent': {'name': 'Emergent Premier', 'since': '1998-02-02'}}],
                 'party_leadership': {'assignments': [
                     {'nation': 'Alpha', 'party': 'al_main', 'component': None, 'holders': [{'person': 'p_co1'}]},
                     {'nation': 'Alpha', 'party': 'al_duo', 'component': None,
                      'holders': [{'person': 'p_co1'}, {'person': 'p_co2'}]}],
                     'executives': [], 'office_identities': []}}
        save = self.tmp / 'fixture-save.json.gz'
        save.write_bytes(gzip.compress(json.dumps({'format': 'fixture', 'world': {'world': world}}).encode()))
        before = matrix.canonical(self.alpha)
        report = matrix.campaign_report(self.tmp, save)
        alpha = next(i for i in report['identities'] if i['identity'] == 'Alpha')
        verdicts = {p['role']: p['verdict'] for p in alpha['parties']}
        self.assertEqual(verdicts, {'party:al_main': 'campaign_diverged_expected',
                                    'party:al_duo': 'campaign_matches_reference'})
        self.assertEqual(alpha['executive']['verdict'], 'campaign_incumbent_unnamed')
        self.assertEqual(report['campaign_date'], '2000-06-01')
        # The historical lookup for that date is unchanged by the divergent campaign.
        rebuilt = matrix.build(self.tmp)
        self.assertEqual(matrix.canonical(rebuilt['cases-alpha.json']), before)
        cell = matrix.production_cell(matrix.Production(matrix.Inputs(self.tmp)),
                                      self.role('party:al_main'), matrix.Production(matrix.Inputs(self.tmp)).parties[('Alpha', 'al_main')],
                                      date(2000, 6, 1))
        self.assertEqual([h[0] for h in cell['holders']], ['al_main_new'])


class DeterministicRegeneration(FixtureTree):
    def test_regeneration_is_byte_identical_and_check_detects_staleness(self):
        again = matrix.build(self.tmp)
        self.assertEqual({k: matrix.canonical(v) if not isinstance(v, str) else v for k, v in again.items()},
                         {k: matrix.canonical(v) if not isinstance(v, str) else v for k, v in self.outputs.items()})
        out = self.tmp / matrix.OUTPUT
        matrix.write_outputs(self.outputs, out)
        self.assertEqual(matrix.check_outputs(matrix.build(self.tmp), out), [])
        self.assertEqual(quiet_main(['--root', str(self.tmp), '--check']), 0)
        registry = self.tmp / matrix.REGISTRY
        data = json.loads(registry.read_text(encoding='utf-8'))
        data['parties'][0]['terms'][1]['from'] = DAY('1995-06-11')  # an off-by-one repair must be seen
        registry.write_text(json.dumps(data, indent=1) + '\n', encoding='utf-8', newline='\n')
        problems = matrix.check_outputs(matrix.build(self.tmp), out)
        self.assertIn('stale: summary.json', problems)
        self.assertIn('stale: cases-alpha.json', problems)
        self.assertEqual(quiet_main(['--root', str(self.tmp), '--check']), 1)
        (out / 'stray.json').write_text('{}\n', encoding='utf-8')
        (out / 'README.md').unlink()
        problems = matrix.check_outputs(matrix.build(self.tmp), out)
        self.assertIn('unexpected: stray.json', problems)
        self.assertIn('missing: README.md', problems)

    def test_input_hashes_ignore_checkout_newlines_and_every_read_is_recorded(self):
        crlf = self.tmp / matrix.PENDING_NOTE
        crlf.write_bytes(crlf.read_bytes().replace(b'\n', b'\r\n'))
        rows = {r['path']: r for r in matrix.build(self.tmp)['summary.json']['inputs']}
        original = {r['path']: r for r in self.outputs['summary.json']['inputs']}
        self.assertEqual(rows[matrix.PENDING_NOTE], original[matrix.PENDING_NOTE])
        for rel in (matrix.REGISTRY, matrix.PENDING_NOTE, matrix.RESEARCH + '/alpha.json',
                    matrix.RESEARCH + '/alpha-pending-32.md', matrix.SELF, *matrix.MIRRORED_SEMANTICS):
            self.assertIn(rel, rows)
        self.assertFalse(rows[matrix.PORTRAIT_PREFIX + 'p-exec-1991.png']['exists'])


class IndependentReviewRegressions(FixtureTree):
    def test_negative_acceptance_mentions_do_not_accept_a_packet(self):
        path = self.tmp / matrix.INTEGRATIONS / 'CLAUDE-C01-33/README.md'
        path.parent.mkdir()
        for text in ('# Review\n\nNot accepted.\n',
                     '# Review\n\nThe prior packet was accepted; this submission is pending.\n',
                     '# Review\n\n> **Decision: accepted as bounded research.**\n\nQuoted request only.\n'):
            with self.subTest(text=text):
                path.write_text(text, encoding='utf-8')
                packets, _, _, _ = matrix.packet_provenance(matrix.Inputs(self.tmp))
                self.assertEqual(packets['CLAUDE-C01-33']['status'], 'unclassified_packet')

    def test_accepted_and_unattributed_sources_are_not_wholly_accepted(self):
        packets, owners, _, _ = matrix.packet_provenance(matrix.Inputs(self.tmp))
        status, _, classes = matrix.evidence_class(['src_acc', 'new_unclaimed_source'], owners, packets)
        self.assertEqual(status, 'mixed_intake')
        self.assertEqual(classes, {'accepted': 1, 'unattributed_intake': 1})
        self.assertEqual(matrix.evidence_class(['new_unclaimed_source'], owners, packets)[0], 'unattributed_intake')

    def test_period_observations_never_become_exact_day_holders(self):
        observations = {
            'month': {'name': 'Month holder', 'from': None, 'until': None,
                      'observation_window': {'from': '1990-07-01', 'through': '1990-07-31'}},
            'trial': {'name': 'Trial witness', 'from': None, 'until': None,
                      'attested_period': {'from': '2022-04-19', 'through': '2022-04-21'}}}
        role = {'observations': list(observations), 'entry': {'lifecycle': {}}}
        for when, oid in [('1990-07-01', 'month'), ('1990-07-15', 'month'), ('1990-07-31', 'month'),
                          ('2022-04-19', 'trial'), ('2022-04-20', 'trial'), ('2022-04-21', 'trial')]:
            with self.subTest(when=when):
                cell = matrix.research_cell(role, observations, date.fromisoformat(when))
                self.assertEqual(cell['status'], 'period_attested')
                self.assertNotIn(cell['status'], matrix.IDENTIFIED)
                self.assertNotIn('holders', cell)
                self.assertEqual(cell['possible'][0][0], oid)
        for when in ('1990-06-30', '1990-08-01', '2022-04-18', '2022-04-22'):
            self.assertEqual(matrix.research_cell(role, observations, date.fromisoformat(when))['status'], 'unknown')
        dates = matrix.research_boundaries(role, observations)
        self.assertTrue({date(1990, 6, 30), date(1990, 7, 1), date(1990, 7, 2), date(1990, 7, 30),
                         date(1990, 7, 31), date(1990, 8, 1)} <= set(dates))
        self.assertTrue(all(not label.startswith('handover') for labels in dates.values() for label in labels))

    def test_executive_inherits_paired_office_handover_and_attestation_dates(self):
        for when in ('1996-04-01', '1996-04-02', '1996-04-03', '2005-05-05', '2005-05-06'):
            executive = self.case('executive', when)
            research = self.case('research:al_pm', when)
            self.assertEqual(executive['historical'], research['historical'])
            self.assertEqual(executive['kinds'], research['kinds'])

    def test_invalid_calendar_and_noncanonical_portrait_windows_never_bind(self):
        production = matrix.Production(matrix.Inputs(self.tmp))
        for start, end in [('1990-02-30', '1991-01-01'), ('19900101', '1991-01-01'),
                           ('1990-01-01', '1991-02-30'), ('1990-01-01', '19911231')]:
            with self.subTest(start=start, end=end):
                production.portraits['p_exec']['portraits'] = [record('p_exec', start, end, 'p-exec-1990.png')]
                self.assertEqual(production.portrait('p_exec', date(1990, 6, 1))['binding'], 'unbound')
        pid = 'fictional_v1_alpha_al_main_main_01'
        for start, end in [('2026-09-31', '2036-01-01'), ('20260908', '2036-01-01'),
                           ('2026-09-08', '2035-02-30'), ('2026-09-08', '20350101')]:
            with self.subTest(start=start, end=end):
                production.fictional_portraits[pid]['portraits'] = [fictional_record(pid, 'seed01', 'fictional-alpha-01.png', **{'from': start, 'to': end})]
                self.assertEqual(production.portrait(pid, date(2030, 1, 1))['binding'], 'unbound')

    def test_enabled_missing_book_does_not_invent_legacy_executive_identity(self):
        production = matrix.Production(matrix.Inputs(self.tmp))
        world = {'rules': {'historical_party_leadership': True},
                 'leadership': [{'nation': 'Alpha', 'name': 'Exec Person', 'since': '1988-01-01', 'emergent': None}]}
        for book in (None, {}):
            world['party_leadership'] = book
            self.assertIsNone(matrix.campaign_observation(production, world, 'Alpha')['person'])
        world['rules']['historical_party_leadership'] = False
        self.assertEqual(matrix.campaign_observation(production, world, 'Alpha')['person'], 'p_exec')

    def test_saved_executive_assignment_precedes_legacy_office_row(self):
        production = matrix.Production(matrix.Inputs(self.tmp))
        world = {'rules': {'historical_party_leadership': True}, 'leadership': [],
                 'party_leadership': {'executives': [{'nation': 'Alpha', 'holder': {'person': 'p_new'}}]}}
        self.assertEqual(matrix.campaign_observation(production, world, 'Alpha')['person'], 'p_new')
        self.assertEqual(matrix.campaign_observation(production, world, 'Alpha')['source'], 'saved_executive_assignment')

    def test_saved_report_pins_exact_save_and_marks_future_history_inapplicable(self):
        save = self.tmp / 'save.json.gz'
        data = json.dumps({'world': {'world': {'year': 2035, 'month': 12, 'day': 31,
             'rules': {'daily_simulation': True}, 'leadership': [], 'party_leadership': None}}}).encode()
        raw = gzip.compress(data, mtime=0)
        save.write_bytes(raw)
        before = save.read_bytes()
        report = matrix.campaign_report(self.tmp, save)
        self.assertEqual(report['save_identity']['sha256'], sha(raw))
        self.assertEqual(report['save_identity']['decoded_sha256'], sha(data))
        self.assertEqual(report['save_identity']['bytes'], len(raw))
        self.assertEqual(report['save_identity']['decoded_bytes'], len(data))
        self.assertTrue(report['inputs'])
        self.assertEqual(report['status'], 'read_only_observation_not_campaign_validation')
        for identity in report['identities']:
            self.assertEqual(identity['executive']['historical']['reason'], 'after_historical_cutoff')
        self.assertEqual(before, save.read_bytes())

    def test_unknown_kind_dissolved_party_is_inapplicable_not_missing(self):
        production = matrix.Production(matrix.Inputs(self.tmp))
        party = production.parties[('Alpha', 'al_gone')]
        party['kind'] = 'unknown'
        cell = matrix.production_cell(production, self.role('party:al_gone'), party, date(2001, 3, 4))
        self.assertEqual(cell, {'status': 'inapplicable', 'reason': 'organization_not_existing'})

    def test_death_day_coverage_and_open_ended_portraits_are_flagged(self):
        production = matrix.Production(matrix.Inputs(self.tmp))
        for end, expected in [('1995-06-10', False), ('1995-06-11', True), (None, True)]:
            with self.subTest(end=end):
                production.portraits['p_old']['portraits'][0]['to'] = end
                people, _ = matrix.person_audit(production, matrix.Assets(matrix.Inputs(self.tmp), production),
                    'Alpha', self.alpha['observations'], self.alpha['roles'])
                person = next(p for p in people if p['person'] == 'p_old')
                self.assertEqual(person['portrait_windows'][0]['extends_past_recorded_death'], expected)

    def test_changed_research_cutoff_and_duplicate_identity_fail_closed(self):
        packet_path = self.tmp / matrix.RESEARCH / 'alpha.json'
        packet = json.loads(packet_path.read_text())
        packet['research_cutoff'] = '2035-12-31'
        write_tree(self.tmp, {matrix.RESEARCH + '/alpha.json': packet})
        with self.assertRaisesRegex(matrix.MatrixError, 'Research cutoff changed'):
            matrix.build(self.tmp)
        packet['research_cutoff'] = '2026-09-07'
        write_tree(self.tmp, {matrix.RESEARCH + '/alpha.json': packet, matrix.RESEARCH + '/duplicate.json': packet})
        with self.assertRaisesRegex(matrix.MatrixError, 'Duplicate research identity'):
            matrix.build(self.tmp)

    def test_fictional_manifest_entry_does_not_count_as_served_art(self):
        stats = self.alpha['identities'][0]['statistics']
        self.assertEqual(stats['fictional_candidates'], 2)
        self.assertEqual(stats['fictional_candidates_with_portrait'], 1)
        wrong = next(p for p in self.alpha['fictional_candidates'] if p['person'].endswith('_02'))
        self.assertTrue(wrong['portrait_assets'])
        self.assertEqual(wrong['bound_portrait_sample_dates'], [])
        future = self.case('party:al_main', '2030-01-01')['future']
        self.assertTrue(future['listed_in_simulation_future_reference'])
        self.assertFalse(future['served_web_historical_reference'])

    def test_shared_asset_with_missing_checksum_is_reported_not_crashed(self):
        production = matrix.Production(matrix.Inputs(self.tmp))
        production.portraits['p_co2']['portraits'][0].pop('sha256')
        row = matrix.Assets(matrix.Inputs(self.tmp), production).check(matrix.PORTRAIT_PREFIX + 'p-old-cartoon.png')
        self.assertEqual(row['status'], 'sha256_mismatch')
        self.assertIn(None, row['manifest_sha256'])


class RealCommittedOutput(unittest.TestCase):
    """Actual production observations recorded in the committed matrix."""

    @classmethod
    def setUpClass(cls):
        cls.outputs = matrix.build()
        cls.summary = cls.outputs['summary.json']

    def role(self, file, key):
        return next(r for r in self.outputs[file]['roles'] if r['key'] == key)

    def case(self, file, key, when):
        return next(c for c in self.role(file, key)['cases'] if c['date'] == when)

    def test_committed_matrix_is_current(self):
        self.assertEqual(matrix.check_outputs(self.outputs, matrix.ROOT / matrix.OUTPUT), [])

    def test_scope_is_preparation_and_covers_all_eight_cases(self):
        self.assertEqual(self.summary['status'], 'preparation_only_not_s23_acceptance')
        self.assertFalse(self.summary['s23_complete'])
        self.assertFalse(self.summary['c06_complete'])
        self.assertEqual([c['case'] for c in self.summary['cases']],
                         ['France', 'Japan', 'India', 'Brazil', 'SouthAfrica', 'Tonga', 'SaudiArabia', 'USSR -> Russia'])
        statuses = {p['packet'][-2:]: p['status'] for p in self.summary['packets']}
        self.assertEqual({k for k, v in statuses.items() if v == 'accepted'},
                         {'01', '02', '03', '04', '07', '08', '23', '24', '25', '27', '30', '32', '33'})
        self.assertEqual({k for k, v in statuses.items() if v == 'pending'},
                         {'05', '06', *(f'{n:02d}' for n in range(9, 23)), '26'})
        self.assertNotIn('unclassified_packet', statuses.values())

    def test_every_role_has_the_required_dates_and_honest_bindings(self):
        required = {d for d, _ in matrix.FIXED_DATES} | {date(y, 1, 1) for y in matrix.YEARS}
        allowed = set(self.summary['historical_status'])
        for name, doc in self.outputs.items():
            if not name.startswith('cases-'):
                continue
            for role in doc['roles']:
                dates = {date.fromisoformat(c['date']) for c in role['cases']}
                self.assertTrue(required <= dates, (name, role['key']))
                for cell in role['cases']:
                    when = date.fromisoformat(cell['date'])
                    self.assertIn(cell['historical']['status'], allowed)
                    if when > matrix.CUTOFF or when < matrix.REFERENCE_FROM:
                        self.assertNotIn('holders', cell['historical'])
                    if role['family'] == 'research_role':
                        self.assertEqual(cell['image'], 'no_production_identity')
                    if isinstance(cell['image'], list):
                        for image in cell['image']:
                            if image['person'].startswith('fictional_'):
                                self.assertGreaterEqual(when, matrix.FICTIONAL_FROM)
                    if 'campaign' in cell:
                        self.assertEqual(cell['date'], '1990-01-01')
                        self.assertIn(cell['campaign']['verdict'], matrix.CAMPAIGN_VERDICTS)
                        self.assertNotEqual(cell['campaign']['verdict'], 'campaign_differs_from_reference')

    def test_actual_boundary_observations(self):
        tonga = self.case('cases-tonga.json', 'research:to_king', '2006-09-11')['historical']
        self.assertEqual(tonga['status'], 'boundary_day')
        self.assertEqual([r[1] for r in tonga['holders']], ['stated_end_day', 'stated_start_day'])
        ldp = self.case('cases-japan.json', 'party:jp_ldp', '1991-10-31')['historical']['holders']
        self.assertEqual(ldp, [['jp_ldp_kiichi_miyazawa_19911031', 'historical_on']])
        death = self.case('cases-india.json', 'party:in_inc', '1991-05-21')
        self.assertIn('death_day_of', death['kinds'])
        self.assertNotIn('holders', death['historical'])
        rpr = self.case('cases-france.json', 'party:fr_rpr', '2002-09-22')['historical']
        self.assertEqual(rpr, {'status': 'inapplicable', 'reason': 'organization_not_existing'})
        # A year-level observation is not proof of the PM on 1 January.
        tonga_period = self.case('cases-tonga.json', 'research:to_pm', '1990-01-01')['historical']
        self.assertEqual(tonga_period['status'], 'period_attested')
        self.assertNotIn('holders', tonga_period)
        tonga_exec = self.case('cases-tonga.json', 'executive', '2006-09-11')['historical']
        self.assertEqual(tonga_exec, tonga)
        ussr_window = self.case('cases-ussr-russia.json', 'research:su_cpsu_general_secretary', '1990-07-01')['historical']
        self.assertEqual(ussr_window['status'], 'period_attested')
        self.assertNotIn('holders', ussr_window)


if __name__ == '__main__':
    unittest.main()
