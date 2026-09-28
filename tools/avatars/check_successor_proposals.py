#!/usr/bin/env python3
"""Validate the CLAUDE-C04-PREP-01 fictional successor proposals (France/Tonga).

Proposal-only preparation for C04. This checker reads the draft dossiers and
their source register, plus current repository contracts (read-only), and exits
non-zero on any violation. It never writes, installs, renders or fetches.

What it enforces, per proposal:
  * explicit fictional labels on the person, name, biography and appearance brief;
  * a stable ``draft_c04_<fr|to>_NN`` ID that cannot collide with an existing
    fictional template, portrait, grant, historical person or term ID;
  * an invented name that does not match or closely resemble the repository's
    real-person corpus (census people, portrait and figure records, C01 research
    holders and research text) or an existing fictional name, and, for Tonga,
    carries no noble title, royal name or honorific;
  * a compatible, non-hereditary role from the packet's role catalog, a game
    party row that exists and is not excluded, and valid C01 cross-references;
  * the role's sourced minimum age and authored plausibility band;
  * a potential eligibility window inside 2026-09-08 .. 2035-12-31 with no
    dated event, narrative year or pre-cutoff office claim (historical-window
    contamination) and no reference to a real person or historical term record;
  * no date-triggered or automatic incumbent replacement: succession only on
    actual gameplay vacancies/elections, incumbents retained, no dated gates;
  * sources that resolve, with bytes/SHA-256 for every response read (matched
    against the fetch log) or an explicit block record.

Usage:
  python -X utf8 tools/avatars/check_successor_proposals.py [--packet DIR]
      [--root DIR] [--json] [--list-inputs]
"""
from __future__ import annotations

import argparse
import datetime as dt
import glob
import hashlib
import json
import pathlib
import re
import sys
import unicodedata

ROOT = pathlib.Path(__file__).resolve().parents[2]
PACKET = pathlib.Path('docs/campaign-certification/C04/preparation/france-tonga')
FORMAT = 'spheres-c04-fictional-successor-proposals/v1'
SOURCES_FORMAT = 'spheres-c04-proposal-sources/v1'
CUTOFF = dt.date(2026, 9, 7)
FROM = dt.date(2026, 9, 8)
UNTIL = dt.date(2035, 12, 31)
UNTIL_EXCLUSIVE = dt.date(2036, 1, 1)

ID_RE = re.compile(r'^draft_c04_(fr|to)_(\d{2})$')
NATION_PREFIX = {'France': 'fr', 'Tonga': 'to'}
CATEGORIES = {'party_leadership', 'executive_eligibility', 'collective_institution', 'hereditary_office'}
GATE_KINDS = {'vacancy', 'election', 'statutory', 'nomination', 'institutional_selection',
              'appointment_by_institution', 'party_process', 'authored_plausibility'}
TRIGGER_GATES = {'vacancy', 'election'}
SOURCE_KINDS = {'primary_institutional', 'official_explanatory', 'party_statute',
                'official_legislative_document', 'institutional_corroboration'}
ACCESS = {'downloaded', 'blocked', 'unreachable', 'not_found'}
STATUS = 'proposal_only_not_installed'
SELECTION = 'actual_succession_events_only'
# Keys that would encode an appointment, accession or replacement on a date.
FORBIDDEN_KEYS = {'takes_office_on', 'assumes_office_on', 'appointed_on', 'appointment_date',
                  'accession_date', 'replacement_date', 'replaces', 'replaces_incumbent',
                  'scheduled_appointment', 'auto_replace', 'automatic_replacement', 'effective_on',
                  'office_start', 'term_start', 'term_from', 'inauguration', 'sworn_in_on',
                  'election_won_on', 'incumbent_replaced', 'succeeds'}
HONORIFICS = {'lord', 'lady', 'baron', 'baroness', 'prince', 'princess', 'king', 'queen', 'hrh',
              'hsh', 'sir', 'dame', 'hon', 'honourable', 'honorable', 'noble', 'count', 'countess',
              'duke', 'duchess', 'marquis', 'chief', 'excellency'}
ROMAN = {'i', 'ii', 'iii', 'iv', 'v', 'vi', 'vii', 'viii', 'ix', 'x'}
LORD_STOP = {'chamberlain', 'chief', 'justice', 'of', 'the', 'speaker', 'president', 'mayor',
             'bishop', 'high', 'privy', 'provost', 'lieutenant', 'protector', 'regent', 'regents',
             'regency', 'crown', 'prince', 'princess', 'consort', 'today', 'will', 'and', 'in'}
# Research files are named <nation>-<topic>.md; map the prefix to the census nation ID.
MD_NATION = {'france': 'France', 'tonga': 'Tonga', 'japan': 'Japan', 'india': 'India', 'brazil': 'Brazil',
             'russia': 'Russia', 'ussr': 'USSR', 'south': 'SouthAfrica', 'saudi': 'SaudiArabia'}
