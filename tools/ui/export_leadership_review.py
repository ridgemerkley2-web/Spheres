#!/usr/bin/env python3
"""Enrich native read-only leadership snapshots with strictly validated era art.

Run the native export separately. This script never starts a server or reads a
user campaign, and never edits its input, either manifest, or production code.
"""
from __future__ import annotations
import argparse
from copy import deepcopy
import hashlib
import json
from pathlib import Path
import sys

ROOT = Path(__file__).resolve().parents[2]
sys.path.insert(0, str(ROOT / 'tools/avatars'))
import person_art_pipeline as historical_art
import fictional_art_pipeline as fictional_art
from research_import_io import read_snapshot, write_transaction


def portrait_at(records, when):
    matches = [p for p in records if p['from'] <= when and (p.get('to') is None or when < p['to'])]
    if len(matches) != 1:
        return None
    p = matches[0]
    prefix = 'spheres-web/ui/person-portraits/'
    if not p['asset'].startswith(prefix):
        return None
    name = p['asset'][len(prefix):]
    if '/' in name or '\\' in name or '..' in name:
        return None
    value = {k: p.get(k) for k in ('credit', 'method', 'style', 'status', 'from', 'to',
        'source_url', 'license', 'license_url', 'composition', 'era_note')}
    identity = p.get('identity_source') or {}
    value.update(url='/art/people/' + name,
        source_license=identity.get('license'),
        source_license_url=identity.get('license_url'),
        source_credit=identity.get('credit'))
    return value


def enrich_board(board, real, fiction):
    # Native JSON has distinct occurrences for each context. Preserve that
    # separation even when a test/caller shares one Python person dictionary.
    result = json.loads(json.dumps(board))
    reference_date = board['date']
    campaign_date = board['campaign_date']

    def walk(value, when, preview=False):
        if isinstance(value, list):
            for item in value:
                walk(item, when, preview)
        elif isinstance(value, dict):
            for key, child in list(value.items()):
                walk(child, campaign_date if key in ('campaign', 'executive_person') else when,
                     preview or key == 'future_preview')
            if isinstance(value.get('id'), str) and isinstance(value.get('name'), str):
                pid = value['id']
                is_fiction = (value.get('fiction') or {}).get('origin') == 'fictional_successor'
                art_date = '2026-09-08' if is_fiction and preview else when
                value['portrait'] = portrait_at((fiction if is_fiction else real).get(pid, []), art_date)
                if value['portrait'] and is_fiction:
                    value['portrait']['origin'] = 'fictional_successor'
                    if preview:
                        value['portrait'].update(presentation_context='fictional_future_preview', preview_date=art_date)
    walk(result, reference_date)
    return result


def build(source, real, fiction):
    if source.get('read_only') is not True or source.get('campaign_date') != '1990-01-01' or source.get('historical_reference_through') != '2026-09-07':
        raise ValueError('Requires the unchanged native read-only 1990/2030 exporter contract')
    result = deepcopy(source)
    for nation in result['nations']:
        for key, date in [('reference_1990', '1990-01-01'), ('future_reference_2030', '2030-01-01')]:
            board = nation[key]
            if board.get('date') != date or board.get('nation') != nation['nation'] or board.get('error'):
                raise ValueError('Mismatched or invalid native snapshot')
            nation[key] = enrich_board(board, real, fiction)
    result['presentation'] = {
        'purpose': 'Static visual review using the production GovernmentUI renderer',
        'sample_campaign_only': True,
        'user_save_loaded': False,
        'commands_available': False,
        'portrait_selection': 'Exact person and half-open appearance era, after strict physical manifest validation',
        'art_route_rewrite': 'Only rendered /art/people filenames are mapped to the repository person-portraits directory',
    }
    return result


def main():
    cli = argparse.ArgumentParser(description=__doc__)
    cli.add_argument('--input', required=True, type=Path)
    cli.add_argument('--output', type=Path, default=ROOT / 'tools/ui/leadership-government-review.json')
    args = cli.parse_args()
    source_path, output_path = args.input.resolve(), args.output.resolve()
    registry_path = ROOT / 'spheres-sim/data/party_leaders.json'
    manifest_path = ROOT / 'spheres-web/data/person_portraits.json'
    fictional_path = ROOT / 'spheres-web/data/fictional_portraits.json'
    catalog_path = ROOT / 'spheres-web/data/future_candidates_2035.json'
    inputs = [source_path, registry_path, manifest_path, fictional_path, catalog_path]
    if output_path in inputs or not output_path.is_relative_to(ROOT / 'tools/ui'):
        raise ValueError('Output must be a separate static review fixture inside tools/ui')
    reads = {p: read_snapshot(p) for p in inputs}
    source, registry, manifest, fictional_manifest, catalog = [reads[p][0] for p in inputs]
    real = historical_art.validate_manifest(manifest, ROOT, historical_art.people_from_registry(registry))
    fiction = fictional_art.validate(fictional_manifest, catalog, ROOT, manifest)
    for checked in (real, fiction):
        if not checked['valid']:
            raise ValueError('Portrait validation failed: ' + '; '.join(checked['errors']))
    result = build(source, real['ready'], fiction['ready'])
    result['source_hashes'] = {p.relative_to(ROOT).as_posix() if p.is_relative_to(ROOT) else p.name:
                             hashlib.sha256(reads[p][1]).hexdigest() for p in inputs}
    write_transaction([(output_path, result)], expected={p: reads[p][1] for p in inputs})
    print(json.dumps({'fixture': str(output_path), 'nations': len(result['nations']),
        'validated_historical_portraits': sum(map(len, real['ready'].values())),
        'validated_fictional_portraits': sum(map(len, fiction['ready'].values())),
        'source_files_unchanged': True, 'server_started': False}))


if __name__ == '__main__':
    main()
