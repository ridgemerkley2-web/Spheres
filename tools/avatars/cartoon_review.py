#!/usr/bin/env python3
"""Read-only cartoon asset check and review export (CLAUDE-C03-REVIEW-01).

Reads the checked-in portrait manifests (historical and fictional), the
country-selector figure manifest, the display-derivative manifest, the
leadership production inventory, the fictional candidate catalogue, the
party-leader registry and the reference audits, plus every tracked image file
under the art roots, and writes a deterministic JSON + Markdown export to
docs/campaign-certification/C03/preparation/cartoon-review/.

Each image is checked for existence, dimensions (read from the PNG/WebP/JPEG
header), byte size and SHA-256 against its manifest record. The export lists
duplicate images bound to different identities, appearance intervals and their
coverage of the sourced art windows, source/rights gaps, the review decisions
recorded in the manifests and the visual review sample recorded by this packet.

It never edits, generates, approves or substitutes an image or a manifest.
Automated findings are integrity and consistency observations, not visual
approval. A duplicate is a review finding, not proof of a wrong identity.

Default mode writes the export; --check regenerates it in memory and exits 1
when a committed export file differs. Standard library only; no network access.
The file inventory is the git index when available (untracked files are
ignored), otherwise a directory walk. Authored text (credits, era notes,
rationales) stays in the manifests: every item carries a JSON pointer to its
record, which the reviewer resolves in the hash-verified manifest.
"""
from __future__ import annotations

import argparse
from collections import Counter, defaultdict
from datetime import date, timedelta
import hashlib
import json
import ntpath
import os
from pathlib import Path, PurePosixPath
import re
import struct
import subprocess
import sys

ROOT = Path(__file__).resolve().parents[2]
OUTPUT = Path('docs/campaign-certification/C03/preparation/cartoon-review')
EXPORT_JSON = OUTPUT / 'cartoon-review.json'
EXPORT_MD = OUTPUT / 'cartoon-review.md'
VISUAL_REVIEW = OUTPUT / 'visual-review-sample.json'
FORMAT = 'spheres-c03-cartoon-review/v1'
SAMPLE_FORMAT = 'spheres-c03-visual-review-sample/v1'
TASK = 'CLAUDE-C03-REVIEW-01'

HISTORICAL_MANIFEST = 'spheres-web/data/person_portraits.json'
FICTIONAL_MANIFEST = 'spheres-web/data/fictional_portraits.json'
SELECTOR_MANIFEST = 'spheres-web/data/nation_figures.json'
DISPLAY_MANIFEST = 'spheres-web/ui/display-art/manifest.json'
PRODUCTION = 'spheres-web/data/leadership_production_2035.json'
CATALOG = 'spheres-web/data/future_candidates_2035.json'
REGISTRY = 'spheres-sim/data/party_leaders.json'
# (role, repository path, required)
INPUTS = (
    ('historical_portraits', HISTORICAL_MANIFEST, True),
    ('fictional_portraits', FICTIONAL_MANIFEST, True),
    ('selector_figures', SELECTOR_MANIFEST, True),
    ('display_derivatives', DISPLAY_MANIFEST, True),
    ('production_inventory', PRODUCTION, True),
    ('fictional_catalog', CATALOG, True),
    ('person_registry', REGISTRY, True),
    ('reference_audit_uk', 'spheres-web/ui/person-portraits/references/source-review-uk-v1.json', False),
    ('s10c_reviewed_reference', 'docs/campaign-certification/S10/c/manifest.json', False),
    ('visual_review_sample', VISUAL_REVIEW.as_posix(), False),
)
PERSON_ART = 'spheres-web/ui/person-portraits'
SELECTOR_PHOTOS = 'spheres-web/ui/portraits'
SELECTOR_ART = 'spheres-web/ui/leader-art'
DISPLAY_ART = 'spheres-web/ui/display-art'
ART_ROOTS = (PERSON_ART, SELECTOR_PHOTOS, SELECTOR_ART, DISPLAY_ART)
IMAGE_EXTS = {'.png', '.webp', '.jpg', '.jpeg'}
HISTORICAL = ('1990-01-01', '2026-09-08')
FICTIONAL = ('2026-09-08', '2036-01-01')
COLLECTIONS = ('historical', 'fictional', 'selector', 'unregistered', 'missing')
COLLECTION_SOURCES = {
    'historical': {'label': 'Historical person cartoons', 'manifest': HISTORICAL_MANIFEST, 'identity_registry': REGISTRY,
                   'countries': 'sourced party terms and office observations in ' + PRODUCTION},
    'fictional': {'label': 'Fictional successor cartoons (not real people)', 'manifest': FICTIONAL_MANIFEST,
                  'identity_registry': CATALOG, 'countries': 'fictional catalogue nation'},
    'selector': {'label': 'Country-selector historical figures', 'manifest': SELECTOR_MANIFEST,
                 'identity_registry': SELECTOR_MANIFEST, 'countries': 'selector NationId'},
    'unregistered': {'label': 'Tracked cartoon-root files bound by no manifest (not active avatars)', 'manifest': None,
                     'identity_registry': None, 'countries': 'none'},
    'missing': {'label': 'Known people with sourced art windows and no cartoon', 'manifest': PRODUCTION,
                'identity_registry': REGISTRY, 'countries': 'sourced party terms and office observations in ' + PRODUCTION},
}
SEVERITIES = ('error', 'warning', 'notice')
ALLOWED_DECISIONS = ('fixes_proposed', 'reference_check_required')
FORBIDDEN_SAMPLE_KEYS = ('approved', 'accepted', 'approval', 'acceptance')
IDENTITY_ROLES = {'cartoon-master', 'cartoon-display-derivative', 'identity-reference-photo',
                  'identity-photo-display-derivative', 'identity-reference-generated-study'}
HASH_RE = re.compile(r'[0-9a-f]{64}\Z')
ID_RE = re.compile(r'[A-Za-z0-9][A-Za-z0-9_.-]{0,119}\Z')
PHOTO_NAME_RE = re.compile(r'(?P<nation>[A-Za-z0-9]+)-(?P<hex>[0-9a-f]{6,16})\.webp\Z')
LEADER_NAME_RE = re.compile(r'(?P<nation>[A-Za-z0-9]+)-leader-(?P<hex>[0-9a-f]{12})\.png\Z')
DISPLAY_NAME_RE = re.compile(r'.+-(?P<hex>[0-9a-f]{16})\.webp\Z')
STYLE_LINE_RE = re.compile(r'^Style reference:\s*`([^`]+)`', re.M)
SOF_MARKERS = {0xC0, 0xC1, 0xC2, 0xC3, 0xC5, 0xC6, 0xC7, 0xC9, 0xCA, 0xCB, 0xCD, 0xCE, 0xCF}