# Office, candidacy and party words forbidden in the undated pre-cutoff background:
# an invented person may have a private life before 2026-09-08, never a public
# office, party role, candidacy or election record.
PRE_CUTOFF_TERMS = [
    'deputy', 'depute', 'deputee', 'minister', 'ministre', 'mayor', 'maire', 'senator', 'senateur',
    'councillor', 'councilor', 'conseiller', 'conseillere', 'representative', 'parliament',
    'parlement', 'prime minister', 'premier ministre', 'president', 'presidente', 'secretary',
    'secretaire', 'party', 'parti', 'elected', 'elu', 'elue', 'election', 'candidate', 'candidat',
    'candidature', 'campaign', 'campagne', 'noble', 'governor', 'speaker', 'cabinet', 'assembly',
    'assemblee', 'militant', 'activist', 'member of', 'membre', 'office holder', 'officeholder',
    'politician', 'government', 'gouvernement']
HEREDITARY_PATTERNS = [r'\bson of\b', r'\bdaughter of\b', r'\bheirs?\b', r'\binherit\w*',
                       r'\bnoble title\b', r'\btitle ?holder\b', r'\bestate holder\b',
                       r'\bhereditary\b', r'\broyal (?:family|blood|descent|lineage|house)\b',
                       r'\bprincess?\b', r'\blord\b', r'\blady\b', r'\btofia\b', r'\bdescendant\b',
                       r'\bnopele\b', r'\bchiefly title\b', r'\bmatapule\b', r'\bnoble family\b',
                       r'\bcrown prince\b', r'\bdynasty\b', r'\bpeerage\b']
YEAR_RE = re.compile(r'(?<!\d)(19\d\d|200\d|201\d|202[0-6])(?!\d)')
ISO_RE = re.compile(r'^\d{4}-\d{2}-\d{2}$')
SHA_RE = re.compile(r'^[0-9a-f]{64}$')
C01_REF_RE = re.compile(r'^c01:([a-z0-9\-]+\.json)#([A-Za-z0-9_\-]+)$')
SRC_REF_RE = re.compile(r'^(src_[a-z0-9_]+)#([a-z0-9_]+)$')


def norm(text) -> str:
    """Accent-, case- and apostrophe-insensitive comparison form."""
    text = unicodedata.normalize('NFKD', str(text))
    text = ''.join(c for c in text if not unicodedata.combining(c))
    text = text.lower()
    text = re.sub(r"[‘’ʻʼ'`´]", '', text)
    text = re.sub(r'[^a-z0-9]+', ' ', text)
    return ' '.join(text.split())


def levenshtein(a: str, b: str) -> int:
    if a == b:
        return 0
    prev = list(range(len(b) + 1))
    for i, ca in enumerate(a, 1):
        cur = [i]
        for j, cb in enumerate(b, 1):
            cur.append(min(prev[j] + 1, cur[j - 1] + 1, prev[j - 1] + (ca != cb)))
        prev = cur
    return prev[-1]


def parse_date(value):
    if not isinstance(value, str) or not ISO_RE.match(value):
        return None
    try:
        return dt.date.fromisoformat(value)
    except ValueError:
        return None


def walk(obj, path='$'):
    """Yield (path, key, value) for every node below obj."""
    if isinstance(obj, dict):
        for key, value in obj.items():
            sub = f'{path}.{key}'
            yield sub, key, value
            yield from walk(value, sub)
    elif isinstance(obj, list):
        for i, value in enumerate(obj):
            sub = f'{path}[{i}]'
            yield sub, None, value
            yield from walk(value, sub)


def strings(obj):
    for _, _, value in walk(obj):
        if isinstance(value, str):
            yield value


