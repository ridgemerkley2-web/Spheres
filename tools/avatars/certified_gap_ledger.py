#!/usr/bin/env python3
"""Certified-country research gap ledger (CLAUDE-C01-GAPS-01).

Joins the existing C01 campaign census, the C01 research packets and their
acceptance/integration records for the eight certified cases into one
deterministic ledger of what is represented, what is documented, what is
evidenced (and how securely) and what is still unresearched, followed by
numbered next research batches of at most ten chains each.

This is a gap audit of known, checked-in inputs. It does not discover new
organizations, accept any claim, extend any interval or change a generator,
packet or index. Missing evidence stays a reported gap; an isolated
attestation never becomes an interval. Executive observations never count as
party-leader coverage, and a research organization is never silently mapped
to a simulation party row.

Default mode regenerates docs/campaign-certification/C01/gap-ledger/ from the
pinned inputs; --check compares without writing; --refresh-attribution
rebuilds the pinned source-attribution input from git history (the only mode
that needs git). No network access.
"""
from __future__ import annotations

import argparse
import calendar
import hashlib
import json
import re
import subprocess
import unicodedata
from datetime import date, timedelta
from pathlib import Path

ROOT = Path(__file__).resolve().parents[2]
C01 = Path('docs/campaign-certification/C01')
RESEARCH = C01 / 'research'
OUTPUT = C01 / 'gap-ledger'
ATTRIBUTION = OUTPUT / 'inputs' / 'source-attribution.json'
INTEGRATIONS = C01 / 'integrations'
INTEGRATION_RECORD = Path('docs/campaign-certification/verification/2026-09-27-claude-integration.md')
SOURCE_AUDIT = Path('docs/campaign-certification/verification/evidence/2026-09-27-claude-integration/premerge-source-audit.json')
TASK_QUEUE = Path('docs/planning/ai-task-queue.json')
INPUTS = (C01 / 'census.json', C01 / 'countries.json', C01 / 'represented-organizations.json',
          C01 / 'roles-and-lifecycle.json', C01 / 'work-orders.json', C01 / 'research-index.json',
          INTEGRATION_RECORD, SOURCE_AUDIT, TASK_QUEUE)
PERIOD_FROM = '1990-01-01'
CUTOFF = '2026-09-07'
MAX_BATCH = 10

# Commits that first added research extracts (or first mentioned an extract-less source id), classified by
# packet. Pinned by hash, not by commit message: the Saudi packet's first commit still carries its pre-renumbering
# label "CLAUDE-C01-03". An unknown commit is an error, never a guess.
COMMIT_PACKETS = {
    '233b1320': 'S10', '0f461647': 'S10', '7d85b9f7': 'S10', '99b0b9d4': 'S10', 'db53947a': 'S10',
    '13b41be4': 'S10', '4643bd97': 'S10',
    '8e94a3b7': 'CLAUDE-C01-01', '94fefb21': 'CLAUDE-C01-02', '389f2cf0': 'CLAUDE-C01-03',
    'af01abe7': 'CLAUDE-C01-04', '98402d22': 'CLAUDE-C01-05', 'ffcd27cd': 'CLAUDE-C01-06',
    'e3cacf1c': 'CLAUDE-C01-07', 'd2e44c9b': 'CLAUDE-C01-08', 'c3f40188': 'CLAUDE-C01-09',
    'f71f2f2f': 'CLAUDE-C01-10', 'd57a7bd4': 'CLAUDE-C01-11', '50b9a800': 'CLAUDE-C01-12',
    '68e406b3': 'CLAUDE-C01-13', 'fb0dbc1c': 'CLAUDE-C01-14', '33fd34e8': 'CLAUDE-C01-15',
    '0f1d604b': 'CLAUDE-C01-16', '9c738b95': 'CLAUDE-C01-17', 'd03f4982': 'CLAUDE-C01-18',
    '9417dc93': 'CLAUDE-C01-19', 'f0da27b2': 'CLAUDE-C01-20', '0c841429': 'CLAUDE-C01-21',
    'f8676b0f': 'CLAUDE-C01-22', '8701a28e': 'CLAUDE-C01-26',
}

EVIDENCE_CLASSES = {
    'production_registry': 'Existing simulation registry rows and term records from the C01 census (partial; not a research acceptance).',
    's10_discovery_intake': 'S10 political-atlas discovery intake: qualified bounded discovery observations, not accepted office histories.',
    'c01_accepted': 'C01 research packet accepted and integrated as bounded research by Codex.',
    'c01_integrated_pending': 'C01 research packet merged into integration; historical acceptance pending.',
}

# Claimed or submitted C01 packets and repairs that are not integrated. Their targets are excluded from new batches
# and never labelled accepted. The queue supplies their state; the targets below are taken from their handoffs.
IN_FLIGHT = {
    'CLAUDE-C01-23': {'case': 'France', 'targets': ['institution:fr_presidency'],
                      'scope': 'French presidents, 1990-2026'},
    'CLAUDE-C01-24': {'case': 'Tonga', 'targets': ['role:to_speaker'],
                      'scope': 'Tongan Speakers of the Legislative Assembly, 1990-2026'},
    'CLAUDE-C01-25': {'case': 'SaudiArabia', 'targets': ['role:sa_shura_chair', 'role:sa_succession_chair'],
                      'scope': 'Saudi Shura Council and Allegiance Commission chairs, 1990-2026'},
    'CLAUDE-C01-27': {'case': 'India', 'targets': ['party:India/in_bjp'],
                      'scope': 'Bharatiya Janata Party presidents, 1990-2026'},
    'CLAUDE-C01-28': {'case': 'USSR -> Russia',
                      'targets': ['party:Russia/' + p for p in ('ru_kprf', 'ru_ldpr', 'ru_yabloko', 'ru_apr', 'ru_vybor')],
                      'scope': 'Five Russian party-leader chains; submitted for review, no accepted mapping'},
    'CLAUDE-C01-29': {'case': 'Japan',
                      'targets': ['party:Japan/jp_jsp', 'party:Japan/jp_jsp/jp_jsp_1945', 'party:Japan/jp_jsp/jp_sdp_1996'],
                      'scope': 'Japan Socialist Party / Social Democratic Party chairs; submitted with source review held, no accepted mapping'},
    'CLAUDE-C01-30': {'case': 'SouthAfrica',
                      'targets': ['party:SouthAfrica/' + p for p in ('za_acdp', 'za_ff', 'za_ifp')],
                      'scope': 'ACDP, Freedom Front and IFP party leaders; active research claim, no accepted mapping'},
    'CLAUDE-C01-SOURCE-05': {'case': 'USSR -> Russia', 'targets': [],
                             'scope': 'Source-review repair of one CLAUDE-C01-05 source (no coverage change)'},
    'CLAUDE-C01-SOURCE-06': {'case': 'SaudiArabia', 'targets': [],
                             'scope': 'Source-review repair of one CLAUDE-C01-06 source (no coverage change)'},
    'CLAUDE-C01-SOURCE-17': {'case': 'Brazil', 'targets': [],
                             'scope': 'Source-review repair of the DCN No. 1/2023 records (no coverage change)'},
    'CLAUDE-C01-SOURCE-26': {'case': 'USSR -> Russia', 'targets': [],
                             'scope': 'Source-review repair of CLAUDE-C01-26 (facsimile provenance and defects)'},
}
IN_FLIGHT_ID = re.compile(r'^CLAUDE-C01-(\d\d|SOURCE-\d\d)$')

