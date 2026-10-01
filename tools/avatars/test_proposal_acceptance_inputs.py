"""Preparation provenance stays explicit and separate from live production."""
import json
from pathlib import Path
import shutil
import tempfile
import unittest
from unittest.mock import patch

import check_country_cast as cast
import proposal_acceptance_inputs as baseline


class PreparationBaselineTests(unittest.TestCase):
    @classmethod
    def setUpClass(cls):
        cls.snapshot = baseline.Snapshot(cast.ROOT)

    def copy_fixture(self):
        tmp = tempfile.TemporaryDirectory(prefix='spheres-preparation-inputs-')
        self.addCleanup(tmp.cleanup)
        root = Path(tmp.name)
        shutil.copytree(cast.ROOT / baseline.DIRECTORY, root / baseline.DIRECTORY)
        return root

    def test_scope_names_the_exact_preproduction_revision_and_grants_nothing(self):
        scope = self.snapshot.scope()
        self.assertEqual(scope['source_revision'], baseline.BASE)
        self.assertEqual(scope['kind'], 'immutable_preparation_baseline')
        self.assertFalse(scope['live_production_checked'])
        self.assertEqual(len(self.snapshot.files), 68)

    def test_old_uninstalled_people_and_fictional_names_are_retained_exactly(self):
        proposal = json.loads(self.snapshot.read(cast.DEFAULT.as_posix()))
        inputs = cast.load_inputs(snapshot=self.snapshot)
        people = {p['id'] for p in inputs['registry']['people']}
        reserved = [p['person_id'] for p in proposal['people']
                    if p['identity']['status'] == 'reserved_not_imported']
        self.assertEqual(len(reserved), 5)
        self.assertTrue(set(reserved).isdisjoint(people))
        future = json.loads(self.snapshot.read('spheres-web/data/future_candidates_2035.json'))
        ids = {p['person_id'] for p in future['candidates']}
        self.assertNotIn('fictional_to_sitani_lolohea', ids)
        self.assertFalse(cast.check(proposal, **inputs)['country_completed'])

    def test_portable_fixture_load_does_not_call_git_or_network(self):
        root = self.copy_fixture()
        with patch('subprocess.run', side_effect=AssertionError('Git/process access forbidden')):
            snapshot = baseline.Snapshot(root)
        self.assertEqual(snapshot.read('spheres-sim/data/leaders_1990.json'),
                         self.snapshot.read('spheres-sim/data/leaders_1990.json'))

    def test_changed_manifest_is_rejected_before_inputs_are_used(self):
        root = self.copy_fixture()
        path = root / baseline.DIRECTORY / 'manifest.json'
        path.write_bytes(path.read_bytes().replace(b'"version": 1', b'"version": 2'))
        with self.assertRaisesRegex(ValueError, 'manifest SHA-256 mismatch'):
            baseline.Snapshot(root)

    def test_changed_compressed_body_is_rejected(self):
        root = self.copy_fixture()
        path = root / baseline.DIRECTORY / 'inputs.zip'
        data = bytearray(path.read_bytes())
        data[len(data) // 2] ^= 1
        path.write_bytes(data)
        with self.assertRaisesRegex(ValueError, 'bundle SHA-256 mismatch'):
            baseline.Snapshot(root)

    def test_missing_baseline_input_cannot_fall_back_to_live_files(self):
        with self.assertRaisesRegex(ValueError, 'absent from pinned preparation baseline'):
            self.snapshot.read('spheres-sim/data/tonga_institutional_leadership.json')

    def test_live_loading_remains_an_explicit_unfiltered_current_file_path(self):
        with patch.object(cast, 'read', return_value={'sentinel': 'live'}) as reader:
            values = cast.load_inputs()
        self.assertEqual(reader.call_count, 4)
        self.assertTrue(all(v == {'sentinel': 'live'} for v in values.values()))
        self.assertEqual(baseline.live_scope()['kind'], 'live_repository_inputs')

    def test_packet_bytes_at_the_baseline_match_the_unchanged_documents(self):
        for path in [cast.DEFAULT.as_posix(),
                     'docs/campaign-certification/C04/preparation/france-tonga/proposals.json',
                     'docs/campaign-certification/C04/preparation/france-tonga/sources.json']:
            with self.subTest(path=path):
                self.assertEqual((cast.ROOT / path).read_text(encoding='utf-8').encode(),
                                 self.snapshot.read(path))


if __name__ == '__main__':
    unittest.main()
