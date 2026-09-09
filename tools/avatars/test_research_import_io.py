from pathlib import Path
import hashlib
import json
import tempfile
import unittest
from unittest.mock import patch
import research_import_io as io


class TransactionTests(unittest.TestCase):
    def test_exact_repeat_preserves_original_bytes_and_timestamp(self):
        with tempfile.TemporaryDirectory() as directory:
            path=Path(directory)/'people.json'; original=b'{"people":[]}\n';path.write_bytes(original)
            timestamp=path.stat().st_mtime_ns
            self.assertEqual(io.write_transaction([(path,{'people':[]})]),[])
            self.assertEqual(path.read_bytes(),original)
            self.assertEqual(path.stat().st_mtime_ns,timestamp)

    def test_legacy_noop_normalizes_only_newlines_then_is_byte_stable(self):
        with tempfile.TemporaryDirectory() as directory:
            path=Path(directory)/'people.json'
            original=b'{\r\n  "people": [], "note": "escaped\\r\\nline"\r\n}\r\n'
            path.write_bytes(original);value=json.loads(original)
            self.assertEqual(io.write_transaction([(path,value)],{path:original}),[str(path.resolve())])
            normalized=original.replace(b'\r\n',b'\n')
            self.assertEqual(path.read_bytes(),normalized)
            self.assertEqual(json.loads(normalized),value)
            self.assertNotIn(b'\r',normalized)
            provenance=hashlib.sha256(normalized).hexdigest();timestamp=path.stat().st_mtime_ns
            self.assertEqual(io.write_transaction([(path,value)]),[])
            self.assertEqual(hashlib.sha256(path.read_bytes()).hexdigest(),provenance)
            self.assertEqual(path.stat().st_mtime_ns,timestamp)

    def test_changed_legacy_json_and_new_receipt_share_lf_roundtrip_bytes(self):
        with tempfile.TemporaryDirectory() as directory:
            path=Path(directory)/'people.json';receipt=Path(directory)/'receipt.json'
            path.write_bytes(b'{"people":[]}\r\n')
            value={'people':['Kaifu'], 'note':'native 海\r\nline'}
            encoded=(json.dumps(value,ensure_ascii=False,indent=2)+'\n').encode('utf8')
            provenance={'source_sha256':hashlib.sha256(encoded).hexdigest()}
            io.write_transaction([(path,value),(receipt,provenance)])
            self.assertEqual(path.read_bytes(),encoded)
            self.assertEqual(json.loads(path.read_bytes()),value)
            self.assertNotIn(b'\r',path.read_bytes());self.assertNotIn(b'\r',receipt.read_bytes())
            self.assertEqual(json.loads(receipt.read_bytes())['source_sha256'],hashlib.sha256(path.read_bytes()).hexdigest())
            self.assertEqual(io.write_transaction([(path,value),(receipt,provenance)]),[])

    def test_failed_second_replacement_restores_first_file_and_receipt(self):
        with tempfile.TemporaryDirectory() as directory:
            a,b,c=[Path(directory)/f'{name}.json' for name in ['registry','manifest','receipt']]
            old_a=b'{"old":"registry"}\r\n';old_b=b'{"old":"manifest"}\n'
            a.write_bytes(old_a);b.write_bytes(old_b)
            real=io.os.replace;calls=0
            def replace(source,target):
                nonlocal calls
                calls+=1
                if calls==2:raise OSError('simulated second output failure')
                return real(source,target)
            with patch.object(io.os,'replace',side_effect=replace):
                with self.assertRaises(OSError):
                    io.write_transaction([(a,{'new':1}),(b,{'new':2}),(c,{'receipt':'success'})])
            self.assertEqual(a.read_bytes(),old_a);self.assertEqual(b.read_bytes(),old_b)
            self.assertFalse(c.exists());self.assertEqual(sorted(p.name for p in Path(directory).iterdir()),['manifest.json','registry.json'])

    def test_preflight_stage_failure_or_stale_snapshot_leaves_every_target_unchanged(self):
        with tempfile.TemporaryDirectory() as directory:
            a=Path(directory)/'registry.json';a.write_bytes(b'{"old":1}\n')
            original=a.read_bytes()
            with self.assertRaises(OSError):
                io.write_transaction([(a,{'new':1}),(Path(directory)/'missing'/'receipt.json',{})])
            self.assertEqual(a.read_bytes(),original)
            with self.assertRaises(ValueError):
                io.write_transaction([(a,{'new':1})],{a:b'{"outdated":1}'})
            self.assertEqual(a.read_bytes(),original)

    def test_aliasing_output_paths_and_receipts_are_rejected(self):
        with tempfile.TemporaryDirectory() as directory:
            root=Path(directory).resolve();registry=root/'registry.json';source=root/'research.json'
            with self.assertRaises(ValueError):io.write_transaction([(registry,{}),(registry,{})])
            for receipt in [registry,source]:
                with self.assertRaises(ValueError):io.import_paths(source,receipt,root,{registry})
            with self.assertRaises(ValueError):io.import_paths(registry,None,root,{registry})


if __name__=='__main__':unittest.main(verbosity=2)