class Repo:
    """Read-only view of the current repository contracts and research corpus."""

    def __init__(self, root: pathlib.Path):
        self.root = pathlib.Path(root)
        self.inputs: list[dict] = []
        self.existing_ids: dict[str, str] = {}
        self.historical_ids: set[str] = set()
        self.term_ids: set[str] = set()
        self.fictional_names: set[str] = set()
        self.real_names: dict[str, str] = {}
        self.game_rows: dict[str, set[str]] = {}
        self.name_pools: dict[str, dict] = {}
        self.country_pool: dict[str, str] = {}
        self.c01_ids: dict[str, set[str]] = {}
        self.c01_claims: dict[str, set[str]] = {}
        self.hereditary_tokens: set[str] = set()
        self.nation_tokens: dict[str, set[str]] = {}
        self.corpus = ''
        self._load()

    def _nation_text(self, nation, text):
        if nation:
            self.nation_tokens.setdefault(nation, set()).update(norm(text).split())

    def _read(self, rel):
        data = (self.root / rel).read_bytes()
        self.inputs.append({'path': rel, 'bytes': len(data), 'sha256': hashlib.sha256(data).hexdigest()})
        return data

    def _json(self, rel):
        return json.loads(self._read(rel).decode('utf-8'))

    def _real(self, name, where):
        n = norm(name)
        if len(n.split()) >= 2:
            self.real_names.setdefault(n, where)

    def _load(self):
        cand = self._json('spheres-web/data/future_candidates_2035.json')
        for c in cand['candidates']:
            self.existing_ids[c['person_id']] = 'future_candidates_2035.json'
            self.fictional_names.add(norm(c['name']))
        port = self._json('spheres-web/data/fictional_portraits.json')
        for pid, person in port['people'].items():
            self.existing_ids[pid] = 'fictional_portraits.json'
            self.fictional_names.add(norm(person['name']))
        prod = self._json('spheres-web/data/leadership_production_2035.json')
        for person in prod['people']:
            self.existing_ids[person['id']] = 'leadership_production_2035.json people'
            self.historical_ids.add(person['id'])
            self._real(person['name'], 'leadership_production_2035.json')
        for pid in prod['future'].get('validated_cartoon_assets', {}):
            self.existing_ids[pid] = 'leadership_production_2035.json future'
        for country in prod['countries']:
            rows = self.game_rows.setdefault(country['id'], set())
            for party in country['parties']:
                rows.add(party['id'])
                for term in party.get('terms', []):
                    self.term_ids.add(term['id'])
        seats = self._json('spheres-sim/data/future_party_leadership_seats.json')
        for party in seats['parties']:
            for pid in party['authored_woman_candidates'] + party['authored_man_candidates']:
                self.existing_ids[pid] = 'future_party_leadership_seats.json'
        future = self._json('spheres-sim/data/future_party_leadership.json')
        self.name_pools = future['name_pools']
        self.country_pool = future['country_name_pools']
        elig = self._json('spheres-sim/data/party_executive_eligibility.json')
        for grant in elig['historical_grants'] + elig['future_office_grants']:
            self.existing_ids[grant['person']] = 'party_executive_eligibility.json'
            self.term_ids.add(grant['term'])
        leaders = self._json('spheres-sim/data/party_leaders.json')
        people = {}
        for person in leaders['people']:
            people[person['id']] = person
            self.existing_ids[person['id']] = 'party_leaders.json'
            self.historical_ids.add(person['id'])
            self._real(person['name'], 'party_leaders.json')
            if person.get('native'):
                self._real(person['native'], 'party_leaders.json native')
        for party in leaders['parties']:
            for term in party['terms']:
                self.term_ids.add(term['id'])
                person = people.get(term['person'])
                if person:
                    self._nation_text(party['nation'], f"{person['name']} {person.get('native') or ''}")
        portraits = self._json('spheres-web/data/person_portraits.json')
        for pid, person in portraits['people'].items():
            self.existing_ids[pid] = 'person_portraits.json'
            self.historical_ids.add(pid)
            self._real(person['name'], 'person_portraits.json')
        figures = self._json('spheres-web/data/nation_figures.json')
        for nation_id, nation in figures['nations'].items():
            for key in ('figure', 'canonical_lookup'):
                if nation.get(key):
                    self._real(nation[key], 'nation_figures.json')
                    self._nation_text(nation_id, nation[key])
        roles = self._json('docs/campaign-certification/C01/roles-and-lifecycle.json')
        for record in roles['term_records']:
            self.term_ids.add(record['id'])
        for person in roles['appearance_eligibility_research_backlog']:
            self._real(person['name'], 'roles-and-lifecycle.json backlog')
        texts = []
        research = sorted(glob.glob(str(self.root / 'docs/campaign-certification/C01/research/*.json')))
        for path in research:
            rel = pathlib.Path(path).relative_to(self.root).as_posix()
            packet = self._json(rel)
            fname = pathlib.Path(path).name
            ids = self.c01_ids.setdefault(fname, set())
            claims = self.c01_claims.setdefault(fname, set())
            for src in packet.get('sources', []):
                for claim in src.get('claims', []):
                    claims.add(claim['id'])
            for entity in packet.get('organizations', []) + packet.get('institutions', []):
                ids.add(entity['id'])
                for role in entity.get('roles', []):
                    ids.add(role['id'])
            for p, key, value in walk(packet):
                if key == 'holder_claims' and isinstance(value, list):
                    for holder in value:
                        if isinstance(holder, dict) and holder.get('name'):
                            self._real(holder['name'], f'{fname} holder_claims')
            packet_text = '\n'.join(strings(packet))
            texts.append(packet_text)
            self._nation_text(packet.get('nation'), packet_text)
            if packet.get('nation') == 'Tonga':
                self._tonga_tokens(packet_text)
                for entity in packet.get('institutions', []):
                    for role in entity.get('roles', []):
                        if role.get('kind') == 'head_of_state':
                            for holder in role.get('holder_claims', []):
                                if isinstance(holder, dict) and holder.get('name'):
                                    self._add_hereditary(holder['name'])
        for path in sorted(glob.glob(str(self.root / 'docs/campaign-certification/C01/research/*.md'))):
            rel = pathlib.Path(path).relative_to(self.root).as_posix()
            text = self._read(rel).decode('utf-8')
            texts.append(text)
            self._nation_text(MD_NATION.get(pathlib.Path(path).name.split('-')[0]), text)
            if pathlib.Path(path).name.startswith('tonga'):
                self._tonga_tokens(text)
        self.corpus = ' ' + norm('\n'.join(texts)) + ' '

    def _add_hereditary(self, name):
        for token in norm(name).split():
            if token not in ROMAN and len(token) > 1:
                self.hereditary_tokens.add(token)

    def _tonga_tokens(self, text):
        # Noble titles are written 'Lord <Title>' / 'Lady <Title>' and royal names follow
        # 'Prince', 'Princess', 'HRH' or 'HSH' in the sources. Deliberately conservative:
        # it also guards ordinary names that appear inside royal styles.
        pattern = (r"\b(?:Lord|Lady|Prince|Princess|HRH|HSH)\s+(?:Crown\s+Prince(?:ss)?\s+)?"
                   r"([A-Z‘’'`][\w‘’'`āēīōū\-]+)")
        for m in re.finditer(pattern, text):
            raw = re.sub(r"[’']s$", '', m.group(1))
            token = norm(raw)
            token = token.split()[0] if token else ''
            if token and token not in LORD_STOP and len(token) > 1:
                self.hereditary_tokens.add(token)


