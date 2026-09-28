"""Synthetic cross-shard guard tests; these do not execute a native campaign."""
import copy
from pathlib import Path
import subprocess
import tempfile
import unittest
from unittest.mock import patch

import distributed_stability as distributed
import stability_matrix as matrix
import test_stability_matrix as fixtures


class DistributedTests(unittest.TestCase):
    def setUp(self):
        self.temp = tempfile.TemporaryDirectory()
        self.addCleanup(self.temp.cleanup)
        self.root = Path(self.temp.name)
        self.plan = matrix.read_json(Path(matrix.__file__).with_name('stability-full.json'))
        self.manifest = {'format': distributed.FORMAT, 'revision': 'a' * 40,
                         'files': {name: {'bytes': 12, 'sha256': str(i) * 64} for i, name in enumerate(distributed.BUNDLE_FILES)},
                         'cells': self.plan['cells'], 'qualification': False, 's25_complete': False}

    def records(self, identity='france-1990'):
        freeze = {'candidate_revision': 'a' * 40, 'only_cell': identity, 'jobs': 1, 'batch_sha256': '0' * 64}
        for key, file in [('binary', 'native-test'), ('frozen_plan', 'plan.json'), ('frozen_harness', 'stability_matrix.py')]:
            freeze[key] = {'path': '/original/' + file, **self.manifest['files'][file]}
        proof = {'revision': 'a' * 40, 'only_cell': identity, 'jobs': 1, 'passed': False,
                 'coverage': {'full_matrix_passed': False}, 'selected_cell_passed': True,
                 'binary_unchanged': True, 'interrupted': False, 'batch_sha256': '0' * 64,
                 'cells': [{'id': identity, 'passed': True}]}
        return freeze, proof

    def test_valid_shard_remains_partial(self):
        freeze, proof = self.records()
        self.assertEqual(distributed.validate_shard_records(self.manifest, self.plan, 'france-1990', freeze, proof, '0' * 64), proof['cells'][0])
        self.assertFalse(proof['passed'])

    def test_mixed_candidate_binary_plan_and_harness_refused(self):
        for field in ['candidate_revision', 'binary', 'frozen_plan', 'frozen_harness']:
            freeze, proof = self.records()
            if field == 'candidate_revision':
                freeze[field] = 'b' * 40
            else:
                freeze[field]['sha256'] = 'f' * 64
            with self.subTest(field=field), self.assertRaises(matrix.InvalidEvidence):
                distributed.validate_shard_records(self.manifest, self.plan, 'france-1990', freeze, proof, '0' * 64)

    def test_missing_duplicate_wrong_cell_failure_drift_and_full_claim_refused(self):
        changes = [lambda p: p.update(cells=[]),
                   lambda p: p['cells'].append(copy.deepcopy(p['cells'][0])),
                   lambda p: p['cells'][0].update(id='tonga-7'),
                   lambda p: p['cells'][0].update(passed=False),
                   lambda p: p.update(only_cell='france-7'),
                   lambda p: p.update(selected_cell_passed=False),
                   lambda p: p.update(binary_unchanged=False),
                   lambda p: p.update(interrupted=True),
                   lambda p: p.update(passed=True),
                   lambda p: p['coverage'].update(full_matrix_passed=True),
                   lambda p: p.update(jobs=2), lambda p: p.update(batch_sha256='b' * 64)]
        for change in changes:
            freeze, proof = self.records()
            change(proof)
            with self.subTest(change=change), self.assertRaises(matrix.InvalidEvidence):
                distributed.validate_shard_records(self.manifest, self.plan, 'france-1990', freeze, proof, '0' * 64)

    def make_bundle(self):
        folder = self.root / 'bundle'
        folder.mkdir()
        (folder / 'native-test').write_bytes(b'synthetic, never executable')
        (folder / 'plan.json').write_bytes(matrix.json_bytes(self.plan))
        (folder / 'stability_matrix.py').write_bytes(Path(matrix.__file__).read_bytes())
        (folder / 'distributed_stability.py').write_bytes(Path(distributed.__file__).read_bytes())
        value = {**self.manifest, 'files': {name: distributed.content_pin(folder / name) for name in distributed.BUNDLE_FILES}}
        matrix.write_json_new(folder / 'manifest.json', value)
        digest = distributed.content_pin(folder / 'manifest.json')['sha256']
        return folder, digest

    def test_bundle_manifest_and_input_bytes_are_pinned(self):
        root, digest = self.make_bundle()
        distributed.bundle(root, digest)
        with self.assertRaisesRegex(matrix.InvalidEvidence, 'manifest changed'):
            distributed.bundle(root, 'f' * 64)
        (root / 'native-test').write_bytes(b'changed')
        with self.assertRaisesRegex(matrix.InvalidEvidence, 'input changed'):
            distributed.bundle(root, digest)

    def test_full_plan_cannot_be_shortened_even_after_repinning(self):
        root, _ = self.make_bundle()
        shortened = copy.deepcopy(self.plan)
        shortened['cells'].pop()
        (root / 'plan.json').write_bytes(matrix.json_bytes(shortened))
        value = matrix.read_json(root / 'manifest.json')
        value['files']['plan.json'] = distributed.content_pin(root / 'plan.json')
        value['cells'] = shortened['cells']
        (root / 'manifest.json').write_bytes(matrix.json_bytes(value))
        with self.assertRaises(matrix.InvalidEvidence):
            distributed.bundle(root, distributed.content_pin(root / 'manifest.json')['sha256'])

    def test_candidate_blob_binding_rejects_untracked_or_alternate_helpers(self):
        repo = self.root / 'repo'
        repo.mkdir()
        subprocess.run(['git', 'init', '--quiet', str(repo)], check=True)
        tracked = repo / 'helper.py'
        tracked.write_bytes(b'# candidate source\n')
        subprocess.run(['git', '-C', str(repo), 'add', 'helper.py'], check=True, capture_output=True)
        subprocess.run(['git', '-C', str(repo), '-c', 'user.name=Synthetic test', '-c', 'user.email=synthetic@example.invalid',
                        'commit', '--quiet', '-m', 'Synthetic provenance fixture'], check=True, capture_output=True)
        revision = subprocess.check_output(['git', '-C', str(repo), 'rev-parse', 'HEAD'], text=True).strip()
        distributed.require_tracked_input(repo, revision, 'helper.py', tracked)
        other = repo / 'untracked.py'
        other.write_bytes(tracked.read_bytes())
        with self.assertRaisesRegex(matrix.InvalidEvidence, 'not tracked'):
            distributed.require_tracked_input(repo, revision, 'untracked.py', other)
        other.write_bytes(b'# different loaded source\n')
        with self.assertRaisesRegex(matrix.InvalidEvidence, 'Loaded source differs'):
            distributed.require_tracked_input(repo, revision, 'helper.py', other)
        tracked.write_bytes(b'# dirty candidate source\n')
        with self.assertRaisesRegex(matrix.InvalidEvidence, 'candidate blob'):
            distributed.require_tracked_input(repo, revision, 'helper.py', tracked)
        with self.assertRaisesRegex(matrix.InvalidEvidence, 'unchanged checked-out'):
            distributed.require_clean_revision(repo, revision)

    def test_same_candidate_shards_from_another_batch_refused(self):
        freeze, proof = self.records()
        freeze['batch_sha256'] = proof['batch_sha256'] = 'b' * 64
        with self.assertRaisesRegex(matrix.InvalidEvidence, 'another frozen batch'):
            distributed.validate_shard_records(self.manifest, self.plan, 'france-1990', freeze, proof, '0' * 64)

    def make_shards(self):
        folder = self.root / 'shards'
        folder.mkdir()
        for cell in self.plan['cells']:
            root = folder / cell['id']
            root.mkdir()
            freeze, proof = self.records(cell['id'])
            matrix.write_json_new(root / 'freeze.json', freeze)
            matrix.write_json_new(root / 'result.json', proof)
        return folder

    def test_missing_extra_or_retry_shard_directory_refused(self):
        folder = self.make_shards()
        common = (self.root, self.manifest, self.plan)
        (folder / 'retry').mkdir()
        with patch.object(distributed, 'bundle', return_value=common), self.assertRaises(matrix.InvalidEvidence):
            distributed.aggregate(self.root, '0' * 64, folder, self.root / 'result.json')
        (folder / 'retry').rmdir()
        (folder / 'tonga-7').rename(folder / 'tonga-7-wrong')
        with patch.object(distributed, 'bundle', return_value=common), self.assertRaises(matrix.InvalidEvidence):
            distributed.aggregate(self.root, '0' * 64, folder, self.root / 'result.json')

    def test_aggregation_calls_independent_verifier_for_all_24_and_retains_failure(self):
        folder = self.make_shards()
        def verify(path):
            if path.name == 'india-42':
                raise matrix.InvalidEvidence('actual archived bytes changed')
            return {'passed': False, 'selected_cell_passed': True}
        with patch.object(distributed, 'bundle', return_value=(self.root, self.manifest, self.plan)), \
             patch.object(matrix, 'verify_retained_run', side_effect=verify) as independent:
            result = distributed.aggregate(self.root, '0' * 64, folder, self.root / 'failure.json')
        self.assertEqual(independent.call_count, 24)
        self.assertEqual(result['coverage']['passed'], 23)
        self.assertFalse(result['passed'])
        self.assertEqual(result['failures'][0]['id'], 'india-42')
        self.assertFalse(result['qualification'])

    def test_only_complete_verified_24_can_pass_matrix_but_never_certification(self):
        folder = self.make_shards()
        with patch.object(distributed, 'bundle', return_value=(self.root, self.manifest, self.plan)), \
             patch.object(matrix, 'verify_retained_run', return_value={'passed': False, 'selected_cell_passed': True}) as independent:
            result = distributed.aggregate(self.root, '0' * 64, folder, self.root / 'pass.json')
        self.assertEqual(independent.call_count, 24)
        self.assertTrue(result['passed'])
        self.assertFalse(result['qualification'])
        self.assertFalse(result['s25_complete'])
        self.assertFalse(result['coverage']['controlled_ussr_to_russia_case_passed'])

    def test_end_to_end_shard_dispatch_and_real_retained_verification(self):
        """Mock only the native process; real requests, gzip, pins and verifier."""
        frozen, digest = self.make_bundle()
        shards = self.root / 'actual-fixture-shards'
        shards.mkdir()
        class SyntheticProcess:
            def __init__(self, args, **kwargs):
                request = matrix.read_json(kwargs['env']['SPHERES_S25_REQUEST'])
                cell = {key: request[key] for key in ('id', 'country', 'seed', 'through')}
                native = Path(kwargs['env']['SPHERES_S25_OUT'])
                report = fixtures.synthetic_native_report(native, cell, request['revision'])
                matrix.write_json_new(native / 'result.json', report)
                kwargs['stdout'].write(fixtures.TestSummaryTests.GOOD.encode())
                self.returncode = 0
            def wait(self, timeout=None):
                return self.returncode
            def kill(self):
                self.returncode = -9
        with patch.object(matrix.subprocess, 'Popen', SyntheticProcess):
            for cell in self.plan['cells']:
                result = distributed.run_cell(frozen, digest, cell['id'], shards / cell['id'], 60)
                self.assertFalse(result['full_matrix_passed'])
        result = distributed.aggregate(frozen, digest, shards, self.root / 'actual-fixture-aggregate.json')
        self.assertTrue(result['passed'])
        self.assertEqual(result['coverage']['passed'], 24)
        self.assertFalse(result['qualification'])


if __name__ == '__main__':
    unittest.main()
