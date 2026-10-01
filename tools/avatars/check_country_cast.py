#!/usr/bin/env python3
"""Check a bounded country-cast proposal against existing records; never install it.

This is a review aid, not an identity resolver or an office/portrait importer.
Exact observation copies retain uncertainty and null boundaries. A passing check
does not independently verify sources or authorize a runtime mapping.
"""
from __future__ import annotations

import argparse
from datetime import date, timedelta
import hashlib
import json
from pathlib import Path
import sys

ROOT = Path(__file__).resolve().parents[2]
DEFAULT = Path('docs/campaign-certification/C06/preparation/tonga-cast-01/proposal.json')
INPUTS = {
    'research': 'docs/campaign-certification/C01/research/tonga.json',
    'registry': 'spheres-sim/data/party_leaders.json',
    'opening': 'spheres-sim/data/leaders_1990.json',
    'portraits': 'spheres-web/data/person_portraits.json',
}


def read(path):
    return json.loads(Path(path).read_text(encoding='utf-8'))


def digest(value):
    return hashlib.sha256(json.dumps(value, ensure_ascii=False, sort_keys=True,
                                     separators=(',', ':')).encode('utf-8')).hexdigest()


def text_digest(path):
    """Pin receipt text independently of Git's platform newline conversion."""
    return hashlib.sha256(Path(path).read_text(encoding='utf-8').encode('utf-8')).hexdigest()


def require(condition, message):
    if not condition:
        raise ValueError(message)


def opening_for(opening, nation):
    rows = opening if isinstance(opening, list) else opening['rows']
    matches = [row for row in rows if row['nation'] == nation]
    require(len(matches) == 1, 'Opening country must resolve exactly once')
    return matches[0]


