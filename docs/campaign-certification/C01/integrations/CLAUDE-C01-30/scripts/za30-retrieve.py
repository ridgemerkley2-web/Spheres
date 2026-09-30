import concurrent.futures,datetime,gzip,hashlib,json,pathlib,re,subprocess,time
WT=pathlib.Path("D:/spheres-offload/codex-next-20260928/review-za30")
OUT=pathlib.Path("D:/spheres-offload/codex-next-20260928/za30-review");OUT.mkdir(exist_ok=False);(OUT/"bodies").mkdir()
data=json.loads((WT/"docs/campaign-certification/C01/research/south-africa.json").read_text(encoding="utf8"))
sources=[s for s in data["sources"] if s["id"].startswith(("za_acdp","za_ff","za_ifp"))];assert len(sources)==39
def get(s):
 e=json.loads((WT/s["snapshot"]["path"]).read_text(encoding="utf8"));body=OUT/"bodies"/(s["id"]+".bin");head=body.with_suffix(".headers")
 cmd=["curl.exe","--silent","--show-error","--location","--max-time","25","--max-redirs","3","--dump-header",str(head),"--output",str(body),"--write-out","%{http_code}\\n%{url_effective}",s["url"]]
 t=time.time();p=subprocess.run(cmd,capture_output=True,text=True)
 r={"source_id":s["id"],"url":s["url"],"started_utc":datetime.datetime.fromtimestamp(t,datetime.timezone.utc).isoformat(),"elapsed_seconds":time.time()-t,"exit_code":p.returncode,"response":p.stdout,"error":p.stderr,"expected_bytes":e["source_response_bytes"],"expected_sha256":e["source_response_sha256"],"body_path":str(body)}
 if body.exists():
  b=body.read_bytes();r.update(bytes=len(b),sha256=hashlib.sha256(b).hexdigest())
  r["exact"]=r["bytes"]==r["expected_bytes"] and r["sha256"]==r["expected_sha256"]
  if r["exact"]:
   decoded=gzip.decompress(b) if b.startswith(b"\x1f\x8b") else b
   r["decoded_bytes"]=len(decoded);r["decoded_sha256"]=hashlib.sha256(decoded).hexdigest()
   if "decoded_response_sha256" in e:assert r["decoded_sha256"]==e["decoded_response_sha256"]
   body.with_suffix(".html").write_bytes(decoded)
 else:r["exact"]=False
 print(json.dumps({"id":s["id"],"exact":r["exact"],"status":p.stdout.splitlines()[:1],"error":p.returncode}),flush=True)
 return r
with concurrent.futures.ThreadPoolExecutor(max_workers=3) as pool:rows=list(pool.map(get,sources))
(OUT/"retrieval.json").write_text(json.dumps({"reviewed_head":"cb0f1153f0d26729445acbc05843c3f16bf78ee7","recipe":"plain curl.exe without Accept-Encoding or automatic decompression; original bodies untouched","sources":rows},indent=2)+"\n",encoding="utf8")
print("EXACT",sum(r["exact"] for r in rows),"/",len(rows))

