"""Pinned pre-production inputs for immutable preparation regressions.

No live file is replaced or filtered. The ordinary validators retain their
strict logic; this read-only view supplies their recorded baseline inputs.
Live production acceptance belongs to the separate production coverage tools.
"""
from __future__ import annotations

import fnmatch
import hashlib
import io
import json
from pathlib import Path, PurePosixPath
import zipfile

BASE = '5ea4f8fcd05e1858b1dfa83ae34c6d3703d6b104'
DIRECTORY = 'tools/avatars/fixtures/preparation-baseline-5ea4f8fc'
MANIFEST_SHA256 = '3ab8ffd59623ea55fa878e421323e954d46b12b2af4a497ee4afaf5c87db2b1b'
BUNDLE_SHA256 = '2c9f1536252c35aaa4b79f0b09cc752d6a6ed0e8a353006df2649cb5ca8440e6'


class Snapshot:
    def __init__(self, root):
        directory = Path(root) / DIRECTORY
        manifest_bytes = (directory / 'manifest.json').read_bytes()
        if hashlib.sha256(manifest_bytes).hexdigest() != MANIFEST_SHA256:
            raise ValueError('Preparation baseline manifest SHA-256 mismatch')
        manifest = json.loads(manifest_bytes)
        bundle = (directory / 'inputs.zip').read_bytes()
        if (manifest.get('version') != 1 or manifest.get('source_revision') != BASE
                or manifest.get('bundle_sha256') != BUNDLE_SHA256
                or hashlib.sha256(bundle).hexdigest() != BUNDLE_SHA256):
            raise ValueError('Preparation baseline revision or bundle SHA-256 mismatch')
        self.files = {}
        with zipfile.ZipFile(io.BytesIO(bundle)) as archive:
            names = archive.namelist()
            expected = [row['path'] for row in manifest['files']]
            if len(names) != len(set(names)) or len(expected) != len(set(expected)) or set(names) != set(expected):
                raise ValueError('Preparation baseline file inventory changed')
            for row in manifest['files']:
                path = row['path']
                if PurePosixPath(path).is_absolute() or '..' in PurePosixPath(path).parts or '\\' in path:
                    raise ValueError('Unsafe preparation baseline path')
                data = archive.read(path)
                blob = hashlib.sha1(b'blob ' + str(len(data)).encode() + b'\0' + data).hexdigest()
                if (len(data) != row['bytes'] or hashlib.sha256(data).hexdigest() != row['sha256']
                        or blob != row['git_blob']):
                    raise ValueError('Preparation baseline file/hash mismatch: ' + path)
                self.files[path] = data
        self.manifest = manifest

    def read(self, path):
        if path not in self.files:
            raise ValueError('Input absent from pinned preparation baseline: ' + path)
        return self.files[path]

    def paths(self, pattern):
        return sorted(path for path in self.files if fnmatch.fnmatchcase(path, pattern))

    def scope(self):
        return {'kind': 'immutable_preparation_baseline', 'source_revision': BASE,
                'manifest_sha256': MANIFEST_SHA256, 'bundle_sha256': BUNDLE_SHA256,
                'live_production_checked': False,
                'limit': 'Preparation regression inputs only; no current runtime, artwork or country completion approval.'}


def live_scope():
    return {'kind': 'live_repository_inputs', 'source_revision': None,
            'live_production_checked': False,
            'limit': 'Strict proposal compatibility against current inputs; not a production acceptance check.'}
