"""Produce external readable review inputs without substituting for original bytes."""
import argparse
import hashlib
import json
import pathlib
import re
from html.parser import HTMLParser

class Text(HTMLParser):
    def __init__(self):
        super().__init__(convert_charrefs=True)
        self.parts = []
        self.hidden = 0
    def handle_starttag(self, tag, attrs):
        if tag in ['script', 'style']:
            self.hidden += 1
        if tag in ['p', 'div', 'br', 'h1', 'h2', 'h3', 'h4', 'tr', 'li']:
            self.parts.append('\n')
    def handle_endtag(self, tag):
        if tag in ['script', 'style']:
            self.hidden -= 1
        if tag in ['p', 'div', 'h1', 'h2', 'h3', 'h4', 'tr', 'li']:
            self.parts.append('\n')
    def handle_data(self, data):
        if self.hidden == 0:
            self.parts.append(data)

parser = argparse.ArgumentParser(description=__doc__)
parser.add_argument('--repo', required=True, type=pathlib.Path)
parser.add_argument('--external', required=True, type=pathlib.Path)
args = parser.parse_args()
here = pathlib.Path(__file__).resolve().parent
packet = json.loads((args.repo / 'docs/campaign-certification/C01/research/japan.json').read_text(encoding='utf8'))
verified = {}
for receipt in sorted(here.glob('retrieval-*/retrieval.json')):
    for row in json.loads(receipt.read_text(encoding='utf8'))['results']:
        if row['exact']:
            verified.setdefault(row['id'], row)
decoded = args.external / 'decoded'
decoded.mkdir(exist_ok=True)
manifest = []
for source in [s for s in packet['sources'] if s['id'].startswith('jp_komeito_')]:
    if source['id'] not in verified:
        continue
    record = verified[source['id']]
    raw = pathlib.Path(record['body_external']).read_bytes()
    assert len(raw) == record['expected_bytes'] and hashlib.sha256(raw).hexdigest() == record['expected_sha256']
    extract = json.loads((args.repo / source['snapshot']['path']).read_text(encoding='utf8'))
    declared_encoding = extract.get('source_character_encoding', 'utf8')
    encoding = 'cp932' if declared_encoding == 'Shift_JIS (cp932)' else declared_encoding.split(' (', 1)[0]
    text = raw.decode(encoding)
    if 'kokkai.ndl.go.jp' in source['url']:
        data = json.loads(text)
        records = data.get('speechRecord') or [s for m in data.get('meetingRecord', []) for s in m.get('speechRecord', [])]
        selected = set()
        for claim in source['claims']:
            loc = claim['locator']
            n = loc.get('speechOrder')
            if isinstance(n, list):
                selected.update(n)
            elif n is not None:
                selected.add(n)
        records = [r for r in records if not selected or r['speechOrder'] in selected]
        readable = '\n\n'.join(json.dumps({k: v for k, v in row.items() if k != 'speech'}, ensure_ascii=False) + '\n' + row['speech'] for row in records)
    else:
        parser = Text()
        parser.feed(text)
        readable = '\n'.join(line.strip() for line in ''.join(parser.parts).splitlines() if line.strip())
    path = decoded / (source['id'] + '.txt')
    path.write_text(readable, encoding='utf8')
    manifest.append({'id': source['id'], 'original_external': record['body_external'], 'original_sha256': record['sha256'], 'encoding': encoding, 'readable_external': str(path), 'readable_sha256': hashlib.sha256(path.read_bytes()).hexdigest(), 'claims': source['claims']})
(args.external / 'readable-manifest.json').write_text(json.dumps(manifest, ensure_ascii=False, indent=2) + '\n', encoding='utf8')
print('Decoded', len(manifest), 'verified original responses for review. This is not a content acceptance decision.')
