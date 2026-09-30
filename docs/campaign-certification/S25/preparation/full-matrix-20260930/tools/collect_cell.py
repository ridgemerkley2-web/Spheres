"""Local, create-only collection of one completed GitHub cell artifact; never launches native code."""
from __future__ import annotations
import argparse,datetime,hashlib,importlib,json,os,pathlib,re,shutil,stat,sys,traceback,zipfile

BASE=pathlib.Path(__file__).resolve().parent
MANIFEST_SHA="70a51be54a7e3100a5267ca310bf5c7f05a3c388a389b7c54904e6365c477cc8"
REVISION="5d970f6d7370baf16760585c641d81253d1c2175"
RUN_ID=36474011141
ATTEMPT=1
BRANCH="codex/s25-run-20260928"
REPO="ridgemerkley2-web/Spheres"

def require(ok,message):
    if not ok: raise ValueError(message)

def utc():
    return datetime.datetime.now(datetime.timezone.utc).isoformat()

def plain(path):
    path=pathlib.Path(path).absolute()
    for node in (path,*path.parents):
        require(not node.is_symlink() and not getattr(node,"is_junction",lambda:False)(),
                "Refuse symlink/junction path: "+str(node))
    return path

def pin(path):
    path=plain(path); require(path.is_file(),"Missing regular input: "+str(path))
    h=hashlib.sha256();size=0
    with path.open("rb") as f:
        while block:=f.read(1024*1024):h.update(block);size+=len(block)
    return {"bytes":size,"sha256":h.hexdigest()}

def no_duplicates(pairs):
    out={}
    for k,v in pairs:
        require(k not in out,"Duplicate JSON key: "+k);out[k]=v
    return out

def read_json(path):
    return json.loads(plain(path).read_text(encoding="utf-8"),object_pairs_hook=no_duplicates)

def new_json(path,value):
    with plain(path).open("x",encoding="utf-8",newline="\n") as f:
        json.dump(value,f,indent=2);f.write("\n");f.flush();os.fsync(f.fileno())

def pinned_bundle():
    frozen=plain(BASE/"frozen")
    require(pin(frozen/"manifest.json")["sha256"]==MANIFEST_SHA,"Frozen manifest changed")
    manifest=read_json(frozen/"manifest.json")
    require(manifest["revision"]==REVISION and manifest["workflow_run_id"]==str(RUN_ID)
            and manifest["workflow_run_attempt"]==str(ATTEMPT),"Wrong frozen batch")
    require(set(manifest["files"])=={"native-test","plan.json","stability_matrix.py","distributed_stability.py"},"Wrong frozen member set")
    for name,expected in manifest["files"].items():
        require(pin(frozen/name)==expected,"Frozen file changed: "+name)
    # Import only the separately pinned trusted verifier, never code from a cell ZIP.
    sys.dont_write_bytecode=True
    sys.path.insert(0,str(frozen))
    driver=importlib.import_module("distributed_stability")
    require(pathlib.Path(driver.__file__).resolve()==(frozen/"distributed_stability.py").resolve(),"Loaded wrong verifier")
    _,checked,plan=driver.bundle(frozen,MANIFEST_SHA)
    return driver,checked,plan

def metadata(artifact,job,run,cell,artifact_id,job_id,manifest):
    require(type(artifact_id) is int and artifact_id>0 and type(job_id) is int and job_id>0,"Positive expected IDs required")
    require(cell in {c["id"] for c in manifest["cells"]},"Unknown declared cell")
    require(artifact.get("id")==artifact_id,"Artifact ID mismatch")
    require(artifact.get("name")==f"full-stability-cell-{cell}-{ATTEMPT}","Artifact name/cell/attempt mismatch")
    require(type(artifact.get("size_in_bytes")) is int and artifact["size_in_bytes"]>0,"Missing exact API ZIP size")
    require(type(artifact.get("digest")) is str and re.fullmatch(r"sha256:[0-9a-f]{64}",artifact["digest"]),"Missing exact API ZIP SHA256")
    require(artifact.get("url")==f"https://api.github.com/repos/{REPO}/actions/artifacts/{artifact_id}","Wrong artifact API repository/ID")
    require(artifact.get("archive_download_url")==f"https://api.github.com/repos/{REPO}/actions/artifacts/{artifact_id}/zip","Wrong ZIP API repository/ID")
    ar=artifact.get("workflow_run",{})
    require(ar.get("id")==RUN_ID and ar.get("head_sha")==REVISION and ar.get("head_branch")==BRANCH,"Artifact run/head/branch mismatch")
    require(job.get("id")==job_id and job.get("run_id")==RUN_ID and job.get("name")==f"cell ({cell})","Job/cell/run identity mismatch")
    require(job.get("status")=="completed" and job.get("conclusion") in {"success","failure","cancelled","timed_out","neutral","skipped","action_required","stale"},"Require completed cell job")
    require(run.get("id")==RUN_ID and run.get("run_attempt")==ATTEMPT and run.get("head_sha")==REVISION and run.get("head_branch")==BRANCH,"Run/head/attempt mismatch")
    require(run.get("repository",{}).get("full_name")==REPO,"Wrong run repository")
    require(run.get("path")==".github/workflows/stability-full.yml","Wrong workflow")
    return {"bytes":artifact["size_in_bytes"],"sha256":artifact["digest"][7:]}