# A completed source repair accepts only its independently reviewed source identity,
# never the parent historical packet. Additional completion types need an explicit review.
SOURCE_REPAIRS = {
    'CLAUDE-C01-SOURCE-05': {'parent': 'CLAUDE-C01-05', 'nation': 'Russia',
                            'source': 'ru_garf_cec_result_19910619',
                            'snapshot': RESEARCH / 'sources/russia-garf-cec-result-19910619-facts.json'},
    'CLAUDE-C01-SOURCE-06': {'parent': 'CLAUDE-C01-06', 'nation': 'SaudiArabia',
                            'source': 'sa_bush41_address_19900808',
                            'snapshot': RESEARCH / 'sources/saudi-arabia-bush41-address-19900808-facts.json'},
    'CLAUDE-C01-SOURCE-17': {'parent': 'CLAUDE-C01-17', 'nation': 'Brazil',
                            'source_snapshots': {
                                'br_cn_dcn1_20230102_p20_26': 'reviewed-extracts/brazil-congress-dcn-alckmin-diploma-termo-20230102-facts.json',
                                'br_cn_dcn1_20230102_p18_19': 'reviewed-extracts/brazil-congress-dcn-lula-diploma-20230102-facts.json',
                                'br_cn_dcn1_20230102_p1_8': 'reviewed-extracts/brazil-congress-dcn-lula-posse-20230102-facts.json',
                            }},
    'CLAUDE-C01-SOURCE-26': {'parent': 'CLAUDE-C01-26', 'nation': 'USSR',
                            'source_snapshots': {
                                'su_garf_exhibit_law_2392i': 'reviewed-extracts/ussr-garf-law-2392i-19910905-facts.json',
                                'su_garf_exhibit_res_1362i_19900315': 'reviewed-extracts/ussr-garf-res-1362i-19900315-facts.json',
                                'su_ips_cm_res_1177_19901124': 'reviewed-extracts/ussr-ips-cm-res-1177-19901124-facts.json',
                                'su_ips_cm_res_27_19910110': 'reviewed-extracts/ussr-ips-cm-res-27-19910110-facts.json',
                                'su_ips_cm_res_525_19900526': 'reviewed-extracts/ussr-ips-cm-res-525-19900526-facts.json',
                                'su_km_rasp_943r_19910819': 'reviewed-extracts/ussr-ips-km-rasp-943r-19910819-facts.json',
                                'su_kou_post_53_19911123': 'reviewed-extracts/ussr-ips-kou-post-53-19911123-facts.json',
                                'su_kou_rasp_212r_19911219': 'reviewed-extracts/ussr-ips-kou-rasp-212r-19911219-facts.json',
                                'su_kou_rasp_23r_19910904': 'reviewed-extracts/ussr-ips-kou-rasp-23r-19910904-facts.json',
                                'su_kou_rasp_25r_19910906': 'reviewed-extracts/ussr-ips-kou-rasp-25r-19910906-facts.json',
                                'su_kou_rasp_62r_19911005': 'reviewed-extracts/ussr-ips-kou-rasp-62r-19911005-facts.json',
                                'su_mek_rasp_2r_19911010': 'reviewed-extracts/ussr-ips-mek-rasp-2r-19911010-facts.json',
                                'su_mek_rasp_6r_19911112': 'reviewed-extracts/ussr-ips-mek-rasp-6r-19911112-facts.json',
                                'su_mgek_post_7_19911128': 'reviewed-extracts/ussr-ips-mgek-post-7-19911128-facts.json',
                                'su_mgek_rasp_23r_19911217': 'reviewed-extracts/ussr-ips-mgek-rasp-23r-19911217-facts.json',
                                'su_mgek_rasp_7r_19911115': 'reviewed-extracts/ussr-ips-mgek-rasp-7r-19911115-facts.json',
                                'su_rsfsr_res_2017i_19911212': 'reviewed-extracts/ussr-ips-rsfsr-res-2017i-19911212-facts.json',
                                'su_rsfsr_ukaz_299_19911219': 'reviewed-extracts/ussr-ips-rsfsr-ukaz-299-19911219-facts.json',
                                'su_rada_law_1861i_19901226': 'reviewed-extracts/ussr-rada-law-1861i-19901226-facts.json',
                                'su_rada_res_1870i_19901227': 'reviewed-extracts/ussr-rada-res-1870i-19901227-facts.json',
                                'su_snd4_steno_vol3': 'reviewed-extracts/ussr-snd4-stenogram-vol3-19901226-facts.json',
                                'su_snd5_bulletin5_19910904': 'reviewed-extracts/ussr-snd5-bulletin5-19910904-facts.json',
                                'su_sprsfsr_1990_8_art59_19891224': 'reviewed-extracts/ussr-sprsfsr-art59-19891224-facts.json',
                                'su_sprsfsr_1990_8_art60_19900112': 'reviewed-extracts/ussr-sprsfsr-art60-19900112-facts.json',
                                'su_sten_vs_bulletin1_19910826': 'reviewed-extracts/ussr-sten-vs-bulletin1-19910826-facts.json',
                                'su_sten_vs_bulletin2_19910826': 'reviewed-extracts/ussr-sten-vs-bulletin2-19910826-facts.json',
                                'su_ved_1991_35': 'reviewed-extracts/ussr-ved-1991-35-19910828-facts.json',
                                'su_ved_1991_36': 'reviewed-extracts/ussr-ved-1991-36-19910904-facts.json',
                                'su_ved_1991_37': 'reviewed-extracts/ussr-ved-1991-37-19910911-facts.json',
                                'su_ved_1991_41': 'reviewed-extracts/ussr-ved-1991-41-19911009-facts.json',
                            }},
}

# Applicability windows, pinned only where a checked-in claim anchors the boundary. Days outside a window are
# reported as inapplicable to that role, not as missing. Everything else is audited over the full period, and the
# span before a role's first observation is flagged as of unknown applicability.
APPLICABILITY = {
    ('USSR', '*'): {'until': '1991-12-25', 'anchors': ['su_telcon_resignation_decrees_19911225', 'su_almaata_declaration_19911221'],
                    'note': 'Audit boundary for USSR-identity roles at the recorded 25 December 1991 resignation of the USSR President; not an accepted dissolution date.'},
    ('Russia', 'ru_rsfsr_president'): {'from': '1991-04-24', 'until': '1991-12-25',
                                       'anchors': ['ru_rsfsr_president_office_created_19910424', 'ru_law_2094i_rename_19911225'],
                                       'note': 'Office created by the law of 24 April 1991; the state was renamed the Russian Federation on 25 December 1991.'},
    ('Russia', 'ru_rsfsr_vice_president'): {'from': '1991-04-24',
                                            'anchors': ['ru_rsfsr_president_office_created_19910424'],
                                            'note': 'Office created with the RSFSR presidency; its end is not stated in the pinned inputs.'},
    ('Russia', 'ru_president'): {'from': '1991-12-25', 'anchors': ['ru_law_2094i_rename_19911225'],
                                 'note': 'President of the Russian Federation from the renaming of 25 December 1991.'},
    ('SouthAfrica', 'za_state_president'): {'until': '1994-05-10', 'anchors': ['za_mandela_oath_of_office_19940510'],
                                            'note': 'State President under the 1983 Constitution; the first President under the 1993 Constitution took the oath on 10 May 1994.'},
    ('SouthAfrica', 'za_president_election'): {'from': '1994-05-09', 'anchors': ['za_mandela_na_nomination_19940509'],
                                               'note': 'President of the Republic from the National Assembly sitting of 9 May 1994.'},
    ('SouthAfrica', 'za_deputy_president'): {'from': '1994-05-10', 'anchors': ['za_de_klerk_state_president_proclamation_98_19940509'],
                                             'note': 'Deputy President offices from 10 May 1994 (Proclamation No. 98 of 1994).'},
}

# Hereditary offices: a party or electoral succession path does not apply; the holder chain is still researched.
HEREDITARY = {'to_crown': 'Tongan hereditary monarchy', 'sa_crown': 'Saudi dynastic monarchy'}

