#!/usr/bin/env python3
"""S23 preparation: certified-country historical and cartoon boundary matrix.

Generates deterministic date/role/appearance cases for the eight CP1 country
cases from checked-in C01 research, the C01 census outputs and the production
bindings the game actually serves. It never edits research, runtime data, art or
saves, never accesses the network and never waives a gap.

Each case reports five things separately:

* historical identity: who the evidence places in the role on that date, and
  how strongly (stated interval, boundary day, day attestation, period
  attestation, bracketed, uncertain, unknown, unresearched or inapplicable);
* role: the research role, production party row/component or national executive;
* source acceptance: accepted C01 packet, integrated-but-pending C01 packet,
  unattributed discovery intake, production registry or authored fiction;
* actual image binding: a mirror of spheres-web/src/person_portraits.rs;
* asset availability: the bound file exists, matches its manifest hash and is on
  the served allowlist.

Unknown interval boundaries stay unknown: an isolated attestation covers its own
day only, a stated start without an end never becomes an established interval,
and one observation's start never ends another. Historical lookups and campaign
incumbents are compared without letting either overwrite the other; a divergent
campaign incumbent is an expected outcome, not an error.

Run normally to write the matrix, or with --check to fail on stale output.
"""
from __future__ import annotations

import argparse
from collections import Counter, defaultdict
from datetime import date, timedelta
import gzip
import hashlib
import json
from pathlib import Path
import re
import sys

ROOT = Path(__file__).resolve().parents[2]
OUTPUT = Path('docs/campaign-certification/S23/preparation/boundary-matrix')
FORMAT = 'spheres-s23-boundary-matrix/v1'
CASE_FORMAT = 'spheres-s23-boundary-matrix-cases/v1'

C01 = 'docs/campaign-certification/C01'
RESEARCH = C01 + '/research'
INTEGRATIONS = C01 + '/integrations'
PENDING_NOTE = 'docs/campaign-certification/verification/2026-09-27-claude-integration.md'
CENSUS = C01 + '/census.json'
COUNTRIES = C01 + '/countries.json'
ORGANIZATIONS = C01 + '/represented-organizations.json'
ROLES_AND_LIFECYCLE = C01 + '/roles-and-lifecycle.json'
REGISTRY = 'spheres-sim/data/party_leaders.json'
LEADERS_1990 = 'spheres-sim/data/leaders_1990.json'
ELIGIBILITY = 'spheres-sim/data/party_executive_eligibility.json'
PORTRAITS = 'spheres-web/data/person_portraits.json'
FICTIONAL_PORTRAITS = 'spheres-web/data/fictional_portraits.json'
FIGURES = 'spheres-web/data/nation_figures.json'
FUTURE = 'spheres-web/data/future_candidates_2035.json'
BOARD = 'spheres-web/data/leadership_production_2035.json'
ALLOWLIST = 'spheres-web/src/person_avatar_assets.rs'
PORTRAIT_PREFIX = 'spheres-web/ui/person-portraits/'
FUTURE_CATALOG = 'spheres-web/data/future_candidates_2035.json'
# Rust whose binding semantics this module mirrors. They are hashed so that a
# semantic change makes the committed matrix stale and forces a re-review.
MIRRORED_SEMANTICS = (
    'spheres-sim/src/clock.rs',
    'spheres-sim/src/party_leadership.rs',
    'spheres-sim/src/party_leadership_future.rs',
    'spheres-sim/src/party_executive_eligibility.rs',
    'spheres-web/src/person_portraits.rs',
)
SELF = 'tools/avatars/certified_boundary_matrix.py'

REFERENCE_FROM = date(1990, 1, 1)
CUTOFF = date(2026, 9, 7)
FICTIONAL_FROM = date(2026, 9, 8)
FICTIONAL_UNTIL = date(2036, 1, 1)  # exclusive
YEARS = range(1990, 2036)
FIXED_DATES = (
    (date(1989, 12, 31), 'reference_start_day_before'),
    (date(1990, 1, 1), 'reference_start'),
    (date(2026, 9, 6), 'cutoff_day_before'),
    (date(2026, 9, 7), 'cutoff_day'),
    (date(2026, 9, 8), 'fictional_start_day'),
    (date(2030, 1, 1), 'future_2030'),
    (date(2035, 12, 31), 'fictional_last_day'),
    (date(2036, 1, 1), 'fictional_end_exclusive'),
)
CAMPAIGN_START = REFERENCE_FROM

# Audit pairing for comparing the game's single national executive office with
# separately researched offices. The basis is the office title in
# spheres-sim/data/leaders_1990.json. It is a comparison aid only: it never maps
# an identity, grants an office or reconciles research to production.
EXECUTIVE_PAIRING = {
    'France': (),
    'Japan': ('jp_pm',),
    'India': ('in_pm',),
    'Brazil': ('br_president',),
    'SouthAfrica': ('za_state_president', 'za_president_election'),
    'Tonga': ('to_king',),
    'SaudiArabia': ('sa_king', 'sa_pm'),
    'USSR': ('su_cpsu_general_secretary', 'su_supreme_soviet_chair'),
    'Russia': ('ru_rsfsr_president', 'ru_president'),
}
PAIRING_BASIS = ('Audit pairing by the office named in the 1990 executive row (or, for the '
                 'successor Russia, the researched presidency). Comparison aid only; it is '
                 'not an identity reconciliation, office mapping or research acceptance.')

ACCEPTANCE_CLASSES = {
    'accepted': 'Evidence belongs to a C01 packet with an integration acceptance record (bounded research acceptance, not country certification).',
    'pending': 'Evidence belongs to a C01 packet integrated on 27 September 2026 whose historical acceptance remains pending.',
    'unclassified_packet': 'Evidence belongs to a C01 packet report with neither an acceptance record nor a pending-integration listing.',
    'unattributed_intake': 'Checked-in discovery evidence that no numbered C01 packet report claims; its intake batch and acceptance are not established by this audit.',
    'mixed_intake': 'Evidence mixes accepted packet sources with unattributed discovery sources; the whole observation is not labelled accepted.',
    'production_registry': 'spheres-sim/data/party_leaders.json: the partial production registry the game serves; not reviewed as C01 research.',
    'fictional_catalog': 'Authored future fiction (spheres-web/data/future_candidates_2035.json); never historical evidence.',
}

CAMPAIGN_VERDICTS = {
    'campaign_matches_reference': 'The campaign holder is among the production reference holders on that date.',
    'campaign_subset_by_rank_selection': 'The campaign seated a subset of the reference holders (leaders outrank acting holders).',
    'campaign_differs_from_reference': 'The fresh-start seating differs from the reference holders on the same date; a mirror defect to investigate.',
    'both_vacant': 'Neither the campaign nor the reference has a holder.',
    'campaign_diverged_expected': 'The campaign holder differs from the reference holder; an expected outcome of play, never an error.',
    'campaign_incumbent_unnamed': 'The campaign office is held by an unnamed emergent incumbent; no person or art can bind.',
    'campaign_role_vacant': 'The campaign has no holder for the role.',
    'historical_identity_not_established': 'The historical lookup does not establish a holder on that date, so nothing is compared.',
    'identity_reconciliation_required': 'History names a research holder that is not reconciled to a production person.',
    'identity_not_present_at_campaign_start': 'A successor identity has no office or party assignment at the 1990 start.',
    'no_campaign_observation': 'No saved campaign was observed for that date.',
}
STATUS_ORDER = ('boundary_day', 'established', 'attested', 'period_attested', 'bracketed')
RELATION_STATUS = {
    'stated_start_day': 'boundary_day', 'stated_end_day': 'boundary_day',
    'within_stated_interval': 'established', 'attested_on_day': 'attested',
    'within_attested_period': 'period_attested', 'within_observation_window': 'period_attested',
    'bracketed_by_evidence': 'bracketed',
}
IDENTIFIED = ('boundary_day', 'established', 'attested')


class MatrixError(ValueError):
    """Inconsistent inputs; never silently repaired."""


def require(condition, message):
    if not condition:
        raise MatrixError(message)


# ---------------------------------------------------------------- inputs ----

class Inputs:
    """Every file read is recorded with bytes and SHA-256.

    Text is hashed after CRLF->LF normalisation so that the digest is a claim
    about content, not about the checkout's newline convention (this repository
    is checked out with core.autocrlf=true on Windows). Binary files are hashed
    raw. A referenced asset that is absent is recorded as absent.
    """

    TEXT = {'.json', '.md', '.rs', '.py', '.txt'}

    def __init__(self, root: Path):
        self.root = Path(root)
        self.files = {}

    def _read(self, rel):
        path = self.root / rel
        require(path.is_file(), f'Required input is missing: {rel}')
        raw = path.read_bytes()
        normalized = Path(rel).suffix in self.TEXT
        data = raw.replace(b'\r\n', b'\n') if normalized else raw
        self.files[rel] = {'path': rel, 'bytes': len(data),
                           'sha256': hashlib.sha256(data).hexdigest(),
                           'newline_normalized': normalized}
        return data

    def text(self, rel):
        return self._read(rel).decode('utf-8-sig')

    def json(self, rel):
        return json.loads(self.text(rel))

    def asset(self, rel):
        """Hash a referenced binary asset; absence is an observation, not an error."""
        if rel in self.files:
            return self.files[rel]
        path = self.root / rel
        if not path.is_file():
            self.files[rel] = {'path': rel, 'exists': False, 'bytes': None, 'sha256': None,
                               'newline_normalized': False}
        else:
            self._read(rel)
            self.files[rel]['exists'] = True
        return self.files[rel]

    def glob(self, rel_dir, pattern):
        base = self.root / rel_dir
        return sorted(p.relative_to(self.root).as_posix() for p in base.glob(pattern) if p.is_file())

    def listing(self):
        rows = []
        for rel in sorted(self.files):
            row = dict(self.files[rel])
            row.setdefault('exists', True)
            rows.append(row)
        return rows


# ----------------------------------------------------------------- dates ----

def iso(value):
    return value.isoformat() if value else None


def day(value):
    return date.fromisoformat(value) if value else None


def exact_day(value):
    """Mirror the served portrait selector's exact Gregorian YYYY-MM-DD guard."""
    if not isinstance(value, str):
        return False
    try:
        return date.fromisoformat(value).isoformat() == value
    except ValueError:
        return False


def month_end(y, m):
    return (date(y + (m == 12), m % 12 + 1, 1) - timedelta(days=1))


def bound(db):
    """Mirror party_leadership::DateBound::bounds -> (earliest, latest) or None."""
    if not db:
        return None
    kind, value = db.get('kind'), db.get('value')
    if kind == 'day':
        d = date.fromisoformat(value)
        return d, d
    if kind == 'month':
        y, m = int(value[:4]), int(value[5:7])
        return date(y, m, 1), month_end(y, m)
    if kind == 'year':
        y = int(value)
        return date(y, 1, 1), date(y, 12, 31)
    return None


