#!/usr/bin/env python3
"""Package S08 evidence without awarding completion or rerunning qualification.

Usage: python collect-s08-evidence.py PIN [--apply]
Default: read-only inventory/dry-run. --apply refuses an existing destination.
Repeat --performance-dir, --include or --failed-attempt for additional S08 roots.
No builds, browsers, servers, tests, save operations, or original-file changes.
"""
import argparse
import datetime
import hashlib
import json
import math
import os
from pathlib import Path, PurePosixPath
import re
import shutil
import subprocess
import sys
import zipfile

BASE = Path(__file__).resolve().parent
S08_BASE = '68e863055ee6ddf6c9796547995dccf455557edd'
TEXT_AND_IMAGES = {'.json', '.jsonl', '.log', '.txt', '.png', '.jpg', '.webp', '.html',
    '.exit', '.py', '.cjs', '.js', '.css', '.patch', '.ps1', '.md', '.csv', '.rs', '.toml', '.yaml', '.yml', '.before'}
BINARY = {'.exe', '.dll', '.pdb', '.so', '.a', '.lib', '.obj', '.o', '.rlib', '.rmeta', '.wasm'}
SKIP_DIRS = {'node_modules', '__pycache__', '.git', 'target', 'integration-target'}
CHUNK_BYTES = 64 * 1024 * 1024
ZIP_LIMIT = 90 * 1024 * 1024
COMPRESS_RECORD_BYTES = 8 * 1024 * 1024

def require(ok, why):
    if not ok: raise ValueError(why)
def stamp(): return datetime.datetime.now(datetime.timezone.utc).isoformat()
def sha(path):
    with Path(path).open('rb') as handle: return hashlib.file_digest(handle, 'sha256').hexdigest()
def read(path): return json.loads(Path(path).read_text(encoding='utf-8-sig'))
def write(path, value):
    with Path(path).open('x', encoding='utf-8') as handle:
        json.dump(value, handle, indent=2, ensure_ascii=False); handle.write('\n')
def git(repo, *args):
    return subprocess.check_output(['git', '-c', 'core.longpaths=true', *args], cwd=repo, stderr=subprocess.PIPE).decode().strip()
def safe_relative(value):
    path = PurePosixPath(value)
    require(not path.is_absolute() and '..' not in path.parts and ':' not in str(path), 'Unsafe package path: ' + str(path))
    return path.as_posix()
def under(path, parent): return path == parent or parent in path.parents
def is_link(path):
    return path.is_symlink() or (hasattr(path, 'is_junction') and path.is_junction())

def campaign_file(path, prefix):
    # Preserve actual exported campaign bytes; avoid parsing a 150 MB world just
    # to distinguish it from a small API observation.
    if path.name.endswith('.campaign.json') or 'saves' in [p.lower() for p in path.parts]: return True
    if path.name.lower() in {'save.json', 'save.json.bak'}: return True
    if path.suffix.lower() != '.json': return False
    text = prefix.decode('utf-8-sig', errors='ignore')
    return bool(re.search(r'"format"\s*:\s*"spheres-(?:[^"\n]*save|campaign)"', text)
        or re.search(r'"world"\s*:\s*\{\s*"(?:year|schema_version|nations)"', text))

def browser_revision(value):
    recorded = value.get('build_evidence', {})
    if isinstance(recorded, dict) and recorded.get('revision'): return recorded['revision']
    build = value.get('build', {})
    return build.get('revision') if isinstance(build, dict) else None

def s08_revision(repo, revision, pin):
    if not isinstance(revision, str) or not re.fullmatch('[0-9a-f]{40}', revision) or revision == S08_BASE: return False
    try:
        git(repo, 'merge-base', '--is-ancestor', S08_BASE, revision)
        git(repo, 'merge-base', '--is-ancestor', revision, pin)
        return True
    except subprocess.CalledProcessError: return False

def snapshot_allowed(path, root, kind):
    if kind == 'genuine_export': return path.name.endswith('.campaign.json')
    if kind == 'feature_performance': return path.name == 'purchased.campaign.json'
    if kind == 'supplier_browser': return path.name.startswith('s08-') and path.suffix.lower() == '.json'
    return False

