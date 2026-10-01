"""One coordinated, sequential original-response pass; never retries a request."""
import datetime as dt
import gzip
import hashlib
import json
import subprocess
import time
from html.parser import HTMLParser
from pathlib import Path

ROOT = Path('D:/spheres-offload/codex-next-20260928/review-c01-46-20261001')
OUT = Path(__file__).resolve().parent / 'attempt-01'

def stamp():
    return dt.datetime.now(dt.timezone.utc).isoformat()

def pin(path):
    data = path.read_bytes()
    return {'path': str(path), 'bytes': len(data), 'sha256': hashlib.sha256(data).hexdigest()}

class Text(HTMLParser):
    def __init__(self):
        super().__init__(convert_charrefs=True)
        self.hidden = 0
        self.parts = []
    def handle_starttag(self, tag, attrs):
        if tag in ('script', 'style'): self.hidden += 1
        if tag in ('p', 'div', 'h1', 'h2', 'h3', 'li', 'br', 'time'): self.parts.append('\n')
    def handle_endtag(self, tag):
        if tag in ('script', 'style') and self.hidden: self.hidden -= 1
        if tag in ('p', 'div', 'h1', 'h2', 'h3', 'li', 'time'): self.parts.append('\n')
    def handle_data(self, data):
        if not self.hidden: self.parts.append(data)

def main():
    OUT.mkdir(parents=True, exist_ok=False)
    packet = json.loads((ROOT / 'docs/campaign-certification/C01/research/russia.json').read_text(encoding='utf-8'))
    sources = packet['sources'][170:]
    attempts = []
    stopped = False
    for number, source in enumerate(sources):
        if stopped:
            attempts.append({'source_id': source['id'], 'url': source['url'], 'status': 'not_attempted_after_429'})
            continue
        if number: time.sleep(10)
        sid = source['id']
        body, headers = OUT / (sid+'.body'), OUT / (sid+'.headers')
        command = ['curl.exe', '--silent', '--show-error', '--location', '--max-time', '45', '--retry', '0',
                   '--dump-header', str(headers), '--output', str(body), '--write-out', '%{json}', source['url']]
        started = stamp()
        result = subprocess.run(command, capture_output=True)
        (OUT / (sid+'.curl.stdout')).write_bytes(result.stdout)
        (OUT / (sid+'.curl.stderr')).write_bytes(result.stderr)
        try: curl = json.loads(result.stdout)
        except (ValueError, UnicodeDecodeError): curl = {}
        row = {'source_id': sid, 'url': source['url'], 'started_utc': started, 'finished_utc': stamp(),
               'command': command, 'exit_code': result.returncode, 'http_status': curl.get('http_code'),
               'curl': curl, 'files': [pin(p) for p in (body, headers, OUT/(sid+'.curl.stdout'), OUT/(sid+'.curl.stderr')) if p.exists()]}
        extract = json.loads((ROOT / source['snapshot']['path']).read_text(encoding='utf-8'))
        row['submitted_pin'] = {'bytes': extract['source_response_bytes'], 'sha256': extract['source_response_sha256']}
        if body.exists():
            data = body.read_bytes()
            row['matches_submitted_pin'] = len(data) == row['submitted_pin']['bytes'] and hashlib.sha256(data).hexdigest() == row['submitted_pin']['sha256']
            if curl.get('http_code') == 200:
                decoded = gzip.decompress(data) if data[:2] == b'\x1f\x8b' else data
                decoded_path = OUT / (sid+'.decoded.html')
                decoded_path.write_bytes(decoded)
                parser=Text(); parser.feed(decoded.decode('utf-8', errors='replace'))
                text_path=OUT/(sid+'.text.txt')
                text_path.write_text('\n'.join(x.strip() for x in ''.join(parser.parts).splitlines() if x.strip())+'\n', encoding='utf-8', newline='\n')
                row['files'] += [pin(decoded_path), pin(text_path)]
        attempts.append(row)
        (OUT/'attempts.json').write_text(json.dumps(attempts,ensure_ascii=False,indent=2)+'\n',encoding='utf-8',newline='\n')
        print(sid, row['http_status'], row.get('matches_submitted_pin'), flush=True)
        stopped = row['http_status'] == 429
    (OUT/'attempts.json').write_text(json.dumps(attempts,ensure_ascii=False,indent=2)+'\n',encoding='utf-8',newline='\n')

if __name__ == '__main__': main()
