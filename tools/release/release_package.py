"""Deterministic portable packages and checkout-independent extraction checks.

Source documentation uses committed Git bytes. ZIP_STORED avoids cross-host
compressor variation. Rebuilding identical executables is a separate concern.
"""
from pathlib import Path, PurePosixPath
import argparse
import ctypes
import datetime
import hashlib
import json
import os
import re
import shutil
import stat
import subprocess
import sys
import tempfile
import time
import zipfile


class PackageError(ValueError):
    pass


REPOSITORY = 'https://github.com/ridgemerkley2-web/Spheres'
OPTIONAL_DOCS = ('CURRENT_ARCHITECTURE.md', 'DECISIONS.md', 'PLAYTEST.md', 'PLAYER_DECISIONS.md', 'MILITARY_OPERATIONS.md',
                 'CAMPAIGN_AIMS.md', 'SECTOR_PROFILES.md', 'TECH_REFERENCE_REPAIR.md', 'HEADLESS_BASELINE_2026-09-04.md',
                 'DAILY_CALIBRATION.md', 'PERFORMANCE.md', 'INVESTMENT_COMPLETION_AUDIT.md', 'ART_DIRECTION.md',
                 'MANUFACTURING.md', 'PROVINCE_ECONOMY.md')
REQUIRED_INPUTS = {
    'README.md': 'README.md', 'Cargo.lock': 'source/Cargo.lock',
    'spheres-web/ui/height-detail.json': 'attribution/height-detail.json',
    'spheres-web/ui/terrain-tiles/manifest.json': 'attribution/terrain-tiles/manifest.json',
    'spheres-web/ui/lake-surfaces.json': 'attribution/lake-surfaces.json',
    'tools/terrain/README.md': 'attribution/terrain-sources.md',
    'spheres-web/data/nation_figures.json': 'attribution/nation-figures-and-rights.json',
    'spheres-web/data/person_portraits.json': 'attribution/person-portraits.json',
    'spheres-web/data/fictional_portraits.json': 'attribution/fictional-portraits.json',
    'spheres-web/data/person_models.json': 'attribution/person-models.json',
    'tools/avatars/historical_flags/sources.json': 'attribution/historical-flag-sources.json',
    'tools/avatars/README.md': 'attribution/artwork-source-policy.md',
    'tools/avatars/flag-icons-LICENSE': 'attribution/flag-icons-LICENSE',
    'spheres-web/ui/display-art/manifest.json': 'attribution/display-derivatives.json',
    'spheres-sim/data/sectors_1990.json': 'attribution/broad-sector-observations.json',
    'spheres-web/ui/area-art/manifest.json': 'attribution/areas/manifest.json',
    'spheres-web/ui/page-art/manifest.json': 'attribution/page-art.json',
    'spheres-web/ui/component-art/manifest.json': 'attribution/component-art.json',
    'tools/arsenal/vendor/model-viewer/LICENSE': 'attribution/model-viewer/LICENSE',
    'tools/arsenal/vendor/model-viewer/THREE-LICENSE': 'attribution/model-viewer/THREE-LICENSE',
    'tools/arsenal/vendor/model-viewer/LIT-LICENSE': 'attribution/model-viewer/LIT-LICENSE',
    'tools/arsenal/vendor/model-viewer/GAINMAP-LICENSE': 'attribution/model-viewer/GAINMAP-LICENSE',
}
EMBEDDED_ASSETS = {
    '/': 'spheres-web/ui/index.html', '/campaign-ui.js': 'spheres-web/ui/campaign-ui.js',
    '/campaign-transport.js': 'spheres-web/ui/campaign-transport.js', '/globe3d.js': 'spheres-web/ui/globe3d.js',
    '/equipment-ui.js': 'spheres-web/ui/equipment-ui.js', '/equipment-mesh.js': 'spheres-web/ui/equipment-mesh.js',
    '/arsenal-models.js': 'spheres-web/ui/arsenal-models.js', '/arsenal3d.js': 'spheres-web/ui/arsenal3d.js',
    '/equipment-model.js': 'spheres-web/ui/equipment-model.js',
    '/equipment-export.js': 'spheres-web/ui/equipment-export.js',
    '/art/pages/saved-campaigns-v1.webp': 'spheres-web/ui/page-art/saved-campaigns-v1.webp',
    '/art/pages/nation-selection-v1.webp': 'spheres-web/ui/page-art/nation-selection-v1.webp',
    '/art/pages/tank-designer-v1.webp': 'spheres-web/ui/page-art/tank-designer-v1.webp',
    '/height-detail.png': 'spheres-web/ui/height-detail.png',
    '/relief.png': 'spheres-web/ui/relief.png',
    '/terrain-tiles/manifest.json': 'spheres-web/ui/terrain-tiles/manifest.json',
}
DEVICE = re.compile(r'(CON|PRN|AUX|NUL|COM[1-9¹²³]|LPT[1-9¹²³])(?:\..*)?\Z', re.I)
HASH = re.compile(r'[a-f0-9]{64}\Z')