def within(start, until, at):
    """Mirror party_leadership::within (half-open, conservative bounds)."""
    first = bound(start)
    if not first:
        return False
    if until and until.get('kind') == 'open':
        end = CUTOFF + timedelta(days=1)
    else:
        last = bound(until)
        end = last[0] if last else None
    return end is not None and first[1] <= at < end


def life_contains(born, died, at):
    """Mirror party_leadership::life_contains."""
    b, d = bound(born), bound(died)
    return (b is None or at >= b[1]) and (d is None or at < d[0])


def possible_life(born, died, at):
    b, d = bound(born), bound(died)
    return (b is None or at >= b[0]) and (d is None or at < d[1])


def slug(text):
    text = re.sub(r'(?<=[a-z])(?=[A-Z])', '-', text)
    return re.sub(r'[^a-z0-9]+', '-', text.lower()).strip('-')


# ------------------------------------------------------ packet provenance ----

PACKET_LINE = re.compile(r'^Packet: \*\*(CLAUDE-C01-\d+)\*\*', re.M)
SOURCES_ADDED = re.compile(r'^## Sources added\s*$(.*?)(?=^## |\Z)', re.M | re.S)
SOURCE_ROW = re.compile(r'^\|\s*`([A-Za-z0-9_.:-]+)`\s*\|', re.M)
INTEGRATION_DIR = re.compile(r'^CLAUDE-C01-(\d+(?:-\d+)*)$')
PENDING_ROW = re.compile(r'^\|\s*(C01-\d+)\s*\|\s*`([0-9a-f]{40})`\s*\|', re.M)


def packet_provenance(inputs):
    """Classify C01 packets and attribute each research source to its packet(s).

    A packet report's "Sources added" table names the sources it contributed.
    Acceptance comes from integration records; pending status from the
    27 September 2026 integration note. A source that no packet report claims
    is unattributed discovery intake, not proof of its batch or acceptance.
    """
    accepted = {}
    base = inputs.root / INTEGRATIONS
    for folder in sorted(p for p in base.iterdir() if p.is_dir()):
        match = INTEGRATION_DIR.match(folder.name)
        readme = f'{INTEGRATIONS}/{folder.name}/README.md'
        if not match or not (inputs.root / readme).is_file():
            continue
        text = inputs.text(readme)
        # A mention such as "not accepted" or "previously accepted" is not a
        # decision. Match the explicit decision forms in the reviewed records.
        if not re.search(r'^(?:\*\*Decision: accepted\b|Accepted as a bounded research intake\b)', text, re.M):
            continue
        for number in match.group(1).split('-'):
            accepted[f'CLAUDE-C01-{number}'] = readme
    note = inputs.text(PENDING_NOTE)
    section = note.split('## Integrated submissions', 1)
    require(len(section) == 2, 'Pending integration note lacks its submission table')
    pending = {f'CLAUDE-{packet}': head for packet, head in PENDING_ROW.findall(section[1])}
    require(pending, 'Pending integration note lists no packets')
    # Folders such as CLAUDE-C01-SOURCE-05 or CLAUDE-C01-GAPS-01 record bounded
    # source repairs or tools; they never accept a whole numbered packet.
    others = sorted(p.name for p in base.iterdir() if p.is_dir() and not INTEGRATION_DIR.match(p.name))
    packets, owners = {}, defaultdict(set)
    for rel in inputs.glob(RESEARCH, '*.md'):
        if rel.endswith('/README.md'):
            continue
        text = inputs.text(rel)
        match = PACKET_LINE.search(text)
        require(match, f'Packet report without a packet identifier: {rel}')
        packet = match.group(1)
        require(packet not in packets, f'Duplicate packet report: {packet}')
        section = SOURCES_ADDED.search(text)
        sources = SOURCE_ROW.findall(section.group(1)) if section else []
        if packet in accepted:
            status, basis = 'accepted', accepted[packet]
        elif packet in pending:
            status, basis = 'pending', PENDING_NOTE
        else:
            status, basis = 'unclassified_packet', None
        packets[packet] = {'packet': packet, 'report': rel, 'status': status,
                           'status_source': basis, 'reviewed_head': pending.get(packet),
                           'sources_listed': len(sources)}
        for source in sources:
            owners[source].add(packet)
    return packets, owners, sorted(set(accepted) | set(pending)), others


def evidence_class(source_ids, owners, packets):
    classes, found = Counter(), set()
    for source in source_ids:
        packs = owners.get(source)
        if not packs:
            classes['unattributed_intake'] += 1
            continue
        for packet in packs:
            classes[packets[packet]['status']] += 1
            found.add(packet)
    if classes['pending']:
        overall = 'pending'
    elif classes['unclassified_packet']:
        overall = 'unclassified_packet'
    elif classes['accepted'] and classes['unattributed_intake']:
        overall = 'mixed_intake'
    elif classes['accepted']:
        overall = 'accepted'
    else:
        overall = 'unattributed_intake'
    return overall, sorted(found), dict(sorted(classes.items()))


# --------------------------------------------------------- research model ----

def research_model(identity, packet, owners, packets):
    """Roles and holder observations for one identity's C01 research packet."""
    claims, claim_source = {}, {}
    for source in packet['sources']:
        for claim in source['claims']:
            claims[claim['id']] = claim
            claim_source[claim['id']] = source['id']
    roles, observations = [], {}
    for category in ('organizations', 'institutions'):
        for entry in packet[category]:
            lifecycle = entry.get('lifecycle', {})
            for role in entry['roles']:
                key = 'research:' + role['id']
                obs_ids = []
                for index, holder in enumerate(role.get('holder_claims', []), 1):
                    oid = f"{role['id']}#{index}"
                    if isinstance(holder, str):
                        claim = claims[holder]
                        period = claim.get('period')
                        attested = claim.get('attested_on')
                        span = None
                        if not attested and period:
                            if period.get('from') == period.get('through'):
                                attested = period.get('from')
                            else:
                                span = {'from': period.get('from'), 'through': period.get('through')}
                        sources = [claim_source[holder]]
                        record = {'role': key, 'name': None, 'identity_text': claim['text'],
                                  'claim': holder, 'from': None, 'until': None,
                                  'attested_on': attested}
                        if span:
                            record['attested_period'] = span
                        for field in ('precision', 'uncertainty'):
                            if claim.get(field):
                                record[field] = claim[field]
                    else:
                        sources = holder['sources']
                        record = {'role': key, 'name': holder['name'], 'from': holder.get('from'),
                                  'until': holder.get('until'), 'attested_on': holder.get('attested_on')}
                        if holder.get('attested_period'):
                            record['attested_period'] = holder['attested_period']
                        if holder.get('observation_window'):
                            record['observation_window'] = holder['observation_window']
                            record['precision'] = holder.get('precision')
                        record['claim_ids'] = holder['claim_ids']
                        for field in ('precision', 'note', 'uncertainty'):
                            if holder.get(field):
                                record[field] = holder[field]
                    acceptance, found, classes = evidence_class(sources, owners, packets)
                    record.update({'sources': sources, 'acceptance': acceptance,
                                   'packets': found, 'source_classes': classes})
                    observations[oid] = record
                    obs_ids.append(oid)
                roles.append({
                    'key': key, 'identity': identity, 'family': 'research_role',
                    'role_id': role['id'], 'title': role['title'], 'kind': role['kind'],
                    'entry': {'id': entry['id'], 'category': category, 'name': entry['name'],
                              'kind': entry.get('kind'),
                              'lifecycle': {k: lifecycle.get(k) for k in ('status', 'from', 'until')}},
                    'production_binding': 'none: research-only role; no game party row, office or person is mapped (represented_party_ids is empty/unreconciled)',
                    'observations': obs_ids,
                })
    return roles, observations


def research_relation(obs, at):
    """Relation of one research observation to a date. Never extends an interval."""
    start, end, attested = day(obs.get('from')), day(obs.get('until')), day(obs.get('attested_on'))
    if start == at:
        return 'stated_start_day'
    if end == at:
        return 'stated_end_day'
    if start and end and start < at < end:
        return 'within_stated_interval'
    if attested == at:
        return 'attested_on_day'
    for key, relation in (('attested_period', 'within_attested_period'),
                          ('observation_window', 'within_observation_window')):
        span = obs.get(key)
        if span and day(span['from']) <= at <= day(span['through']):
            return relation
    evidence = [d for d in (start, end, attested) if d]
    for key in ('attested_period', 'observation_window'):
        if obs.get(key):
            evidence += [day(obs[key]['from']), day(obs[key]['through'])]
    if len(evidence) >= 2 and min(evidence) < at < max(evidence):
        return 'bracketed_by_evidence'
    if start and not end and at > start:
        return 'after_stated_start'
    if end and not start and at < end:
        return 'before_stated_end'
    return None


def research_cell(role, observations, at):
    lifecycle = role['entry']['lifecycle']
    if at < REFERENCE_FROM:
        return {'status': 'inapplicable', 'reason': 'before_reference_period'}
    if at > CUTOFF:
        return {'status': 'inapplicable', 'reason': 'after_historical_cutoff'}
    created, ended = day(lifecycle.get('from')), day(lifecycle.get('until'))
    if created and at < created:
        return {'status': 'inapplicable', 'reason': 'institution_not_yet_created_per_source'}
    if ended and at >= ended:
        return {'status': 'inapplicable', 'reason': 'institution_ended_per_source'}
    if not role['observations']:
        return {'status': 'unresearched', 'reason': 'role_has_no_holder_observation'}
    holders, one_sided, windows = [], [], []
    for oid in role['observations']:
        relation = research_relation(observations[oid], at)
        if relation in ('within_attested_period', 'within_observation_window'):
            # The source may attest the holder sometime within a month, year,
            # or multi-day hearing. It does not establish every individual day.
            windows.append([oid, relation])
        elif relation in RELATION_STATUS:
            holders.append([oid, relation])
        elif relation:
            one_sided.append((oid, relation))
    cell = {}
    if holders:
        cell['status'] = min((RELATION_STATUS[r] for _, r in holders), key=STATUS_ORDER.index)
        cell['holders'] = holders
    elif windows:
        cell['status'] = 'period_attested'
        cell['reason'] = 'observation_window_not_an_exact_day_or_term'
    elif one_sided:
        cell['status'] = 'uncertain'
        cell['reason'] = 'one_sided_observation_only'
    else:
        cell['status'] = 'unknown'
        cell['reason'] = 'no_observation_covers_date'
    if one_sided and not holders:
        # Pointers, not incumbents: the most recent stated start and the nearest
        # stated end. Their other boundaries remain unknown.
        after = [(observations[o]['from'], o) for o, r in one_sided if r == 'after_stated_start']
        before = [(observations[o]['until'], o) for o, r in one_sided if r == 'before_stated_end']
        possible = []
        if after:
            possible.append([max(after)[1], 'after_stated_start'])
        if before:
            possible.append([min(before)[1], 'before_stated_end'])
        cell['possible'] = possible
        if len(one_sided) > len(possible):
            cell['possible_total'] = len(one_sided)
    if windows:
        cell.setdefault('possible', []).extend(windows)
    names = {observations[o]['name'] or observations[o].get('claim') for o, r in holders
             if RELATION_STATUS[r] != 'boundary_day'}
    if len(names) > 1:
        cell['multiple_holders'] = sorted(names)
    return cell


