"""Validated research bounds and staged, recoverable multi-file JSON writes.

Each replacement is atomic. Ordinary write errors roll the group back; this is
not a filesystem-wide crash transaction. Expected snapshots reject stale input.
"""
from __future__ import annotations
from calendar import monthrange
from datetime import date
import json
import os
from pathlib import Path
import re
import tempfile


def snapshot(path: Path) -> bytes | None:
    return path.read_bytes() if path.exists() else None


def read_snapshot(path: Path):
    raw = path.read_bytes()
    return json.loads(raw.decode('utf-8-sig')), raw


def import_paths(source: Path, receipt: Path | None, root: Path, protected):
    if not source.is_relative_to(root) or source in protected:
        raise ValueError('Research source must be inside the repository and distinct from output files')
    if receipt and (not receipt.is_relative_to(root) or receipt in {*protected, source}):
        raise ValueError('Receipt must be inside the repository and distinct from source, registry, manifest and policy')


def write_transaction(entries, expected=None):
    """Stage every output; preserve exact no-op bytes when already LF-compliant."""
    expected = {Path(p).resolve(): v for p, v in (expected or {}).items()}
    planned, paths = [], set()
    for path, value in entries:
        path = Path(path).resolve()
        if path in paths:
            raise ValueError('Duplicate/colliding output path')
        paths.add(path)
        old = snapshot(path)
        if path in expected and old != expected[path]:
            raise ValueError(f'Concurrent edit detected before import: {path}')
        # JSON provenance hashes must match the LF bytes enforced by .gitattributes.
        # Preserve compliant no-op formatting and timestamps; a legacy newline-only
        # correction must not reserialize or otherwise change the document.
        if old is not None and json.loads(old.decode('utf-8-sig')) == value:
            new = old.replace(b'\r\n', b'\n').replace(b'\r', b'\n')
            if new == old:
                continue
        else:
            new = (json.dumps(value, ensure_ascii=False, indent=2) + '\n').encode('utf-8')
        planned.append((path, old, new))
    temporary, prepared, committed = set(), [], []

    def stage(path, content):
        fd, name = tempfile.mkstemp(prefix=f'.{path.name}.import-', suffix='.tmp', dir=path.parent)
        result = Path(name)
        temporary.add(result)
        with os.fdopen(fd, 'wb') as handle:
            handle.write(content)
            handle.flush()
            os.fsync(handle.fileno())
        return result

    try:
        for path, old, new in planned:
            prepared.append((path, old, new, stage(path, new), stage(path, old) if old is not None else None))
        for path, original in expected.items():
            if snapshot(path) != original:
                raise ValueError(f'Concurrent edit detected before import: {path}')
        for path, old, new, staged, backup in prepared:
            if snapshot(path) != old:
                raise ValueError(f'Concurrent edit detected during import: {path}')
            os.replace(staged, path)
            committed.append((path, old, new, backup))
    except BaseException as original_error:
        rollback_errors = []
        for path, old, new, backup in reversed(committed):
            try:
                if snapshot(path) != new:
                    raise ValueError('target changed externally; newer content was not overwritten')
                if old is None:
                    path.unlink()
                else:
                    os.replace(backup, path)
            except (OSError, ValueError) as error:
                rollback_errors.append(f'{path}: {error}')
        if rollback_errors:
            raise RuntimeError('Import failed; rollback needs review: ' + '; '.join(rollback_errors)) from original_error
        raise
    finally:
        for path in temporary:
            path.unlink(missing_ok=True)
    return [str(path) for path, _, _ in planned]


def bound(value, *, allow_open=False):
    if not isinstance(value, dict) or set(value) - {'kind', 'value'}:
        raise ValueError('Invalid researched date bound')
    kind, text = value.get('kind'), value.get('value')
    if kind == 'unknown' and text is None:
        return None
    if kind == 'open' and text is None and allow_open:
        return None
    formats = {'day': r'[0-9]{4}-[0-9]{2}-[0-9]{2}', 'month': r'[0-9]{4}-[0-9]{2}', 'year': r'[0-9]{4}'}
    if kind not in formats or not isinstance(text, str) or not re.fullmatch(formats[kind], text):
        raise ValueError('Invalid researched date bound')
    first = date.fromisoformat(text + {'day': '', 'month': '-01', 'year': '-01-01'}[kind])
    last = (date(first.year, 12, 31) if kind == 'year' else
            date(first.year, first.month, monthrange(first.year, first.month)[1]) if kind == 'month' else first)
    return first, last


def life(first, last):
    a = bound(first) if first is not None else None
    b = bound(last) if last is not None else None
    if a and b and a[0] > b[1]:
        raise ValueError('Reversed researched life/organization dates')


def window(first, last):
    a, b = bound(first), bound(last, allow_open=True)
    if a and b and a[0] >= b[1]:
        raise ValueError('Empty or reversed researched office window')