# Offices named by the census institution-policy records that have no research institution yet.
EXPECTED_OFFICES = {
    'France': [{'office': 'institution:fr_presidency', 'title': 'President of the Republic', 'policy': 'France'},
               {'office': 'institution:fr_prime_minister', 'title': 'Prime Minister', 'policy': 'France'}],
}

STATUS = ('no_evidence', 'attested_within_period', 'attested_day', 'open_end', 'boundary_imprecise',
          'acting_definite', 'definite')
UNRESOLVED = {'no_evidence', 'attested_within_period', 'attested_day', 'open_end', 'boundary_imprecise'}


def read_json(path, root=ROOT):
    return json.loads((root / path).read_text(encoding='utf-8-sig'))


def sha(path, root=ROOT):
    data = (root / path).read_bytes()
    normalization = {}
    if path.suffix == '.md':
        # Review Markdown has no fixed Git checkout EOL. Hash the text the parser reads,
        # with LF endings explicitly recorded; JSON/source evidence remains byte-exact.
        data = data.decode('utf-8').replace('\r\n', '\n').replace('\r', '\n').encode('utf-8')
        normalization['hash_encoding'] = 'UTF-8 text with LF line endings'
    return {'path': path.as_posix(), 'bytes': len(data), 'sha256': hashlib.sha256(data).hexdigest(), **normalization}


def canonical(data):
    return json.dumps(data, ensure_ascii=False, indent=2) + '\n'


def norm(text):
    """Collision key: NFKC, case-folded, letters and digits only (any script)."""
    folded = unicodedata.normalize('NFKC', text or '').casefold()
    stripped = ''.join(ch for ch in unicodedata.normalize('NFKD', folded) if not unicodedata.combining(ch))
    return ''.join(ch for ch in stripped if unicodedata.category(ch)[0] in 'LN')


def strip_parens(text):
    return re.sub(r'\s*\([^)]*\)', '', text or '').strip()


def bounds(value):
    """(earliest, latest) ISO days for a census date object or an ISO day; None when absent/open/unknown."""
    if value is None:
        return None
    if isinstance(value, str):
        date.fromisoformat(value)
        return value, value
    kind, text = value.get('kind'), value.get('value')
    if kind in ('open', 'unknown') or not text:
        return None
    if kind == 'day':
        date.fromisoformat(text)
        return text, text
    if kind == 'month':
        year, month = int(text[:4]), int(text[5:7])
        return f'{text}-01', f'{text}-{calendar.monthrange(year, month)[1]:02d}'
    if kind == 'year':
        return f'{text}-01-01', f'{text}-12-31'
    raise ValueError(f'Unsupported date kind: {kind}')


def precision(value):
    if value is None:
        return 'absent'
    if isinstance(value, str):
        return 'day'
    return value.get('kind') or 'absent'


# ---------------------------------------------------------------- attribution

def git(*args, root=ROOT):
    return subprocess.run(['git', *args], cwd=root, check=True, capture_output=True, text=True,
                          encoding='utf-8').stdout


def refresh_attribution(root=ROOT):
    """Rebuild the pinned attribution from git history (first commit adding each extract or mentioning an id)."""
    added = {}
    current = None
    log = git('log', '--diff-filter=A', '--name-only', '--format=@@%h', '--', (RESEARCH / 'sources').as_posix(), root=root)
    for line in log.splitlines():
        if line.startswith('@@'):
            current = line[2:][:8]
        elif line.strip():
            added[line.strip()] = current  # log is newest first; keep overwriting to reach the first addition
    packets = {}
    for nation, path in certified_packets(root).items():
        packet = read_json(path, root)
        rows = {}
        for source in packet['sources']:
            snapshot = source.get('snapshot', {}).get('path')
            if snapshot and snapshot in added:
                commit, via = added[snapshot], 'extract_first_added'
            else:
                found = git('log', '--reverse', '--format=%h', '-S', f'"id": "{source["id"]}"', '--', path.as_posix(), root=root).split()
                if not found:
                    raise ValueError(f'No commit mentions source {source["id"]}')
                commit, via = found[0][:8], 'source_id_first_seen'
            if commit not in COMMIT_PACKETS:
                raise ValueError(f'Unclassified commit {commit} for source {source["id"]}; classify it in COMMIT_PACKETS')
            rows[source['id']] = {'commit': commit, 'packet': COMMIT_PACKETS[commit], 'via': via}
        packets[nation] = dict(sorted(rows.items()))
    return {'format': 'spheres-c01-gap-source-attribution/v1',
            'generated_from': git('rev-parse', 'HEAD', root=root).strip(),
            'rule': 'Each source is attributed to the commit that first added its extract, or, for extract-less sources, the first commit whose packet file mentions its id. Commits are classified by hash in COMMIT_PACKETS.',
            'sources': packets}


def certified_packets(root=ROOT):
    census = read_json(C01 / 'census.json', root)
    index = read_json(C01 / 'research-index.json', root)
    identities = set(census['certified_identity_ids'])
    return {row['nation']: Path(row['packet']) for row in index['countries'] if row['nation'] in identities}


def acceptance_records(root=ROOT):
    """Accepted packets -> explicit review records; a directory alone is not an acceptance."""
    records = {}
    for entry in sorted((root / INTEGRATIONS).iterdir()):
        match = re.fullmatch(r'CLAUDE-C01-(\d\d(?:-\d\d)*)', entry.name)
        if entry.is_dir() and match:
            record = entry / 'README.md'
            if not record.is_file():
                raise ValueError(f'Missing explicit acceptance record for {entry.name}')
            decision = record.read_text(encoding='utf-8')
            if not re.search(r'^(?:\*\*Decision: accepted\b|Accepted as a bounded research intake\b)', decision, re.M):
                raise ValueError(f'No explicit accepted decision in {record.relative_to(root).as_posix()}')
            for number in match.group(1).split('-'):
                records[f'CLAUDE-C01-{number}'] = record.relative_to(root)
    return records


def acceptance_classes(root=ROOT):
    """Packet -> evidence class, from explicit reviews and the 27 September integration record."""
    classes = {'S10': 's10_discovery_intake'}
    classes.update({packet: 'c01_accepted' for packet in acceptance_records(root)})
    record = (root / INTEGRATION_RECORD).read_text(encoding='utf-8')
    for number in re.findall(r'^\| C01-(\d\d) \| `[0-9a-f]{40}` \|$', record, flags=re.M):
        packet = f'CLAUDE-C01-{number}'
        if classes.get(packet) == 'c01_accepted':
            raise ValueError(f'{packet} is both accepted and pending')
        classes[packet] = 'c01_integrated_pending'
    return classes


def modifications(root=ROOT):
    """Extract path -> later integrated packets that also changed it (from the premerge source audit)."""
    changed = {}
    for row in read_json(SOURCE_AUDIT, root):
        for path in row.get('own_changed_paths', []):
            if '/research/sources/' in path:
                changed.setdefault(path, set()).add(f'CLAUDE-C01-{int(row["packet"]):02d}')
    return {path: sorted(packets) for path, packets in changed.items()}


