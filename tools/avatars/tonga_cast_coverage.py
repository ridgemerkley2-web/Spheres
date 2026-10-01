#!/usr/bin/env python3
"""Audit the production Tonga cast without rewriting the immutable proposal.

This is a coverage gate, not an independent historical or visual review. Exact
source bindings, physical artwork and pinned review evidence are all required;
an empty 1990 party table never waives later organizations or institutions.
"""
from __future__ import annotations

import argparse
from datetime import date, timedelta
import hashlib
import json
from pathlib import Path
import sys

import person_art_pipeline as art
import fictional_art_pipeline as fiction

ROOT = Path(__file__).resolve().parents[2]
DEFAULT = 'docs/campaign-certification/C06/countries/tonga/manifest.json'
FIRST, CUTOFF, FUTURE, UNTIL = '1990-01-01', '2026-09-07', '2026-09-08', '2036-01-01'
INPUTS = {
    'research': 'docs/campaign-certification/C01/research/tonga.json',
    'registry': 'spheres-sim/data/party_leaders.json',
    'portraits': 'spheres-web/data/person_portraits.json',
    'fictional_portraits': 'spheres-web/data/fictional_portraits.json',
    'fictional_catalog': 'spheres-web/data/future_candidates_2035.json',
    'institutions': 'spheres-sim/data/tonga_institutional_leadership.json',
}
CHECKS = {'boundary_dates', 'save_compatibility', 'production_browser',
          'institutional_rules', 'likeness_visual_review', 'country_signoff'}
SIM_PINS = {'spheres-sim/src/institutional_leadership.rs', 'spheres-sim/src/world.rs',
            'spheres-sim/src/lib.rs', 'spheres-sim/src/government.rs',
            'spheres-sim/src/company_save.rs',
            'spheres-sim/src/party_leadership.rs', 'spheres-sim/src/data/mod.rs',
            INPUTS['institutions'], INPUTS['registry']}
ART_PINS = {INPUTS[k] for k in ('portraits', 'fictional_portraits', 'fictional_catalog')}
WEB_PINS = {'spheres-web/src/person_portraits.rs', 'spheres-web/src/main.rs',
            'spheres-web/src/government_view.rs',
            'spheres-web/src/person_avatar_assets.rs', 'spheres-web/ui/index.html',
            'spheres-web/ui/government-ui.js', 'spheres-web/ui/government-ui.css'}
CHECK_PINS = {
    'boundary_dates': SIM_PINS | {INPUTS['research']},
    'save_compatibility': SIM_PINS,
    'institutional_rules': SIM_PINS,
    'production_browser': SIM_PINS | ART_PINS | WEB_PINS,
    'likeness_visual_review': ART_PINS | {INPUTS['registry'], INPUTS['institutions']},
    'country_signoff': SIM_PINS | ART_PINS | WEB_PINS | {INPUTS['research']},
}
FICTION_IDS = {
    'fictional_to_lesieli_fotu': 'peoples_representative',
    'fictional_to_sitani_lolohea': 'prime_minister',
    'fictional_to_pisila_tukuafu': 'nonelected_minister',
    'fictional_to_kalolo_matalehu': 'party_organizer',
}


def read(path):
    return json.loads(Path(path).read_text(encoding='utf-8-sig'))


def digest(value):
    return hashlib.sha256(json.dumps(value, ensure_ascii=False, sort_keys=True,
                                     separators=(',', ':')).encode()).hexdigest()


def inventory(research):
    """Immutable observation hashes preserve null bounds and original wording."""
    roles, holders = {}, {}
    for section in ('organizations', 'institutions'):
        for institution in research.get(section, []):
            listed = institution.get('roles', [])
            # An organization with no researched role still needs a disposition.
            for role in listed or [{'id': '__unresearched_role__', 'holder_claims': []}]:
                key = institution['id'] + '/' + role['id']
                if key in roles:
                    raise ValueError('Duplicate research role: ' + key)
                roles[key] = {'institution_id': institution['id'], 'role_id': role['id'],
                              'section': section, 'holder_keys': []}
                for holder in role.get('holder_claims', []):
                    hkey = key + '/' + digest(holder)
                    if hkey in holders:
                        raise ValueError('Duplicate source holder observation: ' + hkey)
                    holders[hkey] = {'role_key': key, 'holder': holder}
                    roles[key]['holder_keys'].append(hkey)
    return roles, holders


