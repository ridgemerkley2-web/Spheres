#!/usr/bin/env python3
"""Prepare and audit anonymous S26 observations; never award qualification."""
import argparse
from datetime import date, datetime, timezone
import hashlib
import json
from pathlib import Path
import re
import sys

FORMAT = 'spheres-human-playtest/v1'
PROTOCOL_REVISION = 'd2b21c15ad8070ad9bde5363d888a31e1c4dc63c'
COUNTRIES = ('France', 'Japan', 'India', 'Brazil', 'South Africa', 'Tonga',
             'Saudi Arabia', 'USSR → Russia')
STEPS = ('budget', 'construction', 'recorded_result', 'save')
OUTCOMES = ('complete', 'blocked', 'not_attempted')
HELP = ('none', 'in_game', 'facilitator')


def template():
    return {
        'format': FORMAT,
        'protocol_source_revision': PROTOCOL_REVISION,
        'study': {'id': 'S26-STUDY-001', 'cohort_locked_at_utc': None,
                  'first_time_cohort': [], 'distinct_humans_attestation': None},
        'build': {'source_revision': None, 'data_assets_revision': None,
                  'package_sha256': None, 'evidence_refs': []},
        'planned_sessions': [
            {'slot': f'PLAN-{i + 1:02}', 'country_case': country,
             'participant': None,
             'phase': 'opening' if country in ('France', 'Japan', 'India', 'Brazil', 'Tonga') else 'later',
             'keyboard_only': country == 'South Africa',
             'viewport_width': 390 if country == 'Saudi Arabia' else 1440}
            for i, country in enumerate(COUNTRIES)],
        'participants': [], 'sessions': [], 'evidence': []}


def session_template():
    return {
        'id': None, 'state': 'planned', 'participant': None, 'country_case': None,
        'phase': None, 'started_utc': None, 'ended_utc': None,
        'build_source_revision': None,
        'environment': {'os': None, 'browser': None, 'viewport_width': None,
                        'viewport_height': None, 'keyboard_only': None},
        'campaign': {'nation': None, 'date': None, 'seed': None, 'origin': None,
                     'start_save': None, 'provenance': None},
        'evidence_refs': [], 'interaction_notes': None, 'abort_reason': None,
        'tasks': {step: {'outcome': 'not_attempted', 'assistance': 'none',
                         'at_utc': None, 'notes': None, 'evidence_refs': []}
                  for step in STEPS},
        'government': {'outcome': 'not_answered', 'answer': None, 'basis': None,
                       'assistance': 'none', 'evidence_refs': []},
        'actionable_blocker': {'outcome': 'not_answered', 'answer': None, 'basis': None,
                              'next_action': None, 'assistance': 'none', 'evidence_refs': []},
        'coaching_events': [], 'failures': []}


def _time(value):
    if not isinstance(value, str):
        return None
    try:
        parsed = datetime.fromisoformat(value.replace('Z', '+00:00'))
        return parsed.astimezone(timezone.utc) if parsed.tzinfo else None
    except ValueError:
        return None


def _text(value):
    return isinstance(value, str) and bool(value.strip())


def _hex(value, length):
    return isinstance(value, str) and re.fullmatch(f'[0-9a-f]{{{length}}}', value) is not None


def _sha(path):
    digest = hashlib.sha256()
    with path.open('rb') as stream:
        for chunk in iter(lambda: stream.read(1024 * 1024), b''):
            digest.update(chunk)
    return digest.hexdigest()


def _unique_object(pairs):
    result = {}
    for key, value in pairs:
        if key in result:
            raise ValueError(f'Duplicate JSON field: {key}')
        result[key] = value
    return result


