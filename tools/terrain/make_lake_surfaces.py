#!/usr/bin/env python
"""Transcribe six lake water surfaces from HydroLAKES; preserve shipped shores.

The original ETOPO relief stores beds in these six large lakes. This small
render-only table supplies water elevations from HydroLAKES v1's Elevation
attribute instead. It does not derive water levels from lake-bed elevations.

python tools/terrain/make_lake_surfaces.py --hydrolakes /path/to/HydroLAKES_points_v10_shp.zip \
    --natural-earth /path/to/ne_10m_lakes.geojson [--check]
"""
import argparse
import hashlib
import json
import re
import struct
import zipfile
from pathlib import Path

ROOT = Path(__file__).resolve().parents[2]
TARGET = ROOT / 'spheres-web/ui/lake-surfaces.json'
SOURCE_SHA = '587df22034639230899498775d6272966c93adcfbda831fc704bb36b6cc9d681'
LAKES = {'Lake Baikal': 11, 'Lake Erie': 9, 'Lake Huron': 8,
         'Lake Michigan': 6, 'Lake Ontario': 7, 'Lake Superior': 5}


def sha(data):
    return hashlib.sha256(data).hexdigest()


def fnv(text):
    value = 2166136261
    for byte in text.encode('ascii'):
        value = ((value ^ byte) * 16777619) & 0xffffffff
    return value


def build(args):
    archive = Path(args.hydrolakes).read_bytes()
    if sha(archive) != SOURCE_SHA:
        raise ValueError('HydroLAKES source differs from the reviewed download')
    with zipfile.ZipFile(args.hydrolakes) as zipped:
        dbf = zipped.read(next(name for name in zipped.namelist() if name.endswith('.dbf')))
    count, header, length = struct.unpack_from('<IHH', dbf, 4)
    fields, offset = [], 1
    for i in range(32, header - 1, 32):
        name = dbf[i:i + 11].split(b'\0')[0].decode('ascii')
        size = dbf[i + 16]
        fields.append((name, offset, size))
        offset += size
    rows = {}
    for i in range(count):
        start = header + i * length
        record_id = int(dbf[start + 1:start + 10].strip())
        if record_id not in LAKES.values():
            continue
        rows[record_id] = {name: dbf[start + off:start + off + size].decode('utf-8').strip()
                           for name, off, size in fields}
        if len(rows) == len(LAKES):
            break
    if len(rows) != len(LAKES):
        raise ValueError('A reviewed lake is absent from HydroLAKES')
    # Reuse the exact existing projection/simplification functions without
    # executing make_rivers.py's independent generation/write pipeline.
    helper = ROOT / 'tools/terrain/make_rivers.py'
    namespace = {'__file__': str(helper)}
    exec(helper.read_text(encoding='utf-8').split('# --- rivers ')[0], namespace)
    rivers = (ROOT / 'spheres-web/ui/rivers.js').read_text(encoding='utf-8')
    paths = json.loads(re.search(r'lakes:(\[.*\])};', rivers, re.S)[1])
    natural_earth_bytes = Path(args.natural_earth).read_bytes()
    features = json.loads(natural_earth_bytes)['features']
    result = []
    for feature in features:
        name = feature['properties'].get('name')
        if name not in LAKES:
            continue
        geometry = feature['geometry']
        polygons = geometry['coordinates'] if geometry['type'] == 'MultiPolygon' else [geometry['coordinates']]
        rings = [[point[:2] for point in polygon[0]] for polygon in polygons if polygon and len(polygon[0]) >= 4]
        path = namespace['path_closed']([namespace['project_part'](ring) for ring in rings])
        if paths.count(path) != 1:
            raise ValueError(f'{name} no longer matches exactly one shipped shoreline')
        record = rows[LAKES[name]]
        result.append({'name': name, 'lake_index': paths.index(path), 'path_fnv1a': fnv(path),
                       'path_sha256': sha(path.encode('ascii')), 'surface_metres': int(record['Elevation']),
                       'hydrolakes_id': LAKES[name], 'hydrolakes_name': record['Lake_name'],
                       'hydrolakes_elevation_field': record['Elevation']})
    if len(result) != len(LAKES):
        raise ValueError('A reviewed lake is absent from Natural Earth')
    result.sort(key=lambda lake: lake['lake_index'])
    return {'product': 'Water-surface correction for the six rendered ETOPO bed lakes',
            'scope': 'Visual terrain and picking only; no simulation or historical water-level model.',
            'method': 'Transcribe HydroLAKES v1 Elevation. Apply only inside the exact existing RIVERS.lakes path, verified against the original Natural Earth projection pipeline. Other lake and land elevations remain NOAA ETOPO.',
            'limitations': 'HydroLAKES elevations are DEM-derived representative water levels, not surveyed summit values or a reconstruction of 1990 lake levels.',
            'source': {'title': 'HydroLAKES version 1.0',
                       'url': 'https://www.hydrosheds.org/products/hydrolakes',
                       'download_url': 'https://data.hydrosheds.org/file/hydrolakes/HydroLAKES_points_v10_shp.zip',
                       'archive_bytes': len(archive), 'archive_sha256': SOURCE_SHA,
                       'documentation': 'https://data.hydrosheds.org/file/technical-documentation/HydroLAKES_TechDoc_v10.pdf',
                       'field': 'Elevation: lake water surface in metres above sea level',
                       'license': 'Creative Commons Attribution 4.0 International',
                       'license_url': 'https://creativecommons.org/licenses/by/4.0/',
                       'attribution': 'Messager, M.L., Lehner, B., Grill, G., Nedeva, I., Schmitt, O. (2016). Estimating the volume and age of water stored in global lakes using a geo-statistical approach. Nature Communications 7, 13603.',
                       'doi': 'https://doi.org/10.1038/ncomms13603'},
            'shoreline_source': {'title': 'Natural Earth 1:10m lakes; existing simplified game paths',
                                 'url': 'https://www.naturalearthdata.com/downloads/10m-physical-vectors/10m-lakes/',
                                 'geojson_sha256': sha(natural_earth_bytes), 'license': 'Public domain',
                                 'rivers_js_sha256': sha(rivers.encode('utf-8'))},
            'lakes': result}


if __name__ == '__main__':
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('--hydrolakes', required=True)
    parser.add_argument('--natural-earth', required=True)
    parser.add_argument('--check', action='store_true')
    args = parser.parse_args()
    result = build(args)
    encoded = (json.dumps(result, ensure_ascii=False, indent=2) + '\n').encode('utf-8')
    if args.check:
        if TARGET.read_bytes() != encoded:
            raise SystemExit('Lake-surface provenance differs from regeneration')
    else:
        TARGET.write_bytes(encoded)
    print(json.dumps([[lake['lake_index'], lake['surface_metres'], lake['path_fnv1a']] for lake in result['lakes']]))
    print(f'{len(result["lakes"])} sourced water surfaces; {len(encoded)} bytes; SHA256 {sha(encoded)}')
