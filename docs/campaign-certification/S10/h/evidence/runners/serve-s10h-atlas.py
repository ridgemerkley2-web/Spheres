"""Local read-only character review server; never serves campaign save paths."""
from http.server import SimpleHTTPRequestHandler,ThreadingHTTPServer
from pathlib import Path
from urllib.parse import unquote,urlsplit
import sys
ROOT=Path(__file__).resolve().parent/'integration'
class Handler(SimpleHTTPRequestHandler):
 def __init__(self,*args,**kwargs):super().__init__(*args,directory=str(ROOT),**kwargs)
 def do_GET(self):
  name=unquote(urlsplit(self.path).path).lstrip('/')
  target=(ROOT/name).resolve()
  if not target.is_relative_to(ROOT) or not any(name.startswith(p) for p in ['tools/ui/','docs/campaign-certification/C01/','spheres-web/ui/','spheres-web/data/']) or not target.is_file():
   self.send_error(404);return
  super().do_GET()
 def do_HEAD(self):self.send_error(405)
 def end_headers(self):self.send_header('Cache-Control','no-store');super().end_headers()
ThreadingHTTPServer(('127.0.0.1',int(sys.argv[1])),Handler).serve_forever()
