#!/usr/bin/env python3
"""Validate C01 primary-source intake and emit bounded discovery work orders.

This register cannot grant offices, create characters or certify a country. It
preserves source claims separately from the existing playable party catalogue.
No network access; source snapshots and input bytes are verified offline.
"""
from __future__ import annotations

import argparse
from datetime import date
import hashlib
import json
from pathlib import Path
from urllib.parse import urlsplit

import campaign_census as census

ROOT = Path(__file__).resolve().parents[2]
RESEARCH = Path('docs/campaign-certification/C01/research')
OUTPUT = Path('docs/campaign-certification/C01/research-index.json')
CUTOFF = '2026-09-07'
ROLE_KINDS = {'party_leader', 'party_chair', 'parliamentary_leader', 'head_of_state',
              'head_of_government', 'collective_seat', 'institutional_office', 'other'}


def require(condition, message):
    if not condition:
        raise ValueError(message)


def sha(path):
    return hashlib.sha256(path.read_bytes()).hexdigest()


def dates(value):
    """Validate structured historical dates; access/publication dates are metadata."""
    if isinstance(value, dict):
        for key, child in value.items():
            if key in {'from', 'until', 'through', 'attested_on'} and child is not None:
                require(isinstance(child, str), f'Historical date {key} must be ISO day or null')
                require(date.fromisoformat(child).isoformat() == child and child <= CUTOFF,
                        f'Historical date exceeds cutoff or is invalid: {key}={child}')
            else:
                dates(child)
        for end in ('until', 'through'):
            if value.get('from') and value.get(end):
                require(value['from'] <= value[end], 'Reversed historical interval')
    elif isinstance(value, list):
        for item in value:
            dates(item)


def validate(packet, root, nations, party_ids):
    nation = packet.get('nation')
    require(packet.get('version') == 1 and nation in nations, 'Unknown packet version or nation')
    require(packet.get('research_cutoff') == CUTOFF, 'Research cutoff changed without review')
    coverage = packet.get('coverage', {})
    require(coverage.get('status') == 'partial_primary_source_inventory' and coverage.get('unresolved'),
            'Partial intake cannot close a census or omit unresolved work')
    require(coverage.get('period') == {'from': '1990-01-01', 'through': CUTOFF}, 'Packet census interval must be explicit')
    dates(packet)
    sources, claims = {}, {}
    for source in packet['sources']:
        sid = source['id']
        require(sid not in sources and sid, f'Duplicate source: {sid}')
        url = urlsplit(source['url'])
        require(url.scheme == 'https' and url.hostname and not url.username and not url.password,
                f'Invalid public source URL: {sid}')
        require(source.get('title') and source.get('publisher') and source.get('claims'), f'Incomplete source: {sid}')
        date.fromisoformat(source['accessed_date'])
        if source.get('published_date'):
            require(date.fromisoformat(source['published_date']) <= date.fromisoformat(source['accessed_date']), 'Source published after access')
        sources[sid] = source
        for claim in source['claims']:
            cid = claim['id']
            require(cid not in claims and cid and claim.get('text'), f'Duplicate/empty claim: {cid}')
            claims[cid] = sid
        if snapshot := source.get('snapshot'):
            # Only checked-in source evidence, never an arbitrary filesystem read.
            path = (root / snapshot['path']).resolve()
            allowed = (root / RESEARCH / 'sources').resolve()
            require(path.is_relative_to(allowed) and path.is_file(), 'Snapshot escapes the research sources directory')
            require(path.stat().st_size == snapshot['bytes'] and sha(path) == snapshot['sha256'], 'Source snapshot checksum mismatch')

    def references(row):
        refs = row.get('sources', [])
        require(refs and len(refs) == len(set(refs)) and set(refs) <= sources.keys(), 'Unknown or duplicate source reference')
        ids = row.get('claim_ids', [])
        require(ids and len(ids) == len(set(ids)) and set(ids) <= claims.keys(), 'Unknown, duplicate or missing claim reference')
        require(all(claims[cid] in refs for cid in ids), 'Claim does not belong to a cited source')

    entries, roles = {}, set()
    for category in ('organizations', 'institutions'):
        for entry in packet[category]:
            identity = entry['id']
            require(identity and identity not in entries, f'Duplicate research identity: {identity}')
            require(entry.get('name') and entry.get('kind') and entry.get('lifecycle', {}).get('status'), 'Missing identity or lifecycle uncertainty')
            require(entry.get('coverage', {}).get('status') in {'partial', 'reporting_identity_only'}, 'Entry cannot imply complete history')
            require(entry['coverage'].get('unresolved'), 'Entry must record unresolved historical work')
            references(entry)
            mapped = entry['represented_party_ids']
            require(len(mapped) == len(set(mapped)) and set(mapped) <= party_ids[nation], 'Unknown or foreign represented party mapping')
            for role in entry['roles']:
                require(role['id'] not in roles and role['kind'] in ROLE_KINDS and role.get('title'), 'Duplicate role or unsupported role kind')
                roles.add(role['id'])
                references(role)
                for holder in role.get('holder_claims', []):
                    if isinstance(holder, str):
                        require(holder in claims and claims[holder] in role['sources'],
                                'Holder observation must belong to a cited role source')
                    else:
                        require(isinstance(holder, dict) and holder.get('name'), 'Holder claim needs a sourced identity')
                        references(holder)
            entries[identity] = category
    require(entries, 'Empty discovery packet cannot establish research progress')
    return {'sources': sources, 'claims': claims, 'entries': entries, 'roles': roles}


