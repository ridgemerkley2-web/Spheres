"""Validate the E05 important-company research pilot (research preparation only).

Reads docs/research/company-pilot/{sources.json, dossiers/*.json, README.md} and
the game's current identifiers (designer platforms, arsenal kits, ammunition
families, supplier-programme roster, contractor sectors, company orders and the
nations' sourced technology evidence) straight from source text. Nothing is
built, run, installed or written. Exits 1 and lists every violation of:

  duplicate_id  - dossier, entity, claim, product, lineage, source or file IDs
  period        - dates outside 1990-01-01..2026-09-07, a claim newer than its
                  source, or a period endpoint that no claim attests
  identity      - merger/rename confusion: renames across legal entities,
                  counterparties that are the subject itself, names used outside
                  their validity, one business split across dossiers
  reality       - real history mixed with fictional or future products
  provenance    - claims without source/locator, lead or inaccessible sources
                  used as evidence, missing response hashes or byte checks
  mapping       - proposed game targets that do not exist, flows that the
                  company code does not implement, or forbidden shortcuts
  catalog       - the readable catalog omits a dossier or the cutoff
  schema        - malformed records
"""
import argparse
import datetime as dt
import hashlib
import json
import pathlib
import re
import sys

WINDOW_FROM = dt.date(1990, 1, 1)
CUTOFF = dt.date(2026, 9, 7)
FRESH_FROM = dt.date(2025, 9, 7)   # an ongoing period needs evidence this recent
RECHECK_MINUTES = 30
PILOT_NATIONS = {'France': 4, 'Japan': 4}
ACTIVITIES = {'ground', 'aerospace', 'naval', 'civilian_industrial'}
EVIDENCE_KINDS = {'official_company', 'annual_report', 'registration_document', 'securities_filing',
                  'official_register', 'parliament', 'government', 'defence_ministry', 'audit_office',
                  'national_library', 'space_agency'}
LEAD_KINDS = {'news', 'encyclopaedia', 'search_result'}
ENTITY_KINDS = {'company', 'state_administration', 'state_holding', 'holding', 'joint_venture', 'subsidiary'}
SAME_ENTITY_EVENTS = {'rename', 'status_change'}
CROSS_ENTITY_EVENTS = {'business_transfer', 'merger', 'acquisition', 'divestiture', 'joint_venture',
                       'stake_change', 'holding_formation', 'spin_off', 'alliance'}
REALITY = {'historical', 'announced', 'cancelled', 'fictional'}
MILESTONES = {'announced', 'selected', 'contract', 'order', 'export_order', 'development_start', 'adopted',
              'roll_out', 'first_flight', 'first_launch', 'launch', 'sea_trials', 'type_certification',
              'entry_into_service', 'operational', 'first_delivery', 'delivery', 'commissioned',
              'production_milestone', 'market_launch', 'cancelled', 'development_discontinued',
              'terminated', 'transferred'}
COMPLETION = {'entry_into_service', 'operational', 'first_delivery', 'delivery', 'commissioned', 'market_launch'}
ENDINGS = {'cancelled', 'development_discontinued', 'terminated'}
MAPPING_FLOWS = {
    'designer_platform': 'supplier_equipment_flow',
    'ammunition_family': 'supplier_ammunition_flow',
    'supplier_catalogue_roster': 'supplier_programme_slot',
    'sector_contractor_slot': 'sector_contractor_service',
    'arsenal_kit': 'no_company_consumer',
    'tech_node': 'reference_only',
}
CONFIDENCE = {'high', 'medium', 'low'}
FORBIDDEN_SHORTCUTS = ('government_production_capacity', 'free_stock', 'parallel_financial_ledger')
GAME_INPUTS = {
    'platforms': 'spheres-sim/src/equipment.rs',
    'kits': 'spheres-sim/src/arsenal.rs',
    'ammunition': 'spheres-sim/src/equipment_ammunition_production.rs',
    'roster': 'spheres-sim/src/supplier_catalogue.rs',
    'sectors': 'spheres-sim/src/sector_contractors.rs',
    'orders': 'spheres-sim/src/companies.rs',
}
ID_RE = re.compile(r'^[a-z0-9]+(?:_[a-z0-9]+)*$')
SHA_RE = re.compile(r'^[0-9a-f]{64}$')
PDATE_RE = re.compile(r'^(\d{4})(?:-(\d{2})(?:-(\d{2}))?)?$')
MAX_ANCHOR_WORDS = 12


class Problems:
    def __init__(self):
        self.items = []

    def add(self, category, message):
        self.items.append((category, message))