def safe_parts(info):
    name=info.filename
    require(info.orig_filename==name and "\0" not in name,"Truncated/NUL ZIP name")
    require(name and not name.startswith(("/","\\")) and "\\" not in name,"Absolute/backslash ZIP path")
    directory=info.is_dir()
    value=name[:-1] if directory else name
    parts=value.split("/")
    require(all(x and x not in {".",".."} for x in parts),"Empty/dot/traversal ZIP path")
    for part in parts:
        require(not any(ord(c)<32 or c in '<>:"|?*' for c in part),"Unsafe Windows ZIP path")
        require(not part.endswith((" ",".")),"Trailing Windows path alias")
        require(part.split(".")[0].upper() not in {"CON","PRN","AUX","NUL","CLOCK$",*[f"COM{i}" for i in range(1,10)],*[f"LPT{i}" for i in range(1,10)]},"Reserved Windows path")
    mode=stat.S_IFMT(info.external_attr>>16)
    require(mode in ({0,stat.S_IFDIR} if directory else {0,stat.S_IFREG}),"ZIP link/special/type mismatch")
    require(not (info.flag_bits&1),"Encrypted ZIP not supported")
    require(directory or not (info.external_attr&0x10),"DOS-directory/file mismatch")
    return parts,directory

def preflight_zip(z):
    infos=z.infolist();require(infos and len(infos)<=100000,"Empty/unbounded ZIP directory")
    explicit=set();tree={};result=[]
    for info in infos:
        parts,directory=safe_parts(info);rel="/".join(parts)
        require(rel.casefold() not in explicit,"Duplicate/case-colliding ZIP entry")
        explicit.add(rel.casefold())
        for i in range(1,len(parts)+1):
            name="/".join(parts[:i]);kind="directory" if i<len(parts) or directory else "file"
            key=name.casefold()
            require(key not in tree or tree[key]==(name,kind),"Case alias/file-directory collision")
            tree[key]=(name,kind)
        result.append((info,rel,directory))
    return result

def extract(zip_path,destination,expected):
    zip_path=plain(zip_path);destination=plain(destination)
    require(not destination.exists(),"Refuse existing shard destination")
    before=pin(zip_path);require(before==expected,"ZIP byte size/digest differs from API")
    with zipfile.ZipFile(zip_path) as z:
        rows=preflight_zip(z)
        total=sum(info.file_size for info,_,directory in rows if not directory)
        require(total+2*1024**3<=shutil.disk_usage(destination.parent).free,"Insufficient space; ZIP retained")
        destination.mkdir(exist_ok=False)
        files=[]
        for info,relative,directory in rows:
            target=plain(destination/relative)
            require(target.resolve().is_relative_to(destination.resolve()),"Output escaped shard directory")
            if directory:
                target.mkdir(parents=True,exist_ok=True);continue
            target.parent.mkdir(parents=True,exist_ok=True)
            h=hashlib.sha256();n=0
            with z.open(info) as src,target.open("xb") as dst:
                while block:=src.read(1024*1024):
                    dst.write(block);h.update(block);n+=len(block)
                    require(n<=info.file_size,"ZIP expanded beyond declared size")
                dst.flush();os.fsync(dst.fileno())
            require(n==info.file_size,"ZIP entry size mismatch")
            entry={"path":relative,"bytes":n,"sha256":h.hexdigest()}
            require(pin(target)=={k:entry[k] for k in ("bytes","sha256")},"Extracted file changed")
            files.append(entry)
    require(pin(zip_path)==before,"Source ZIP changed during extraction")
    return files