def audit(data, evidence_root=None):
    """Validate claimed observations, separately reporting coverage and integrity.

    Files are verified only under the explicitly supplied local root. A report
    without file verification can never be ready for human review. Anonymous
    codes cannot themselves prove that distinct real people participated.
    """
    errors, gaps = [], []
    if not isinstance(data, dict):
        data = {}
        errors.append('Record must be a JSON object.')
    if data.get('format') != FORMAT:
        errors.append('Unsupported record format.')
    if data.get('protocol_source_revision') != PROTOCOL_REVISION:
        errors.append('Protocol reference changed; review rather than silently changing criteria.')

    def rows(key):
        value = data.get(key)
        if not isinstance(value, list) or any(not isinstance(row, dict) for row in value):
            errors.append(f'{key}: expected a list of objects.')
            return []
        return value

    evidence, verified = {}, []
    root = Path(evidence_root).resolve() if evidence_root is not None else None
    for item in rows('evidence'):
        identity = item.get('id')
        if not isinstance(identity, str) or not re.fullmatch(r'E[0-9]{3,6}', identity):
            errors.append('Evidence IDs must be anonymous E001-style codes.')
            continue
        if identity in evidence:
            errors.append(f'Duplicate evidence ID: {identity}.')
            continue
        evidence[identity] = item
        path = item.get('path')
        if (not _text(path) or '\\' in path or ':' in path or path.startswith('/')
                or '..' in path.split('/')):
            errors.append(f'{identity}: use a relative forward-slash evidence path inside the evidence root.')
            continue
        if (not _hex(item.get('sha256'), 64) or type(item.get('bytes')) is not int
                or item['bytes'] < 1 or item.get('kind') not in ('notes', 'image', 'video', 'save', 'build')):
            errors.append(f'{identity}: require nonempty raw bytes, SHA-256 and evidence kind.')
            continue
        if root is not None:
            try:
                resolved = (root / path).resolve()
                if not resolved.is_relative_to(root):
                    errors.append(f'{identity}: evidence path escapes its root.')
                elif not resolved.is_file():
                    errors.append(f'{identity}: evidence file is missing.')
                elif resolved.stat().st_size != item['bytes'] or _sha(resolved) != item['sha256']:
                    errors.append(f'{identity}: evidence bytes/hash mismatch.')
                else:
                    verified.append(identity)
            except OSError as error:
                errors.append(f'{identity}: evidence cannot be read ({error.__class__.__name__}).')

    def refs(value, label, required=False, kind=None):
        if not isinstance(value, list) or any(not isinstance(x, str) for x in value):
            errors.append(f'{label}: evidence_refs must be a list of evidence IDs.')
            return
        if len(value) != len(set(value)) or any(x not in evidence for x in value):
            errors.append(f'{label}: duplicate or unknown evidence reference.')
        if required and not value:
            errors.append(f'{label}: actual observations need evidence.')
        if kind and not any(evidence.get(x, {}).get('kind') == kind for x in value):
            errors.append(f'{label}: require {kind} evidence.')

    participants = {}
    for person in rows('participants'):
        code = person.get('code')
        if not isinstance(code, str) or not re.fullmatch(r'P[0-9]{3,6}', code):
            errors.append('Participant codes must use anonymous P001-style identifiers, not names/contact details.')
            continue
        if code in participants:
            errors.append(f'Duplicate participant: {code}.')
            continue
        participants[code] = person
        if (person.get('operator') not in ('human', 'agent')
                or type(person.get('independent')) is not bool
                or type(person.get('first_time')) is not bool):
            errors.append(f'{code}: explicitly record operator, independence and first-time status.')
        refs(person.get('eligibility_evidence'), code, required=True)

    study = data.get('study') if isinstance(data.get('study'), dict) else {}
    cohort = study.get('first_time_cohort', [])
    if not isinstance(cohort, list) or any(not isinstance(x, str) for x in cohort):
        errors.append('First-time cohort must be a list of participant codes.')
        cohort = []
    if len(cohort) != len(set(cohort)):
        errors.append('Duplicate participant in the declared five-person cohort.')
    cohort_time = _time(study.get('cohort_locked_at_utc'))
    if len(cohort) != 5:
        gaps.append('Declare five distinct first-time humans for the 4-of-5 opening-path sample before sessions begin.')
    if cohort and (cohort_time is None or not _text(study.get('distinct_humans_attestation'))):
        errors.append('A declared cohort needs an aware lock timestamp and a facilitator attestation of distinct humans.')
    for code in cohort:
        person = participants.get(code, {})
        if person.get('operator') != 'human' or person.get('independent') is not True or person.get('first_time') is not True:
            errors.append(f'{code}: cohort member must be an explicitly independent first-time human.')

    build = data.get('build') if isinstance(data.get('build'), dict) else {}
    sessions, seen, all_ids, excluded, issues = [], set(), set(), [], []
    planned_count = len(rows('planned_sessions'))
    for session in rows('sessions'):
        state = session.get('state')
        identity = session.get('id')
        if isinstance(identity, str):
            if identity in all_ids:
                errors.append(f'Duplicate session: {identity}.')
                continue
            all_ids.add(identity)
        if state == 'planned':
            planned_count += 1
            continue
        if not isinstance(identity, str) or not re.fullmatch(r'S[0-9]{3,6}', identity):
            errors.append('Actual session IDs must use S001-style identifiers.')
            continue
        seen.add(identity)
        before = len(errors)
        if state not in ('observed', 'aborted'):
            errors.append(f'{identity}: session must be planned, observed or aborted.')
        code = session.get('participant')
        if not isinstance(code, str) or code not in participants:
            errors.append(f'{identity}: unknown participant.')
            person = {}
        else:
            person = participants[code]
        if session.get('country_case') not in COUNTRIES or session.get('phase') not in ('opening', 'later'):
            errors.append(f'{identity}: declare a CP1 country case and opening/later phase.')
        start, end = _time(session.get('started_utc')), _time(session.get('ended_utc'))
        if start is None or end is None or start >= end:
            errors.append(f'{identity}: require ordered aware start/end timestamps.')
        if start is not None and cohort_time is not None and start < cohort_time:
            errors.append(f'{identity}: observations precede the declared cohort lock.')
        if session.get('build_source_revision') != build.get('source_revision'):
            errors.append(f'{identity}: mixed build; report a changed candidate in a separate study.')
        env = session.get('environment') if isinstance(session.get('environment'), dict) else {}
        if (not _text(env.get('os')) or not _text(env.get('browser'))
                or any(type(env.get(k)) is not int or env[k] <= 0 for k in ('viewport_width', 'viewport_height'))
                or type(env.get('keyboard_only')) is not bool):
            errors.append(f'{identity}: record OS, browser version, actual viewport and keyboard use.')
        if not _text(session.get('interaction_notes')):
            errors.append(f'{identity}: record actual interaction observations.')
        refs(session.get('evidence_refs'), identity, required=True)
        campaign = session.get('campaign') if isinstance(session.get('campaign'), dict) else {}
        try:
            date.fromisoformat(campaign.get('date', ''))
        except (ValueError, TypeError):
            errors.append(f'{identity}: record the actual starting campaign date.')
        nations = ('USSR', 'Russia') if session.get('country_case') == 'USSR → Russia' else (session.get('country_case'),)
        if campaign.get('nation') not in nations or type(campaign.get('seed')) is not int:
            errors.append(f'{identity}: country case must match the actual nation; record the seed.')
        if campaign.get('origin') not in ('fresh', 'ordinary_save', 'authored_fixture'):
            errors.append(f'{identity}: disclose campaign/save origin.')
        if not _text(campaign.get('provenance')):
            errors.append(f'{identity}: describe source/save lineage and any authored fixture.')
        if campaign.get('origin') != 'fresh' or session.get('phase') == 'later':
            refs([campaign.get('start_save')], identity + ' start_save', required=True, kind='save')
            if campaign.get('origin') == 'fresh':
                errors.append(f'{identity}: later-game coverage requires a sourced save, not a fresh start.')
        tasks = session.get('tasks') if isinstance(session.get('tasks'), dict) else {}
        step_times, performed = [], False
        for step in STEPS:
            task = tasks.get(step) if isinstance(tasks.get(step), dict) else {}
            outcome = task.get('outcome')
            if outcome not in OUTCOMES or task.get('assistance') not in HELP:
                errors.append(f'{identity}/{step}: explicit outcome and assistance required.')
            if outcome in ('complete', 'blocked'):
                performed = True
                when = _time(task.get('at_utc'))
                if when is None or start is None or end is None or not start <= when <= end:
                    errors.append(f'{identity}/{step}: observation time must fall within the session.')
                elif outcome == 'complete':
                    step_times.append(when)
                if not _text(task.get('notes')):
                    errors.append(f'{identity}/{step}: describe observed result or failure.')
                refs(task.get('evidence_refs'), f'{identity}/{step}', required=True,
                     kind='save' if step == 'save' and outcome == 'complete' else None)
        if step_times != sorted(step_times):
            errors.append(f'{identity}: completed path steps are out of budget/construction/result/save order.')
        for key in ('government', 'actionable_blocker'):
            answer = session.get(key) if isinstance(session.get(key), dict) else {}
            if answer.get('outcome') not in ('correct', 'incorrect', 'not_answered') or answer.get('assistance') not in HELP:
                errors.append(f'{identity}/{key}: explicit answer outcome and assistance required.')
            if answer.get('outcome') in ('correct', 'incorrect'):
                performed = True
                if not _text(answer.get('answer')) or not _text(answer.get('basis')):
                    errors.append(f'{identity}/{key}: retain the participant answer and observer comparison with current UI.')
                if key == 'actionable_blocker' and answer.get('outcome') == 'correct' and not _text(answer.get('next_action')):
                    errors.append(f'{identity}: a correct blocker needs an actionable next step.')
                refs(answer.get('evidence_refs'), f'{identity}/{key}', required=True)
        coaching = session.get('coaching_events')
        if not isinstance(coaching, list) or any(not isinstance(x, dict) for x in coaching):
            errors.append(f'{identity}: coaching_events must be a list.')
            coaching = []
        for event in coaching:
            when = _time(event.get('at_utc'))
            if (event.get('task') not in (*STEPS, 'government', 'actionable_blocker', 'navigation')
                    or not _text(event.get('notes')) or when is None or start is None or end is None
                    or not start <= when <= end):
                errors.append(f'{identity}: coaching event needs an in-session time, task and description.')
        failures = session.get('failures')
        if not isinstance(failures, list) or any(not isinstance(x, dict) for x in failures):
            errors.append(f'{identity}: failures must be a list.')
            failures = []
        for failure in failures:
            if (not isinstance(failure.get('id'), str) or not re.fullmatch(r'S26-F[0-9]{3,6}', failure['id'])
                    or type(failure.get('severity')) is not int or failure['severity'] not in range(4)
                    or not _text(failure.get('notes'))):
                errors.append(f'{identity}: failure needs stable S26-F001-style ID, severity 0–3 and notes.')
            refs(failure.get('evidence_refs'), identity + ' failure', required=True)
            issues.append({'session': identity, **failure})
        for step in STEPS:
            if isinstance(tasks.get(step), dict) and tasks[step].get('outcome') == 'blocked' and not any(f.get('task') == step for f in failures):
                errors.append(f'{identity}/{step}: blocked task needs a linked failure record.')
        if state == 'aborted' and not _text(session.get('abort_reason')):
            errors.append(f'{identity}: retain why the session ended early.')
        if state == 'observed' and not performed:
            errors.append(f'{identity}: an empty session is not observed coverage.')
        if person.get('operator') != 'human' or person.get('independent') is not True:
            excluded.append({'session': identity, 'reason': 'Agent-operated or non-independent session; no human credit.'})
        elif len(errors) == before:
            sessions.append(session)

    if seen:
        if not _hex(build.get('source_revision'), 40) or not _hex(build.get('data_assets_revision'), 40) or not _hex(build.get('package_sha256'), 64):
            errors.append('Actual observations need exact Git source/data/assets revisions and package SHA-256.')
        refs(build.get('evidence_refs'), 'build', required=True, kind='build')
        if isinstance(build.get('evidence_refs'), list) and not any(
                isinstance(x, str) and evidence.get(x, {}).get('kind') == 'build'
                and evidence[x].get('sha256') == build.get('package_sha256')
                for x in build['evidence_refs']):
            errors.append('Package SHA-256 must match the actual pinned build artifact.')
    for code in participants:
        attempts = sorted([s for s in sessions if s['participant'] == code], key=lambda s: _time(s['started_utc']))
        if any(_time(a['ended_utc']) > _time(b['started_utc']) for a, b in zip(attempts, attempts[1:])):
            errors.append(f'{code}: overlapping sessions cannot be independent observations by one person.')
    actual = [s for s in sessions if s['state'] == 'observed']
    actual_people = sorted({s['participant'] for s in sessions})
    first_time = [p for p in actual_people if participants[p].get('first_time') is True]
    opening = []
    for code in cohort:
        attempts = sorted([s for s in sessions if s['participant'] == code], key=lambda s: _time(s['started_utc']))
        first = attempts[0] if attempts else None
        success = bool(first and first['state'] == 'observed' and first['phase'] == 'opening'
                       and all(first['tasks'][step]['outcome'] == 'complete'
                               and first['tasks'][step]['assistance'] != 'facilitator' for step in STEPS)
                       and not any(e['task'] in (*STEPS, 'navigation') for e in first['coaching_events']))
        opening.append({'participant': code, 'first_session': first['id'] if first else None,
                        'uncoached_opening_success': success})
    identifications = []
    for code in actual_people:
        observations = [s for s in sessions if s['participant'] == code]
        identifications.append({'participant': code,
                                **{key: any(s[key]['outcome'] == 'correct' for s in observations)
                                   for key in ('government', 'actionable_blocker')},
                                'sessions': [s['id'] for s in observations]})
    covered = {s['country_case'] for s in actual}
    coverage = {country: [s['id'] for s in actual if s['country_case'] == country] for country in COUNTRIES}
    keyboard = [s['id'] for s in actual if s['environment']['keyboard_only']]
    narrow = [s['id'] for s in actual if s['environment']['viewport_width'] == 390]
    later = [{'session': s['id'], 'date': s['campaign']['date'], 'save': s['campaign']['start_save']}
             for s in actual if s['phase'] == 'later']
    if len(first_time) < 5:
        gaps.append(f'Independent first-time humans: {len(first_time)}/5 minimum.')
    if len(actual) < 8:
        gaps.append(f'Observed independent human sessions: {len(actual)}/8 minimum (aborted/planned/AI excluded).')
    if len(covered) < 8:
        gaps.append('Missing country cases: ' + ', '.join(c for c in COUNTRIES if c not in covered) + '.')
    successes = sum(row['uncoached_opening_success'] for row in opening)
    if len(cohort) != 5 or successes < 4:
        gaps.append(f'First-attempt uncoached opening paths: {successes}/4 required in the declared five-person cohort.')
    missing_identification = [x['participant'] for x in identifications if not x['government'] or not x['actionable_blocker']]
    if not identifications or missing_identification:
        gaps.append('Government/actionable-blocker explanations missing or incorrect: ' + (', '.join(missing_identification) or 'no human observations') + '.')
    if not keyboard:
        gaps.append('No observed keyboard-only human session.')
    if not narrow:
        gaps.append('No observed 390px-layout human session.')
    if not later:
        gaps.append('No observed later-game session from a pinned save.')
    if root is None:
        gaps.append('Evidence files have not been verified against a local evidence root.')
    return {'format': 'spheres-s26-preparation-report/v1',
            's26_complete': False, 'qualification_awarded': False,
            'status': 'invalid_records' if errors else 'coverage_incomplete' if gaps else 'ready_for_human_review',
            'scope': 'Recorded coverage only. Human authenticity, S24 dependency, source/save suitability and final S26 acceptance require independent review.',
            'errors': errors, 'missing': gaps, 'planned_sessions': planned_count,
            'observed_human_sessions': len(actual), 'independent_humans': len(actual_people),
            'first_time_humans': len(first_time), 'excluded_sessions': excluded,
            'verified_evidence': verified, 'country_coverage': coverage,
            'opening_path_sample': opening, 'identifications': identifications,
            'keyboard_sessions': keyboard, 'narrow_390px_sessions': narrow,
            'later_save_sessions': later, 'failures': issues}