def pdate(value):
    """Partial ISO date -> (first possible day, last possible day, precision)."""
    if not isinstance(value, str):
        return None
    m = PDATE_RE.match(value)
    if not m:
        return None
    y, mo, d = int(m.group(1)), m.group(2), m.group(3)
    try:
        if d:
            day = dt.date(y, int(mo), int(d))
            return day, day, 3
        if mo:
            first = dt.date(y, int(mo), 1)
            nxt = dt.date(y + (int(mo) == 12), int(mo) % 12 + 1, 1)
            return first, nxt - dt.timedelta(days=1), 2
        return dt.date(y, 1, 1), dt.date(y, 12, 31), 1
    except ValueError:
        return None


def utc(value):
    try:
        t = dt.datetime.fromisoformat(str(value).replace('Z', '+00:00'))
    except ValueError:
        return None
    return t if t.tzinfo else None


def within(inner, outer):
    """The attested range lies inside the endpoint's range (evidence at least as precise)."""
    return outer[0] <= inner[0] and inner[1] <= outer[1]


def in_window(p):
    return p[0] >= WINDOW_FROM and p[1] <= CUTOFF


def section(text, start, end_pattern):
    i = text.find(start)
    if i < 0:
        return ''
    m = re.search(end_pattern, text[i:])
    return text[i:i + m.end()] if m else text[i:]


def load_game(root, problems):
    game, hashes = {}, {}
    texts = {}
    for key, rel in GAME_INPUTS.items():
        path = root / rel
        if not path.is_file():
            problems.add('mapping', f'Game input {rel} is missing; mapping targets cannot be verified.')
            texts[key] = ''
            continue
        raw = path.read_bytes().replace(b'\r\n', b'\n')   # LF-normalised: same digest on every checkout
        hashes[rel] = hashlib.sha256(raw).hexdigest()
        texts[key] = raw.decode('utf-8', 'replace')
    game['designer_platform'] = set(re.findall(r'PlatformDef\s*\{\s*id:\s*"([a-z0-9_]+)"', texts['platforms']))
    game['arsenal_kit'] = set(re.findall(r'\bkit\(\s*"([a-z0-9_]+)"', section(texts['kits'], 'pub const DECK', r'\n\];')))
    catalog = section(texts['ammunition'], 'pub const AMMUNITION_CATALOG', r'\n\];')
    game['ammunition_family'] = set(re.findall(r'ammo!\(\s*"([a-z0-9_]+)"', catalog)) | set(
        re.findall(r'\bid:\s*"([a-z0-9_]+)"', catalog))
    roster = section(texts['roster'], 'const ROSTER', r'\];')
    game['supplier_catalogue_roster'] = {f'{n}:{p}' for n, p, _ in re.findall(r'\("([A-Za-z]+)",\s*"([a-z0-9_]+)",\s*(\d+)\)', roster)}
    keys = section(texts['sectors'], 'pub fn key(self)', r'\n    \}')
    game['sectors'] = set(re.findall(r'Self::\w+\s*=>\s*"([a-z]+)"', keys))
    orders = section(texts['orders'], 'pub enum CompanyOrder', r'\n\}')
    game['orders'] = set(re.findall(r'^\s{4}([A-Z][A-Za-z]+)\s*[{,]', orders, re.M))
    for key in ('designer_platform', 'arsenal_kit', 'ammunition_family', 'supplier_catalogue_roster', 'sectors', 'orders'):
        if not game[key] and all((root / rel).is_file() for rel in GAME_INPUTS.values()):
            problems.add('mapping', f'No {key} identifiers could be read from current game source.')
    game['nations'] = {}
    return game, hashes


def nation_tech(root, nation, game, hashes):
    if nation in game['nations']:
        return game['nations'][nation]
    rel = f'spheres-sim/data/nations/{nation.lower()}.json'
    path = root / rel
    techs = None
    if path.is_file():
        raw = path.read_bytes().replace(b'\r\n', b'\n')
        hashes[rel] = hashlib.sha256(raw).hexdigest()
        data = json.loads(raw.decode('utf-8'))
        techs = {t['id']: t.get('source', '') for t in data.get('tech_1990', {}).get('granted', [])}
    game['nations'][nation] = techs
    return techs


def words(text):
    return len(str(text).split())