def sha(data):
    return hashlib.sha256(data).hexdigest()


def json_bytes(value):
    return (json.dumps(value, ensure_ascii=False, sort_keys=True, indent=2) + '\n').encode('utf-8')


def package_name(value):
    if not re.fullmatch(r'[A-Za-z0-9][A-Za-z0-9._-]{0,63}', value) or '..' in value or value.endswith('.') or DEVICE.fullmatch(value):
        raise argparse.ArgumentTypeError('Use a simple 1–64 character ASCII name without traversal, trailing dots or Windows device names.')
    return value


def safe_member(value):
    if not isinstance(value, str) or not value or '\\' in value or value.startswith('/'):
        raise PackageError(f'Unsafe package path: {value!r}')
    for part in value.split('/'):
        if part in ('', '.', '..') or part.endswith(('.', ' ')) or DEVICE.fullmatch(part) or any(ord(c) < 32 or c in '<>:"|?*' for c in part):
            raise PackageError(f'Unsafe package path: {value!r}')
    return PurePosixPath(value)


def regular(path):
    path = Path(os.path.abspath(path))
    if path.is_symlink() or not path.is_file() or path.resolve() != path:
        raise PackageError(f'Expected a regular file without linked parents: {path}')
    return path


def git(root, *args):
    try:
        return subprocess.check_output(['git', *args], cwd=root, stderr=subprocess.PIPE)
    except subprocess.CalledProcessError as error:
        raise PackageError('Git validation failed: ' + error.stderr.decode('utf-8', 'replace').strip()) from error


def clean_revision(root):
    if git(root, 'status', '--porcelain', '--untracked-files=no').strip():
        raise PackageError('Commit tracked changes before packaging.')
    revision = git(root, 'rev-parse', '--verify', 'HEAD^{commit}').decode().strip()
    if not re.fullmatch(r'[a-f0-9]{40}', revision):
        raise PackageError('A full committed source identity is required.')
    return revision


def native_identity(binary):
    try:
        result = subprocess.run([str(binary), '--build-info'], capture_output=True, timeout=15, check=False)
    except (OSError, subprocess.TimeoutExpired) as error:
        raise PackageError(f'Cannot inspect executable build identity: {error}') from error
    if result.returncode:
        raise PackageError('Executable --build-info failed: ' + result.stderr.decode('utf-8', 'replace'))
    try:
        return json.loads(result.stdout)
    except (ValueError, UnicodeError) as error:
        raise PackageError('Executable --build-info must emit one JSON object.') from error


def cargo_metadata(root):
    try:
        return json.loads(subprocess.check_output(['cargo', 'metadata', '--locked', '--offline', '--format-version', '1'], cwd=root, stderr=subprocess.PIPE))
    except (subprocess.CalledProcessError, ValueError, OSError) as error:
        raise PackageError(f'Offline locked dependency metadata failed: {error}') from error