def completed_source_repair(task, classes, root=ROOT):
    """Require an integrated queue decision plus a scoped accepted review with intact evidence."""
    tid = task['id']
    spec = SOURCE_REPAIRS.get(tid)
    if spec is None:
        raise ValueError(f'Completion of {tid} needs an explicit ledger review rule')
    folder = INTEGRATIONS / tid
    review_path, summary_path = folder / 'manifest.json', folder / 'README.md'
    if not (root / review_path).is_file() or not (root / summary_path).is_file():
        raise ValueError(f'Missing completed source repair review for {tid}')
    review = read_json(review_path, root)
    if (review.get('format') != 'spheres-research-review/v1' or review.get('task') != tid
            or review.get('status') != 'accepted_for_integration' or review.get('reviewer') != 'Codex'
            or not re.fullmatch(r'[0-9a-f]{40}', review.get('reviewed_commit', ''))
            or not review.get('decision') or not review.get('evidence')):
        raise ValueError(f'Completed source repair {tid} lacks a bounded accepted review')
    evidence_paths = []
    for record in review['evidence']:
        relative = Path(record['path'])
        if relative.is_absolute() or '..' in relative.parts:
            raise ValueError(f'Invalid source repair evidence path for {tid}: {relative}')
        path = folder / relative
        data = (root / path).read_bytes()
        if len(data) != record['bytes'] or hashlib.sha256(data).hexdigest() != record['sha256']:
            raise ValueError(f'Source repair evidence changed for {tid}: {relative}')
        evidence_paths.append(path)
    if 'source_snapshots' in spec:
        sources = list(spec['source_snapshots'])
        snapshots = [folder / path for path in spec['source_snapshots'].values()]
        if any(path not in evidence_paths for path in snapshots):
            raise ValueError(f'Source repair {tid} lacks reviewed extract evidence')
    else:
        sources, snapshots = [spec['source']], [spec['snapshot']]
    inputs = list(dict.fromkeys([review_path, summary_path, *snapshots, *evidence_paths]))
    return {'task': tid, 'state': 'complete', 'case': IN_FLIGHT[tid]['case'],
            'source': ', '.join(sources), 'source_ids': sources,
            'nation': spec['nation'], 'scope': review['decision'],
            'reviewed_commit': review['reviewed_commit'],
            'parent_packet': spec['parent'], 'parent_evidence_class': classes[spec['parent']],
            'parent_acceptance_changed': False, 'historical_coverage_changed': False,
            'evidence': [sha(path, root) for path in inputs]}


# ---------------------------------------------------------------- coverage

def ordinal(text):
    return date.fromisoformat(text).toordinal()


def iso(number):
    return date.fromordinal(number).isoformat()


def window_for(nation, role_id):
    rule = APPLICABILITY.get((nation, role_id)) or APPLICABILITY.get((nation, '*')) or {}
    return {'from': max(rule.get('from', PERIOD_FROM), PERIOD_FROM), 'until': min(rule.get('until', CUTOFF), CUTOFF),
            'pinned': bool(rule), 'anchors': rule.get('anchors', []), 'note': rule.get('note')}


def chain_window(row, disclosure):
    """Party-chain window: the case window, narrowed only by the census's own founded/dissolved disclosures."""
    window = window_for(row['nation'], row['id'])
    if not disclosure:
        return window
    found = bounds(disclosure['bounds'].get('founded'))
    ended = bounds(disclosure['bounds'].get('dissolved'))
    anchors = list(window['anchors'])
    if found and found[0] > window['from']:
        window['from'] = found[0]
        anchors.append(f'census lifecycle disclosure: founded {found[0]}')
    if ended and ended[1] < window['until']:
        window['until'] = ended[1]
        anchors.append(f'census lifecycle disclosure: dissolved by {ended[1]}')
    if window['from'] > window['until']:
        window['until'] = window['from']
    window['anchors'] = anchors
    window['pinned'] = window['pinned'] or bool(anchors)
    return window


def coverage(entries, window):
    """Day statuses over the window from normalized evidence; returns segments and day counts."""
    lo, hi = ordinal(window['from']), ordinal(window['until'])
    days = [0] * (hi - lo + 1)

    def paint(start, end, code):
        start, end = max(start, lo), min(end, hi)
        for number in range(start, end + 1):
            if code > days[number - lo]:
                days[number - lo] = code

    code = {name: index for index, name in enumerate(STATUS)}
    for entry in entries:
        definite = code['acting_definite'] if entry['kind'] == 'acting' else code['definite']
        start, end = entry['from'], entry['until']
        if start and end:
            first, last = ordinal(start[1]), ordinal(end[0])
            if first <= last:
                paint(first, last, definite)
                paint(ordinal(start[0]), first - 1, code['boundary_imprecise'])
                paint(last + 1, ordinal(end[1]), code['boundary_imprecise'])
            else:
                paint(ordinal(start[0]), ordinal(end[1]), code['boundary_imprecise'])
        elif start:
            paint(ordinal(start[0]), ordinal(start[1]) - 1, code['boundary_imprecise'])
            paint(ordinal(start[1]), hi, code['open_end'])
        elif end:
            paint(ordinal(end[0]), ordinal(end[1]), code['attested_day'])
        for point in entry['points']:
            paint(ordinal(point), ordinal(point), code['attested_day'])
        for first, last in entry['periods']:
            paint(ordinal(first), ordinal(last), code['attested_within_period'])
    segments, counts = [], {name: 0 for name in STATUS}
    for offset, value in enumerate(days):
        name = STATUS[value]
        counts[name] += 1
        if segments and segments[-1]['status'] == name:
            segments[-1]['through'] = iso(lo + offset)
        else:
            segments.append({'from': iso(lo + offset), 'through': iso(lo + offset), 'status': name})
    first_evidence = next((s['from'] for s in segments if s['status'] != 'no_evidence'), None)
    return {'segments': segments, 'days': counts, 'window_days': len(days),
            'unresolved_days': sum(counts[name] for name in UNRESOLVED), 'first_evidence': first_evidence}


def missing_fields(entry):
    fields = []
    if entry['from'] is None:
        fields.append('from')
    elif entry['from_precision'] != 'day':
        fields.append(f'from (known only to {entry["from_precision"]})')
    if entry['until'] is None:
        fields.append('until' if entry['until_precision'] != 'open' else 'until (open in registry; continuity to cutoff unconfirmed)')
    elif entry['until_precision'] != 'day':
        fields.append(f'until (known only to {entry["until_precision"]})')
    return fields


# ---------------------------------------------------------------- ledger