def check_sources(reg, problems):
    sources, leads, inaccessible = {}, set(), set()
    if reg.get('schema') != 'spheres-company-pilot-sources-v1':
        problems.add('schema', 'sources.json must use schema spheres-company-pilot-sources-v1.')
    if reg.get('research_cutoff') != CUTOFF.isoformat():
        problems.add('period', 'sources.json must record the frozen research cutoff 2026-09-07.')
    seen_urls = {}
    for s in reg.get('sources', []):
        sid = s.get('id', '')
        where = f'source {sid or "?"}'
        if not ID_RE.match(sid):
            problems.add('schema', f'{where}: invalid ID.')
        if sid in sources:
            problems.add('duplicate_id', f'{where}: duplicate source ID.')
        sources[sid] = s
        url = s.get('url', '')
        if not url.startswith('https://') and not url.startswith('http://'):
            problems.add('provenance', f'{where}: missing URL.')
        if url in seen_urls:
            problems.add('duplicate_id', f'{where}: same URL as source {seen_urls[url]}; reuse one record.')
        seen_urls[url] = sid
        if s.get('kind') not in EVIDENCE_KINDS:
            problems.add('provenance', f'{where}: kind {s.get("kind")!r} is not an official evidence kind.')
        for field in ('title', 'publisher', 'rights_note'):
            if not str(s.get(field, '')).strip():
                problems.add('provenance', f'{where}: missing {field}.')
        if 'logo' in json.dumps(s).lower() and 'no logo' not in str(s.get('rights_note', '')).lower():
            problems.add('provenance', f'{where}: logo material must not be recorded as evidence.')
        accessed = pdate(s.get('accessed'))
        covers = pdate(s.get('covers_through'))
        if not accessed or accessed[2] != 3:
            problems.add('provenance', f'{where}: missing exact access date.')
        if not covers:
            problems.add('provenance', f'{where}: missing covers_through date.')
        elif accessed and covers[0] > accessed[1]:
            problems.add('period', f'{where}: covers_through is later than the access date.')
        if s.get('published') is not None and not pdate(s.get('published')):
            problems.add('schema', f'{where}: malformed published date.')
        r = s.get('response') or {}
        fetched = utc(r.get('fetched_utc'))
        if r.get('status') != 200 or not isinstance(r.get('bytes'), int) or r.get('bytes', 0) <= 0:
            problems.add('provenance', f'{where}: response status/bytes missing.')
        if not SHA_RE.match(str(r.get('sha256', ''))):
            problems.add('provenance', f'{where}: response SHA-256 missing.')
        if not fetched:
            problems.add('provenance', f'{where}: response fetched_utc missing.')
        if not r.get('content_type') or not r.get('final_url'):
            problems.add('provenance', f'{where}: response content type or final URL missing.')
        b = s.get('byte_stability') or {}
        status = b.get('status')
        recheck = utc(b.get('recheck_utc'))
        if (not SHA_RE.fullmatch(str(b.get('recheck_sha256', '')))
                or type(b.get('recheck_bytes')) is not int or b.get('recheck_bytes', 0) <= 0):
            problems.add('provenance', f'{where}: recheck identity needs a SHA-256 and a positive integer byte count.')
        if status not in ('byte_stable', 'changed'):
            problems.add('provenance', f'{where}: byte stability must be checked (byte_stable or changed).')
        elif not recheck or not fetched or (recheck - fetched).total_seconds() < RECHECK_MINUTES * 60:
            problems.add('provenance', f'{where}: recheck must follow the first download by {RECHECK_MINUTES}+ minutes.')
        elif status == 'byte_stable' and (b.get('recheck_sha256') != r.get('sha256') or b.get('recheck_bytes') != r.get('bytes')):
            problems.add('provenance', f'{where}: marked byte_stable but the recheck differs.')
        elif status == 'changed':
            if b.get('recheck_sha256') == r.get('sha256'):
                problems.add('provenance', f'{where}: marked changed but the recheck hash is identical.')
            if b.get('anchors_reverified') is not True or not str(b.get('note', '')).strip():
                problems.add('provenance', f'{where}: a changed response needs re-verified anchors and a note.')
    for lead in reg.get('leads', []):
        if lead.get('kind') not in LEAD_KINDS or not lead.get('url'):
            problems.add('provenance', f'lead {lead.get("url")}: leads need a URL and a lead kind.')
        leads.add(lead.get('url'))
    for item in reg.get('inaccessible', []):
        if not item.get('url') or not item.get('result') or not utc(item.get('attempted_utc')):
            problems.add('provenance', f'inaccessible {item.get("url")}: record URL, attempt time and result.')
        inaccessible.add(item.get('url'))
    for sid, s in sources.items():
        if s.get('url') in leads:
            problems.add('provenance', f'source {sid}: a news/encyclopaedia lead cannot be evidence.')
        if s.get('url') in inaccessible:
            problems.add('provenance', f'source {sid}: recorded as inaccessible, so it cannot support claims.')
    return sources


