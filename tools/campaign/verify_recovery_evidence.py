"""Verify every retained recovery artifact; optionally restore into a new directory."""
from pathlib import Path
import argparse
import gzip
import hashlib
import json


def safe_path(root, relative):
    target = (root / relative).resolve()
    if target == root or not target.is_relative_to(root):
        raise ValueError('Evidence path escaped its root.')
    return target


def verify(packet, output=None):
    packet = packet.resolve(strict=True)
    manifest = json.loads((packet / 'manifest.json').read_text(encoding='utf-8'))
    assert manifest['format'] == 'spheres-recovery-evidence/v2'
    if output is not None:
        output = output.resolve()
        if output.exists():
            raise ValueError('Restore destination must be new.')
    chunks = {}
    for key, entry in manifest['objects'].items():
        packed = safe_path(packet, entry['path']).read_bytes()
        assert len(packed) == entry['retained_bytes']
        assert hashlib.sha256(packed).hexdigest() == entry['retained_sha256']
        raw = gzip.decompress(packed)
        assert len(raw) == entry['bytes'] <= manifest['chunk_size']
        assert hashlib.sha256(raw).hexdigest() == key
        chunks[key] = raw
    names = set()
    for entry in manifest['files']:
        name = entry['original_relative_path']
        assert name not in names, 'Duplicate restoration path'
        names.add(name)
        raw = b''.join(chunks[key] for key in entry['chunks'])
        assert len(raw) == entry['original_bytes']
        assert hashlib.sha256(raw).hexdigest() == entry['original_sha256']
        if output is not None:
            target = safe_path(output, name)
            target.parent.mkdir(parents=True, exist_ok=True)
            with target.open('xb') as stream:
                stream.write(raw)
    print(f'PASS: {len(names)} original artifacts, {len(chunks)} lossless chunks.')


if __name__ == '__main__':
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('packet', type=Path)
    parser.add_argument('--restore', type=Path)
    args = parser.parse_args()
    verify(args.packet, args.restore)