def checked_identity(info, revision, platform):
    if not isinstance(revision, str) or not re.fullmatch(r'[a-f0-9]{40}', revision) or not isinstance(info, dict) or info.get('full_revision') != revision or info.get('revision') != revision[:12]:
        raise PackageError('Executable full/short revision must exactly match clean HEAD.')
    if platform not in ('windows', 'linux') or info.get('target_os') != platform or info.get('target_arch') not in ('x86_64', 'aarch64'):
        raise PackageError('Executable target OS/architecture does not match the package.')
    if not isinstance(info.get('version'), str) or not re.fullmatch(r'[0-9]+\.[0-9]+\.[0-9]+(?:[-+][A-Za-z0-9.-]+)?', info['version']):
        raise PackageError('Executable version is missing or unsafe.')
    if type(info.get('built_at_unix_seconds')) is not int or info['built_at_unix_seconds'] <= 0:
        raise PackageError('Executable needs a positive integer build epoch.')
    return {key: info.get(key) for key in ('version', 'revision', 'full_revision', 'target_os', 'target_arch', 'built_at_unix_seconds', 'branch')}


def tracked_inputs(root, revision):
    entries = {}
    for row in git(root, 'ls-tree', '-r', '-z', revision).split(b'\0'):
        if row:
            metadata, name = row.split(b'\t', 1)
            entries[name.decode('utf-8')] = metadata.decode().split()
    selected = dict(REQUIRED_INPUTS)
    for name in OPTIONAL_DOCS:
        if name in entries:
            selected[name] = name
    for name in entries:
        if name.startswith('tools/area-art/prompts/') and name.endswith('.md'):
            selected[name] = 'attribution/areas/prompts/' + name.removeprefix('tools/area-art/prompts/')
    def blob(source):
        if source not in entries or tuple(entries[source][:2]) not in (('100644', 'blob'), ('100755', 'blob')):
            raise PackageError(f'Required tracked regular source is missing: {source}')
        return git(root, 'cat-file', 'blob', entries[source][2])
    payload, provenance, assets = {}, {}, {}
    for source, target in sorted(selected.items()):
        data = blob(source)
        safe_member(target)
        payload[target] = (data, 0o644)
        provenance[source] = {'git_blob': entries[source][2], 'sha256': sha(data), 'bytes': len(data), 'package_path': target}
    selected_assets = dict(EMBEDDED_ASSETS)
    terrain = json.loads(blob('spheres-web/ui/terrain-tiles/manifest.json'))
    tile = next((row for row in terrain['tiles'].values() if isinstance(row.get('file'), str) and row.get('bytes', 0) > 1024), None)
    if tile is None:
        raise PackageError('Terrain manifest has no representative file tile.')
    safe_member(tile['file'])
    selected_assets['/terrain-tiles/'+tile['file']] = 'spheres-web/ui/terrain-tiles/'+tile['file']
    for route, source in sorted(selected_assets.items()):
        committed = blob(source)
        data = regular(root/source).read_bytes()
        # Rust include_str! embeds checkout line endings. Normalize only for
        # source identity; smoke compares the exact expected embedded bytes.
        if data != committed and (not source.endswith(('.js', '.html', '.json')) or data.replace(b'\r\n', b'\n') != committed):
            raise PackageError('Embedded asset differs from committed source: ' + source)
        assets[route] = {'sha256': sha(data), 'bytes': len(data), 'source': source, 'committed_sha256': sha(committed)}
    return payload, provenance, assets


