"""Synthetic guards only; no campaign, retained-matrix verification, or simulation."""
import copy
import datetime as dt
import os
from pathlib import Path
import subprocess
import sys
import tempfile
import unittest

import dependent_verify as d


def good_records():
    coverage = dict(declared=24, attempted=24, passed=24, failed=[], missing_requested=[], missing_full_cases=[],
                    requested_plan_passed=True, pilot_passed=False, full_matrix_passed=True,
                    controlled_ussr_to_russia_case_passed=False, qualification=False, s25_complete=False)
    rows = [dict(id=c['id'], passed=True, archives_verified=4) for c in d.CELLS]
    cfg = dict(retained=str(Path.cwd() / 'synthetic-retained'), helper=dict(path='pinned-helper', bytes=12, sha256='0'*64))
    proof = dict(format='spheres-stability-matrix-result/v1', revision=d.REVISION, scope='full', plan_id='s25-full-v1',
                 passed=True, binary_unchanged=True, interrupted=False, resource_halted=False, jobs=4,
                 qualification=False, s25_complete=False, coverage=coverage, cells=rows)
    verified = dict(format='spheres-stability-retained-verification/v1', integrity_verified=True, passed=True,
                    candidate_revision=d.REVISION, source=cfg['retained'], frozen_harness_sha256='0'*64,
                    verifier=cfg['helper'], native_reexecuted=False, binary_reexecuted_or_rebuilt=False,
                    qualification=False, s25_complete=False, coverage=copy.deepcopy(coverage), cells=copy.deepcopy(rows),
                    archives_verified=96, files_verified=999)
    return proof, verified, cfg


class Guards(unittest.TestCase):
    def test_complete_fixed_plan_and_full_pass(self):
        d.validate_plan(dict(format='spheres-stability-plan/v1', id='s25-full-v1', scope='full', cells=d.CELLS))
        p, v, c = good_records()
        d.validate_final(p, v, c, 0)

    def test_plan_rejects_pilot_missing_duplicate_seed_or_horizon(self):
        base = dict(format='spheres-stability-plan/v1', id='s25-full-v1', scope='full', cells=d.CELLS)
        mutations = [lambda p: p.update(scope='pilot'), lambda p: p['cells'].pop(),
                     lambda p: p['cells'].__setitem__(1, p['cells'][0]),
                     lambda p: p['cells'][0].update(seed=99), lambda p: p['cells'][0].update(through='2034-12-31')]
        for mutate in mutations:
            p = copy.deepcopy(base)
            mutate(p)
            with self.subTest(plan=p), self.assertRaises(ValueError):
                d.validate_plan(p)

    def test_integrity_and_pilot_are_not_full_pass(self):
        for key, value in [('declared', 2), ('attempted', 23), ('passed', 23), ('pilot_passed', True),
                           ('full_matrix_passed', False), ('missing_requested', ['india-1990']),
                           ('missing_full_cases', [{'country': 'India'}]), ('failed', ['france-7'])]:
            p, v, c = good_records()
            p['coverage'][key] = value
            v['coverage'][key] = value
            with self.subTest(key=key), self.assertRaises(ValueError):
                d.validate_final(p, v, c, 0)

    def test_failure_drift_interruption_halt_and_qualification_rejected(self):
        for key, value in [('revision', '1'*40), ('passed', False), ('binary_unchanged', False),
                           ('interrupted', True), ('resource_halted', True), ('qualification', True),
                           ('s25_complete', True), ('only_cell', 'france-1990'), ('jobs', 1)]:
            p, v, c = good_records()
            p[key] = value
            with self.subTest(key=key), self.assertRaises(ValueError):
                d.validate_final(p, v, c, 0)

    def test_count_claim_cannot_hide_missing_duplicate_reordered_or_failed_cell(self):
        changes = [lambda r: r.pop(), lambda r: r.__setitem__(1, r[0]),
                   lambda r: r.reverse(), lambda r: r[0].update(passed=False),
                   lambda r: r[0].update(id='unknown')]
        for location in ('proof', 'verification'):
            for mutate in changes:
                p, v, c = good_records()
                mutate((p if location == 'proof' else v)['cells'])
                with self.subTest(location=location, mutation=mutate), self.assertRaises(ValueError):
                    d.validate_final(p, v, c, 0)

    def test_wrong_verifier_or_source_native_execution_nonzero_exit_rejected(self):
        for key, value in [('candidate_revision', '2'*40), ('integrity_verified', False),
                           ('frozen_harness_sha256', '3'*64), ('verifier', {}), ('source', 'elsewhere'),
                           ('native_reexecuted', True), ('archives_verified', 95), ('selected_cell_passed', True)]:
            p, v, c = good_records()
            v[key] = value
            with self.subTest(key=key), self.assertRaises(ValueError):
                d.validate_final(p, v, c, 0)
        p, v, c = good_records()
        with self.assertRaises(ValueError):
            d.validate_final(p, v, c, 1)

    def test_completion_requires_actual_matching_child_exit_and_path(self):
        now = dt.datetime.now(dt.timezone.utc)
        c = dict(retained=str(Path.cwd() / 'retained'), launcher_started_utc=(now-dt.timedelta(seconds=2)).isoformat())
        r = dict(exit_code=0, elapsed_seconds=1, result=str(Path(c['retained']) / 'result.json'), finished_utc=now.isoformat())
        d.validate_exit(r, c, 0)
        # A normally completed failed run may still receive integrity verification; it cannot pass final gates.
        d.validate_exit({**r, 'exit_code': 1}, c, 1)
        for changed in [{**r, 'exit_code': 1}, {**r, 'result': 'other.json'},
                        {**r, 'elapsed_seconds': 0}, {**r, 'finished_utc': (now+dt.timedelta(days=1)).isoformat()}]:
            with self.assertRaises(ValueError):
                d.validate_exit(changed, c, 0)

    def test_pins_detect_same_length_drift_and_outputs_never_overwrite(self):
        with tempfile.TemporaryDirectory() as tmp:
            path = Path(tmp) / 'file'
            path.write_bytes(b'original')
            saved = d.pin(path)
            d.verify_pin(saved)
            path.write_bytes(b'changed!')
            with self.assertRaises(ValueError):
                d.verify_pin(saved)
            output = Path(tmp) / 'out.json'
            d.write_new(output, {'original': True})
            with self.assertRaises(FileExistsError):
                d.write_new(output, {'overwrite': True})
            self.assertEqual(d.read_json(output), {'original': True})

    def test_duplicate_keys_and_nonfinite_json_rejected(self):
        with tempfile.TemporaryDirectory() as tmp:
            path = Path(tmp) / 'record.json'
            for content in ('{"passed":false,"passed":true}', '{"value":NaN}'):
                path.write_text(content)
                with self.assertRaises(ValueError):
                    d.read_json(path)

    @unittest.skipUnless(os.name == 'nt', 'Exact Windows process handle test')
    def test_waits_exact_owned_synthetic_process_and_rejects_wrong_creation(self):
        with subprocess.Popen([sys.executable, '-B', '-c', 'import time; time.sleep(1.5)'],
                              creationflags=subprocess.CREATE_NO_WINDOW) as child:
            watched = d.WindowsProcess(child.pid)
            try:
                self.assertFalse(watched.poll())
                with self.assertRaisesRegex(ValueError, 'creation time mismatch'):
                    d.WindowsProcess(child.pid, watched.creation + 1)
                while not watched.poll(200):
                    pass
                self.assertEqual(watched.exit_code(), 0)
            finally:
                watched.close()


if __name__ == '__main__':
    unittest.main(verbosity=2)
