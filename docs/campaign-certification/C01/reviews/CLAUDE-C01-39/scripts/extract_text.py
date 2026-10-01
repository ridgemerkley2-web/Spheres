"""Create external reading aids from exact original bodies, never replace originals."""
import hashlib
import html.parser
import json
import pathlib
import re

RECEIPT = pathlib.Path(__file__).resolve().parents[1]
journal = json.loads((RECEIPT / 'source-attempts.json').read_text(encoding='utf-8'))


class Text(html.parser.HTMLParser):
    def __init__(self):
        super().__init__(convert_charrefs=True)
        self.skip = 0
        self.parts = []

    def handle_starttag(self, tag, attrs):
        if tag in ('script', 'style'):
            self.skip += 1
        if tag in ('p', 'div', 'br', 'tr', 'li', 'h1', 'h2', 'h3', 'h4') and not self.skip:
            self.parts.append('\n')

    def handle_endtag(self, tag):
        if tag in ('script', 'style'):
            self.skip = max(0, self.skip - 1)
        if tag in ('p', 'div', 'tr', 'li', 'h1', 'h2', 'h3', 'h4') and not self.skip:
            self.parts.append('\n')

    def handle_data(self, data):
        if not self.skip:
            self.parts.append(data)


sources = {}
for attempt in journal['attempts']:
    if not attempt['exact'] or attempt['source_id'] in sources:
        continue
    raw = pathlib.Path(attempt['body_path']).read_bytes()
    assert hashlib.sha256(raw).hexdigest() == attempt['expected_sha256']
    metadata = {}
    if raw.lstrip().startswith(b'{'):
        post = json.loads(raw)
        metadata = {key: post[key] for key in ('id', 'date', 'date_gmt', 'modified', 'modified_gmt', 'slug', 'status', 'link')}
        markup = post['title']['rendered'] + '\n' + post['content']['rendered']
        encoding = 'utf-8 (JSON post title and content.rendered)'
    else:
        found = re.search(rb'charset\s*=\s*["\']?([a-zA-Z0-9_-]+)', raw, re.I)
        encoding = found.group(1).decode('ascii') if found else 'utf-8'
        try:
            markup = raw.decode(encoding)
        except UnicodeDecodeError:
            encoding = 'windows-1252'
            markup = raw.decode(encoding)
    parser = Text()
    parser.feed(markup)
    lines = [re.sub(r'\s+', ' ', line).strip() for line in ''.join(parser.parts).splitlines()]
    lines = [line for line in lines if line]
    output = pathlib.Path(attempt['body_path']).with_suffix('.text.txt')
    text = '\n'.join(f'{n:03d} {line}' for n, line in enumerate(lines, 1)) + '\n'
    output.write_text(text, encoding='utf-8')
    sources[attempt['source_id']] = {'source_id': attempt['source_id'], 'url': attempt['url'],
        'body_sha256': hashlib.sha256(raw).hexdigest(), 'text_path': str(output),
        'text_sha256': hashlib.sha256(output.read_bytes()).hexdigest(), 'lines': len(lines),
        'decode': encoding, 'post_metadata': metadata}
(RECEIPT / 'source-texts.json').write_text(json.dumps({'method': 'Read-only HTML parsing; scripts/styles excluded, whitespace normalized, numbered text reading aids. Full raw responses retained externally. Text extraction is not independent historical approval.', 'sources': list(sources.values())}, indent=2) + '\n', encoding='utf-8')
print('Extracted', len(sources), 'exact original responses')