def dependency_licenses(metadata, root):
    members = set(metadata.get('workspace_members', []))
    if not members or not isinstance(metadata.get('packages'), list):
        raise PackageError('Cargo metadata must identify workspace members and packages.')
    payload, records, ids = {}, [], set()
    for package in sorted(metadata['packages'], key=lambda item: item['id']):
        identity = package['id']
        if identity in ids:
            raise PackageError('Duplicate Cargo package identity.')
        ids.add(identity)
        if identity in members:
            continue
        directory = regular(Path(package['manifest_path'])).parent
        candidates = {p for p in directory.iterdir() if p.name.upper().startswith(('LICENSE', 'COPYING', 'NOTICE')) and p.is_file()}
        if package.get('license_file'):
            declared = Path(package['license_file'])
            declared = declared if declared.is_absolute() else directory/declared
            if not declared.resolve().is_relative_to(directory):
                raise PackageError(f'Declared dependency license escapes its package: {identity}')
            candidates.add(declared)
        if not candidates:
            raise PackageError(f'Dependency has no distributable license/notice file: {identity}')
        record = {key: package.get(key) for key in ('name', 'version', 'source', 'license', 'repository')}
        if package.get('source') is None:
            try:
                relative_manifest = directory.joinpath('Cargo.toml').relative_to(root).as_posix()
            except ValueError as error:
                raise PackageError('Local dependency must be inside the committed repository: '+identity) from error
            git(root, 'ls-files', '--error-unmatch', '--', relative_manifest)
            identity = f"local:{relative_manifest}#{package['name']}@{package['version']}"
            record.update(source='local-patch', source_manifest=relative_manifest)
        record['id'] = identity
        prefix = f"attribution/rust/{package['name']}-{package['version']}-{sha(identity.encode())[:12]}"
        record['files'] = []
        for source in sorted(candidates):
            source = regular(source)
            target = str(safe_member(prefix+'/'+source.name))
            if target in payload:
                raise PackageError('Duplicate dependency license destination: ' + target)
            data = source.read_bytes()
            payload[target] = (data, 0o644)
            record['files'].append({'path': target, 'sha256': sha(data), 'bytes': len(data)})
        records.append(record)
    payload['attribution/rust-dependencies.json'] = (json_bytes(sorted(records, key=lambda row: row['id'])), 0o644)
    return payload


def timestamp(epoch):
    value = datetime.datetime.fromtimestamp(max(315532800, min(epoch, 4354819198)), datetime.timezone.utc)
    return value.year, value.month, value.day, value.hour, value.minute, value.second//2*2


def validate_paths(paths):
    keys = set()
    for name in paths:
        safe_member(name)
        if name.casefold() in keys:
            raise PackageError('Duplicate or case-colliding package path: ' + name)
        keys.add(name.casefold())
    for name in paths:
        parts = name.split('/')
        if any('/'.join(parts[:i]).casefold() in keys for i in range(1, len(parts))):
            raise PackageError('Package file shadows a directory: ' + name)


def rename_new(source, destination):
    """Atomically publish without replacing even a pre-existing empty directory."""
    if sys.platform == 'win32':
        # Windows scanners may briefly retain a handle on a just-closed file.
        # Each rename still atomically refuses any existing destination.
        for attempt in range(6):
            try:
                os.rename(source, destination)
                break
            except PermissionError as error:
                if error.winerror not in (5, 32, 33) or os.path.lexists(destination) or attempt == 5:
                    raise
                time.sleep(0.05 * (attempt + 1))
    elif sys.platform.startswith('linux'):
        library = ctypes.CDLL(None, use_errno=True)
        function = getattr(library, 'renameat2', None)
        if function is None:
            raise PackageError('Atomic no-replace publication is unavailable.')
        function.argtypes = [ctypes.c_int, ctypes.c_char_p, ctypes.c_int, ctypes.c_char_p, ctypes.c_uint]
        function.restype = ctypes.c_int
        if function(-100, os.fsencode(source), -100, os.fsencode(destination), 1):
            code = ctypes.get_errno()
            raise OSError(code, os.strerror(code), str(destination))
    else:
        raise PackageError('Publication supports Windows and Linux hosts.')


def remove_stage(stage, parent):
    if stage.parent != parent or not stage.name.startswith('.spheres-package-') or stage.is_symlink() or stage.resolve() != stage:
        raise PackageError('Refusing cleanup outside the owned staging directory.')
    shutil.rmtree(stage)