def research_boundaries(role, observations):
    marks = defaultdict(set)
    lifecycle = role['entry']['lifecycle']
    if lifecycle.get('from'):
        add_triple(marks, day(lifecycle['from']), 'creation')
    if lifecycle.get('until'):
        add_triple(marks, day(lifecycle['until']), 'institution_end')
    for oid in role['observations']:
        obs = observations[oid]
        for key in ('from', 'until'):
            if obs.get(key):
                add_triple(marks, day(obs[key]), 'handover')
        if obs.get('attested_on'):
            attested = day(obs['attested_on'])
            if REFERENCE_FROM <= attested <= CUTOFF:
                marks[attested].add('attestation_day')
                marks[attested + timedelta(days=1)].add('attestation_day_after')
        for key in ('attested_period', 'observation_window'):
            span = obs.get(key) or {}
            for endpoint in ('from', 'through'):
                if span.get(endpoint):
                    add_triple(marks, day(span[endpoint]), f'{key}_{endpoint}')
    return marks


def add_triple(marks, boundary, label):
    if not boundary or not REFERENCE_FROM <= boundary <= CUTOFF:
        return
    marks[boundary - timedelta(days=1)].add(f'{label}_day_before')
    marks[boundary].add(f'{label}_day_of')
    marks[boundary + timedelta(days=1)].add(f'{label}_day_after')


# ------------------------------------------------------- production model ----

class Production:
    def __init__(self, inputs):
        self.inputs = inputs
        self.registry = inputs.json(REGISTRY)
        require(self.registry['reference_from'] == iso(REFERENCE_FROM)
                and self.registry['reference_through'] == iso(CUTOFF),
                'Registry reference period changed; review the frozen cutoff first')
        self.people = {p['id']: p for p in self.registry['people']}
        self.parties = {(p['nation'], p['party']): p for p in self.registry['parties']}
        self.links = {l['nation']: l for l in self.registry['office_links']}
        self.leaders = {r['nation']: r for r in inputs.json(LEADERS_1990)['rows']}
        eligibility = inputs.json(ELIGIBILITY)
        self.grants = {(g['nation'], g['party'], g['term']): g for g in eligibility['historical_grants']}
        self.future_grants = eligibility['future_office_grants']
        self.portraits = inputs.json(PORTRAITS)['people']
        self.fictional_portraits = inputs.json(FICTIONAL_PORTRAITS)['people']
        self.figures = inputs.json(FIGURES)['nations']
        future = inputs.json(FUTURE)
        require((future['historical_reference_through'], future['from'], future['until_exclusive'])
                == (iso(CUTOFF), iso(FICTIONAL_FROM), iso(FICTIONAL_UNTIL)),
                'Fictional window changed; review the frozen boundary first')
        self.candidates = {c['person_id']: c for c in future['candidates']}
        self.pool = defaultdict(list)
        for c in future['candidates']:
            self.pool[(c['nation'], c['party'], c.get('component'))].append(c)
        board = inputs.json(BOARD)
        self.jobs = defaultdict(list)
        for job in board['historical_art_jobs']:
            self.jobs[job['person_id']].append(job)
        self.board_people = {p['id']: p for p in board['people']}
        allowlist = inputs.text(ALLOWLIST)
        served = re.findall(r'"([^"]+\.png)" => Some\(include_bytes!\("\.\./ui/person-portraits/([^"]+)"\)\)', allowlist)
        require(served and all(a == b for a, b in served), 'Unreadable or inconsistent served avatar allowlist')
        self.allowlist = {a for a, _ in served}
        organizations = inputs.json(ORGANIZATIONS)
        self.organizations = {o['id']: o for o in organizations}
        lifecycle = inputs.json(ROLES_AND_LIFECYCLE)
        self.disclosures = defaultdict(list)
        for row in lifecycle['lifecycle_disclosures']:
            self.disclosures[row['organization_id']].append(row)
        census_terms = {t['id'] for t in lifecycle['term_records']}
        registry_terms = {t['id'] for p in self.registry['parties'] for t in p['terms']}
        self.census_consistent = census_terms == registry_terms and set(self.organizations) >= {
            f"{n}/{p}" for n, p in self.parties}
        self.backlog = {row['person_id'] for row in lifecycle['appearance_eligibility_research_backlog']}

    # -- mirrored runtime rules -------------------------------------------
    def historical_on(self, party, term, at):
        person = self.people.get(term['person'])
        if not person or not REFERENCE_FROM <= at <= CUTOFF or party['kind'] == 'unknown':
            return False
        component = self.component(party, term.get('component'))
        return (within(term['from'], term['until'], at)
                and life_contains(person.get('born'), person.get('died'), at)
                and life_contains(party.get('founded'), party.get('dissolved'), at)
                and (term.get('component') is None or (component is not None and
                     life_contains(component.get('founded'), component.get('dissolved'), at))))

    def possibly_historical_on(self, party, term, at):
        if term['kind'] == 'candidate' or party['kind'] == 'unknown' or self.historical_on(party, term, at):
            return False
        if not REFERENCE_FROM <= at <= CUTOFF:
            return False
        first, last = bound(term['from']), bound(term['until'])
        start = first[0] if first else REFERENCE_FROM
        end = last[1] if last else CUTOFF + timedelta(days=1)
        person = self.people.get(term['person'])
        component = self.component(party, term.get('component'))
        return (start <= at < end and person is not None
                and possible_life(person.get('born'), person.get('died'), at)
                and possible_life(party.get('founded'), party.get('dissolved'), at)
                and (term.get('component') is None or (component is not None and
                     possible_life(component.get('founded'), component.get('dissolved'), at))))

    def eligible(self, party, term, at):
        return self.historical_on(party, term, at) and within(term['affiliation_from'], term['affiliation_until'], at)

    @staticmethod
    def component(party, cid):
        return next((c for c in party.get('components', []) if c['id'] == cid), None) if cid else None

    @staticmethod
    def rank(term):
        return {'leader': 0, 'co_leader': 0, 'acting': 1}.get(term['kind'], 2)

    def initial_holders(self, party, component, at):
        """Mirror party_leadership::choose(..., initial=true) at campaign start."""
        terms = [t for t in party['terms'] if t.get('component') == component
                 and self.eligible(party, t, at) and t['kind'] != 'candidate']
        terms.sort(key=lambda t: (self.rank(t), t['id']))
        if not terms:
            return []
        best, seen, result = self.rank(terms[0]), set(), []
        for t in terms:
            if self.rank(t) == best and t['person'] not in seen:
                seen.add(t['person'])
                result.append(t['person'])
        return result

    # -- image binding ------------------------------------------------------
    def portrait(self, person_id, at):
        """Mirror person_portraits::portrait with an explicit reason when unbound."""
        if person_id in self.candidates:
            return self.fictional_portrait(person_id, at)
        entry = self.portraits.get(person_id)
        records = entry.get('portraits', []) if entry else []
        if not records:
            return {'person': person_id, 'binding': 'unbound', 'reason': 'no_portrait_record'}
        covering = [p for p in records if exact_day(p.get('from')) and p['from'] <= iso(at)
                    and (p.get('to') is None or (exact_day(p['to']) and iso(at) < p['to']))]
        valid = [p for p in covering
                 if (p.get('identity_source') or {}).get('person_id') == person_id
                 and p.get('style') == 'cartoon' and p.get('method') == 'generated'
                 and p.get('status') == 'illustrated-likeness'
                 and all((p.get('review') or {}).get(k) is True for k in ('identity', 'likeness', 'era', 'visual'))]
        if not covering:
            return {'person': person_id, 'binding': 'unbound', 'reason': 'no_record_covers_date'}
        if not valid:
            identity_ok = any((p.get('identity_source') or {}).get('person_id') == person_id for p in covering)
            return {'person': person_id, 'binding': 'unbound',
                    'reason': 'record_not_reviewed_or_wrong_style' if identity_ok else 'record_identity_mismatch'}
        if len(valid) != 1:
            return {'person': person_id, 'binding': 'unbound', 'reason': 'ambiguous_multiple_records'}
        return self._served(person_id, valid[0])

    def fictional_portrait(self, person_id, at):
        if not FICTIONAL_FROM <= at < FICTIONAL_UNTIL:
            return {'person': person_id, 'binding': 'unbound', 'reason': 'outside_fictional_window'}
        candidate = self.candidates[person_id]
        entry = self.fictional_portraits.get(person_id)
        if not entry or not entry.get('portraits'):
            return {'person': person_id, 'binding': 'unbound', 'reason': 'no_portrait_record'}
        if entry.get('name') != candidate['name'] or entry.get('appearance_seed') != candidate['appearance_seed']:
            return {'person': person_id, 'binding': 'unbound', 'reason': 'record_identity_mismatch'}
        covering = [p for p in entry['portraits'] if exact_day(p.get('from')) and p['from'] <= iso(at)
                    and exact_day(p.get('to')) and iso(at) < p['to']]
        valid = [p for p in covering
                 if (p.get('design_source') or {}).get('kind') == 'authored_fiction'
                 and p['design_source'].get('person_id') == person_id
                 and p['design_source'].get('appearance_seed') == candidate['appearance_seed']
                 and p['design_source'].get('catalog') == FUTURE_CATALOG
                 and p.get('style') == 'cartoon' and p.get('method') == 'generated'
                 and p.get('status') == 'fictional-character'
                 and (p.get('review') or {}).get('design') is True and p['review'].get('visual') is True
                 and p.get('identity_source') is None and p.get('source_url') is None
                 and all(p['review'].get(k) is None for k in ('identity', 'likeness', 'era'))
                 and p['from'] >= iso(FICTIONAL_FROM) and p['to'] <= iso(FICTIONAL_UNTIL)]
        if not covering:
            return {'person': person_id, 'binding': 'unbound', 'reason': 'no_record_covers_date'}
        if len(valid) != 1:
            return {'person': person_id, 'binding': 'unbound',
                    'reason': 'ambiguous_multiple_records' if len(valid) > 1 else 'record_not_reviewed_or_wrong_style'}
        return self._served(person_id, valid[0])

    def _served(self, person_id, record):
        asset = record.get('asset') or ''
        if not asset.startswith(PORTRAIT_PREFIX):
            return {'person': person_id, 'binding': 'unbound', 'reason': 'asset_path_outside_portrait_directory'}
        if asset[len(PORTRAIT_PREFIX):] not in self.allowlist:
            return {'person': person_id, 'binding': 'unbound', 'reason': 'asset_not_on_served_allowlist', 'asset': asset}
        return {'person': person_id, 'binding': 'bound', 'asset': asset,
                'window': [record['from'], record.get('to')]}

    def art_job(self, person_id, at):
        for job in sorted(self.jobs.get(person_id, []), key=lambda j: j['id']):
            if job['from'] <= iso(at) < job['to']:
                return job['id']
        return None

    def image(self, person_id, at, listed_as=None):
        result = self.portrait(person_id, at)
        if listed_as:
            result = {'person': person_id, 'as': listed_as, **{k: v for k, v in result.items() if k != 'person'}}
        if result['binding'] != 'bound' and person_id not in self.candidates:
            job = self.art_job(person_id, at)
            result['art_job'] = job if job else 'none_covers_date'
        return result


