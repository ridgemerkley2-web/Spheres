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


def _build_submitted(raw):
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



# The publication page loads this official JSON through the pinned loader below.
# This is the reporting-obligation universe for ONE accounting exercise, not all
# political organizations, an active-party list, or a historical name register.
TABLE_SOURCE = 'fr_cnccfp_publication_2024'
PDF_SOURCE = 'fr_cnccfp_official_notice_2024'
SOURCES_DIR = RAW.parent
TABLE_RAW = SOURCES_DIR / 'cnccfp-publication-2024.json'
TABLE_SHA256 = '4c3596200e805d0a9335a57f7dc2bf5bf9513bc215208fc069511691c381c487'
SNAPSHOTS = (
    ('fr_cnccfp_publication_page_2024', 'cnccfp-publication-2024.html',
     '94373de5bb996f0c9b8afb58c22754423af2539be2891a91b032c572f7fab8b2', 3660,
     'https://liste.cnccfp.fr/publications/comptes_partis_2024.html',
     'CNCCFP official 2024 publication page'),
    ('fr_cnccfp_publication_loader', 'cnccfp-publication-loader.js',
     '6336c04660b08cd95276687c56a5b190fec8d6e3238a7dc62c9a3c291cf2d337', 8911,
     'https://liste.cnccfp.fr/publications/js/comptes_partis-fa.js',
     'CNCCFP publication table loader'),
    (TABLE_SOURCE, TABLE_RAW.name, TABLE_SHA256, 105818,
     'https://liste.cnccfp.fr/publications/comptes_partis_2024.txt',
     'CNCCFP full publication table for exercise 2024'),
    ('fr_cnccfp_publication_instructions_2024', 'cnccfp-publication-instructions-2024.pdf',
     'cb8bcaf879a446fed65898ef579fd60c7da4250180918c3bded63275f72d8331', 221210,
     'https://liste.cnccfp.fr/publications/cnccfp_notice_publication_comptes_des_partis_2024.pdf',
     'CNCCFP instructions for the 2024 publication table'),
    (PDF_SOURCE, 'cnccfp-obligations-2024.pdf',
     '127fc6028a49240e41a204db7bcb4a6258c23280a7316af93542b920a471899d', 695861,
     'https://cnccfp.fr/wp-content/uploads/2026/02/Avis-relatif-a-la-publication-generale-des-comptes-des-partis-et-groupements-politiques-au-titre-de-lexercice-2024.pdf',
     'CNCCFP official notice and annex, Journal officiel, 10 February 2026'),
)
# One-based PDF pages, independently checked against all 60 AD annex rows.
# The PDF supplies names/status; ONLY the official numbered JSON supplies codes.
# No code was inferred from a name. Formatting-normalized names were used solely
# to cross-check the already-numbered table against the independent PDF annex.
AD_PDF_PAGES = {
    '5': 34, '63': 39, '77': 34, '79': 40, '160': 29, '303': 41,
    '319': 34, '358': 41, '399': 32, '453': 37, '587': 27, '737': 44,
    '738': 41, '765': 37, '770': 30, '864': 28, '868': 29, '885': 29,
    '957': 33, '996': 27, '1016': 25, '1017': 36, '1024': 36, '1076': 40,
    '1179': 29, '1217': 25, '1264': 44, '1266': 30, '1292': 29, '1314': 34,
    '1315': 39, '1317': 44, '1325': 41, '1333': 34, '1334': 28, '1338': 41,
    '1339': 39, '1357': 28, '1362': 43, '1374': 35, '1381': 37, '1382': 39,
    '1396': 36, '1397': 30, '1411': 36, '1428': 36, '1434': 34, '1437': 36,
    '1444': 35, '1456': 35, '1461': 38, '1465': 32, '1474': 44, '1496': 28,
    '1500': 42, '1544': 36, '1548': 32, '1550': 37, '1556': 44, '1598': 40,
}
MOTIFS = {'AD', 'DC', 'HD', 'IC', 'HD+NC', 'RC', 'ANC'}