CODE_TEXT = {
    'missing_file': ('error', 'A manifest references a file that does not exist.'),
    'unsafe_path': ('error', 'A manifest path is not a safe repository path inside the approved roots.'),
    'unreadable_image': ('error', 'The file is not a readable PNG, WebP or JPEG header.'),
    'hash_mismatch': ('error', 'The file bytes do not match the recorded SHA-256 (or none is recorded).'),
    'dimension_mismatch': ('error', 'The header dimensions do not match the recorded width/height.'),
    'invalid_interval': ('error', 'An appearance interval is malformed or empty (from must precede the exclusive to).'),
    'overlapping_intervals': ('error', 'Two appearance intervals of one identity overlap; selection would be ambiguous.'),
    'unknown_identity': ('error', 'The identity is not in its registry or catalogue.'),
    'identity_binding_mismatch': ('error', 'An identity reference names a different identity than its record.'),
    'fictional_identity_collision': ('error', 'A fictional record uses a historical identity or a historical image.'),
    'prompt_record_missing': ('error', 'The recorded prompt record file does not exist.'),
    'prompt_job_person_mismatch': ('error', 'The prompt record job for this asset names a different person.'),
    'visual_review_unknown_item': ('error', 'A visual review entry names an item that is not in this export.'),
    'duplicate_content_different_identities': ('warning', 'Identical image bytes are bound to different identities (a review finding, not proof of a wrong identity).'),
    'duplicate_asset_path_different_identities': ('warning', 'One asset path is bound to different identities.'),
    'interval_outside_period': ('warning', 'An appearance interval extends outside its declared historical or fictional period.'),
    'sharealike_derivative_license_not_recorded': ('warning', 'The identity reference is ShareAlike-licensed but no derivative licence is recorded for the artwork.'),
    'identity_reference_generated': ('warning', 'The identity reference named in the prompt record is an earlier generated study, not a source photograph.'),
    'display_derivative_stale': ('warning', 'The display derivative manifest records a different source hash or size than the current files.'),
    'display_derivative_missing': ('warning', 'No display derivative is recorded for this selector image.'),
    'content_address_mismatch': ('warning', 'The content-addressed filename does not match the file hash.'),
    'name_mismatch': ('warning', 'The manifest name (or appearance seed) is not the registry or catalogue value.'),
    'visual_review_stale': ('warning', 'The visual review sample was recorded against different file bytes.'),
    'production_inventory_stale': ('warning', 'The production inventory was generated from different manifest or registry bytes.'),
    'prompt_job_not_found': ('warning', 'The prompt record exists but has no job for this asset.'),
    'recorded_review_incomplete': ('notice', 'The manifest review record does not mark every required review field true.'),
    'interval_after_death': ('notice', 'The appearance interval continues after the recorded death date.'),
    'coverage_gap': ('notice', 'Part of a sourced art window is not covered by any appearance interval.'),
    'missing_art': ('notice', 'A known person has sourced art windows but no cartoon.'),
    'country_unbound': ('notice', 'No sourced term or office observation binds this person to a country.'),
    'reference_rights_not_recorded': ('notice', 'The identity reference is not shipped and its rights are not recorded in the manifest.'),
    'text_led_interpretation': ('notice', 'No freely licensed identity photograph; the artwork is a text-led interpretation of a resolved identity.'),
    'identity_photo_selected_automatically': ('notice', 'The identity photograph record states an automated title-match selection.'),
    'unbound_file': ('notice', 'A tracked image in an art root is not bound by any manifest or reference audit.'),
}
STATEMENT = ('Read-only automated asset check. Automated findings are integrity and consistency observations, not visual '
             'approval; this export approves, accepts or replaces no artwork. A duplicate image is a review finding, not '
             'proof of a wrong identity.')


# ---------------------------------------------------------------- helpers

def read_json(root: Path, rel: str):
    return json.loads((root / rel).read_text(encoding='utf-8-sig'))


def canonical(data: dict) -> str:
    """Stable JSON: top-level keys indented, each record of a list of objects on its own line."""
    keys = list(data)
    lines = ['{']
    for n, key in enumerate(keys):
        value, comma = data[key], ',' if n < len(keys) - 1 else ''
        if isinstance(value, list) and value and all(isinstance(v, dict) for v in value):
            lines.append(f' {json.dumps(key)}: [')
            for m, element in enumerate(value):
                lines.append('  ' + json.dumps(element, ensure_ascii=False, separators=(',', ':')) + (',' if m < len(value) - 1 else ''))
            lines.append(' ]' + comma)
        else:
            lines.append(f' {json.dumps(key)}: ' + json.dumps(value, ensure_ascii=False, indent=1).replace('\n', '\n ') + comma)
    lines.append('}')
    return '\n'.join(lines) + '\n'


def iso_day(value) -> date:
    if not isinstance(value, str) or not re.fullmatch(r'\d{4}-\d{2}-\d{2}', value):
        raise ValueError('expected an exact YYYY-MM-DD date')
    return date.fromisoformat(value)


def text(value) -> str | None:
    return value if isinstance(value, str) and value.strip() else None


def compact(record: dict) -> dict:
    return {k: v for k, v in record.items() if v not in (None, [], {})}


def image_info(data: bytes) -> tuple[str, int, int]:
    """Format and pixel size from the file header; raises ValueError otherwise."""
    if data[:8] == b'\x89PNG\r\n\x1a\n':
        if len(data) < 24 or data[12:16] != b'IHDR':
            raise ValueError('PNG without a leading IHDR chunk')
        result = ('PNG', *struct.unpack('>II', data[16:24]))
    elif data[:4] == b'RIFF' and data[8:12] == b'WEBP':
        if len(data) < 30:
            raise ValueError('truncated WebP header')
        chunk = data[12:16]
        if chunk == b'VP8 ':
            if data[23:26] != b'\x9d\x01\x2a':
                raise ValueError('VP8 frame without start code')
            result = ('WEBP', struct.unpack('<H', data[26:28])[0] & 0x3FFF, struct.unpack('<H', data[28:30])[0] & 0x3FFF)
        elif chunk == b'VP8L':
            if data[20] != 0x2F:
                raise ValueError('VP8L frame without signature')
            bits = int.from_bytes(data[21:25], 'little')
            result = ('WEBP', (bits & 0x3FFF) + 1, ((bits >> 14) & 0x3FFF) + 1)
        elif chunk == b'VP8X':
            result = ('WEBP', int.from_bytes(data[24:27], 'little') + 1, int.from_bytes(data[27:30], 'little') + 1)
        else:
            raise ValueError('unknown WebP chunk')
    elif data[:3] == b'\xff\xd8\xff':
        i, result = 2, None
        while i + 4 <= len(data):
            if data[i] != 0xFF:
                raise ValueError('malformed JPEG marker sequence')
            while i < len(data) and data[i] == 0xFF:
                i += 1
            marker = data[i]
            i += 1
            if marker == 0x01 or 0xD0 <= marker <= 0xD8:
                continue
            if marker == 0xD9:
                break
            length = struct.unpack('>H', data[i:i + 2])[0]
            if marker in SOF_MARKERS:
                height, width = struct.unpack('>HH', data[i + 3:i + 7])
                result = ('JPEG', width, height)
                break
            i += length
        if result is None:
            raise ValueError('JPEG without a frame header')
    else:
        raise ValueError('not a PNG, WebP or JPEG file')
    if result[1] <= 0 or result[2] <= 0:
        raise ValueError('empty image dimensions')
    return result


def safe_rel(value, allowed=ART_ROOTS) -> str:
    if not isinstance(value, str) or not value or '\\' in value or ':' in value:
        raise ValueError('expected a repository-relative path using forward slashes')
    if PurePosixPath(value).is_absolute() or any(part in {'', '.', '..'} for part in value.split('/')):
        raise ValueError('unsafe repository path')
    if allowed and not any(value.startswith(prefix + '/') for prefix in allowed):
        raise ValueError('path is outside the approved roots')
    return value


def text_digest(root: Path, rel: str) -> dict:
    """Existence and LF-normalized digest of a text provenance file (checkouts differ by core.autocrlf)."""
    path = root / rel
    if not path.is_file():
        return {'path': rel, 'exists': False}
    data = path.read_bytes().replace(b'\r\n', b'\n')
    return {'path': rel, 'exists': True, 'sha256_lf': hashlib.sha256(data).hexdigest()}


def interval_text(first, until) -> str:
    return f"{first} → {until or 'open'} (end excluded)"


def latest_possible(bound) -> date | None:
    """Latest calendar day a day/month/year-precision registry date can denote."""
    if not isinstance(bound, dict):
        return None
    kind, value = bound.get('kind'), bound.get('value')
    try:
        if kind == 'day':
            return iso_day(value)
        if kind == 'month' and re.fullmatch(r'\d{4}-\d{2}', str(value)):
            start = iso_day(value + '-01')
            return date(start.year + (start.month == 12), start.month % 12 + 1, 1) - timedelta(days=1)
        if kind == 'year' and re.fullmatch(r'\d{4}', str(value)):
            return date(int(value), 12, 31)
    except ValueError:
        return None
    return None


def bound_text(bound) -> str | None:
    if isinstance(bound, dict) and bound.get('value'):
        return f"{bound['value']} ({bound.get('kind', 'unknown')} precision)"
    return None


def subtract(windows, covered):
    """Half-open date windows minus the union of covered intervals."""
    gaps = []
    for start, end in windows:
        cursor = start
        for c_start, c_end in sorted(covered):
            if c_end <= cursor or c_start >= end:
                continue
            if c_start > cursor:
                gaps.append((cursor, c_start))
            cursor = max(cursor, c_end)
            if cursor >= end:
                break
        if cursor < end:
            gaps.append((cursor, end))
    return gaps