def compress_exact(path, expected_hash, destination, kind):
    """Each ZIP contains <=64 MiB of original bytes, hence remains <90 MiB.

    Large files are reconstructed by concatenating extracted members in order.
    Both each member and the complete concatenation are verified before return.
    """
    directory = destination / ('campaign-archives' if kind == 'campaign' else 'large-records') / expected_hash
    directory.mkdir(parents=True, exist_ok=False)
    parts = []; overall = hashlib.sha256(); position = 0
    with path.open('rb') as original:
        while True:
            first = original.read(min(1024 * 1024, CHUNK_BYTES))
            if not first: break
            index = len(parts) + 1; target = directory / f'part-{index:04}.zip'
            member = f'bytes-{index:04}'; info = zipfile.ZipInfo(member, (1980, 1, 1, 0, 0, 0))
            info.compress_type = zipfile.ZIP_DEFLATED
            info.external_attr = 0o100644 << 16
            chunk_hash = hashlib.sha256(); size = 0
            with zipfile.ZipFile(target, 'x', compression=zipfile.ZIP_DEFLATED, compresslevel=6, allowZip64=True) as archive:
                with archive.open(info, 'w') as output:
                    data = first
                    while data:
                        output.write(data); chunk_hash.update(data); size += len(data)
                        if size == CHUNK_BYTES: break
                        data = original.read(min(1024 * 1024, CHUNK_BYTES - size))
            require(target.stat().st_size < ZIP_LIMIT, 'Stored ZIP exceeded the declared 90 MiB bound: ' + str(target))
            verified = hashlib.sha256(); extracted_bytes = 0
            with zipfile.ZipFile(target) as archive, archive.open(member) as extracted:
                while data := extracted.read(1024 * 1024):
                    verified.update(data); overall.update(data); extracted_bytes += len(data)
            require(extracted_bytes == size and verified.hexdigest() == chunk_hash.hexdigest(), 'ZIP extraction changed source bytes')
            parts.append({'file': target.relative_to(destination).as_posix(), 'member': member,
                'source_offset': position, 'source_bytes': size, 'source_sha256': verified.hexdigest(),
                'stored_bytes': target.stat().st_size, 'stored_sha256': sha(target)})
            position += size
    require(overall.hexdigest() == expected_hash and sha(path) == expected_hash, 'Source changed or ZIP reconstruction differs: ' + str(path))
    return {'kind': kind, 'encoding': 'ordered-zip-members', 'original_sha256': expected_hash,
        'original_bytes': position, 'parts': parts, 'reconstruction_verified': True}

EXTRACTOR = r'''#!/usr/bin/env python3
"""Restore one original compressed record from inventory.json into a NEW file.
Usage: python restore-record.py INVENTORY LOGICAL_PATH NEW_OUTPUT
"""
import hashlib,json,pathlib,sys,zipfile
inventory=pathlib.Path(sys.argv[1]).resolve(); data=json.loads(inventory.read_text(encoding='utf-8'))
row=next(r for r in data['source_files'] if r['logical_path']==sys.argv[2])
output=pathlib.Path(sys.argv[3]).resolve(); assert not output.exists(),'Refuse to overwrite output'
blob=data['compressed_objects'][row['storage']['object']]; total=hashlib.sha256(); size=0
with output.open('xb') as result:
 for part in blob['parts']:
  archive_path=(inventory.parent/part['file']).resolve()
  assert archive_path.is_relative_to(inventory.parent),'Unsafe archive path'
  with archive_path.open('rb') as stream: assert hashlib.file_digest(stream,'sha256').hexdigest()==part['stored_sha256']
  member_hash=hashlib.sha256(); member_bytes=0
  with zipfile.ZipFile(archive_path) as archive,archive.open(part['member']) as source:
   while chunk:=source.read(1024*1024):
    result.write(chunk);total.update(chunk);member_hash.update(chunk);size+=len(chunk);member_bytes+=len(chunk)
  assert member_bytes==part['source_bytes'] and member_hash.hexdigest()==part['source_sha256']
assert size==row['bytes'] and total.hexdigest()==row['sha256'],'Reconstruction differs'
print(str(output)+' verified SHA256 '+total.hexdigest())
'''