def build(root=ROOT, attribution=None):
    census = read_json(C01 / 'census.json', root)
    if census['existing_research_cutoff'] != CUTOFF or census['historical_from'] != PERIOD_FROM:
        raise ValueError('Census period changed; review the ledger contract before regenerating')
    countries = {c['id']: c for c in read_json(C01 / 'countries.json', root)}
    represented = read_json(C01 / 'represented-organizations.json', root)
    roles = read_json(C01 / 'roles-and-lifecycle.json', root)
    work_orders = read_json(C01 / 'work-orders.json', root)
    index = read_json(C01 / 'research-index.json', root)
    queue = read_json(TASK_QUEUE, root)
    attribution = attribution if attribution is not None else read_json(ATTRIBUTION, root)
    classes = acceptance_classes(root)
    changed = modifications(root)
    packets = certified_packets(root)

    cases = []
    for case in census['certified_country_cases']:
        identities = [part.strip() for part in case.split('->')]
        missing = [i for i in identities if i not in census['certified_identity_ids']]
        if missing:
            raise ValueError(f'Case {case} names unknown identities {missing}')
        cases.append((case, identities))
    if len(cases) != 8:
        raise ValueError('Expected eight certified cases')

    queue_claude = {t['id']: t for t in queue['tasks'] if t.get('owner') == 'Claude' and IN_FLIGHT_ID.match(t['id'])}
    active = {tid: t for tid, t in queue_claude.items() if t.get('state') in ('claimed', 'ready_for_review')}
    completed = {tid: t for tid, t in queue_claude.items() if tid in IN_FLIGHT and t.get('state') == 'complete'}
    if set(active) | set(completed) != set(IN_FLIGHT):
        raise ValueError(f'In-flight table differs from the task queue: queue={sorted(active)} table={sorted(IN_FLIGHT)}')
    completed_repairs = [completed_source_repair(completed[tid], classes, root) for tid in sorted(completed)]
    in_flight = [{'task': tid, 'state': active[tid]['state'], 'branch': active[tid].get('branch'),
                  'case': IN_FLIGHT[tid]['case'], 'scope': IN_FLIGHT[tid]['scope'], 'targets': IN_FLIGHT[tid]['targets'],
                  'accepted': False} for tid in sorted(active)]
    flight_targets = {target: tid for tid in active for target in IN_FLIGHT[tid]['targets']}

    terms_by_org = {}
    for term in roles['term_records']:
        terms_by_org.setdefault(term['organization_id'], []).append(term)
    orders_by_member = {}
    for order in list(work_orders) + list(index.get('work_orders', [])):
        for member in order.get('members', []):
            if isinstance(member, str):  # cartoon-job members are person/job objects, not organization ids
                orders_by_member.setdefault(member, []).append(order['id'])

    ledger_cases, all_batches = [], []
    for case, identities in cases:
        result = case_ledger(root, case, identities, countries, represented, roles, terms_by_org, orders_by_member,
                             packets, attribution, classes, changed, flight_targets)
        ledger_cases.append(result['case'])
        all_batches.extend(result['batches'])

    totals = {'cases': len(ledger_cases), 'represented_rows': sum(c['totals']['represented_rows'] for c in ledger_cases),
              'party_chains_with_unresolved_days': sum(c['totals']['party_chains_unresolved'] for c in ledger_cases),
              'research_roles': sum(c['totals']['research_roles'] for c in ledger_cases),
              'research_roles_with_unresolved_days': sum(c['totals']['research_roles_unresolved'] for c in ledger_cases),
              'next_batches': len(all_batches), 'next_items': sum(len(b['items']) for b in all_batches)}
    inputs = ([sha(path, root) for path in INPUTS]
              + [sha(path, root) for path in sorted(set(packets.values()))]
              + [sha(path, root) for path in sorted(set(acceptance_records(root).values()))]
              + [e for repair in completed_repairs for e in repair['evidence']])
    return {
        'format': 'spheres-c01-certified-gap-ledger/v1',
        'task': 'CLAUDE-C01-GAPS-01',
        'period': {'from': PERIOD_FROM, 'through': CUTOFF, 'cutoff_frozen': True},
        'limitations': [
            'A gap audit of checked-in inputs only: it is not evidence that every real organization, office or holder has been discovered. Worldwide and certified-country discovery remain open (census c01_complete is false; exhaustive country censuses: 0).',
            'The research cutoff is frozen at 2026-09-07; nothing after it is historical research. Fictional successors begin 2026-09-08 and are outside this ledger.',
            'Coverage statuses describe the pinned evidence, not the truth: an attested day or an open-ended start never becomes an interval, and a day with no evidence is not proof that an office was vacant.',
            'Research organizations are never mapped to simulation party rows here; name matches are listed as uncertain candidates and are not counted.',
            'Executive observations (census seed executives, executive gameplay grants, research executive offices) never count as party-leader coverage, and the reverse.',
            'Integrated packets C01-05/06/09-22/26 are research with historical acceptance pending; claimed or submitted packets that are not integrated appear only as in-flight work, never as accepted evidence.',
            'Completed source repairs record bounded independently accepted source-identity evidence separately; they do not promote their parent packet or add historical coverage.',
        ],
        'evidence_classes': EVIDENCE_CLASSES,
        'inputs': inputs,
        'attribution': sha(ATTRIBUTION, root) if (root / ATTRIBUTION).exists() else None,
        'in_flight': in_flight,
        'completed_source_repairs': completed_repairs,
        'cases': ledger_cases,
        'next_batches': all_batches,
        'totals': totals,
    }


