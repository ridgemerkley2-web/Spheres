import pathlib,json,subprocess,hashlib,gzip,time
OUT=pathlib.Path("D:/spheres-offload/codex-next-20260928/za30-review")
initial=json.loads((OUT/"retrieval.json").read_text());dest=OUT/"retry-bodies";dest.mkdir(exist_ok=False);rows=[]
for old in initial["sources"]:
 if old["exact"]:continue
 body=dest/(old["source_id"]+".bin");headers=body.with_suffix(".headers")
 p=subprocess.run(["curl.exe","--silent","--show-error","--location","--max-time","25","--max-redirs","3","--dump-header",str(headers),"--output",str(body),"--write-out","%{http_code}\\n%{url_effective}",old["url"]],capture_output=True,text=True)
 r={k:old[k] for k in ("source_id","url","expected_bytes","expected_sha256")};r.update(exit_code=p.returncode,response=p.stdout,error=p.stderr,body_path=str(body),exact=False)
 if body.exists():
  raw=body.read_bytes();r.update(bytes=len(raw),sha256=hashlib.sha256(raw).hexdigest());r["exact"]=r["bytes"]==r["expected_bytes"] and r["sha256"]==r["expected_sha256"]
  if r["exact"]:body.with_suffix(".html").write_bytes(gzip.decompress(raw) if raw.startswith(b"\x1f\x8b") else raw)
 rows.append(r);print(r["source_id"],r["exact"],p.returncode,flush=True);time.sleep(.2)
(OUT/"retrieval-retry.json").write_text(json.dumps({"scope":"One normal sequential missing-only retry; initial failures retained","sources":rows},indent=2)+"\n",encoding="utf8")
print("EXACT",sum(r["exact"] for r in rows),"/",len(rows))