def write_archive(target, name, payload, epoch):
    with zipfile.ZipFile(target, 'x', compression=zipfile.ZIP_STORED, allowZip64=True) as archive:
        for relative, (data, mode) in sorted(payload.items()):
            info = zipfile.ZipInfo(name+'/'+relative, timestamp(epoch))
            info.create_system = 3
            info.compress_type = zipfile.ZIP_STORED
            info.external_attr = (stat.S_IFREG | mode) << 16
            archive.writestr(info, data)


def package_release(root, binary, output, name=None, platform=None, base=None, *, build_probe=None, metadata_loader=None):
    root = Path(root).resolve()
    platform = platform or ('windows' if sys.platform == 'win32' else 'linux')
    binary = regular(binary)
    revision = clean_revision(root)
    executable_bytes = binary.read_bytes()
    info = checked_identity((build_probe or native_identity)(binary), revision, platform)
    name = package_name(name or f"SPHERES-{info['version']}-{'Windows' if platform == 'windows' else 'Linux'}")
    output = Path(os.path.abspath(output))
    if output.resolve() != output:
        raise PackageError('Output must not resolve through linked parents.')
    release, archive_path = output/name, output/(name+'.zip')
    patch_path = output/(name+'-source.patch') if base else None
    if any(os.path.lexists(p) for p in [release, archive_path] + ([patch_path] if patch_path else [])):
        raise PackageError('Release folder, ZIP or optional patch already exists; choose a new output/name.')
    patch = None
    if base:
        if base.startswith('-'):
            raise PackageError('Patch base must be a commit, not an option.')
        base = git(root, 'rev-parse', '--verify', base+'^{commit}').decode().strip()
        if base == revision:
            raise PackageError('Same-revision patch is empty; omit --base instead.')
        patch = git(root, 'diff', '--binary', '--no-ext-diff', '--no-textconv', base, revision, '--')
    payload, provenance, assets = tracked_inputs(root, revision)
    payload.update(dependency_licenses((metadata_loader or cargo_metadata)(root), root))
    executable = 'spheres-web.exe' if platform == 'windows' else 'spheres-web'
    launcher = 'Play SPHERES.cmd' if platform == 'windows' else 'Play SPHERES.sh'
    payload[executable] = (executable_bytes, 0o755)
    launch = ('@echo off\r\nsetlocal\r\ncd /d "%~dp0"\r\necho Keep this window open while playing. Saves stay here.\r\n"%~dp0spheres-web.exe" --port 7777\r\nif errorlevel 1 pause\r\n' if platform == 'windows' else
              '#!/bin/sh\nset -eu\ncd -- "$(CDPATH= cd -- "$(dirname -- "$0")" && pwd)"\nexec ./spheres-web --port 7777\n')
    payload[launcher] = (launch.encode(), 0o755 if platform == 'linux' else 0o644)
    instructions = (f"SPHERES {info['version']} — {platform} portable package\n\nExtract the full ZIP into a writable folder and run {launcher}.\n"
                    "Linux extractors may require chmod +x spheres-web 'Play SPHERES.sh'. No Rust installation or internet is needed to play.\n"
                    f"Keep the server window open. If port 7777 is occupied, run {executable} --port 7824 from this folder.\n"
                    "About shows the build and save directory. Save before closing. Named saves are under saves/; the default is save.json.\n"
                    "Campaigns offers previous-backup recovery. For upgrades, keep your old executable and original saves in their old folder.\n"
                    "Copy a save into the new release folder and create a NEW named slot there. Older builds may not read saves written by newer builds.\n"
                    "To roll back, launch the preserved old executable with its untouched original save, not the newer save.\n\n"
                    "release.json identifies the executable, source commit, assets and attribution. SHA256SUMS.txt covers package files.\n"
                    "A package is not itself campaign certification. See the accompanying README and current release evidence.\n")
    payload['START-HERE.txt'] = (instructions.encode('utf-8'), 0o644)
    source = {'repository': REPOSITORY, 'revision': revision, 'tracked_inputs': provenance, 'revision_url': REPOSITORY+'/tree/'+revision,
              'patch': None if patch is None else {'base': base, 'file': patch_path.name, 'sha256': sha(patch), 'bytes': len(patch)}}
    payload['source/SOURCE.json'] = (json_bytes(source), 0o644)
    manifest = {'format': 'spheres-release/v1', 'package_name': name, 'version': info['version'], 'revision': revision,
                'short_revision': info['revision'], 'platform': platform, 'architecture': info['target_arch'], 'native_build_info': info,
                'executable': {'path': executable, 'sha256': sha(executable_bytes), 'bytes': len(executable_bytes)}, 'launcher': launcher,
                'source_repository': REPOSITORY, 'runtime_assets': assets, 'archive_method': 'ZIP_STORED; fixed UTC build timestamp and modes',
                'files': {p: {'sha256': sha(b), 'bytes': len(b), 'mode': f'{mode:04o}'} for p, (b, mode) in sorted(payload.items())}}
    payload['release.json'] = (json_bytes(manifest), 0o644)
    payload['SHA256SUMS.txt'] = (''.join(sha(b)+'  '+p+'\n' for p, (b, _) in sorted(payload.items())).encode(), 0o644)
    validate_paths(payload)
    if clean_revision(root) != revision or binary.read_bytes() != executable_bytes:
        raise PackageError('Source/executable changed during preflight.')
    output.mkdir(parents=True, exist_ok=True)
    stage = Path(tempfile.mkdtemp(prefix='.spheres-package-', dir=output)).absolute()
    published = []
    try:
        folder = stage/name
        folder.mkdir()
        for relative, (data, mode) in payload.items():
            target = folder/relative
            target.parent.mkdir(parents=True, exist_ok=True)
            with target.open('xb') as stream:
                stream.write(data)
            target.chmod(mode)
        staged_zip = stage/(name+'.zip')
        write_archive(staged_zip, name, payload, info['built_at_unix_seconds'])
        verified = verify_archive(staged_zip)
        staged_files = [(staged_zip, archive_path)]
        if patch is not None:
            staged_patch = stage/patch_path.name
            staged_patch.write_bytes(patch)
            staged_files.append((staged_patch, patch_path))
        for temporary, target in staged_files:
            os.link(temporary, target)
            published.append((temporary, target))
        rename_new(folder, release)
        return {'passed': True, 'release': str(release), 'zip': str(archive_path), 'zip_bytes': verified['archive']['bytes'],
                'sha256': verified['archive']['sha256'], 'patch': str(patch_path) if patch_path else None, 'revision': revision,
                'platform': platform, 'architecture': info['target_arch'], 'executable_sha256': sha(executable_bytes)}
    except BaseException:
        for temporary, target in reversed(published):
            if target.exists() and os.path.samestat(temporary.stat(), target.stat()):
                target.unlink()
        raise
    finally:
        remove_stage(stage, output)


