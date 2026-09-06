#!/usr/bin/env python3
"""Extract native ETOPO2022 60-second elevation into demand-loaded global tiles.

Run from the repo root with numpy, h5py and Pillow available:
  python -B tools/terrain/make_terrain_tiles.py --source /path/to/ETOPO_2022_v1_60s_N90W180_surface.nc
  python -B tools/terrain/make_terrain_tiles.py --source /path/to/ETOPO_2022_v1_60s_N90W180_surface.nc --check

Each 10-degree tile contains 600x600 ORIGINAL cell-centre samples, north-up,
surrounded by one neighbouring sample on each side. No projection, averaging,
interpolation, invented peaks or vertical exaggeration is baked into this data.
The 602x602 RGB8 PNG stores big-endian RG uint16 over -1500..9000 metres; B=0.
Ocean samples below -1500m are clipped. Fully constant packed tiles need no PNG:
their exact code is in the manifest. Global coverage includes both poles.

The one-pixel gutter supports bilinear/native-gradient sampling. Independent
tile mipmaps need additional seam treatment and are not generated here.
"""
from pathlib import Path
import argparse
import hashlib
import io
import json
import math
import struct
import zlib

import h5py
import numpy as np
import PIL
from PIL import Image

import make_detail_height as height

ROOT = Path(__file__).resolve().parents[2]
OUTPUT = ROOT / 'spheres-web/ui/terrain-tiles'
INTERIOR, GUTTER, COLUMNS, ROWS = 600, 1, 36, 18
PIXELS, DEGREES, SAMPLES_PER_DEGREE = 602, 10, 60
require, digest, decode, png_bytes = height.require, height.digest, height.decode, height.png_bytes


def encode(values):
    # z is float32; arithmetic in float64 prevents a half-code rounding error.
    return height.encode(np.asarray(values, dtype=np.float64))


def tile_key(x, y):
    return f'x{x:02d}_y{y:02d}'


def native_band(z, y, interior=INTERIOR, columns=COLUMNS, rows=ROWS):
    """Read one latitude strip, preserving source samples and polar gutters."""
    require(z.shape == (interior * rows, interior * columns), 'Unexpected tile source shape')
    require(0 <= y < rows, 'Tile row out of range')
    source_rows = np.clip(z.shape[0] - 1 - (y * interior + np.arange(interior + 2) - 1),
                          0, z.shape[0] - 1)
    first, last = int(source_rows.min()), int(source_rows.max())
    strip = z[first:last + 1, :]
    return strip[source_rows - first, :]


def native_tile(band, x, interior=INTERIOR, columns=COLUMNS):
    require(0 <= x < columns, 'Tile column out of range')
    require(band.shape == (interior + 2, interior * columns), 'Unexpected native strip shape')
    source_columns = (x * interior + np.arange(interior + 2) - 1) % band.shape[1]
    return band[:, source_columns]


def validate_png(blob, packed):
    require(blob[:8] == b'\x89PNG\r\n\x1a\n', 'Invalid PNG signature')
    offset, chunks = 8, []
    while offset < len(blob):
        require(offset + 12 <= len(blob), 'Truncated PNG chunk')
        size = struct.unpack_from('>I', blob, offset)[0]
        kind = blob[offset + 4:offset + 8]
        data = blob[offset + 8:offset + 8 + size]
        require(offset + 12 + size <= len(blob), 'Truncated PNG data')
        crc = struct.unpack_from('>I', blob, offset + 8 + size)[0]
        require(zlib.crc32(kind + data) & 0xffffffff == crc, 'PNG chunk CRC mismatch')
        require(kind not in (b'gAMA', b'sRGB', b'iCCP'), 'PNG contains color management chunks')
        chunks.append(kind.decode('ascii'))
        if kind == b'IHDR':
            require(struct.unpack('>IIBBBBB', data) ==
                    (packed.shape[1], packed.shape[0], 8, 2, 0, 0, 0), 'Expected RGB8 tile dimensions')
        offset += 12 + size
    require(offset == len(blob) and chunks[0] == 'IHDR' and chunks[-1] == 'IEND', 'Invalid PNG chunk structure')
    with Image.open(io.BytesIO(blob)) as image:
        require(image.mode == 'RGB' and np.array_equal(np.asarray(image), packed), 'PNG pixel round trip changed')
    require(not np.any(packed[..., 2]), 'Blue channel must be zero')
    return sorted(set(chunks))


