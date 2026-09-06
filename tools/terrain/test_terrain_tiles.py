"""Small, source-independent checks for native terrain tile extraction.

Run: python -B tools/terrain/test_terrain_tiles.py
The 6 x 12 source is deliberately asymmetric, with south-to-north rows and
unique samples. No NOAA download or generated terrain artifacts are required.
"""
from pathlib import Path
import struct
import tempfile
import unittest
import zlib

import numpy as np

import make_terrain_tiles as t


class NativeTileTests(unittest.TestCase):
    def setUp(self):
        self.source = np.arange(72, dtype=np.float32).reshape(6, 12)

    def band(self, y):
        return t.native_band(self.source, y, interior=3, columns=4, rows=2)

    def tile(self, x, y):
        return t.native_tile(self.band(y), x, interior=3, columns=4)

    def test_interiors_cover_every_original_sample_once_north_up(self):
        rows = [np.concatenate([self.tile(x, y)[1:-1, 1:-1]
                                for x in range(4)], axis=1) for y in range(2)]
        assembled = np.concatenate(rows, axis=0)
        np.testing.assert_array_equal(assembled, self.source[::-1])
        self.assertEqual(assembled.dtype, self.source.dtype)
        self.assertEqual(np.unique(assembled).size, self.source.size)

    def test_northwest_cells_wrap_longitude_and_duplicate_polar_row(self):
        expected = [[71, 60, 61, 62, 63],
                    [71, 60, 61, 62, 63],
                    [59, 48, 49, 50, 51],
                    [47, 36, 37, 38, 39],
                    [35, 24, 25, 26, 27]]
        np.testing.assert_array_equal(self.tile(0, 0), expected)

    def test_southeast_cells_wrap_longitude_and_duplicate_polar_row(self):
        expected = [[44, 45, 46, 47, 36],
                    [32, 33, 34, 35, 24],
                    [20, 21, 22, 23, 12],
                    [8, 9, 10, 11, 0],
                    [8, 9, 10, 11, 0]]
        np.testing.assert_array_equal(self.tile(3, 1), expected)

    def test_packed_seams_share_both_adjacent_native_cells(self):
        tiles = {(x, y): t.encode(self.tile(x, y))
                 for y in range(2) for x in range(4)}
        for y in range(2):
            for x in range(4):
                with self.subTest(x=x, y=y, axis='longitude'):
                    np.testing.assert_array_equal(tiles[x, y][:, -2:],
                                                  tiles[(x + 1) % 4, y][:, :2])
        for x in range(4):
            with self.subTest(x=x, axis='latitude'):
                np.testing.assert_array_equal(tiles[x, 0][-2:], tiles[x, 1][:2])
                np.testing.assert_array_equal(tiles[x, 0][0], tiles[x, 0][1])
                np.testing.assert_array_equal(tiles[x, 1][-1], tiles[x, 1][-2])

    def test_invalid_grid_shape_and_tile_indices_are_rejected(self):
        with self.assertRaises(ValueError):
            t.native_band(self.source[:, :-1], 0, interior=3, columns=4, rows=2)
        for y in (-1, 2):
            with self.subTest(y=y), self.assertRaises(ValueError):
                self.band(y)
        for x in (-1, 4):
            with self.subTest(x=x), self.assertRaises(ValueError):
                self.tile(x, 0)
        with self.assertRaises(ValueError):
            t.native_tile(self.band(0)[:-1], 0, interior=3, columns=4)

    def test_tile_names_include_padded_coordinates(self):
        self.assertEqual(t.tile_key(0, 0), 'x00_y00')
        self.assertEqual(t.tile_key(3, 9), 'x03_y09')
        self.assertEqual(t.tile_key(35, 17), 'x35_y17')


