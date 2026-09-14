"""Preserve a clean S15 publication and attempt one ordinary authorized push."""
import datetime,hashlib,json,os,pathlib,subprocess,sys
base=pathlib.Path(__file__).resolve().parent;repo=base/'integration'
prior='4470920ba9ba66c755311091af0ebe2f43bda5ad'
def run(args,**extra):
    return subprocess.run(args,cwd=repo,text=True,encoding='utf8',errors='replace',stdout=subprocess.PIPE,stderr=subprocess.STDOUT,creationflags=subprocess.CREATE_NO_WINDOW,**extra)
def git(*args):
    result=run(['git','-c','core.longpaths=true',*args]);result.check_returncode();return result.stdout.strip()
head=git('rev-parse','HEAD');assert head==sys.argv[1]
assert git('branch','--show-current')=='codex/campaign-certification'
assert not git('status','--porcelain')
stamp=datetime.datetime.now(datetime.timezone.utc).strftime('%Y%m%dT%H%M%SZ')
bundle=base/('S15-incremental-'+head[:12]+'-'+stamp+'.bundle');assert not bundle.exists()
git('bundle','create',str(bundle),prior+'..codex/campaign-certification')
verification=git('bundle','verify',str(bundle));assert head in git('bundle','list-heads',str(bundle))
with bundle.open('rb') as f:checksum=hashlib.file_digest(f,'sha256').hexdigest()
env=dict(os.environ);env.update(GIT_TERMINAL_PROMPT='0',GCM_INTERACTIVE='Never')
log=base/('S15-push-'+stamp+'.log')
try:
    pushed=run(['git','-c','core.longpaths=true','push','origin','codex/campaign-certification'],env=env,timeout=60)
    code=pushed.returncode;output=pushed.stdout
except subprocess.TimeoutExpired as error:
    code=None;output=error.stdout or ''
    if isinstance(output,bytes):output=output.decode('utf8',errors='replace')
    output+='\nPush timed out; remote outcome is unconfirmed.\n'
log.write_text(output,encoding='utf8')
record={'head':head,'clean':not git('status','--porcelain'),'pushed':code==0,'push_exit_code':code,'push_log':str(log),
    'bundle':str(bundle),'bundle_sha256':checksum,'bundle_bytes':bundle.stat().st_size,'bundle_verification':verification,
    'prerequisite_commit':prior,'prerequisite_bundle':'S11-incremental-4470920ba9ba-20260914T032406Z.bundle',
    'scope':'Incremental backup of S15 source and retained qualification. Requires the recorded prior S11 closure history.'}
(base/('S15-git-disposition-'+stamp+'.json')).write_text(json.dumps(record,indent=2)+'\n',encoding='utf8')
print(json.dumps(record,indent=2),flush=True)
print(output,flush=True)