def markdown(report):
    lines = ['# S26 human playtest preparation report', '', f"Status: **{report['status']}**.",
             'S26 completion: **not awarded**. G5/CP1: **not awarded**.', '', report['scope'], '',
             f"Observed independent human sessions: {report['observed_human_sessions']}; first-time humans: {report['first_time_humans']}; planned slots: {report['planned_sessions']}.",
             '', '| Country case | Observed session IDs |', '| --- | --- |']
    lines += [f"| {country} | {', '.join(ids) or 'Missing'} |" for country, ids in report['country_coverage'].items()]
    for title, items in [('Record errors', report['errors']), ('Missing coverage', report['missing'])]:
        lines += ['', f'## {title}', ''] + ([f'- {x}' for x in items] or ['None recorded.'])
    lines += ['', '## First-attempt opening sample', '', '| Participant | First session | Uncoached path |', '| --- | --- | --- |']
    lines += [f"| {x['participant']} | {x['first_session'] or 'Missing'} | {'Yes' if x['uncoached_opening_success'] else 'No'} |" for x in report['opening_path_sample']]
    lines += ['', '## Retained failures', '']
    lines += [f"- {x.get('id', 'Missing ID')} ({x['session']}, severity {x.get('severity', '?')}): {x.get('notes', '')}" for x in report['failures']] or ['None recorded; an empty plan is not evidence of no defects.']
    return '\n'.join(lines) + '\n'


