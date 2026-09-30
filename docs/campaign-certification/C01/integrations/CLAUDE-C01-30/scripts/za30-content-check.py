import pathlib,json,re,html.parser,unicodedata
WT=pathlib.Path("D:/spheres-offload/codex-next-20260928/review-za30");OUT=pathlib.Path("D:/spheres-offload/codex-next-20260928/za30-review")
class Text(html.parser.HTMLParser):
 def __init__(self):super().__init__();self.skip=0;self.p=[]
 def handle_starttag(self,t,a):
  if t in ("script","style"):self.skip+=1
 def handle_endtag(self,t):
  if t in ("script","style"):self.skip=max(0,self.skip-1)
 def handle_data(self,d):
  if not self.skip:self.p.append(d)
def norm(s):return " ".join(unicodedata.normalize("NFKC",s).replace("’","'").replace("‘","'").replace("“",'"').replace("”",'"').split())
data=json.loads((WT/"docs/campaign-certification/C01/research/south-africa.json").read_text(encoding="utf8"));rows=[];notes=[]
for s in data["sources"]:
 if not s["id"].startswith(("za_acdp","za_ff","za_ifp")):continue
 f=next((p for p in (OUT/"bodies"/(s["id"]+".html"),OUT/"retry-bodies"/(s["id"]+".html")) if p.exists()),None)
 if not f:notes.append(s["id"]+" INACCESSIBLE");continue
 b=f.read_bytes()
 try:t=b.decode("utf8")
 except UnicodeDecodeError:t=b.decode("cp1252",errors="replace")
 p=Text();p.feed(t);text=norm(" ".join(p.p));f.with_suffix(".txt").write_text(text,encoding="utf8")
 notes.append("\nSOURCE "+s["id"]+"\nORIGINAL "+s["original_url"]+"\nPUBLISHED "+str(s["published_date"]))
 for c in s["claims"]:
  qs=re.findall(r'["“]([^"”]+)["”]',c["text"]);found=[]
  for q in qs:
   at=text.lower().find(norm(q).lower());found.append({"quote":q,"found":at>=0,"context":text[max(0,at-110):at+len(q)+140] if at>=0 else None})
  rows.append({"id":c["id"],"source_id":s["id"],"text":c["text"],"quotes":found})
  notes.append("CLAIM "+c["id"]+"\n"+c["text"]+"\n"+json.dumps(found,ensure_ascii=False))
(OUT/"content-check.json").write_text(json.dumps(rows,ensure_ascii=False,indent=2)+"\n",encoding="utf8");(OUT/"claims-review.txt").write_text("\n".join(notes)+"\n",encoding="utf8")
print("CLAIMS",len(rows),"UNMATCHED",json.dumps([{"id":r["id"],"quotes":[q["quote"] for q in r["quotes"] if not q["found"]]} for r in rows if any(not q["found"] for q in r["quotes"])],ensure_ascii=False))