def parse_publication(raw):
    """Validate the observed table structure without ever executing its HTML."""
    data = json.loads(raw.decode('utf-8-sig'))
    if not isinstance(data, dict) or set(data) != {'headers', 'rows'}:
        raise ValueError('Unexpected official publication shape')
    headers = data['headers']
    if (not isinstance(headers, list) or len(headers) != 1
            or not isinstance(headers[0], list) or len(headers[0]) != 8
            or not isinstance(headers[0][0], dict) or headers[0][0].get('text') != 'N°'
            or not isinstance(headers[0][1], dict)
            or headers[0][1].get('text') != 'Nom du parti politique'
            or headers[0][6:] != ['Décision', 'Motif']):
        raise ValueError('Unexpected publication identity/status columns')
    if not isinstance(data['rows'], list):
        raise ValueError('Publication rows must be a list')
    rows = {}
    for index, row in enumerate(data['rows']):
        if not isinstance(row, list) or len(row) != 8:
            raise ValueError('Unexpected publication row width')
        if type(row[0]) is not int or row[0] <= 0:
            raise ValueError('Official publication identifier must be a positive integer')
        if not isinstance(row[1], str) or not row[1].strip():
            raise ValueError('Empty official publication name')
        code = str(row[0])
        if code in rows:
            raise ValueError('Duplicate official publication identifier')
        if row[6] not in ('Respect', 'Non-respect') or row[7] not in MOTIFS:
            raise ValueError('Unknown publication decision or motif')
        if (row[7] == 'DC') != (row[6] == 'Respect'):
            raise ValueError('Inconsistent publication decision and motif')
        if any(value is not None and not isinstance(value, str) for value in row[2:6]):
            raise ValueError('Unexpected publication document metadata')
        if row[7] == 'AD' and any(value is not None for value in row[2:6]):
            raise ValueError('Absent-filing row unexpectedly links filed material')
        rows[code] = {'index': index, 'row': row}
    if len(rows) != 635:
        raise ValueError('Expected 635 unique official filing-obligation identities')
    return rows


def reconcile_publication(submitted, rows):
    """Join exclusively by explicit CNCCFP IDs; names never create a binding."""
    csv_codes = set(submitted)
    publication_codes = set(rows)
    missing = publication_codes - csv_codes
    absent = {code for code, record in rows.items() if record['row'][7] == 'AD'}
    if len(csv_codes) != 575 or not csv_codes <= publication_codes:
        raise ValueError('Submitted CSV identities are not the expected publication subset')
    if len(missing) != 60 or missing != absent:
        raise ValueError('The 60 non-CSV identities must equal the official AD set')
    if missing != set(AD_PDF_PAGES):
        raise ValueError('Absent-filing identifiers differ from the reviewed PDF cross-check')
    return missing