def build(root=ROOT):
    countries = census.load(root, 'docs/campaign-certification/C01/countries.json')
    represented = census.load(root, 'docs/campaign-certification/C01/represented-organizations.json')
    nations = {c['id'] for c in countries}
    party_ids = {n: {r['id'] for r in represented if r['nation'] == n} for n in nations}
    packets, inputs, work = [], [], []
    unique = {k: set() for k in ('sources', 'claims', 'entries', 'roles')}
    discovered = set()
    for path in sorted((root / RESEARCH).glob('*.json')):
        packet = json.loads(path.read_text(encoding='utf-8'))
        ids = validate(packet, root, nations, party_ids)
        nation = packet['nation']
        require(nation not in discovered, f'Duplicate country packet: {nation}')
        discovered.add(nation)
        for key in unique:
            require(not unique[key].intersection(ids[key]), f'Cross-packet duplicate {key}')
            unique[key].update(ids[key])
        packets.append({'nation': nation, 'packet': path.relative_to(root).as_posix(),
            'organization_observations': len(packet['organizations']), 'institution_observations': len(packet['institutions']),
            'role_observations': len(ids['roles']), 'source_claims': len(ids['claims']),
            'mapping_pending': sum(not e['represented_party_ids'] for group in ('organizations', 'institutions') for e in packet[group]),
            'country_census_complete': False, 'unrepresented_organization_count': None,
            'coverage': packet['coverage']})
        members = list(ids['entries'])
        for offset in range(0, len(members), 10):
            work.append({'id': f'C01-{nation}-DISC-B{offset // 10 + 1:03d}', 'nation': nation,
                'status': 'open', 'phase': 'discovery_reconciliation',
                'depends_on': [f'C01-{nation}-ORG-001', f'C01-{nation}-ROLE-001'],
                'members': members[offset:offset + 10], 'maximum_reviewed_records': 10,
                'deliverable': 'Reconcile exact source identities and jurisdiction; research separately dated party, parliamentary and executive roles. Add sourced history before character or succession eligibility. Empty game mappings remain unknown.'})
        inputs.append({'path': path.relative_to(root).as_posix(), 'bytes': path.stat().st_size, 'sha256': sha(path)})
    for name in ('tools/avatars/campaign_research.py', 'tools/avatars/import_cnccfp_census.py',
                 'docs/campaign-certification/C01/countries.json', 'docs/campaign-certification/C01/represented-organizations.json'):
        path = root / name
        inputs.append({'path': name, 'bytes': path.stat().st_size, 'sha256': sha(path)})
    return {'format': 'spheres-c01-research-intake/v1', 'research_cutoff': CUTOFF,
        'c01_complete': False, 'g2_prerequisite_satisfied': False, 'runtime_roster_modified': False,
        'counts': {'country_packets': len(packets), 'countries_without_new_discovery_packet': len(nations - discovered),
            'organization_observations': sum(p['organization_observations'] for p in packets),
            'institution_observations': sum(p['institution_observations'] for p in packets),
            'sources': len(unique['sources']), 'source_claims': len(unique['claims']),
            'discovery_batches': len(work), 'exhaustive_country_censuses': 0},
        'interpretation': 'Source-backed discovery observations are not verified game parties, complete office histories or permission to create portraits. Reporting years do not establish organization lifespans. Unknown representation is not absence.',
        'countries': packets, 'work_orders': work, 'source_files': inputs}


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('--check', action='store_true')
    args = parser.parse_args()
    result = build()
    text = census.canonical(result)
    target = ROOT / OUTPUT
    if args.check:
        require(target.exists() and target.read_text(encoding='utf-8') == text, 'Research index differs from inputs')
    else:
        target.write_text(text, encoding='utf-8', newline='\n')
    print(json.dumps({'check': args.check, **result['counts']}))


if __name__ == '__main__':
    main()