def case_ledger(root, case, identities, countries, represented, roles, terms_by_org, orders_by_member, packets,
                attribution, classes, changed, flight_targets):
    party_rows = [r for r in represented if r['nation'] in identities]
    lifecycle = {d['organization_id']: d for d in roles['lifecycle_disclosures'] if d['kind'] == 'existing_registry_bounds'}
    editorial = [d for d in roles['lifecycle_disclosures'] if d['kind'] != 'existing_registry_bounds' and d.get('nation') in identities]
    research = {nation: read_json(packets[nation], root) for nation in identities if nation in packets}
    source_urls, source_class, source_packets, claim_source = {}, {}, {}, {}
    for nation, packet in research.items():
        attributed = attribution['sources'].get(nation, {})
        for source in packet['sources']:
            row = attributed.get(source['id'])
            if row is None:
                raise ValueError(f'Source {source["id"]} ({nation}) has no pinned attribution; run --refresh-attribution')
            packet_id = row['packet']
            if packet_id not in classes:
                raise ValueError(f'Packet {packet_id} has no acceptance class')
            later = changed.get(source.get('snapshot', {}).get('path'), [])
            involved = sorted({packet_id, *later})
            source_packets[source['id']] = involved
            source_class[source['id']] = sorted({classes[p] for p in involved if p in classes})
            source_urls[source['id']] = source['url']
            for claim in source['claims']:
                claim_source[claim['id']] = source['id']

    # Represented rows and components: party-leader chains from census term records only.
    chains = []
    for row in sorted(party_rows, key=lambda r: r['id']):
        entries = []
        for term in sorted(terms_by_org.get(row['id'], []), key=lambda t: t['id']):
            if term['kind'] not in ('leader', 'co_leader', 'acting'):
                raise ValueError(f'Unexpected term kind {term["kind"]}')
            entries.append({'record': term['id'], 'holder': term['person'], 'kind': term['kind'], 'role': term['role'],
                            'from': bounds(term['from']), 'until': bounds(term['until']),
                            'from_precision': precision(term['from']), 'until_precision': precision(term['until']),
                            'points': [], 'periods': [], 'evidence': ['production_registry']})
        window = chain_window(row, lifecycle.get(row['id']))
        cov = coverage(entries, window)
        chains.append({'id': row['id'], 'nation': row['nation'], 'name': row['name'], 'native': row.get('native'),
                       'representation': row['representation'], 'organization_kind': row['organization_kind'],
                       'components': row.get('component_ids', []), 'history_status': row['history_status'],
                       'role_family': 'party_leader',
                       'window': window, 'coverage': summarize(cov),
                       'terms': [{'record': e['record'], 'holder': e['holder'], 'kind': e['kind'], 'role': e['role'],
                                  'from': e['from'], 'until': e['until'], 'missing_fields': missing_fields(e)} for e in entries],
                       'declared_gaps': [{'from': g['from'], 'until': g['until'], 'reason': g['reason'],
                                          'sources': g.get('sources', [])} for g in row.get('declared_gaps', [])],
                       'sources': row.get('sources', []),
                       'related_work_orders': sorted(set(orders_by_member.get(row['id'], []))),
                       'in_flight': flight_targets.get(f'party:{row["id"]}'),
                       'uncertain_research_candidates': []})

    # Research institutions and organization roles.
    role_rows, orgs = [], []
    for nation, packet in research.items():
        for category in ('organizations', 'institutions'):
            for entity in packet[category]:
                if category == 'organizations':
                    orgs.append({'id': entity['id'], 'nation': nation, 'name': entity['name'], 'kind': entity.get('kind'),
                                 'mapped': entity.get('represented_party_ids', []), 'roles': [r['id'] for r in entity['roles']]})
                for role in entity['roles']:
                    role_rows.append(research_role(nation, category, entity, role, claim_source, source_class,
                                                   source_packets, source_urls, flight_targets))

    # Uncertain mappings: research organizations whose name matches a represented row. Never counted.
    for chain in chains:
        keys = {norm(chain['name']), norm(chain.get('native') or ''), norm(strip_parens(chain['name'])),
                norm(strip_parens(chain.get('native') or ''))}
        code = chain['id'].split('/')[-1]
        if '_' in code:
            keys.add(norm(code.split('_', 1)[1]))
        keys.discard('')
        for org in orgs:
            if org['nation'] == chain['nation'] and ({norm(org['name']), norm(strip_parens(org['name']))} & keys):
                chain['uncertain_research_candidates'].append({
                    'organization': org['id'], 'name': org['name'], 'roles': org['roles'], 'counted': False,
                    'mapped_by_packet': chain['id'] in org['mapped']})
    candidate_orgs = {c['organization'] for chain in chains for c in chain['uncertain_research_candidates']}

    exceptions = case_exceptions(identities, countries, roles, research, role_rows)
    for disclosure in editorial:
        exceptions.append({'nation': disclosure['nation'], 'kind': disclosure['kind'], 'institution': disclosure['organization_id'],
                           'note': disclosure['note'], 'sources': disclosure['sources'],
                           'effect': 'Listed only; an editorial continuation disclosure does not narrow the historical audit window.'})
    exceptions.sort(key=lambda r: (r['nation'], r['kind'], r.get('role') or r.get('institution') or r.get('fact', '')))
    collisions = alias_collisions(chains, orgs, research)
    cross_role = cross_role_people(chains, role_rows)

    items = []
    component_terms = {c['id']: len(c['terms']) for c in chains if c['representation'] != 'simulation_party_row'}
    for chain in chains:
        unresolved = chain['coverage']['unresolved_days'] > 0 or chain['declared_gaps']
        if chain['in_flight'] or not unresolved:
            continue
        item = batch_item('party_leader_chain', chain['nation'], chain['id'], chain['name'], 'party leader (census term records)',
                          chain['coverage'], [t for t in chain['terms'] if t['missing_fields']],
                          leads(chain['sources'] + [u for g in chain['declared_gaps'] for u in g['sources']]),
                          chain['related_work_orders'], chain['declared_gaps'],
                          [c['organization'] for c in chain['uncertain_research_candidates']],
                          priority=0 if chain['representation'] == 'simulation_party_row' else 3)
        item['window'] = {k: chain['window'][k] for k in ('from', 'until', 'pinned')}
        if chain['representation'] == 'simulation_party_row':
            parts = sorted(f'{c} ({component_terms[c]} terms)' for c in component_terms
                           if c.startswith(chain['id'] + '/') and component_terms[c])
            if parts:
                item['components_with_terms'] = parts
                item['note'] = ('Leadership recorded on components or name phases does not fill this parent chain; '
                                'check whether the parent office is separate from the components listed.')
        else:
            item['note'] = ('Component or historical name phase: its active window is not bounded in the pinned inputs; '
                            'confirm when it existed before seeking leaders for the unresolved spans.')
        items.append(item)
    for role in role_rows:
        if role['exception'] or role['in_flight'] or role['coverage']['unresolved_days'] == 0:
            continue
        if role['category'] == 'organizations' and role['entity'] in candidate_orgs:
            continue  # evidence for an uncertain represented-row candidate is folded into that chain's item
        priority = 1 if role['kind'] in ('head_of_state', 'head_of_government') else 2
        if role['category'] == 'organizations':
            priority = 4
        item = batch_item('research_role_chain', role['nation'], f'{role["entity"]}#{role["id"]}', role['title'],
                          role['kind'], role['coverage'], [h for h in role['holders'] if h['missing_fields']],
                          role['leads'], sorted(set(orders_by_member.get(role['entity'], []))), [], [], priority=priority)
        item['window'] = {k: role['window'][k] for k in ('from', 'until', 'pinned')}
        item['first_evidence'] = role['coverage']['first_evidence']
        if not role['window']['pinned'] and role['coverage']['first_evidence'] and role['coverage']['first_evidence'] > role['window']['from']:
            item['note'] = ('Applicability before the first observation is not established in the pinned inputs: confirm when '
                            'the office existed before seeking earlier holders.')
        items.append(item)
    for expected in EXPECTED_OFFICES.get(case, []):
        present = any(f'institution:{e["id"]}' == expected['office'] for p in research.values() for e in p['institutions'])
        if present:
            continue
        policy = next((p for p in roles['institution_policy_records'] if p['nation'] == expected['policy']), None)
        flight = flight_targets.get(expected['office'])
        if flight:
            continue
        items.append({'item': 'missing_institution', 'nation': expected['policy'], 'entity': expected['office'],
                      'title': expected['title'], 'priority': 1,
                      'missing_fields': [{'field': 'institution and holder chain', 'from': PERIOD_FROM, 'through': CUTOFF}],
                      'leads': leads(policy['sources'] if policy else []),
                      'basis': policy['fact'] if policy else None, 'related_work_orders': []})

    items.sort(key=lambda i: (i['priority'], -i.get('unresolved_days', 0), i['entity']))
    batches = []
    slug = case.replace(' -> ', '-')
    for offset in range(0, len(items), MAX_BATCH):
        batches.append({'id': f'GAP-{slug}-B{offset // MAX_BATCH + 1:03d}', 'case': case, 'status': 'open',
                        'maximum_items': MAX_BATCH, 'items': items[offset:offset + MAX_BATCH]})

    evidence_counts = {}
    for role in role_rows:
        for holder in role['holders']:
            key = '+'.join(holder['evidence']) or 'unattributed'
            evidence_counts[key] = evidence_counts.get(key, 0) + 1
    case_row = {
        'case': case, 'identities': identities,
        'totals': {
            'represented_rows': sum(1 for c in chains if c['representation'] == 'simulation_party_row'),
            'components_or_name_phases': sum(1 for c in chains if c['representation'] != 'simulation_party_row'),
            'party_chains_unresolved': sum(1 for c in chains if c['coverage']['unresolved_days'] or c['declared_gaps']),
            'documented_research_organizations': len(orgs),
            'documented_unmapped_organizations': sum(1 for o in orgs if not o['mapped']),
            'research_organizations_with_roles': sum(1 for o in orgs if o['roles']),
            'research_institutions': sum(len(p['institutions']) for p in research.values()),
            'research_roles': len(role_rows),
            'research_roles_unresolved': sum(1 for r in role_rows if r['coverage']['unresolved_days'] and not r['exception']),
            'research_holders_by_evidence': dict(sorted(evidence_counts.items())),
            'seed_executive_observations': sum(1 for s in roles['seed_executive_observations'] if s['nation'] in identities),
            'executive_gameplay_grants': sum(1 for g in roles['historical_executive_gameplay_grants'] if g['nation'] in identities),
        },
        'party_chains': chains,
        'research_roles': role_rows,
        'documented_organizations': {
            'total': len(orgs),
            'with_roles': sorted(o['id'] for o in orgs if o['roles']),
            'discovery_work_orders': sorted({w for o in orgs for w in orders_by_member.get(o['id'], [])}),
            'note': 'Research organization observations (registers, filings, lists) are documented but unreconciled; their identity reconciliation is tracked by the existing discovery work orders, not re-batched here.',
        },
        'executive_observations': {
            'seed_executives': [s for s in roles['seed_executive_observations'] if s['nation'] in identities],
            'gameplay_grants': sorted(g['term'] for g in roles['historical_executive_gameplay_grants'] if g['nation'] in identities),
            'note': 'Executive observations are listed separately; they never close party-leader gaps.',
        },
        'institutional_exceptions': exceptions,
        'alias_collisions': collisions,
        'cross_role_people': cross_role,
    }
    return {'case': case_row, 'batches': batches}