def write_or_check(path, blob, check):
    if check:
        require(path.is_file() and path.read_bytes() == blob, 'Regenerated artifact differs: ' + path.name)
    else:
        path.write_bytes(blob)


def landmark_records(z):
    points = [('Everest coordinate', 86.925, 27.9917), ('Mont Blanc coordinate', 6.865, 45.833),
              ('Grand Canyon coordinate', -112.14, 36.06), ('Dead Sea coordinate', 35.50, 31.50),
              ('Tokyo Bay coordinate', 139.90, 35.50), ('Amazon coordinate', -62.0, -3.0),
              ('Gibraltar coordinate', -5.35, 36.14), ('Cape Horn coordinate', -67.27, -55.98)]
    records = []
    for name, lon, lat in points:
        # Cell containing the coordinate, computed independently of tile extraction.
        col = min(21599, max(0, math.floor((lon + 180) * 60)))
        north_row = min(10799, max(0, math.floor((90 - lat) * 60)))
        source_row = 10799 - north_row
        x, y = col // INTERIOR, north_row // INTERIOR
        px, py = col % INTERIOR + 1, north_row % INTERIOR + 1
        value = float(z[source_row, col])
        tile = native_tile(native_band(z, y), x)
        require(float(tile[py, px]) == value, 'Independent landmark cell mismatch: ' + name)
        coded = encode(np.array([[value]]))
        records.append({'name': name, 'longitude': lon, 'latitude': lat, 'tile': tile_key(x, y),
                        'pixel_xy': [px, py], 'source_row_column': [source_row, col],
                        'source_cell_centre_lon_lat': [-180 + (col + .5) / 60, 90 - (north_row + .5) / 60],
                        'source_metres': value, 'encoded_metres': float(decode(coded)[0, 0])})
    return records


def rust_embed(tiles):
    lines = ['// Generated by tools/terrain/make_terrain_tiles.py. Do not edit.',
             '// Only nonconstant PNG tiles are embedded; manifest.json describes all 648 tiles.',
             'pub fn terrain_tile(name: &str) -> Option<&\'static [u8]> {', '    match name {']
    for tile in tiles.values():
        if tile['file'] is not None:
            name = tile['file']
            lines.append(f'        "{name}" => Some(include_bytes!("{name}")),')
    lines.extend(['        _ => None,', '    }', '}', ''])
    return '\n'.join(lines).encode('utf-8')