def identity(manifest,plan,cell,freeze,proof,root):
    require(freeze.get("batch_sha256")==MANIFEST_SHA==proof.get("batch_sha256"),"Different batch/retry")
    require(freeze.get("candidate_revision")==REVISION==proof.get("revision"),"Mixed candidate")
    for key,name in (("binary","native-test"),("frozen_plan","plan.json"),("frozen_harness","stability_matrix.py")):
        p=freeze.get(key,{})
        require({k:p.get(k) for k in ("bytes","sha256")}==manifest["files"][name],"Mixed frozen "+key)
    require(freeze.get("only_cell")==cell==proof.get("only_cell"),"Wrong shard assignment")
    require(freeze.get("jobs")==1==proof.get("jobs"),"Wrong shard concurrency")
    require(read_json(root/"plan.json")==plan,"Shortened/changed full plan")
    require(proof.get("passed") is False and proof.get("coverage",{}).get("full_matrix_passed") is False,"Forbidden full-matrix claim")
    require(freeze.get("qualification") is False and freeze.get("s25_complete") is False
            and proof.get("qualification") is False and proof.get("s25_complete") is False,"Forbidden qualification claim")
    outcomes=proof.get("cells")
    require(type(outcomes) is list and len(outcomes)==1 and outcomes[0].get("id")==cell,"Missing/mixed/duplicate cell")

def inspect_cell(driver,manifest,plan,cell,root,job):
    freeze=read_json(root/"freeze.json");proof=read_json(root/"result.json")
    identity(manifest,plan,cell,freeze,proof,root)
    verified=driver.matrix.verify_retained_run(root)
    require(verified.get("integrity_verified") is True and verified.get("native_reexecuted") is False,"No retained integrity result")
    require(verified.get("passed") is False and verified.get("qualification") is False
            and verified.get("s25_complete") is False,"Forbidden aggregate qualification claim")
    selected=verified.get("selected_cell_passed") is True
    if selected:
        require(job["conclusion"]=="success","Native success contradicts completed GitHub job")
        driver.validate_shard_records(manifest,plan,cell,freeze,proof,MANIFEST_SHA)
    return {"retained_integrity_verified":True,"selected_cell_passed":selected,
            "github_job_conclusion":job["conclusion"],"verification":verified}