def research_role(nation, category, entity, role, claim_source, source_class, source_packets, source_urls, flight_targets):
    holders, entries = [], []
    for position, holder in enumerate(role.get('holder_claims', [])):
        if isinstance(holder, str):
            source = claim_source.get(holder)
            holders.append({'position': position, 'name': None, 'claim': holder, 'attested_on': None, 'from': None, 'until': None,
                            'evidence': source_class.get(source, []), 'packets': source_packets.get(source, []),
                            'missing_fields': ['holder identity and dates (claim-only observation)']})
            continue
        refs = holder.get('sources', [])
        evidence = sorted({c for s in refs for c in source_class.get(s, [])})
        involved = sorted({p for s in refs for p in source_packets.get(s, [])})
        period = holder.get('attested_period') or holder.get('observation_window')
        entry = {'record': f'{role["id"]}[{position}]', 'holder': holder['name'], 'kind': 'holder',
                 'from': bounds(holder.get('from')), 'until': bounds(holder.get('until')),
                 'from_precision': 'day' if holder.get('from') else 'absent',
                 'until_precision': 'day' if holder.get('until') else 'absent',
                 'points': [holder['attested_on']] if holder.get('attested_on') else [],
                 'periods': [(period['from'], period['through'])] if period else [], 'evidence': evidence}
        entries.append(entry)
        holders.append({'position': position, 'name': holder['name'], 'attested_on': holder.get('attested_on'),
                        'from': holder.get('from'), 'until': holder.get('until'),
                        'attested_period': period, 'evidence': evidence, 'packets': involved,
                        'missing_fields': missing_fields(entry)})
    window = window_for(nation, role['id'])
    exception = None
    if role['kind'] == 'collective_seat':
        exception = 'collective_institution'
    target = flight_targets.get(f'role:{role["id"]}') or flight_targets.get(f'institution:{entity["id"]}')
    return {'nation': nation, 'category': category, 'entity': entity['id'], 'entity_name': entity['name'],
            'id': role['id'], 'title': role['title'], 'kind': role['kind'],
            'hereditary': HEREDITARY.get(entity['id']), 'exception': exception, 'window': window,
            'coverage': summarize(coverage(entries, window)), 'holders': holders,
            'source_count': len(role.get('sources', [])),
            'leads': leads([source_urls[s] for s in role.get('sources', []) if s in source_urls]),
            'in_flight': target}


def summarize(cov):
    unresolved = [s for s in cov['segments'] if s['status'] in UNRESOLVED]
    longest = sorted(unresolved, key=lambda s: (-(ordinal(s['through']) - ordinal(s['from'])), s['from']))[:8]
    return {'window_days': cov['window_days'], 'days': cov['days'], 'unresolved_days': cov['unresolved_days'],
            'first_evidence': cov['first_evidence'], 'segments': cov['segments'],
            'longest_unresolved': sorted(longest, key=lambda s: s['from'])}


def leads(urls):
    seen, result = set(), []
    for url in urls:
        if url and url not in seen:
            seen.add(url)
            result.append(url)
    return result[:6] or ['no existing lead in the pinned inputs; start from the institution\'s own primary records']


def batch_item(kind, nation, entity, title, role, cov, holders, lead_urls, orders, declared, candidates, priority):
    fields = [{'field': 'holder', 'from': s['from'], 'through': s['through'], 'current_status': s['status']}
              for s in cov['longest_unresolved']]
    for holder in holders:
        fields.append({'field': 'dates', 'holder': holder.get('holder') or holder.get('name') or holder.get('claim'),
                       'record': holder.get('record') or holder.get('claim') or holder.get('position'),
                       'missing': holder['missing_fields']})
    return {'item': kind, 'nation': nation, 'entity': entity, 'title': title, 'role': role, 'priority': priority,
            'unresolved_days': cov['unresolved_days'], 'missing_fields': fields, 'declared_gaps': declared,
            'uncertain_research_candidates': candidates, 'leads': lead_urls, 'related_work_orders': orders}


def case_exceptions(identities, countries, roles, research, role_rows):
    rows = []
    for nation in identities:
        if countries[nation]['party_rows'] == 0:
            rows.append({'nation': nation, 'kind': 'no_simulation_party_rows',
                         'note': 'No simulation party rows: documented research organizations are unrepresented and no party-leader chain is expected in the census.'})
        for policy in roles['institution_policy_records']:
            if policy['nation'] == nation:
                rows.append({'nation': nation, 'kind': 'institution_policy', 'fact': policy['fact'],
                             'gameplay_assumption': policy['gameplay_assumption'], 'sources': policy['sources']})
    for role in role_rows:
        if role['hereditary']:
            rows.append({'nation': role['nation'], 'kind': 'hereditary_office', 'role': role['id'], 'institution': role['entity'],
                         'note': f'{role["hereditary"]}: no party or electoral succession path; the holder chain is still researched.'})
        if role['exception'] == 'collective_institution':
            rows.append({'nation': role['nation'], 'kind': 'collective_institution', 'role': role['id'], 'institution': role['entity'],
                         'note': 'Collective seats: a seat census, not a single-holder chain; excluded from chain batches.'})
        if role['window']['pinned']:
            rows.append({'nation': role['nation'], 'kind': 'applicability_window', 'role': role['id'],
                         'from': role['window']['from'], 'until': role['window']['until'],
                         'anchors': role['window']['anchors'], 'note': role['window']['note']})
    for nation, packet in research.items():
        for institution in packet['institutions']:
            if not institution['roles']:
                rows.append({'nation': nation, 'kind': 'institution_without_roles', 'institution': institution['id'],
                             'note': f'{institution.get("kind")}: observed as an institution with no office roles in the pinned research.'})
    return sorted(rows, key=lambda r: (r['nation'], r['kind'], r.get('role') or r.get('institution') or r.get('fact', '')))


def alias_collisions(chains, orgs, research):
    groups = {}
    for chain in chains:
        for label in {chain['name'], chain.get('native') or ''}:
            if norm(label):
                groups.setdefault((chain['nation'], norm(label)), {})[chain['id']] = ('represented_row', chain['name'])
    for org in orgs:
        if norm(org['name']):
            groups.setdefault((org['nation'], norm(org['name'])), {})[org['id']] = ('research_organization', org['name'])
    for nation, packet in research.items():
        for institution in packet['institutions']:
            if norm(institution['name']):
                groups.setdefault((nation, norm(institution['name'])), {})[institution['id']] = ('research_institution', institution['name'])
    rows = []
    for (nation, key), members in sorted(groups.items()):
        if len(members) < 2:
            continue
        kinds = {kind for kind, _ in members.values()}
        if 'represented_row' in kinds and 'research_organization' in kinds:
            classification = 'uncertain_registry_research_match'
        elif 'research_institution' in kinds and len(kinds) > 1:
            classification = 'organization_versus_institution'
        else:
            classification = 'same_name_distinct_ids'
        rows.append({'nation': nation, 'key': key, 'classification': classification, 'merged': False,
                     'members': [{'id': i, 'kind': k, 'name': n} for i, (k, n) in sorted(members.items())]})
    return rows


def cross_role_people(chains, role_rows):
    """People named both in party-leader terms and in research office holders: roles stay separate."""
    leaders = {}
    for chain in chains:
        for term in chain['terms']:
            leaders.setdefault(norm(term['holder'].replace('_', ' ')), set()).add((chain['id'], term['role']))
    rows = {}
    for role in role_rows:
        if role['kind'] in ('party_leader', 'party_chair'):
            continue
        for holder in role['holders']:
            key = norm(holder['name'] or '')
            if key and key in leaders:
                row = rows.setdefault(key, {'person_key': key, 'party_leader_roles': sorted(leaders[key]), 'office_roles': set()})
                row['office_roles'].add((role['entity'], role['id']))
    return [{'person_key': k, 'party_leader_roles': [list(x) for x in v['party_leader_roles']],
             'office_roles': [list(x) for x in sorted(v['office_roles'])],
             'note': 'Same name in a party-leader chain and an office chain; neither counts toward the other.'}
            for k, v in sorted(rows.items())]


# ---------------------------------------------------------------- markdown