# ----------------------------------------------------------------- assets ----

class Assets:
    def __init__(self, inputs, production):
        self.inputs, self.production, self.rows = inputs, production, {}
        users = defaultdict(set)
        for manifest in (production.portraits, production.fictional_portraits):
            for pid, entry in manifest.items():
                for record in entry.get('portraits', []):
                    if record.get('asset'):
                        users[record['asset']].add(pid)
        self.users = users

    def check(self, asset):
        if asset in self.rows:
            return self.rows[asset]
        info = self.inputs.asset(asset)
        recorded = sorted({r.get('sha256') for manifest in (self.production.portraits, self.production.fictional_portraits)
                           for entry in manifest.values() for r in entry.get('portraits', [])
                           if r.get('asset') == asset}, key=lambda value: json.dumps(value))
        name = asset[len(PORTRAIT_PREFIX):] if asset.startswith(PORTRAIT_PREFIX) else asset
        people = sorted(self.users.get(asset, ()))
        slug_ok = all(name.startswith(('fictional-' if p in self.production.candidates else p.replace('_', '-') + '-'))
                      for p in people)
        row = {'asset': asset, 'exists': info['exists'], 'bytes': info['bytes'], 'sha256': info['sha256'],
               'manifest_sha256': recorded, 'sha256_matches_manifest': info['exists'] and recorded == [info['sha256']],
               'on_served_allowlist': name in self.production.allowlist,
               'referenced_by': people, 'shared_across_people': len(people) > 1,
               'filename_matches_person': slug_ok}
        if not row['exists']:
            row['status'] = 'missing_file'
        elif not row['sha256_matches_manifest']:
            row['status'] = 'sha256_mismatch'
        elif not row['on_served_allowlist']:
            row['status'] = 'not_on_served_allowlist'
        elif row['shared_across_people'] or not slug_ok:
            row['status'] = 'possible_wrong_person_binding'
        else:
            row['status'] = 'available'
        self.rows[asset] = row
        return row

    def cell(self, images):
        result = []
        for asset in sorted({i['asset'] for i in images if i.get('binding') == 'bound'}):
            result.append([asset, self.check(asset)['status']])
        return result


# --------------------------------------------------------------- matrix ----

def case_dates(extra):
    marks = defaultdict(set)
    for year in YEARS:
        marks[date(year, 1, 1)].add('yearly')
    for when, label in FIXED_DATES:
        marks[when].add(label)
    for when, labels in extra.items():
        marks[when] |= labels
    return sorted(marks.items())


def production_boundaries(production, party, component):
    marks = defaultdict(set)
    entity = production.component(party, component) if component else party
    for key, label in (('founded', 'founding'), ('dissolved', 'dissolution')):
        span = bound(entity.get(key)) if entity else None
        if span and span[0] == span[1]:
            add_triple(marks, span[0], label)
    if component:
        for key, label in (('founded', 'founding'), ('dissolved', 'dissolution')):
            span = bound(party.get(key))
            if span and span[0] == span[1]:
                add_triple(marks, span[0], label)
    for term in party['terms']:
        if term.get('component') != component:
            continue
        for key in ('from', 'until'):
            span = bound(term[key])
            if span and span[0] == span[1]:
                add_triple(marks, span[0], 'handover')
        person = production.people.get(term['person'], {})
        died = bound(person.get('died'))
        if died and died[0] == died[1]:
            first, last = bound(term['from']), bound(term['until'])
            earliest = first[0] if first else REFERENCE_FROM
            latest = last[1] if last else CUTOFF
            if earliest <= died[0] <= latest:
                add_triple(marks, died[0], 'death')
    return marks


def production_role(production, identity, party, component):
    oid = f"{identity}/{party['party']}" + (f"/{component}" if component else '')
    organization = production.organizations.get(oid, {})
    entity = production.component(party, component) if component else party
    terms = [t for t in party['terms'] if t.get('component') == component]
    lifecycle = {k: entity.get(k) for k in ('founded', 'dissolved') if entity and entity.get(k)}
    if component:
        lifecycle.update({f'party_{k}': party.get(k) for k in ('founded', 'dissolved') if party.get(k)})
    pool = production.pool.get((identity, party['party'], component), [])
    continuation = [d for d in production.disclosures.get(f"{identity}/{party['party']}", [])
                    if d['kind'] == 'future_editorial_continuation_disclosure']
    return {
        'key': f"party:{party['party']}" + (f"/{component}" if component else ''),
        'identity': identity, 'family': 'production_party_leadership',
        'title': (entity or {}).get('name') or organization.get('name') or party['party'],
        'party': party['party'], 'component': component, 'organization_id': oid,
        'party_kind': party['kind'], 'coverage': party['coverage'],
        'history_status': organization.get('history_status'),
        'lifecycle': lifecycle,
        # Registry gaps are declared per party row and apply to each of its components.
        'declared_gaps': [{'from': g['from'], 'until': g['until'], 'reason': g['reason']}
                          for g in party.get('gaps', [])],
        'observations': [t['id'] for t in terms],
        'fictional_pool': [c['person_id'] for c in pool],
        'continuation_disclosure': continuation[0]['status'] if continuation else None,
        'production_binding': 'party_leaders.json terms served by party_leadership::reference_view; portraits by person_portraits::portrait',
    }


def gap_overlaps(gaps, at):
    result = []
    for index, gap in enumerate(gaps):
        first, last = bound(gap['from']), bound(gap['until'])
        start = first[0] if first else date.min
        end = last[1] if last else date.max
        if start <= at <= end:
            result.append(index)
    return result


def production_cell(production, role, party, at):
    component = role['component']
    terms = [t for t in party['terms'] if t.get('component') == component]
    entity = production.component(party, component) if component else party
    cell = {}
    if at < REFERENCE_FROM:
        cell.update(status='inapplicable', reason='before_reference_period')
    elif at > CUTOFF:
        cell.update(status='inapplicable', reason='after_historical_cutoff')
    elif not (possible_life(party.get('founded'), party.get('dissolved'), at)
              and (not component or possible_life(entity.get('founded'), entity.get('dissolved'), at))):
        cell.update(status='inapplicable', reason='organization_not_existing')
    elif party['kind'] == 'unknown':
        cell.update(status='unresearched', reason='party_kind_unknown_no_researched_chain')
    else:
        holders = [[t['id'], 'historical_on'] for t in terms
                   if production.historical_on(party, t, at) and t['kind'] != 'candidate']
        possible = [[t['id'], 'uncertain_bounds'] for t in terms if production.possibly_historical_on(party, t, at)]
        eligible = [t['id'] for t in terms if production.eligible(party, t, at)]
        if holders:
            cell['status'] = 'established'
            cell['holders'] = holders
            kinds = {next(t['kind'] for t in terms if t['id'] == h[0]) for h in holders}
            if len(holders) > 1:
                cell['multiple_holders'] = 'co_leaders' if kinds == {'co_leader'} else 'overlapping_' + '_'.join(sorted(kinds))
        elif possible:
            cell.update(status='uncertain', reason='imprecise_or_unknown_bounds_only')
        elif not terms:
            cell.update(status='unresearched', reason='no_registry_term_for_organization')
        else:
            cell.update(status='unknown', reason='no_registry_term_covers_date')
        if possible:
            cell['possible'] = possible
        if sorted(eligible) != sorted(h[0] for h in holders):
            cell['eligible'] = eligible
        gaps = gap_overlaps(role['declared_gaps'], at)
        if gaps:
            cell['declared_gaps'] = gaps
    return cell


def term_person(party, term_id):
    return next(t['person'] for t in party['terms'] if t['id'] == term_id)


def future_cell(production, identity, party, role, at):
    if not FICTIONAL_FROM <= at < FICTIONAL_UNTIL:
        return {'eligible': False,
                'reason': 'before_fictional_window' if at < FICTIONAL_FROM else 'after_fictional_window'}
    pool = production.pool.get((identity, party['party'], role['component']), [])
    component = production.component(party, role['component'])
    entity = component if role['component'] else party
    exists = (bound(party.get('founded')) is None or bound(party['founded'])[0] <= at) and \
             (bound(party.get('dissolved')) is None or bound(party['dissolved'])[1] > at)
    if role['component']:
        exists = exists and (bound(entity.get('founded')) is None or bound(entity['founded'])[0] <= at) and \
            (bound(entity.get('dissolved')) is None or bound(entity['dissolved'])[1] > at)
    bound_people, reasons = [], Counter()
    for candidate in pool:
        result = production.portrait(candidate['person_id'], at)
        if result['binding'] == 'bound':
            bound_people.append(candidate['person_id'])
        else:
            reasons[result['reason']] += 1
    available = all(c.get('component_available') for c in pool) if pool else False
    cell = {'eligible': True, 'candidates': len(pool),
            'eligibility_scope': 'fictional_window_only; a live campaign vacancy and saved eligibility are still required',
            'component_available_for_campaign_vacancy': available,
            'listed_in_simulation_future_reference': exists and available,
            'served_web_historical_reference': False,
            'executive_authorized': sum(bool(c['executive_eligibility'].get('authorized')) for c in pool)}
    if bound_people:
        cell['portraits_bound'] = bound_people
    if reasons:
        cell['portraits_unbound'] = dict(sorted(reasons.items()))
    return cell


