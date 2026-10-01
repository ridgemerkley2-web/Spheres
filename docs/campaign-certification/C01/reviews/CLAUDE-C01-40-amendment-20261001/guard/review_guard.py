"""Proposed tests-only amendment; in-memory fixtures, no source or repo edits."""
from pathlib import Path
import copy
import datetime
import difflib
import hashlib
import io
import json
import subprocess
import sys
import types
import unittest

ROOT = Path('C:/Users/ridge/Documents/Codex/2026-09-05/pick-up-the-spheres-game-on/work/campaign-certification/integration')
OUT = Path(__file__).resolve().parent
REF = '5054de7bfa94eb884161da26a63869409f334fd5'
PATH = 'tools/avatars/test_india_cpim_general_secretaries_c01_40.py'


def blob(path):
    return subprocess.check_output(['git', 'show', REF + ':' + path], cwd=ROOT)


NEW_TEST = '''    def test_styling_date_does_not_override_independently_cited_office_evidence(self):
        # Synthetic date collisions exercise generic evidence rules only. They
        # do not change the selected historical observations or source records.
        for name, day in NEWLY_ELECTED:
            with self.subTest(name=name, day=day):
                packet, rows = copy.deepcopy(self.packet), copy.deepcopy(self.rows)
                claims = {c['id']: c for s in packet['sources'] for c in s['claims']}
                sources = {c['id']: s['id'] for s in packet['sources'] for c in s['claims']}
                holder = next(h for h in cpm_role(packet)['holder_claims'] if h['name'] == name)
                direct, = [c for c in holder['claim_ids'] if rows[c]['event_kind'] in HOLDER_KINDS]
                holder['attested_on'] = day
                claims[direct]['attested_on'] = rows[direct]['attested_on'] = day
                synthetic = f'Synthetic explicit office attestation: {name}, General Secretary, on {day}.'
                claims[direct]['text'] = rows[direct]['text'] = synthetic
                cpm_rules(packet, rows)
                # The historical fixture is still immutable even though this
                # independent synthetic evidence satisfies the generic rules.
                with self.assertRaises(AssertionError):
                    cpm_invariants(packet, rows)
                styling, = [c for c in cpm_role(packet)['claim_ids']
                            if rows[c]['event_kind'] in NEWLY_ELECTED_KINDS
                            and rows[c]['holder_name'] == name and claims[c].get('attested_on') == day]
                holder['claim_ids'] = [styling] + [c for c in holder['claim_ids'] if c != direct]
                holder['sources'] = list(dict.fromkeys(sources[c] for c in holder['claim_ids']))
                # Same person/day, but citing the styling instead of direct
                # office evidence remains insufficient under this amendment.
                with self.assertRaises(AssertionError):
                    cpm_rules(packet, rows)

'''


def write_new(name, raw):
    with (OUT / name).open('xb') as stream:
        stream.write(raw)


def main():
    original = blob(PATH).decode('utf-8')
    marker = '    def test_starts_and_ends_only_where_a_source_states_one(self):\n'
    assert original.count(marker) == 1
    red = original.replace(marker, NEW_TEST + marker)
    styled = "    # A 'newly elected' styling on or for the election never dates the holder it names.\n" \
             "    styled = {(rows[c]['holder_name'], claims[c].get('attested_on')) for c in role['claim_ids']\n" \
             "              if rows[c]['event_kind'] in NEWLY_ELECTED_KINDS}\n"
    guard = '        assert (name, holder[\'attested_on\']) not in styled, (name, "a \'newly elected\' styling is not an observation")\n'
    date_guard = "        assert not {holder['attested_on'], holder['from'], holder['until']} & set(NEVER_HOLDER_DATE), name\n"
    fixed = red
    for text in (styled, guard, date_guard):
        assert fixed.count(text) == 1, text
        fixed = fixed.replace(text, '')
    pinned = '    assert got == HOLDERS, got\n'
    fixed = fixed.replace(pinned, pinned +
        "    # This blacklist describes this exact fixture, not every possible\n"
        "    # independently supported observation that could share an event day.\n"
        "    for holder in role['holder_claims']:\n"
        "        assert not {holder['attested_on'], holder['from'], holder['until']} & set(NEVER_HOLDER_DATE), holder['name']\n")
    patch = ''.join(difflib.unified_diff(original.splitlines(True), fixed.splitlines(True),
                    fromfile='a/' + PATH, tofile='b/' + PATH))
    write_new('claim-driven-guard.patch', patch.encode('utf-8'))
    sys.path.insert(0, str(ROOT / 'tools/avatars'))
    packet = json.loads(blob('docs/campaign-certification/C01/research/india.json'))
    run_records = []
    for label, code in [('red-original-rules', red), ('green-claim-driven-rules', fixed)]:
        module = types.ModuleType(label.replace('-', '_'))
        module.__file__ = str(ROOT / PATH)
        exec(compile(code, module.__file__, 'exec'), module.__dict__)
        cls = module.IndiaCpimGeneralSecretariesTests
        cls.packet = copy.deepcopy(packet)
        cls.sources = {s['id']: s for s in packet['sources']}
        cls.claims = {c['id']: c for s in packet['sources'] for c in s['claims']}
        cls.org = module.cpm_org(cls.packet)
        cls.role = module.cpm_role(cls.packet)
        cls.extracts = {sid: json.loads(blob(cls.sources[sid]['snapshot']['path'])) for sid in module.NEW_SOURCES}
        cls.rows = {row['claim_id']: row for extract in cls.extracts.values() for row in extract['rows']}
        cls.setUpClass = classmethod(lambda cls: None)
        tests = ['test_styling_date_does_not_override_independently_cited_office_evidence']
        if label.startswith('green'):
            tests.append('test_holders_are_exactly_as_intended')
            tests.extend(name for name in dir(cls) if name.startswith('test_') and 'mutation' in name)
        suite = unittest.TestSuite(cls(name) for name in tests)
        stream = io.StringIO()
        result = unittest.TextTestRunner(stream=stream, verbosity=2).run(suite)
        output = stream.getvalue().encode('utf-8')
        write_new(label + '.log', output)
        row = {'label': label, 'tests': tests, 'tests_run': result.testsRun,
               'failures': len(result.failures), 'errors': len(result.errors),
               'passed': result.wasSuccessful(), 'log_sha256': hashlib.sha256(output).hexdigest()}
        run_records.append(row)
        print(json.dumps(row))
    receipt = {'checked_utc': datetime.datetime.now(datetime.timezone.utc).isoformat(),
               'submission_revision': REF, 'input_test_blob_sha256': hashlib.sha256(original.encode()).hexdigest(),
               'patch_sha256': hashlib.sha256(patch.encode()).hexdigest(), 'runs': run_records,
               'scope': 'Synthetic in-memory generic-rule regression and unchanged exact-holder checks; no historical source validation or network/repository mutation.'}
    write_new('result.json', (json.dumps(receipt, indent=2) + '\n').encode())
    assert run_records[0]['failures'] == 3 and run_records[0]['errors'] == 0
    assert run_records[1]['passed']


if __name__ == '__main__':
    main()
