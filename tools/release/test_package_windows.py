"""Real temporary Git fixtures; no compiler, network or installed game required."""
import argparse
import json
import os
from pathlib import Path
import shutil
import stat
import subprocess
import sys
import tempfile
import unittest
from unittest.mock import patch
import zipfile

import release_package as release


class PackagingTests(unittest.TestCase):
    def setUp(self):
        self.temp = tempfile.TemporaryDirectory()
        self.addCleanup(self.temp.cleanup)
        self.base = Path(self.temp.name).resolve()
        self.root = self.base/'checkout'
        self.root.mkdir()
        self.git('init', '-q')
        self.git('config', 'user.email', 'fixture@example.invalid')
        self.git('config', 'user.name', 'Package test')
        self.git('config', 'core.autocrlf', 'false')
        for source in set(release.REQUIRED_INPUTS) | set(release.EMBEDDED_ASSETS.values()):
            self.write(source, ('fixture '+source+'\n').encode())
        tile = b'png-shaped-test-data'*100
        self.write('spheres-web/ui/terrain-tiles/x00_y00.png', tile)
        self.write('spheres-web/ui/terrain-tiles/manifest.json', release.json_bytes({'tiles': {'x00_y00': {'file': 'x00_y00.png', 'bytes': len(tile), 'sha256': release.sha(tile)}}}))
        self.write('vendor/patched/Cargo.toml', b'[package]\nname="patched"\nversion="1.0.0"\n')
        self.write('vendor/patched/LICENSE', b'Fixture license\n')
        self.write('vendor/patched/NOTICE', b'Fixture notice\n')
        self.git('add', '.')
        self.git('commit', '-qm', 'fixture')
        self.binary = self.base/'native'
        self.binary.write_bytes(b'controlled native bytes')

    def git(self, *args):
        return subprocess.check_output(['git', *args], cwd=self.root, stderr=subprocess.PIPE).decode().strip()

    def write(self, relative, data):
        file = self.root/relative
        file.parent.mkdir(parents=True, exist_ok=True)
        file.write_bytes(data)

    def metadata(self, root):
        return {'workspace_members': ['workspace'], 'packages': [
            {'id': 'workspace', 'name': 'game', 'version': '1.0.0', 'source': None},
            {'id': 'path+file://'+root.as_posix()+'/vendor/patched#1.0.0', 'name': 'patched', 'version': '1.0.0',
             'source': None, 'license': 'MIT', 'manifest_path': str(root/'vendor/patched/Cargo.toml')} ]}

    def info(self, platform='windows'):
        revision = self.git('rev-parse', 'HEAD')
        return {'version': '0.1.0', 'revision': revision[:12], 'full_revision': revision,
                'target_os': platform, 'target_arch': 'x86_64', 'built_at_unix_seconds': 1770000001,
                'branch': 'fixture', 'save_directory': str(self.base/'runtime-only')}

    def pack(self, directory='out', platform='windows', **kwargs):
        return release.package_release(self.root, self.binary, self.base/directory,
            name='SPHERES-test', platform=platform, build_probe=lambda _: self.info(platform),
            metadata_loader=self.metadata, **kwargs)

    def archive(self, **kwargs):
        return Path(self.pack(**kwargs)['zip'])

    def rewrite(self, source, change):
        destination = self.base/'changed.zip'
        with zipfile.ZipFile(source) as original:
            entries = {i.filename: [i, original.read(i)] for i in original.infolist()}
        change(entries)
        with zipfile.ZipFile(destination, 'w') as altered:
            for info, data in entries.values():
                altered.writestr(info, data)
        return destination

    def rehash(self, entries):
        prefix = 'SPHERES-test/'
        manifest = json.loads(entries[prefix+'release.json'][1])
        for key in list(manifest['files']):
            if prefix+key not in entries:
                del manifest['files'][key]
        entries[prefix+'release.json'][1] = release.json_bytes(manifest)
        entries[prefix+'SHA256SUMS.txt'][1] = ''.join(release.sha(data)+'  '+name[len(prefix):]+'\n'
            for name, (_, data) in sorted(entries.items()) if name != prefix+'SHA256SUMS.txt').encode()

    def test_reproducible_across_output_paths_mtimes_and_checkout_relocation(self):
        first = self.archive(directory='first')
        os.utime(self.root/'README.md', (1, 1))
        second = self.archive(directory='second')
        self.assertEqual(first.read_bytes(), second.read_bytes())
        relocated = self.base/'relocated'
        shutil.copytree(self.root, relocated)
        self.root = relocated
        third = self.archive(directory='third')
        self.assertEqual(first.read_bytes(), third.read_bytes())
        manifest = release.verify_archive(third)['manifest']
        self.assertNotIn('save_directory', manifest['native_build_info'])
        self.assertNotIn(str(self.base), json.dumps(manifest))

    def test_roundtrip_platforms_modes_and_patched_licenses(self):
        for platform in ('windows', 'linux'):
            with self.subTest(platform=platform):
                archive = self.archive(directory=platform, platform=platform)
                result = release.extract_verified(archive, self.base/(platform+'-extracted'))
                folder = Path(result['extracted_root'])
                self.assertEqual((folder/result['manifest']['executable']['path']).read_bytes(), self.binary.read_bytes())
                records = json.loads((folder/'attribution/rust-dependencies.json').read_text())
                self.assertEqual(len(records), 1)
                self.assertEqual(records[0]['source'], 'local-patch')
                self.assertEqual(len(records[0]['files']), 2)
                self.assertEqual(result['manifest']['files'][result['manifest']['executable']['path']]['mode'], '0755')
                self.assertEqual(set(result['manifest']['runtime_assets']), set(release.EMBEDDED_ASSETS) | {'/terrain-tiles/x00_y00.png'})

    def test_reorganized_player_docs_ship_at_their_linked_paths(self):
        guide = b'# Player guide\nCampaign recovery and controls.\n'
        decisions = b'# Player decisions\nReviewed orders and receipts.\n'
        self.write('docs/PLAYING.md', guide)
        self.write('docs/reference/PLAYER_DECISIONS.md', decisions)
        self.git('add', 'docs')
        self.git('commit', '-qm', 'relocated player documentation')
        result = release.extract_verified(self.archive(), self.base/'docs-extracted')
        folder = Path(result['extracted_root'])
        self.assertEqual((folder/'docs/PLAYING.md').read_bytes(), guide)
        self.assertEqual((folder/'docs/reference/PLAYER_DECISIONS.md').read_bytes(), decisions)
        self.assertFalse((folder/'PLAYER_DECISIONS.md').exists())
        self.assertEqual(result['manifest']['files']['docs/PLAYING.md']['sha256'], release.sha(guide))

    def test_stale_or_mismatched_binary_refuses_before_output(self):
        for field, value in [('full_revision', 'f'*40), ('revision', 'f'*12), ('target_os', 'linux'), ('target_arch', 'unknown'), ('built_at_unix_seconds', True)]:
            with self.subTest(field=field):
                info = self.info(); info[field] = value
                with self.assertRaises(release.PackageError):
                    release.package_release(self.root, self.binary, self.base/'blocked', platform='windows', build_probe=lambda _: info, metadata_loader=self.metadata)
                self.assertFalse((self.base/'blocked').exists())

    def test_dirty_tracked_input_refuses(self):
        self.write('README.md', b'dirty')
        with self.assertRaisesRegex(release.PackageError, 'Commit tracked'):
            self.pack()
        self.assertFalse((self.base/'out').exists())

    def test_untracked_prompts_never_enter_package(self):
        self.write('tools/area-art/prompts/untracked.md', b'not approved source')
        manifest = release.verify_archive(self.archive())['manifest']
        self.assertFalse(any('untracked' in p for p in manifest['files']))

    def test_missing_dependency_license_refuses_before_publication(self):
        (self.root/'vendor/patched/LICENSE').unlink(); (self.root/'vendor/patched/NOTICE').unlink()
        self.git('add', '-u'); self.git('commit', '-qm', 'missing licenses')
        with self.assertRaisesRegex(release.PackageError, 'no distributable'):
            self.pack()
        self.assertFalse((self.base/'out').exists())

    def test_reserved_names_and_lexical_parent_paths(self):
        for name in ('../escape', 'CON', 'nul.txt', 'COM1', 'bad.', 'a/b', 'a\\b'):
            with self.subTest(name=name), self.assertRaises(argparse.ArgumentTypeError):
                release.package_name(name)
        directory = self.base/'another'; directory.mkdir()
        self.assertEqual(release.regular(directory/'..'/'native'), self.binary)

    def test_existing_outputs_preserved(self):
        for suffix in ('', '.zip', '-source.patch'):
            with self.subTest(suffix=suffix):
                out = self.base/('collision'+str(len(suffix))); out.mkdir()
                target = out/('SPHERES-test'+suffix)
                if not suffix:
                    target.mkdir(); (target/'sentinel').write_bytes(b'keep')
                else:
                    target.write_bytes(b'keep')
                base = self.git('rev-parse', 'HEAD') if suffix.endswith('patch') else None
                with self.assertRaises(release.PackageError):
                    self.pack(directory=out.name, base=base)
                self.assertEqual((target/'sentinel').read_bytes() if not suffix else target.read_bytes(), b'keep')

    def test_patch_is_explicit_and_source_pinned(self):
        base = self.git('rev-parse', 'HEAD')
        no_patch = self.pack(directory='without')
        self.assertIsNone(no_patch['patch'])
        self.write('README.md', b'new documentation\n'); self.git('add', '.'); self.git('commit', '-qm', 'second')
        result = self.pack(directory='with', base=base)
        provenance = json.loads((Path(result['release'])/'source/SOURCE.json').read_text())
        self.assertEqual(provenance['patch']['sha256'], release.sha(Path(result['patch']).read_bytes()))
        self.assertEqual(provenance['patch']['base'], base)

    def test_tampered_payload_refuses_extraction(self):
        changed = self.rewrite(self.archive(), lambda entries: entries['SPHERES-test/README.md'].__setitem__(1, b'changed'))
        with self.assertRaisesRegex(release.PackageError, 'Checksum mismatch'):
            release.extract_verified(changed, self.base/'no-extraction')
        self.assertFalse((self.base/'no-extraction').exists())

    def test_missing_required_attribution_cannot_be_rehashed_into_pass(self):
        archive = self.archive()
        for member in ('source/Cargo.lock', 'source/SOURCE.json', 'attribution/person-portraits.json', 'attribution/rust-dependencies.json'):
            with self.subTest(member=member):
                def change(entries):
                    del entries['SPHERES-test/'+member]; self.rehash(entries)
                changed = self.rewrite(archive, change)
                with self.assertRaises(release.PackageError):
                    release.verify_archive(changed)

    def test_license_file_removal_cannot_be_rehashed_into_pass(self):
        def change(entries):
            key = next(k for k in entries if k.startswith('SPHERES-test/attribution/rust/') and k.endswith('/LICENSE'))
            del entries[key]; self.rehash(entries)
        with self.assertRaises(release.PackageError):
            release.verify_archive(self.rewrite(self.archive(), change))

    def test_unsafe_archive_members_refused(self):
        archive = self.archive()
        for member in ('../outside', 'SPHERES-test/../outside', 'SPHERES-test/CON', 'SPHERES-test/C:evil', 'SPHERES-test/a\\b', 'SPHERES-test/README.MD'):
            with self.subTest(member=member):
                def change(entries):
                    info = zipfile.ZipInfo(member); info.external_attr = (stat.S_IFREG | 0o644) << 16
                    entries[member] = [info, b'bad']
                with self.assertRaises(release.PackageError):
                    release.verify_archive(self.rewrite(archive, change))

    def test_symlink_archive_entry_refused(self):
        def change(entries):
            entries['SPHERES-test/README.md'][0].external_attr = (stat.S_IFLNK | 0o777) << 16
        with self.assertRaises(release.PackageError):
            release.verify_archive(self.rewrite(self.archive(), change))

    def test_corrupted_second_archive_read_cannot_publish(self):
        archive = self.archive()
        original = zipfile.ZipFile.read
        verified = release.verify_archive(archive)
        def corrupt(instance, name, *args, **kwargs):
            data = original(instance, name, *args, **kwargs)
            return b'corrupt' if getattr(name, 'filename', name).endswith('/README.md') else data
        with patch.object(release, 'verify_archive', return_value=verified), patch.object(zipfile.ZipFile, 'read', corrupt):
            with self.assertRaisesRegex(release.PackageError, 'Extracted bytes'):
                release.extract_verified(archive, self.base/'no-publication')
        self.assertFalse((self.base/'no-publication').exists())
        self.assertFalse(list(self.base.glob('.spheres-package-*')))

    def test_publish_collision_rolls_back_only_owned_zip(self):
        def collide(source, destination):
            destination.mkdir(); (destination/'sentinel').write_bytes(b'foreign')
            raise FileExistsError('concurrent output')
        with patch.object(release, 'rename_new', side_effect=collide), self.assertRaises(FileExistsError):
            self.pack()
        self.assertEqual((self.base/'out/SPHERES-test/sentinel').read_bytes(), b'foreign')
        self.assertFalse((self.base/'out/SPHERES-test.zip').exists())
        self.assertFalse(list((self.base/'out').glob('.spheres-package-*')))

    def test_existing_extraction_is_not_replaced(self):
        archive = self.archive()
        destination = self.base/'exists'; destination.mkdir(); (destination/'save.json').write_bytes(b'user save')
        with self.assertRaises(release.PackageError):
            release.extract_verified(archive, destination)
        self.assertEqual((destination/'save.json').read_bytes(), b'user save')

    def test_invalid_zip_cli_records_failure_and_never_overwrites_report(self):
        bad = self.base/'bad.zip'; bad.write_bytes(b'bad zip')
        report = self.base/'failure.json'
        command = [sys.executable, '-B', str(Path(__file__).with_name('verify_package.py')), str(bad), str(self.base/'extract'), '--report', str(report)]
        result = subprocess.run(command, capture_output=True)
        self.assertEqual(result.returncode, 2, result.stderr)
        self.assertFalse(json.loads(report.read_text())['passed'])
        before = report.read_bytes()
        self.assertNotEqual(subprocess.run(command, capture_output=True).returncode, 0)
        self.assertEqual(report.read_bytes(), before)


if __name__ == '__main__':
    unittest.main()