def art_inventory(root: Path) -> tuple[list[str], str]:
    """Tracked image paths under the art roots (git index), else a directory walk."""
    if (root / '.git').exists():
        try:
            out = subprocess.run(['git', '-c', 'core.quotepath=false', 'ls-files', '-z', '--', *ART_ROOTS],
                                 cwd=root, capture_output=True, check=True).stdout.decode('utf-8')
            return sorted(p for p in out.split('\0') if p and PurePosixPath(p).suffix.lower() in IMAGE_EXTS), 'git-index'
        except (OSError, subprocess.CalledProcessError):
            pass
    found = []
    for prefix in ART_ROOTS:
        base = root / prefix
        for dirpath, _dirs, files in os.walk(base) if base.is_dir() else ():
            found += [(Path(dirpath) / n).relative_to(root).as_posix() for n in files if PurePosixPath(n).suffix.lower() in IMAGE_EXTS]
    return sorted(found), 'filesystem'


# ---------------------------------------------------------------- builder

class Builder:
    def __init__(self, root: Path):
        self.root = Path(root)
        self.inputs, self.data = [], {}
        for role, rel, required in INPUTS:
            path = self.root / rel
            if not path.is_file():
                if required:
                    raise SystemExit(f'Required input is missing: {rel}')
                continue
            raw = path.read_bytes()
            # JSON text can be checked out with CRLF without changing its content.
            # Keep the scope explicit; image identities remain exact raw bytes.
            canonical_bytes = raw.replace(b'\r\n', b'\n')
            self.inputs.append({'role': role, 'path': rel, 'hash_scope': 'utf8-lf',
                                'bytes': len(canonical_bytes), 'sha256': hashlib.sha256(canonical_bytes).hexdigest()})
            self.data[role] = json.loads(raw.decode('utf-8-sig'))
        self.file_cache: dict[str, dict] = {}
        self.bindings: dict[str, list[dict]] = defaultdict(list)
        self.items: list[dict] = []
        self.findings: list[dict] = []
        self.inventory, self.inventory_mode = art_inventory(self.root)
        self.nation_names = {c['id']: c['name'] for c in self.data['production_inventory'].get('countries', [])
                             if isinstance(c, dict) and 'id' in c}
        self.registry = {p.get('id'): p for p in self.data['person_registry'].get('people', []) if isinstance(p, dict)}
        production = self.data['production_inventory'].get('people', [])
        self.production = {p['id']: p for p in production if isinstance(p, dict) and 'id' in p}
        self.production_index = {p['id']: n for n, p in enumerate(production) if isinstance(p, dict) and 'id' in p}

    # -- files and findings
    def file_facts(self, rel: str) -> dict:
        if rel not in self.file_cache:
            path = self.root / rel
            facts = {'path': rel, 'exists': path.is_file()}
            if facts['exists']:
                data = path.read_bytes()
                facts.update(bytes=len(data), sha256=hashlib.sha256(data).hexdigest())
                try:
                    facts.update(zip(('format', 'width', 'height'), image_info(data)))
                except ValueError as error:
                    facts['error'] = str(error)
            self.file_cache[rel] = facts
        return self.file_cache[rel]

    def bind(self, rel: str, item_id: str | None, role: str, identity: str | None):
        self.bindings[rel].append({'item': item_id, 'role': role, 'identity': identity})

    def find(self, code: str, item: str | None, message: str, path: str | None = None, severity: str | None = None):
        self.findings.append({'severity': severity or CODE_TEXT[code][0], 'code': code, 'item': item, 'path': path, 'message': message})

    @staticmethod
    def lean(item: dict) -> dict:
        """Drop empty fields and a card path that repeats the asset path."""
        item = compact(item)
        if item.get('card') == item.get('asset'):
            item.pop('card', None)
        return item

    def check_asset(self, item_id: str | None, rel, recorded: dict, allowed=ART_ROOTS) -> str | None:
        """Integrity findings for one recorded asset; returns the safe path (existing or not) or None."""
        try:
            rel = safe_rel(rel, allowed)
        except ValueError as error:
            self.find('unsafe_path', item_id, f'{rel!r}: {error}')
            return None
        facts = self.file_facts(rel)
        sha = recorded.get('sha256')
        if not isinstance(sha, str) or not HASH_RE.fullmatch(sha):
            self.find('hash_mismatch', item_id, f'{rel}: no full lowercase SHA-256 is recorded', rel)
            sha = None
        if not facts['exists']:
            self.find('missing_file', item_id, f'{rel} does not exist; a recorded asset is not a delivered image', rel)
            return rel
        if 'error' in facts:
            self.find('unreadable_image', item_id, f"{rel}: {facts['error']}", rel)
        if sha and sha != facts['sha256']:
            self.find('hash_mismatch', item_id, f"{rel}: recorded {sha[:12]}…, actual {facts['sha256'][:12]}…", rel)
        for field in ('width', 'height'):
            if field in recorded and 'error' not in facts and recorded[field] != facts[field]:
                self.find('dimension_mismatch', item_id, f'{rel}: recorded {field} {recorded[field]}, header {facts[field]}', rel)
        return rel

    def interval(self, item_id: str, record: dict, period, period_name: str, outside_severity: str) -> dict:
        first, until = record.get('from'), record.get('to', '__absent__')
        shown = None if until == '__absent__' else until
        result = {'from': first, 'to': shown, 'valid': False}
        try:
            start = iso_day(first)
            if until == '__absent__':
                raise ValueError('an explicit exclusive to date (or null) is required')
            end = date.max if until is None else iso_day(until)
            if start >= end:
                raise ValueError('from must precede the exclusive to date')
        except ValueError as error:
            self.find('invalid_interval', item_id, f'{interval_text(first, shown)}: {error}')
            return result
        result['valid'] = True
        if start < iso_day(period[0]) or end > iso_day(period[1]):
            self.find('interval_outside_period', item_id,
                      f'{interval_text(first, until)} extends outside the {period_name} period {interval_text(*period)}',
                      severity=outside_severity)
        return result

    def countries_for(self, pid: str) -> list[str]:
        return sorted({r.get('nation') for r in (self.production.get(pid) or {}).get('reasons', []) if r.get('nation')})

    # -- prompt records
    def prompt_summary(self, item_id: str, pid: str, rel, asset: str) -> dict | None:
        if not isinstance(rel, str):
            self.find('prompt_record_missing', item_id, 'no prompt record path is recorded')
            return None
        try:
            rel = safe_rel(rel, ('tools/avatars',))
        except ValueError as error:
            self.find('unsafe_path', item_id, f'prompt record {rel!r}: {error}')
            return None
        record = text_digest(self.root, rel)
        if not record['exists']:
            self.find('prompt_record_missing', item_id, f'{rel} does not exist', rel)
            return record
        if not rel.endswith('.json'):
            record['kind'] = 'exact prompt text'
            return record
        try:
            document = read_json(self.root, rel)
        except (ValueError, OSError) as error:
            self.find('prompt_job_not_found', item_id, f'{rel} is not readable JSON: {error}', rel)
            return record
        job = self.match_job(document, asset)
        if job is None:
            self.find('prompt_job_not_found', item_id, f'{rel} has no job whose asset is {PurePosixPath(asset).name}', rel)
            return record
        if job.get('person_id') and job['person_id'] != pid:
            self.find('prompt_job_person_mismatch', item_id, f"{rel} binds this asset to {job['person_id']}", rel)
        record['kind'] = 'job in a batch prompt record'
        record['reference_note'] = text(job.get('reference_note'))
        record['reference_source'] = text(job.get('source'))
        record['reference_files'] = [ntpath.basename(str(r)) for r in (job.get('refs') or []) if r]
        identity_reference = job.get('identity_reference')
        if isinstance(identity_reference, str):
            record['identity_reference'] = identity_reference
            try:
                generated_study = '/references/' not in safe_rel(identity_reference, (PERSON_ART,))
            except ValueError:
                generated_study = False
            if generated_study:
                self.bind(identity_reference, item_id, 'identity-reference-generated-study', f'person:{pid}')
                self.find('identity_reference_generated', item_id,
                          f'{rel} names {identity_reference} as the identity reference; it is a generated study, not a source photograph',
                          identity_reference)
        return compact(record)

    @staticmethod
    def match_job(document, asset: str):
        name = PurePosixPath(asset).name
        stack = [document]
        while stack:
            node = stack.pop()
            if isinstance(node, dict):
                value = node.get('asset')
                if isinstance(value, str) and (value == asset or PurePosixPath(value).name == name) and ('prompt' in node or 'person_id' in node):
                    return node
                final = node.get('final_art')
                if isinstance(final, dict) and final.get('asset') == asset:
                    return {**node, 'refs': [i.get('asset') for i in node.get('inputs', []) if isinstance(i, dict)]}
                stack.extend(reversed(list(node.values())))
            elif isinstance(node, list):
                stack.extend(reversed(node))
        return None

    # -- collections
    def historical(self):
        people = self.data['historical_portraits'].get('people')
        if not isinstance(people, dict):
            raise SystemExit(f'{HISTORICAL_MANIFEST} requires a people object')
        covered = set()
        for pid in sorted(people):
            person = people[pid] if isinstance(people[pid], dict) else {}
            portraits = person.get('portraits')
            if not isinstance(portraits, list) or not portraits:
                continue
            covered.add(pid)
            known, countries = self.registry.get(pid), self.countries_for(pid)
            windows = []
            for window in (self.production.get(pid) or {}).get('required_art_windows', []):
                try:
                    lo = max(iso_day(window['from']), iso_day(HISTORICAL[0]))
                    hi = min(iso_day(window['to']), iso_day(HISTORICAL[1]))
                except (KeyError, TypeError, ValueError):
                    continue
                if lo < hi:
                    windows.append((lo, hi))
            spans, person_items = [], []
            for index, portrait in enumerate(portraits):
                item_id = f'historical:{pid}#{index}'
                if not isinstance(portrait, dict):
                    self.find('invalid_interval', item_id, 'portrait record is not an object')
                    continue
                item = {'id': item_id, 'collection': 'historical', 'name': person.get('name') or pid,
                        'identity': {'key': f'person:{pid}', 'person_id': pid, 'status': 'known' if known else 'unknown'},
                        'countries': countries, 'record': f'/people/{pid}/portraits/{index}'}
                if not ID_RE.fullmatch(pid) or known is None:
                    self.find('unknown_identity', item_id, f'{pid} is not an authored person ID in {REGISTRY}')
                elif person.get('name') not in [known.get('name'), known.get('native'), *(known.get('aliases') or [])]:
                    self.find('name_mismatch', item_id, f"manifest name {person.get('name')!r} is not {known.get('name')!r} or an alias")
                reference = portrait.get('identity_source') if isinstance(portrait.get('identity_source'), dict) else {}
                if reference.get('person_id') != pid:
                    self.find('identity_binding_mismatch', item_id, f"identity_source.person_id is {reference.get('person_id')!r}, record is {pid}")
                asset = self.check_asset(item_id, portrait.get('asset'), portrait, (PERSON_ART,))
                if asset:
                    item['asset'] = item['card'] = asset
                    self.bind(asset, item_id, 'cartoon-master', f'person:{pid}')
                item['interval'] = self.interval(item_id, portrait, HISTORICAL, 'historical', 'warning')
                if item['interval']['valid']:
                    start = iso_day(portrait['from'])
                    end = date.max if portrait['to'] is None else iso_day(portrait['to'])
                    spans.append((start, end, item_id))
                    died = latest_possible((known or {}).get('died'))
                    if died and end - timedelta(days=1) > died:
                        self.find('interval_after_death', item_id,
                                  f"{interval_text(portrait['from'], portrait['to'])} continues after the recorded death "
                                  f"({bound_text(known.get('died'))}); clip to end by {(died + timedelta(days=1)).isoformat()} (exclusive)")
                rights = {'identity_reference': reference.get('kind'), 'reference_url': reference.get('source_url'),
                          'reference_license': reference.get('license'), 'derivative_license': portrait.get('derivative_license'),
                          'artwork_license': portrait.get('license')}
                if reference.get('kind') == 'observed_portrait' and reference.get('asset'):
                    ref_path = self.check_asset(item_id, reference.get('asset'), reference, (PERSON_ART,))
                    if ref_path:
                        rights['reference_file'] = ref_path
                        self.bind(ref_path, item_id, 'identity-reference-photo', f'person:{pid}')
                license_name = str(reference.get('license') or '')
                if reference.get('kind') == 'observed_portrait' and license_name:
                    if 'BY-SA' in license_name.upper() and not portrait.get('derivative_license'):
                        self.find('sharealike_derivative_license_not_recorded', item_id,
                                  f'identity reference is {license_name}; no derivative_license is recorded for the cartoon')
                else:
                    self.find('reference_rights_not_recorded', item_id,
                              f"identity reference {reference.get('source_url') or '(none recorded)'} is not shipped; its licence is not recorded")
                item['rights'] = compact(rights)
                item['prompt'] = self.prompt_summary(item_id, pid, portrait.get('prompt_record'), portrait.get('asset') or '')
                item['generated_at'] = portrait.get('generated_at')
                review = portrait.get('review') if isinstance(portrait.get('review'), dict) else {}
                item['review'] = compact({k: review.get(k) for k in ('identity', 'likeness', 'era', 'visual', 'reviewer', 'reviewed_at')})
                if any(review.get(k) is not True for k in ('identity', 'likeness', 'era', 'visual')):
                    self.find('recorded_review_incomplete', item_id, 'identity/likeness/era/visual are not all recorded true')
                if not countries:
                    self.find('country_unbound', item_id, f'{pid} has no sourced party term or office observation in the production '
                              'inventory; the country is unknown and no recorded role displays this cartoon')
                item['life'] = compact({'born': bound_text((known or {}).get('born')), 'died': bound_text((known or {}).get('died'))})
                item['required_windows'] = [{'from': a.isoformat(), 'to': b.isoformat()} for a, b in windows]
                person_items.append(item)
            spans.sort()
            for previous, current in zip(spans, spans[1:]):
                if current[0] < previous[1]:
                    self.find('overlapping_intervals', current[2], f'overlaps {previous[2]}; selection would be ambiguous')
            gaps = subtract(windows, [(a, b) for a, b, _ in spans])
            for item in person_items:
                item['coverage_gaps'] = [{'from': a.isoformat(), 'to': b.isoformat()} for a, b in gaps]
            if gaps and person_items:
                self.find('coverage_gap', person_items[0]['id'], f'{sum((b - a).days for a, b in gaps)} days of sourced art windows are not covered: '
                          + '; '.join(interval_text(a.isoformat(), b.isoformat()) for a, b in gaps))
            self.items.extend(self.lean(i) for i in person_items)
        for pid in sorted(self.production):
            prod = self.production[pid]
            windows = [w for w in prod.get('required_art_windows', []) if isinstance(w, dict)]
            if pid in covered or prod.get('period_kind', 'historical') != 'historical' or not windows:
                continue
            item_id, known = f'missing:{pid}', self.registry.get(pid)
            self.items.append(compact({
                'id': item_id, 'collection': 'missing', 'name': prod.get('name') or pid,
                'identity': {'key': f'person:{pid}', 'person_id': pid, 'status': 'known' if known else 'unknown'},
                'countries': self.countries_for(pid), 'record': f'/people/{self.production_index[pid]}',
                'required_windows': [{'from': w.get('from'), 'to': w.get('to')} for w in windows],
                'reasons': sorted({(r.get('kind'), r.get('nation'), r.get('party')) for r in prod.get('reasons', [])}, key=str),
                'pending_art_jobs': len(prod.get('pending_art_job_ids', []))}))
            self.find('missing_art', item_id, f'no cartoon for {len(windows)} sourced art window(s)')
            if known is None:
                self.find('unknown_identity', item_id, f'{pid} is not an authored person ID in {REGISTRY}')

    def fictional(self):
        people = self.data['fictional_portraits'].get('people')
        if not isinstance(people, dict):
            raise SystemExit(f'{FICTIONAL_MANIFEST} requires a people object')
        catalog = {c.get('person_id'): c for c in self.data['fictional_catalog'].get('candidates', []) if isinstance(c, dict)}
        historical_art = [p for person in self.data['historical_portraits'].get('people', {}).values()
                          if isinstance(person, dict) for p in person.get('portraits', []) if isinstance(p, dict)]
        historical_hashes = {p.get('sha256') for p in historical_art}
        historical_assets = {p.get('asset') for p in historical_art}
        for pid in sorted(people):
            person = people[pid] if isinstance(people[pid], dict) else {}
            candidate, spans = catalog.get(pid), []
            for index, portrait in enumerate(person.get('portraits') or []):
                item_id = f'fictional:{pid}#{index}'
                if not isinstance(portrait, dict):
                    self.find('invalid_interval', item_id, 'portrait record is not an object')
                    continue
                nation = (candidate or {}).get('nation')
                item = {'id': item_id, 'collection': 'fictional', 'name': person.get('name') or pid,
                        'identity': {'key': f'fictional:{pid}', 'person_id': pid, 'status': 'known' if candidate else 'unknown'},
                        'countries': [nation] if nation else [], 'record': f'/people/{pid}/portraits/{index}'}
                if candidate is None or candidate.get('origin') != 'fictional_successor' or not pid.startswith('fictional_'):
                    self.find('unknown_identity', item_id, f'{pid} is not a fictional_successor in {CATALOG}')
                elif person.get('name') != candidate.get('name') or person.get('appearance_seed') != candidate.get('appearance_seed'):
                    self.find('name_mismatch', item_id, 'name or appearance seed differs from the fictional catalogue')
                if pid in self.registry:
                    self.find('fictional_identity_collision', item_id, f'{pid} is also a historical registry identity')
                design = portrait.get('design_source') if isinstance(portrait.get('design_source'), dict) else {}
                if design.get('person_id') != pid:
                    self.find('identity_binding_mismatch', item_id, f"design_source.person_id is {design.get('person_id')!r}, record is {pid}")
                asset = self.check_asset(item_id, portrait.get('asset'), portrait, (PERSON_ART,))
                if asset:
                    item['asset'] = item['card'] = asset
                    self.bind(asset, item_id, 'cartoon-master', f'fictional:{pid}')
                    if asset in historical_assets or self.file_facts(asset).get('sha256') in historical_hashes:
                        self.find('fictional_identity_collision', item_id, 'the image is also registered to a historical person')
                item['interval'] = self.interval(item_id, portrait, FICTIONAL, 'fictional', 'error')
                if item['interval']['valid']:
                    spans.append((iso_day(portrait['from']), iso_day(portrait['to']) if portrait['to'] else date.max, item_id))
                item['rights'] = compact({'artwork_license': portrait.get('license'), 'identity_reference': 'none (original fictional design)'})
                item['prompt'] = self.prompt_summary(item_id, pid, portrait.get('prompt_record'), portrait.get('asset') or '')
                item['generated_at'] = portrait.get('generated_at')
                review = portrait.get('review') if isinstance(portrait.get('review'), dict) else {}
                item['review'] = compact({k: review.get(k) for k in ('design', 'visual', 'reviewer', 'reviewed_at')})
                if any(review.get(k) is not True for k in ('design', 'visual')):
                    self.find('recorded_review_incomplete', item_id, 'design/visual are not both recorded true')
                if candidate:
                    item['fiction'] = compact({k: candidate.get(k) for k in ('party', 'profile_id', 'editorial_status')})
                self.items.append(self.lean(item))
            spans.sort()
            for previous, current in zip(spans, spans[1:]):
                if current[0] < previous[1]:
                    self.find('overlapping_intervals', current[2], f'overlaps {previous[2]}; selection would be ambiguous')

    def check_display(self, record: dict) -> bool:
        """Integrity of one display-derivative record; True when it can serve as a card image."""
        try:
            source, derived = safe_rel(record.get('source')), safe_rel(record.get('display'), (DISPLAY_ART,))
        except ValueError as error:
            self.find('unsafe_path', None, f"display manifest entry {record.get('display')!r}: {error}")
            return False
        src, out = self.file_facts(source), self.file_facts(derived)
        if not out['exists']:
            self.find('missing_file', None, f'display derivative {derived} does not exist', derived)
            return False
        if 'error' in out:
            self.find('unreadable_image', None, f"{derived}: {out['error']}", derived)
            return False
        match = DISPLAY_NAME_RE.fullmatch(PurePosixPath(derived).name)
        if not match or not out['sha256'].startswith(match['hex']):
            self.find('content_address_mismatch', None, f'{derived}: filename hash is not the leading 16 hex of its SHA-256', derived)
        if [out.get('width'), out.get('height')] != record.get('display_size') or out['bytes'] != record.get('display_bytes'):
            self.find('display_derivative_stale', None, f'{derived}: recorded display size/bytes differ from the file', derived)
        if src['exists'] and (src.get('sha256') != record.get('source_sha256') or src.get('bytes') != record.get('source_bytes')
                              or [src.get('width'), src.get('height')] != record.get('source_size')):
            self.find('display_derivative_stale', None, f'{derived}: derived from {source} bytes that are no longer current', derived)
        return True

    def selector(self):
        figures = self.data['selector_figures'].get('nations')
        if not isinstance(figures, dict):
            raise SystemExit(f'{SELECTOR_MANIFEST} requires a nations object')
        display = {}
        for record in self.data['display_derivatives'] if isinstance(self.data['display_derivatives'], list) else []:
            if isinstance(record, dict) and self.check_display(record):
                display[record['source']] = record
        for nation in sorted(figures):
            spec = figures[nation]
            if not isinstance(spec, dict):
                continue
            item_id = f'selector:{nation}'
            identity = f"figure:{spec['wikidata']}" if spec.get('wikidata') else f"figure:name:{str(spec.get('canonical_lookup', '')).casefold()}"
            item = {'id': item_id, 'collection': 'selector', 'name': spec.get('figure') or nation,
                    'identity': compact({'key': identity, 'wikidata': spec.get('wikidata'), 'status': 'known'}),
                    'countries': [nation], 'record': f'/nations/{nation}', 'years': spec.get('years')}
            if nation not in self.nation_names:
                self.find('unknown_identity', item_id, f'{nation} is not a NationId in {PRODUCTION}')
            art = spec.get('leader_art') if isinstance(spec.get('leader_art'), dict) else None
            photo = spec.get('portrait') if isinstance(spec.get('portrait'), dict) else None
            rights = {'identity_reference': 'archival photograph' if photo else 'none (text-led interpretation)',
                      'reference_license': (photo or {}).get('license')}
            if photo:
                rel = self.check_asset(item_id, f"{SELECTOR_PHOTOS}/{photo.get('asset')}", photo, (SELECTOR_PHOTOS,))
                match = PHOTO_NAME_RE.fullmatch(str(photo.get('asset')))
                facts = self.file_facts(rel) if rel else {}
                if facts.get('sha256') and (not match or match['nation'] != nation or not facts['sha256'].startswith(match['hex'])):
                    self.find('content_address_mismatch', item_id, f'{rel}: expected <{nation}>-<hash prefix>.webp', rel)
                if rel:
                    rights['reference_file'] = rel
                    self.bind(rel, item_id, 'identity-reference-photo', identity)
                    if rel in display:
                        self.bind(display[rel]['display'], item_id, 'identity-photo-display-derivative', identity)
                if 'automated' in str(photo.get('review', '')).lower():
                    self.find('identity_photo_selected_automatically', item_id, f"identity photograph review: {photo.get('review')!r}")
            if art:
                rel = self.check_asset(item_id, f"{SELECTOR_ART}/{art.get('asset')}", art, (SELECTOR_ART,))
                match = LEADER_NAME_RE.fullmatch(str(art.get('asset')))
                facts = self.file_facts(rel) if rel else {}
                if facts.get('sha256') and (not match or match['nation'] != nation or not facts['sha256'].startswith(match['hex'])):
                    self.find('content_address_mismatch', item_id, f'{rel}: expected <{nation}>-leader-<12 hash hex>.png', rel)
                if rel:
                    item['asset'] = item['card'] = rel
                    self.bind(rel, item_id, 'cartoon-master', identity)
                    if rel in display:
                        item['card'] = display[rel]['display']
                        self.bind(item['card'], item_id, 'cartoon-display-derivative', identity)
                    elif facts.get('exists'):
                        self.find('display_derivative_missing', item_id, f'{rel} has no display512 derivative in {DISPLAY_MANIFEST}', rel)
                if photo and art.get('identity_source_asset') != photo.get('asset'):
                    self.find('identity_binding_mismatch', item_id,
                              f"leader_art.identity_source_asset {art.get('identity_source_asset')!r} is not portrait.asset {photo.get('asset')!r}")
                if not photo:
                    if art.get('identity_source_wikidata') != spec.get('wikidata'):
                        self.find('identity_binding_mismatch', item_id,
                                  f"leader_art.identity_source_wikidata {art.get('identity_source_wikidata')!r} is not {spec.get('wikidata')!r}")
                    self.find('text_led_interpretation', item_id, str(art.get('identity_review') or spec.get('portrait_status') or 'no identity photograph recorded'))
                if 'BY-SA' in str(rights['reference_license'] or '').upper():
                    self.find('sharealike_derivative_license_not_recorded', item_id,
                              f"identity photograph is {rights['reference_license']}; the selector character art records no derivative licence")
                prompt = art.get('prompt_record')
                try:
                    item['prompt'] = text_digest(self.root, safe_rel(prompt, ('tools/avatars',)))
                    if not item['prompt']['exists']:
                        self.find('prompt_record_missing', item_id, f'{prompt} does not exist', prompt)
                except ValueError as error:
                    self.find('unsafe_path', item_id, f'prompt record {prompt!r}: {error}')
                item['generated_at'] = art.get('generated')
                item['style'] = art.get('style')
                item['review'] = compact({'text': art.get('review'), 'identity': art.get('identity_review')})
            item['rights'] = compact(rights)
            self.items.append(self.lean(item))

    def references(self):
        for ref in (self.data.get('reference_audit_uk') or {}).get('references', []):
            if not isinstance(ref, dict):
                continue
            rel = self.check_asset(None, ref.get('asset'), ref, (PERSON_ART,))
            if rel:
                self.bind(rel, None, 'identity-reference-photo', f"person:{ref.get('person_id')}")

    def unregistered(self):
        for rel in self.inventory:
            if self.bindings.get(rel):
                continue
            cartoon_root = (rel.startswith(PERSON_ART + '/') and '/references/' not in rel and rel.endswith('.png')) or rel.startswith(SELECTOR_ART + '/')
            item_id = f'file:{rel}' if cartoon_root else None
            if cartoon_root:
                self.items.append({'id': item_id, 'collection': 'unregistered', 'name': PurePosixPath(rel).name,
                                   'identity': {'key': None, 'status': 'unbound'}, 'asset': rel})
            self.find('unbound_file', item_id, f'{rel} is tracked in an art root but no manifest or reference audit binds it', rel)

    def production_freshness(self):
        recorded = self.data['production_inventory'].get('input_hashes') or {}
        current = {i['path']: i['sha256'] for i in self.inputs}
        for path in (REGISTRY, HISTORICAL_MANIFEST, FICTIONAL_MANIFEST):
            if path in recorded and path in current and recorded[path] != current[path]:
                self.find('production_inventory_stale', None, f'{PRODUCTION} recorded {path} as {recorded[path][:12]}…, current is '
                          f'{current[path][:12]}…; country bindings and art windows may be stale', path)

    def duplicates(self) -> list[dict]:
        by_hash = defaultdict(list)
        for rel in sorted(set(self.inventory) | set(self.bindings)):
            sha = self.file_facts(rel).get('sha256')
            if sha:
                by_hash[sha].append(rel)
        groups = []
        for sha in sorted(by_hash):
            members = by_hash[sha]
            identities = sorted({b['identity'] for rel in members for b in self.bindings.get(rel, []) if b['identity'] and b['role'] in IDENTITY_ROLES})
            items = sorted({b['item'] for rel in members for b in self.bindings.get(rel, []) if b['item']})
            if len(members) > 1 or len(identities) > 1:
                groups.append({'sha256': sha, 'paths': members, 'identities': identities, 'items': items, 'different_identities': len(identities) > 1})
            if len(identities) > 1:
                message = f"{len(members)} file(s) with SHA-256 {sha[:12]}… are bound to {', '.join(identities)}; a review finding, not proof of a wrong identity"
                for item in items or [None]:
                    self.find('duplicate_content_different_identities', item, message, members[0])
        for rel in sorted(self.bindings):
            identities = sorted({b['identity'] for b in self.bindings[rel] if b['identity'] and b['role'] in IDENTITY_ROLES})
            if len(identities) > 1:
                for item in sorted({b['item'] for b in self.bindings[rel] if b['item']}) or [None]:
                    self.find('duplicate_asset_path_different_identities', item, f"{rel} is bound to {', '.join(identities)}", rel)
        return groups

    def visual_sample(self) -> dict | None:
        sample = self.data.get('visual_review_sample')
        if sample is None:
            return None
        where = VISUAL_REVIEW.as_posix()
        if sample.get('format') != SAMPLE_FORMAT:
            raise SystemExit(f'{where}: format must be {SAMPLE_FORMAT}')
        if any(key in sample for key in FORBIDDEN_SAMPLE_KEYS):
            raise SystemExit(f'{where}: approval/acceptance fields are not allowed in a preparation review')
        by_id = {item['id']: item for item in self.items}
        entries = []
        for position, entry in enumerate(sample.get('entries', [])):
            at = f'{where} entry {position}'
            if entry.get('decision') not in ALLOWED_DECISIONS:
                raise SystemExit(f"{at}: decision {entry.get('decision')!r} is not one of {ALLOWED_DECISIONS}; "
                                 'this packet records visual notes, never approval or acceptance')
            if any(key in entry for key in FORBIDDEN_SAMPLE_KEYS):
                raise SystemExit(f'{at}: approval/acceptance fields are not allowed in a preparation review')
            fixes = entry.get('fixes')
            if not isinstance(fixes, list) or not fixes or not all(text(f) for f in fixes):
                raise SystemExit(f'{at}: at least one specific fix is required')
            item = by_id.get(entry.get('item'))
            current = self.file_facts(item['asset']).get('sha256') if item and item.get('asset') else None
            matches = bool(current) and current == entry.get('sha256')
            if item is None:
                self.find('visual_review_unknown_item', None, f"{at} names {entry.get('item')!r}")
            elif not matches:
                self.find('visual_review_stale', item['id'], f"{at} reviewed {str(entry.get('sha256'))[:12]}…, current file is {str(current)[:12]}…")
            if item is not None:
                item['visual_review'] = {'decision': entry['decision'], 'fixes': len(fixes), 'current_sha256_matches': matches}
            entries.append({**entry, 'current_sha256_matches': matches})
        return {k: sample.get(k) for k in ('format', 'reviewer', 'reviewed_on', 'statement', 'method', 'sizes_viewed',
                                           'cross_sample_observations')} | {'entries': entries}

    def style_references(self) -> list[dict]:
        refs = []
        style = self.data['production_inventory'].get('style') or {}
        try:
            anchor = safe_rel(style.get('anchor'), (PERSON_ART,))
        except ValueError:
            anchor = None
        if anchor:
            refs.append({'role': 'approved person-cartoon style anchor (production inventory)', 'path': anchor,
                         'recorded_sha256': style.get('anchor_sha256'), 'sha256': self.file_facts(anchor).get('sha256'),
                         'approval': style.get('user_style_approval')})
        counts = Counter()
        for spec in (self.data['selector_figures'].get('nations') or {}).values():
            prompt = ((spec.get('leader_art') or {}) if isinstance(spec, dict) else {}).get('prompt_record')
            try:
                prompt = safe_rel(prompt, ('tools/avatars',))
            except ValueError:
                continue
            if (self.root / prompt).is_file():
                match = STYLE_LINE_RE.search((self.root / prompt).read_text(encoding='utf-8-sig'))
                if match:
                    counts[match.group(1)] += 1
        if counts:
            path, count = sorted(counts.items(), key=lambda kv: (-kv[1], kv[0]))[0]
            try:
                sha = self.file_facts(safe_rel(path, (SELECTOR_ART,))).get('sha256')
            except ValueError:
                sha = None
            refs.append({'role': 'country-selector style reference (most cited "Style reference" in selector prompt records)',
                         'path': path, 'cited_by_prompt_records': count, 'sha256': sha})
        person = (((self.data.get('s10c_reviewed_reference') or {}).get('portrait_scope')) or {}).get('person_id')
        if person:
            item = next((i for i in self.items if i['id'].startswith(f'historical:{person}#')), {})
            refs.append({'role': 'reviewed reference cartoon (S10.c Tupou IV review)', 'path': item.get('asset'),
                         'scope': self.data['s10c_reviewed_reference']['portrait_scope'],
                         'review_record': 'docs/campaign-certification/S10/c/README.md'})
        for ref in refs:
            for item in self.items:
                if ref.get('path') and item.get('asset') == ref['path']:
                    ref['item'] = item['id']
                    item.setdefault('style_reference', []).append(ref['role'])
        return refs

    # -- assembly
    def build(self) -> dict:
        self.historical()
        self.fictional()
        self.selector()
        self.references()
        self.unregistered()
        self.production_freshness()
        groups = self.duplicates()
        sample = self.visual_sample()
        style_refs = self.style_references()
        order = {c: n for n, c in enumerate(COLLECTIONS)}

        def sort_key(item):
            countries = item.get('countries') or []
            country = self.nation_names.get(countries[0], countries[0]) if countries else '\uffff'
            return (country.casefold(), order[item['collection']], str(item['name']).casefold(), item['id'])

        self.items.sort(key=sort_key)
        position = {item['id']: n for n, item in enumerate(self.items)}
        self.findings.sort(key=lambda f: (position.get(f['item'], len(position)), SEVERITIES.index(f['severity']), f['code'], f['path'] or '', f['message']))
        seen = Counter()
        for finding in self.findings:
            base = f"{finding['code']}@{finding['item'] or finding['path'] or 'repository'}"
            seen[base] += 1
            finding['id'] = base if seen[base] == 1 else f'{base}#{seen[base]}'
        by_item = defaultdict(list)
        for finding in self.findings:
            by_item[finding['item']].append(finding)
        self.findings = [compact(f) for f in self.findings]
        for item in self.items:
            item['labels'] = self.labels(item, by_item.get(item['id'], []))
        files = []
        for rel in sorted(set(self.inventory) | set(self.bindings)):
            bindings = sorted({(b['role'], b['identity'], b['item']) for b in self.bindings.get(rel, [])}, key=lambda b: tuple(x or '' for x in b))
            record = {**self.file_facts(rel), 'bindings': [list(b) for b in bindings]}
            if rel not in self.inventory:
                record['tracked'] = False
            files.append(compact(record))
        used = sorted({c for item in self.items for c in item.get('countries', [])})
        return {
            'format': FORMAT, 'task': TASK, 'statement': STATEMENT,
            'generated_by': 'tools/avatars/cartoon_review.py',
            'regenerate': 'python -X utf8 tools/avatars/cartoon_review.py',
            'check': 'python -X utf8 tools/avatars/cartoon_review.py --check',
            'date_convention': 'from inclusive, to exclusive',
            'historical_period': {'from': HISTORICAL[0], 'to': HISTORICAL[1]},
            'fictional_period': {'from': FICTIONAL[0], 'to': FICTIONAL[1]},
            'file_inventory': self.inventory_mode,
            'hash_note': ('JSON input bytes and SHA-256 use explicit utf8-lf scope: only CRLF is replaced with LF; '
                          'all other bytes are retained. The browser verifies the same scope and separately records '
                          'received raw byte counts and hashes. Images remain hashed as exact raw bytes. Prompt '
                          'records use CRLF-to-LF normalization (sha256_lf). Original evidence files are never rewritten.'),
            'binding_fields': ['role', 'identity', 'item'],
            'collections': COLLECTION_SOURCES,
            'nations': {n: self.nation_names.get(n, n) for n in used},
            'inputs': self.inputs,
            'style_references': style_refs,
            'summary': self.summary(files, groups),
            'codes': {code: {'severity': sev, 'meaning': meaning} for code, (sev, meaning) in sorted(CODE_TEXT.items())},
            'visual_review_sample': sample,
            'items': self.items,
            'findings': self.findings,
            'duplicates': groups,
            'files': files,
        }

    @staticmethod
    def labels(item: dict, findings: list[dict]) -> list[str]:
        codes = {f['code'] for f in findings}
        labels = ['missing-art' if item['collection'] == 'missing' else item['collection']]
        if item.get('style_reference'):
            labels.append('style-reference')
        if 'missing_file' in codes:
            labels.append('file-missing')
        if item['identity'].get('status') in ('unknown', 'unbound') or 'unknown_identity' in codes:
            labels.append('unknown-identity')
        if not item.get('countries'):
            labels.append('country-unknown')
        if codes & {'duplicate_content_different_identities', 'duplicate_asset_path_different_identities'}:
            labels.append('duplicate')
        if codes & {'invalid_interval', 'overlapping_intervals', 'interval_outside_period', 'interval_after_death'}:
            labels.append('interval-issue')
        if 'coverage_gap' in codes:
            labels.append('coverage-gap')
        if codes & {'sharealike_derivative_license_not_recorded', 'reference_rights_not_recorded', 'identity_reference_generated'}:
            labels.append('rights-gap')
        if item.get('visual_review'):
            labels.append('sample-reviewed')
        if any(f['severity'] == 'error' for f in findings):
            labels.append('integrity-error')
        return labels

    def summary(self, files: list[dict], groups: list[dict]) -> dict:
        by_collection = Counter(item['collection'] for item in self.items)
        by_code = Counter(f['code'] for f in self.findings)
        by_severity = Counter(f['severity'] for f in self.findings)
        roots = Counter(next((r for r in ART_ROOTS if f['path'].startswith(r + '/')), 'other') for f in files)
        return {
            'items': {c: by_collection.get(c, 0) for c in COLLECTIONS},
            'cartoon_items_with_files': sum(1 for i in self.items if i['collection'] in ('historical', 'fictional', 'selector') and i.get('asset')),
            'files': {'total': len(files), 'existing': sum(1 for f in files if f['exists']),
                      'by_root': {r: roots.get(r, 0) for r in ART_ROOTS}},
            'duplicate_groups': len(groups),
            'duplicate_groups_with_different_identities': sum(1 for g in groups if g['different_identities']),
            'findings_by_severity': {s: by_severity.get(s, 0) for s in SEVERITIES},
            'findings_by_code': {code: by_code[code] for code in sorted(by_code)},
            'countries_with_items': len({c for i in self.items for c in i.get('countries', [])}),
            'visual_review_entries': len((self.data.get('visual_review_sample') or {}).get('entries', [])),
            'approved_by_this_export': 0,
        }