def check_dossier(d, fname, sources, game, hashes, root, problems, ids, used_sources):
    did = d.get('id', '')
    where = f'dossier {did or fname}'
    if d.get('schema') != 'spheres-company-pilot-dossier-v1':
        problems.add('schema', f'{where}: wrong schema.')
    if not ID_RE.match(did):
        problems.add('schema', f'{where}: invalid ID.')
    if fname != f'{did}.json':
        problems.add('duplicate_id', f'{where}: file {fname} must be named after its ID.')
    register(ids, 'dossier', did, where, problems)
    nation = d.get('nation')
    if nation not in PILOT_NATIONS:
        problems.add('schema', f'{where}: nation {nation!r} is outside this pilot.')
    window = d.get('research_window') or {}
    if window.get('from') != WINDOW_FROM.isoformat() or window.get('through') != CUTOFF.isoformat():
        problems.add('period', f'{where}: research window must be 1990-01-01..2026-09-07.')
    acts = set(d.get('activities', []))
    if not acts or not acts <= ACTIVITIES:
        problems.add('schema', f'{where}: activities must be drawn from {sorted(ACTIVITIES)}.')
    fp = d.get('future_policy') or {}
    if fp.get('after') != CUTOFF.isoformat() or fp.get('real_history') != 'not_predicted' or fp.get('later_products') != 'fictional_or_unknown':
        problems.add('reality', f'{where}: future policy must keep post-cutoff history unpredicted and later products fictional or unknown.')

    claims = {}
    for c in d.get('claims', []):
        cid = c.get('id', '')
        cw = f'{where} claim {cid or "?"}'
        register(ids, 'claim', cid, cw, problems)
        claims[cid] = c
        src = sources.get(c.get('source'))
        if not src:
            problems.add('provenance', f'{cw}: source {c.get("source")!r} is not in the registry.')
        else:
            used_sources.add(c.get('source'))
        loc = c.get('locator')
        if not isinstance(loc, dict) or not any(str(v).strip() for v in loc.values()):
            problems.add('provenance', f'{cw}: missing locator (page, section, record or field).')
        if not str(c.get('text', '')).strip():
            problems.add('provenance', f'{cw}: missing claim text.')
        anchor = c.get('anchor')
        if not str(anchor or '').strip():
            problems.add('provenance', f'{cw}: missing locating anchor.')
        elif words(anchor) > MAX_ANCHOR_WORDS:
            problems.add('provenance', f'{cw}: anchor exceeds {MAX_ANCHOR_WORDS} words; paraphrase instead of quoting.')
        attests = c.get('attests') or []
        if not attests:
            problems.add('period', f'{cw}: claim attests no date.')
        before = bool(c.get('context_before_window'))
        for a in attests:
            p = pdate(a)
            if not p:
                problems.add('schema', f'{cw}: malformed date {a!r}.')
                continue
            if before:
                if p[1] >= WINDOW_FROM:
                    problems.add('period', f'{cw}: pre-window context must predate 1990.')
            elif not in_window(p):
                problems.add('period', f'{cw}: {a} is outside the research window 1990-01-01..2026-09-07.')
            if src:
                covers = pdate(src.get('covers_through'))
                if covers and p[0] > covers[1]:
                    problems.add('period', f'{cw}: {a} is not covered by source {src["id"]} (covers through {src.get("covers_through")}).')

    def supported(claim_ids, label, endpoint=None, basis=None, need_fresh=False):
        """Every period element cites claims; endpoints must be attested at matching precision."""
        if not claim_ids:
            problems.add('provenance', f'{label}: cites no claims.')
            return
        cited = []
        for cid in claim_ids:
            if cid not in claims:
                problems.add('provenance', f'{label}: unknown claim {cid!r}.')
            else:
                cited.append(claims[cid])
        in_scope = [(c, pdate(a)) for c in cited for a in c.get('attests', []) if pdate(a)]
        if endpoint is not None:
            e = pdate(endpoint)
            if not e:
                problems.add('schema', f'{label}: malformed date {endpoint!r}.')
            elif basis == 'window_start':
                if endpoint != WINDOW_FROM.isoformat() or not any(
                        c.get('context_before_window') or p[0] <= dt.date(1990, 12, 31) for c, p in in_scope):
                    problems.add('period', f'{label}: a window-start period needs 1990 or pre-1990 context evidence.')
            elif not any(within(p, e) and not c.get('context_before_window') for c, p in in_scope):
                problems.add('period', f'{label}: no cited claim attests {endpoint} at that precision.')
        if need_fresh and not any(p[1] >= FRESH_FROM and not c.get('context_before_window') for c, p in in_scope):
            problems.add('period', f'{label}: an ongoing period needs evidence dated on or after {FRESH_FROM}.')

    def interval(item, label):
        """Validate a [from, until) period, except an inclusive last_observed endpoint.

        An observation is evidence on its recorded day, not a legal end event.
        until None means ongoing at the cutoff. Partial dates retain their precision.
        """
        f = pdate(item.get('from'))
        u = pdate(item['until']) if item.get('until') else None
        if not f or (item.get('until') and not u):
            problems.add('schema', f'{label}: malformed interval.')
            return False
        if not in_window(f) or (u and not in_window(u)):
            problems.add('period', f'{label}: interval must lie within 1990-01-01..2026-09-07.')
        if u and u[1] < f[0]:
            problems.add('period', f'{label}: interval ends before it starts.')
        if item.get('from_basis') not in (None, 'window_start', 'first_observed', 'event'):
            problems.add('schema', f'{label}: unknown from_basis {item.get("from_basis")!r}.')
        if item.get('until_basis') not in (None, 'ended', 'last_observed'):
            problems.add('schema', f'{label}: unknown until_basis {item.get("until_basis")!r}.')
        if item.get('until_basis') and not item.get('until'):
            problems.add('schema', f'{label}: until_basis needs an until date.')
        return True

    entities, name_owner = {}, {}
    for e in d.get('entities', []):
        eid = e.get('id', '')
        ew = f'{where} entity {eid or "?"}'
        register(ids, 'entity', eid, ew, problems)
        entities[eid] = e
        if e.get('kind') not in ENTITY_KINDS:
            problems.add('schema', f'{ew}: unknown kind {e.get("kind")!r}.')
        reg = e.get('registration')
        if reg:
            if not reg.get('register') or not reg.get('number'):
                problems.add('schema', f'{ew}: registration needs register and number.')
            supported(reg.get('claims', []), f'{ew} registration')
        names = e.get('names') or []
        if not names:
            problems.add('identity', f'{ew}: an entity needs at least one sourced name interval.')
        prev_until = None
        for i, n in enumerate(names):
            nw = f'{ew} name {n.get("name")!r}'
            if not interval(n, nw):
                continue
            if i and prev_until is None:
                problems.add('identity', f'{nw}: follows an open-ended name.')
            if prev_until is not None and n.get('from') != prev_until:
                problems.add('identity', f'{nw}: must start exactly where the previous name ended ({prev_until}).')
            prev_until = n.get('until')
            supported(n.get('claims', []), f'{nw} start', n.get('from'), n.get('from_basis'))
            if n.get('until'):
                supported(n.get('claims', []), f'{nw} end', n.get('until'))
            else:
                supported(n.get('claims', []), f'{nw} (ongoing)', need_fresh=True)
            name_owner.setdefault(n.get('name'), set()).add(eid)
        for r in e.get('roles', []):
            rw = f'{ew} role {r.get("role")!r}'
            interval(r, rw)
            supported(r.get('claims', []), f'{rw} start', r.get('from'), r.get('from_basis'))
            if r.get('until'):
                supported(r.get('claims', []), f'{rw} end', r.get('until'))
            else:
                supported(r.get('claims', []), f'{rw} (ongoing)', need_fresh=True)
    for name, owners in name_owner.items():
        if len(owners) > 1:
            problems.add('identity', f'{where}: name {name!r} is attached to several entities {sorted(owners)}; separate identities need separate names.')
    subjects = [e for e in entities.values() if e.get('subject')]
    if len(subjects) != 1:
        problems.add('identity', f'{where}: exactly one entity must be the mapped subject.')

    def names_of(eid):
        return [n for n in (entities.get(eid) or {}).get('names', [])]

    def valid_name(eid, name, when):
        p = pdate(when)
        for n in names_of(eid):
            if n.get('name') != name:
                continue
            f = pdate(n.get('from'))
            u = pdate(n['until']) if n.get('until') else (CUTOFF, CUTOFF, 3)
            inclusive_end = n.get('until') is None or n.get('until_basis') == 'last_observed'
            before_end = bool(u and p and (p[0] <= u[1] if inclusive_end else p[0] < u[1]))
            if f and p and p[1] >= f[0] and before_end:
                return True
        return False

    for ev in d.get('lineage', []):
        lid = ev.get('id', '')
        lw = f'{where} lineage {lid or "?"}'
        register(ids, 'lineage', lid, lw, problems)
        kind, eid, when = ev.get('kind'), ev.get('entity'), ev.get('date')
        p = pdate(when)
        if not p or not in_window(p):
            problems.add('period', f'{lw}: date {when!r} must lie within the research window.')
        supported(ev.get('claims', []), lw, when)
        if eid not in entities:
            problems.add('identity', f'{lw}: unknown entity {eid!r}.')
            continue
        if kind in SAME_ENTITY_EVENTS:
            if ev.get('counterparties'):
                problems.add('identity', f'{lw}: a {kind} keeps one legal entity and cannot have counterparties.')
            old = [n for n in names_of(eid) if n.get('name') == ev.get('from_name')]
            new = [n for n in names_of(eid) if n.get('name') == ev.get('to_name')]
            if not old or not new:
                problems.add('identity', f'{lw}: {kind} must connect two names of the same entity; use a transfer, merger or acquisition between entities.')
            else:
                if old[-1].get('until') != when or new[0].get('from') != when:
                    problems.add('identity', f'{lw}: {kind} date must end the old name and start the new one.')
        elif kind in CROSS_ENTITY_EVENTS:
            if ev.get('from_name') or ev.get('to_name'):
                problems.add('identity', f'{lw}: a {kind} is not a rename; record names on the entities instead.')
            cps = ev.get('counterparties') or []
            if not cps:
                problems.add('identity', f'{lw}: a {kind} needs its counterparties.')
            own = {n.get('name') for n in names_of(eid)}
            for cp in cps:
                if cp.get('name') in own:
                    problems.add('identity', f'{lw}: counterparty {cp.get("name")!r} is a name of the subject entity itself.')
                other = cp.get('entity')
                if other is not None:
                    if other == eid or other not in entities:
                        problems.add('identity', f'{lw}: counterparty entity must be a different recorded entity.')
                    elif own & {n.get('name') for n in names_of(other)}:
                        problems.add('identity', f'{lw}: counterparty entity shares a name with the subject.')
        else:
            problems.add('schema', f'{lw}: unknown lineage kind {kind!r}.')

    for link in d.get('country_links', []):
        cw = f'{where} country link {link.get("country")!r}'
        if not link.get('country') or not link.get('kind'):
            problems.add('schema', f'{cw}: needs country and kind.')
        p = pdate(link.get('date'))
        if not p or not in_window(p):
            problems.add('period', f'{cw}: date {link.get("date")!r} must lie within the research window.')
        supported(link.get('claims', []), cw, link.get('date'))
    for own in d.get('ownership', []):
        ow = f'{where} ownership {own.get("holder")!r}'
        if not own.get('holder') or not own.get('share'):
            problems.add('schema', f'{ow}: needs holder and share.')
        p = pdate(own.get('date'))
        if not p or not in_window(p):
            problems.add('period', f'{ow}: date {own.get("date")!r} must lie within the research window.')
        supported(own.get('claims', []), ow, own.get('date'))

    real_names = set()
    for prod in d.get('products', []):
        pid = prod.get('id', '')
        pw = f'{where} product {pid or "?"}'
        register(ids, 'product', pid, pw, problems)
        reality = prod.get('reality')
        if reality not in REALITY:
            problems.add('reality', f'{pw}: reality must be one of {sorted(REALITY)}.')
            continue
        milestones = prod.get('milestones') or []
        if reality == 'fictional':
            if not prod.get('fictional') or '(fictional)' not in str(prod.get('name', '')).lower():
                problems.add('reality', f'{pw}: a fictional product must be flagged and labelled "(fictional)".')
            if milestones or prod.get('claims') or prod.get('sources'):
                problems.add('reality', f'{pw}: a fictional product cannot cite real sources, claims or milestones.')
            af = pdate(prod.get('available_from')) if prod.get('available_from') else None
            if prod.get('available_from') and (not af or af[0] <= CUTOFF):
                problems.add('reality', f'{pw}: fictional availability must follow the 2026-09-07 cutoff.')
            continue
        if prod.get('fictional'):
            problems.add('reality', f'{pw}: a {reality} product cannot be flagged fictional.')
        if '(fictional)' in str(prod.get('name', '')).lower():
            problems.add('reality', f'{pw}: a real product cannot carry a fictional label.')
        real_names.add(str(prod.get('name', '')).strip().lower())
        if not milestones:
            problems.add('reality', f'{pw}: a real product needs dated, sourced milestones.')
        events = set()
        for m in milestones:
            mw = f'{pw} milestone {m.get("event")!r}'
            ev = m.get('event')
            events.add(ev)
            if ev not in MILESTONES:
                problems.add('schema', f'{mw}: unknown milestone event.')
            p = pdate(m.get('date'))
            if not p or not in_window(p):
                problems.add('period', f'{mw}: date {m.get("date")!r} must lie within 1990-01-01..2026-09-07.')
            supported(m.get('claims', []), mw, m.get('date'))
            maker, name = m.get('maker_entity'), m.get('maker_name')
            if maker not in entities:
                problems.add('identity', f'{mw}: maker entity {maker!r} is not recorded in this dossier.')
            elif p and not valid_name(maker, name, m.get('date')):
                problems.add('identity', f'{mw}: {name!r} was not the name of {maker} on {m.get("date")}.')
            written = m.get('name_as_written')
            if written and written != name and not m.get('retrospective_label'):
                problems.add('identity', f'{mw}: source wording {written!r} differs from the name at the date; mark it retrospective.')
        if reality == 'announced':
            if prod.get('outcome') != 'unknown_after_cutoff':
                problems.add('reality', f'{pw}: an announced product must leave its outcome unknown after the cutoff.')
            if events & COMPLETION:
                problems.add('reality', f'{pw}: an announced product cannot record delivery or service entry.')
        if reality == 'cancelled':
            if not events & ENDINGS:
                problems.add('reality', f'{pw}: a cancelled product needs a sourced cancellation or termination.')
            if events & COMPLETION:
                problems.add('reality', f'{pw}: a cancelled product cannot record delivery or service entry.')
        if reality == 'historical' and events & ENDINGS and not events & COMPLETION:
            problems.add('reality', f'{pw}: an undelivered, ended programme is cancelled, not historical.')

    for prod in d.get('products', []):
        if prod.get('reality') == 'fictional' and str(prod.get('name', '')).lower().replace('(fictional)', '').strip() in real_names:
            problems.add('reality', f'{where} product {prod.get("id")}: a fictional product reuses a real product name.')

    gm = d.get('game_mapping') or {}
    constraints = gm.get('constraints') or {}
    for key in FORBIDDEN_SHORTCUTS:
        if constraints.get(key) is not False:
            problems.add('mapping', f'{where}: constraint {key} must be false; no government capacity, free stock or parallel ledger.')
    if constraints.get('opening_balances_or_stock') != 'not_sourced_not_proposed':
        problems.add('mapping', f'{where}: opening balances or stock must stay unproposed.')
    products = {p.get('id'): p for p in d.get('products', [])}
    proposed = gm.get('proposed') or []
    if not proposed:
        problems.add('mapping', f'{where}: propose at least one existing game target.')
    for m in proposed:
        kind, target = m.get('target_kind'), str(m.get('target_id', ''))
        mw = f'{where} mapping {kind}:{target}'
        if kind not in MAPPING_FLOWS:
            problems.add('mapping', f'{mw}: unknown target kind.')
            continue
        if m.get('flow') != MAPPING_FLOWS[kind]:
            problems.add('mapping', f'{mw}: flow must be {MAPPING_FLOWS[kind]} for this target kind.')
        if m.get('confidence') not in CONFIDENCE or not str(m.get('uncertainty', '')).strip():
            problems.add('mapping', f'{mw}: needs a confidence and an uncertainty flag.')
        if m.get('claims'):
            supported(m['claims'], mw)
        if kind in ('designer_platform', 'supplier_catalogue_roster', 'arsenal_kit', 'ammunition_family'):
            basis = m.get('basis_products') or []
            if not basis or any(b not in products or products[b].get('reality') == 'fictional' for b in basis):
                problems.add('mapping', f'{mw}: must cite real products from this dossier.')
        if kind in ('supplier_catalogue_roster', 'sector_contractor_slot', 'tech_node'):
            code, _, rest = target.partition(':')
            if code != nation:
                problems.add('mapping', f'{mw}: target belongs to {code!r}, not the dossier nation {nation!r}.')
        if kind == 'sector_contractor_slot':
            if target.partition(':')[2] not in game['sectors']:
                problems.add('mapping', f'{mw}: sector is not a current contractor sector.')
        elif kind == 'tech_node':
            techs = nation_tech(root, nation, game, hashes)
            tech = target.partition(':')[2]
            ref = (m.get('existing_reference') or {}).get('match_text', '')
            if techs is None:
                problems.add('mapping', f'{mw}: nation technology evidence is missing.')
            elif tech not in techs:
                problems.add('mapping', f'{mw}: technology is not granted in the nation data.')
            elif not ref or ref not in techs[tech]:
                problems.add('mapping', f'{mw}: existing_reference.match_text must appear in that technology source.')
        elif target not in game.get(kind, set()):
            problems.add('mapping', f'{mw}: target does not exist in current game source.')
    flow = gm.get('integration_flow') or {}
    steps = flow.get('steps') or []
    if not steps:
        problems.add('mapping', f'{where}: describe the integration through existing company orders.')
    for step in steps:
        if step.get('order') not in game['orders']:
            problems.add('mapping', f'{where}: integration step {step.get("order")!r} is not a current CompanyOrder.')
    return d