def main():
    # Windows redirected stdout may otherwise use a legacy code page for →.
    if hasattr(sys.stdout, 'reconfigure'):
        sys.stdout.reconfigure(encoding='utf-8')
    parser = argparse.ArgumentParser(description=__doc__)
    sub = parser.add_subparsers(dest='command', required=True)
    init = sub.add_parser('init', help='Create empty records and an unfilled session template in a NEW directory.')
    init.add_argument('directory', type=Path)
    check = sub.add_parser('report', help='Read records, verify local evidence and report missing coverage.')
    check.add_argument('record', type=Path)
    check.add_argument('--evidence-root', type=Path)
    check.add_argument('--output', type=Path, help='NEW directory for JSON and Markdown; otherwise print JSON.')
    pin = sub.add_parser('pin', help='Print a raw file identity for the evidence list; does not modify the file.')
    pin.add_argument('file', type=Path)
    pin.add_argument('--root', type=Path, required=True)
    pin.add_argument('--id', required=True)
    pin.add_argument('--kind', choices=('notes', 'image', 'video', 'save', 'build'), required=True)
    args = parser.parse_args()
    if args.command == 'init':
        if args.directory.exists():
            parser.error('Choose a new directory; existing records are never overwritten.')
        args.directory.mkdir(parents=True)
        for name, data in [('records.json', template()), ('session-template.json', session_template())]:
            (args.directory / name).write_text(json.dumps(data, indent=2, ensure_ascii=False) + '\n', encoding='utf-8', newline='\n')
        return 0
    if args.command == 'pin':
        source, root = args.file.resolve(), args.root.resolve()
        if not re.fullmatch(r'E[0-9]{3,6}', args.id) or not source.is_file() or not source.is_relative_to(root):
            parser.error('Use an E001-style ID and an existing file inside the declared evidence root.')
        print(json.dumps({'id': args.id, 'path': source.relative_to(root).as_posix(),
                          'kind': args.kind, 'bytes': source.stat().st_size, 'sha256': _sha(source)}, indent=2))
        return 0
    try:
        data = json.loads(args.record.read_text(encoding='utf-8-sig'), object_pairs_hook=_unique_object)
    except (OSError, ValueError) as error:
        parser.error(str(error))
    report = audit(data, args.evidence_root or args.record.resolve().parent)
    payload = json.dumps(report, indent=2, ensure_ascii=False) + '\n'
    if args.output:
        if args.output.exists():
            parser.error('Choose a new output directory; previous reports are never overwritten.')
        args.output.mkdir(parents=True)
        (args.output / 'report.json').write_text(payload, encoding='utf-8', newline='\n')
        (args.output / 'report.md').write_text(markdown(report), encoding='utf-8', newline='\n')
    else:
        print(payload, end='')
    # Missing real coverage is deliberately nonzero, even when template/schema is valid.
    return 2 if report['errors'] else 1 if report['missing'] else 0


if __name__ == '__main__':
    raise SystemExit(main())
