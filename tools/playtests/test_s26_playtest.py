"""Synthetic validator fixtures only: these are never human playtest results."""
import copy
from datetime import datetime, timedelta, timezone
import hashlib
import json
from pathlib import Path
import subprocess
import sys
import tempfile
import unittest

import s26_playtest as kit


class PlaytestRecords(unittest.TestCase):
    def setUp(self):
        self.tmp = tempfile.TemporaryDirectory()
        self.addCleanup(self.tmp.cleanup)
        self.root = Path(self.tmp.name)
        self.data = kit.template()
        for i, (kind, name, content) in enumerate([
                ('notes', 'synthetic-notes.txt', b'SYNTHETIC validator fixture, no human observed'),
                ('save', 'synthetic-save.json', b'{"synthetic_test_only": true}'),
                ('build', 'synthetic-build.bin', b'SYNTHETIC package bytes')], 1):
            (self.root / name).write_bytes(content)
            self.data['evidence'].append({'id': f'E{i:03}', 'path': name, 'kind': kind,
                                          'bytes': len(content), 'sha256': hashlib.sha256(content).hexdigest()})
        self.data['build'] = {'source_revision': '1' * 40, 'data_assets_revision': '2' * 40,
                              'package_sha256': self.data['evidence'][2]['sha256'], 'evidence_refs': ['E003']}
        self.data['study'] = {'id': 'S26-SYNTHETIC-TEST', 'cohort_locked_at_utc': '2026-01-01T00:00:00Z',
                              'first_time_cohort': [f'P{i:03}' for i in range(1, 6)],
                              'distinct_humans_attestation': 'SYNTHETIC test input; not an attestation about real people.'}
        self.data['participants'] = [{'code': f'P{i:03}', 'operator': 'human', 'independent': True,
                                     'first_time': True, 'eligibility_evidence': ['E001']} for i in range(1, 6)]
        countries = ['France', 'Japan', 'India', 'Brazil', 'Tonga', 'South Africa', 'Saudi Arabia', 'USSR → Russia']
        for i, country in enumerate(countries):
            start = datetime(2026, 1, 2, 10, tzinfo=timezone.utc) + timedelta(days=i)
            session = kit.session_template()
            session.update(id=f'S{i + 1:03}', state='observed', participant=f'P{i % 5 + 1:03}',
                           country_case=country, phase='opening' if i < 5 else 'later',
                           started_utc=start.isoformat(), ended_utc=(start + timedelta(hours=1)).isoformat(),
                           build_source_revision='1' * 40, evidence_refs=['E001'],
                           interaction_notes='SYNTHETIC fixture: keyboard/layout interactions recorded.')
            session['environment'] = {'os': 'SYNTHETIC OS', 'browser': 'SYNTHETIC 1',
                                      'viewport_width': 390 if i == 6 else 1440,
                                      'viewport_height': 900, 'keyboard_only': i == 5}
            session['campaign'] = {'nation': 'Russia' if i == 7 else country,
                                    'date': '1990-01-01' if i < 5 else '2030-01-01', 'seed': 1990,
                                    'origin': 'fresh' if i < 5 else 'ordinary_save',
                                    'start_save': None if i < 5 else 'E002',
                                    'provenance': 'SYNTHETIC fixture lineage; no campaign was run.'}
            for n, step in enumerate(kit.STEPS):
                session['tasks'][step] = {'outcome': 'complete', 'assistance': 'none',
                                           'at_utc': (start + timedelta(minutes=10 + n)).isoformat(),
                                           'notes': 'SYNTHETIC observed effect.',
                                           'evidence_refs': ['E002'] if step == 'save' else ['E001']}
            for key in ('government', 'actionable_blocker'):
                session[key] = {'outcome': 'correct', 'answer': 'SYNTHETIC response',
                                'basis': 'SYNTHETIC comparison', 'assistance': 'none', 'evidence_refs': ['E001']}
            session['actionable_blocker']['next_action'] = 'SYNTHETIC concrete recovery action.'
            self.data['sessions'].append(session)

    def report(self, data=None):
        return kit.audit(self.data if data is None else data, self.root)

    def assertInvalid(self, phrase, data=None):
        report = self.report(data)
        self.assertEqual(report['status'], 'invalid_records', report)
        self.assertIn(phrase, '\n'.join(report['errors']))
        self.assertFalse(report['qualification_awarded'])

    def test_empty_shipped_plan_has_no_people_results_or_qualification(self):
        report = self.report(kit.template())
        self.assertEqual(report['errors'], [])
        self.assertEqual(report['status'], 'coverage_incomplete')
        self.assertEqual(report['observed_human_sessions'], 0)
        self.assertEqual(report['first_time_humans'], 0)
        self.assertEqual(len(report['country_coverage']), 8)
        self.assertTrue(all(not x for x in report['country_coverage'].values()))
        self.assertFalse(report['s26_complete'])

    def test_complete_synthetic_record_is_only_ready_for_review_never_certified(self):
        report = self.report()
        self.assertEqual(report['errors'], [])
        self.assertEqual(report['missing'], [])
        self.assertEqual(report['status'], 'ready_for_human_review')
        self.assertEqual(report['observed_human_sessions'], 8)
        self.assertEqual(report['first_time_humans'], 5)
        self.assertFalse(report['s26_complete'])
        self.assertFalse(report['qualification_awarded'])

    def test_duplicate_people_cohort_and_sessions_are_rejected(self):
        for field in ('participants', 'sessions'):
            with self.subTest(field=field):
                data = copy.deepcopy(self.data)
                data[field].append(copy.deepcopy(data[field][0]))
                self.assertInvalid('Duplicate', data)
        data = copy.deepcopy(self.data)
        planned = kit.session_template()
        planned['id'] = 'S001'
        data['sessions'].append(planned)
        self.assertInvalid('Duplicate session', data)
        self.data['study']['first_time_cohort'][1] = 'P001'
        self.assertInvalid('Duplicate participant in the declared')

    def test_agent_and_non_independent_sessions_never_count(self):
        for change in ({'operator': 'agent'}, {'independent': False}):
            with self.subTest(change=change):
                data = copy.deepcopy(self.data)
                data['participants'][0].update(change)
                report = self.report(data)
                self.assertEqual(report['observed_human_sessions'], 6)
                self.assertEqual(len(report['excluded_sessions']), 2)
                self.assertTrue(report['errors'])

    def test_unplayed_slots_and_planned_session_templates_give_no_credit(self):
        data = kit.template()
        data['sessions'] = [kit.session_template() for _ in range(20)]
        report = self.report(data)
        self.assertEqual(report['observed_human_sessions'], 0)
        self.assertEqual(report['planned_sessions'], 28)
        self.assertEqual(report['status'], 'coverage_incomplete')

    def test_one_coached_newcomer_allowed_but_two_fail_four_of_five(self):
        self.data['sessions'][0]['tasks']['budget']['assistance'] = 'facilitator'
        self.assertEqual(self.report()['status'], 'ready_for_human_review')
        self.data['sessions'][1]['tasks']['save']['assistance'] = 'facilitator'
        self.assertEqual(self.report()['status'], 'coverage_incomplete')
        self.assertEqual(sum(x['uncoached_opening_success'] for x in self.report()['opening_path_sample']), 3)

    def test_in_game_advisor_is_not_facilitator_coaching(self):
        for s in self.data['sessions']:
            for step in s['tasks'].values():
                step['assistance'] = 'in_game'
        self.assertEqual(self.report()['status'], 'ready_for_human_review')

    def test_navigation_coaching_event_overrides_claimed_uncoached_steps(self):
        for s in self.data['sessions'][:2]:
            s['coaching_events'] = [{'at_utc': s['tasks']['budget']['at_utc'],
                                     'task': 'navigation', 'notes': 'Facilitator pointed to the budget button.'}]
        self.assertEqual(self.report()['status'], 'coverage_incomplete')

    def test_retry_does_not_replace_first_attempt_or_make_person_new_again(self):
        for s in self.data['sessions'][:2]:
            s['tasks']['budget']['assistance'] = 'facilitator'
            retry = copy.deepcopy(s)
            retry['id'] += '99'
            retry['started_utc'] = '2026-02-01T10:00:00Z'
            retry['ended_utc'] = '2026-02-01T11:00:00Z'
            for step in retry['tasks'].values():
                step['at_utc'] = '2026-02-01T10:30:00Z'
                step['assistance'] = 'none'
            self.data['sessions'].append(retry)
        report = self.report()
        self.assertEqual(report['errors'], [])
        self.assertEqual(report['status'], 'coverage_incomplete')
        self.assertEqual(report['opening_path_sample'][0]['first_session'], 'S001')

    def test_aborted_attempt_is_retained_and_does_not_count_as_country_session(self):
        self.data['sessions'][0].update(state='aborted', abort_reason='SYNTHETIC participant stopped.')
        report = self.report()
        self.assertEqual(report['errors'], [])
        self.assertEqual(report['observed_human_sessions'], 7)
        self.assertEqual(report['country_coverage']['France'], [])
        self.assertFalse(report['opening_path_sample'][0]['uncoached_opening_success'])

    def test_missing_identification_from_additional_human_is_not_ignored(self):
        self.data['participants'].append({'code': 'P006', 'operator': 'human', 'independent': True,
                                         'first_time': False, 'eligibility_evidence': ['E001']})
        s = self.data['sessions'][-1]
        s['participant'] = 'P006'
        s['government']['outcome'] = 'incorrect'
        s['actionable_blocker']['outcome'] = 'not_answered'
        report = self.report()
        self.assertEqual(report['errors'], [])
        self.assertTrue(any('P006' in x for x in report['missing']))

    def test_correct_blocker_needs_an_action_and_comparison_basis(self):
        self.data['sessions'][0]['actionable_blocker']['next_action'] = ''
        self.assertInvalid('actionable next step')
        self.data['sessions'][0]['actionable_blocker']['basis'] = None
        self.assertInvalid('participant answer and observer comparison')

    def test_country_keyboard_narrow_and_later_coverage_are_independent(self):
        for name, mutate, missing in [
            ('country', lambda d: d['sessions'].pop(), 'USSR → Russia'),
            ('keyboard', lambda d: d['sessions'][5]['environment'].update(keyboard_only=False), 'keyboard-only'),
            ('narrow', lambda d: d['sessions'][6]['environment'].update(viewport_width=1440), '390px'),
            ('later', lambda d: [s.update(phase='opening') for s in d['sessions']], 'later-game')]:
            with self.subTest(name=name):
                data = copy.deepcopy(self.data)
                mutate(data)
                self.assertIn(missing, '\n'.join(self.report(data)['missing']))

    def test_exact_build_save_provenance_and_file_integrity_are_required(self):
        for change, phrase in [
            (lambda d: d['sessions'][0].update(build_source_revision='3' * 40), 'mixed build'),
            (lambda d: d['build'].update(package_sha256='a' * 64), 'actual pinned build'),
            (lambda d: d['sessions'][-1]['campaign'].update(start_save=None), 'evidence_refs'),
            (lambda d: d['sessions'][-1]['campaign'].update(provenance=''), 'lineage'),
            (lambda d: d['sessions'][-1]['campaign'].update(nation='France'), 'actual nation'),
            (lambda d: d['sessions'][0]['tasks']['save'].update(evidence_refs=['E001']), 'save evidence')]:
            with self.subTest(phrase=phrase):
                data = copy.deepcopy(self.data)
                change(data)
                self.assertInvalid(phrase, data)
        (self.root / 'synthetic-notes.txt').write_text('changed', encoding='utf-8')
        self.assertInvalid('mismatch')

    def test_no_evidence_verification_never_reports_ready(self):
        report = kit.audit(self.data)
        self.assertEqual(report['status'], 'coverage_incomplete')
        self.assertIn('local evidence root', '\n'.join(report['missing']))

    def test_unknown_duplicate_and_escaping_evidence_refused(self):
        for path in ('../secret', '/absolute', 'C:/absolute', 'dir\\file'):
            with self.subTest(path=path):
                data = copy.deepcopy(self.data)
                data['evidence'][0]['path'] = path
                self.assertInvalid('evidence path', data)
        self.data['sessions'][0]['evidence_refs'] = ['E999']
        self.assertInvalid('unknown evidence reference')

    def test_time_and_cohort_postselection_errors(self):
        self.data['study']['cohort_locked_at_utc'] = '2026-03-01T00:00:00Z'
        self.assertInvalid('precede the declared cohort')
        self.data['study']['cohort_locked_at_utc'] = '2026-01-01T00:00:00'
        self.assertInvalid('aware lock timestamp')

    def test_path_order_and_same_human_overlap_are_rejected(self):
        original = copy.deepcopy(self.data)
        self.data['sessions'][0]['tasks']['budget']['at_utc'] = self.data['sessions'][0]['tasks']['save']['at_utc']
        self.assertInvalid('out of budget/construction/result/save order')
        self.data = original
        first, later = self.data['sessions'][0], self.data['sessions'][5]
        later['started_utc'], later['ended_utc'] = first['started_utc'], first['ended_utc']
        for step in kit.STEPS:
            later['tasks'][step]['at_utc'] = first['tasks'][step]['at_utc']
        self.assertInvalid('overlapping sessions')

    def test_blocked_tasks_need_linked_persistent_failure(self):
        s = self.data['sessions'][0]
        s['tasks']['construction']['outcome'] = 'blocked'
        self.assertInvalid('linked failure')
        s['failures'] = [{'id': 'S26-F001', 'task': 'construction', 'severity': 2,
                           'notes': 'SYNTHETIC primary control unclear.', 'evidence_refs': ['E001']}]
        report = self.report()
        self.assertEqual(report['errors'], [])
        self.assertEqual(report['failures'][0]['id'], 'S26-F001')
        self.assertFalse(report['opening_path_sample'][0]['uncoached_opening_success'])

    def test_malformed_input_is_invalid_without_crashing(self):
        for data in (None, [], {}, {'format': kit.FORMAT, 'participants': [None], 'sessions': []}):
            with self.subTest(data=data):
                self.assertEqual(kit.audit(data, self.root)['status'], 'invalid_records')

    def test_cli_reports_empty_plan_nonzero_and_refuses_overwriting_records(self):
        script = Path(kit.__file__).resolve()
        target = self.root / 'new-study'
        init = subprocess.run([sys.executable, str(script), 'init', str(target)], capture_output=True)
        self.assertEqual(init.returncode, 0, init.stderr)
        before = (target / 'records.json').read_bytes()
        again = subprocess.run([sys.executable, str(script), 'init', str(target)], capture_output=True)
        self.assertEqual(again.returncode, 2)
        self.assertEqual((target / 'records.json').read_bytes(), before)
        checked = subprocess.run([sys.executable, str(script), 'report', str(target / 'records.json')], capture_output=True)
        self.assertEqual(checked.returncode, 1)
        self.assertEqual(json.loads(checked.stdout)['observed_human_sessions'], 0)

    def test_duplicate_json_fields_cannot_hide_an_earlier_session_list(self):
        record = self.root / 'duplicate.json'
        record.write_text('{"sessions": ["retained failure"], "sessions": []}', encoding='utf-8')
        result = subprocess.run([sys.executable, kit.__file__, 'report', str(record)], capture_output=True)
        self.assertEqual(result.returncode, 2)
        self.assertIn(b'Duplicate JSON field: sessions', result.stderr)

    def test_pin_reads_exact_bytes_and_cannot_escape_evidence_root(self):
        source = self.root / 'synthetic-notes.txt'
        before = source.read_bytes()
        command = [sys.executable, kit.__file__, 'pin', str(source), '--root', str(self.root),
                   '--id', 'E009', '--kind', 'notes']
        result = subprocess.run(command, capture_output=True)
        self.assertEqual(result.returncode, 0, result.stderr)
        metadata = json.loads(result.stdout)
        self.assertEqual(metadata['sha256'], hashlib.sha256(before).hexdigest())
        self.assertEqual(metadata['bytes'], len(before))
        self.assertEqual(source.read_bytes(), before)
        command[command.index('--root') + 1] = str(self.root / 'elsewhere')
        result = subprocess.run(command, capture_output=True)
        self.assertEqual(result.returncode, 2)


if __name__ == '__main__':
    unittest.main()