def no_duplicate_keys(pairs):
    value = {}
    for key, item in pairs:
        if key in value:
            raise PackageError('Duplicate JSON key: ' + key)
        value[key] = item
    return value


def verify_archive(file):
    try:
        return _verify_archive(file)
    except (zipfile.BadZipFile, KeyError, TypeError, UnicodeError, argparse.ArgumentTypeError) as error:
        raise PackageError('Invalid release archive: '+str(error)) from error


def _verify_archive(file):
    file = regular(file)
    raw_hash = sha(file.read_bytes())
    with zipfile.ZipFile(file) as archive:
        infos = archive.infolist()
        if not infos or len(infos) > 20000:
            raise PackageError('Empty or unreasonable package entry count.')
        names = [i.filename for i in infos]
        validate_paths(names)
        if any(i.is_dir() or i.flag_bits & 1 or stat.S_IFMT(i.external_attr >> 16) != stat.S_IFREG for i in infos):
            raise PackageError('Only unencrypted regular-file entries are accepted.')
        roots = {safe_member(name).parts[0] for name in names}
        if len(roots) != 1 or any(len(safe_member(name).parts) < 2 for name in names):
            raise PackageError('Package needs one named root directory.')
        name = package_name(next(iter(roots)))
        by_name = {i.filename.removeprefix(name+'/'): i for i in infos}
        required = {'release.json', 'SHA256SUMS.txt', 'START-HERE.txt', 'source/SOURCE.json', 'attribution/rust-dependencies.json'} | set(REQUIRED_INPUTS.values())
        if not required.issubset(by_name):
            raise PackageError('Missing package identity/checksums/instructions.')
        if sum(i.file_size for i in infos) > 4*1024**3:
            raise PackageError('Package exceeds the four-GiB extraction bound.')
        manifest = json.loads(archive.read(by_name['release.json']), object_pairs_hook=no_duplicate_keys)
        sums = {}
        for line in archive.read(by_name['SHA256SUMS.txt']).decode('utf-8').splitlines():
            digest, separator, member = line.partition('  ')
            if not separator or not HASH.fullmatch(digest):
                raise PackageError('Malformed checksum entry.')
            safe_member(member)
            if member in sums:
                raise PackageError('Duplicate checksum entry.')
            sums[member] = digest
        if set(sums) != set(by_name)-{'SHA256SUMS.txt'}:
            raise PackageError('Checksum inventory must cover every payload file exactly.')
        for member, digest in sums.items():
            if sha(archive.read(by_name[member])) != digest:
                raise PackageError('Checksum mismatch: '+member)
        if manifest.get('format') != 'spheres-release/v1' or manifest.get('package_name') != name:
            raise PackageError('Wrong release manifest format/root.')
        info = checked_identity(manifest.get('native_build_info'), manifest.get('revision'), manifest.get('platform'))
        if manifest.get('short_revision') != info['revision'] or manifest.get('architecture') != info['target_arch'] or manifest.get('version') != info['version']:
            raise PackageError('Release and native identity disagree.')
        files = manifest.get('files', {})
        if set(files) != set(by_name)-{'release.json', 'SHA256SUMS.txt'}:
            raise PackageError('Release file inventory is incomplete or contains extra entries.')
        for member, record in files.items():
            entry = by_name[member]
            if record.get('sha256') != sums[member] or record.get('bytes') != entry.file_size or record.get('mode') != f'{stat.S_IMODE(entry.external_attr >> 16):04o}':
                raise PackageError('Release file metadata differs: '+member)
            if record['mode'] not in ('0644', '0755'):
                raise PackageError('Unsupported file permissions.')
        executable = manifest.get('executable', {})
        expected = 'spheres-web.exe' if manifest['platform'] == 'windows' else 'spheres-web'
        launcher = 'Play SPHERES.cmd' if manifest['platform'] == 'windows' else 'Play SPHERES.sh'
        if executable.get('path') != expected or expected not in files or executable.get('sha256') != files[expected]['sha256'] or executable.get('bytes') != files[expected]['bytes'] or manifest.get('launcher') != launcher or launcher not in files:
            raise PackageError('Executable or launcher identity is inconsistent.')
        if files[expected]['mode'] != '0755' or files[launcher]['mode'] != ('0755' if manifest['platform'] == 'linux' else '0644'):
            raise PackageError('Executable/launcher permissions are inconsistent.')
        source = json.loads(archive.read(by_name['source/SOURCE.json']), object_pairs_hook=no_duplicate_keys)
        if source.get('revision') != manifest['revision'] or source.get('repository') != REPOSITORY:
            raise PackageError('Source provenance differs from the release identity.')
        provenance = source.get('tracked_inputs', {})
        for original, packaged in REQUIRED_INPUTS.items():
            row = provenance.get(original, {})
            if row.get('package_path') != packaged or row.get('sha256') != sums[packaged] or row.get('bytes') != files[packaged]['bytes']:
                raise PackageError('Required source attribution is missing or inconsistent: '+original)
        dependencies = json.loads(archive.read(by_name['attribution/rust-dependencies.json']), object_pairs_hook=no_duplicate_keys)
        if not isinstance(dependencies, list) or not dependencies:
            raise PackageError('Dependency license inventory is empty.')
        referenced = set()
        identities = set()
        for dependency in dependencies:
            if dependency.get('id') in identities or not dependency.get('files'):
                raise PackageError('Missing or duplicate dependency license record.')
            identities.add(dependency.get('id'))
            for row in dependency['files']:
                member = row.get('path', '')
                if not member.startswith('attribution/rust/') or member in referenced or member not in files or row.get('sha256') != sums[member] or row.get('bytes') != files[member]['bytes']:
                    raise PackageError('Dependency license reference is missing or inconsistent.')
                referenced.add(member)
        if referenced != {p for p in files if p.startswith('attribution/rust/')}:
            raise PackageError('Dependency license payload and inventory disagree.')
        assets = manifest.get('runtime_assets', {})
        terrain = json.loads(archive.read(by_name['attribution/terrain-tiles/manifest.json']))
        tile = next((r for r in terrain['tiles'].values() if isinstance(r.get('file'), str) and r.get('bytes', 0) > 1024), None)
        if tile is None:
            raise PackageError('Missing representative terrain tile.')
        safe_member(tile['file'])
        if set(assets) != set(EMBEDDED_ASSETS) | {'/terrain-tiles/'+tile['file']} or any(not HASH.fullmatch(row.get('sha256','')) or type(row.get('bytes')) is not int or row['bytes'] <= 0 for row in assets.values()):
            raise PackageError('Required embedded asset identity inventory is incomplete.')
        tile_pin = assets['/terrain-tiles/'+tile['file']]
        if tile_pin['sha256'] != tile.get('sha256') or tile_pin['bytes'] != tile['bytes']:
            raise PackageError('Runtime terrain tile differs from its source manifest.')
        extraction_files = {p: {'sha256': sha(archive.read(i)), 'bytes': i.file_size, 'mode': stat.S_IMODE(i.external_attr >> 16)} for p, i in by_name.items()}
    if sha(file.read_bytes()) != raw_hash:
        raise PackageError('Archive changed during verification.')
    return {'format': 'spheres-package-verification/v1', 'passed': True, 'archive': {'path': str(file), 'sha256': raw_hash, 'bytes': file.stat().st_size},
            'package_name': name, 'manifest': manifest, 'files_verified': len(infos), 'verified_files': extraction_files}