def register(ids, kind, value, where, problems):
    bucket = ids.setdefault(kind, {})
    if not value:
        problems.add('schema', f'{where}: missing {kind} ID.')
        return
    if not ID_RE.match(value):
        problems.add('schema', f'{where}: {kind} ID {value!r} is not lowercase snake case.')
    if value in bucket:
        problems.add('duplicate_id', f'{where}: duplicate {kind} ID {value!r} (also {bucket[value]}).')
    bucket[value] = where


def cross_checks(dossiers, problems):
    regs, names, slots = {}, {}, {}
    per_nation = {}
    acts = set()
    for d in dossiers:
        per_nation[d.get('nation')] = per_nation.get(d.get('nation'), 0) + 1
        acts |= set(d.get('activities', []))
        for e in d.get('entities', []):
            reg = e.get('registration') or {}
            key = (reg.get('register'), str(reg.get('number', '')).replace(' ', ''))
            if reg.get('number'):
                if key in regs and regs[key] != d.get('id'):
                    problems.add('identity', f'registration {key[0]} {key[1]} appears in dossiers {regs[key]} and {d.get("id")}: one business, one dossier.')
                regs[key] = d.get('id')
            for n in e.get('names', []):
                f = pdate(n.get('from'))
                u = pdate(n['until']) if n.get('until') else (CUTOFF, CUTOFF, 3)
                if not f or not u:
                    continue
                for other_id, of, ou in names.get(n.get('name'), []):
                    if other_id != d.get('id') and f[0] <= ou[1] and of[0] <= u[1]:
                        problems.add('identity', f'name {n.get("name")!r} is used by dossiers {other_id} and {d.get("id")} in overlapping periods.')
                names.setdefault(n.get('name'), []).append((d.get('id'), f, u))
        for m in (d.get('game_mapping') or {}).get('proposed', []):
            if m.get('target_kind') == 'supplier_catalogue_roster':
                target = m.get('target_id')
                if target in slots and slots[target] != d.get('id'):
                    problems.add('mapping', f'supplier programme {target} is proposed for {slots[target]} and {d.get("id")}; one existing programme cannot become two firms.')
                slots[target] = d.get('id')
    for nation, want in PILOT_NATIONS.items():
        if per_nation.get(nation, 0) != want:
            problems.add('schema', f'pilot needs {want} {nation} dossiers, found {per_nation.get(nation, 0)}.')
    if set(per_nation) - set(PILOT_NATIONS):
        problems.add('schema', f'unexpected nations {sorted(set(per_nation) - set(PILOT_NATIONS))}.')
    if acts != ACTIVITIES:
        problems.add('schema', f'the pilot must cover {sorted(ACTIVITIES)}; missing {sorted(ACTIVITIES - acts)}.')