def executive_role(production, identity, research_roles):
    row = production.leaders.get(identity)
    link = production.links.get(identity)
    paired = [f'research:{rid}' for rid in EXECUTIVE_PAIRING.get(identity, ())]
    missing = [key for key in paired if key not in research_roles]
    pool = [c for c in production.candidates.values() if c['nation'] == identity]
    authorized = sum(bool(c['executive_eligibility'].get('authorized')) for c in pool)
    return {
        'key': 'executive', 'identity': identity, 'family': 'production_national_executive',
        'title': row['office'] if row else 'none at campaign start (successor identity)',
        'start_row': {'name': row['name'], 'office': row['office'], 'since': row['since'],
                      'tie': row.get('tie')} if row else None,
        'office_link': {'person': link['person'], 'since': link['since']} if link else None,
        'office_link_matches_start_row': bool(row and link and link['since'] == row['since']
                                              and row['name'] in (production.people[link['person']]['name'],
                                                                  production.people[link['person']].get('native'))),
        'paired_research_roles': [k for k in paired if k not in missing],
        'paired_research_roles_missing': missing,
        'pairing_basis': PAIRING_BASIS,
        'future_executive_policy': {
            'fictional_candidates': len(pool), 'executive_authorized_candidates': authorized,
            'reviewed_future_office_grants': sum(g['nation'] == identity for g in production.future_grants),
            'note': 'A fictional national executive can be seated only through an actual campaign succession; unauthorized pools leave a future executive unnamed.'},
        'production_binding': 'leaders_1990.json start row and party_leaders.json office_links at campaign start only; later incumbents are produced by play, never scheduled by date.',
        'observations': [],
    }


def combine_research(cells):
    holders, possible, reasons = [], [], []
    statuses = [c['status'] for c in cells]
    for c in cells:
        holders += c.get('holders', [])
        possible += c.get('possible', [])
        if c.get('reason'):
            reasons.append(c['reason'])
    for status in STATUS_ORDER + ('uncertain', 'unknown', 'unresearched', 'inapplicable'):
        if status in statuses:
            break
    cell = {'status': status}
    if holders:
        cell['holders'] = holders
    if possible and not holders:
        cell['possible'] = possible
    if not holders and reasons:
        cell['reason'] = sorted(set(reasons))[0] if len(set(reasons)) == 1 else 'mixed:' + ','.join(sorted(set(reasons)))
    return cell


def acceptance_of(historical, observations, fallback=None):
    refs = historical.get('holders') or historical.get('possible') or []
    classes = sorted({observations[ref[0]]['acceptance'] for ref in refs if ref[0] in observations})
    return classes or ([fallback] if fallback else ['none'])


def campaign_start_party(production, party, component, historical_holders):
    campaign = production.initial_holders(party, component, CAMPAIGN_START)
    history = [term_person(party, h[0]) for h in historical_holders]
    if set(campaign) == set(history):
        verdict = 'campaign_matches_reference' if campaign else 'both_vacant'
    elif set(campaign) < set(history):
        verdict = 'campaign_subset_by_rank_selection'
    else:
        verdict = 'campaign_differs_from_reference'
    return {'observed': 'fresh_campaign_start', 'holders': campaign, 'verdict': verdict}


def compare_incumbent(historical, observations, campaign):
    """Compare a campaign incumbent with a historical lookup; neither overrides the other.

    `historical` is a matrix cell's historical part. `campaign` is
    {'person': id or None, 'name': str or None, 'source': ...}. A divergent
    campaign is an expected outcome and is never reported as an error.
    """
    if campaign is None:
        return 'no_campaign_observation'
    if not campaign.get('person') and not campaign.get('name'):
        return 'campaign_role_vacant'
    if not campaign.get('person'):
        return 'campaign_incumbent_unnamed'
    if historical['status'] not in IDENTIFIED:
        return 'historical_identity_not_established'
    refs = historical.get('holders', [])
    people = {observations[r[0]].get('person') for r in refs if r[0] in observations}
    people.discard(None)
    if not people:
        return 'identity_reconciliation_required'
    return 'campaign_matches_reference' if campaign['person'] in people else 'campaign_diverged_expected'


def build_identity(production, assets, identity, packet, owners, packets, alive_at_start=True):
    research_roles, observations = research_model(identity, packet, owners, packets) if packet else ([], {})
    research_by_key = {r['key']: r for r in research_roles}
    roles = []
    # Production party rows/components, in registry order.
    party_rows = [p for (n, _), p in production.parties.items() if n == identity]
    for party in party_rows:
        for component in [c['id'] for c in party.get('components', [])] or [None]:
            role = production_role(production, identity, party, component)
            for term in party['terms']:
                if term.get('component') != component:
                    continue
                person = production.people.get(term['person'], {})
                grant = production.grants.get((identity, party['party'], term['id']))
                observations[term['id']] = {
                    'role': role['key'], 'person': term['person'], 'name': person.get('name'),
                    'kind': term['kind'], 'term_role': term['role'],
                    'from': term['from'], 'until': term['until'],
                    'affiliation_from': term['affiliation_from'], 'affiliation_until': term['affiliation_until'],
                    'born': person.get('born'), 'died': person.get('died'),
                    'acceptance': 'production_registry',
                    'executive_grant': grant['executive_role'] if grant else None,
                    'appearance_backlog': term['person'] in production.backlog,
                }
            dates = case_dates(production_boundaries(production, party, component))
            cases = []
            for at, kinds in dates:
                historical = production_cell(production, role, party, at)
                # The reference view decorates holders and uncertain entries alike;
                # only holders count as served role coverage.
                listed = {}
                for group, label in (('holders', 'holder'), ('possible', 'possible')):
                    for ref in historical.get(group, []):
                        listed.setdefault(term_person(party, ref[0]), label)
                images = [production.image(pid, at, label) for pid, label in listed.items()]
                cell = {'date': iso(at), 'kinds': sorted(kinds), 'historical': historical,
                        'acceptance': acceptance_of(historical, observations),
                        'image': images if images else 'no_holder',
                        'asset': assets.cell(images) if images else 'n/a'}
                if at == CAMPAIGN_START and alive_at_start:
                    cell['campaign'] = campaign_start_party(production, party, component, historical.get('holders', []))
                if at >= CUTOFF:
                    cell['future'] = future_cell(production, identity, party, role, at)
                cases.append(cell)
            role['cases'] = cases
            roles.append(role)
    # National executive.
    executive = executive_role(production, identity, research_by_key)
    start_person = executive['office_link']['person'] if executive['office_link'] else None
    cases = []
    executive_dates = defaultdict(set)
    for key in executive['paired_research_roles']:
        for when, labels in research_boundaries(research_by_key[key], observations).items():
            executive_dates[when].update(labels)
    if start_person:
        death = bound(production.people.get(start_person, {}).get('died'))
        if death and death[0] == death[1]:
            add_triple(executive_dates, death[0], 'death')
    for at, kinds in case_dates(executive_dates):
        paired = [research_cell(research_by_key[k], observations, at) for k in executive['paired_research_roles']]
        if not paired:
            if at < REFERENCE_FROM or at > CUTOFF:
                historical = {'status': 'inapplicable',
                              'reason': 'before_reference_period' if at < REFERENCE_FROM else 'after_historical_cutoff'}
            else:
                historical = {'status': 'unresearched', 'reason': 'no_research_role_for_executive_office'}
        else:
            historical = combine_research(paired)
        cell = {'date': iso(at), 'kinds': sorted(kinds), 'historical': historical,
                'acceptance': acceptance_of(historical, observations)}
        if start_person and REFERENCE_FROM <= at < FICTIONAL_UNTIL:
            image = production.image(start_person, at)
            image['context'] = 'fresh_campaign_start' if at == CAMPAIGN_START else 'if_original_executive_retained'
            cell['image'] = [image]
            cell['asset'] = assets.cell([image])
        else:
            cell['image'] = 'campaign_dependent' if REFERENCE_FROM <= at < FICTIONAL_UNTIL else 'no_holder'
            cell['asset'] = 'n/a'
        if at == CAMPAIGN_START:
            if executive['start_row']:
                observed = {'person': start_person if executive['office_link_matches_start_row'] else None,
                            'name': executive['start_row']['name']}
                cell['campaign'] = {'observed': 'fresh_campaign_start', 'incumbent': executive['start_row']['name'],
                                    'person': observed['person'],
                                    'verdict': compare_incumbent(historical, observations, observed)}
            else:
                cell['campaign'] = {'observed': 'fresh_campaign_start', 'incumbent': None,
                                    'verdict': 'identity_not_present_at_campaign_start'}
        if FICTIONAL_FROM <= at < FICTIONAL_UNTIL:
            cell['future'] = {'executive_authorized_candidates': executive['future_executive_policy']['executive_authorized_candidates'],
                              'fictional_candidates': executive['future_executive_policy']['fictional_candidates']}
        cases.append(cell)
    executive['cases'] = cases
    roles.insert(0, executive)
    # Research roles.
    for role in research_roles:
        cases = []
        for at, kinds in case_dates(research_boundaries(role, observations)):
            historical = research_cell(role, observations, at)
            cases.append({'date': iso(at), 'kinds': sorted(kinds), 'historical': historical,
                          'acceptance': acceptance_of(historical, observations),
                          'image': 'no_production_identity', 'asset': 'n/a'})
        role['cases'] = cases
        roles.append(role)
    return roles, observations


def person_audit(production, assets, identity, observations, roles):
    """Person-level cartoon windows for everyone the production side binds here."""
    people = sorted({o['person'] for o in observations.values() if o.get('person')}
                    | {r['office_link']['person'] for r in roles if r.get('office_link')})
    rows = []
    for pid in people:
        person = production.people.get(pid, {})
        died = bound(person.get('died'))
        records = production.portraits.get(pid, {}).get('portraits', [])
        windows = []
        for record in records:
            asset = assets.check(record['asset']) if record.get('asset') else None
            windows.append({'from': record.get('from'), 'to': record.get('to'),
                            'asset': record.get('asset'), 'asset_status': asset['status'] if asset else 'no_asset',
                            'extends_past_recorded_death': bool(died and (record.get('to') is None or
                                (exact_day(record['to']) and date.fromisoformat(record['to']) > died[1])))})
        board = production.board_people.get(pid, {})
        rows.append({'person': pid, 'name': person.get('name'), 'born': person.get('born'), 'died': person.get('died'),
                     'portrait_windows': windows,
                     'required_art_windows': board.get('required_art_windows', []),
                     'board_status': board.get('status'),
                     'pending_art_jobs': sorted(j['id'] for j in production.jobs.get(pid, []))})
    fictional = []
    for (nation, party, component), pool in sorted(production.pool.items(), key=lambda kv: (kv[0][0], kv[0][1], kv[0][2] or '')):
        if nation != identity:
            continue
        for candidate in pool:
            records = production.fictional_portraits.get(candidate['person_id'], {}).get('portraits', [])
            # A manifest entry alone is not a served portrait. Check every
            # selector transition, including the ends of overlapping windows.
            samples = {FICTIONAL_FROM}
            for record in records:
                for key in ('from', 'to'):
                    if exact_day(record.get(key)):
                        when = day(record[key])
                        if FICTIONAL_FROM <= when < FICTIONAL_UNTIL:
                            samples.add(when)
            bound_samples = [iso(when) for when in sorted(samples)
                             if production.portrait(candidate['person_id'], when)['binding'] == 'bound']
            fictional.append({'person': candidate['person_id'], 'name': candidate['name'], 'party': party,
                              'component': component,
                              'portrait_assets': [[r.get('asset'), assets.check(r['asset'])['status']] for r in records if r.get('asset')],
                              'bound_portrait_sample_dates': bound_samples,
                              'executive_authorized': bool(candidate['executive_eligibility'].get('authorized'))})
    return rows, fictional