def _file_pin(root, pin, errors, context):
    if not isinstance(pin, dict):
        errors.append(context + ': receipt path/hash object required')
        return False
    try:
        path = art.safe_path(root, pin.get('path'))
        if not path.is_file() or hashlib.sha256(path.read_bytes()).hexdigest() != pin.get('sha256'):
            raise ValueError('missing file or SHA-256 mismatch')
    except (OSError, ValueError) as exc:
        errors.append(context + ': ' + str(exc))
        return False
    return True


def validate(manifest, *, research, registry, portraits, fictional_portraits,
             fictional_catalog, institutions, root=ROOT):
    errors, blockers = [], []
    if not isinstance(manifest, dict):
        return {'valid': False, 'ready_for_country_signoff': False,
                'errors': ['Production manifest must be an object'], 'blockers': []}
    required_header = {'version': 1, 'nation': 'Tonga', 'historical_cutoff': CUTOFF,
                       'from': FIRST, 'until_exclusive': UNTIL, 'kind': 'production_country_cast'}
    for key, value in required_header.items():
        if type(manifest.get(key)) is not type(value) or manifest.get(key) != value:
            errors.append(key + ': production scope or historical cutoff changed')
    if research.get('nation') != 'Tonga' or research.get('research_cutoff') != CUTOFF:
        errors.append('Research nation/cutoff differs')
    expected_roles, observations = inventory(research)
    people = art.people_from_registry(registry)
    roles = manifest.get('roles', [])
    if not isinstance(roles, list):
        errors.append('roles must be an array'); roles = []
    role_map = {}
    for row in roles:
        if not isinstance(row, dict):
            errors.append('role must be an object'); continue
        key = row.get('institution_id', '') + '/' + row.get('role_id', '')
        if key not in expected_roles or key in role_map:
            errors.append('Unknown or duplicate role: ' + key)
        role_map[key] = row
        status = row.get('status')
        if status == 'unresolved':
            blockers.append(key + ': ' + str(row.get('reason') or 'research/integration unresolved'))
        elif status == 'institutional_exception':
            # An exception must identify its real institutional basis; it can
            # never erase a named observed holder from the required coverage.
            if not row.get('reason') or not row.get('claim_ids'):
                errors.append(key + ': sourced institutional exception required')
            known_claims = {c['id'] for s in research.get('sources', []) for c in s.get('claims', [])}
            if not set(row.get('claim_ids', [])) <= known_claims:
                errors.append(key + ': exception cites unknown research claims')
            _file_pin(root, row.get('review'), errors, key + ' exception review')
            if expected_roles.get(key, {}).get('holder_keys'):
                blockers.append(key + ': named holders cannot be waived by an institutional exception')
        elif status == 'covered':
            _file_pin(root, row.get('chain_review'), errors, key + ' dated-chain review')
        else:
            errors.append(key + ': unknown role disposition')
    for key in sorted(set(expected_roles) - set(role_map)):
        blockers.append('Missing researched role disposition: ' + key)

    bindings = manifest.get('historical_bindings', [])
    if not isinstance(bindings, list):
        errors.append('historical_bindings must be an array'); bindings = []
    seen, required_people = set(), set()
    runtime_bindings = institutions.get('historical_bindings', [])
    for binding in bindings:
        if not isinstance(binding, dict):
            errors.append('historical binding must be an object'); continue
        key = binding.get('observation_key')
        observation = observations.get(key)
        if not observation or key in seen:
            errors.append('Unknown or duplicate holder observation: ' + str(key)); continue
        seen.add(key)
        if binding.get('holder') != observation['holder']:
            errors.append(str(key) + ': accepted holder evidence/boundaries changed')
        pid = binding.get('person_id')
        if binding.get('status') == 'unresolved':
            blockers.append(str(key) + ': ' + str(binding.get('reason') or 'identity unresolved'))
            if pid is not None:
                errors.append(str(key) + ': unresolved identity must not claim a person ID')
            continue
        if binding.get('status') != 'installed_reference':
            errors.append(str(key) + ': only installed exact references or explicit unresolved identities are supported')
            continue
        if pid not in people or pid.startswith('fictional_'):
            errors.append(str(key) + ': exact historical registry person required'); continue
        required_people.add(pid)
        _file_pin(root, binding.get('identity_review'), errors, str(key) + ' identity review')
        matching = [row for row in runtime_bindings if row.get('id') == binding.get('binding_id')]
        role = expected_roles[observation['role_key']]
        if len(matching) != 1 or any(matching[0].get(field) != expected for field, expected in (
                ('person_id', pid), ('holder', observation['holder']),
                ('institution_id', role['institution_id']), ('role_id', role['role_id']))):
            errors.append(str(key) + ': matching installed institutional reference is missing')
    for key in sorted(set(observations) - seen):
        blockers.append('Missing holder identity disposition: ' + key)

    historical = art.validate_manifest(portraits, root, people)
    future = fiction.validate(fictional_portraits, fictional_catalog, root, portraits)
    errors.extend('historical art: ' + e for e in historical['errors'])
    errors.extend('fictional art: ' + e for e in future['errors'])
    appearance = manifest.get('historical_people', [])
    if not isinstance(appearance, list):
        errors.append('historical_people must be an array'); appearance = []
    era_map = {}
    # Opening king, named premier, and exact inherited heir are actual game uses.
    required_people.update(institutions.get('opening', {}).get(field) for field in
                           ('king_person_id', 'prime_minister_person_id', 'heir_person_id'))
    required_people.discard(None)
    for entry in appearance:
        pid = entry.get('person_id') if isinstance(entry, dict) else None
        if pid not in people or pid in era_map:
            errors.append('Unknown or duplicate historical appearance person: ' + str(pid)); continue
        era_map[pid] = entry
        windows = entry.get('required_appearance_intervals', [])
        if not windows:
            blockers.append(pid + ': no reviewed appearance windows')
        _file_pin(root, entry.get('appearance_review'), errors, pid + ' appearance review')
        for window in windows:
            try:
                start, end = art.iso_day(window['from']), art.iso_day(window['to'])
                if not date.fromisoformat(FIRST) <= start < end <= date.fromisoformat(UNTIL):
                    raise ValueError('appearance interval falls outside the campaign')
                if window.get('meaning') != 'appearance_not_office_tenure':
                    raise ValueError('appearance eras must not claim office tenure')
                served = [p for p in historical['ready'].get(pid, []) if
                          (p.get('style'), p.get('method'), p.get('status')) ==
                          ('cartoon', 'generated', 'illustrated-likeness')]
                for first, until in art.missing_intervals(served, start, end):
                    blockers.append(f'{pid}: no reviewed cartoon for {first} through {until} exclusive')
            except (KeyError, ValueError, TypeError) as exc:
                errors.append(pid + ': ' + str(exc))
    for pid in sorted(required_people - set(era_map)):
        blockers.append('Missing historical artwork requirement: ' + pid)
    # Every dated attestation must be visible in a justified appearance window;
    # unknown boundaries remain unknown and are not stretched into terms.
    for binding in bindings:
        if not isinstance(binding, dict) or binding.get('status') != 'installed_reference': continue
        holder, pid = binding.get('holder'), binding.get('person_id')
        if not isinstance(holder, dict) or not holder.get('attested_on'): continue
        at = holder['attested_on']
        if not any(p.get('from', '') <= at < p.get('to', '') for p in era_map.get(pid, {}).get('required_appearance_intervals', [])):
            blockers.append(str(pid) + ': dated holder observation has no required appearance window: ' + at)

    catalog_rows = institutions.get('future_candidates', [])
    actual = {c.get('person', {}).get('id'): c for c in catalog_rows}
    if len(actual) != len(catalog_rows) or set(actual) != set(FICTION_IDS):
        errors.append('Institutional catalogue must preserve the four exact authored civilian identities')
    exported = {c.get('person_id'): c for c in fictional_catalog.get('candidates', [])}
    for pid, role in FICTION_IDS.items():
        c = actual.get(pid, {})
        e = exported.get(pid, {})
        if c.get('role') != role or c.get('origin') != 'fictional_successor' or not c.get('fictional_biography'):
            errors.append(pid + ': distinct civilian role, fiction label and biography required')
        if (e.get('name'), e.get('appearance_seed'), e.get('institution'), e.get('party')) != (
                c.get('person', {}).get('name'), c.get('appearance_seed'), 'tonga_civilian_institutions', None):
            errors.append(pid + ': exact institutional portrait catalogue export required')
        for start, end in art.missing_intervals(future['ready'].get(pid, []), date.fromisoformat(FUTURE), date.fromisoformat(UNTIL)):
            blockers.append(f'{pid}: fictional cartoon missing {start} through {end} exclusive')
        if role == 'party_organizer' and manifest.get('fictional_party_office_grants', []):
            errors.append('Unreviewed PTOA rules cannot grant Kalolo a real-party office, people seat or premiership')
    checks = manifest.get('checks', {})
    if not isinstance(checks, dict):
        errors.append('checks must be an object'); checks = {}
    for name in sorted(CHECKS):
        check = checks.get(name)
        if not isinstance(check, dict) or check.get('status') != 'passed':
            blockers.append('Unfinished country acceptance check: ' + name); continue
        _file_pin(root, check, errors, name)
        pins = check.get('source_pins')
        if not isinstance(pins, dict) or not pins:
            errors.append(name + ': tested-source pins required'); continue
        if not CHECK_PINS[name] <= set(pins):
            errors.append(name + ': required production source pins missing: ' +
                          ', '.join(sorted(CHECK_PINS[name] - set(pins))))
        for path, sha in pins.items():
            _file_pin(root, {'path': path, 'sha256': sha}, errors, name + ' source pin')
    if manifest.get('country_complete') is True and (errors or blockers):
        errors.append('Country completion claimed while requirements remain open')
    return {'valid': not errors, 'ready_for_country_signoff': not errors and not blockers,
            'country_completed': manifest.get('country_complete') is True and not errors and not blockers,
            'nation': 'Tonga', 'role_count': len(expected_roles), 'holder_observation_count': len(observations),
            'installed_reference_count': sum(b.get('status') == 'installed_reference' for b in bindings if isinstance(b, dict)),
            'required_historical_people': sorted(required_people), 'fictional_candidate_count': len(actual),
            'errors': errors, 'blockers': blockers,
            'limit': 'Coverage and receipt-integrity check only; no independent historical/visual review or parent milestone qualification.'}


def main(argv=None):
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('--root', type=Path, default=ROOT)
    parser.add_argument('--manifest', type=Path, default=Path(DEFAULT))
    parser.add_argument('--inventory', action='store_true')
    parser.add_argument('--require-complete', action='store_true')
    args = parser.parse_args(argv)
    data = {k: read(args.root / p) for k, p in INPUTS.items()}
    if args.inventory:
        roles, holders = inventory(data['research'])
        print(json.dumps({'roles': roles, 'observations': holders}, ensure_ascii=False, indent=2))
        return 0
    manifest = args.manifest if args.manifest.is_absolute() else args.root / args.manifest
    report = validate(read(manifest), root=args.root, **data)
    print(json.dumps(report, ensure_ascii=False, indent=2))
    return 0 if report['valid'] and (not args.require_complete or report['ready_for_country_signoff']) else 1


if __name__ == '__main__':
    try:
        raise SystemExit(main())
    except (OSError, ValueError, TypeError, KeyError) as error:
        print('Tonga production cast audit failed: ' + str(error), file=sys.stderr)
        raise SystemExit(1)