def build(raw, publication=None, snapshot_bytes=None):
    packet = _build_submitted(raw)
    period = {'from': '2024-01-01', 'through': '2024-12-31'}
    observed_sources = {}
    for source_id, filename, digest, size, url, title in SNAPSHOTS:
        if source_id == TABLE_SOURCE and publication is not None:
            content = publication
        elif snapshot_bytes is not None:
            content = snapshot_bytes[filename]
        else:
            content = (ROOT / SOURCES_DIR / filename).read_bytes()
        if len(content) != size or hashlib.sha256(content).hexdigest() != digest:
            raise ValueError(f'Official snapshot checksum mismatch: {filename}')
        observed_sources[source_id] = {
            'id': source_id, 'title': title, 'url': url, 'publisher': 'CNCCFP',
            'accessed_date': '2026-09-13',
            'snapshot': {'path': (SOURCES_DIR / filename).as_posix(),
                         'sha256': digest, 'bytes': size}, 'claims': []}
        if source_id == TABLE_SOURCE:
            table = parse_publication(content)
    submitted = {o['external_ids']['CNCCFP']: o for o in packet['organizations']}
    missing = reconcile_publication(submitted, table)
    table_claims, absent_claims, organizations, differing_names = [], [], [], []
    for code, record in sorted(table.items(), key=lambda pair: int(pair[0])):
        row, index = record['row'], record['index']
        identity = f'fr_cnccfp_{code}'
        table_claim = f'{identity}_publication_2024'
        table_claims.append({
            'id': table_claim,
            'text': f'The official 2024 publication table lists CNCCFP number {code} as {row[1]}, with decision {row[6]} and motif {row[7]}.',
            'period': period,
            'locator': {'json_pointer': f'/rows/{index}', 'CNCCFP_number': code,
                        'Exercice': '2024', 'name_column': 1,
                        'decision_column': 6, 'motif_column': 7},
            'uncertainty': 'A pinned exercise-specific financial observation, not a lifespan, legal dissolution finding, current sanction or dated leadership role.'})
        if code in submitted:
            organization = submitted[code]
        else:
            organization = {
                'id': identity, 'name': row[1], 'kind': 'financial_reporting_identity',
                'represented_party_ids': [], 'representation_status': 'unreconciled',
                'external_ids': {'CNCCFP': code},
                'lifecycle': {'status': 'unknown', 'note': 'Non-filing does not establish dormancy, dissolution, formation or lack of political activity.'},
                'roles': [], 'sources': [], 'claim_ids': [],
                'coverage': {'status': 'reporting_identity_only', 'period': period,
                    'unresolved': ['Reconcile the official reporting identity with game rows, components and territorial jurisdiction.',
                                   'Research organization type, lifecycle and separately dated party/legislative/executive roles.']}}
        organization['sources'].append(TABLE_SOURCE)
        organization['claim_ids'].append(table_claim)
        organization['financial_reporting_observation'] = {
            'exercise': '2024', 'obligated': True, 'in_submitted_csv': code in submitted,
            'publication_name': row[1], 'decision_as_published': row[6],
            'motif_as_published': row[7], 'source': TABLE_SOURCE,
            'claim_id': table_claim, 'json_pointer': f'/rows/{index}'}
        if code in missing:
            pdf_claim = f'{identity}_annex_absent_2024'
            absent_claims.append({
                'id': pdf_claim,
                'text': f'The official notice annex lists {row[1]} with the non-filing motif AD.',
                'period': period,
                'locator': {'pdf_page_1_based': AD_PDF_PAGES[code],
                            'table': 'Annex: party decisions', 'row_name': row[1],
                            'motif': 'AD'},
                'uncertainty': 'The annex has no CNCCFP-number column. The identifier comes from the separately cited numbered publication table; apostrophe, spacing and line-break typography may differ.'})
            organization['sources'].append(PDF_SOURCE)
            organization['claim_ids'].append(pdf_claim)
            organization['coverage']['unresolved'].append(
                'AD is a non-filing observation for 2024; independently verify any subsequent filing, appeal or organizational status.')
        elif row[1] != organization['name']:
            differing_names.append(code)
            organization['name_observations'] = [
                {'name': organization['name'], 'source': SOURCE,
                 'claim_id': f'{identity}_reported_2024'},
                {'name': row[1], 'source': TABLE_SOURCE, 'claim_id': table_claim}]
            organization['coverage']['unresolved'].append(
                'The two official snapshots use different names or character encodings for this same CNCCFP code. Preserve both; investigate rename dates and spelling without creating a new identity or silently correcting either source.')
        organizations.append(organization)

    observed_sources[TABLE_SOURCE]['claims'] = table_claims
    observed_sources[TABLE_SOURCE]['scope_note'] = (
        'Retrieved after the historical cutoff, but explicitly an exercise-2024 publication. '
        'Later website corrections or name updates are possible; do not infer their effective dates.')
    observed_sources[PDF_SOURCE]['published_date'] = '2026-02-10'
    observed_sources[PDF_SOURCE]['claims'] = [
        {'id': 'fr_cnccfp_full_notice_scope',
         'text': 'The official notice reports 635 filing obligations, 575 accounts submitted and 60 non-filings for 2024.',
         'period': period, 'locator': {'pdf_page_1_based': 3, 'section': 'II.A'}},
        {'id': 'fr_cnccfp_absence_not_dissolution',
         'text': 'The notice distinguishes non-filing from formal dissolution or withdrawal from its reporting regime. Its annex notes that subsequent decisions on administrative appeals may appear on the website.',
         'locator': {'pdf_pages_1_based': [3, 25], 'sections': ['II.A', 'Annex introduction']}}
    ] + absent_claims
    observed_sources['fr_cnccfp_publication_instructions_2024']['claims'] = [{
        'id': 'fr_cnccfp_number_and_ad_definition',
        'text': 'The official table instructions define column A as the CNCCFP-assigned party identifier, column B as the name for the relevant exercise, and motif AD as accounts not filed.',
        'locator': {'pdf_page_1_based': 1, 'columns': ['A', 'B', 'H']}}]
    observed_sources['fr_cnccfp_publication_page_2024']['claims'] = [{
        'id': 'fr_cnccfp_table_page_binding',
        'text': 'The official 2024 publication page embeds the comptes_partis-fa.js loader and a table_json display container.',
        'locator': {'script_src': 'js/comptes_partis-fa.js', 'element_id': 'table_json'}}]
    observed_sources['fr_cnccfp_publication_loader']['claims'] = [{
        'id': 'fr_cnccfp_table_json_binding',
        'text': 'The official loader constructs the table from comptes_partis_<selected exercise>.txt as JSON.',
        'locator': {'javascript_property': 'widgetOptions.build_source'}}]
    packet['sources'].extend(observed_sources.values())
    packet['organizations'] = organizations
    packet['coverage']['scope'] = (
        '635 unique CNCCFP identifiers in the pinned exercise-2024 filing-obligation publication: '
        '575 in the submitted-accounts CSV and 60 additional AD entries. '
        'This financial universe is enumerated; the country political-organization and leadership census is not complete.')
    packet['coverage']['financial_reporting_universe'] = {
        'exercise': '2024', 'status': 'enumerated_for_pinned_publication',
        'obligated_identities': 635, 'submitted_csv_identities': 575,
        'additional_non_filing_identities': 60,
        'additional_cnccfp_codes': sorted(missing, key=int),
        'differing_name_observation_codes': differing_names,
        'country_census_complete': False}
    packet['coverage']['unresolved'][0] = (
        'The 2024 financial-obligation universe is now enumerated, but it is not a register of every French political organization in 1990–2026.')
    packet['coverage']['unresolved'].append(
        'Twelve shared CNCCFP codes have differing names or character encodings across the two pinned sources; exact rename dates and the causes of differences remain unverified.')
    packet['coverage']['unresolved'].append(
        'AD does not prove an organization dissolved, stopped political activity, or remained non-compliant after this publication; later appeals and filings require dated evidence.')
    return packet


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
    print(json.dumps({'check': args.check, 'reporting_identities': 635, 'submitted_identities': 575,
                      'additional_non_filing_identities': 60, 'country_census_complete': False}))


if __name__ == '__main__':
    main()