# -------------------------------------------------------------- statistics ----

def role_stats(role):
    historical_samples = [c for c in role['cases'] if 'yearly' in c['kinds'] and c['date'] <= iso(CUTOFF)]
    status = Counter(c['historical']['status'] for c in historical_samples)
    images = Counter()
    for c in historical_samples:
        if isinstance(c['image'], list):
            for i in c['image']:
                if i.get('as', 'holder') == 'holder':
                    images[i['binding']] += 1
    stats = {'cases': len(role['cases']),
             'yearly_historical_samples': len(historical_samples),
             'yearly_status': dict(sorted(status.items())),
             'boundary_cases': sum(1 for c in role['cases'] if any(k.split('_day_')[0] in ('handover', 'death', 'dissolution', 'founding', 'creation', 'institution_end') for k in c['kinds'])),
             'attestation_cases': sum(1 for c in role['cases'] if any(k.startswith('attestation') for k in c['kinds']))}
    if images:
        stats['yearly_image_bindings'] = dict(sorted(images.items()))
    return stats


def identity_summary(production, identity, roles, observations, people, fictional):
    by_family = defaultdict(list)
    for role in roles:
        by_family[role['family']].append(role)
    research = by_family['research_role']
    party = by_family['production_party_leadership']
    executive = by_family['production_national_executive'][0]
    obs_acceptance = Counter(o['acceptance'] for o in observations.values())
    unbound_people = sorted({(i['person'], i.get('art_job', '')) for r in party for c in r['cases']
                             if 'yearly' in c['kinds'] and isinstance(c['image'], list)
                             for i in c['image'] if i['binding'] == 'unbound' and i.get('as') == 'holder'})
    kinds = Counter(k for r in roles for c in r['cases'] for k in c['kinds'])
    stats = {
        'roles': {'research': len(research), 'production_party': len(party), 'executive': 1},
        'cases': sum(len(r['cases']) for r in roles),
        'case_kinds': dict(sorted(kinds.items())),
        'observations_by_acceptance': dict(sorted(obs_acceptance.items())),
        'research_yearly_status': dict(sorted(sum((Counter(role_stats(r)['yearly_status']) for r in research), Counter()).items())),
        'production_party_yearly_status': dict(sorted(sum((Counter(role_stats(r)['yearly_status']) for r in party), Counter()).items())),
        'production_party_yearly_image_bindings': dict(sorted(sum((Counter(role_stats(r).get('yearly_image_bindings', {})) for r in party), Counter()).items())),
        'executive_yearly_status': role_stats(executive)['yearly_status'],
        'people_without_bound_art_at_yearly_samples': len({p for p, _ in unbound_people}),
        'unbound_yearly_samples_without_art_job': sorted({p for p, job in unbound_people if job == 'none_covers_date'}),
        'fictional_candidates': len(fictional),
        'fictional_candidates_with_portrait': sum(1 for f in fictional if f['bound_portrait_sample_dates']),
        'fictional_executive_authorized': sum(f['executive_authorized'] for f in fictional),
        'research_roles_without_holder_observation': sorted(r['role_id'] for r in research if not r['observations']),
        'windows_extending_past_recorded_death': sorted(p['person'] for p in people
                                                         if any(w['extends_past_recorded_death'] for w in p['portrait_windows'])),
        # An unknown end keeps a registry holder "possible" through the cutoff.
        'registry_terms_with_unknown_end': sorted(oid for oid, o in observations.items()
                                                  if o.get('acceptance') == 'production_registry'
                                                  and (o.get('until') or {}).get('kind') == 'unknown'),
    }
    return stats


def headline(identity, stats, roles, production):
    lines = []
    research = [r for r in roles if r['family'] == 'research_role']
    party = [r for r in roles if r['family'] == 'production_party_leadership']
    executive = roles[0]
    if not research:
        lines.append('No C01 office or leadership research role exists for this identity; every historical identity shown comes from the production registry.')
    else:
        with_holders = [r for r in research if r['observations']]
        rs = stats['research_yearly_status']
        total = sum(rs.values())
        identified = sum(rs.get(s, 0) for s in IDENTIFIED)
        lines.append(f"Research roles: {len(research)} ({len(with_holders)} with holder observations); "
                     f"{identified} of {total} yearly role-samples 1990-2026 identify a holder "
                     f"(stated interval, boundary day or exact day attestation); "
                     f"{rs.get('period_attested', 0)} have period observations only, "
                     f"{rs.get('bracketed', 0)} are bracketed, {rs.get('uncertain', 0)} uncertain, "
                     f"{rs.get('unknown', 0)} unknown and {rs.get('unresearched', 0)} unresearched.")
        acc = stats['observations_by_acceptance']
        lines.append('Research holder observations by acceptance: ' +
                     ', '.join(f'{k} {v}' for k, v in acc.items() if k not in ('production_registry',)) + '.')
    if party:
        ps = stats['production_party_yearly_status']
        total = sum(ps.values())
        images = stats['production_party_yearly_image_bindings']
        lines.append(f"Production party rows/components: {len(party)}; {ps.get('established', 0)} of {total} yearly samples 1990-2026 "
                     f"have an established registry holder, {ps.get('uncertain', 0)} uncertain, {ps.get('unknown', 0)} unknown, "
                     f"{ps.get('unresearched', 0)} unresearched and {ps.get('inapplicable', 0)} inapplicable.")
        if images:
            lines.append(f"Registry holder portraits at yearly samples: {images.get('bound', 0)} bound, {images.get('unbound', 0)} unbound "
                         f"({stats['people_without_bound_art_at_yearly_samples']} distinct people without served art).")
    else:
        lines.append('No simulation party rows: no production party leadership and no fictional successor pool exist for this identity.')
    if executive['start_row']:
        start = next(c for c in executive['cases'] if c['date'] == iso(CAMPAIGN_START))
        image = start['image'][0] if isinstance(start['image'], list) else None
        retained = [c for c in executive['cases'] if 'yearly' in c['kinds'] and isinstance(c['image'], list)
                    and c['image'][0]['binding'] == 'bound']
        lines.append(f"Campaign-start executive {executive['start_row']['name']}: portrait "
                     f"{'bound' if image and image['binding'] == 'bound' else 'unbound'} on 1990-01-01; "
                     f"if retained, bound at {len(retained)} of 46 yearly samples.")
        lines.append(f"Campaign-start comparison: {start['campaign']['verdict']}.")
    else:
        lines.append('Successor identity: no executive or party assignment exists at the 1990 campaign start.')
    if not executive['paired_research_roles']:
        lines.append('The national executive office has no paired research role, so its historical chain is unresearched.')
    else:
        es = stats['executive_yearly_status']
        lines.append(f"Paired executive research ({', '.join(executive['paired_research_roles'])}): "
                     f"{sum(es.get(s, 0) for s in IDENTIFIED)} of {sum(v for k, v in es.items() if k != 'inapplicable')} yearly samples 1990-2026 identify a holder.")
    if party:
        lines.append(f"Future: {stats['fictional_candidates']} fictional candidates, {stats['fictional_candidates_with_portrait']} with a served "
                     f"portrait, {stats['fictional_executive_authorized']} authorized for a national executive.")
    if stats['windows_extending_past_recorded_death']:
        lines.append('Portrait window extends past a recorded death: ' + ', '.join(stats['windows_extending_past_recorded_death']) + '.')
    if stats['unbound_yearly_samples_without_art_job']:
        lines.append('Unbound registry holders with no art job covering the sample date: ' +
                     ', '.join(stats['unbound_yearly_samples_without_art_job']) + '.')
    if stats['registry_terms_with_unknown_end']:
        lines.append('Registry terms with an unknown end stay "possible" through the cutoff: ' +
                     ', '.join(stats['registry_terms_with_unknown_end']) + '.')
    return lines


def case_notes(case, identities, roles):
    """Case-level notes that no single identity carries."""
    notes = []
    if len(identities) > 1:
        ends = [r for r in roles if r['family'] == 'research_role' and r['entry']['lifecycle'].get('until')]
        dissolved = [r for r in roles if r['family'] == 'production_party_leadership' and r['lifecycle'].get('dissolved')]
        if not ends and not dissolved:
            notes.append(f"Transition {case}: no structured dissolution, creation or retitling day links these identities in the "
                         'research or the registry, so the transition itself has no day-level boundary case; only stated holder '
                         'and institution boundaries are tested. The successor exists in play only after a campaign creates it.')
    return notes


# ----------------------------------------------------------------- output ----

def canonical(value):
    """Pretty top level; each case compact on one line for readable diffs."""
    placeholders = {}

    def swap(node):
        if isinstance(node, dict):
            if 'key' in node and isinstance(node.get('cases'), list):
                node = dict(node)
                cells = []
                for cell in node['cases']:
                    token = f'@@CASE{len(placeholders)}@@'
                    placeholders[token] = json.dumps(cell, ensure_ascii=False, separators=(',', ':'))
                    cells.append(token)
                node['cases'] = cells
            return {k: swap(v) for k, v in node.items()}
        if isinstance(node, list):
            return [swap(v) for v in node]
        return node

    text = json.dumps(swap(value), ensure_ascii=False, indent=1)
    return re.sub(r'"(@@CASE\d+@@)"', lambda m: placeholders[m.group(1)], text) + '\n'


