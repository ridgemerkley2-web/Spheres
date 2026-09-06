#!/usr/bin/env python3
"""Bake a separate 4800x2036 ETOPO elevation texture; existing relief is untouched.

Requires numpy, h5py and Pillow. Download NOAA's original source outside the repo:
https://www.ncei.noaa.gov/products/etopo-global-relief-model
Then run from the repo root:
  python -B tools/terrain/make_detail_height.py --source /path/to/ETOPO_2022_v1_60s_N90W180_surface.nc
  python -B tools/terrain/make_detail_height.py --source /path/to/ETOPO_2022_v1_60s_N90W180_surface.nc --check

The imported make_relief.warp uses the exact Robinson canvas extent, increasing
source latitude, bilinear sampling, longitude wrapping and off-globe edge clamp.
It is called at 9600x4072; each 2x2 block is averaged in metres before encoding.
RG stores an unsigned big-endian 16-bit code over a FIXED -1500..9000 metre range.
B is zero. This independent elevation field does not replace bathymetry or AO.
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

import make_relief as relief

ROOT = Path(__file__).resolve().parents[2]
WIDTH, HEIGHT = 4800, 2036
ELEV_LO, ELEV_HI = -1500.0, 9000.0
STEP_METRES = (ELEV_HI - ELEV_LO) / 65535.0
SOURCE_NAME = 'ETOPO_2022_v1_60s_N90W180_surface.nc'
SOURCE_BYTES = 478290125
SOURCE_SHA256 = '74bbc5c23e85188d93f0beba8116babf69e1a8f160e8dc9f053bf8df01b197b6'
PRODUCT_URL = 'https://www.ncei.noaa.gov/products/etopo-global-relief-model'
SOURCE_URL = ('https://www.ngdc.noaa.gov/thredds/fileServer/global/ETOPO2022/'
              '60s/60s_surface_elev_netcdf/' + SOURCE_NAME)
OUTPUT = ROOT / 'spheres-web/ui/height-detail.png'


def require(condition, message):
    if not condition:
        raise ValueError(message)


def digest(path):
    result = hashlib.sha256()
    with Path(path).open('rb') as stream:
        for chunk in iter(lambda: stream.read(1024 * 1024), b''):
            result.update(chunk)
    return result.hexdigest()


def source_code_digest(path):
    # Git may check the same source out with CRLF on Windows and LF elsewhere.
    return hashlib.sha256(Path(path).read_text(encoding='utf-8').encode('utf-8')).hexdigest()


def encode(height):
    code = np.rint((np.clip(height, ELEV_LO, ELEV_HI) - ELEV_LO) / STEP_METRES).astype(np.uint16)
    packed = np.zeros((*code.shape, 3), dtype=np.uint8)
    packed[..., 0] = (code >> 8).astype(np.uint8)
    packed[..., 1] = (code & 255).astype(np.uint8)
    return packed


def decode(packed):
    return (packed[..., 0].astype(np.float64) * 256.0 + packed[..., 1]) * STEP_METRES + ELEV_LO


def png_bytes(packed):
    out = io.BytesIO()
    Image.fromarray(packed).save(out, format='PNG', optimize=True, compress_level=9)
    return out.getvalue()


def validate_png(blob, packed):
    require(blob[:8] == b'\x89PNG\r\n\x1a\n', 'Invalid PNG signature')
    offset, chunks = 8, []
    while offset < len(blob):
        size = struct.unpack_from('>I', blob, offset)[0]
        kind = blob[offset + 4:offset + 8]
        data = blob[offset + 8:offset + 8 + size]
        crc = struct.unpack_from('>I', blob, offset + 8 + size)[0]
        require(zlib.crc32(kind + data) & 0xffffffff == crc, 'PNG chunk CRC mismatch')
        require(kind not in (b'gAMA', b'sRGB', b'iCCP'), 'PNG color management would corrupt packed elevation')
        chunks.append(kind.decode('ascii'))
        if kind == b'IHDR':
            require(struct.unpack('>IIBBBBB', data) == (WIDTH, HEIGHT, 8, 2, 0, 0, 0), 'Expected RGB8 dimensions')
        offset += size + 12
    require(offset == len(blob) and chunks[-1] == 'IEND', 'Truncated or trailing PNG data')
    with Image.open(io.BytesIO(blob)) as image:
        require(image.mode == 'RGB' and image.size == (WIDTH, HEIGHT), 'Unexpected decoded PNG layout')
        require(np.array_equal(np.asarray(image), packed), 'PNG did not preserve elevation bytes')
    require(not np.any(packed[..., 2]), 'B must remain zero')
    return {'chunk_types': sorted(set(chunks)), 'crc_valid': True, 'color_chunks_absent': True,
            'pixel_round_trip_exact': True, 'blue_channel_all_zero': True}


def validate_source(source, file):
    require(source.stat().st_size == SOURCE_BYTES and digest(source) == SOURCE_SHA256,
            'Source differs from the pinned official NOAA ice-surface download')
    z = file['z']
    require(z.shape == (10800, 21600) and z.dtype == np.dtype('float32'), 'Unexpected source grid')
    require(not any(name in z.attrs for name in ('scale_factor', 'add_offset')), 'Unexpected source scale/offset')
    require(bytes(z.attrs['units']) == b'meters' and bytes(z.attrs['positive']) == b'up', 'Source units differ')
    require(bytes(z.attrs['vert_crs_name']) == b'EGM2008', 'Unexpected vertical datum')
    require(int(np.asarray(file.attrs['node_offset']).reshape(-1)[0]) == 1, 'Source must use cell centres')
    lat, lon = file['lat'][:], file['lon'][:]
    require(np.all(np.diff(lat) > 0) and np.all(np.diff(lon) > 0), 'Coordinate arrays must increase')
    require(np.max(np.abs(lat - (relief.ET_LAT0 + np.arange(10800) / 60.0))) < 1e-8,
            'Latitude centres disagree with the imported sampler')
    require(np.max(np.abs(lon - (relief.ET_LON0 + np.arange(21600) / 60.0))) < 1e-8,
            'Longitude centres disagree with the imported sampler')
    fill = float(np.asarray(z.attrs['_FillValue']).reshape(-1)[0])
    low, high = math.inf, -math.inf
    for row in range(0, z.shape[0], 256):
        block = z[row:row + 256, :]
        require(np.all(np.isfinite(block)) and not np.any(block == fill), 'Source contains invalid elevations')
        low, high = min(low, float(block.min())), max(high, float(block.max()))
    return {'dataset': 'z', 'shape': list(z.shape), 'dtype': str(z.dtype), 'units': 'metres',
            'vertical_datum': 'EGM2008', 'cell_registration': True, 'latitude_increases_south_to_north': True,
            'latitude_endpoints': [float(lat[0]), float(lat[-1])],
            'longitude_endpoints': [float(lon[0]), float(lon[-1])], 'invalid_cells': 0,
            'minimum_metres': low, 'maximum_metres': high,
            'southern_row_mean_metres': float(np.mean(z[0, :], dtype=np.float64)),
            'northern_row_mean_metres': float(np.mean(z[-1, :], dtype=np.float64))}


def source_sample(z, lon, lat):
    """Independent scalar bilinear reference for selected output texels."""
    sy = (lat - relief.ET_LAT0) * relief.ET_STEP
    sx = (max(-180.0, min(180.0, lon)) - relief.ET_LON0) * relief.ET_STEP
    x0, y0 = math.floor(sx), math.floor(sy)
    fx, fy = sx - x0, sy - y0
    a = float(z[y0, x0 % relief.ET_W]) * (1 - fx) + float(z[y0, (x0 + 1) % relief.ET_W]) * fx
    b = float(z[y0 + 1, x0 % relief.ET_W]) * (1 - fx) + float(z[y0 + 1, (x0 + 1) % relief.ET_W]) * fx
    return a * (1 - fy) + b * fy


def validate_landmarks(z, heights, decoded):
    points = [('Gibraltar', -5.35, 36.14, (2333, 624)), ('Cape Horn', -67.27, -55.98, (1657, 2007)),
              ('Tokyo Bay', 139.90, 35.50, (4154, 634)), ('Everest coordinate', 86.925, 27.9917, (3518, 747)),
              ('Amazon coordinate', -62.0, -3.0, (1574, 1215)),
              ('Dead Sea', 35.50, 31.50, None), ('Mont Blanc coordinate', 6.865, 45.833, None),
              ('Mariana Trench coordinate', 142.20, 11.35, None)]
    records = []
    for name, lon, lat, expected in points:
        x, y = relief.project(lon, lat)
        px, py = math.floor(x / relief.W * WIDTH), math.floor(y / relief.H_EXT * HEIGHT)
        require(expected is None or (px, py) == expected, 'Landmark projection mismatch: ' + name)
        samples = []
        for dy in range(2):
            for dx in range(2):
                cx = (px * 2 + dx + .5) * relief.W / (WIDTH * 2)
                cy = (py * 2 + dy + .5) * relief.H_EXT / (HEIGHT * 2)
                sample_lat = float(relief.lat_from_canvas_y(cy))
                sample_lon = math.degrees((cx - relief.W / 2) / (0.8487 * relief.R * relief.interp(relief.RX, abs(sample_lat))))
                samples.append(source_sample(z, sample_lon, sample_lat))
        reference = float(np.mean(samples))
        require(abs(reference - heights[py, px]) < 1e-8, 'Independent supersample mismatch: ' + name)
        records.append({'name': name, 'longitude': lon, 'latitude': lat, 'texel_xy': [px, py],
                        'supersample_mean_metres': float(heights[py, px]),
                        'encoded_metres': float(decoded[py, px]), 'reference_error_metres': abs(reference - heights[py, px])})
    for lat in (-55.0, 0.0, 80.0):
        require(abs(source_sample(z, -180.0, lat) - source_sample(z, 180.0, lat)) < 1e-8,
                'Longitude wrap seam mismatch')
    return records


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('--source', type=Path, required=True)
    parser.add_argument('--output', type=Path, default=OUTPUT)
    parser.add_argument('--chunk-rows', type=int, default=32)
    parser.add_argument('--check', action='store_true', help='Regenerate in memory and compare both committed artifacts; write nothing.')
    args = parser.parse_args()
    require(args.chunk_rows > 0, 'chunk-rows must be positive')
    require(args.output.resolve().name == 'height-detail.png', 'Output must be named height-detail.png; existing UI assets are protected')
    require(abs(relief.H_EXT - 1018.1941195106424) < 1e-10, 'Projection extent changed')
    require(np.max(np.abs(relief.lat_from_canvas_y(np.array([0, relief.Y_TOP, relief.H_EXT])) - [83, 0, -58])) < 1e-10,
            'Projection inverse endpoints changed')
    fixture = encode(np.array([-2000, -1500, -427, 0, 1000, 9000, 10000], dtype=np.float64))
    require(np.array_equal(fixture[:, :2], [[0, 0], [0, 0], [26, 41], [36, 146], [60, 244], [255, 255], [255, 255]]),
            'Fixed elevation encoding changed')
    untouched = {name: digest(ROOT / 'spheres-web/ui' / name) for name in ('relief.png', 'coast.png', 'cover.png', 'lake.png')}
    with h5py.File(args.source, 'r') as file:
        source_info = validate_source(args.source, file)
        print('Validated original NOAA source; warping 9600x4072.', flush=True)
        high, round_trip, off_globe = relief.warp(file['z'], WIDTH * 2, HEIGHT * 2, chunk_rows=args.chunk_rows)
        require(round_trip < 1e-9 and np.all(np.isfinite(high)), 'Invalid projection warp')
        heights = high.reshape(HEIGHT, 2, WIDTH, 2).mean(axis=(1, 3))
        del high
        require(heights.max() <= ELEV_HI, 'Fixed ceiling clips a sampled summit')
        packed = encode(heights)
        decoded = decode(packed)
        quantization_error = float(np.max(np.abs(decoded - np.clip(heights, ELEV_LO, ELEV_HI))))
        require(quantization_error <= STEP_METRES / 2 + 1e-9, 'Elevation quantization exceeded half a code')
        landmarks = validate_landmarks(file['z'], heights, decoded)
    blob = png_bytes(packed)
    hygiene = validate_png(blob, packed)
    require(blob == png_bytes(packed), 'Repeated PNG serialization differed')
    provenance = {
        'schema': 1, 'product': 'Separate higher-resolution elevation texture',
        'source': {'name': SOURCE_NAME, 'product_url': PRODUCT_URL, 'download_url': SOURCE_URL,
                   'citation': 'NOAA National Centers for Environmental Information (2022), ETOPO 2022, doi:10.25921/fd45-gt74.',
                   'bytes': SOURCE_BYTES, 'sha256': SOURCE_SHA256, **source_info},
        'projection': {'name': 'Robinson, exact existing mapgen canvas', 'canvas_width': relief.W,
                       'canvas_height_exact': relief.H_EXT, 'radius': relief.R, 'y_top': relief.Y_TOP,
                       'latitude_clip': [relief.LAT_BOT, relief.LAT_TOP],
                       'max_inverse_round_trip_error_canvas_units': round_trip,
                       'off_globe_supersamples_edge_clamped': off_globe},
        'sampling': {'supersample_size': [WIDTH * 2, HEIGHT * 2], 'output_size': [WIDTH, HEIGHT],
                     'method': 'Bilinear source sampling; mean of 2x2 elevations before clipping and quantization',
                     'longitude': 'Clamp projected longitude to +/-180, then wrap source columns across seam'},
        'encoding': {'mode': 'RGB8', 'red_green': 'Big-endian uint16', 'blue': 0,
                     'minimum_metres': ELEV_LO, 'maximum_metres': ELEV_HI, 'step_metres': STEP_METRES,
                     'decode': '(R * 256 + G) * (10500 / 65535) - 1500',
                     'maximum_quantization_error_metres': quantization_error,
                     'clipped_below_minimum_texels': int(np.count_nonzero(heights < ELEV_LO)),
                     'clipped_above_maximum_texels': int(np.count_nonzero(heights > ELEV_HI)),
                     'sampled_minimum_metres': float(heights.min()), 'sampled_maximum_metres': float(heights.max())},
        'landmark_note': 'Coordinate neighbourhood samples at this texture resolution; not surveyed summit heights.',
        'landmarks': landmarks, 'png': {'file': args.output.name, 'bytes': len(blob),
                                        'sha256': hashlib.sha256(blob).hexdigest(), **hygiene},
        'reproducibility': {'same_array_png_serialization_identical': True, 'numpy': np.__version__,
                            'h5py': h5py.__version__, 'pillow': PIL.__version__,
                            'source_code_hash_normalization': 'UTF-8 text with LF line endings',
                            'generator_sha256': source_code_digest(__file__),
                            'projection_helper_sha256': source_code_digest(relief.__file__)},
    }
    metadata = (json.dumps(provenance, indent=2, sort_keys=True) + '\n').encode('utf-8')
    if args.check:
        require(args.output.read_bytes() == blob, 'Regenerated PNG differs from committed artifact')
        recorded = json.loads(args.output.with_suffix('.json').read_text(encoding='utf-8'))
        # A matching PNG remains reproducible if a compatible library version
        # or a comment-only generator change produced it. Preserve its original
        # environment record and compare all source/data/encoding facts exactly.
        require({k: v for k, v in recorded.items() if k != 'reproducibility'} ==
                {k: v for k, v in provenance.items() if k != 'reproducibility'}, 'Regenerated data provenance differs')
    else:
        args.output.write_bytes(blob)
        args.output.with_suffix('.json').write_bytes(metadata)
    require(all(digest(ROOT / 'spheres-web/ui' / name) == before for name, before in untouched.items()),
            'An existing terrain texture changed during this operation')
    print(json.dumps({'status': 'checked' if args.check else 'written', 'output': str(args.output),
                      'bytes': len(blob), 'sha256': provenance['png']['sha256'],
                      'sampled_peak_metres': provenance['encoding']['sampled_maximum_metres'],
                      'quantization_error_metres': quantization_error, 'existing_textures_unchanged': True}, indent=2))


if __name__ == '__main__':
    main()