def age_bounds(birth_year, start, end):
    """Conservative ages: youngest possible at start, oldest possible at end."""
    return start.year - birth_year - 1, end.year - birth_year


def narrative_texts(proposal):
    bio = proposal.get('biography') or {}
    app = proposal.get('appearance_brief') or {}
    return [('biography.background_before_cutoff', bio.get('background_before_cutoff', '')),
            ('biography.hypothetical_path_after_cutoff', bio.get('hypothetical_path_after_cutoff', '')),
            ('appearance_brief.text', app.get('text', ''))]


def validate(doc, sources, repo: Repo):
    errors: list[tuple[str, str, str]] = []

    def err(code, path, message):
        errors.append((code, path, message))

    # ---------- packet header ----------
    if doc.get('format') != FORMAT:
        err('E_FORMAT', '$.format', f'expected {FORMAT}')
    if doc.get('status') != 'proposal_only_preparation':
        err('E_FORMAT', '$.status', 'packet must be proposal_only_preparation')
    if doc.get('installed') is not False or doc.get('c04_complete') is not False:
        err('E_FORMAT', '$', 'installed and c04_complete must both be false')
    if (doc.get('historical_research_cutoff'), doc.get('fictional_from'),
            doc.get('fictional_until_inclusive'), doc.get('until_exclusive')) != (
            CUTOFF.isoformat(), FROM.isoformat(), UNTIL.isoformat(), UNTIL_EXCLUSIVE.isoformat()):
        err('E_FORMAT', '$', 'cutoff/window constants differ from the frozen C01 boundary')
    contract = doc.get('selection_contract') or {}
    if (contract.get('selection') != SELECTION or contract.get('incumbents_retained') is not True
            or contract.get('date_triggers_allowed') is not False):
        err('E_INCUMBENT', '$.selection_contract',
            'packet contract must be actual_succession_events_only, incumbents retained, no date triggers')

    # ---------- sources ----------
    src_claims = {}
    src_kind = {}
    if sources.get('format') != SOURCES_FORMAT:
        err('E_SOURCE', 'sources.format', f'expected {SOURCES_FORMAT}')
    fetch = {(f.get('url'), f.get('bytes'), f.get('sha256')) for f in sources.get('fetch_log', [])}
    seen_src, seen_claim = set(), set()
    for i, src in enumerate(sources.get('sources', [])):
        p = f'sources[{i}]'
        sid = src.get('id', '')
        if not re.match(r'^src_[a-z0-9_]+$', sid) or sid in seen_src:
            err('E_SOURCE', p, f'missing, malformed or duplicate source id {sid!r}')
        seen_src.add(sid)
        for key in ('title', 'publisher', 'url', 'kind', 'access_status', 'accessed_date'):
            if not src.get(key):
                err('E_SOURCE', f'{p}.{key}', 'required')
        if src.get('kind') not in SOURCE_KINDS:
            err('E_SOURCE', f'{p}.kind', f'unknown kind {src.get("kind")!r}')
        if src.get('access_status') not in ACCESS:
            err('E_SOURCE', f'{p}.access_status', 'unknown access status')
        if parse_date(src.get('accessed_date')) is None:
            err('E_SOURCE', f'{p}.accessed_date', 'must be an ISO date')
        if not str(src.get('url', '')).startswith(('https://', 'http://')):
            err('E_SOURCE', f'{p}.url', 'must be an http(s) URL')
        claims = src.get('claims', [])
        if src.get('access_status') == 'downloaded':
            if not isinstance(src.get('bytes'), int) or src['bytes'] <= 0 or not SHA_RE.match(str(src.get('sha256', ''))):
                err('E_SOURCE', p, 'downloaded source needs positive bytes and a SHA-256 of the response read')
            elif (src.get('url'), src.get('bytes'), src.get('sha256')) not in fetch:
                err('E_SOURCE', p, 'downloaded source is not matched by an entry in fetch_log')
            if src.get('http_status') != 200:
                err('E_SOURCE', p, 'downloaded source must record HTTP 200')
            if src.get('hash_reproducible') not in (True, False, None):
                err('E_SOURCE', p, 'hash_reproducible must be true, false or null')
            if not claims:
                err('E_SOURCE', p, 'downloaded source carries no claim')
        else:
            if claims:
                err('E_SOURCE', p, 'no claim may be drawn from a blocked, unreachable or missing page')
            if not src.get('block_note'):
                err('E_SOURCE', p, 'blocked/unreachable source needs a block_note')
        src_kind[sid] = src.get('kind')
        for j, claim in enumerate(claims):
            cp = f'{p}.claims[{j}]'
            cid = claim.get('id', '')
            if not re.match(r'^[a-z0-9_]+$', cid) or cid in seen_claim:
                err('E_SOURCE', cp, f'missing, malformed or duplicate claim id {cid!r}')
            seen_claim.add(cid)
            if not claim.get('text') or not claim.get('locator'):
                err('E_SOURCE', cp, 'claim needs text and locator')
            src_claims[f'{sid}#{cid}'] = sid

    def check_ref(ref, path, allow_c01=True):
        if isinstance(ref, str) and ref in src_claims:
            return True
        if allow_c01 and isinstance(ref, str):
            m = C01_REF_RE.match(ref)
            if m and (m.group(2) in repo.c01_claims.get(m.group(1), set())
                      or m.group(2) in repo.c01_ids.get(m.group(1), set())):
                return True
        err('E_SOURCE', path, f'unresolved reference {ref!r}')
        return False

    # ---------- role catalog and exclusions ----------
    catalog = {}
    for i, role in enumerate(doc.get('role_catalog', [])):
        p = f'role_catalog[{i}]'
        rid = role.get('role_id')
        if not rid or rid in catalog:
            err('E_ROLE', p, f'missing or duplicate role id {rid!r}')
            continue
        catalog[rid] = role
        if role.get('category') not in CATEGORIES:
            err('E_ROLE', p, 'unknown role category')
        if role.get('nation') not in NATION_PREFIX:
            err('E_ROLE', p, 'unknown role nation')
        if role.get('category') == 'hereditary_office' or role.get('hereditary') is not False:
            if role.get('fictional_proposals_allowed') is not False:
                err('E_ROLE', p, 'hereditary role must never allow fictional proposals')
        rules = role.get('eligibility_rules', [])
        if not rules:
            err('E_SOURCE', p, 'role needs sourced eligibility rules')
        primary = False
        for j, rule in enumerate(rules):
            for k, ref in enumerate(rule.get('source_refs', [])):
                if check_ref(ref, f'{p}.eligibility_rules[{j}].source_refs[{k}]'):
                    sid = src_claims.get(ref)
                    if sid and src_kind.get(sid) in ('primary_institutional', 'party_statute'):
                        primary = True
        if not primary:
            err('E_SOURCE', p, 'role must cite at least one downloaded primary institutional or party-statute claim')
        for k, ref in enumerate(role.get('c01_refs', [])):
            check_ref(ref, f'{p}.c01_refs[{k}]')
        band = role.get('plausibility_band') or {}
        if not isinstance(band.get('min'), int) or not isinstance(band.get('max'), int) or band['min'] >= band['max']:
            err('E_AGE', f'{p}.plausibility_band', 'role needs an integer min<max authored plausibility band')
        lm = role.get('legal_minimum_age')
        if lm is not None and (not isinstance(lm, int) or lm <= 0):
            err('E_AGE', f'{p}.legal_minimum_age', 'must be a positive integer or null')
        if lm is not None and not role.get('legal_minimum_age_refs'):
            err('E_SOURCE', f'{p}.legal_minimum_age_refs', 'a legal minimum age must be sourced')
        for k, ref in enumerate(role.get('legal_minimum_age_refs', [])):
            check_ref(ref, f'{p}.legal_minimum_age_refs[{k}]')
    excluded_roles = {}
    for i, role in enumerate(doc.get('excluded_roles', [])):
        p = f'excluded_roles[{i}]'
        excluded_roles[role.get('role_id')] = role
        if role.get('fictional_proposals_allowed') is not False or not role.get('reason'):
            err('E_ROLE', p, 'excluded role must forbid fictional proposals and give a reason')
        for k, ref in enumerate(role.get('source_refs', [])):
            check_ref(ref, f'{p}.source_refs[{k}]')
        if role.get('role_id') in catalog:
            err('E_ROLE', p, 'role is both catalogued and excluded')
    excluded_rows = {(r.get('nation'), r.get('party')) for r in doc.get('excluded_party_rows', [])}
    for i, row in enumerate(doc.get('excluded_party_rows', [])):
        if not row.get('reason') or not row.get('evidence'):
            err('E_PARTY', f'excluded_party_rows[{i}]', 'excluded row needs a reason and evidence')
    guard = set()
    for i, item in enumerate(doc.get('tonga_hereditary_name_guard', [])):
        for token in norm(item.get('token', '')).split():
            guard.add(token)
        for k, ref in enumerate(item.get('source_refs', [])):
            check_ref(ref, f'tonga_hereditary_name_guard[{i}].source_refs[{k}]')
    hereditary_tokens = repo.hereditary_tokens | guard

    # ---------- proposals ----------
    proposals = doc.get('proposals', [])
    counts = {}
    ids, names = {}, {}
    for i, prop in enumerate(proposals):
        p = f'proposals[{i}]'
        pid = prop.get('draft_id', '')
        nation = prop.get('nation')
        counts[nation] = counts.get(nation, 0) + 1
        # IDs
        m = ID_RE.match(str(pid))
        if not m:
            err('E_ID', f'{p}.draft_id', f'{pid!r} is not draft_c04_<fr|to>_NN')
        elif NATION_PREFIX.get(nation) != m.group(1):
            err('E_ID', f'{p}.draft_id', f'{pid!r} prefix does not match nation {nation!r}')
        if pid in ids:
            err('E_DUP_ID', f'{p}.draft_id', f'{pid!r} duplicates {ids[pid]}')
        ids[pid] = p
        if pid in repo.existing_ids or str(pid).startswith('fictional_v1_'):
            err('E_ID_COLLISION', f'{p}.draft_id',
                f'{pid!r} collides with an existing ID ({repo.existing_ids.get(pid, "fictional_v1_ namespace")})')
        # labels
        if prop.get('fictional') is not True:
            err('E_LABEL', f'{p}.fictional', 'must be true')
        label = str(prop.get('fiction_label', '')).lower()
        if 'fictional' not in label or 'not a real person' not in label:
            err('E_LABEL', f'{p}.fiction_label', 'must say fictional and "not a real person"')
        if prop.get('status') != STATUS:
            err('E_LABEL', f'{p}.status', f'must be {STATUS}')
        name = prop.get('name') or {}
        if name.get('authored') is not True:
            err('E_LABEL', f'{p}.name.authored', 'name must be marked authored')
        bio = prop.get('biography') or {}
        if 'fictional' not in str(bio.get('label', '')).lower():
            err('E_LABEL', f'{p}.biography.label', 'biography must be labelled fictional')
        app = prop.get('appearance_brief') or {}
        if 'fictional' not in str(app.get('label', '')).lower():
            err('E_LABEL', f'{p}.appearance_brief.label', 'appearance brief must be labelled fictional')
        # names
        display, given, family = norm(name.get('display', '')), norm(name.get('given', '')), norm(name.get('family', ''))
        if not given or not family or display != f'{given} {family}':
            err('E_NAME', f'{p}.name', 'display must be given + family')
        else:
            if display in names:
                err('E_NAME_DUP', f'{p}.name', f'name duplicates {names[display]}')
            names[display] = p
            if display in repo.real_names:
                err('E_NAME_REAL', f'{p}.name', f'matches a real person ({repo.real_names[display]})')
            for real, where in repo.real_names.items():
                tokens = real.split()
                if given in tokens and family in tokens:
                    err('E_NAME_REAL', f'{p}.name', f'given and family names both occur in real name {real!r} ({where})')
                elif tokens[-1] == family and levenshtein(tokens[0], given) <= 2:
                    err('E_NAME_REAL', f'{p}.name', f'closely resembles real name {real!r} ({where})')
                elif levenshtein(real, display) <= 2:
                    err('E_NAME_REAL', f'{p}.name', f'within two edits of real name {real!r} ({where})')
            if f' {display} ' in repo.corpus:
                err('E_NAME_REAL', f'{p}.name', 'full name appears in the C01 research corpus')
            if family in repo.nation_tokens.get(nation, set()):
                err('E_NAME_REAL', f'{p}.name',
                    f'family name {family!r} occurs in the {nation} research or roster record; choose one with no recorded bearer')
            if display in repo.fictional_names:
                err('E_NAME_FICTIONAL', f'{p}.name', 'matches an existing fictional template or portrait name')
            pool = repo.name_pools.get(repo.country_pool.get(nation, ''), {})
            given_pool = {norm(n) for n in pool.get('female', []) + pool.get('male', [])}
            if given in given_pool and family in {norm(n) for n in pool.get('surnames', [])}:
                err('E_NAME_FICTIONAL', f'{p}.name', 'combination is producible by the existing template name pool')
            tokens = set(display.split())
            if tokens & HONORIFICS:
                err('E_HEREDITARY_NAME', f'{p}.name', f'honorific in name: {sorted(tokens & HONORIFICS)}')
            if nation == 'Tonga' and tokens & hereditary_tokens:
                err('E_HEREDITARY_NAME', f'{p}.name',
                    f'noble title, royal name or guarded token in name: {sorted(tokens & hereditary_tokens)}')
        # role compatibility
        role_ref = prop.get('role') or {}
        rid = role_ref.get('role_id')
        role = catalog.get(rid)
        if rid in excluded_roles:
            err('E_ROLE', f'{p}.role', f'{rid!r} is an excluded role: {excluded_roles[rid].get("reason", "")}')
        elif role is None:
            err('E_ROLE', f'{p}.role', f'unknown role {rid!r}')
        else:
            if role.get('nation') != nation:
                err('E_ROLE', f'{p}.role', f'role nation {role.get("nation")!r} differs from {nation!r}')
            if (role.get('category') == 'hereditary_office' or role.get('hereditary') is not False
                    or role.get('fictional_proposals_allowed') is not True):
                err('E_ROLE', f'{p}.role', 'role is hereditary or does not allow fictional proposals')
            if role_ref.get('category') != role.get('category'):
                err('E_ROLE', f'{p}.role.category', 'category differs from the role catalog')
        party = (prop.get('party_affiliation') or {}).get('game_party_row')
        rows = repo.game_rows.get(nation, set())
        if role is not None and role.get('game_party_row') is not None and party != role.get('game_party_row'):
            err('E_PARTY', f'{p}.party_affiliation', 'game party row differs from the role catalog')
        if party is not None:
            if party not in rows:
                err('E_PARTY', f'{p}.party_affiliation', f'{party!r} is not a current {nation} game party row')
            if (nation, party) in excluded_rows:
                err('E_PARTY', f'{p}.party_affiliation', f'{party!r} is excluded from invented successors')
        elif not rows and nation is not None:
            pass  # e.g. Tonga: no game rows; mapping is recorded as unresolved
        # age
        age = prop.get('age') or {}
        birth = age.get('birth_year')
        window = prop.get('eligibility_window') or {}
        start, end = parse_date(window.get('earliest_from')), parse_date(window.get('until_inclusive'))
        if not isinstance(birth, int) or isinstance(birth, bool):
            err('E_AGE', f'{p}.age.birth_year', 'integer authored birth year required')
        elif role is not None and start and end:
            youngest, oldest = age_bounds(birth, start, end)
            legal = role.get('legal_minimum_age')
            band = role.get('plausibility_band') or {}
            if legal is not None and youngest < legal:
                err('E_AGE', f'{p}.age', f'may be {youngest} at window start, below the sourced minimum {legal}')
            if isinstance(band.get('min'), int) and youngest < band['min']:
                err('E_AGE', f'{p}.age', f'may be {youngest} at window start, below plausibility minimum {band["min"]}')
            if isinstance(band.get('max'), int) and oldest > band['max']:
                err('E_AGE', f'{p}.age', f'may be {oldest} at window end, above plausibility maximum {band["max"]}')
            if (age.get('youngest_age_at_window_start'), age.get('oldest_age_at_window_end')) != (youngest, oldest):
                err('E_AGE', f'{p}.age', f'declared ages differ from computed ({youngest}, {oldest})')
        # window
        if window.get('kind') != 'potential_eligibility_window':
            err('E_WINDOW', f'{p}.eligibility_window.kind', 'must be potential_eligibility_window')
        if start is None or end is None:
            err('E_WINDOW', f'{p}.eligibility_window', 'earliest_from and until_inclusive must be ISO dates')
        else:
            if start < FROM:
                err('E_HISTORICAL', f'{p}.eligibility_window.earliest_from', f'{start} is before {FROM}')
            if end > UNTIL:
                err('E_WINDOW', f'{p}.eligibility_window.until_inclusive', f'{end} is after {UNTIL}')
            if start > end:
                err('E_WINDOW', f'{p}.eligibility_window', 'window starts after it ends')
        if not window.get('derivation'):
            err('E_WINDOW', f'{p}.eligibility_window.derivation', 'window derivation required')
        # historical-window contamination: every structured date inside the fictional window
        for path, key, value in walk(prop, p):
            d = parse_date(value)
            if isinstance(value, str) and ISO_RE.match(value):
                if d is None:
                    err('E_HISTORICAL', path, f'invalid date {value!r}')
                elif d < FROM:
                    err('E_HISTORICAL', path, f'dated value {value} precedes the fictional window')
                elif d > UNTIL:
                    err('E_WINDOW', path, f'dated value {value} follows the fictional window')
            if key in FORBIDDEN_KEYS:
                err('E_INCUMBENT', path, f'forbidden appointment/replacement key {key!r}')
            if isinstance(value, str) and (value in repo.term_ids or value in repo.historical_ids):
                err('E_OFFICE_RECORD', path, f'refers to a real historical term or person record {value!r}')
        for field, text in narrative_texts(prop):
            if YEAR_RE.search(text or ''):
                err('E_HISTORICAL', f'{p}.{field}', f'narrative contains a pre-window year {YEAR_RE.search(text).group(0)}')
            ntext = ' ' + norm(text or '') + ' '
            for real, where in repo.real_names.items():
                if f' {real} ' in ntext:
                    err('E_REAL_REFERENCE', f'{p}.{field}', f'mentions real person {real!r} ({where})')
        if bio.get('background_undated') is not True or bio.get('background_scope') != 'private_professional_only':
            err('E_HISTORICAL', f'{p}.biography', 'background must be undated and private_professional_only')
        background = ' ' + norm(bio.get('background_before_cutoff', '')) + ' '
        for term in PRE_CUTOFF_TERMS:
            if f' {term} ' in background:
                err('E_HISTORICAL', f'{p}.biography.background_before_cutoff',
                    f'pre-cutoff background claims office, party or candidacy ({term!r})')
        # hereditary claims
        her = prop.get('hereditary_status') or {}
        if (her.get('holds_hereditary_title') is not False or her.get('claims_royal_or_noble_descent') is not False
                or her.get('parentage') != 'not_specified'):
            err('E_HEREDITARY', f'{p}.hereditary_status', 'no title, no royal/noble descent, parentage not_specified')
        for field in ('background_before_cutoff', 'hypothetical_path_after_cutoff'):
            text = norm(bio.get(field, ''))
            for pattern in HEREDITARY_PATTERNS:
                if re.search(pattern, text):
                    err('E_HEREDITARY', f'{p}.biography.{field}', f'hereditary or parentage wording ({pattern})')
        # succession contract: no date-triggered or automatic incumbent replacement
        sc = prop.get('succession_contract') or {}
        if (sc.get('selection') != SELECTION or sc.get('incumbents_retained') is not True
                or sc.get('replaces_incumbent_on_date') is not False or sc.get('date_triggered') is not False
                or sc.get('predicted_appointment') is not False or sc.get('automatic_acting_role') is not False):
            err('E_INCUMBENT', f'{p}.succession_contract',
                'requires actual succession events only, incumbents retained and no date/automatic trigger')
        gates = window.get('gates') or []
        if not gates:
            err('E_INCUMBENT', f'{p}.eligibility_window.gates', 'gates required')
        if not any(g.get('kind') in TRIGGER_GATES for g in gates):
            err('E_INCUMBENT', f'{p}.eligibility_window.gates', 'needs a vacancy or election gate; a date alone never triggers')
        for j, gate in enumerate(gates):
            gp = f'{p}.eligibility_window.gates[{j}]'
            if gate.get('kind') not in GATE_KINDS:
                err('E_INCUMBENT', gp, f'unknown or date-based gate kind {gate.get("kind")!r}')
            for _, _, value in walk(gate):
                if isinstance(value, str) and (ISO_RE.match(value) or re.search(r'\d{4}-\d{2}-\d{2}', value)):
                    err('E_INCUMBENT', gp, 'a gate may not carry a date')
            for k, ref in enumerate(gate.get('source_refs', [])):
                check_ref(ref, f'{gp}.source_refs[{k}]')
        for j, item in enumerate(prop.get('career_constraints', [])):
            if not item.get('text'):
                err('E_ROLE', f'{p}.career_constraints[{j}]', 'constraint text required')
            refs = item.get('source_refs', [])
            if not refs and item.get('basis') != 'authored_plausibility':
                err('E_SOURCE', f'{p}.career_constraints[{j}]', 'constraint needs sources or basis authored_plausibility')
            for k, ref in enumerate(refs):
                check_ref(ref, f'{p}.career_constraints[{j}].source_refs[{k}]')
        # appearance and mapping
        if (app.get('image_generated') is not False or app.get('no_real_likeness') is not True
                or len(str(app.get('text', ''))) < 80):
            err('E_APPEARANCE', f'{p}.appearance_brief', 'text-only brief (>=80 chars), no image, no real likeness')
        for path, key, _ in walk(app, f'{p}.appearance_brief'):
            if key in ('asset', 'image', 'portrait', 'sha256', 'prompt_record'):
                err('E_APPEARANCE', path, 'no artwork may be attached to a proposal')
        mapping = prop.get('contract_mapping') or {}
        if mapping.get('runtime_installed') is not False or mapping.get('status') not in ('unresolved', 'proposed_pending_integration'):
            err('E_MAPPING', f'{p}.contract_mapping', 'mapping must be uninstalled and unresolved/pending')
        for k, ref in enumerate(prop.get('source_refs', [])):
            check_ref(ref, f'{p}.source_refs[{k}]')
    expected = doc.get('expected_counts') or {}
    if counts != expected:
        err('E_COUNT', '$.proposals', f'counts {counts} differ from expected {expected}')
    return errors