def build(root=ROOT):
    inputs = Inputs(root)
    census = inputs.json(CENSUS)
    require((census['historical_from'], census['existing_research_cutoff'], census['fictional_from'], census['until_exclusive'])
            == (iso(REFERENCE_FROM), iso(CUTOFF), iso(FICTIONAL_FROM), iso(FICTIONAL_UNTIL)),
            'Census boundary changed; review the frozen cutoff before regenerating')
    countries = {c['id']: c for c in inputs.json(COUNTRIES)}
    cases = []
    for case in census['certified_country_cases']:
        identities = [x.strip() for x in case.split('->')]
        cases.append((case, identities))
    identities = [i for _, ids in cases for i in ids]
    require(identities == census['certified_identity_ids'], 'Certified cases and identity IDs disagree')
    require(all(i in countries for i in identities), 'Certified identity missing from the census roster')
    packets, owners, listed, other_records = packet_provenance(inputs)
    research = {}
    for rel in inputs.glob(RESEARCH, '*.json'):
        packet = inputs.json(rel)
        require(packet.get('research_cutoff') == iso(CUTOFF), f'Research cutoff changed: {rel}')
        require(packet['nation'] not in research, f'Duplicate research identity: {packet["nation"]}')
        research[packet['nation']] = (rel, packet)
    production = Production(inputs)
    for rel in MIRRORED_SEMANTICS + (SELF,):
        inputs.text(rel)
    assets = Assets(inputs, production)
    outputs, summaries = {}, []
    for case, ids in cases:
        doc_roles, doc_obs, identity_rows, people_rows, fictional_rows = [], {}, [], [], []
        for identity in ids:
            rel, packet = research.get(identity, (None, None))
            roles, observations = build_identity(production, assets, identity, packet, owners, packets,
                                                 alive_at_start=countries[identity]['start_1990'])
            people, fictional = person_audit(production, assets, identity, observations, roles)
            stats = identity_summary(production, identity, roles, observations, people, fictional)
            figure = production.figures.get(identity, {})
            identity_rows.append({
                'identity': identity, 'name': countries[identity]['name'],
                'start_1990': countries[identity]['start_1990'],
                'research_packet': rel, 'production_party_rows': sum(1 for n, _ in production.parties if n == identity),
                'national_emblem': {'figure': figure.get('figure'), 'years': figure.get('years'),
                                    'date_bound': False, 'counts_as_role_coverage': False,
                                    'note': 'Selector emblem from nation_figures.json; never an incumbent, never a fallback for person art.'},
                'statistics': stats,
                'headline': headline(identity, stats, roles, production),
            })
            for role in roles:
                role['statistics'] = role_stats(role)
            doc_roles += roles
            doc_obs.update(observations)
            people_rows += people
            fictional_rows += fictional
        name = f'cases-{slug(case)}.json'
        notes = case_notes(case, ids, doc_roles)
        outputs[name] = {
            'format': CASE_FORMAT, 'case': case, 'case_notes': notes, 'identities': identity_rows,
            'interpretation': 'Source-derived observations of checked-in production bindings, using the pinned runtime mirrors; no native execution is performed. Test fixtures live in tools/avatars/test_certified_boundary_matrix.py and are never mixed into this file. A passing checker does not make this coverage pass.',
            'roles': doc_roles, 'observations': dict(sorted(doc_obs.items())),
            'people': people_rows, 'fictional_candidates': fictional_rows,
        }
        summaries.append({'case': case, 'file': name, 'notes': notes, 'identities': [
            {'identity': row['identity'], 'statistics': row['statistics'], 'headline': row['headline']} for row in identity_rows]})
    wrong = sorted(a for a, row in assets.rows.items() if row['status'] != 'available')
    shared = sorted(a for a, people in assets.users.items() if len(people) > 1)
    summary = {
        'format': FORMAT,
        'status': 'preparation_only_not_s23_acceptance',
        's23_complete': False, 'c06_complete': False,
        'interpretation': ('Deterministic S23 preparation audit. It reports coverage gaps; it does not close C01, C06, S20 or S23, '
                           'accept research, certify a country or change any runtime, research or art record. Passing its tests '
                           'does not mean the coverage it reports passes.'),
        'periods': {'historical_from': iso(REFERENCE_FROM), 'historical_through': iso(CUTOFF),
                    'fictional_from': iso(FICTIONAL_FROM), 'fictional_until_exclusive': iso(FICTIONAL_UNTIL),
                    'date_convention': 'Production and portrait windows are half-open [from, to). Research boundary days report both the ending and the starting observation.'},
        'case_dates': {'yearly': [iso(date(y, 1, 1)) for y in YEARS],
                       'fixed': {iso(d): label for d, label in FIXED_DATES},
                       'boundaries': 'Day before, day of and day after every stated handover, death, dissolution, founding or creation day inside the reference period; research attestation days and the day after each.'},
        'historical_status': {
            'boundary_day': 'A source states a start or end on this day; the listed observations begin or end here.',
            'established': 'Inside a closed interval whose start and end are both stated (research) or a production term active under party_leadership::historical_on.',
            'attested': 'A research observation is attested on exactly this day.',
            'period_attested': 'A holder was observed sometime in this period; not an exact-day incumbent or a continuous term. Listed as possible, never counted as identified.',
            'bracketed': 'Between two evidence points of one observation; interpolated, not a stated interval.',
            'uncertain': 'Only one-sided or imprecise bounds overlap the date; possible holders are pointers, not incumbents.',
            'unknown': 'The role has evidence, but none covers this date.',
            'unresearched': 'The role has no holder evidence at all.',
            'inapplicable': 'Outside the reference period or the organization/institution does not exist per a stated boundary.'},
        'acceptance_classes': ACCEPTANCE_CLASSES,
        'packets': [packets[p] for p in sorted(packets)],
        'packets_without_report': sorted(set(listed) - set(packets)),
        'integration_records_not_packet_acceptance': other_records,
        'executive_pairing': {k: list(v) for k, v in EXECUTIVE_PAIRING.items()},
        'executive_pairing_basis': PAIRING_BASIS,
        'census_outputs_consistent_with_registry': production.census_consistent,
        'cases': summaries,
        'assets': [assets.rows[a] for a in sorted(assets.rows)],
        'wrong_or_unavailable_assets': wrong,
        'assets_shared_across_people': shared,
        'campaign_comparison': ('The fresh campaign start is derived from production data and a runtime mirror, not observed through native execution. Divergent saved campaigns are '
                                'exercised by test fixtures and by --campaign against a supplied save; a divergent incumbent is '
                                'an expected outcome, never an error, and history never overwrites it.'),
        'not_used': ['CLAUDE-C01-GAPS-01 ledger (not a dependency)',
                     'docs/campaign-certification/S10/b saved campaigns (not read; no equivalence to start rows is asserted)'],
        'inputs': inputs.listing(),
    }
    outputs['summary.json'] = summary
    outputs['README.md'] = findings_markdown(summary, outputs)
    return outputs


def findings_markdown(summary, outputs):
    lines = ['# S23 preparation: certified-country boundary matrix', '',
             '**Preparation only.** This deterministic audit does not complete S23, C06, S20 or C01, accept any research,',
             'certify a country or change a runtime, research, art or save record. Passing its tests does not mean the',
             'coverage it reports passes; C06 and the final S23 review remain required, and S20 is a separate canonical',
             'dependency that this audit does not evaluate.', '',
             'Generated by `tools/avatars/certified_boundary_matrix.py` from checked-in inputs recorded with bytes and',
             'SHA-256 in [summary.json](summary.json). Text inputs are hashed after CRLF-to-LF normalisation.', '',
             '```text',
             'python -X utf8 tools/avatars/certified_boundary_matrix.py',
             'python -X utf8 tools/avatars/certified_boundary_matrix.py --check',
             'python -X utf8 -m unittest discover -s tools/avatars -p "test_certified_boundary_matrix.py"',
             '```', '',
             '## What each case reports', '',
             'Every case is one role on one date. Its `historical` part gives the identity and evidence strength;',
             '`acceptance` gives the evidence class; `image` mirrors `person_portraits::portrait`; `asset` checks the',
             'bound file (exists, matches the manifest SHA-256, served allowlist). Research roles have no production',
             'identity, so their image is `no_production_identity`. Party rows also report the fictional pool on future',
             'dates. The national executive reports its campaign-start incumbent and, for later dates, the art that',
             'would show if a campaign retained the original executive.', '',
             'Dates: 1 January 1990-2035, 1989-12-31, the 2026-09-06/07/08 cutoff triple, 2030-01-01, 2035-12-31,',
             '2036-01-01, and the day before, of and after every stated handover, death, dissolution, founding or',
             'creation day. Research attestation days are tested with the following day to show that an isolated',
             'attestation never extends.', '',
             '| Status | Meaning |', '|---|---|']
    for key, text in summary['historical_status'].items():
        lines.append(f'| `{key}` | {text} |')
    lines += ['', '| Acceptance | Meaning |', '|---|---|']
    for key, text in summary['acceptance_classes'].items():
        lines.append(f'| `{key}` | {text} |')
    lines += ['', '| Campaign verdict | Meaning |', '|---|---|']
    for key, text in CAMPAIGN_VERDICTS.items():
        lines.append(f'| `{key}` | {text} |')
    lines += ['', 'The committed matrix holds source-derived production observations using pinned runtime mirrors; no native execution is performed. Divergent-campaign behaviour is',
              'asserted on synthetic fixtures in `tools/avatars/test_certified_boundary_matrix.py`, never here.']
    lines += ['', '## Packet classification', '',
              '| Packet | Status | Sources listed |', '|---|---|---:|']
    for packet in summary['packets']:
        lines.append(f"| {packet['packet']} | {packet['status']} | {packet['sources_listed']} |")
    lines += ['', '## Coverage by country', '',
              '| Case | Identity | Roles (research / party / executive) | Cases | Research yearly identified | Party yearly established | Party yearly portraits bound |',
              '|---|---|---|---:|---:|---:|---:|']
    for case in summary['cases']:
        for row in case['identities']:
            s = row['statistics']
            rs, ps, im = s['research_yearly_status'], s['production_party_yearly_status'], s['production_party_yearly_image_bindings']
            research = f"{sum(rs.get(k, 0) for k in IDENTIFIED)}/{sum(rs.values())}" if rs else 'none'
            party = f"{ps.get('established', 0)}/{sum(ps.values())}" if ps else 'none'
            portraits = f"{im.get('bound', 0)}/{sum(im.values())}" if im else 'none'
            lines.append(f"| {case['case']} | {row['identity']} | {s['roles']['research']} / {s['roles']['production_party']} / 1 | "
                         f"{s['cases']} | {research} | {party} | {portraits} |")
    lines += ['', '## Headline findings', '']
    for case in summary['cases']:
        lines.append(f"### {case['case']}")
        lines.append('')
        for note in case['notes']:
            lines.append(f'- {note}')
        if case['notes']:
            lines.append('')
        for row in case['identities']:
            if len(case['identities']) > 1:
                lines.append(f"**{row['identity']}**")
                lines.append('')
            for text in row['headline']:
                lines.append(f'- {text}')
            lines.append('')
    lines += ['## Role coverage', '',
              'Yearly counts use the 37 samples from 1 January 1990 to 1 January 2026: identified (boundary day, established,',
              'or exact day attested) / period-observed / bracketed / uncertain / unknown / unresearched / inapplicable. Portraits count',
              'established holders at those samples; for the executive they count the campaign-start person if retained.', '',
              '| Identity | Role | Evidence | Yearly id/period/br/unc/unk/unr/n.a. | Boundary cases | Attestation cases | Portraits bound |',
              '|---|---|---|---|---:|---:|---|']
    for case in summary['cases']:
        doc = outputs[case['file']]
        for role in doc['roles']:
            s = role['statistics']
            ys = s['yearly_status']
            yearly = '/'.join(str(n) for n in (sum(ys.get(k, 0) for k in IDENTIFIED), ys.get('period_attested', 0), ys.get('bracketed', 0),
                                               ys.get('uncertain', 0), ys.get('unknown', 0),
                                               ys.get('unresearched', 0), ys.get('inapplicable', 0)))
            evidence = Counter(doc['observations'][o]['acceptance'] for o in role['observations'])
            if role['family'] == 'production_national_executive':
                evidence = Counter(doc['observations'][o]['acceptance'] for key in role['paired_research_roles']
                                   for r in doc['roles'] if r['key'] == key for o in r['observations'])
            evidence_text = ', '.join(f'{k} {v}' for k, v in sorted(evidence.items())) or 'none'
            images = s.get('yearly_image_bindings')
            portraits = f"{images.get('bound', 0)}/{sum(images.values())}" if images else '-'
            title = role['title'].replace('|', '/')
            lines.append(f"| {role['identity']} | `{role['key']}` {title} | {evidence_text} | {yearly} | "
                         f"{s['boundary_cases']} | {s['attestation_cases']} | {portraits} |")
    lines.append('')
    unavailable = summary['wrong_or_unavailable_assets']
    lines += ['## Asset checks', '',
              f"{len(summary['assets'])} bound or referenced portrait assets checked; "
              f"{len(unavailable)} not fully available; {len(summary['assets_shared_across_people'])} manifest assets are shared across people.", '']
    for asset in unavailable:
        row = next(a for a in summary['assets'] if a['asset'] == asset)
        lines.append(f"- `{asset}`: {row['status']}")
    if unavailable:
        lines.append('')
    lines += ['## Limitations', '',
              '- Research roles are not reconciled to production party rows or people (their `represented_party_ids` are empty), so research and registry chains are shown side by side, never merged; executive pairing is an audit aid by office title.',
              '- Only structured holder fields are used. Dates that appear only in claim text, notes or linked documents do not create boundaries, and deaths recorded only in text are not death cases.',
              '- Campaign comparison uses the fresh 1990 start derived from production data. Later incumbents depend on play; `--campaign` compares a supplied save without writing the matrix.',
              '- Portrait checks mirror the served selector and file hashes; they are not a visual likeness review.',
              '- Future-pool listings refer to the simulation future reference. The served web historical-reference endpoint rejects dates after the cutoff; a future candidate or image never appoints an incumbent.',
              '- Acceptance classes come from packet reports, numbered packet integration records and the 27 September 2026 integration note; they do not certify dates, people or likenesses. Source-repair and tool records (' +
              (', '.join(summary['integration_records_not_packet_acceptance']) or 'none') + ') do not change a packet\'s class.',
              '']
    return '\n'.join(lines)


