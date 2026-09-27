"""Exercise task queries and fail-closed queue validation in an isolated fixture."""
import json
from pathlib import Path
import shutil
import subprocess
import sys
import tempfile
import unittest


class WorkboardTests(unittest.TestCase):
    def setUp(self):
        self.temp = tempfile.TemporaryDirectory()
        self.addCleanup(self.temp.cleanup)
        self.root = Path(self.temp.name)
        (self.root/'tools/planning').mkdir(parents=True)
        shutil.copyfile(Path(__file__).with_name('workboard.py'), self.root/'tools/planning/workboard.py')
        (self.root/'docs/planning').mkdir(parents=True)
        (self.root/'handoff.md').write_text('Bounded work')
        self.write('docs/planning/ai-workstreams.json', {
            'integration_branch': 'codex/test', 'status_source': 'plan.json', 'task_source': 'tasks.json',
            'workstreams': [{'owner': 'Claude', 'sessions': ['C01'], 'next': 'Review research'}],
            'coordination_dependencies': {}})
        self.write('plan.json', {'sessions': [{'id': 'C01', 'depends': [], 'status': 'in_progress',
            'title': 'Research', 'output': 'Bounded evidence', 'accept': ['Source review']}],
            'last_completed_session': 'S18', 'next_session': 'S19'})
        self.tasks = [dict(id='REPAIR', owner='Claude', session='C01', state='queued', priority=1,
            title='Repair source', handoff='handoff.md', depends_on=[], next='Compare original content'),
            dict(id='REVIEW', owner='Codex', session='C01', state='blocked', priority=2,
            title='Independent review', handoff='handoff.md', depends_on=['REPAIR'], next='Inspect source')]

    def write(self, path, data):
        (self.root/path).write_text(json.dumps(data), encoding='utf-8')

    def run_board(self, *args):
        self.write('tasks.json', {'tasks': self.tasks})
        return subprocess.run([sys.executable, '-X', 'utf8', str(self.root/'tools/planning/workboard.py'), *args],
                              capture_output=True, text=True, encoding='utf-8')

    def test_owner_query_shows_only_own_tasks(self):
        p = self.run_board('--tasks', '--owner', 'Claude')
        self.assertEqual(p.returncode, 0, p.stderr)
        self.assertIn('REPAIR | Claude', p.stdout)
        self.assertNotIn('REVIEW | Codex', p.stdout)
        self.assertIn('C01 (in_progress)', p.stdout)

    def test_exact_task_reports_blocker(self):
        p = self.run_board('--task', 'REVIEW')
        self.assertEqual(p.returncode, 0, p.stderr)
        self.assertIn('Waiting for: REPAIR', p.stdout)

    def test_wrong_owner_is_an_error(self):
        p = self.run_board('--task', 'REVIEW', '--owner', 'Claude')
        self.assertNotEqual(p.returncode, 0)
        self.assertIn('requested owner', p.stderr)

    def test_unknown_task_is_an_error(self):
        self.assertNotEqual(self.run_board('--task', 'TYPO').returncode, 0)

    def test_invalid_dependency_and_cycle_fail_check(self):
        self.tasks[0]['depends_on'] = ['MISSING']
        self.assertIn('Unknown task dependency', self.run_board('--check').stderr)
        self.tasks[0]['depends_on'] = ['REVIEW']
        self.assertIn('dependency cycle', self.run_board('--check').stderr)

    def test_missing_handoff_and_duplicate_fail_check(self):
        self.tasks[0]['handoff'] = 'missing.md'
        self.assertIn('Missing task handoff', self.run_board('--check').stderr)
        self.tasks[0]['handoff'] = 'handoff.md'
        self.tasks.append(dict(self.tasks[0]))
        self.assertIn('Duplicate task', self.run_board('--check').stderr)

    def test_completed_task_hidden_but_queryable(self):
        self.tasks[0]['state'] = 'complete'
        self.assertNotIn('REPAIR | Claude', self.run_board('--tasks').stdout)
        self.assertIn('REPAIR | Claude | complete', self.run_board('--task', 'REPAIR').stdout)
        self.assertIn('no incomplete task dependency', self.run_board('--task', 'REVIEW').stdout)

    def test_canonical_query_is_preserved(self):
        p = self.run_board('--session', 'C01')
        self.assertEqual(p.returncode, 0, p.stderr)
        self.assertIn('C01 | Claude | in_progress', p.stdout)
        self.assertIn('Acceptance: Source review', p.stdout)
        self.assertIn('2 bounded tasks validated', self.run_board('--check').stdout)


if __name__ == '__main__':
    unittest.main()