def run(root):
    problems = Problems()
    base = root / 'docs/research/company-pilot'
    try:
        reg = json.loads((base / 'sources.json').read_text(encoding='utf-8'))
    except (OSError, ValueError) as e:
        problems.add('schema', f'sources.json unreadable: {e}')
        reg = {}
    sources = check_sources(reg, problems)
    game, hashes = load_game(root, problems)
    ids, dossiers, used = {}, [], set()
    files = sorted((base / 'dossiers').glob('*.json')) if (base / 'dossiers').is_dir() else []
    if not files:
        problems.add('schema', 'no dossiers found in docs/research/company-pilot/dossiers.')
    for path in files:
        try:
            d = json.loads(path.read_text(encoding='utf-8'))
        except ValueError as e:
            problems.add('schema', f'{path.name}: invalid JSON: {e}')
            continue
        dossiers.append(check_dossier(d, path.name, sources, game, hashes, root, problems, ids, used))
    cross_checks(dossiers, problems)
    for sid in sorted(set(sources) - used):
        problems.add('provenance', f'source {sid} is registered but supports no claim.')
    catalog = base / 'README.md'
    text = catalog.read_text(encoding='utf-8') if catalog.is_file() else ''
    if not text:
        problems.add('catalog', 'docs/research/company-pilot/README.md catalog is missing.')
    else:
        if CUTOFF.isoformat() not in text:
            problems.add('catalog', 'catalog must state the 2026-09-07 research cutoff.')
        for d in dossiers:
            if d.get('id') not in text or d.get('display_name', '\0') not in text:
                problems.add('catalog', f'catalog omits dossier {d.get("id")} ({d.get("display_name")}).')
    stats = {
        'dossiers': len(dossiers), 'sources': len(sources),
        'claims': len(ids.get('claim', {})), 'products': len(ids.get('product', {})),
        'mappings': sum(len((d.get('game_mapping') or {}).get('proposed', [])) for d in dossiers),
        'byte_stable': sum(1 for s in sources.values() if (s.get('byte_stability') or {}).get('status') == 'byte_stable'),
        'changed': sum(1 for s in sources.values() if (s.get('byte_stability') or {}).get('status') == 'changed'),
        'inaccessible': len(reg.get('inaccessible', [])),
        'reality': {k: sum(1 for d in dossiers for p in d.get('products', []) if p.get('reality') == k) for k in sorted(REALITY)},
    }
    return problems, stats, hashes


