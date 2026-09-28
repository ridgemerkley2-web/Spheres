"""Retain the named disposable recovery attempts without changing their sources."""
from pathlib import Path
import argparse
import gzip
import hashlib
import json


def collect(base, destination):
    base, destination = base.resolve(strict=True), destination.resolve()
    if destination.exists():
        raise ValueError('Use a new evidence directory; previous attempts are retained.')
    sources = []
    for name in ('recovery-browser-01', 'recovery-browser-02', 'recovery-matrix-01', 'recovery-matrix-02'):
        folder = base / name
        sources.extend(p for p in sorted(folder.iterdir()) if p.is_file())
    sources.extend(base / name for name in (
        'recovery-red.log', 'recovery-red-02.log', 'recovery-green.log',
        'recovery-green-02.log', 'recovery-ui.log', 'recovery-build-final.log'))
    for source in sources:
        if not source.is_file() or source.is_symlink():
            raise ValueError(f'Expected an ordinary retained output: {source}')
    destination.mkdir(parents=True)
    entries, objects = [], {}
    (destination / 'objects').mkdir()
    for source in sources:
        raw = source.read_bytes()
        relative = source.relative_to(base)
        # Fixed binary chunks retain even the original gzip stream exactly.
        # Repeated recovery snapshots share almost all of their saved bytes.
        chunks = []
        for offset in range(0, len(raw), 65536):
            chunk = raw[offset:offset + 65536]
            key = hashlib.sha256(chunk).hexdigest()
            chunks.append(key)
            if key not in objects:
                packed = gzip.compress(chunk, mtime=0)
                target = destination / 'objects' / (key + '.gz')
                target.write_bytes(packed)
                objects[key] = {'path': target.relative_to(destination).as_posix(),
                                'bytes': len(chunk), 'retained_bytes': len(packed),
                                'retained_sha256': hashlib.sha256(packed).hexdigest()}
        restored = b''.join(gzip.decompress((destination / objects[key]['path']).read_bytes()) for key in chunks)
        assert restored == raw
        entries.append({
            'original_relative_path': relative.as_posix(),
            'chunks': chunks,
            'original_bytes': len(raw),
            'original_sha256': hashlib.sha256(raw).hexdigest(),
        })
        assert source.read_bytes() == raw, 'The original evidence changed during collection.'
    manifest = {'format': 'spheres-recovery-evidence/v2',
                'scope': 'Named disposable attempts, including failures; no user campaign directories.',
                'source_base': str(base), 'chunk_size': 65536, 'objects': objects, 'files': entries}
    (destination / 'manifest.json').write_text(json.dumps(manifest, indent=2) + '\n', encoding='utf-8')
    print(json.dumps({'files': len(entries), 'objects': len(objects), 'retained_bytes': sum(e['retained_bytes'] for e in objects.values())}))


if __name__ == '__main__':
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('base', type=Path)
    parser.add_argument('destination', type=Path)
    args = parser.parse_args()
    collect(args.base, args.destination)