class ElevationPackingTests(unittest.TestCase):
    def test_fixed_range_clipping_and_zero_blue(self):
        heights = np.array([[-2000, -1500, -427, 0, 1000, 9000, 10000]], dtype=np.float64)
        expected = [[[0, 0, 0], [0, 0, 0], [26, 41, 0], [36, 146, 0],
                     [60, 244, 0], [255, 255, 0], [255, 255, 0]]]
        packed = t.encode(heights)
        np.testing.assert_array_equal(packed, expected)
        self.assertEqual(packed.dtype, np.uint8)
        self.assertLessEqual(float(np.max(np.abs(t.decode(packed) -
                                                np.clip(heights, -1500, 9000)))),
                             10500 / 65535 / 2 + 1e-9)

    def test_red_green_carry_preserves_adjacent_codes(self):
        codes = np.array([[254, 255, 256, 257, 511, 512]], dtype=np.float64)
        packed = t.encode(-1500 + codes * (10500 / 65535))
        expected = [[[0, 254, 0], [0, 255, 0], [1, 0, 0],
                     [1, 1, 0], [1, 255, 0], [2, 0, 0]]]
        np.testing.assert_array_equal(packed, expected)
        np.testing.assert_allclose(np.diff(t.decode(packed)[0])[:3],
                                   [10500 / 65535] * 3, rtol=0, atol=1e-12)

    def test_float32_source_is_promoted_before_half_code_rounding(self):
        # These are exact float32 values immediately around encoding midpoints.
        # Performing the subtraction/division in float32 rounds the wrong way.
        values = np.array([[-1454.0970458984375, 8997.8369140625,
                            8998.4775390625, 8999.599609375, 8999.919921875]],
                          dtype=np.float32)
        packed = t.encode(values)
        codes = packed[..., 0].astype(np.uint16) * 256 + packed[..., 1]
        np.testing.assert_array_equal(codes, [[287, 65521, 65525, 65533, 65535]])
        self.assertLessEqual(float(np.max(np.abs(t.decode(packed) - values.astype(np.float64)))),
                             10500 / 65535 / 2 + 1e-9)


class PngAndArtifactTests(unittest.TestCase):
    def setUp(self):
        self.packed = t.encode(np.array([[-1500, -427, 0], [1000, 4500, 9000]], dtype=np.float64))
        self.blob = t.png_bytes(self.packed)

    def test_png_preserves_non_square_rgb_bytes_without_color_management(self):
        chunks = t.validate_png(self.blob, self.packed)
        self.assertIn('IHDR', chunks)
        self.assertIn('IDAT', chunks)
        self.assertIn('IEND', chunks)
        self.assertTrue(set(chunks).isdisjoint({'gAMA', 'sRGB', 'iCCP'}))

    def test_valid_crc_color_management_chunks_are_rejected(self):
        insertion = 8 + 12 + struct.unpack_from('>I', self.blob, 8)[0]
        for kind, data in [(b'gAMA', struct.pack('>I', 45455)),
                           (b'sRGB', b'\x00'), (b'iCCP', b'test\x00\x00' + zlib.compress(b'profile'))]:
            chunk = (struct.pack('>I', len(data)) + kind + data +
                     struct.pack('>I', zlib.crc32(kind + data) & 0xffffffff))
            changed = self.blob[:insertion] + chunk + self.blob[insertion:]
            with self.subTest(chunk=kind), self.assertRaisesRegex(ValueError, 'color management'):
                t.validate_png(changed, self.packed)

    def test_corrupt_crc_and_truncated_chunks_are_rejected(self):
        corrupted = bytearray(self.blob)
        corrupted[29] ^= 1  # IHDR CRC; image bytes and chunk structure remain intact.
        with self.assertRaisesRegex(ValueError, 'CRC'):
            t.validate_png(bytes(corrupted), self.packed)
        with self.assertRaisesRegex(ValueError, 'Truncated'):
            t.validate_png(self.blob[:-2], self.packed)

    def test_nonzero_blue_and_changed_pixels_are_rejected(self):
        blue = self.packed.copy()
        blue[0, 0, 2] = 1
        with self.assertRaisesRegex(ValueError, 'Blue'):
            t.validate_png(t.png_bytes(blue), blue)
        changed = self.packed.copy()
        changed[0, 0, 1] = 1
        with self.assertRaisesRegex(ValueError, 'round trip'):
            t.validate_png(self.blob, changed)

    def test_check_accepts_exact_artifact_and_detects_mutation_without_writing(self):
        with tempfile.TemporaryDirectory(prefix='spheres-terrain-tile-test-') as directory:
            path = Path(directory) / 'tile.png'
            t.write_or_check(path, self.blob, False)
            t.write_or_check(path, self.blob, True)
            modified = self.blob + b'modified'
            path.write_bytes(modified)
            with self.assertRaisesRegex(ValueError, 'differs'):
                t.write_or_check(path, self.blob, True)
            self.assertEqual(path.read_bytes(), modified)

    def test_check_rejects_missing_artifact_without_creating_it(self):
        with tempfile.TemporaryDirectory(prefix='spheres-terrain-tile-test-') as directory:
            path = Path(directory) / 'missing.png'
            with self.assertRaisesRegex(ValueError, 'differs'):
                t.write_or_check(path, self.blob, True)
            self.assertFalse(path.exists())


if __name__ == '__main__':
    unittest.main(verbosity=2)