def extract_verified(file, destination):
    result = verify_archive(file)
    destination = Path(os.path.abspath(destination))
    if destination.resolve() != destination or os.path.lexists(destination):
        raise PackageError('Extraction requires a new directory without linked parents.')
    destination.parent.mkdir(parents=True, exist_ok=True)
    stage = Path(tempfile.mkdtemp(prefix='.spheres-package-', dir=destination.parent)).absolute()
    try:
        with zipfile.ZipFile(file) as archive:
            expected = {result['package_name']+'/'+p: row for p, row in result['verified_files'].items()}
            infos = archive.infolist()
            if len(infos) != len(expected) or {i.filename for i in infos} != set(expected):
                raise PackageError('Archive entries changed after verification.')
            for info in infos:
                pin = expected[info.filename]
                if info.file_size != pin['bytes'] or info.external_attr >> 16 != stat.S_IFREG | pin['mode']:
                    raise PackageError('Archive entry metadata changed after verification.')
                target = stage/str(safe_member(info.filename))
                target.parent.mkdir(parents=True, exist_ok=True)
                with target.open('xb') as stream:
                    stream.write(archive.read(info))
                if target.stat().st_size != pin['bytes'] or sha(target.read_bytes()) != pin['sha256']:
                    raise PackageError('Extracted bytes differ from the verified archive: '+info.filename)
                target.chmod(pin['mode'])
        if sha(Path(file).read_bytes()) != result['archive']['sha256']:
            raise PackageError('Archive changed during extraction.')
        rename_new(stage, destination)
    finally:
        if stage.exists():
            remove_stage(stage, destination.parent)
    result['extracted_root'] = str(destination/result['package_name'])
    return result