def run(args):
    driver,manifest,plan=pinned_bundle()
    require(args.cell in {c["id"] for c in plan["cells"]},"Cell not in fixed plan")
    root=plain(BASE/"shards"/args.cell);receipt=plain(BASE/"cell-receipts"/args.cell)
    if args.command=="collect":
        require(not root.exists() and not receipt.exists(),"No overwrite/retry permitted for this cell")
        artifact_path=plain(args.artifact_metadata);job_path=plain(args.job_metadata);run_path=plain(args.run_metadata);zip_path=plain(args.zip)
        artifact=read_json(artifact_path);job=read_json(job_path);workflow=read_json(run_path)
        expected=metadata(artifact,job,workflow,args.cell,args.artifact_id,args.job_id,manifest)
        require(pin(zip_path)==expected,"Downloaded ZIP does not match API")
        # Only now reserve this cell's collection. Failed attempts remain owned and immutable.
        plain(BASE/"shards").mkdir(exist_ok=True);plain(BASE/"cell-receipts").mkdir(exist_ok=True)
        receipt.mkdir(exist_ok=False)
        inputs={}
        for name,path in (("artifact.json",artifact_path),("job.json",job_path),("run.json",run_path)):
            original=pin(path);raw=path.read_bytes()
            with (receipt/name).open("xb") as f:f.write(raw)
            require(pin(receipt/name)==original==pin(path),"Metadata changed during capture")
            inputs[name]=original
        new_json(receipt/"request.json",{"cell":args.cell,"artifact_id":args.artifact_id,"job_id":args.job_id,
                    "zip":str(zip_path),"zip_pin":expected,"metadata_pins":inputs,"manifest_sha256":MANIFEST_SHA,"created_utc":utc()})
        report_path=receipt/"collection.json"
    else:
        require(root.is_dir() and receipt.is_dir(),"Collect this cell before inspecting it")
        previous=read_json(receipt/"request.json")
        artifact=read_json(receipt/"artifact.json");job=read_json(receipt/"job.json");workflow=read_json(receipt/"run.json")
        for name,expected_pin in previous["metadata_pins"].items():require(pin(receipt/name)==expected_pin,"Retained metadata changed")
        expected=metadata(artifact,job,workflow,args.cell,previous["artifact_id"],previous["job_id"],manifest)
        require(previous["cell"]==args.cell and previous["manifest_sha256"]==MANIFEST_SHA and previous["zip_pin"]==expected,"Collection request drift")
        zip_path=plain(previous["zip"]);require(pin(zip_path)==expected,"Source ZIP changed")
        files=read_json(receipt/"extraction.json")["files"]
        actual={x.relative_to(root).as_posix() for x in root.rglob("*") if x.is_file()}
        require(actual=={x["path"] for x in files},"Missing/extra extracted files")
        for item in files:require(pin(root/item["path"])=={k:item[k] for k in ("bytes","sha256")},"Extracted bytes changed")
        report_path=receipt/("inspection-"+datetime.datetime.now(datetime.timezone.utc).strftime("%Y%m%dT%H%M%S%fZ")+".json")
    report={"format":"spheres-distributed-cell-collection/v1","cell":args.cell,"run_id":RUN_ID,
        "revision":REVISION,"attempt":ATTEMPT,"manifest_sha256":MANIFEST_SHA,"started_utc":utc(),
        "extraction_complete":args.command=="inspect","retained_integrity_verified":False,
        "selected_cell_passed":False,"full_matrix_passed":False,"qualification":False,
        "s25_complete":False,"native_executed":False,"mode":args.command}
    exit_code=2
    try:
        if args.command=="collect":
            files=extract(zip_path,root,expected)
            new_json(receipt/"extraction.json",{"source_zip":str(zip_path),"zip_pin":expected,
                "shard_root":str(root),"files":files,"completed_utc":utc()})
            report["extraction_complete"]=True
        report.update(inspect_cell(driver,manifest,plan,args.cell,root,job))
        exit_code=0 if report["selected_cell_passed"] else 1
    except Exception as exc:
        report["failure"]=type(exc).__name__+": "+str(exc)[:4000]
        with (receipt/(report_path.stem+"-failure.txt")).open("x",encoding="utf8") as f:
            f.write(traceback.format_exc())
    finally:
        try:
            report["source_zip_unchanged"]=pin(zip_path)==expected
            require(report["source_zip_unchanged"],"Source ZIP drift")
            _,again,_=pinned_bundle();require(again==manifest,"Frozen bundle drift")
            report["frozen_bundle_unchanged"]=True
        except Exception as exc:
            exit_code=2;report["selected_cell_passed"]=False
            report["postcheck_failure"]=type(exc).__name__+": "+str(exc)[:4000]
        report["finished_utc"]=utc();report["exit_code"]=exit_code
        new_json(report_path,report)
    print(json.dumps({k:report.get(k) for k in ("cell","extraction_complete","retained_integrity_verified",
        "selected_cell_passed","full_matrix_passed","qualification","failure","postcheck_failure","exit_code")},indent=2))
    print("Receipt: "+str(report_path))
    return exit_code

def main():
    parser=argparse.ArgumentParser(description=__doc__)
    subs=parser.add_subparsers(dest="command",required=True)
    collect=subs.add_parser("collect",help="Collect one completed artifact into a NEW matching shard directory")
    collect.add_argument("--cell",required=True)
    for name in ("artifact-metadata","job-metadata","run-metadata","zip"):collect.add_argument("--"+name,required=True,type=pathlib.Path)
    collect.add_argument("--artifact-id",required=True,type=int);collect.add_argument("--job-id",required=True,type=int)
    inspect=subs.add_parser("inspect",help="Read-only recheck of an already collected shard; never reruns native")
    inspect.add_argument("--cell",required=True)
    return run(parser.parse_args())

if __name__=="__main__":
    try:sys.exit(main())
    except Exception as exc:
        print(type(exc).__name__+": "+str(exc),file=sys.stderr);sys.exit(2)

