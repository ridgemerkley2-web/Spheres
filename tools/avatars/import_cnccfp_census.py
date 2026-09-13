#!/usr/bin/env python3
"""Reproduce France's partial discovery packet from a pinned official release.

No network or game writes. Financial reporting is an observation, not an active
party interval, a chair appointment, or a census of every French organization.
"""
import argparse
import csv
import hashlib
import io
import json
from pathlib import Path

ROOT = Path(__file__).resolve().parents[2]
RAW = Path('docs/campaign-certification/C01/research/sources/cnccfp-2024.csv')
DEST = Path('docs/campaign-certification/C01/research/france.json')
SHA256 = '6ce50ac58fe95308b21995463a38a2cc83852980fde19cdd110002e8582c49f2'
SOURCE = 'fr_cnccfp_2024'
URL = 'https://static.data.gouv.fr/resources/comptes-des-partis-et-groupements-politiques/20260210-110641/comptes-partis-exercice-2024.csv'
DATASET = 'https://www.data.gouv.fr/datasets/comptes-des-partis-et-groupements-politiques'
NOTICE = 'https://cnccfp.fr/publication-de-lavis-de-la-cnccfp-sur-les-comptes-des-partis-politiques-exercice-2024/'


def build(raw):
    if hashlib.sha256(raw).hexdigest() != SHA256:
        raise ValueError('Official snapshot checksum mismatch; review a new release explicitly')
    rows = list(csv.DictReader(io.StringIO(raw.decode('utf-8-sig')), delimiter=';'))
    codes = [r['Code_CNCCFP'] for r in rows]
    if len(rows) != 575 or len(set(codes)) != len(codes):
        raise ValueError('Expected 575 unique official reporting identities')
    if any(not c.isdecimal() for c in codes) or any(r['Exercice'] != '2024' or not r['Nom_du_parti'].strip() for r in rows):
        raise ValueError('Invalid official identity, name or accounting year')
    period = {'from': '2024-01-01', 'through': '2024-12-31'}
    organizations, claims = [], []
    for row in sorted(rows, key=lambda r: int(r['Code_CNCCFP'])):
        code = row['Code_CNCCFP']
        identity = f'fr_cnccfp_{code}'
        claim = f'{identity}_reported_2024'
        claims.append({'id': claim, 'text': f"CNCCFP code {code} is listed under the name {row['Nom_du_parti']} in its 2024 accounts dataset.",
                       'period': period, 'locator': {'Code_CNCCFP': code, 'Exercice': '2024'},
                       'uncertainty': 'Reporting identity and exercise name only; active interval, leader, legal type and game-row correspondence require research.'})
        organizations.append({'id': identity, 'name': row['Nom_du_parti'],
            'kind': 'financial_reporting_identity', 'represented_party_ids': [],
            'representation_status': 'unreconciled',
            'external_ids': {'CNCCFP': code},
            'lifecycle': {'status': 'unknown', 'note': 'Accounting year is not formation, dissolution, or proof of continuous political activity.'},
            'roles': [], 'sources': [SOURCE], 'claim_ids': [claim],
            'coverage': {'status': 'reporting_identity_only', 'period': period,
                'unresolved': ['Reconcile this legal reporting identity with game rows, components, historical names and territorial jurisdiction.',
                               'Research organization type, lifecycle and separately dated party/legislative/executive roles.']}})
    return {'version': 1, 'nation': 'France', 'research_cutoff': '2026-09-07',
        'sources': [
            {'id': SOURCE, 'title': 'CNCCFP: comptes des partis, exercice 2024', 'url': URL,
             'publisher': 'CNCCFP', 'accessed_date': '2026-09-13', 'published_date': '2026-02-10',
             'license': {'name': 'Licence Ouverte / Open Licence', 'url': 'https://www.etalab.gouv.fr/licence-ouverte-open-licence/'},
             'snapshot': {'path': RAW.as_posix(), 'sha256': SHA256, 'bytes': len(raw)}, 'claims': claims},
            {'id': 'fr_cnccfp_dataset_scope', 'title': 'CNCCFP dataset description and stable identifiers',
             'url': DATASET, 'publisher': 'CNCCFP / data.gouv.fr', 'accessed_date': '2026-09-13',
             'claims': [{'id': 'fr_cnccfp_identity_scope', 'text': 'The dataset concerns submitted party and grouping accounts. A CNCCFP identifier is retained across a name change when the legal personality is unchanged.'}]},
            {'id': 'fr_cnccfp_notice_2024', 'title': 'CNCCFP publication notice for 2024 accounts',
             'url': NOTICE, 'publisher': 'CNCCFP', 'accessed_date': '2026-09-13', 'published_date': '2026-02-10',
             'claims': [{'id': 'fr_cnccfp_required_2024', 'text': 'The notice identifies 635 parties and groupings subject to the 2024 reporting obligation.', 'period': period}]}],
        'organizations': organizations, 'institutions': [],
        'coverage': {'status': 'partial_primary_source_inventory',
            'period': {'from': '1990-01-01', 'through': '2026-09-07'},
            'scope': '575 unique reporting identities in the frozen 2024 submitted-accounts CSV; a discovery register, not a complete political or leadership roster.',
            'unresolved': ['60 of the 635 identities in the official reporting-obligation universe are absent from this submitted-accounts file; enumerate the full notice separately.',
                'Earlier and later organization histories, non-reporting organizations, independent factions and coalition membership remain unenumerated.',
                'No automatic match to the five existing simulation party rows or their historical components; an empty mapping means unreconciled, not unrepresented.',
                'Overseas/local reporting identities need explicit territorial and campaign-jurisdiction review.',
                'No chair, executive, lifespan, succession, cartoon likeness or 2035 continuation has been inferred from accounts.']}}


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('--check', action='store_true')
    args = parser.parse_args()
    content = json.dumps(build((ROOT / RAW).read_bytes()), ensure_ascii=False, indent=2) + '\n'
    destination = ROOT / DEST
    if args.check:
        if not destination.exists() or destination.read_text(encoding='utf-8') != content:
            raise SystemExit('France discovery packet differs from the pinned import')
    else:
        destination.write_text(content, encoding='utf-8', newline='\n')
    print(json.dumps({'check': args.check, 'reporting_identities': 575, 'country_census_complete': False}))


if __name__ == '__main__':
    main()