def load_packet(root: pathlib.Path, packet: pathlib.Path):
    base = packet if packet.is_absolute() else root / packet
    doc = json.loads((base / 'proposals.json').read_text(encoding='utf-8'))
    sources = json.loads((base / 'sources.json').read_text(encoding='utf-8'))
    return doc, sources


def main(argv=None):
    parser = argparse.ArgumentParser(description=__doc__.splitlines()[0])
    parser.add_argument('--root', type=pathlib.Path, default=ROOT)
    parser.add_argument('--packet', type=pathlib.Path, default=PACKET)
    parser.add_argument('--json', action='store_true', help='print a machine-readable report')
    parser.add_argument('--list-inputs', action='store_true', help='print SHA-256 of every input read')
    args = parser.parse_args(argv)
    try:
        doc, sources = load_packet(args.root, args.packet)
        repo = Repo(args.root)
    except (OSError, ValueError, KeyError) as exc:
        print(f'ERROR: cannot load inputs: {exc}', file=sys.stderr)
        return 2
    errors = validate(doc, sources, repo)
    base = args.packet if args.packet.is_absolute() else args.root / args.packet
    packet_inputs = []
    for fname in ('proposals.json', 'sources.json'):
        data = (base / fname).read_bytes()
        packet_inputs.append({'path': (base / fname).as_posix(), 'bytes': len(data),
                              'sha256': hashlib.sha256(data).hexdigest()})
    if args.json:
        print(json.dumps({'pass': not errors, 'errors': [dict(zip(('code', 'path', 'message'), e)) for e in errors],
                          'proposals': len(doc.get('proposals', [])), 'packet_inputs': packet_inputs,
                          'repository_inputs': repo.inputs}, indent=2, ensure_ascii=False))
    else:
        for code, path, message in errors:
            print(f'{code} {path}: {message}')
        counts = {}
        for prop in doc.get('proposals', []):
            counts[prop.get('nation')] = counts.get(prop.get('nation'), 0) + 1
        if errors:
            print(f'FAIL: {len(errors)} violation(s) in {len(doc.get("proposals", []))} proposals.')
        else:
            summary = ', '.join(f'{k} {v}' for k, v in sorted(counts.items()))
            print(f'PASS: {len(doc["proposals"])} proposal-only fictional drafts ({summary}); '
                  f'{len(sources.get("sources", []))} sources; {len(repo.real_names)} real names and '
                  f'{len(repo.existing_ids)} existing IDs checked; windows within {FROM}..{UNTIL}; '
                  'no hereditary role, date trigger or installation.')
        if args.list_inputs:
            for item in packet_inputs + repo.inputs:
                print(f'input {item["sha256"]} {item["bytes"]:>9} {item["path"]}')
    return 1 if errors else 0


if __name__ == '__main__':
    sys.exit(main())