def build(root: Path = ROOT) -> dict:
    return Builder(Path(root)).build()


# ---------------------------------------------------------------- markdown

def md(value) -> str:
    return str('—' if value is None else value).replace('|', '\\|').replace('\n', ' ')


def render(export: dict) -> str:
    s, items = export['summary'], {i['id']: i for i in export['items']}
    findings = export['findings']
    name = lambda f: items[f['item']]['name'] if f.get('item') in items else (f.get('path') or 'repository')
    lines = ['# Cartoon review export (CLAUDE-C03-REVIEW-01)', '', f"> {export['statement']}", '',
             f"Generated by `{export['generated_by']}` from the pinned inputs below. Regenerate with `{export['regenerate']}`; "
             f"verify with `{export['check']}` (exit 1 when stale). The reviewer `tools/ui/cartoon-review/index.html` presents "
             'this export and re-verifies the input hashes in the browser before showing anything.', '',
             f"Dates use {export['date_convention']}. Historical period "
             f"{interval_text(export['historical_period']['from'], export['historical_period']['to'])}; fictional period "
             f"{interval_text(export['fictional_period']['from'], export['fictional_period']['to'])}. File inventory: {export['file_inventory']}. "
             f"{export['hash_note']}", '', '## Inputs', '', '| Role | Path | Bytes | SHA-256 |', '|---|---|---:|---|']
    lines += [f"| {i['role']} | `{i['path']}` | {i['bytes']} | `{i['sha256']}` |" for i in export['inputs']]
    lines += ['', '## Collections', '', '| Collection | Items | Source |', '|---|---:|---|']
    lines += [f"| {c} | {n} | {md(export['collections'][c]['label'])} |" for c, n in s['items'].items()]
    lines += ['', f"{s['files']['total']} image files inventoried, {s['files']['existing']} present: "
              + ', '.join(f'`{root}` {n}' for root, n in s['files']['by_root'].items()) + '.', '', '## Style references', '']
    for ref in export['style_references']:
        lines.append(f"- **{ref['role']}**: `{ref.get('path')}`" + (f" (cited by {ref['cited_by_prompt_records']} prompt records)" if ref.get('cited_by_prompt_records') else ''))
    lines += ['', '## Automated findings by code', '',
              'Severity: **error** = integrity failure; **warning** = needs a decision; **notice** = recorded gap. None of these is visual approval.', '',
              '| Code | Severity | Count | Meaning |', '|---|---|---:|---|']
    lines += [f"| `{c}` | {export['codes'][c]['severity']} | {n} | {md(export['codes'][c]['meaning'])} |" for c, n in s['findings_by_code'].items()]
    lines += ['', '## Integrity errors', '']
    lines += [f"- `{f['id']}`: {md(f['message'])}" for f in findings if f['severity'] == 'error'] or [
        'None: every bound file exists, has a readable header and matches its recorded hash and dimensions.']
    lines += ['', '## Duplicate images bound to different identities', '']
    different = [g for g in export['duplicates'] if g['different_identities']]
    if different:
        lines += ['| SHA-256 | Files | Identities |', '|---|---|---|']
        lines += [f"| `{g['sha256'][:16]}…` | {', '.join('`' + p + '`' for p in g['paths'])} | {', '.join(g['identities'])} |" for g in different]
    else:
        lines.append(f"None found among {s['files']['existing']} files (exact SHA-256 comparison; {len(export['duplicates'])} duplicate groups in total).")
    lines += ['', '## Warnings', '']
    grouped = defaultdict(list)
    for f in findings:
        if f['severity'] == 'warning':
            grouped[f['code']].append(f)
    for code in sorted(grouped):
        names = [name(f) for f in grouped[code]]
        lines.append(f"- **`{code}`** ({len(names)}): " + ', '.join(names[:20]) + (f', … ({len(names) - 20} more)' if len(names) > 20 else ''))
    if not grouped:
        lines.append('None.')
    lines += ['', '## Appearance intervals and coverage', '']
    for code in ('invalid_interval', 'overlapping_intervals', 'interval_outside_period', 'interval_after_death', 'coverage_gap'):
        rows = [f for f in findings if f['code'] == code]
        lines.append(f'- **`{code}`** ({len(rows)})' + (':' if rows else ''))
        lines += [f'  - {md(name(f))}: {md(f["message"])}' for f in rows]
    missing = [i for i in export['items'] if i['collection'] == 'missing']
    per_country = Counter(export['nations'].get(c, c) for i in missing for c in (i.get('countries') or ['(country unknown)']))
    several = sorted(((c, n) for c, n in per_country.items() if n > 1), key=lambda kv: (-kv[1], kv[0]))
    singles = sum(1 for n in per_country.values() if n == 1)
    lines += [f"- **Missing art** ({len(missing)} people with sourced art windows and no cartoon), by country: "
              + ', '.join(f'{c} {n}' for c, n in several) + (f'; one each in {singles} other countries' if singles else '') + '.', '',
              '## Source and rights gaps', '']
    for code in ('sharealike_derivative_license_not_recorded', 'reference_rights_not_recorded', 'identity_reference_generated',
                 'text_led_interpretation', 'identity_photo_selected_automatically'):
        names = sorted({name(f) for f in findings if f['code'] == code})
        lines.append(f"- **`{code}`** ({len(names)}): " + ', '.join(names[:24]) + (f', … ({len(names) - 24} more)' if len(names) > 24 else ''))
    lines += ['', '## Recorded review decisions (existing manifests)', '']
    recorded = Counter()
    for item in export['items']:
        review = item.get('review') or {}
        if item['collection'] == 'historical':
            recorded['historical: identity, likeness, era and visual all recorded true'
                     if all(review.get(k) is True for k in ('identity', 'likeness', 'era', 'visual')) else 'historical: incomplete'] += 1
        elif item['collection'] == 'fictional':
            recorded['fictional: design and visual recorded true' if all(review.get(k) is True for k in ('design', 'visual')) else 'fictional: incomplete'] += 1
        elif item['collection'] == 'selector':
            recorded['selector: free-text production review recorded' if review.get('text') else 'selector: none recorded'] += 1
    lines += [f'- {k}: {v}' for k, v in sorted(recorded.items())]
    lines += ['', 'These are decisions recorded by earlier production passes. This export reproduces them; it does not re-decide or endorse them.', '',
              '## Visual review sample (this packet)', '']
    sample = export.get('visual_review_sample')
    if sample:
        lines += [f"> {sample['statement']}", '', f"Reviewer: {sample['reviewer']}; reviewed on {sample['reviewed_on']}.", '', md(sample['method']), '']
        if sample.get('cross_sample_observations'):
            lines += ['Across the sample:', ''] + [f'- {md(o)}' for o in sample['cross_sample_observations']] + ['']
        lines += ['| Item | Decision | File unchanged | Specific fixes |', '|---|---|---|---|']
        for entry in sample['entries']:
            lines.append(f"| {md(items.get(entry['item'], {}).get('name', entry['item']))} (`{entry['item']}`) | {entry['decision']} | "
                         f"{'yes' if entry['current_sha256_matches'] else 'NO'} | " + '<br>'.join(md(f) for f in entry['fixes']) + ' |')
    else:
        lines.append('No visual review sample recorded.')
    lines += ['', '## Limitations', '',
              '- Dimensions come from file headers; pixel data is not decoded, so corruption after a valid header is not detected here.',
              '- Duplicates are exact SHA-256 matches; re-encoded, resized or cropped near-duplicates are not detected.',
              '- Identity references that are not shipped (most historical references) cannot be compared by this tool; their rights are reported as gaps, not judged.',
              '- The file inventory is the git index when available; untracked files are ignored.',
              '- Countries and art windows come from the checked-in production inventory, whose recorded input hashes are compared with the current manifests and registry.',
              '- Automated findings never approve artwork; the visual review sample records proposals only.', '']
    return '\n'.join(lines)


