#!/usr/bin/env python3
"""Restore one original compressed record from inventory.json into a NEW file.
Usage: python restore-record.py INVENTORY LOGICAL_PATH NEW_OUTPUT
"""
import hashlib,json,pathlib,sys,zipfile
inventory=pathlib.Path(sys.argv[1]).resolve(); data=json.loads(inventory.read_text(encoding='utf-8'))
row=next(r for r in data['source_files'] if r['logical_path']==sys.argv[2])
output=pathlib.Path(sys.argv[3]).resolve(); assert not output.exists(),'Refuse to overwrite output'
blob=data['compressed_objects'][row['storage']['object']]; total=hashlib.sha256(); size=0
with output.open('xb') as result:
 for part in blob['parts']:
  archive_path=(inventory.parent/part['file']).resolve()
  assert archive_path.is_relative_to(inventory.parent),'Unsafe archive path'
  with archive_path.open('rb') as stream: assert hashlib.file_digest(stream,'sha256').hexdigest()==part['stored_sha256']
  member_hash=hashlib.sha256(); member_bytes=0
  with zipfile.ZipFile(archive_path) as archive,archive.open(part['member']) as source:
   while chunk:=source.read(1024*1024):
    result.write(chunk);total.update(chunk);member_hash.update(chunk);size+=len(chunk);member_bytes+=len(chunk)
  assert member_bytes==part['source_bytes'] and member_hash.hexdigest()==part['source_sha256']
assert size==row['bytes'] and total.hexdigest()==row['sha256'],'Reconstruction differs'
print(str(output)+' verified SHA256 '+total.hexdigest())
