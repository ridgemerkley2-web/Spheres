"""Source-independent shoreline binding checks; no downloads or asset writes.

Run: python -B tools/terrain/test_lake_surfaces.py
"""
import copy
import json
import re
import unittest
from unittest.mock import patch

import make_lake_surfaces as lakes


# A small synthetic peninsula survives the detailed shoreline tolerance, but
# disappears under the old river tolerance. Golden projected coordinates keep
# this test independent of the helper used to regenerate the path.
RING = [[0, 0], [.5, .006], [1, 0], [1, 1], [0, 1], [0, 0]]
PATH = 'M1200 585.14L1203.33 585.1 1206.67 585.14 1206.66 577.6 1200 577.6Z'
COARSE_PATH = 'M1200 585.14L1206.67 585.14 1206.66 577.6 1200 577.6Z'


class ShippedLakeProvenanceTests(unittest.TestCase):
    def test_recorded_source_digest_and_every_binding_match_shipped_paths(self):
        table = json.loads(lakes.TARGET.read_text(encoding='utf-8'))
        rivers = (lakes.ROOT / 'spheres-web/ui/rivers.js').read_text(encoding='utf-8')
        self.assertEqual(table['shoreline_source']['rivers_js_sha256'],
                         lakes.sha(rivers.encode('utf-8')))
        paths = json.loads(re.search(r'lakes:(\[.*\])};', rivers, re.S)[1])
        self.assertEqual({item['name'] for item in table['lakes']}, set(lakes.LAKES))
        self.assertEqual(len(table['lakes']), len(lakes.LAKES))
        for item in table['lakes']:
            with self.subTest(lake=item['name']):
                path = paths[item['lake_index']]
                self.assertEqual(paths.count(path), 1)
                self.assertEqual(item['path_sha256'], lakes.sha(path.encode('ascii')))
                self.assertEqual(item['path_fnv1a'], lakes.fnv(path))
                self.assertEqual(item['hydrolakes_id'], lakes.LAKES[item['name']])


class LakeSurfaceTests(unittest.TestCase):
    def setUp(self):
        self.features = [{'properties': {'name': 'Test Lake'},
                          'geometry': {'type': 'Polygon', 'coordinates': [copy.deepcopy(RING)]}}]
        self.rows = {9: {'Lake_name': 'Source lake name', 'Elevation': '172'}}
        self.lake_names = patch.object(lakes, 'LAKES', {'Test Lake': 9})
        self.lake_names.start()
        self.addCleanup(self.lake_names.stop)

    def build(self, paths):
        return lakes.surface_records(self.features, self.rows, paths)

    def test_detailed_shore_binds_at_its_actual_index_and_keeps_source_level(self):
        result = self.build(['unrelated shoreline', PATH])
        self.assertEqual(result, [{
            'name': 'Test Lake', 'lake_index': 1,
            'path_fnv1a': lakes.fnv(PATH),
            'path_sha256': lakes.sha(PATH.encode('ascii')),
            'surface_metres': 172, 'hydrolakes_id': 9,
            'hydrolakes_name': 'Source lake name',
            'hydrolakes_elevation_field': '172'}])

    def test_coarse_shore_is_not_accepted_as_a_nearby_match(self):
        with self.assertRaisesRegex(ValueError, 'exactly one shipped shoreline'):
            self.build([COARSE_PATH])

    def test_shifted_shore_is_rejected(self):
        with self.assertRaisesRegex(ValueError, 'exactly one shipped shoreline'):
            self.build([PATH.replace('1203.33', '1203.34')])

    def test_ambiguous_shipped_shores_are_rejected(self):
        with self.assertRaisesRegex(ValueError, 'exactly one shipped shoreline'):
            self.build([PATH, PATH])

    def test_duplicate_source_lake_is_rejected(self):
        self.features *= 2
        with self.assertRaisesRegex(ValueError, 'appears more than once'):
            self.build([PATH])

    def test_missing_source_lake_is_rejected(self):
        self.features.clear()
        with self.assertRaisesRegex(ValueError, 'absent from Natural Earth'):
            self.build([PATH])

    def test_polygon_holes_and_source_altitudes_do_not_change_exterior(self):
        geometry = self.features[0]['geometry']
        geometry['coordinates'][0] = [point + [999] for point in RING]
        geometry['coordinates'].append([[.2, .2], [.3, .2], [.3, .3], [.2, .2]])
        self.assertEqual(self.build([PATH])[0]['path_sha256'], lakes.sha(PATH.encode('ascii')))

    def test_multipolygon_exteriors_keep_each_ring(self):
        geometry = self.features[0]['geometry']
        geometry['type'] = 'MultiPolygon'
        geometry['coordinates'] = [[copy.deepcopy(RING)], [copy.deepcopy(RING)]]
        self.assertEqual(self.build([PATH + PATH])[0]['lake_index'], 0)

    def test_nonpolygon_and_empty_shores_are_rejected(self):
        self.features[0]['geometry']['type'] = 'LineString'
        with self.assertRaisesRegex(ValueError, 'no polygon shoreline'):
            self.build([PATH])
        self.features[0]['geometry'] = {'type': 'Polygon', 'coordinates': []}
        with self.assertRaisesRegex(ValueError, 'exactly one shipped shoreline'):
            self.build([''])


if __name__ == '__main__':
    unittest.main()