def build(source, output, check):
    require(output.resolve().name == 'terrain-tiles', 'Output directory must be named terrain-tiles')
    require(not output.is_symlink(), 'Output must not be a symlink')
    if not check:
        output.mkdir(parents=True, exist_ok=True)
    tiles, total_bytes, constant_count = {}, 0, 0
    previous_south = None
    max_error, lowest, highest = 0.0, math.inf, -math.inf
    clipped_low = clipped_high = 0
    tile_peak = None
    untouched = {name: digest(ROOT / 'spheres-web/ui' / name)
                 for name in ('relief.png', 'coast.png', 'cover.png', 'lake.png', 'height-detail.png')}
    with h5py.File(source, 'r') as file:
        source_info = height.validate_source(source, file)
        z = file['z']
        landmarks = landmark_records(z)
        print('Pinned NOAA source validated; extracting original 60-second cells.', flush=True)
        for y in range(ROWS):
            band = native_band(z, y)
            current_south, first_west, previous_east = [], None, None
            for x in range(COLUMNS):
                values = native_tile(band, x).astype(np.float64)
                packed = encode(values)
                core = values[1:-1, 1:-1]
                error = float(np.max(np.abs(decode(packed) - np.clip(values, height.ELEV_LO, height.ELEV_HI))))
                require(error <= height.STEP_METRES / 2 + 1e-9, 'Quantization exceeds half a code')
                max_error = max(max_error, error)
                lowest = min(lowest, float(core.min()))
                if float(core.max()) > highest:
                    highest = float(core.max())
                    cy, cx = np.unravel_index(np.argmax(core), core.shape)
                    tile_peak = {'tile': tile_key(x, y), 'pixel_xy': [int(cx) + 1, int(cy) + 1],
                                 'longitude': -180 + (x * 600 + int(cx) + .5) / 60,
                                 'latitude': 90 - (y * 600 + int(cy) + .5) / 60,
                                 'source_metres': highest}
                clipped_low += int(np.count_nonzero(core < height.ELEV_LO))
                clipped_high += int(np.count_nonzero(core > height.ELEV_HI))
                if previous_east is not None:
                    require(np.array_equal(previous_east, packed[:, :2]), 'East/west gutter seam mismatch')
                else:
                    first_west = packed[:, :2].copy()
                previous_east = packed[:, -2:].copy()
                if previous_south is not None:
                    require(np.array_equal(previous_south[x], packed[:2]), 'North/south gutter seam mismatch')
                if y == 0:
                    require(np.array_equal(packed[0], packed[1]), 'North pole gutter is not clamped')
                if y == ROWS - 1:
                    require(np.array_equal(packed[-1], packed[-2]), 'South pole gutter is not clamped')
                current_south.append(packed[-2:].copy())
                key = tile_key(x, y)
                code = int(packed[0, 0, 0]) * 256 + int(packed[0, 0, 1])
                constant = bool(np.all(packed == packed[0, 0]))
                name, blob = None, None
                if constant:
                    constant_count += 1
                else:
                    name = key + '.png'
                    blob = png_bytes(packed)
                    validate_png(blob, packed)
                    write_or_check(output / name, blob, check)
                    total_bytes += len(blob)
                tiles[key] = {'x': x, 'y': y, 'west': -180 + x * 10, 'east': -170 + x * 10,
                              'north': 90 - y * 10, 'south': 80 - y * 10,
                              'file': name, 'constant_code': code if constant else None,
                              'bytes': len(blob) if blob is not None else 0,
                              'sha256': hashlib.sha256(blob).hexdigest() if blob is not None else None,
                              'source_minimum_metres': float(core.min()), 'source_maximum_metres': float(core.max())}
            require(np.array_equal(previous_east, first_west), 'Antimeridian gutter seam mismatch')
            previous_south = current_south
            print(f'Latitude band {y + 1}/{ROWS}: {len(tiles)} tiles, {total_bytes:,} PNG bytes.', flush=True)
    require(highest <= height.ELEV_HI, 'Encoding clips a source peak')
    manifest = {
        'schema': 1, 'product': 'Native ETOPO2022 geographic elevation tiles',
        'source': {'name': height.SOURCE_NAME, 'product_url': height.PRODUCT_URL, 'download_url': height.SOURCE_URL,
                   'citation': 'NOAA National Centers for Environmental Information (2022), ETOPO 2022, doi:10.25921/fd45-gt74.',
                   'bytes': height.SOURCE_BYTES, 'sha256': height.SOURCE_SHA256, **source_info},
        'grid': {'columns': COLUMNS, 'rows': ROWS, 'tile_degrees': DEGREES,
                 'interior_pixels': INTERIOR, 'gutter_pixels': GUTTER, 'png_pixels': PIXELS,
                 'samples_per_degree': SAMPLES_PER_DEGREE, 'sample_spacing_arcseconds': 60,
                 'bounds': [-180, -90, 180, 90], 'row_order': 'north to south',
                 'column_order': 'west to east', 'cell_registration': 'centres',
                 'nominal_north_south_spacing_km': 1.85,
                 'resolution_note': 'Native 60-arc-second grid spacing; about 1.85km north-south, longitude spacing decreases with latitude. Grid spacing is not a claim of uniform survey accuracy.',
                 'key': 'x{column:02d}_y{row:02d}',
                 'source_row': 'clamp(10799 - (600*y + png_row - 1), 0, 10799)',
                 'source_column': '(600*x + png_column - 1) modulo 21600',
                 'pixel_centre_longitude': 'west + (png_column - 0.5) / 60',
                 'pixel_centre_latitude': 'north - (png_row - 0.5) / 60',
                 'texture_u': '(1 + (longitude - west) * 60) / 602',
                 'texture_v': '(1 + (north - latitude) * 60) / 602',
                 'gutters': 'One exact neighbouring native cell. Longitude wraps; latitude clamps to terminal source row.',
                 'mipmaps': 'None. Independent tile mipmaps require additional seam treatment.'},
        'encoding': {'mode': 'RGB8', 'red_green': 'Big-endian uint16', 'blue': 0,
                     'minimum_metres': height.ELEV_LO, 'maximum_metres': height.ELEV_HI,
                     'step_metres': height.STEP_METRES, 'decode': '(R * 256 + G) * (10500 / 65535) - 1500',
                     'sampling': 'Exact native source cells, no spatial resampling, averaging or exaggeration',
                     'interpolation': 'Decode RG to scalar elevation before interpolation or mip reduction; do not interpolate packed channels.',
                     'constant_tiles': 'file=null: every pixel including gutters has constant_code; synthesize this exact uint16 code.',
                     'maximum_quantization_error_metres': max_error, 'clipped_below_minimum_samples': clipped_low,
                     'clipped_above_maximum_samples': clipped_high, 'source_minimum_metres': lowest, 'source_maximum_metres': highest},
        'validation': {'native_sample_count': 21600 * 10800, 'all_tile_gutters_match': True,
                       'antimeridian_wrap_matches': True, 'polar_gutters_clamped': True,
                       'png_crcs_and_pixel_round_trips_exact': True, 'color_chunks_absent': True,
                       'blue_channel_all_zero': True, 'landmark_cell_mapping_independently_checked': True},
        'landmark_note': 'Original source cell at the supplied coordinate; not a surveyed summit height.',
        'landmarks': landmarks, 'highest_native_cell': tile_peak,
        'statistics': {'tile_count': len(tiles), 'png_tiles': len(tiles) - constant_count,
                       'constant_tiles': constant_count, 'total_png_bytes': total_bytes},
        'tiles': tiles,
        'reproducibility': {'numpy': np.__version__, 'h5py': h5py.__version__, 'pillow': PIL.__version__,
                            'source_code_hash_normalization': 'UTF-8 text with LF line endings',
                            'generator_sha256': height.source_code_digest(__file__),
                            'encoding_source_helper_sha256': height.source_code_digest(height.__file__)}}
    manifest_bytes = (json.dumps(manifest, indent=2, sort_keys=True) + '\n').encode('utf-8')
    if check:
        recorded = json.loads((output / 'manifest.json').read_text(encoding='utf-8'))
        require({k: v for k, v in recorded.items() if k != 'reproducibility'} ==
                {k: v for k, v in manifest.items() if k != 'reproducibility'}, 'Regenerated tile manifest differs')
    else:
        write_or_check(output / 'manifest.json', manifest_bytes, False)
    embed = rust_embed(tiles)
    write_or_check(output / 'embed.rs', embed, check)
    expected = {'manifest.json', 'embed.rs'} | {t['file'] for t in tiles.values() if t['file'] is not None}
    require({p.name for p in output.iterdir()} == expected, 'Unexpected or stale artifacts in terrain-tiles directory')
    require(all(digest(ROOT / 'spheres-web/ui' / name) == before for name, before in untouched.items()),
            'An existing terrain asset changed')
    print(json.dumps({'status': 'checked' if check else 'written', 'output': str(output),
                      **manifest['statistics'], 'manifest_sha256': digest(output / 'manifest.json'),
                      'maximum_quantization_error_metres': max_error, 'highest_native_cell': tile_peak,
                      'existing_terrain_assets_unchanged': True}, indent=2))


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('--source', type=Path, required=True)
    parser.add_argument('--output', type=Path, default=OUTPUT)
    parser.add_argument('--check', action='store_true', help='Regenerate and compare all artifacts; write nothing.')
    args = parser.parse_args()
    build(args.source.resolve(), args.output.resolve(), args.check)


if __name__ == '__main__':
    main()