def check(proposal, research, registry, opening, portraits, root=ROOT):
    require(proposal['version'] == 1, 'Unsupported proposal version')
    require(proposal['status'] == 'proposal_only', 'A cast check cannot install or accept a country')
    require(proposal['nation'] == research['nation'] == 'Tonga', 'This first slice is Tonga only')
    require(proposal['historical_cutoff'] == research['research_cutoff'] == '2026-09-07',
            'Historical cutoff changed')
    require(proposal['authority'] == {
        'runtime_installation': False, 'office_eligibility': False,
        'scheduled_succession': False, 'art_approval': False,
        'country_completion': False, 'party_mapping': False,
    }, 'Proposal must grant no runtime, art, party or completion authority')
    cutoff = date.fromisoformat(proposal['historical_cutoff'])
    people = {p['id']: p for p in registry['people']}
    claims = {c['id']: (s, c) for s in research['sources'] for c in s['claims']}
    roles = {(i['id'], r['id']): r for i in research['institutions'] for r in i['roles']}
    original = opening_for(opening, proposal['nation'])
    require(digest(original) == proposal['opening_record_sha256'], 'Opening record changed; review mappings')
    require([p for p in registry['parties'] if p.get('nation') == 'Tonga'] == [],
            'Tonga party rows changed; review scope before mapping parties')
    reviews = {r['id']: r for r in proposal['reviews']}
    require(len(reviews) == len(proposal['reviews']), 'Duplicate review IDs')
    for review in reviews.values():
        path = (root / review['path']).resolve()
        require(path.is_relative_to(root.resolve()), 'Review path escapes repository')
        require(path.is_file(), 'Missing review: ' + review['path'])
        require(review['hash_encoding'] == 'utf8_lf', 'Unknown receipt text encoding')
        require(text_digest(path) == review['sha256'],
                'Review receipt changed: ' + review['path'])

    entries = proposal['people']
    require(len(entries) == 8, 'This bounded slice must retain its eight portrait priorities')
    require(len({e['person_id'] for e in entries}) == len(entries), 'Duplicate person IDs')
    require([e['priority'] for e in entries] == list(range(1, 9)), 'Priorities must be unique and ordered')
    observation_count = held_aliases = existing = reserved = reused = 0
    source_enrichment = []
    seen_observations = set()
    for entry in entries:
        pid = entry['person_id']
        require(entry['party_ids'] == [], pid + ': party mapping is outside this slice')
        identity = entry['identity']
        slot = entry['opening_slot']
        if identity['record'] is not None:
            require(entry['name'] == identity['record']['name'], pid + ': existing display name differs')
        if identity['status'] == 'existing':
            existing += 1
            require(pid in people and identity['record'] == people[pid], pid + ': existing facts differ')
            if slot == 'primary':
                anchor = {k: original.get(k) for k in ('name', 'office', 'since')}
                links = [x for x in registry['office_links'] if x['nation'] == 'Tonga']
                require(len(links) == 1 and links[0]['person'] == pid
                        and links[0]['since'] == original['since'], pid + ': primary office link differs')
            elif slot == 'heir':
                anchor = original['heir']
            elif slot == 'secondary':
                matches = [a for a in original.get('also', []) if a['name'] == people[pid]['name']]
                require(len(matches) == 1, pid + ': secondary opening holder differs')
                anchor = matches[0]
            else:
                raise ValueError(pid + ': existing identity must have an exact opening reference')
            require(anchor == entry['opening_record'] and anchor['name'] == people[pid]['name'],
                    pid + ': opening identity differs')
        else:
            reserved += 1
            require(identity['status'] == 'reserved_not_imported' and identity['record'] is None,
                    pid + ': unknown identity status')
            require(pid not in people and slot is None and entry['opening_record'] is None,
                    pid + ': reserved identity is already installed or has invented opening use')
            require(not any(p['name'] == entry['name'] for p in people.values()),
                    pid + ': named identity already exists; review instead of duplicating it')

        reviewed_urls = []
        direct_observations = 0
        for observation in entry['observations']:
            observation_count += 1
            role = roles.get((observation['institution_id'], observation['role_id']))
            require(role is not None, pid + ': unknown institutional role')
            holder = observation['holder']
            matches = [h for h in role.get('holder_claims', []) if isinstance(h, dict) and h == holder]
            require(len(matches) == 1, pid + ': observation differs from accepted packet (including uncertainty)')
            fingerprint = digest([observation['institution_id'], observation['role_id'], holder])
            require(fingerprint not in seen_observations, pid + ': duplicate observation')
            seen_observations.add(fingerprint)
            binding = observation['binding']
            if binding == 'alias_held':
                held_aliases += 1
                require(bool(observation['hold_reason']), pid + ': alias requires an explicit hold')
            elif binding == 'existing_identity_proposal':
                direct_observations += 1
                require(identity['status'] == 'existing' and slot != 'heir', pid + ': unsafe existing binding')
            else:
                require(binding == 'new_identity_proposal' and identity['status'] == 'reserved_not_imported',
                        pid + ': new identity must remain a proposal')
            if binding != 'alias_held':
                require(entry['name'].removeprefix('Prince ') == holder['name'].removeprefix('Prince '),
                        pid + ': holder name needs a separate alias review')
            evidence = observation['evidence']
            require(set(evidence) == set(holder['claim_ids']), pid + ': holder claim set changed')
            for cid, pin in evidence.items():
                require(cid in claims, pid + ': unknown claim ' + cid)
                source, claim = claims[cid]
                require(pin == {'source_id': source['id'], 'url': source['url'], 'claim_sha256': digest(claim)},
                        pid + ': source or claim pin differs: ' + cid)
            reviewed = observation['reviewed_claim_ids']
            require(reviewed and set(reviewed) <= set(holder['claim_ids']), pid + ': invalid reviewed claims')
            require(observation['review_id'] in reviews, pid + ': missing acceptance receipt')
            require(bool(observation['review_limit']), pid + ': missing factual-review limit')
            for cid, pin in observation['bridge_evidence'].items():
                require(binding == 'alias_held' and cid in claims, pid + ': alias bridge cannot grant a binding')
                source, claim = claims[cid]
                require(pin == {'source_id': source['id'], 'url': source['url'], 'claim_sha256': digest(claim)},
                        pid + ': alias bridge claim changed')
            if binding == 'existing_identity_proposal':
                reviewed_urls.extend(claims[cid][0]['url'] for cid in reviewed)

        # Art intervals are editorial requests. They are never copied to role dates.
        art = entry['art']
        start, end = date.fromisoformat(art['from']), date.fromisoformat(art['to'])
        require(date(1990, 1, 1) <= start < end <= cutoff + timedelta(days=1),
                pid + ': invalid historical art interval')
        require(art['interval_meaning'] == 'appearance_request_not_office_tenure', pid + ': art/office conflation')
        if art['status'] == 'reuse_existing':
            reused += 1
            require(identity['status'] == 'existing', pid + ': cannot reuse another person\'s art')
            actual = portraits['people'].get(pid, {}).get('portraits', [])
            require(art['existing_portrait'] in actual, pid + ': existing portrait differs')
            portrait = art['existing_portrait']
            require((art['from'], art['to']) == (portrait['from'], portrait['to']),
                    pid + ': existing portrait interval widened')
            require(portrait['style'] == 'cartoon' and portrait['identity_source']['person_id'] == pid,
                    pid + ': reused portrait is not the exact person cartoon')
        else:
            require(art['status'] == 'reference_and_review_required' and art['existing_portrait'] is None,
                    pid + ': missing art cannot be marked approved')
            require(art['gates'] == ['exact_identity', 'dated_likeness', 'source_rights', 'physical_png',
                                     'identity_likeness_era_visual_review'], pid + ': art gates changed')
        allowed_dispositions = ({'reuse'} if art['status'] == 'reuse_existing' else
                                {'ready_for_dated_reference'} if identity['status'] == 'existing' else
                                {'identity_review_then_dated_reference'})
        require(art['disposition'] in allowed_dispositions and bool(art['next_action']),
                pid + ': misleading artwork readiness')

        if entry['people_only_preview']:
            require(identity['status'] == 'existing' and direct_observations > 0 and slot != 'heir',
                    pid + ': proposed people-only payload needs an existing direct identity')
            source_enrichment.append({'id': pid, 'name': people[pid]['name'],
                                      'sources': sorted(set(reviewed_urls))})

    return {
        'status': 'valid_proposal_only', 'nation': proposal['nation'],
        'existing_opening_identity_references': existing,
        'reserved_uninstalled_person_ids': reserved,
        'exact_research_holder_observations': observation_count,
        'held_alias_associations': held_aliases,
        'existing_portraits_reused': reused,
        'portrait_requests_needing_reference_and_review': len(entries) - reused,
        'people_only_preview': {'people': source_enrichment},
        'runtime_records_changed': 0, 'party_terms_added': 0, 'office_links_added': 0,
        'art_created_or_approved': 0, 'country_completed': False,
    }


def load_inputs(root=ROOT):
    return {key: read(root / path) for key, path in INPUTS.items()}


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('proposal', nargs='?', type=Path, default=ROOT / DEFAULT)
    args = parser.parse_args()
    result = check(read(args.proposal), **load_inputs())
    print(json.dumps(result, ensure_ascii=False, indent=2))
    return 0


if __name__ == '__main__':
    try:
        raise SystemExit(main())
    except (ValueError, KeyError, TypeError, OSError) as error:
        print(f'Country-cast proposal failed: {error}', file=sys.stderr)
        raise SystemExit(1)
