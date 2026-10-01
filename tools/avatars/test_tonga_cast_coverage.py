"""Production-coverage negative cases, independent of the old proposal fixture."""
from copy import deepcopy
import hashlib
import json
from pathlib import Path
import tempfile
import unittest
from unittest.mock import patch

import tonga_cast_coverage as gate


class TongaCoverageTests(unittest.TestCase):
    def setUp(self):
        self.tmp = tempfile.TemporaryDirectory(prefix='spheres-tonga-coverage-')
        self.addCleanup(self.tmp.cleanup)
        self.root = Path(self.tmp.name)
        self.receipt = self.root / 'review.json'
        self.receipt.write_text('{"status":"passed","fixture":true}\n', encoding='utf-8')
        self.pin = {'path': 'review.json', 'sha256': hashlib.sha256(self.receipt.read_bytes()).hexdigest()}
        self.pids = ['taufaahau_tupou_iv', 'fatafehi_tuipelehake', 'siaosi_taufaahau_manumataongo']
        holder = {'name': 'Synthetic reviewed holder', 'attested_on': '1990-02-03',
                  'from': None, 'until': None, 'claim_ids': ['fixture_claim'], 'uncertainty': 'Day observation only.'}
        self.research = {'nation': 'Tonga', 'research_cutoff': gate.CUTOFF,
                         'sources': [{'id': 'fixture', 'claims': [{'id': 'fixture_claim'}]}],
                         'organizations': [], 'institutions': [{'id': 'crown', 'roles': [
                             {'id': 'king', 'holder_claims': [holder]}]}]}
        self.registry = {'people': [{'id': pid, 'name': 'Synthetic ' + pid} for pid in self.pids], 'parties': []}
        self.institutions = {'opening': dict(zip(
            ('king_person_id', 'prime_minister_person_id', 'heir_person_id'), self.pids)),
            'historical_bindings': [{'id': 'binding', 'institution_id': 'crown', 'role_id': 'king',
                                    'person_id': self.pids[0], 'holder': holder}],
            'future_candidates': []}
        self.export = {'candidates': []}
        for i, (pid, role) in enumerate(gate.FICTION_IDS.items()):
            c = {'person': {'id': pid, 'name': pid}, 'role': role, 'origin': 'fictional_successor',
                 'appearance_seed': f'{i:016x}', 'fictional_biography': 'Invented fixture biography.'}
            self.institutions['future_candidates'].append(c)
            self.export['candidates'].append({'person_id': pid, 'name': pid,
                                              'appearance_seed': c['appearance_seed'],
                                              'institution': 'tonga_civilian_institutions', 'party': None})
        self.historical_ready = {pid: [{'from': gate.FIRST, 'to': gate.UNTIL, 'style': 'cartoon',
                                        'method': 'generated', 'status': 'illustrated-likeness'}] for pid in self.pids}
        self.future_ready = {pid: [{'from': gate.FUTURE, 'to': gate.UNTIL}] for pid in gate.FICTION_IDS}
        self.historical_check = patch.object(gate.art, 'validate_manifest', side_effect=lambda *a:
            {'valid': True, 'errors': [], 'ready': self.historical_ready})
        self.future_check = patch.object(gate.fiction, 'validate', side_effect=lambda *a:
            {'valid': True, 'errors': [], 'ready': self.future_ready})
        self.historical_check.start(); self.addCleanup(self.historical_check.stop)
        self.future_check.start(); self.addCleanup(self.future_check.stop)
        # Image decoding and rights are the existing pipelines' responsibility;
        # these unit tests exercise the extra country coverage contract.
        sources = {}
        for path in set().union(*gate.CHECK_PINS.values()):
            p = self.root / path
            p.parent.mkdir(parents=True, exist_ok=True)
            p.write_text('Pinned synthetic test source.\n', encoding='utf-8')
            sources[path] = hashlib.sha256(p.read_bytes()).hexdigest()
        observation_key = next(iter(gate.inventory(self.research)[1]))
        self.manifest = {'version': 1, 'kind': 'production_country_cast', 'nation': 'Tonga',
            'historical_cutoff': gate.CUTOFF, 'from': gate.FIRST, 'until_exclusive': gate.UNTIL,
            'roles': [{'institution_id': 'crown', 'role_id': 'king', 'status': 'covered', 'chain_review': self.pin}],
            'historical_bindings': [{'observation_key': observation_key, 'binding_id': 'binding',
                'holder': holder, 'person_id': self.pids[0], 'status': 'installed_reference', 'identity_review': self.pin}],
            'historical_people': [{'person_id': pid, 'appearance_review': self.pin,
                'required_appearance_intervals': [{'from': gate.FIRST, 'to': gate.UNTIL,
                                                  'meaning': 'appearance_not_office_tenure'}]} for pid in self.pids],
            'checks': {k: dict(self.pin, status='passed', source_pins={p: sources[p] for p in paths})
                       for k, paths in gate.CHECK_PINS.items()}}

    def report(self):
        return gate.validate(self.manifest, research=self.research, registry=self.registry,
            institutions=self.institutions, portraits={}, fictional_portraits={}, fictional_catalog=self.export, root=self.root)

    def test_complete_fixture_is_ready_without_automatically_claiming_a_country(self):
        result = self.report()
        self.assertEqual(result['errors'], [])
        self.assertEqual(result['blockers'], [])
        self.assertTrue(result['ready_for_country_signoff'])
        self.assertFalse(result['country_completed'])

    def test_zero_opening_parties_cannot_hide_later_research_role(self):
        self.research['organizations'].append({'id': 'later_movement', 'roles': []})
        result = self.report()
        self.assertFalse(result['ready_for_country_signoff'])
        self.assertTrue(any('later_movement/__unresearched_role__' in b for b in result['blockers']))

    def test_unknown_boundaries_cannot_be_converted_to_effective_tenure(self):
        self.manifest['historical_bindings'][0]['holder'] = deepcopy(self.manifest['historical_bindings'][0]['holder'])
        self.manifest['historical_bindings'][0]['holder']['from'] = '1990-02-03'
        self.assertTrue(any('boundaries changed' in e for e in self.report()['errors']))

    def test_institutional_exception_cannot_waive_named_people(self):
        self.manifest['roles'][0].update(status='institutional_exception', reason='Institutional body',
                                         claim_ids=['fixture_claim'], review=self.pin)
        self.assertTrue(any('cannot be waived' in b for b in self.report()['blockers']))

    def test_queued_or_missing_physical_art_cannot_close_country(self):
        self.historical_ready.pop(self.pids[0])
        self.future_ready.pop('fictional_to_pisila_tukuafu')
        self.manifest['country_complete'] = True
        result = self.report()
        self.assertFalse(result['country_completed'])
        self.assertTrue(any('no reviewed cartoon' in b for b in result['blockers']))
        self.assertTrue(any('fictional cartoon missing' in b for b in result['blockers']))
        self.assertTrue(any('Country completion claimed' in e for e in result['errors']))

    def test_small_window_cannot_omit_the_dated_observation(self):
        self.manifest['historical_people'][0]['required_appearance_intervals'][0]['to'] = '1990-02-01'
        self.assertTrue(any('dated holder observation' in b for b in self.report()['blockers']))

    def test_partial_fictional_art_is_not_full_2035_coverage(self):
        self.future_ready['fictional_to_lesieli_fotu'][0]['to'] = '2030-01-01'
        self.assertTrue(any('2030-01-01 through 2036-01-01' in b for b in self.report()['blockers']))

    def test_unreviewed_ptoa_permission_is_rejected(self):
        self.manifest['fictional_party_office_grants'] = ['fictional_to_kalolo_matalehu']
        self.assertTrue(any('Unreviewed PTOA' in e for e in self.report()['errors']))

    def test_fictional_identity_export_must_match_runtime_exactly(self):
        self.export['candidates'][0]['appearance_seed'] = 'aaaaaaaaaaaaaaaa'
        self.assertTrue(any('exact institutional portrait' in e for e in self.report()['errors']))

    def test_missing_native_evidence_stays_a_blocker(self):
        del self.manifest['checks']['save_compatibility']
        self.assertTrue(any('save_compatibility' in b for b in self.report()['blockers']))

    def test_receipt_hash_and_tested_source_changes_invalidate_readiness(self):
        self.receipt.write_text('Changed receipt', encoding='utf-8')
        self.assertTrue(any('SHA-256 mismatch' in e for e in self.report()['errors']))

    def test_arbitrary_source_pins_cannot_stand_in_for_native_verification(self):
        self.manifest['checks']['save_compatibility']['source_pins'] = {'review.json': self.pin['sha256']}
        self.assertTrue(any('required production source pins' in e for e in self.report()['errors']))

    def test_unknown_person_and_changed_cutoff_fail_closed(self):
        self.manifest['historical_bindings'][0]['person_id'] = 'fictional_to_lesieli_fotu'
        self.manifest['historical_cutoff'] = '2035-12-31'
        self.assertFalse(self.report()['valid'])


if __name__ == '__main__':
    unittest.main()