def main(argv=None):
    parser = argparse.ArgumentParser(description=__doc__, formatter_class=argparse.RawDescriptionHelpFormatter)
    parser.add_argument('--root', default=str(pathlib.Path(__file__).resolve().parents[2]),
                        help='repository root (default: this checkout)')
    args = parser.parse_args(argv)
    problems, stats, hashes = run(pathlib.Path(args.root))
    if problems.items:
        print(f'FAIL: {len(problems.items)} company-pilot violation(s)')
        for category, message in problems.items:
            print(f'- [{category}] {message}')
        return 1
    print(f"PASS: {stats['dossiers']} dossiers (France {PILOT_NATIONS['France']}, Japan {PILOT_NATIONS['Japan']}); "
          f"{stats['sources']} official sources ({stats['byte_stable']} byte-stable, {stats['changed']} changed and re-verified), "
          f"{stats['inaccessible']} inaccessible recorded; {stats['claims']} claims; {stats['products']} products "
          + ', '.join(f'{k} {v}' for k, v in stats['reality'].items())
          + f"; {stats['mappings']} proposed mappings. Research only; no game data changed.")
    for rel, digest in sorted(hashes.items()):
        print(f'  input {rel} sha256 {digest}')
    return 0


if __name__ == '__main__':
    sys.exit(main())