def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('candidate'); parser.add_argument('--repo', type=Path, default=BASE / 'integration')
    parser.add_argument('--performance-dir', type=Path, action='append', default=[])
    parser.add_argument('--include', type=Path, action='append', default=[], help='Explicit additional S08 raw record or directory')
    parser.add_argument('--failed-attempt', type=Path, action='append', default=[], help='Explicit failed S08 browser result.json, including failures before build identity')
    parser.add_argument('--browser-result', type=Path, action='append', default=[], help='Additional browser result with a verified S08 revision')
    parser.add_argument('--apply', action='store_true')
    args = parser.parse_args(); pin = args.candidate; repo = args.repo.resolve()
    require(re.fullmatch('[0-9a-f]{40}', pin), 'Use a complete lowercase candidate SHA')
    require(git(repo, 'rev-parse', 'HEAD') == pin, 'Checkout HEAD differs from requested runtime')
    destination = repo / 'docs/campaign-certification/S08/evidence'
    attributes = destination.parent / '.gitattributes'
    require(not destination.exists(), 'Refusing to overwrite existing evidence: ' + str(destination))
    if attributes.exists():
        require(re.search(rb'(?m)^evidence/\*\*\s+-text\s*\r?$', attributes.read_bytes()), 'Existing S08 attributes must preserve evidence/** bytes')
    roots = []; excluded_roots = []; browser_records = []; source_rows = {}; references = []
    def add(path, logical, kind='records'):
        path = Path(path).resolve(); require(path.exists() and not is_link(path), 'Missing or linked input: ' + str(path))
        require(not under(path, destination), 'Cannot collect the destination itself')
        roots.append((path, safe_relative(logical), kind))

    for path in sorted((BASE / 'evidence').glob('S08-*')):
        kind = 'genuine_export' if path.name.startswith('S08-genuine-supplier-') else ('feature_performance' if path.name.startswith('S08-feature-performance-') else 'records')
        add(path, path.name, kind)
    require(roots, 'No S08 evidence found')
    for path in args.performance_dir:
        require(path.name.startswith('S08-'), 'Additional performance directory must be explicitly S08-named')
        add(path, 'additional-performance/' + path.name)
    for path in args.include: add(path, 'additional/' + path.name)

    def add_browser(result_path, kind, explicit_failed=False):
        value = read(result_path); revision = browser_revision(value)
        if explicit_failed: require(value.get('passed') is not True, 'Explicit failed attempt is passed: ' + str(result_path))
        elif kind != 'supplier_browser' and not s08_revision(repo, revision, pin):
            excluded_roots.append({'path': str(result_path.parent), 'reason': 'General browser artifact does not identify an S08 revision', 'revision': revision}); return
        label = result_path.parent.name
        # The general runner uses one fixed artifacts/browser-ci directory.
        if label == 'browser-ci': label = 'current-general-browser'
        category = 'supplier' if kind == 'supplier_browser' else ('explicit-failed' if explicit_failed else 'general')
        logical = 'browser/' + category + '/' + label
        add(result_path.parent, logical, kind)
        browser_records.append({'source_result': str(result_path), 'logical_result': logical + '/' + result_path.name,
            'passed': value.get('passed'), 'revision': revision, 'same_final_candidate': revision == pin,
            'scope': 'Explicitly identified failed S08 attempt' if explicit_failed else 'Supplier-only S08 folder' if kind == 'supplier_browser' else 'Revision checked against S08 ancestry'})
        for screenshot in value.get('screenshots') or []:
            if isinstance(screenshot, str):
                relative = safe_relative(screenshot)
                require((result_path.parent / relative).is_file(), 'Referenced screenshot is missing: ' + relative)

    supplier = repo / 'artifacts/browser-supplier-imports-ci'
    if supplier.exists():
        for result_path in sorted(supplier.glob('*/result.json')): add_browser(result_path.resolve(), 'supplier_browser')
    general = repo / 'artifacts/browser-ci'
    if general.exists():
        for result_path in sorted(general.rglob('result.json')): add_browser(result_path.resolve(), 'general_browser')
    for path in args.browser_result: add_browser(path.resolve(), 'general_browser')
    for path in args.failed_attempt: add_browser(path.resolve(), 'supplier_browser' if 'supplier' in str(path).lower() else 'general_browser', True)
    for pattern in ('run-s08-*.py', 'accept-s08-*.py', 's08-*.cjs', 's08-*.py'):
        for path in sorted(BASE.glob(pattern)): add(path, 'runners/' + path.name)
    add(Path(__file__), 'runners/' + Path(__file__).name)
    for name in ('ci-supplier-imports.cjs', 'ci-browser.cjs', 'ci-integrated.cjs'):
        add(repo / 'tools/ui' / name, 'runners/tools/ui/' + name)
    add(repo / 'tools/campaign/run-lifetime-baseline.ps1', 'runners/tools/campaign/run-lifetime-baseline.ps1')
    # The Chrome fallback wrapper belongs to an earlier session but is an actual
    # S08 runner dependency; copy only this named source, never its old artifacts.
    if (BASE / 's05-installed-browser.cjs').exists(): add(BASE / 's05-installed-browser.cjs', 'runners/dependencies/s05-installed-browser.cjs')

    for root, logical, kind in roots:
        if root.is_file(): files = [(root, logical)]
        else:
            files = []
            for directory, directories, names in os.walk(root, followlinks=False):
                base = Path(directory)
                directories[:] = sorted(d for d in directories if d.lower() not in SKIP_DIRS and not is_link(base / d))
                for name in sorted(names):
                    path = base / name
                    require(not is_link(path), 'Linked evidence file: ' + str(path))
                    files.append((path, safe_relative(str(PurePosixPath(logical) / path.relative_to(root).as_posix()))))
        for path, target in files:
            with path.open('rb') as handle: prefix = handle.read(64 * 1024)
            row = {'original_path': str(path), 'logical_path': target, 'bytes': path.stat().st_size, 'sha256': sha(path)}
            suffix = path.suffix.lower(); campaign = campaign_file(path, prefix)
            reason = None
            if suffix in BINARY or prefix.startswith((b'MZ', b'\x7fELF')): reason = 'Compiled binary: hash-only provenance'
            elif campaign and not snapshot_allowed(path, root, kind): reason = 'Unrelated or unverified campaign copy: hash-only provenance; genuine S08 exporter and supplier-journey phases are retained separately'
            elif suffix not in TEXT_AND_IMAGES: reason = 'Unsupported or opaque archive: hash-only provenance'
            if reason:
                if not any(x['original_path'] == row['original_path'] for x in references): references.append(dict(row, reason=reason))
                continue
            row['category'] = 'campaign' if campaign else 'record'
            row['storage'] = {'kind': 'compressed', 'object': row['sha256']} if campaign or row['bytes'] > COMPRESS_RECORD_BYTES else {'kind': 'raw', 'file': target}
            if target in source_rows:
                require(source_rows[target]['sha256'] == row['sha256'], 'Conflicting destination: ' + target)
            else: source_rows[target] = row

    rows = sorted(source_rows.values(), key=lambda row: row['logical_path'])
    objects = {}
    for row in rows:
        if row['storage']['kind'] == 'compressed': objects.setdefault(row['sha256'], row)
    raw_rows = [row for row in rows if row['storage']['kind'] == 'raw']
    estimate = {'original_file_count': len(rows), 'original_bytes_including_identical_copies': sum(row['bytes'] for row in rows),
        'raw_copy_count': len(raw_rows), 'raw_copy_bytes': sum(row['bytes'] for row in raw_rows),
        'unique_compressed_objects': len(objects), 'unique_uncompressed_object_bytes': sum(row['bytes'] for row in objects.values()),
        'zip_parts': sum(math.ceil(row['bytes'] / CHUNK_BYTES) for row in objects.values()),
        'maximum_source_bytes_per_zip': CHUNK_BYTES, 'stored_zip_limit_bytes': ZIP_LIMIT,
        'estimate_note': 'Dry-run does not compress or estimate a compression ratio. Every part holds at most 64 MiB before compression; exact stored sizes are recorded on apply.',
        'external_hash_only_references': len(references)}
    if not args.apply:
        print(json.dumps({'mode': 'dry_run', 'candidate': pin, 'destination': str(destination), 'estimated_layout': estimate,
            'browser_records': browser_records, 'excluded_roots': excluded_roots}, indent=2)); return

    # All source paths/hashes are known before the first repository write. A
    # interrupted apply remains visible and is never automatically overwritten.
    destination.mkdir(parents=True, exist_ok=False)
    if not attributes.exists():
        with attributes.open('xb') as handle:
            handle.write(b'# Preserve original qualification evidence bytes, including line endings.\nevidence/** -text\n')
    compressed = {}
    for digest, row in sorted(objects.items()):
        compressed[digest] = compress_exact(Path(row['original_path']), digest, destination, row['category'])
    for row in raw_rows:
        original = Path(row['original_path']); target = destination / row['storage']['file']
        target.parent.mkdir(parents=True, exist_ok=True)
        with original.open('rb') as source_handle, target.open('xb') as output: shutil.copyfileobj(source_handle, output)
        require(sha(target) == row['sha256'] == sha(original), 'Raw evidence bytes changed: ' + str(original))
    (destination / 'restore-record.py').write_bytes(EXTRACTOR.encode('utf-8'))
    readme = ('# S08 raw evidence\n\nThis package records evidence only; it does not award campaign certification. '
        'Failed developmental attempts remain alongside the final-candidate records. See inventory.json for each source path, phase, byte count and SHA256.\n\n'
        'Small records and screenshots are exact byte copies. Large records and genuine supplier campaign snapshots are deduplicated only when the entire original SHA256 matches. '
        'Each compressed object is an ordered sequence of ZIP members, each containing at most 64 MiB of original bytes. Every ZIP is below 90 MiB. '
        'The collector verified each extracted member and their full concatenation against the original hashes.\n\n'
        'To restore a snapshot into a new file, use its logical_path from inventory.json:\n\n'
        '    python restore-record.py inventory.json "S08-genuine-supplier-LABEL/export/2130-purchased.campaign.json" "restored.campaign.json"\n\n'
        'The example path is illustrative; actual dates and filenames come from the inventory. The helper refuses an existing output and verifies every archive/member and the complete reconstructed source. '
        'No executable or unrelated legacy campaign bytes are included. Their original paths and hashes remain in external_references or the original qualification proof records. '
        'Current-run process samples and timings do not imply browser throughput.\n')
    (destination / 'README.md').write_bytes(readme.encode('utf-8'))
    manifest = [{'file': p.relative_to(destination).as_posix(), 'bytes': p.stat().st_size, 'sha256': sha(p)} for p in sorted(destination.rglob('*')) if p.is_file()]
    require(git(repo, 'rev-parse', 'HEAD') == pin, 'Candidate changed during collection')
    for row in rows: require(sha(row['original_path']) == row['sha256'], 'Original evidence changed during collection: ' + row['original_path'])
    inventory = {'format': 'spheres-s08-evidence-inventory', 'version': 1, 'session': 'S08',
        'status': 'evidence_collected_only', 'candidate': pin, 'collected_utc': stamp(),
        'collector_sha256': sha(__file__), 'source_files': rows, 'compressed_objects': compressed,
        'byte_preservation_attributes': {'path': '../.gitattributes', 'sha256': sha(attributes), 'rule': 'evidence/** -text'},
        'stored_files': manifest, 'external_references': references, 'browser_records': browser_records,
        'excluded_roots': excluded_roots, 'layout': estimate,
        'scope': 'No completion decision or external certification. Current-candidate passes, developmental failures and source-only audits retain their own recorded scope.',
        'inventory_note': 'This inventory excludes its own hash. inventory.sha256 hashes its final bytes. Raw files and compressed members retain their original line endings.'}
    write(destination / 'inventory.json', inventory)
    with (destination / 'inventory.sha256').open('x', encoding='ascii') as handle: handle.write(sha(destination / 'inventory.json') + '  inventory.json\n')
    print(json.dumps({'mode': 'applied', 'candidate': pin, 'destination': str(destination), 'source_files': len(rows),
        'stored_files': len(manifest) + 2, 'stored_bytes': sum(p.stat().st_size for p in destination.rglob('*') if p.is_file()),
        'compressed_objects': len(compressed), 'inventory_sha256': sha(destination / 'inventory.json')}, indent=2))

if __name__ == '__main__':
    try: main()
    except (ValueError, KeyError, OSError, subprocess.CalledProcessError) as error:
        print('S08 evidence collection refused: ' + str(error), file=sys.stderr); raise SystemExit(1)