# --------------------------------------------------------------- campaign ----

def load_campaign(path, with_provenance=False):
    raw = Path(path).read_bytes()
    provenance = {'path': str(path), 'bytes': len(raw), 'sha256': hashlib.sha256(raw).hexdigest(),
                  'gzip': raw[:2] == b'\x1f\x8b'}
    if raw[:2] == b'\x1f\x8b':
        raw = gzip.decompress(raw)
    provenance.update(decoded_bytes=len(raw), decoded_sha256=hashlib.sha256(raw).hexdigest())
    data = json.loads(raw.decode('utf-8-sig'))
    world = data
    while isinstance(world, dict) and 'world' in world and 'year' not in world:
        world = world['world']
    require(isinstance(world, dict) and 'year' in world, 'Unrecognised campaign save shape')
    return (world, provenance) if with_provenance else world


def campaign_date(world):
    daily = (world.get('rules') or {}).get('daily_simulation')
    return date(world['year'], world['month'], max(world.get('day') or 1, 1) if daily else 1)


def campaign_observation(production, world, identity):
    """Mirror party_leadership::executive_id_with / campaign_view for one identity."""
    rows = [r for r in (world.get('leadership') or []) if r.get('nation') == identity]
    office = rows[0] if rows else None
    book = world.get('party_leadership')
    rules = world.get('rules') or {}
    person = None
    # The enabled runtime checks the saved executive assignment before an
    # office row. In particular, an absent book must not trigger legacy art.
    if rules.get('historical_party_leadership') and book is not None:
        executive = next((e for e in book.get('executives', []) if e['nation'] == identity), None)
        if executive:
            person = executive['holder']['person']
            record = production.people.get(person) or production.candidates.get(person) or {}
            return {'person': person, 'name': (office or {}).get('name') or record.get('name'),
                    'source': 'saved_executive_assignment'}
    if office is None:
        return {'person': None, 'name': None, 'source': 'no_leadership_row'}
    named = office.get('name') is not None and office.get('emergent') is None
    if rules.get('historical_party_leadership'):
        if book is not None and named:
            link = next((o for o in book.get('office_identities', [])
                         if o['nation'] == identity and o['since'] == office.get('since')), None)
            person = link['person'] if link else None
    elif named:
        link = production.links.get(identity)
        if link and link['since'] == office.get('since'):
            record = production.people.get(link['person'], {})
            if office['name'] in (record.get('name'), record.get('native')):
                person = link['person']
    name = office.get('name') or (office.get('emergent') or {}).get('name')
    return {'person': person, 'name': name, 'source': 'saved_leadership_row'}


def campaign_report(root, save_path):
    outputs = build(root)
    world, save_identity = load_campaign(save_path, with_provenance=True)
    at = campaign_date(world)
    inputs = Inputs(root)
    production = Production(inputs)
    report = {'format': 'spheres-s23-saved-boundary-comparison/v1',
              'status': 'read_only_observation_not_campaign_validation',
              'save': str(save_path), 'save_identity': save_identity,
              'campaign_date': iso(at), 'historical_period': outputs['summary.json']['periods'],
              'inputs': outputs['summary.json']['inputs'],
              'interpretation': 'Saved identity observations only: no native loading, replay, integrity validation, historical overwrite or S23 acceptance is performed.',
              'identities': []}
    for name, doc in outputs.items():
        if not name.startswith('cases-'):
            continue
        for identity in doc['identities']:
            ident = identity['identity']
            executive = next(r for r in doc['roles'] if r['identity'] == ident and r['key'] == 'executive')
            research_roles = {r['key']: r for r in doc['roles'] if r['identity'] == ident and r['family'] == 'research_role'}
            paired = [research_cell(research_roles[k], doc['observations'], at) for k in executive['paired_research_roles']]
            if at < REFERENCE_FROM or at > CUTOFF:
                historical = {'status': 'inapplicable', 'reason': 'before_reference_period' if at < REFERENCE_FROM else 'after_historical_cutoff'}
            else:
                historical = combine_research(paired) if paired else {'status': 'unresearched'}
            observed = campaign_observation(production, world, ident)
            parties = []
            book = world.get('party_leadership') or {}
            for assignment in book.get('assignments', []):
                if assignment['nation'] != ident:
                    continue
                key = f"party:{assignment['party']}" + (f"/{assignment['component']}" if assignment.get('component') else '')
                party = production.parties.get((ident, assignment['party']))
                role = next((r for r in doc['roles'] if r['identity'] == ident and r['key'] == key), None)
                cell = production_cell(production, role, party, at) if role and party else {'status': 'unresearched'}
                reference = {term_person(party, h[0]) for h in cell.get('holders', [])} if party else set()
                campaign = [h['person'] for h in assignment.get('holders', [])]
                if not campaign:
                    verdict = 'campaign_role_vacant'
                elif cell['status'] not in IDENTIFIED:
                    verdict = 'historical_identity_not_established'
                elif set(campaign) <= reference:
                    verdict = 'campaign_matches_reference'
                else:
                    verdict = 'campaign_diverged_expected'
                parties.append({'role': key, 'campaign_holders': campaign,
                                'reference_holders': sorted(reference), 'verdict': verdict,
                                'images': [production.image(p, at) for p in campaign]})
            report['identities'].append({
                'identity': ident,
                'executive': {'campaign': observed, 'historical': historical,
                              'verdict': compare_incumbent(historical, doc['observations'], observed),
                              'image': production.image(observed['person'], at) if observed['person'] else None},
                'parties': parties})
    return report


# -------------------------------------------------------------------- cli ----

def write_outputs(outputs, directory):
    directory.mkdir(parents=True, exist_ok=True)
    for name, value in outputs.items():
        text = value if isinstance(value, str) else canonical(value)
        (directory / name).write_text(text, encoding='utf-8', newline='\n')


def check_outputs(outputs, directory):
    problems = []
    for name, value in outputs.items():
        text = value if isinstance(value, str) else canonical(value)
        path = directory / name
        if not path.exists():
            problems.append(f'missing: {name}')
        elif path.read_text(encoding='utf-8') != text:
            problems.append(f'stale: {name}')
    if directory.exists():
        for path in sorted(directory.iterdir()):
            if path.is_file() and path.name not in outputs:
                problems.append(f'unexpected: {path.name}')
    return problems


def main(argv=None):
    parser = argparse.ArgumentParser(description=__doc__.split('\n\n')[0])
    parser.add_argument('--check', action='store_true', help='Fail if the committed matrix differs from current inputs')
    parser.add_argument('--root', type=Path, default=ROOT, help='Repository root (fixture trees in tests)')
    parser.add_argument('--output-dir', type=Path, default=None)
    parser.add_argument('--campaign', type=Path, help='Compare a saved campaign (JSON or gzip) and print a report; writes nothing')
    args = parser.parse_args(argv)
    if args.campaign:
        print(json.dumps(campaign_report(args.root, args.campaign), ensure_ascii=False, indent=1))
        return 0
    directory = args.output_dir or (args.root / OUTPUT)
    outputs = build(args.root)
    if args.check:
        problems = check_outputs(outputs, directory)
        if problems:
            print('Boundary matrix is stale:\n  ' + '\n  '.join(problems), file=sys.stderr)
            return 1
    else:
        write_outputs(outputs, directory)
    counts = {c['case']: sum(i['statistics']['cases'] for i in c['identities']) for c in outputs['summary.json']['cases']}
    print(json.dumps({'check': args.check, 'status': outputs['summary.json']['status'], 'cases': counts,
                      'total_cases': sum(counts.values())}, ensure_ascii=False))
    return 0


if __name__ == '__main__':
    sys.exit(main())