def render(ledger):
    lines = ['# Certified-country research gap ledger', '',
             f'Generated by `tools/avatars/certified_gap_ledger.py` ({ledger["task"]}). Period {ledger["period"]["from"]} to '
             f'{ledger["period"]["through"]} (frozen cutoff). Regenerate with `python -X utf8 tools/avatars/certified_gap_ledger.py`; '
             'check with `--check`.', '', '## Limitations', '']
    lines += [f'- {text}' for text in ledger['limitations']]
    lines += ['', '## Evidence classes', '']
    lines += [f'- `{k}`: {v}' for k, v in ledger['evidence_classes'].items()]
    lines += ['', '## In-flight work (excluded from new batches, never accepted)', '', '| Task | State | Case | Scope | Targets |', '|---|---|---|---|---|']
    for row in ledger['in_flight']:
        lines.append(f'| {row["task"]} | {row["state"]} | {row["case"]} | {row["scope"]} | {", ".join(row["targets"]) or "none (source repair)"} |')
    if ledger['completed_source_repairs']:
        lines += ['', '## Completed source repairs (bounded acceptance only)', '',
                  'These integrated source-identity reviews add no historical coverage and do not accept the parent packet.', '']
        for repair in ledger['completed_source_repairs']:
            lines.append(f'- `{repair["task"]}` / `{repair["source"]}`: {repair["scope"]}')
            lines.append(f'  - Reviewed `{repair["reviewed_commit"]}`; parent `{repair["parent_packet"]}` remains '
                         f'`{repair["parent_evidence_class"]}`. Review and evidence hashes are pinned in `ledger.json`.')
    lines += ['', '## Totals', '']
    lines += [f'- {k.replace("_", " ")}: {v}' for k, v in ledger['totals'].items()]
    for case in ledger['cases']:
        t = case['totals']
        lines += ['', f'## {case["case"]}', '',
                  f'Represented rows {t["represented_rows"]}, components or name phases {t["components_or_name_phases"]}, '
                  f'party chains with gaps {t["party_chains_unresolved"]}; research organizations {t["documented_research_organizations"]} '
                  f'({t["documented_unmapped_organizations"]} unmapped, {t["research_organizations_with_roles"]} with roles); '
                  f'research institutions {t["research_institutions"]}, roles {t["research_roles"]} ({t["research_roles_unresolved"]} with unresolved days); '
                  f'seed executives {t["seed_executive_observations"]}, executive grants {t["executive_gameplay_grants"]}.', '']
        if t['research_holders_by_evidence']:
            lines.append('Research holders by evidence class: ' + ', '.join(f'`{k}` {v}' for k, v in t['research_holders_by_evidence'].items()) + '.')
            lines.append('')
        if case['party_chains']:
            lines += ['| Party chain | Kind | Definite days | Unresolved days | Declared gaps | Candidates (not counted) | In flight |', '|---|---|---|---|---|---|---|']
            for c in case['party_chains']:
                cov = c['coverage']
                lines.append(f'| `{c["id"]}` {c["name"]} | {c["representation"]} | {cov["days"]["definite"]} | {cov["unresolved_days"]} | '
                             f'{len(c["declared_gaps"])} | {", ".join(x["organization"] for x in c["uncertain_research_candidates"]) or "-"} | {c["in_flight"] or "-"} |')
            lines.append('')
        if case['research_roles']:
            lines += ['| Research role | Kind | Window | Holders | Definite days | Unresolved days | Exception / in flight |', '|---|---|---|---|---|---|---|']
            for r in case['research_roles']:
                cov = r['coverage']
                lines.append(f'| `{r["entity"]}#{r["id"]}` {r["title"]} | {r["kind"]} | {r["window"]["from"]} to {r["window"]["until"]} | '
                             f'{len(r["holders"])} | {cov["days"]["definite"]} | {cov["unresolved_days"]} | {r["exception"] or r["in_flight"] or r["hereditary"] or "-"} |')
            lines.append('')
        if case['institutional_exceptions']:
            lines += ['Institutional exceptions:', '']
            for e in case['institutional_exceptions']:
                label = e.get('role') or e.get('institution') or ''
                text = e.get('note') or e.get('fact')
                lines.append(f'- `{e["kind"]}` {e["nation"]} {label}: {text}')
            lines.append('')
        if case['alias_collisions']:
            lines += [f'Alias collisions (never merged): {len(case["alias_collisions"])}.', '']
            for a in case['alias_collisions'][:12]:
                lines.append(f'- `{a["classification"]}`: ' + '; '.join(f'{m["id"]} ({m["kind"]})' for m in a['members']))
            if len(case['alias_collisions']) > 12:
                lines.append(f'- and {len(case["alias_collisions"]) - 12} more in ledger.json')
            lines.append('')
        if case['cross_role_people']:
            lines += ['People in both a party-leader chain and an office chain (roles kept separate): '
                      + ', '.join(p['person_key'] for p in case['cross_role_people']) + '.', '']
    lines += ['', '## Next research batches', '',
              'Each batch holds at most ten chains. Claim a batch by its ID; in-flight targets are excluded. Missing fields list the '
              'longest unresolved spans and every record with missing or imprecise dates; leads are URLs already cited in the pinned inputs.', '']
    for batch in ledger['next_batches']:
        lines += [f'### {batch["id"]} ({batch["case"]}, {len(batch["items"])} items)', '']
        for item in batch['items']:
            spans = [f'{f["from"]}..{f["through"]} ({f["current_status"]})' for f in item['missing_fields'] if f['field'] == 'holder'][:3]
            records = [f for f in item['missing_fields'] if f['field'] == 'dates']
            lines.append(f'- `{item["entity"]}` {item["title"]} [{item["item"]}; priority {item["priority"]}; unresolved days {item.get("unresolved_days", "n/a")}]')
            if item['item'] == 'missing_institution':
                lines.append(f'  - Missing: the whole institution and holder chain, {PERIOD_FROM}..{CUTOFF}. Basis: {item["basis"]}')
            if spans:
                lines.append(f'  - Unresolved spans: {"; ".join(spans)}')
            if records:
                lines.append(f'  - Records with missing dates: {len(records)} (e.g. {records[0]["holder"]}: {", ".join(records[0]["missing"])})')
            if item.get('declared_gaps'):
                lines.append(f'  - Declared registry gaps: {len(item["declared_gaps"])}')
            lines.append(f'  - Leads: {", ".join(item["leads"][:3])}')
        lines.append('')
    return '\n'.join(lines).rstrip() + '\n'


def outputs(root=ROOT, attribution=None):
    ledger = build(root, attribution)
    return {OUTPUT / 'ledger.json': canonical(ledger), OUTPUT / 'ledger.md': render(ledger)}


def stale(files, root=ROOT):
    """Generated paths whose checked-in content differs from a fresh build."""
    return [path.as_posix() for path, content in files.items()
            if not (root / path).exists() or (root / path).read_text(encoding='utf-8') != content]


def write(files, root=ROOT):
    for path, content in files.items():
        (root / path).parent.mkdir(parents=True, exist_ok=True)
        (root / path).write_text(content, encoding='utf-8', newline='\n')


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('--check', action='store_true', help='Compare generated ledger files without writing')
    parser.add_argument('--refresh-attribution', action='store_true', help='Rebuild the pinned attribution from git history')
    args = parser.parse_args()
    if args.refresh_attribution:
        write({ATTRIBUTION: canonical(refresh_attribution())})
    files = outputs()
    if args.check:
        differing = stale(files)
        if differing:
            raise SystemExit(f'Gap ledger differs from its pinned inputs: {", ".join(differing)}')
    else:
        write(files)
    ledger = json.loads(files[OUTPUT / 'ledger.json'])
    print(canonical({'check': args.check, 'totals': ledger['totals']}), end='')


if __name__ == '__main__':
    main()