# ---------------------------------------------------------------- io

def outputs(root: Path = ROOT) -> dict[Path, str]:
    export = build(root)
    return {EXPORT_JSON: canonical(export), EXPORT_MD: render(export)}


def stale(files: dict[Path, str], root: Path = ROOT) -> list[str]:
    """Generated paths whose checked-in content differs (newline-normalized) from a fresh build."""
    return [path.as_posix() for path, content in files.items()
            if not (root / path).is_file() or (root / path).read_text(encoding='utf-8') != content]


def write(files: dict[Path, str], root: Path = ROOT):
    for path, content in files.items():
        (root / path).parent.mkdir(parents=True, exist_ok=True)
        (root / path).write_text(content, encoding='utf-8', newline='\n')


def main(argv=None) -> int:
    parser = argparse.ArgumentParser(description=__doc__, formatter_class=argparse.RawDescriptionHelpFormatter)
    parser.add_argument('--check', action='store_true', help='compare the committed export without writing; exit 1 when stale')
    parser.add_argument('--root', type=Path, default=ROOT, help=argparse.SUPPRESS)
    args = parser.parse_args(argv)
    files = outputs(args.root)
    if args.check:
        differing = stale(files, args.root)
        if differing:
            print('Cartoon review export is stale; regenerate with `python -X utf8 tools/avatars/cartoon_review.py`: '
                  + ', '.join(differing), file=sys.stderr)
            return 1
    else:
        write(files, args.root)
    summary = json.loads(files[EXPORT_JSON])['summary']
    print(json.dumps({'check': args.check, **{k: summary[k] for k in ('items', 'findings_by_severity',
          'duplicate_groups_with_different_identities', 'visual_review_entries', 'approved_by_this_export')}}, indent=1))
    return 0


if __name__ == '__main__':
    sys.exit(main())
